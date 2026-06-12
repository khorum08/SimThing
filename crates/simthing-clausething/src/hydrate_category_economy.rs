//! CT-2c economic-category hydration into existing Resource Flow and ResourceEconomy authoring.
//!
//! Categories are parsed as ClauseThing-side admission metadata only. The emitted
//! [`GameModeSpec`] contains ordinary properties, overlays, Resource Flow arenas,
//! base-flow obligations, and ResourceEconomy registrations; no category runtime artifact is produced.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, LogTier, SubFieldRole, SubFieldSpec,
};
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::region_field::{
    ArenaPressureBindingSpec, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
    MappingExecutionProfile, PressurePlacementSpec, PressureSourceSpec, RegionFieldCadenceSpec,
    RegionFieldFormulaBindingSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
    RegionFieldReductionSpec, RegionFieldSpec,
};
use simthing_spec::spec::resource_economy::{
    RecipeInputSpec, ResourceEconomyOptInMode, ResourceEconomySpec, ResourceRecipeSpec,
    ResourceTransferSpec,
};
use simthing_spec::spec::resource_flow::{
    BaseFlowDirectionSpec, BaseFlowObligationSpec, GatedRateOpSpec, GatedRateSpec,
    GatedRateTriggerSpec, RateFormulaOp, RateFormulaOpSpec, RateFormulaOperandSpec,
    RateFormulaSpec, ResourceFlowOptInMode, ResourceFlowSpec,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    #[allow(dead_code)] // application-level kind; used by CT-3b+4a binding hydration
    kind: String,
    depth: u32,
    parent: Option<String>,
}

#[derive(Debug, Clone)]
struct ResourceEntry {
    namespace: String,
    name: String,
    display_name: String,
}

/// One template-authored literal `_add` base rate, pre-fold.
#[derive(Debug, Clone)]
struct BaseRateRow {
    template_id: String,
    category: String,
    resource: String,
    axis: EconomicAxis,
    arena: String,
    base: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ModifierFoldKey {
    category: String,
    resource: String,
    axis: EconomicAxis,
}

#[derive(Debug, Clone, Default)]
struct ModifierFold {
    mult_sum: f32,
    add_sum: f32,
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
    let mut region_fields = Vec::new();
    let mut mapping_profile = MappingExecutionProfile::Disabled;
    let mut trigger_properties = Vec::new();
    let mut script_values: BTreeMap<String, RateFormulaSpec> = BTreeMap::new();

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
            "region_field" => region_fields.push(parse_region_field_block(property)?),
            "mapping" => mapping_profile = parse_mapping_block(property)?,
            "trigger_property" => trigger_properties.push(parse_trigger_property_block(property)?),
            "script_value" => {
                let (id, formula) = parse_script_value_block(property)?;
                if script_values.insert(id, formula).is_some() {
                    return Err(HydrateError::new_spanned(
                        "duplicate script_value id",
                        Some(property.key.span.clone()),
                    ));
                }
            }
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

    validate_category_table(&categories)?;

    let category_names = categories.keys().cloned().collect::<Vec<_>>();
    let resource_names = resources.keys().cloned().collect::<Vec<_>>();
    let mut used_pairs: BTreeMap<(String, String), (PropertySpec, ArenaSpec)> = BTreeMap::new();
    let mut contributions = Vec::new();
    let mut base_rates = Vec::new();
    let mut decoded_modifier_keys = Vec::new();
    let mut folds: BTreeMap<ModifierFoldKey, ModifierFold> = BTreeMap::new();
    let mut gated_rates = Vec::new();
    let mut gated_pairs: BTreeSet<(String, String)> = BTreeSet::new();

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
            &mut base_rates,
            &mut gated_rates,
            &mut gated_pairs,
            &script_values,
        )?;
    }

    for modifier in modifier_blocks {
        parse_modifier_folds(
            modifier,
            &categories,
            &category_names,
            &resource_names,
            &mut folds,
            &mut decoded_modifier_keys,
        )?;
    }

    let base_obligations = apply_modifier_folds(&base_rates, &folds, &categories)?;

    // CT-RF-EML-RATE-0: gated pairs carry the immutable `rate_base` column
    // the per-tick effective-rate band recomputes the intrinsic column from.
    for pair in &gated_pairs {
        if let Some((property, _)) = used_pairs.get_mut(pair) {
            property.sub_fields.push(rate_base_subfield());
        }
    }

    let (mut properties, arenas): (Vec<_>, Vec<_>) = used_pairs.into_values().unzip();
    properties.extend(trigger_properties);

    Ok(HydratedCategoryEconomyPack {
        game_mode: GameModeSpec {
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
            resource_flow: Some(ResourceFlowSpec {
                opt_in_mode: arena_defaults.opt_in_mode,
                arenas,
                couplings: vec![],
                base_obligations,
                gated_rates,
            }),
            resource_economy: None,
            resource_flow_execution_profile: Default::default(),
            region_fields,
            mapping_execution_profile: mapping_profile,
        },
        contributions,
        decoded_modifier_keys,
    })
}

/// CT-3b+4a: `region_field { … }` → [`RegionFieldSpec`]. Designer authors the
/// physical knobs; the slot/column layout is derived mechanically from
/// `grid_size` to match the first-slice mapping runtime contract
/// (field col 0; EML resource/wa/wb/urgency cols 1–4; parent slot = grid²).
fn parse_region_field_block(property: &RawProperty) -> Result<RegionFieldSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`region_field` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut name = None;
    let mut grid_size = None;
    let mut horizon = None;
    let mut alpha_self = None;
    let mut gamma_neighbor = None;
    let mut cadence = None;
    let mut urgency = None;
    let mut pressure_binding = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "name" => name = Some(read_scalar_text(field, "name")?),
            "grid_size" => grid_size = Some(read_scalar_u32(field, "grid_size")?),
            "horizon" => horizon = Some(read_scalar_u32(field, "horizon")?),
            "alpha_self" => alpha_self = Some(read_scalar_f32(field, "alpha_self")?),
            "gamma_neighbor" => gamma_neighbor = Some(read_scalar_f32(field, "gamma_neighbor")?),
            "cadence" => {
                let text = read_scalar_text(field, "cadence")?;
                cadence = Some(match text.as_str() {
                    "EveryTick" => RegionFieldCadenceSpec::EveryTick,
                    "OnEvent" => RegionFieldCadenceSpec::OnEvent,
                    other => {
                        return Err(HydrateError::new_spanned(
                            format!("unsupported cadence `{other}` (EveryTick or OnEvent)"),
                            Some(field.key.span.clone()),
                        ));
                    }
                });
            }
            "urgency" => urgency = Some(parse_urgency_block(field)?),
            "pressure_binding" => pressure_binding = Some(parse_pressure_binding_block(field)?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported region_field field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let grid_size = require_field(grid_size, "grid_size", property)?;
    let cell_count = grid_size * grid_size;
    let (weights, threshold, event_kind) = require_field(urgency, "urgency", property)?;

    Ok(RegionFieldSpec {
        name: require_field(name, "name", property)?,
        grid_size,
        n_dims: 5,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::Normalized,
        horizon: require_field(horizon, "horizon", property)?,
        allow_extended_horizon: false,
        alpha_self: require_field(alpha_self, "alpha_self", property)?,
        gamma_neighbor: require_field(gamma_neighbor, "gamma_neighbor", property)?,
        source_cap: None,
        source_policy: Default::default(),
        cadence: require_field(cadence, "cadence", property)?,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: Some(RegionFieldReductionSpec {
            child_slot_start: 0,
            child_slot_count: cell_count,
            child_col: 0,
            parent_slot: cell_count,
            parent_col: 0,
            order_band: 0,
        }),
        parent_formula: Some(RegionFieldFormulaBindingSpec {
            formula_class: "field_urgency".into(),
            tree_id: Some(1),
            weight_pressure: Some(weights.0),
            weight_resource: Some(weights.1),
        }),
        commitment: Some(FirstSliceCommitmentSpec {
            source_formula_class: "field_urgency".into(),
            parent_slot: cell_count,
            urgency_col: 4,
            threshold,
            direction: FirstSliceCommitmentDirectionSpec::Upward,
            event_kind,
        }),
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding,
    })
}

fn parse_urgency_block(property: &RawProperty) -> Result<((f32, f32), f32, u32), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`urgency` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut weight_pressure = None;
    let mut weight_resource = None;
    let mut threshold = None;
    let mut event_kind = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "weight_pressure" => weight_pressure = Some(read_scalar_f32(field, "weight_pressure")?),
            "weight_resource" => weight_resource = Some(read_scalar_f32(field, "weight_resource")?),
            "threshold" => threshold = Some(read_scalar_f32(field, "threshold")?),
            "event_kind" => event_kind = Some(read_scalar_u32(field, "event_kind")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported urgency field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok((
        (
            require_field(weight_pressure, "weight_pressure", property)?,
            require_field(weight_resource, "weight_resource", property)?,
        ),
        require_field(threshold, "threshold", property)?,
        require_field(event_kind, "event_kind", property)?,
    ))
}

fn parse_pressure_binding_block(
    property: &RawProperty,
) -> Result<ArenaPressureBindingSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`pressure_binding` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut arena = None;
    let mut source = None;
    let mut placements = Vec::new();
    for field in &block.properties {
        match field.key.text.as_str() {
            "arena" => arena = Some(read_scalar_text(field, "arena")?),
            "source" => {
                let text = read_scalar_text(field, "source")?;
                source = Some(match text.as_str() {
                    "IntrinsicFlow" => PressureSourceSpec::IntrinsicFlow,
                    "AllocatedFlow" => PressureSourceSpec::AllocatedFlow,
                    other => {
                        return Err(HydrateError::new_spanned(
                            format!("unsupported pressure source `{other}`"),
                            Some(field.key.span.clone()),
                        ));
                    }
                });
            }
            "placement" => {
                let RawValue::Block(body) = &field.value else {
                    return Err(HydrateError::new_spanned(
                        "`placement` must be a block",
                        Some(field.key.span.clone()),
                    ));
                };
                let mut target = None;
                let mut row = None;
                let mut col = None;
                for entry in &body.properties {
                    match entry.key.text.as_str() {
                        "target" => target = Some(read_scalar_text(entry, "target")?),
                        "row" => row = Some(read_scalar_u32(entry, "row")?),
                        "col" => col = Some(read_scalar_u32(entry, "col")?),
                        other => {
                            return Err(HydrateError::new_spanned(
                                format!("unsupported placement field `{other}`"),
                                Some(entry.key.span.clone()),
                            ));
                        }
                    }
                }
                placements.push(PressurePlacementSpec {
                    target_id: require_field(target, "target", field)?,
                    row: require_field(row, "row", field)?,
                    col: require_field(col, "col", field)?,
                });
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported pressure_binding field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(ArenaPressureBindingSpec {
        arena: require_field(arena, "arena", property)?,
        source: require_field(source, "source", property)?,
        placements,
    })
}

fn parse_mapping_block(property: &RawProperty) -> Result<MappingExecutionProfile, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`mapping` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut profile = MappingExecutionProfile::Disabled;
    for field in &block.properties {
        match field.key.text.as_str() {
            "profile" => {
                let text = read_scalar_text(field, "profile")?;
                profile = match text.as_str() {
                    "Disabled" => MappingExecutionProfile::Disabled,
                    "SparseRegionFieldV1" => MappingExecutionProfile::SparseRegionFieldV1,
                    other => {
                        return Err(HydrateError::new_spanned(
                            format!("unsupported mapping profile `{other}`"),
                            Some(field.key.span.clone()),
                        ));
                    }
                };
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported mapping field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(profile)
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
    base_rates: &mut Vec<BaseRateRow>,
    gated_rates: &mut Vec<GatedRateSpec>,
    gated_pairs: &mut BTreeSet<(String, String)>,
    script_values: &BTreeMap<String, RateFormulaSpec>,
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
            if entry.key.text == "gated" {
                parse_gated_entry(
                    entry,
                    &template_id,
                    &category,
                    axis,
                    categories,
                    resources,
                    category_names,
                    resource_names,
                    arena_defaults,
                    used_pairs,
                    gated_rates,
                    gated_pairs,
                    script_values,
                )?;
                continue;
            }
            if entry.key.text == "triggered_produces_modifier" || entry.key.text == "trigger" {
                return Err(HydrateError::new_spanned(
                    "bare triggered forms are not authorable; use a `gated { trigger { … } … }` block (CT-RF-EML-RATE-0)",
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
            // `value:` rates are dynamic — they bypass the static fold and
            // become always-on terms on the effective-rate EvalEML band.
            if let RawValue::Scalar(scalar) = &entry.value {
                if scalar.text.starts_with("value:") {
                    let formula =
                        resolve_script_value(&scalar.text, script_values, &entry.key.span)?;
                    let (_, arena_name) = ensure_flow_pair(
                        &decoded.category,
                        &decoded.resource,
                        resources,
                        arena_defaults,
                        used_pairs,
                    )?;
                    gated_pairs.insert((decoded.category.clone(), decoded.resource.clone()));
                    let index = gated_rates.len();
                    gated_rates.push(GatedRateSpec {
                        id: format!(
                            "{template_id}_{}_{}_value{index}",
                            decoded.category, decoded.resource
                        ),
                        arena: arena_name,
                        install: InstallTargetSpec::ScenarioListed {
                            target_id: template_id.clone(),
                        },
                        direction: match decoded.axis {
                            EconomicAxis::Produces => BaseFlowDirectionSpec::Produce,
                            EconomicAxis::Upkeep => BaseFlowDirectionSpec::Upkeep,
                            EconomicAxis::Cost => unreachable!("cost rejected above"),
                        },
                        op: match decoded.op {
                            EconomicOp::Add => GatedRateOpSpec::Add,
                            EconomicOp::Mult => GatedRateOpSpec::Mult,
                        },
                        rate: 0.0,
                        trigger: None,
                        rate_formula: Some(formula),
                    });
                    continue;
                }
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
            if !amount.is_finite() || amount < 0.0 {
                return Err(HydrateError::new_spanned(
                    format!("base flow rate must be finite and non-negative, got `{amount}`"),
                    Some(entry.key.span.clone()),
                ));
            }
            base_rates.push(BaseRateRow {
                template_id: template_id.clone(),
                category: decoded.category.clone(),
                resource: decoded.resource.clone(),
                axis: decoded.axis,
                arena: arena_name.clone(),
                base: amount,
            });
        }
    }
    Ok(())
}

/// CT-2c-REMEDIAL-3 — static category modifiers fold into effective base
/// rates at hydration, per the §6 inheritance asymmetry:
///
/// - `_add` folds match the row's **exact** category (leaf-only, per-producer);
/// - `_mult` folds sweep the row's category **ancestor chain** (subtree
///   broadcast-down, expanded at compile time — never a runtime category walk);
/// - stacking is **additive-in-effect**: `effective = (base + Σadd) × (1 + Σmult)`,
///   summed in deterministic BTreeMap key order. Rates are per-tick flow
///   magnitudes, so install-time folding is exact; a per-tick `Multiply`
///   overlay on a rate column would compound and is rejected as a mechanism.
/// - a fold no authored production consumes is a hard error (no dead modifiers);
/// - a negative or non-finite effective rate is a hard error.
fn apply_modifier_folds(
    base_rates: &[BaseRateRow],
    folds: &BTreeMap<ModifierFoldKey, ModifierFold>,
    categories: &BTreeMap<String, CategoryEntry>,
) -> Result<Vec<BaseFlowObligationSpec>, HydrateError> {
    let mut consumed: BTreeSet<ModifierFoldKey> = BTreeSet::new();
    let mut obligations = Vec::with_capacity(base_rates.len());

    for row in base_rates {
        let chain = category_ancestor_chain(&row.category, categories)?;
        let mut add_sum = 0.0_f32;
        let mut mult_sum = 0.0_f32;
        for (key, fold) in folds {
            if key.resource != row.resource || key.axis != row.axis {
                continue;
            }
            if key.category == row.category {
                add_sum += fold.add_sum;
                consumed.insert(key.clone());
            }
            if chain.iter().any(|ancestor| *ancestor == key.category) {
                mult_sum += fold.mult_sum;
                consumed.insert(key.clone());
            }
        }
        let effective = (row.base + add_sum) * (1.0 + mult_sum);
        if !effective.is_finite() || effective < 0.0 {
            return Err(HydrateError::new(format!(
                "effective {} rate for `{}` on `{}_{}` is `{effective}` after modifier fold — must be finite and non-negative",
                match row.axis {
                    EconomicAxis::Produces => "produce",
                    EconomicAxis::Upkeep => "upkeep",
                    EconomicAxis::Cost => "cost",
                },
                row.template_id,
                row.category,
                row.resource,
            )));
        }
        let (direction, axis_label) = match row.axis {
            EconomicAxis::Produces => (BaseFlowDirectionSpec::Produce, "produce"),
            EconomicAxis::Upkeep => (BaseFlowDirectionSpec::Upkeep, "upkeep"),
            EconomicAxis::Cost => {
                return Err(HydrateError::new(
                    "`cost` keys require a discrete ResourceEconomySpec context",
                ));
            }
        };
        obligations.push(BaseFlowObligationSpec {
            id: format!(
                "{}_{}_{}_{axis_label}",
                row.template_id, row.category, row.resource
            ),
            arena: row.arena.clone(),
            install: InstallTargetSpec::ScenarioListed {
                target_id: row.template_id.clone(),
            },
            direction,
            rate: effective,
        });
    }

    for key in folds.keys() {
        if !consumed.contains(key) {
            return Err(HydrateError::new(format!(
                "modifier key for category `{}` resource `{}` matches no authored production — dead modifiers are rejected",
                key.category, key.resource
            )));
        }
    }
    Ok(obligations)
}

fn parse_modifier_folds(
    property: &RawProperty,
    categories: &BTreeMap<String, CategoryEntry>,
    category_names: &[String],
    resource_names: &[String],
    folds: &mut BTreeMap<ModifierFoldKey, ModifierFold>,
    decoded_modifier_keys: &mut Vec<DecodedEconomicKey>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`modifier` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut decoded_any = false;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" | "display_name" => {
                read_scalar_text(field, field.key.text.as_str())?;
            }
            "triggered_produces_modifier" => {
                return Err(HydrateError::new_spanned(
                    "triggered/gated generated forms are deferred to the EML effective-rate band (CT-RF-EML-RATE-0; consumer CT-3b+4a implementation)",
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
                debug_assert!(
                    categories.contains_key(&decoded.category),
                    "decoder only returns registered categories"
                );
                let amount = read_scalar_f32(field, key)?;
                if !amount.is_finite() {
                    return Err(HydrateError::new_spanned(
                        format!("modifier amount must be finite, got `{amount}`"),
                        Some(field.key.span.clone()),
                    ));
                }
                let fold = folds
                    .entry(ModifierFoldKey {
                        category: decoded.category.clone(),
                        resource: decoded.resource.clone(),
                        axis: decoded.axis,
                    })
                    .or_default();
                match decoded.op {
                    EconomicOp::Add => fold.add_sum += amount,
                    EconomicOp::Mult => fold.mult_sum += amount,
                }
                decoded_modifier_keys.push(decoded);
                decoded_any = true;
            }
        }
    }

    if !decoded_any {
        return Err(HydrateError::new_spanned(
            "modifier requires at least one economic modifier key",
            Some(property.key.span.clone()),
        ));
    }
    Ok(())
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
            "triggered/gated generated forms are deferred to the EML effective-rate band (CT-RF-EML-RATE-0; consumer CT-3b+4a implementation)",
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
        let mut parent = None;
        for field in &body.properties {
            match field.key.text.as_str() {
                "kind" => kind = Some(read_scalar_text(field, "kind")?),
                "depth" => depth = Some(read_scalar_u32(field, "depth")?),
                "parent" => parent = Some(read_scalar_text(field, "parent")?),
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
                depth: require_field(depth, "depth", entry)?,
                parent,
            },
        );
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

/// CT-RF-EML-RATE-0: one `gated { trigger { … } <economic_key> = N }` entry
/// inside a `produces`/`upkeep` block → [`GatedRateSpec`]. The decoded key
/// must match the template's category and the enclosing axis; both `_add`
/// and `_mult` ops are admitted (additive-in-effect for `_mult`).
#[allow(clippy::too_many_arguments)]
fn parse_gated_entry(
    entry: &RawProperty,
    template_id: &str,
    category: &str,
    axis: EconomicAxis,
    categories: &BTreeMap<String, CategoryEntry>,
    resources: &BTreeMap<String, ResourceEntry>,
    category_names: &[String],
    resource_names: &[String],
    arena_defaults: &ArenaDefaults,
    used_pairs: &mut BTreeMap<(String, String), (PropertySpec, ArenaSpec)>,
    gated_rates: &mut Vec<GatedRateSpec>,
    gated_pairs: &mut BTreeSet<(String, String)>,
    script_values: &BTreeMap<String, RateFormulaSpec>,
) -> Result<(), HydrateError> {
    let _ = categories;
    let RawValue::Block(block) = &entry.value else {
        return Err(HydrateError::new_spanned(
            "`gated` must be a block",
            Some(entry.key.span.clone()),
        ));
    };

    let mut trigger = None;
    let mut rate_entry: Option<(DecodedEconomicKey, f32, Option<RateFormulaSpec>)> = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "trigger" => trigger = Some(parse_gated_trigger_block(field)?),
            key => {
                let decoded = decode_economic_modifier_key_spanned(
                    key,
                    category_names,
                    resource_names,
                    Some(field.key.span.clone()),
                )?;
                if decoded.category != category {
                    return Err(HydrateError::new_spanned(
                        format!(
                            "gated key category `{}` does not match unit_template category `{category}`",
                            decoded.category
                        ),
                        Some(field.key.span.clone()),
                    ));
                }
                if decoded.axis != axis {
                    return Err(HydrateError::new_spanned(
                        "gated key axis does not belong in this block",
                        Some(field.key.span.clone()),
                    ));
                }
                if rate_entry.is_some() {
                    return Err(HydrateError::new_spanned(
                        "one economic key per `gated` block; author multiple blocks",
                        Some(field.key.span.clone()),
                    ));
                }
                if let RawValue::Scalar(scalar) = &field.value {
                    if scalar.text.starts_with("value:") {
                        let formula =
                            resolve_script_value(&scalar.text, script_values, &field.key.span)?;
                        rate_entry = Some((decoded, 0.0, Some(formula)));
                        continue;
                    }
                }
                let amount = read_scalar_f32(field, key)?;
                if !amount.is_finite() || amount < 0.0 {
                    return Err(HydrateError::new_spanned(
                        format!("gated rate must be finite and non-negative, got `{amount}`"),
                        Some(field.key.span.clone()),
                    ));
                }
                rate_entry = Some((decoded, amount, None));
            }
        }
    }

    let trigger = trigger.ok_or_else(|| {
        HydrateError::new_spanned(
            "gated block requires `trigger`",
            Some(entry.key.span.clone()),
        )
    })?;
    let (decoded, amount, rate_formula) = rate_entry.ok_or_else(|| {
        HydrateError::new_spanned(
            "gated block requires one economic key",
            Some(entry.key.span.clone()),
        )
    })?;

    let (_, arena_name) = ensure_flow_pair(
        &decoded.category,
        &decoded.resource,
        resources,
        arena_defaults,
        used_pairs,
    )?;
    gated_pairs.insert((decoded.category.clone(), decoded.resource.clone()));

    let direction = match decoded.axis {
        EconomicAxis::Produces => BaseFlowDirectionSpec::Produce,
        EconomicAxis::Upkeep => BaseFlowDirectionSpec::Upkeep,
        EconomicAxis::Cost => {
            return Err(HydrateError::new_spanned(
                "`cost` keys require a discrete ResourceEconomySpec context",
                Some(entry.key.span.clone()),
            ));
        }
    };
    let op = match decoded.op {
        EconomicOp::Add => GatedRateOpSpec::Add,
        EconomicOp::Mult => GatedRateOpSpec::Mult,
    };
    let index = gated_rates.len();
    gated_rates.push(GatedRateSpec {
        id: format!(
            "{template_id}_{}_{}_gated{index}",
            decoded.category, decoded.resource
        ),
        arena: arena_name,
        install: InstallTargetSpec::ScenarioListed {
            target_id: template_id.into(),
        },
        direction,
        op,
        rate: amount,
        trigger: Some(trigger),
        rate_formula,
    });
    Ok(())
}

/// Resolve a `value:NAME` scalar against the fixture's `script_value`
/// definitions. Flat in v1 — recursion is rejected at parse (formulas hold
/// only literal/property operands, so reference cycles cannot be authored).
fn resolve_script_value(
    text: &str,
    script_values: &BTreeMap<String, RateFormulaSpec>,
    span: &crate::raw::RawSpan,
) -> Result<RateFormulaSpec, HydrateError> {
    let name = text.strip_prefix("value:").unwrap_or(text);
    script_values.get(name).cloned().ok_or_else(|| {
        HydrateError::new_spanned(format!("unknown script_value `{name}`"), Some(span.clone()))
    })
}

/// `script_value { id base add/mult/floor_at/ceil_at … }` → [`RateFormulaSpec`].
/// Operands: scalar literal, or `{ property = ns::name }` for a per-tick
/// column read. Ordered in source order (list-collect duplication policy).
fn parse_script_value_block(
    property: &RawProperty,
) -> Result<(String, RateFormulaSpec), HydrateError> {
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
            RawValue::Scalar(scalar) => {
                if scalar.text.starts_with("value:") {
                    return Err(HydrateError::new_spanned(
                        "script_value recursion is not admitted in v1 (flat formulas only)",
                        Some(field.key.span.clone()),
                    ));
                }
                RateFormulaOperandSpec::Literal(read_scalar_f32(field, &field.key.text)?)
            }
            RawValue::Block(operand_block) => {
                let mut key = None;
                for entry in &operand_block.properties {
                    match entry.key.text.as_str() {
                        "property" => {
                            let text = read_scalar_text(entry, "property")?;
                            let Some((namespace, name)) = text.split_once("::") else {
                                return Err(HydrateError::new_spanned(
                                    format!(
                                        "operand `property` must be `namespace::name`, got `{text}`"
                                    ),
                                    Some(entry.key.span.clone()),
                                ));
                            };
                            key = Some(PropertyKey::new(namespace, name));
                        }
                        other => {
                            return Err(HydrateError::new_spanned(
                                format!("unsupported operand field `{other}`"),
                                Some(entry.key.span.clone()),
                            ));
                        }
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
                    "operand must be a scalar or `{ property = ns::name }`",
                    Some(field.key.span.clone()),
                ));
            }
        };
        ops.push(RateFormulaOpSpec { op, operand });
    }

    Ok((
        id.ok_or_else(|| {
            HydrateError::new_spanned(
                "script_value requires `id`",
                Some(property.key.span.clone()),
            )
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

fn parse_gated_trigger_block(property: &RawProperty) -> Result<GatedRateTriggerSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`trigger` must be a block",
            Some(property.key.span.clone()),
        ));
    };
    let mut property_key = None;
    let mut at_least = None;
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
                    format!("unsupported trigger field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(GatedRateTriggerSpec {
        property: property_key.ok_or_else(|| {
            HydrateError::new_spanned(
                "trigger requires `property`",
                Some(property.key.span.clone()),
            )
        })?,
        at_least: at_least.ok_or_else(|| {
            HydrateError::new_spanned(
                "trigger requires `at_least`",
                Some(property.key.span.clone()),
            )
        })?,
    })
}

/// Plain registered property carrying the gate's watched Amount value.
fn parse_trigger_property_block(property: &RawProperty) -> Result<PropertySpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`trigger_property` must be a block",
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
                    format!("unsupported trigger_property field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    let missing = |what: &str| {
        HydrateError::new_spanned(
            format!("trigger_property requires `{what}`"),
            Some(property.key.span.clone()),
        )
    };
    Ok(PropertySpec {
        id: id.ok_or_else(|| missing("id"))?,
        namespace: namespace.ok_or_else(|| missing("namespace"))?,
        name: name.ok_or_else(|| missing("name"))?,
        display_name,
        description: String::new(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: ClampBehavior::Unbounded,
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

fn rate_base_subfield() -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named("rate_base".into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "rate_base".into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }
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
        ("country", "Faction", 1, None),
        ("planet", "Location", 2, Some("country")),
        ("pop", "Cohort", 3, Some("planet")),
    ]
    .into_iter()
    .map(|(name, kind, depth, parent)| {
        (
            name.into(),
            CategoryEntry {
                kind: kind.into(),
                depth,
                parent: parent.map(Into::into),
            },
        )
    })
    .collect()
}

/// Walk `category` plus its ancestor chain (self first), erroring on unknown
/// parents or cycles. Inheritance asymmetry consumes this: `_mult` folds
/// sweep the chain; `_add` folds match the exact category only.
fn category_ancestor_chain(
    category: &str,
    categories: &BTreeMap<String, CategoryEntry>,
) -> Result<Vec<String>, HydrateError> {
    let mut chain = vec![category.to_string()];
    let mut current = category;
    while let Some(parent) = categories
        .get(current)
        .and_then(|entry| entry.parent.as_deref())
    {
        if chain.iter().any(|seen| seen == parent) {
            return Err(HydrateError::new(format!(
                "category parent cycle through `{parent}`"
            )));
        }
        if !categories.contains_key(parent) {
            return Err(HydrateError::new(format!(
                "category `{current}` names unknown parent `{parent}`"
            )));
        }
        chain.push(parent.to_string());
        current = parent;
    }
    Ok(chain)
}

fn validate_category_table(
    categories: &BTreeMap<String, CategoryEntry>,
) -> Result<(), HydrateError> {
    for (name, entry) in categories {
        let chain = category_ancestor_chain(name, categories)?;
        if let Some(parent) = chain.get(1) {
            let parent_depth = categories[parent.as_str()].depth;
            if parent_depth >= entry.depth {
                return Err(HydrateError::new(format!(
                    "category `{name}` (depth {}) must be deeper than parent `{parent}` (depth {parent_depth}) — broadcast is down-only",
                    entry.depth
                )));
            }
        }
    }
    Ok(())
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
