//! TP-SHIPSIZE-DECODER-0: shipsize / `ship_*` modifier decoder family.
//!
//! Longest-match segmentation against registered ship classes and a closed
//! attribute set. Lowers to overlays (Add leaf-only / Multiply subtree-sweep),
//! triggered modifiers, column-backed complex triggers, and bounded `EvalEML`
//! `ExactDeterministic` formula trees (≤32 nodes).

use std::collections::BTreeMap;

use simthing_core::{
    eml_nodes, EmlNodeGpu, OverlayKind, OverlayLifecycle, OverlaySource, SubFieldRole,
    TransformOp,
};
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::resource_flow::{
    GatedRateOpSpec, GatedRateSpec, GatedRateTriggerSpec, RateFormulaOp, RateFormulaOperandSpec,
    RateFormulaOpSpec, RateFormulaSpec,
};
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::{
    CapabilityCategorySpec, CapabilitySpec, CapabilityTreeSpec, EventSpec, GameModeSpec,
    OverlaySpec, PropertySpec, SpecVersion,
};
use simthing_spec::{EffectSpec, ScopeRef, TriggerDirection, TriggerSpec};

use crate::error::HydrateError;
use crate::hydrate_category_economy::EconomicOp;
use crate::raw::{RawDocument, RawProperty, RawSpan, RawValue};

pub const MAX_SHIP_EML_NODES: usize = 32;

/// Closed ship/country modifier attributes admitted by TP-SHIPSIZE-DECODER-0.
pub const SHIP_MODIFIER_ATTRIBUTES: &[&str] = &[
    "hull",
    "weapon_damage",
    "fire_rate",
    "upkeep",
    "naval_cap",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipModifierOp {
    Add,
    Mult,
}

impl From<ShipModifierOp> for EconomicOp {
    fn from(op: ShipModifierOp) -> Self {
        match op {
            ShipModifierOp::Add => EconomicOp::Add,
            ShipModifierOp::Mult => EconomicOp::Mult,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShipModifierFamily {
    Shipsize { class: String },
    Ship,
    Ships,
    Country,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedShipModifierKey {
    pub family: ShipModifierFamily,
    pub attribute: String,
    pub op: ShipModifierOp,
}

#[derive(Debug, Clone)]
pub struct ShipClassEntry {
    pub custom_kind: String,
}

#[derive(Debug, Clone)]
pub struct ShipPropertyEntry {
    pub namespace: String,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct ColumnTriggerAlias {
    pub property: PropertyKey,
    pub at_least: f32,
}

#[derive(Debug, Clone)]
pub struct HydratedShipsizeDecoderPack {
    pub game_mode: GameModeSpec,
    pub decoded_keys: Vec<DecodedShipModifierKey>,
    pub ship_class_custom_kinds: BTreeMap<String, String>,
    pub eml_node_counts: Vec<usize>,
}

/// Decode one shipsize / `ship_*` modifier key (no span).
pub fn decode_ship_modifier_key(
    key: &str,
    ship_classes: &[String],
) -> Result<DecodedShipModifierKey, HydrateError> {
    decode_ship_modifier_key_spanned(key, ship_classes, None)
}

/// Decode with optional source span for hard-errors.
pub fn decode_ship_modifier_key_spanned(
    key: &str,
    ship_classes: &[String],
    span: Option<RawSpan>,
) -> Result<DecodedShipModifierKey, HydrateError> {
    let (family, stem, op) = split_family_and_op(key, span.clone())?;
    match family {
        InnerFamily::Shipsize => decode_shipsize_stem(stem, op, ship_classes, span),
        InnerFamily::Ship => decode_fixed_stem(stem, op, ShipModifierFamily::Ship, span),
        InnerFamily::Ships => decode_fixed_stem(stem, op, ShipModifierFamily::Ships, span),
        InnerFamily::Country => {
            if stem == "cost" || stem.ends_with("_cost") {
                return Err(HydrateError::new_spanned(
                    "`cost` ship/country modifier keys require a discrete ResourceEconomySpec context",
                    span,
                ));
            }
            decode_fixed_stem(stem, op, ShipModifierFamily::Country, span)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InnerFamily {
    Shipsize,
    Ship,
    Ships,
    Country,
}

fn split_family_and_op(
    key: &str,
    span: Option<RawSpan>,
) -> Result<(InnerFamily, &str, ShipModifierOp), HydrateError> {
    let (family, stem_with_op) = if let Some(rest) = key.strip_prefix("shipsize_") {
        (InnerFamily::Shipsize, rest)
    } else if let Some(rest) = key.strip_prefix("ships_") {
        (InnerFamily::Ships, rest)
    } else if let Some(rest) = key.strip_prefix("ship_") {
        (InnerFamily::Ship, rest)
    } else if let Some(rest) = key.strip_prefix("country_") {
        (InnerFamily::Country, rest)
    } else {
        return Err(HydrateError::new_spanned(
            format!("unknown ship modifier key `{key}`"),
            span,
        ));
    };
    let Some((stem, op_suffix)) = stem_with_op.rsplit_once('_') else {
        return Err(HydrateError::new_spanned(
            "missing op suffix `_add`/`_mult`",
            span,
        ));
    };
    let op = match op_suffix {
        "add" => ShipModifierOp::Add,
        "mult" => ShipModifierOp::Mult,
        _ => {
            return Err(HydrateError::new_spanned(
                "missing op suffix `_add`/`_mult`",
                span,
            ));
        }
    };
    Ok((family, stem, op))
}

fn decode_fixed_stem(
    stem: &str,
    op: ShipModifierOp,
    family: ShipModifierFamily,
    span: Option<RawSpan>,
) -> Result<DecodedShipModifierKey, HydrateError> {
    if !SHIP_MODIFIER_ATTRIBUTES.contains(&stem) {
        return Err(HydrateError::new_spanned(
            format!("unknown ship modifier attribute `{stem}`"),
            span,
        ));
    }
    Ok(DecodedShipModifierKey {
        family,
        attribute: stem.into(),
        op,
    })
}

fn decode_shipsize_stem(
    stem: &str,
    op: ShipModifierOp,
    ship_classes: &[String],
    span: Option<RawSpan>,
) -> Result<DecodedShipModifierKey, HydrateError> {
    let mut matches = Vec::new();
    for class in ship_classes {
        if let Some(rest) = stem.strip_prefix(&format!("{class}_")) {
            if SHIP_MODIFIER_ATTRIBUTES.contains(&rest) {
                matches.push((class.clone(), rest.to_string()));
            }
        }
    }
    if matches.is_empty() {
        if ship_classes.iter().any(|class| stem == class.as_str()) {
            return Err(HydrateError::new_spanned(
                "shipsize key missing attribute segment after class",
                span,
            ));
        }
        if let Some(class) = ship_classes
            .iter()
            .filter(|class| stem.starts_with(&format!("{class}_")))
            .max_by_key(|class| class.len())
        {
            let attr = stem.strip_prefix(&format!("{class}_")).unwrap_or(stem);
            return Err(HydrateError::new_spanned(
                format!("unknown shipsize attribute `{attr}`"),
                span,
            ));
        }
        return Err(HydrateError::new_spanned(
            format!("unregistered ship class in `{stem}`"),
            span,
        ));
    }
    let longest = matches.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
    let best: Vec<_> = matches
        .into_iter()
        .filter(|(class, _)| class.len() == longest)
        .collect();
    if best.len() != 1 {
        let op_label = match op {
            ShipModifierOp::Add => "add",
            ShipModifierOp::Mult => "mult",
        };
        return Err(HydrateError::new_spanned(
            format!("ambiguous shipsize modifier key `shipsize_{stem}_{op_label}`"),
            span,
        ));
    }
    let (class, attribute) = best.into_iter().next().unwrap();
    Ok(DecodedShipModifierKey {
        family: ShipModifierFamily::Shipsize { class },
        attribute,
        op,
    })
}

/// Hydrate the synthetic TP-SHIPSIZE-DECODER-0 proof fixture.
pub fn hydrate_shipsize_decoder_pack(
    document: &RawDocument,
) -> Result<HydratedShipsizeDecoderPack, HydrateError> {
    let RawValue::Block(root) = &document.root else {
        return Err(HydrateError::new("document root must be a property block"));
    };
    if root.properties.len() != 1 {
        return Err(HydrateError::new(
            "TP shipsize decoder fixture expects exactly one top-level block",
        ));
    }
    let fixture = &root.properties[0];
    let RawValue::Block(body) = &fixture.value else {
        return Err(HydrateError::new_spanned(
            "top-level fixture value must be a block",
            Some(fixture.key.span.clone()),
        ));
    };

    let mut display_name = fixture.key.text.clone();
    let mut ship_classes: BTreeMap<String, ShipClassEntry> = BTreeMap::new();
    let mut properties: BTreeMap<String, ShipPropertyEntry> = BTreeMap::new();
    let mut trigger_aliases: BTreeMap<String, ColumnTriggerAlias> = BTreeMap::new();
    let mut script_values: BTreeMap<String, RateFormulaSpec> = BTreeMap::new();
    let mut modifier_blocks = Vec::new();
    let mut triggered_blocks = Vec::new();
    let mut complex_blocks = Vec::new();
    let mut trigger_properties = Vec::new();

    for property in &body.properties {
        match property.key.text.as_str() {
            "display_name" => display_name = read_scalar_text(property, "display_name")?,
            "ship_class_map" => merge_ship_class_map(property, &mut ship_classes)?,
            "ship_property" => {
                let entry = parse_ship_property(property)?;
                if properties.insert(entry.name.clone(), entry).is_some() {
                    return Err(HydrateError::new_spanned(
                        "duplicate ship_property registration",
                        Some(property.key.span.clone()),
                    ));
                }
            }
            "trigger_alias" => parse_trigger_alias(property, &mut trigger_aliases)?,
            "trigger_property" => trigger_properties.push(parse_trigger_property(property)?),
            "script_value" => {
                let (id, formula) = parse_script_value(property)?;
                if script_values.insert(id, formula).is_some() {
                    return Err(HydrateError::new_spanned(
                        "duplicate script_value id",
                        Some(property.key.span.clone()),
                    ));
                }
            }
            "modifier" => modifier_blocks.push(property),
            "triggered_modifier" => triggered_blocks.push(property),
            "complex_trigger_modifier" => complex_blocks.push(property),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported TP shipsize fixture field `{other}`"),
                    Some(property.key.span.clone()),
                ));
            }
        }
    }

    if ship_classes.is_empty() {
        return Err(HydrateError::new("ship_class_map is required"));
    }
    if properties.is_empty() {
        return Err(HydrateError::new("at least one ship_property is required"));
    }

    let class_names: Vec<String> = ship_classes.keys().cloned().collect();
    let mut decoded_keys = Vec::new();
    let mut overlays = Vec::new();
    let mut events = Vec::new();
    let mut gated_rates = Vec::new();
    let mut eml_node_counts = Vec::new();
    let mut capability_trees = Vec::new();
    let custom_kinds: BTreeMap<String, String> = ship_classes
        .iter()
        .map(|(class, entry)| (class.clone(), entry.custom_kind.clone()))
        .collect();

    for (class, entry) in &ship_classes {
        capability_trees.push(build_ship_class_capability_tree(class, entry, &properties)?);
    }

    for block in modifier_blocks {
        parse_modifier_block(
            block,
            &class_names,
            &properties,
            &custom_kinds,
            &script_values,
            &mut decoded_keys,
            &mut overlays,
            &mut gated_rates,
            &mut eml_node_counts,
        )?;
    }

    for block in triggered_blocks {
        parse_triggered_modifier_block(
            block,
            &class_names,
            &properties,
            &custom_kinds,
            &trigger_aliases,
            &mut decoded_keys,
            &mut overlays,
            &mut events,
        )?;
    }

    for block in complex_blocks {
        parse_complex_trigger_modifier_block(
            block,
            &class_names,
            &properties,
            &custom_kinds,
            &mut decoded_keys,
            &mut gated_rates,
            &mut eml_node_counts,
        )?;
    }

    let mut game_properties: Vec<PropertySpec> = trigger_properties;
    for entry in properties.values() {
        game_properties.push(property_spec_from_entry(entry));
    }

    Ok(HydratedShipsizeDecoderPack {
        game_mode: GameModeSpec {
            id: fixture.key.text.clone(),
            display_name,
            description: String::new(),
            spec_version: SpecVersion::default(),
            metadata: Default::default(),
            domain_packs: vec![],
            properties: game_properties,
            overlays,
            capability_trees,
            events,
            resource_flow: if gated_rates.is_empty() {
                None
            } else {
                Some(simthing_spec::ResourceFlowSpec {
                    opt_in_mode: simthing_spec::ResourceFlowOptInMode::Disabled,
                    arenas: vec![],
                    couplings: vec![],
                    base_obligations: vec![],
                    capacity_budget: None,
                    gated_rates,
                    need_weight_profiles: vec![],
                })
            },
            resource_economy: None,
            resource_flow_execution_profile: Default::default(),
            region_fields: vec![],
            mapping_execution_profile: Default::default(),
        },
        decoded_keys,
        ship_class_custom_kinds: custom_kinds,
        eml_node_counts,
    })
}

fn build_ship_class_capability_tree(
    class: &str,
    entry: &ShipClassEntry,
    properties: &BTreeMap<String, ShipPropertyEntry>,
) -> Result<CapabilityTreeSpec, HydrateError> {
    let hull = properties.get("hull").ok_or_else(|| {
        HydrateError::new("ship_property `hull` required for capability-tree lowering")
    })?;
    Ok(CapabilityTreeSpec {
        tree_id: format!("tp_shipsize_{class}_hull"),
        tree_kind: entry.custom_kind.clone(),
        owner_kind: "Ship".into(),
        categories: vec![CapabilityCategorySpec {
            property_namespace: hull.namespace.clone(),
            property_name: hull.name.clone(),
            display_name: format!("{class} hull"),
            tier: 0,
            max_active: None,
            entries: vec![CapabilitySpec {
                id: format!("{class}_hull_seed"),
                display_name: format!("{class} hull seed"),
                description: String::new(),
                flavor_text: String::new(),
                research_cost: 0.0,
                activation: Default::default(),
                icon: String::new(),
                thumbnail: String::new(),
                card_image: String::new(),
                unlock_video: None,
                model_preview: None,
                prereqs: vec![],
                unlocks_ship_components: vec![],
                unlocks_buildings: vec![],
                unlocks_units: vec![],
                unlocks_weapons: vec![],
                effects: vec![],
            }],
        }],
        install: InstallTargetSpec::AllOfKind {
            kind: "Ship".into(),
        },
    })
}

fn property_spec_from_entry(entry: &ShipPropertyEntry) -> PropertySpec {
    PropertySpec {
        id: format!("{}_{}", entry.namespace, entry.name),
        namespace: entry.namespace.clone(),
        name: entry.name.clone(),
        display_name: entry.display_name.clone(),
        description: String::new(),
        sub_fields: vec![simthing_core::SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: simthing_core::ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "Amount".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        }],
    }
}

fn property_key_for_attribute(
    attribute: &str,
    properties: &BTreeMap<String, ShipPropertyEntry>,
) -> Result<PropertyKey, HydrateError> {
    let entry = properties.get(attribute).ok_or_else(|| {
        HydrateError::new(format!("no ship_property registered for attribute `{attribute}`"))
    })?;
    Ok(PropertyKey::new(&entry.namespace, &entry.name))
}

fn overlay_install_for_key(
    decoded: &DecodedShipModifierKey,
    custom_kinds: &BTreeMap<String, String>,
) -> InstallTargetSpec {
    match (&decoded.family, decoded.op) {
        (ShipModifierFamily::Shipsize { class }, ShipModifierOp::Add) => InstallTargetSpec::AllOfKind {
            kind: custom_kinds
                .get(class)
                .cloned()
                .unwrap_or_else(|| format!("ship_hull_{class}")),
        },
        (ShipModifierFamily::Country, _) => InstallTargetSpec::AllOfKind {
            kind: "Faction".into(),
        },
        (_, ShipModifierOp::Add) => InstallTargetSpec::AllOfKind {
            kind: "Ship".into(),
        },
        (_, ShipModifierOp::Mult) => InstallTargetSpec::AllOfKind {
            kind: "Ship".into(),
        },
    }
}

fn build_overlay(
    id: &str,
    decoded: &DecodedShipModifierKey,
    amount: f32,
    properties: &BTreeMap<String, ShipPropertyEntry>,
    custom_kinds: &BTreeMap<String, String>,
) -> Result<OverlaySpec, HydrateError> {
    let property_key = property_key_for_attribute(&decoded.attribute, properties)?;
    let transform = match decoded.op {
        ShipModifierOp::Add => TransformOp::Add(amount),
        ShipModifierOp::Mult => TransformOp::Multiply(1.0 + amount),
    };
    Ok(OverlaySpec {
        id: id.into(),
        display_name: String::new(),
        targets_property: format!("{}::{}", property_key.namespace, property_key.name),
        sub_field_deltas: vec![(SubFieldRole::Amount, transform)],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Policy,
        source: OverlaySource::Player,
        install: overlay_install_for_key(decoded, custom_kinds),
    })
}

#[allow(clippy::too_many_arguments)]
fn parse_modifier_block(
    property: &RawProperty,
    class_names: &[String],
    properties: &BTreeMap<String, ShipPropertyEntry>,
    custom_kinds: &BTreeMap<String, String>,
    script_values: &BTreeMap<String, RateFormulaSpec>,
    decoded_keys: &mut Vec<DecodedShipModifierKey>,
    overlays: &mut Vec<OverlaySpec>,
    gated_rates: &mut Vec<GatedRateSpec>,
    eml_node_counts: &mut Vec<usize>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`modifier` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut block_id = property.key.text.clone();
    for field in &block.properties {
        if field.key.text == "id" {
            block_id = read_scalar_text(field, "id")?;
            continue;
        }
        if field.key.text == "display_name" {
            read_scalar_text(field, "display_name")?;
            continue;
        }
        let decoded = decode_ship_modifier_key_spanned(
            &field.key.text,
            class_names,
            Some(field.key.span.clone()),
        )?;
        decoded_keys.push(decoded.clone());
        if let RawValue::Scalar(scalar) = &field.value {
            if let Some(name) = scalar.text.strip_prefix("value:") {
                let formula = script_values.get(name).ok_or_else(|| {
                    HydrateError::new_spanned(
                        format!("unknown script_value `{name}`"),
                        Some(field.key.span.clone()),
                    )
                })?;
                let nodes = compile_value_formula_eml(formula, false);
                record_eml_nodes(&nodes, eml_node_counts)?;
                let trigger_key = property_key_for_attribute(&decoded.attribute, properties)?;
                gated_rates.push(GatedRateSpec {
                    id: format!("{block_id}_{}", field.key.text),
                    arena: format!("ship_{}", decoded.attribute),
                    install: overlay_install_for_key(&decoded, custom_kinds),
                    direction: simthing_spec::BaseFlowDirectionSpec::Produce,
                    op: match decoded.op {
                        ShipModifierOp::Add => GatedRateOpSpec::Add,
                        ShipModifierOp::Mult => GatedRateOpSpec::Mult,
                    },
                    rate: 0.0,
                    trigger: None,
                    rate_formula: Some(formula.clone()),
                });
                let _ = trigger_key;
                continue;
            }
        }
        let amount = read_scalar_f32(field, &field.key.text)?;
        overlays.push(build_overlay(
            &format!("{block_id}_{}", field.key.text),
            &decoded,
            amount,
            properties,
            custom_kinds,
        )?);
    }
    Ok(())
}

fn parse_triggered_modifier_block(
    property: &RawProperty,
    class_names: &[String],
    properties: &BTreeMap<String, ShipPropertyEntry>,
    custom_kinds: &BTreeMap<String, String>,
    trigger_aliases: &BTreeMap<String, ColumnTriggerAlias>,
    decoded_keys: &mut Vec<DecodedShipModifierKey>,
    overlays: &mut Vec<OverlaySpec>,
    events: &mut Vec<EventSpec>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`triggered_modifier` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut id = None;
    let mut potential = None;
    let mut modifier_key = None;
    let mut amount = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "potential" => potential = Some(field),
            key => {
                let decoded = decode_ship_modifier_key_spanned(
                    key,
                    class_names,
                    Some(field.key.span.clone()),
                )?;
                if modifier_key.is_some() {
                    return Err(HydrateError::new_spanned(
                        "one modifier key per triggered_modifier block",
                        Some(field.key.span.clone()),
                    ));
                }
                modifier_key = Some(decoded.clone());
                amount = Some(read_scalar_f32(field, key)?);
                decoded_keys.push(decoded);
            }
        }
    }

    let id = require_field(id, "id", property)?;
    let potential = require_field(potential, "potential", property)?;
    let decoded = require_field(modifier_key, "modifier key", property)?;
    let amount = require_field(amount, "modifier amount", property)?;
    let trigger = parse_column_backed_potential(potential, trigger_aliases)?;

    let mut overlay = build_overlay(&id, &decoded, amount, properties, custom_kinds)?;
    overlay.lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(OverlayLifecycle::Permanent),
    };
    let overlay_id = overlay.id.clone();
    overlays.push(overlay);

    events.push(EventSpec {
        id: id.clone(),
        trigger: TriggerSpec::Threshold {
            target: ScopeRef::Current,
            property: trigger.property.clone(),
            role: SubFieldRole::Amount,
            threshold: trigger.at_least,
            direction: TriggerDirection::Rising,
        },
        effects: vec![EffectSpec::ActivateOverlayRef {
            target: ScopeRef::Current,
            overlay_ref: overlay_id,
        }],
        cooldown: None,
        priority: Default::default(),
        install: InstallTargetSpec::SessionRoot,
    });
    Ok(())
}

fn parse_complex_trigger_modifier_block(
    property: &RawProperty,
    class_names: &[String],
    _properties: &BTreeMap<String, ShipPropertyEntry>,
    custom_kinds: &BTreeMap<String, String>,
    decoded_keys: &mut Vec<DecodedShipModifierKey>,
    gated_rates: &mut Vec<GatedRateSpec>,
    eml_node_counts: &mut Vec<usize>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`complex_trigger_modifier` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut id = None;
    let mut trigger_field = None;
    let mut modifier_key = None;
    let mut amount = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "trigger" => trigger_field = Some(field),
            key => {
                let decoded = decode_ship_modifier_key_spanned(
                    key,
                    class_names,
                    Some(field.key.span.clone()),
                )?;
                if modifier_key.is_some() {
                    return Err(HydrateError::new_spanned(
                        "one modifier key per complex_trigger_modifier block",
                        Some(field.key.span.clone()),
                    ));
                }
                modifier_key = Some(decoded.clone());
                amount = Some(read_scalar_f32(field, key)?);
                decoded_keys.push(decoded);
            }
        }
    }

    let id = require_field(id, "id", property)?;
    let trigger_field = require_field(trigger_field, "trigger", property)?;
    let decoded = require_field(modifier_key, "modifier key", property)?;
    let amount = require_field(amount, "modifier amount", property)?;
    let trigger = parse_column_backed_trigger_block(trigger_field)?;

    let formula = RateFormulaSpec {
        base: amount,
        ops: vec![RateFormulaOpSpec {
            op: RateFormulaOp::Mult,
            operand: RateFormulaOperandSpec::Property(trigger.property.clone()),
        }],
    };
    let mut nodes = compile_value_formula_eml(&formula, false);
    nodes.push(EmlNodeGpu {
        opcode: eml_nodes::opcode::CMP_GE,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    });
    nodes.push(EmlNodeGpu {
        opcode: eml_nodes::opcode::MUL,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    });
    nodes.push(EmlNodeGpu {
        opcode: eml_nodes::opcode::RETURN_TOP,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    });
    record_eml_nodes(&nodes, eml_node_counts)?;

    gated_rates.push(GatedRateSpec {
        id: id.clone(),
        arena: format!("ship_{}", decoded.attribute),
        install: overlay_install_for_key(&decoded, custom_kinds),
        direction: simthing_spec::BaseFlowDirectionSpec::Produce,
        op: match decoded.op {
            ShipModifierOp::Add => GatedRateOpSpec::Add,
            ShipModifierOp::Mult => GatedRateOpSpec::Mult,
        },
        rate: 0.0,
        trigger: Some(trigger),
        rate_formula: Some(formula),
    });
    Ok(())
}

fn parse_column_backed_potential(
    property: &RawProperty,
    aliases: &BTreeMap<String, ColumnTriggerAlias>,
) -> Result<ColumnTriggerAlias, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`potential` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    if block.properties.len() != 1 {
        return Err(HydrateError::new_spanned(
            "potential admits exactly one column-backed trigger alias",
            Some(property.key.span.clone()),
        ));
    }
    let field = &block.properties[0];
    let alias = aliases.get(&field.key.text).ok_or_else(|| {
        HydrateError::new_spanned(
            format!(
                "non-column-backed trigger `{}` is not admitted (complex_trigger_modifier requires a registered column-backed alias)",
                field.key.text
            ),
            Some(field.key.span.clone()),
        )
    })?;
    let value = read_scalar_text(field, &field.key.text)?;
    if value != "yes" && value != "true" && value != "1" {
        return Err(HydrateError::new_spanned(
            format!("trigger alias `{}` expects affirmative scalar", field.key.text),
            Some(field.key.span.clone()),
        ));
    }
    Ok(alias.clone())
}

fn parse_column_backed_trigger_block(
    property: &RawProperty,
) -> Result<GatedRateTriggerSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`trigger` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut property_key = None;
    let mut at_least = Some(0.5);
    for field in &block.properties {
        match field.key.text.as_str() {
            "property" => {
                let text = read_scalar_text(field, "property")?;
                let Some((namespace, name)) = text.split_once("::") else {
                    return Err(HydrateError::new_spanned(
                        format!("trigger `property` must be `namespace::name`, got `{text}`"),
                        Some(field.key.span.clone()),
                    ));
                };
                property_key = Some(PropertyKey::new(namespace, name));
            }
            "at_least" => at_least = Some(read_scalar_f32(field, "at_least")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!(
                        "non-column-backed trigger field `{other}` is not admitted (column-backed `property` required)"
                    ),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(GatedRateTriggerSpec {
        property: property_key.ok_or_else(|| {
            HydrateError::new_spanned(
                "column-backed complex trigger requires `property`",
                Some(property.key.span.clone()),
            )
        })?,
        at_least: at_least.unwrap_or(0.5),
    })
}

fn record_eml_nodes(nodes: &[EmlNodeGpu], counts: &mut Vec<usize>) -> Result<(), HydrateError> {
    if nodes.len() > MAX_SHIP_EML_NODES {
        return Err(HydrateError::new(format!(
            "compiled ship EML tree has {} nodes (max {MAX_SHIP_EML_NODES})",
            nodes.len()
        )));
    }
    counts.push(nodes.len());
    Ok(())
}

/// Compile a flat `value:` formula to `ExactDeterministic` EML nodes (test + hydration).
pub fn compile_value_formula_eml(formula: &RateFormulaSpec, negate: bool) -> Vec<EmlNodeGpu> {
    let mut nodes = vec![literal_node(formula.base)];
    for op in &formula.ops {
        push_operand(&mut nodes, &op.operand);
        nodes.push(op_node(match op.op {
            RateFormulaOp::Add => eml_nodes::opcode::ADD,
            RateFormulaOp::Mult => eml_nodes::opcode::MUL,
            RateFormulaOp::FloorAt => eml_nodes::opcode::MAX,
            RateFormulaOp::CeilAt => eml_nodes::opcode::MIN,
        }));
    }
    if negate {
        nodes.push(op_node(eml_nodes::opcode::NEG));
    }
    nodes.push(op_node(eml_nodes::opcode::RETURN_TOP));
    nodes
}

fn literal_node(value: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: value.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn slot_value_node(col: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn op_node(opcode: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn push_operand(nodes: &mut Vec<EmlNodeGpu>, operand: &RateFormulaOperandSpec) {
    match operand {
        RateFormulaOperandSpec::Literal(value) => nodes.push(literal_node(*value)),
        RateFormulaOperandSpec::Property(key) => {
            nodes.push(slot_value_node(property_column_stub(key)));
        }
    }
}

fn property_column_stub(key: &PropertyKey) -> u32 {
    let hash = key
        .namespace
        .bytes()
        .chain(key.name.bytes())
        .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    1 + (hash % 7)
}

fn merge_ship_class_map(
    property: &RawProperty,
    classes: &mut BTreeMap<String, ShipClassEntry>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`ship_class_map` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    for entry in &block.properties {
        let RawValue::Block(body) = &entry.value else {
            return Err(HydrateError::new_spanned(
                "ship_class_map entry must be a block",
                Some(entry.key.span.clone()),
            ));
        };
        let mut custom_kind = None;
        for field in &body.properties {
            match field.key.text.as_str() {
                "custom_kind" => custom_kind = Some(read_scalar_text(field, "custom_kind")?),
                other => {
                    return Err(HydrateError::new_spanned(
                        format!("unsupported ship_class_map field `{other}`"),
                        Some(field.key.span.clone()),
                    ));
                }
            }
        }
        classes.insert(
            entry.key.text.clone(),
            ShipClassEntry {
                custom_kind: require_field(custom_kind, "custom_kind", entry)?,
            },
        );
    }
    Ok(())
}

fn parse_ship_property(property: &RawProperty) -> Result<ShipPropertyEntry, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`ship_property` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut namespace = None;
    let mut name = None;
    let mut display_name = String::new();
    for field in &block.properties {
        match field.key.text.as_str() {
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "name" => name = Some(read_scalar_text(field, "name")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "id" => {
                read_scalar_text(field, "id")?;
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported ship_property field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(ShipPropertyEntry {
        namespace: require_field(namespace, "namespace", property)?,
        name: require_field(name, "name", property)?,
        display_name,
    })
}

fn parse_trigger_alias(
    property: &RawProperty,
    aliases: &mut BTreeMap<String, ColumnTriggerAlias>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`trigger_alias` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    for entry in &block.properties {
        let RawValue::Block(body) = &entry.value else {
            return Err(HydrateError::new_spanned(
                "trigger_alias entry must be a block",
                Some(entry.key.span.clone()),
            ));
        };
        let mut property_key = None;
        let mut at_least = Some(0.5);
        for field in &body.properties {
            match field.key.text.as_str() {
                "property" => {
                    let text = read_scalar_text(field, "property")?;
                    let Some((namespace, name)) = text.split_once("::") else {
                        return Err(HydrateError::new_spanned(
                            format!("alias `property` must be `namespace::name`, got `{text}`"),
                            Some(field.key.span.clone()),
                        ));
                    };
                    property_key = Some(PropertyKey::new(namespace, name));
                }
                "at_least" => at_least = Some(read_scalar_f32(field, "at_least")?),
                other => {
                    return Err(HydrateError::new_spanned(
                        format!("unsupported trigger_alias field `{other}`"),
                        Some(field.key.span.clone()),
                    ));
                }
            }
        }
        aliases.insert(
            entry.key.text.clone(),
            ColumnTriggerAlias {
                property: require_field(property_key, "property", entry)?,
                at_least: at_least.unwrap_or(0.5),
            },
        );
    }
    Ok(())
}

fn parse_trigger_property(property: &RawProperty) -> Result<PropertySpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`trigger_property` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut namespace = None;
    let mut name = None;
    let mut display_name = String::new();
    for field in &block.properties {
        match field.key.text.as_str() {
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "name" => name = Some(read_scalar_text(field, "name")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "id" => {
                read_scalar_text(field, "id")?;
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported trigger_property field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(PropertySpec {
        id: format!(
            "{}_{}",
            namespace.as_deref().unwrap_or("tp"),
            name.as_deref().unwrap_or("trigger")
        ),
        namespace: require_field(namespace, "namespace", property)?,
        name: require_field(name, "name", property)?,
        display_name,
        description: String::new(),
        sub_fields: vec![simthing_core::SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: simthing_core::ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "Amount".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        }],
    })
}

fn parse_script_value(property: &RawProperty) -> Result<(String, RateFormulaSpec), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`script_value` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut id = None;
    let mut base = None;
    let mut ops = Vec::new();
    for field in &block.properties {
        let op = match field.key.text.as_str() {
            "id" => {
                id = Some(read_scalar_text(field, "id")?);
                continue;
            }
            "base" => {
                base = Some(read_scalar_f32(field, "base")?);
                continue;
            }
            "add" => RateFormulaOp::Add,
            "mult" => RateFormulaOp::Mult,
            "floor_at" => RateFormulaOp::FloorAt,
            "ceil_at" => RateFormulaOp::CeilAt,
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported script_value field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        };
        let operand = match &field.value {
            RawValue::Scalar(_) => {
                RateFormulaOperandSpec::Literal(read_scalar_f32(field, &field.key.text)?)
            }
            RawValue::Block(operand_block) => {
                let mut key = None;
                for entry in &operand_block.properties {
                    if entry.key.text == "property" {
                        let text = read_scalar_text(entry, "property")?;
                        let Some((namespace, name)) = text.split_once("::") else {
                            return Err(HydrateError::new_spanned(
                                format!("operand `property` must be `namespace::name`, got `{text}`"),
                                Some(entry.key.span.clone()),
                            ));
                        };
                        key = Some(PropertyKey::new(namespace, name));
                    } else {
                        return Err(HydrateError::new_spanned(
                            "operand block requires `property`",
                            Some(field.key.span.clone()),
                        ));
                    }
                }
                RateFormulaOperandSpec::Property(key.ok_or_else(|| {
                    HydrateError::new_spanned(
                        "operand block requires `property`",
                        Some(field.key.span.clone()),
                    )
                })?)
            }
            _ => {
                return Err(HydrateError::new_spanned(
                    "operand must be scalar or property block",
                    Some(field.key.span.clone()),
                ));
            }
        };
        ops.push(RateFormulaOpSpec { op, operand });
    }
    Ok((
        id.ok_or_else(|| {
            HydrateError::new_spanned("script_value requires `id`", Some(property.key.span.clone()))
        })?,
        RateFormulaSpec {
            base: base.ok_or_else(|| {
                HydrateError::new_spanned(
                    "script_value requires `base`",
                    Some(property.key.span.clone()),
                )
            })?,
            ops,
        },
    ))
}

fn require_field<T>(
    value: Option<T>,
    what: &str,
    property: &RawProperty,
) -> Result<T, HydrateError> {
    value.ok_or_else(|| {
        HydrateError::new_spanned(
            format!("{what} is required"),
            Some(property.key.span.clone()),
        )
    })
}

fn read_scalar_text(property: &RawProperty, field: &str) -> Result<String, HydrateError> {
    match &property.value {
        RawValue::Scalar(scalar) => Ok(scalar.text.clone()),
        _ => Err(HydrateError::new_spanned(
            format!("`{field}` must be a scalar"),
            Some(property.key.span.clone()),
        )),
    }
}

fn read_scalar_f32(property: &RawProperty, field: &str) -> Result<f32, HydrateError> {
    let text = read_scalar_text(property, field)?;
    text.parse::<f32>().map_err(|_| {
        HydrateError::new_spanned(
            format!("`{field}` must be a finite number, got `{text}`"),
            Some(property.key.span.clone()),
        )
    })
}