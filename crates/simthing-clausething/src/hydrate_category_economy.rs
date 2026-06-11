//! CT-2c economic-category hydration into existing Resource Flow and ResourceEconomy authoring.
//!
//! Categories are parsed as ClauseThing-side admission metadata only. The emitted
//! [`GameModeSpec`] contains ordinary properties, overlays, Resource Flow arenas,
//! base-flow obligations, and ResourceEconomy registrations; no category runtime artifact is produced.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, LogTier, OverlayKind, OverlayLifecycle,
    OverlaySource, SubFieldRole, SubFieldSpec, TransformOp,
};
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::overlay::OverlaySpec;
use simthing_spec::spec::resource_economy::{
    RecipeInputSpec, ResourceEconomyOptInMode, ResourceEconomySpec, ResourceRecipeSpec,
    ResourceTransferSpec,
};
use simthing_spec::spec::resource_flow::{
    BaseFlowDirectionSpec, BaseFlowObligationSpec, ResourceFlowOptInMode, ResourceFlowSpec,
};
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::{ArenaSpec, FissionPolicySpec, GameModeSpec, PropertySpec, SpecVersion};

use crate::error::HydrateError;
use crate::raw::{RawDocument, RawProperty, RawValue};

#[derive(Debug, Clone)]
pub struct HydratedCategoryEconomyPack {
    pub game_mode: GameModeSpec,
    /// Diagnostic mirror of literal `_add` contributions; not consumed by install/session proof.
    pub contributions: Vec<CategoryFlowContribution>,
    pub decoded_modifier_keys: Vec<DecodedEconomicKey>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CategoryFlowContribution {
    pub category: String,
    pub resource: String,
    pub axis: EconomicAxis,
    pub property: PropertyKey,
    pub arena: String,
    pub rate: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EconomicAxis {
    Produces,
    Upkeep,
    Cost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EconomicOp {
    Add,
    Mult,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecodedEconomicKey {
    pub category: String,
    pub resource: String,
    pub axis: EconomicAxis,
    pub op: EconomicOp,
}

#[derive(Debug, Clone)]
struct CategoryEntry {
    kind: String,
}

#[derive(Debug, Clone)]
struct ResourceEntry {
    namespace: String,
    name: String,
    display_name: String,
}

#[derive(Debug, Clone)]
struct ArenaDefaults {
    opt_in_mode: ResourceFlowOptInMode,
    max_participants: u32,
    max_coupling_fanout: u32,
    max_orderband_depth: u32,
}

impl Default for ArenaDefaults {
    fn default() -> Self {
        Self {
            opt_in_mode: ResourceFlowOptInMode::Disabled,
            max_participants: 16,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
        }
    }
}

/// Decode one CT-2c economic modifier key against closed category/resource sets.
pub fn decode_economic_modifier_key(
    key: &str,
    categories: &[String],
    resources: &[String],
) -> Result<DecodedEconomicKey, HydrateError> {
    decode_economic_modifier_key_spanned(key, categories, resources, None)
}

/// Hydrate one synthetic CT-2c category economy fixture.
pub fn hydrate_category_economy_pack(
    document: &RawDocument,
) -> Result<HydratedCategoryEconomyPack, HydrateError> {
    let RawValue::Block(root) = &document.root else {
        return Err(HydrateError::new("document root must be a property block"));
    };
    if root.properties.len() != 1 {
        return Err(HydrateError::new(
            "CT-2c category economy expects exactly one top-level fixture",
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
    let mut description = String::new();
    let mut categories = builtin_categories();
    let mut resources = BTreeMap::new();
    let mut arena_defaults = ArenaDefaults::default();
    let mut unit_templates = Vec::new();
    let mut modifier_blocks = Vec::new();

    for property in &body.properties {
        match property.key.text.as_str() {
            "display_name" => display_name = read_scalar_text(property, "display_name")?,
            "description" => description = read_scalar_text(property, "description")?,
            "category_map" => merge_category_map(property, &mut categories)?,
            "resource" => {
                let resource = parse_resource_block(property)?;
                if resources.insert(resource.name.clone(), resource).is_some() {
                    return Err(HydrateError::new_spanned(
                        "duplicate resource registration",
                        Some(property.key.span.clone()),
                    ));
                }
            }
            "resource_flow" => arena_defaults = parse_resource_flow_defaults(property)?,
            "unit_template" => unit_templates.push(property),
            "modifier" => modifier_blocks.push(property),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported CT-2c category fixture field `{other}`"),
                    Some(property.key.span.clone()),
                ));
            }
        }
    }

    if resources.is_empty() {
        return Err(HydrateError::new(
            "category economy fixture requires `resource` entries",
        ));
    }
    if unit_templates.is_empty() {
        return Err(HydrateError::new(
            "category economy fixture requires at least one `unit_template`",
        ));
    }

    let category_names = categories.keys().cloned().collect::<Vec<_>>();
    let resource_names = resources.keys().cloned().collect::<Vec<_>>();
    let mut used_pairs: BTreeMap<(String, String), (PropertySpec, ArenaSpec)> = BTreeMap::new();
    let mut contributions = Vec::new();
    let mut base_obligations = Vec::new();
    let mut decoded_modifier_keys = Vec::new();
    let mut overlays = Vec::new();

    for template in unit_templates {
        parse_unit_template(
            template,
            &categories,
            &resources,
            &category_names,
            &resource_names,
            &arena_defaults,
            &mut used_pairs,
            &mut contributions,
            &mut base_obligations,
        )?;
    }

    for modifier in modifier_blocks {
        let modifier_overlays = parse_modifier_block(
            modifier,
            &categories,
            &resources,
            &category_names,
            &resource_names,
            &arena_defaults,
            &mut used_pairs,
            &mut decoded_modifier_keys,
        )?;
        overlays.extend(modifier_overlays);
    }

    let (properties, arenas): (Vec<_>, Vec<_>) = used_pairs.into_values().unzip();

    Ok(HydratedCategoryEconomyPack {
        game_mode: GameModeSpec {
            id: fixture.key.text.clone(),
            display_name,
            description,
            spec_version: SpecVersion::default(),
            metadata: Default::default(),
            domain_packs: vec![],
            properties,
            overlays,
            capability_trees: vec![],
            events: vec![],
            resource_flow: Some(ResourceFlowSpec {
                opt_in_mode: arena_defaults.opt_in_mode,
                arenas,
                couplings: vec![],
                base_obligations,
            }),
            resource_economy: None,
            resource_flow_execution_profile: Default::default(),
            region_fields: vec![],
            mapping_execution_profile: Default::default(),
        },
        contributions,
        decoded_modifier_keys,
    })
}

/// Hydrate the minimal CT-2c discrete Daily Economy dialect to ResourceEconomySpec.
pub fn hydrate_daily_economy_game_mode(
    document: &RawDocument,
) -> Result<GameModeSpec, HydrateError> {
    let RawValue::Block(root) = &document.root else {
        return Err(HydrateError::new("document root must be a property block"));
    };
    if root.properties.len() != 1 {
        return Err(HydrateError::new(
            "CT-2c daily economy expects exactly one top-level fixture",
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
    let mut description = String::new();
    let mut properties = Vec::new();
    let mut opt_in_mode = ResourceEconomyOptInMode::Disabled;
    let mut transfers = Vec::new();
    let mut recipes = Vec::new();

    for property in &body.properties {
        match property.key.text.as_str() {
            "display_name" => display_name = read_scalar_text(property, "display_name")?,
            "description" => description = read_scalar_text(property, "description")?,
            "property" => properties.push(parse_daily_property_block(property)?),
            "resource_economy" => opt_in_mode = parse_resource_economy_block(property)?,
            "transfer" => transfers.push(parse_transfer_block(property)?),
            "recipe" => recipes.push(parse_recipe_block(property)?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported CT-2c daily economy field `{other}`"),
                    Some(property.key.span.clone()),
                ));
            }
        }
    }

    if properties.is_empty() {
        return Err(HydrateError::new(
            "daily economy fixture requires at least one `property` block",
        ));
    }

    Ok(GameModeSpec {
        id: fixture.key.text.clone(),
        display_name,
        description,
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties,
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: None,
        resource_economy: Some(ResourceEconomySpec {
            opt_in_mode,
            transfers,
            recipes,
            emissions: vec![],
            emit_on_threshold: vec![],
        }),
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    })
}

fn parse_unit_template(
    property: &RawProperty,
    categories: &BTreeMap<String, CategoryEntry>,
    resources: &BTreeMap<String, ResourceEntry>,
    category_names: &[String],
    resource_names: &[String],
    arena_defaults: &ArenaDefaults,
    used_pairs: &mut BTreeMap<(String, String), (PropertySpec, ArenaSpec)>,
    contributions: &mut Vec<CategoryFlowContribution>,
    base_obligations: &mut Vec<BaseFlowObligationSpec>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`unit_template` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut template_id = None;
    let mut category_seen = None;
    let mut resources_block = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => {
                template_id = Some(read_scalar_text(field, "id")?);
            }
            "category" => category_seen = Some(read_scalar_text(field, "category")?),
            "resources" => resources_block = Some(field),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported unit_template field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let template_id = template_id.ok_or_else(|| {
        HydrateError::new_spanned(
            "unit_template requires `id`",
            Some(property.key.span.clone()),
        )
    })?;
    let category = category_seen.ok_or_else(|| {
        HydrateError::new_spanned(
            "unit_template requires `category`",
            Some(property.key.span.clone()),
        )
    })?;
    if !categories.contains_key(&category) {
        return Err(unmapped_category_error(
            &category,
            Some(property.key.span.clone()),
        ));
    }
    let resources_block = resources_block.ok_or_else(|| {
        HydrateError::new_spanned(
            "unit_template requires `resources`",
            Some(property.key.span.clone()),
        )
    })?;
    let RawValue::Block(resource_body) = &resources_block.value else {
        return Err(HydrateError::new_spanned(
            "`resources` must be a block",
            Some(resources_block.key.span.clone()),
        ));
    };

    for flow_block in &resource_body.properties {
        let axis = match flow_block.key.text.as_str() {
            "produces" => EconomicAxis::Produces,
            "upkeep" => EconomicAxis::Upkeep,
            "cost" => {
                return Err(HydrateError::new_spanned(
                    "`cost` keys require a discrete ResourceEconomySpec context",
                    Some(flow_block.key.span.clone()),
                ));
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported resources field `{other}`"),
                    Some(flow_block.key.span.clone()),
                ));
            }
        };
        let RawValue::Block(entries) = &flow_block.value else {
            return Err(HydrateError::new_spanned(
                format!("`{}` must be a block", flow_block.key.text),
                Some(flow_block.key.span.clone()),
            ));
        };
        for entry in &entries.properties {
            if entry.key.text == "triggered_produces_modifier" || entry.key.text == "trigger" {
                return Err(HydrateError::new_spanned(
                    "triggered/gated produces forms are deferred to CT-2b",
                    Some(entry.key.span.clone()),
                ));
            }
            let decoded = decode_economic_modifier_key_spanned(
                &entry.key.text,
                category_names,
                resource_names,
                Some(entry.key.span.clone()),
            )?;
            if decoded.category != category {
                return Err(HydrateError::new_spanned(
                    format!(
                        "key category `{}` does not match unit_template category `{category}`",
                        decoded.category
                    ),
                    Some(entry.key.span.clone()),
                ));
            }
            if decoded.axis != axis {
                return Err(HydrateError::new_spanned(
                    format!(
                        "key axis does not belong in `{}` block",
                        flow_block.key.text
                    ),
                    Some(entry.key.span.clone()),
                ));
            }
            if decoded.op != EconomicOp::Add {
                return Err(HydrateError::new_spanned(
                    "unit_template resources accept only `_add` literal contributions",
                    Some(entry.key.span.clone()),
                ));
            }
            let amount = read_scalar_f32(entry, &entry.key.text)?;
            let (property_key, arena_name) = ensure_flow_pair(
                &decoded.category,
                &decoded.resource,
                resources,
                arena_defaults,
                used_pairs,
            )?;
            let signed = match decoded.axis {
                EconomicAxis::Produces => amount,
                EconomicAxis::Upkeep => -amount,
                EconomicAxis::Cost => unreachable!("cost rejected above"),
            };
            contributions.push(CategoryFlowContribution {
                category: decoded.category.clone(),
                resource: decoded.resource.clone(),
                axis: decoded.axis,
                property: property_key,
                arena: arena_name.clone(),
                rate: signed,
            });
            push_base_flow_obligation(
                base_obligations,
                &template_id,
                &decoded.category,
                &decoded.resource,
                &arena_name,
                decoded.axis,
                amount,
            )?;
        }
    }
    Ok(())
}

fn push_base_flow_obligation(
    base_obligations: &mut Vec<BaseFlowObligationSpec>,
    template_id: &str,
    category: &str,
    resource: &str,
    arena_name: &str,
    axis: EconomicAxis,
    rate: f32,
) -> Result<(), HydrateError> {
    if !rate.is_finite() || rate < 0.0 {
        return Err(HydrateError::new(format!(
            "base flow rate must be finite and non-negative, got `{rate}`"
        )));
    }
    let (direction, axis_label) = match axis {
        EconomicAxis::Produces => (BaseFlowDirectionSpec::Produce, "produce"),
        EconomicAxis::Upkeep => (BaseFlowDirectionSpec::Upkeep, "upkeep"),
        EconomicAxis::Cost => {
            return Err(HydrateError::new(
                "`cost` keys require a discrete ResourceEconomySpec context",
            ));
        }
    };
    base_obligations.push(BaseFlowObligationSpec {
        id: format!("{template_id}_{category}_{resource}_{axis_label}"),
        arena: arena_name.into(),
        install: InstallTargetSpec::ScenarioListed {
            target_id: template_id.into(),
        },
        direction,
        rate,
    });
    Ok(())
}

fn parse_modifier_block(
    property: &RawProperty,
    categories: &BTreeMap<String, CategoryEntry>,
    resources: &BTreeMap<String, ResourceEntry>,
    category_names: &[String],
    resource_names: &[String],
    arena_defaults: &ArenaDefaults,
    used_pairs: &mut BTreeMap<(String, String), (PropertySpec, ArenaSpec)>,
    decoded_modifier_keys: &mut Vec<DecodedEconomicKey>,
) -> Result<Vec<OverlaySpec>, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`modifier` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut id = None;
    let mut overlays = Vec::new();

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "display_name" => {
                read_scalar_text(field, "display_name")?;
            }
            "triggered_produces_modifier" => {
                return Err(HydrateError::new_spanned(
                    "triggered/gated generated forms are deferred to CT-2b",
                    Some(field.key.span.clone()),
                ));
            }
            key => {
                let decoded = decode_economic_modifier_key_spanned(
                    key,
                    category_names,
                    resource_names,
                    Some(field.key.span.clone()),
                )?;
                if decoded.axis == EconomicAxis::Cost {
                    return Err(HydrateError::new_spanned(
                        "`cost` modifier keys require a discrete ResourceEconomySpec context",
                        Some(field.key.span.clone()),
                    ));
                }
                let amount = read_scalar_f32(field, key)?;
                let (property_key, _) = ensure_flow_pair(
                    &decoded.category,
                    &decoded.resource,
                    resources,
                    arena_defaults,
                    used_pairs,
                )?;
                let category = categories
                    .get(&decoded.category)
                    .expect("decoder only returns registered category");
                let transform = match decoded.op {
                    EconomicOp::Add => TransformOp::Add(amount),
                    EconomicOp::Mult => TransformOp::Multiply(1.0 + amount),
                };
                let modifier_id = id.clone().unwrap_or_else(|| "ct2c_modifier".into());
                overlays.push(OverlaySpec {
                    id: format!("{modifier_id}_{key}"),
                    display_name: String::new(),
                    targets_property: format!("{}::{}", property_key.namespace, property_key.name),
                    sub_field_deltas: vec![(SubFieldRole::Named("flow".into()), transform)],
                    lifecycle: OverlayLifecycle::Permanent,
                    kind: OverlayKind::Policy,
                    source: OverlaySource::Player,
                    install: InstallTargetSpec::AllOfKind {
                        kind: category.kind.clone(),
                    },
                });
                decoded_modifier_keys.push(decoded);
            }
        }
    }

    if overlays.is_empty() {
        return Err(HydrateError::new_spanned(
            "modifier requires at least one economic modifier key",
            Some(property.key.span.clone()),
        ));
    }
    Ok(overlays)
}

fn ensure_flow_pair(
    category: &str,
    resource: &str,
    resources: &BTreeMap<String, ResourceEntry>,
    arena_defaults: &ArenaDefaults,
    used_pairs: &mut BTreeMap<(String, String), (PropertySpec, ArenaSpec)>,
) -> Result<(PropertyKey, String), HydrateError> {
    if !used_pairs.contains_key(&(category.into(), resource.into())) {
        let resource_entry = resources
            .get(resource)
            .ok_or_else(|| unregistered_resource_error(resource, None, resources.keys()))?;
        let arena_name = format!("{category}_{resource}");
        let property_name = format!("{category}_{resource}_flow");
        let property_key = PropertyKey::new(&resource_entry.namespace, &property_name);
        let property =
            build_flow_property_spec(category, resource_entry, &property_name, &arena_name);
        let arena = ArenaSpec {
            name: arena_name,
            flow_property: property_key.clone(),
            balance_property: None,
            max_participants: arena_defaults.max_participants,
            max_coupling_fanout: arena_defaults.max_coupling_fanout,
            max_orderband_depth: arena_defaults.max_orderband_depth,
            fission_policy: FissionPolicySpec::Reject,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 0,
            expected_max_children_per_intermediate: 0,
            explicit_participants: Vec::new(),
            enrollment: None,
            wildcard_admission: None,
        };
        if used_pairs
            .values()
            .any(|(_, existing)| existing.name == arena.name)
        {
            return Err(HydrateError::new(format!(
                "arena name collision for `{}`",
                arena.name
            )));
        }
        used_pairs.insert((category.into(), resource.into()), (property, arena));
    }
    let (property, arena) = used_pairs
        .get(&(category.into(), resource.into()))
        .expect("flow pair inserted above");
    Ok((
        PropertyKey::new(&property.namespace, &property.name),
        arena.name.clone(),
    ))
}

fn decode_economic_modifier_key_spanned(
    key: &str,
    categories: &[String],
    resources: &[String],
    span: Option<crate::raw::RawSpan>,
) -> Result<DecodedEconomicKey, HydrateError> {
    if key.starts_with("shipsize_") {
        return Err(HydrateError::new_spanned(
            "shipsize grammar family is not admitted by CT-2c",
            span,
        ));
    }
    if key.contains("triggered_produces_modifier") {
        return Err(HydrateError::new_spanned(
            "triggered/gated generated forms are deferred to CT-2b",
            span,
        ));
    }
    let Some((stem, op)) = key.rsplit_once('_') else {
        return Err(HydrateError::new_spanned(
            "missing op suffix `_add`/`_mult`",
            span,
        ));
    };
    let op = match op {
        "add" => EconomicOp::Add,
        "mult" => EconomicOp::Mult,
        _ => {
            return Err(HydrateError::new_spanned(
                "missing op suffix `_add`/`_mult`",
                span,
            ));
        }
    };
    let Some((pair, axis)) = stem.rsplit_once('_') else {
        return Err(HydrateError::new_spanned(
            "missing flow axis `_produces`/`_upkeep`/`_cost`",
            span,
        ));
    };
    let axis = match axis {
        "produces" => EconomicAxis::Produces,
        "upkeep" => EconomicAxis::Upkeep,
        "cost" => EconomicAxis::Cost,
        _ => {
            return Err(HydrateError::new_spanned(
                "missing flow axis `_produces`/`_upkeep`/`_cost`",
                span,
            ));
        }
    };

    let mut matches = Vec::new();
    for category in categories {
        for resource in resources {
            if pair == format!("{category}_{resource}") {
                matches.push((category.clone(), resource.clone()));
            }
        }
    }
    if matches.is_empty() {
        if let Some(resource) = resources
            .iter()
            .filter(|resource| pair.ends_with(&format!("_{resource}")))
            .max_by_key(|resource| resource.len())
        {
            let category = pair.strip_suffix(&format!("_{resource}")).unwrap_or(pair);
            return Err(unmapped_category_error(category, span));
        }
        if let Some(category) = categories
            .iter()
            .filter(|category| pair.starts_with(&format!("{category}_")))
            .max_by_key(|category| category.len())
        {
            let resource = pair.strip_prefix(&format!("{category}_")).unwrap_or(pair);
            return Err(unregistered_resource_error(
                resource,
                span,
                resources.iter(),
            ));
        }
        return Err(HydrateError::new_spanned(
            format!("unknown economic modifier key `{key}`"),
            span,
        ));
    }

    let longest_category = matches
        .iter()
        .map(|(category, _)| category.len())
        .max()
        .unwrap_or(0);
    let best = matches
        .into_iter()
        .filter(|(category, _)| category.len() == longest_category)
        .collect::<Vec<_>>();
    if best.len() != 1 {
        return Err(HydrateError::new_spanned(
            format!("ambiguous economic modifier key `{key}`"),
            span,
        ));
    }
    let (category, resource) = best.into_iter().next().unwrap();
    Ok(DecodedEconomicKey {
        category,
        resource,
        axis,
        op,
    })
}

fn merge_category_map(
    property: &RawProperty,
    categories: &mut BTreeMap<String, CategoryEntry>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`category_map` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    for entry in &block.properties {
        let RawValue::Block(body) = &entry.value else {
            return Err(HydrateError::new_spanned(
                "category_map entry must be a block",
                Some(entry.key.span.clone()),
            ));
        };
        let mut kind = None;
        let mut depth = None;
        for field in &body.properties {
            match field.key.text.as_str() {
                "kind" => kind = Some(read_scalar_text(field, "kind")?),
                "depth" => depth = Some(read_scalar_u32(field, "depth")?),
                other => {
                    return Err(HydrateError::new_spanned(
                        format!("unsupported category_map field `{other}`"),
                        Some(field.key.span.clone()),
                    ));
                }
            }
        }
        categories.insert(
            entry.key.text.clone(),
            CategoryEntry {
                kind: require_field(kind, "kind", entry)?,
            },
        );
        require_field(depth, "depth", entry)?;
    }
    Ok(())
}

fn parse_resource_block(property: &RawProperty) -> Result<ResourceEntry, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`resource` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut id = None;
    let mut namespace = None;
    let mut name = None;
    let mut display_name = String::new();
    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "name" => name = Some(read_scalar_text(field, "name")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported resource field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    require_field(id, "id", property)?;
    Ok(ResourceEntry {
        namespace: require_field(namespace, "namespace", property)?,
        name: require_field(name, "name", property)?,
        display_name,
    })
}

fn parse_resource_flow_defaults(property: &RawProperty) -> Result<ArenaDefaults, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`resource_flow` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut defaults = ArenaDefaults::default();
    for field in &block.properties {
        match field.key.text.as_str() {
            "opt_in" => defaults.opt_in_mode = parse_flow_opt_in(field)?,
            "max_participants" => {
                defaults.max_participants = read_scalar_u32(field, "max_participants")?
            }
            "max_coupling_fanout" => {
                defaults.max_coupling_fanout = read_scalar_u32(field, "max_coupling_fanout")?;
            }
            "max_orderband_depth" => {
                defaults.max_orderband_depth = read_scalar_u32(field, "max_orderband_depth")?;
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported resource_flow field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(defaults)
}

fn parse_flow_opt_in(property: &RawProperty) -> Result<ResourceFlowOptInMode, HydrateError> {
    let text = read_scalar_text(property, "opt_in")?;
    match text.as_str() {
        "FlatStarOptIn" => Ok(ResourceFlowOptInMode::FlatStarOptIn),
        "Disabled" => Ok(ResourceFlowOptInMode::Disabled),
        other => Err(HydrateError::new_spanned(
            format!("`opt_in` must be `FlatStarOptIn` or `Disabled`, got `{other}`"),
            Some(property.key.span.clone()),
        )),
    }
}

fn parse_resource_economy_block(
    property: &RawProperty,
) -> Result<ResourceEconomyOptInMode, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`resource_economy` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut opt_in_mode = ResourceEconomyOptInMode::Disabled;
    for field in &block.properties {
        match field.key.text.as_str() {
            "opt_in" => {
                let text = read_scalar_text(field, "opt_in")?;
                opt_in_mode = match text.as_str() {
                    "Disabled" => ResourceEconomyOptInMode::Disabled,
                    "TransferOnly" => ResourceEconomyOptInMode::TransferOnly,
                    "EmissionOnly" => ResourceEconomyOptInMode::EmissionOnly,
                    "TransferAndEmission" => ResourceEconomyOptInMode::TransferAndEmission,
                    other => {
                        return Err(HydrateError::new_spanned(
                            format!("unsupported resource_economy opt_in `{other}`"),
                            Some(field.key.span.clone()),
                        ));
                    }
                };
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported resource_economy field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(opt_in_mode)
}

fn parse_daily_property_block(property: &RawProperty) -> Result<PropertySpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`property` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut id = None;
    let mut namespace = None;
    let mut name = None;
    let mut display_name = String::new();
    let mut default = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "name" => name = Some(read_scalar_text(field, "name")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "default" => default = Some(read_scalar_f32(field, "default")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported property field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(PropertySpec {
        id: require_field(id, "id", property)?,
        namespace: require_field(namespace, "namespace", property)?,
        name: require_field(name, "name", property)?,
        display_name,
        description: String::new(),
        sub_fields: vec![amount_subfield(default.unwrap_or(0.0))],
    })
}

fn parse_transfer_block(property: &RawProperty) -> Result<ResourceTransferSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`transfer` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut id = None;
    let mut source = None;
    let mut target = None;
    let mut amount = None;
    let mut order_band = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "source" => source = Some(parse_property_key(field)?),
            "target" => target = Some(parse_property_key(field)?),
            "amount" => amount = Some(read_scalar_f32(field, "amount")?),
            "order_band" => order_band = Some(read_scalar_u32(field, "order_band")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported transfer field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(ResourceTransferSpec {
        id: require_field(id, "id", property)?,
        source: require_field(source, "source", property)?,
        source_role: amount_role(),
        target: require_field(target, "target", property)?,
        target_role: amount_role(),
        amount: require_field(amount, "amount", property)?,
        order_band: require_field(order_band, "order_band", property)?,
    })
}

fn parse_recipe_block(property: &RawProperty) -> Result<ResourceRecipeSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`recipe` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut id = None;
    let mut inputs = Vec::new();
    let mut target = None;
    let mut throttle = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "input" => inputs.push(parse_recipe_input_block(field)?),
            "target" => target = Some(parse_property_key(field)?),
            "throttle" | "throttle_hint_max_per_tick" => {
                throttle = Some(read_scalar_u32(field, &field.key.text)?)
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported recipe field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    if inputs.is_empty() {
        return Err(HydrateError::new_spanned(
            "recipe requires at least one `input` block",
            Some(property.key.span.clone()),
        ));
    }
    Ok(ResourceRecipeSpec {
        id: require_field(id, "id", property)?,
        inputs,
        target: require_field(target, "target", property)?,
        target_role: amount_role(),
        throttle_hint_max_per_tick: require_field(throttle, "throttle", property)?,
    })
}

fn parse_recipe_input_block(property: &RawProperty) -> Result<RecipeInputSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`input` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut property_key = None;
    let mut unit_cost = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "property" => property_key = Some(parse_property_key(field)?),
            "unit_cost" => unit_cost = Some(read_scalar_f32(field, "unit_cost")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported input field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(RecipeInputSpec {
        property: require_field(property_key, "property", property)?,
        role: amount_role(),
        unit_cost: require_field(unit_cost, "unit_cost", property)?,
    })
}

fn build_flow_property_spec(
    category: &str,
    resource: &ResourceEntry,
    property_name: &str,
    arena_name: &str,
) -> PropertySpec {
    let display_resource = if resource.display_name.is_empty() {
        resource.name.as_str()
    } else {
        resource.display_name.as_str()
    };
    PropertySpec {
        id: format!("{}_{}_{}_flow", resource.namespace, category, resource.name),
        namespace: resource.namespace.clone(),
        name: property_name.into(),
        display_name: format!("{category} {display_resource} flow"),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: arena_name.into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: arena_name.into(),
                },
            ),
        ],
    }
}

fn amount_subfield(default: f32) -> SubFieldSpec {
    SubFieldSpec {
        role: amount_role(),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default,
        display_name: "Amount".into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }
}

fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

fn amount_role() -> SubFieldRole {
    SubFieldRole::Named("amount".into())
}

fn parse_property_key(property: &RawProperty) -> Result<PropertyKey, HydrateError> {
    let text = read_scalar_text(property, &property.key.text)?;
    let Some((namespace, name)) = text.split_once("::") else {
        return Err(HydrateError::new_spanned(
            format!("property reference must be `namespace::name`, got `{text}`"),
            Some(property.key.span.clone()),
        ));
    };
    if namespace.is_empty() || name.is_empty() {
        return Err(HydrateError::new_spanned(
            format!("property reference must be `namespace::name`, got `{text}`"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(PropertyKey::new(namespace, name))
}

fn builtin_categories() -> BTreeMap<String, CategoryEntry> {
    [
        ("country", "Faction", 1),
        ("planet", "Location", 2),
        ("pop", "Cohort", 3),
    ]
    .into_iter()
    .map(|(name, kind, _depth)| (name.into(), CategoryEntry { kind: kind.into() }))
    .collect()
}

fn unmapped_category_error(category: &str, span: Option<crate::raw::RawSpan>) -> HydrateError {
    HydrateError::new_spanned(
        format!(
            "category `{category}` is unmapped; add `category_map {{ {category} = {{ kind = Cohort depth = 1 }} }}`"
        ),
        span,
    )
}

fn unregistered_resource_error<'a>(
    resource: &str,
    span: Option<crate::raw::RawSpan>,
    registered: impl IntoIterator<Item = &'a String>,
) -> HydrateError {
    let list = registered
        .into_iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join(", ");
    HydrateError::new_spanned(
        format!("resource `{resource}` is not registered; registered resources: [{list}]"),
        span,
    )
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
            format!("`{field}` must be a numeric literal, got `{text}`"),
            Some(property.key.span.clone()),
        )
    })
}

fn read_scalar_u32(property: &RawProperty, field: &str) -> Result<u32, HydrateError> {
    let text = read_scalar_text(property, field)?;
    text.parse::<u32>().map_err(|_| {
        HydrateError::new_spanned(
            format!("`{field}` must be a non-negative integer literal, got `{text}`"),
            Some(property.key.span.clone()),
        )
    })
}

fn require_field<T>(
    value: Option<T>,
    field: &str,
    property: &RawProperty,
) -> Result<T, HydrateError> {
    value.ok_or_else(|| {
        HydrateError::new_spanned(
            format!("missing required field `{field}`"),
            Some(property.key.span.clone()),
        )
    })
}
