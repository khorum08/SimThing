//! Scenario-agnostic field-economy hydration.
//!
//! This lowers authoring blocks onto existing spec surfaces only: `PropertySpec`,
//! `OverlaySpec`, `ResourceEconomySpec`, and `EmlGadgetStackSpec`.
//!
//! Production output coefficients and local-flow suppression couplings consume
//! the existing conjunctive-transfer output scale and OrderBand surfaces. RF-5A
//! discharges need-binding authoring via `need_binding` (semantic
//! entity/property/role → full-cell admission).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use simthing_core::{
    ClampBehavior, OverlayKind, OverlayLifecycle, OverlaySource, PlacedParticipantValidationError,
    SimThingKind, StructuralCoord, StructuralGridPlacement, SubFieldRole, SubFieldSpec, TransformOp,
    validate_location_ids_have_structural_placements,
};
use simthing_spec::spec::eml_gadget::{EmlGadgetInstanceSpec, EmlGadgetStackSpec};
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::need_binding::{NeedBindingSpec, SemanticPropertyLocusSpec};
use simthing_spec::spec::overlay::OverlaySpec;
use simthing_spec::spec::property::PropertySpec;
use simthing_spec::spec::resource_economy::{
    EmissionFormulaSpec, EmitBufferSpec, EmitOnThresholdSpec, RecipeInputSpec,
    ResourceEconomyOptInMode, ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec,
    ResourceTransferSpec,
};
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::spec::trigger::TriggerDirection;

use crate::error::HydrateError;
use crate::hydrate_scenario::{
    HydratedScenarioGridMetadata, HydratedScenarioNode, HydratedScenarioOwner,
    header_or_block_body, read_scalar_f32, read_scalar_text, read_scalar_u32, require_block,
};
use crate::raw::{RawProperty, RawSpan};

const DEFAULT_NAMESPACE: &str = "field_economy";
const ADMITTED_WEIGHT_PROFILES: &[&str] = &[
    "expansion-need",
    "disruption-need",
    "manufacturing-need",
    "opportunity",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedFieldEconomy {
    pub id: String,
    pub namespace: String,
    pub production_buildings: Vec<HydratedProductionBuilding>,
    pub flow_couplings: Vec<HydratedFlowCoupling>,
    pub stockpile_silos: Vec<HydratedStockpileSilo>,
    pub field_resource_quantities: Vec<HydratedFieldResourceQuantity>,
    pub disruption_presences: Vec<HydratedDisruptionPresence>,
    pub owner_policy_overlays: Vec<HydratedOwnerPolicyOverlay>,
    pub weight_profiles: Vec<HydratedFieldEconomyWeightProfile>,
    /// RF-5A semantic need bindings (profile-id join + entity/property/role loci).
    pub need_bindings: Vec<NeedBindingSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedProductionBuilding {
    pub id: String,
    pub location: String,
    pub input_resource: String,
    pub input_amount: f32,
    pub output_resource: String,
    pub output_coefficient: f32,
    pub throttle_hint_max_per_tick: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedFlowCoupling {
    pub id: String,
    pub source_location: String,
    pub source_resource: String,
    pub source_unit_cost: f32,
    pub pressure_location: String,
    pub pressure_resource: String,
    pub pressure_unit_cost: f32,
    pub weight_owner: String,
    pub weight_resource: String,
    pub weight_unit_cost: f32,
    pub sink_location: String,
    pub sink_resource: String,
    pub output_coefficient: f32,
    pub order_band: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedStockpileSilo {
    pub id: String,
    pub owner: String,
    pub resource: String,
    pub current: f32,
    /// Owner field clause token for spanned host diagnostics.
    #[serde(skip)]
    pub owner_span_token: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedFieldResourceQuantity {
    pub id: String,
    pub location: String,
    pub resource: String,
    pub amount: f32,
    /// Location field clause token for spanned host diagnostics.
    #[serde(skip)]
    pub location_span_token: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedDisruptionPresence {
    pub id: String,
    pub location: String,
    pub resource: String,
    pub amount: f32,
    pub threshold: f32,
    pub event_kind: u32,
    pub direction: TriggerDirection,
    /// Location field clause token for spanned host diagnostics.
    #[serde(skip)]
    pub location_span_token: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedOwnerPolicyOverlay {
    pub id: String,
    pub owner: String,
    pub targets_property: String,
    pub transform: HydratedOwnerPolicyTransform,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HydratedOwnerPolicyTransform {
    Add(f32),
    Multiply(f32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedFieldEconomyWeightProfile {
    pub id: String,
    pub profile: String,
    pub stack: EmlGadgetStackSpec,
}

#[derive(Debug, Clone)]
pub struct FieldEconomyLowering {
    pub hydrated: HydratedFieldEconomy,
    pub properties: Vec<PropertySpec>,
    pub overlays: Vec<OverlaySpec>,
    pub resource_economy: ResourceEconomySpec,
    pub need_bindings: Vec<NeedBindingSpec>,
}

#[derive(Debug, Clone)]
struct ParsedFieldEconomy {
    id: String,
    namespace: String,
    production_buildings: Vec<HydratedProductionBuilding>,
    flow_couplings: Vec<HydratedFlowCoupling>,
    stockpile_silos: Vec<HydratedStockpileSilo>,
    field_resource_quantities: Vec<HydratedFieldResourceQuantity>,
    disruption_presences: Vec<HydratedDisruptionPresence>,
    owner_policy_overlays: Vec<HydratedOwnerPolicyOverlay>,
    weight_profiles: Vec<HydratedFieldEconomyWeightProfile>,
    need_bindings: Vec<NeedBindingSpec>,
    span: RawSpan,
}

#[derive(Debug, Clone)]
struct ResourceAmount {
    resource: String,
    amount: f32,
}

#[derive(Debug, Clone)]
struct ResourceOutput {
    resource: String,
    coefficient: f32,
}

#[derive(Debug, Clone)]
struct CouplingInput {
    location: String,
    resource: String,
    unit_cost: f32,
}

#[derive(Debug, Clone)]
struct CouplingSink {
    location: String,
    resource: String,
}

#[derive(Debug, Clone)]
struct CouplingWeight {
    owner: String,
    resource: String,
    unit_cost: f32,
}

#[derive(Debug, Clone)]
struct WeightInput {
    input_col: u32,
    weight_col: u32,
}

pub fn hydrate_field_economy_property(
    property: &RawProperty,
    root_node: &HydratedScenarioNode,
    owners: &[HydratedScenarioOwner],
    grid_metadata: &HydratedScenarioGridMetadata,
) -> Result<FieldEconomyLowering, HydrateError> {
    let parsed = parse_field_economy_property(property)?;
    validate_field_economy(&parsed, root_node, owners, grid_metadata)?;
    lower_field_economy(parsed)
}

fn parse_field_economy_property(
    property: &RawProperty,
) -> Result<ParsedFieldEconomy, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "field_economy")?;
    let mut id = header_id;
    let mut namespace = DEFAULT_NAMESPACE.to_string();
    let mut production_buildings = Vec::new();
    let mut flow_couplings = Vec::new();
    let mut stockpile_silos = Vec::new();
    let mut field_resource_quantities = Vec::new();
    let mut disruption_presences = Vec::new();
    let mut owner_policy_overlays = Vec::new();
    let mut weight_profiles = Vec::new();
    let mut need_bindings = Vec::new();
    let mut seen_ids = BTreeSet::new();

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => {
                let explicit_id = read_scalar_text(field, "id")?;
                if !id.is_empty() && id != explicit_id {
                    return Err(HydrateError::new_spanned(
                        format!("header id `{id}` does not match explicit id `{explicit_id}`"),
                        Some(field.key.span.clone()),
                    ));
                }
                id = explicit_id;
            }
            "namespace" => namespace = read_scalar_text(field, "namespace")?,
            "production_building" => {
                let building = parse_production_building(field)?;
                admit_unique_field_economy_id(
                    &mut seen_ids,
                    "production_building",
                    &building.id,
                    field,
                )?;
                production_buildings.push(building);
            }
            "flow_coupling" => {
                let coupling = parse_flow_coupling(field)?;
                admit_unique_field_economy_id(
                    &mut seen_ids,
                    "flow_coupling",
                    &coupling.id,
                    field,
                )?;
                flow_couplings.push(coupling);
            }
            "stockpile_silo" => {
                let silo = parse_stockpile_silo(field)?;
                admit_unique_field_economy_id(&mut seen_ids, "stockpile_silo", &silo.id, field)?;
                stockpile_silos.push(silo);
            }
            "field_resource_quantity" => {
                let quantity = parse_field_resource_quantity(field)?;
                admit_unique_field_economy_id(
                    &mut seen_ids,
                    "field_resource_quantity",
                    &quantity.id,
                    field,
                )?;
                field_resource_quantities.push(quantity);
            }
            "disruption_presence" => {
                let presence = parse_disruption_presence(field)?;
                admit_unique_field_economy_id(
                    &mut seen_ids,
                    "disruption_presence",
                    &presence.id,
                    field,
                )?;
                disruption_presences.push(presence);
            }
            "owner_policy_overlay" => {
                let overlay = parse_owner_policy_overlay(field)?;
                admit_unique_field_economy_id(
                    &mut seen_ids,
                    "owner_policy_overlay",
                    &overlay.id,
                    field,
                )?;
                owner_policy_overlays.push(overlay);
            }
            "weight_profile" => {
                let profile = parse_weight_profile(field)?;
                admit_unique_field_economy_id(&mut seen_ids, "weight_profile", &profile.id, field)?;
                weight_profiles.push(profile);
            }
            "need_binding" => {
                let binding = parse_need_binding(field)?;
                // Profile-id join: need_binding id may match weight_profile id.
                // Uniqueness among need_bindings only (separate set).
                need_bindings.push(binding);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported field_economy field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if id.is_empty() {
        return Err(HydrateError::new_spanned(
            "`field_economy` requires an id",
            Some(property.key.span.clone()),
        ));
    }
    if namespace.is_empty() {
        return Err(HydrateError::new_spanned(
            "`field_economy.namespace` cannot be empty",
            Some(property.key.span.clone()),
        ));
    }
    // Uniqueness among need_bindings; id may equal a weight_profile (profile-id join).
    let mut seen_need_ids = BTreeSet::new();
    for binding in &need_bindings {
        if !seen_need_ids.insert(binding.id.clone()) {
            return Err(HydrateError::new_spanned(
                format!("duplicate need_binding id `{}`", binding.id),
                Some(property.key.span.clone()),
            ));
        }
    }
    // Profile-id join: attach stack from matching weight_profile (same id).
    let mut joined_bindings = Vec::with_capacity(need_bindings.len());
    for mut binding in need_bindings {
        if binding.stack.gadgets.is_empty() {
            let matches: Vec<_> = weight_profiles
                .iter()
                .filter(|p| p.id == binding.id)
                .collect();
            if matches.len() > 1 {
                return Err(HydrateError::new_spanned(
                    format!(
                        "need_binding `{}` profile-id join is ambiguous ({} weight_profile ids)",
                        binding.id,
                        matches.len()
                    ),
                    Some(property.key.span.clone()),
                ));
            }
            let Some(profile) = matches.first() else {
                return Err(HydrateError::new_spanned(
                    format!(
                        "need_binding `{}` has empty stack and no weight_profile with the same id",
                        binding.id
                    ),
                    Some(property.key.span.clone()),
                ));
            };
            binding.stack = profile.stack.clone();
            if binding.profile.is_empty() {
                binding.profile = profile.profile.clone();
            } else if binding.profile != profile.profile {
                return Err(HydrateError::new_spanned(
                    format!(
                        "need_binding `{}` profile `{}` mismatches weight_profile profile `{}`",
                        binding.id, binding.profile, profile.profile
                    ),
                    Some(property.key.span.clone()),
                ));
            }
        }
        joined_bindings.push(binding);
    }
    Ok(ParsedFieldEconomy {
        id,
        namespace,
        production_buildings,
        flow_couplings,
        stockpile_silos,
        field_resource_quantities,
        disruption_presences,
        owner_policy_overlays,
        weight_profiles,
        need_bindings: joined_bindings,
        span: property.key.span.clone(),
    })
}

fn parse_production_building(
    property: &RawProperty,
) -> Result<HydratedProductionBuilding, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "production_building")?;
    let mut id = header_id;
    let mut location = None;
    let mut input = None;
    let mut output = None;
    let mut throttle_hint_max_per_tick = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "location" => location = Some(read_scalar_text(field, "location")?),
            "input" => input = Some(parse_resource_amount(field, "input")?),
            "output" => output = Some(parse_resource_output(field)?),
            "throttle_hint_max_per_tick" => {
                throttle_hint_max_per_tick =
                    Some(read_scalar_u32(field, "throttle_hint_max_per_tick")?)
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported production_building field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    let input = require_local(input, "input", property)?;
    let output = require_local(output, "output", property)?;
    Ok(HydratedProductionBuilding {
        id: require_id(id, "production_building", property)?,
        location: require_local(location, "location", property)?,
        input_resource: input.resource,
        input_amount: input.amount,
        output_resource: output.resource,
        output_coefficient: output.coefficient,
        throttle_hint_max_per_tick: require_local(
            throttle_hint_max_per_tick,
            "throttle_hint_max_per_tick",
            property,
        )?,
    })
}

fn parse_flow_coupling(property: &RawProperty) -> Result<HydratedFlowCoupling, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "flow_coupling")?;
    let mut id = header_id;
    let mut source = None;
    let mut pressure = None;
    let mut weight = None;
    let mut sink = None;
    let mut output_coefficient = None;
    let mut order_band = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "source" => source = Some(parse_coupling_input(field, "source")?),
            "pressure" => pressure = Some(parse_coupling_input(field, "pressure")?),
            "weight" => weight = Some(parse_coupling_weight(field)?),
            "sink" => sink = Some(parse_coupling_sink(field)?),
            "output_coefficient" => {
                output_coefficient = Some(read_scalar_f32(field, "output_coefficient")?)
            }
            "order_band" => order_band = Some(read_scalar_u32(field, "order_band")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported flow_coupling field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let source = require_local(source, "source", property)?;
    let pressure = require_local(pressure, "pressure", property)?;
    let weight = require_local(weight, "weight", property)?;
    let sink = require_local(sink, "sink", property)?;
    Ok(HydratedFlowCoupling {
        id: require_id(id, "flow_coupling", property)?,
        source_location: source.location,
        source_resource: source.resource,
        source_unit_cost: source.unit_cost,
        pressure_location: pressure.location,
        pressure_resource: pressure.resource,
        pressure_unit_cost: pressure.unit_cost,
        weight_owner: weight.owner,
        weight_resource: weight.resource,
        weight_unit_cost: weight.unit_cost,
        sink_location: sink.location,
        sink_resource: sink.resource,
        output_coefficient: require_local(
            output_coefficient,
            "output_coefficient",
            property,
        )?,
        order_band: require_local(order_band, "order_band", property)?,
    })
}

fn parse_stockpile_silo(property: &RawProperty) -> Result<HydratedStockpileSilo, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "stockpile_silo")?;
    let mut id = header_id;
    let mut owner = None;
    let mut owner_span = None;
    let mut resource = None;
    let mut current = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "owner" => {
                owner_span = Some(field.key.span.token_index);
                owner = Some(read_scalar_text(field, "owner")?);
            }
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            "current" => current = Some(read_scalar_f32(field, "current")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported stockpile_silo field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(HydratedStockpileSilo {
        id: require_id(id, "stockpile_silo", property)?,
        owner: require_local(owner, "owner", property)?,
        resource: require_local(resource, "resource", property)?,
        current: require_local(current, "current", property)?,
        owner_span_token: owner_span,
    })
}

fn parse_field_resource_quantity(
    property: &RawProperty,
) -> Result<HydratedFieldResourceQuantity, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "field_resource_quantity")?;
    let mut id = header_id;
    let mut location = None;
    let mut location_span = None;
    let mut resource = None;
    let mut amount = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "location" => {
                location_span = Some(field.key.span.token_index);
                location = Some(read_scalar_text(field, "location")?);
            }
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            "amount" => amount = Some(read_scalar_f32(field, "amount")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported field_resource_quantity field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(HydratedFieldResourceQuantity {
        id: require_id(id, "field_resource_quantity", property)?,
        location: require_local(location, "location", property)?,
        resource: require_local(resource, "resource", property)?,
        amount: require_local(amount, "amount", property)?,
        location_span_token: location_span,
    })
}

fn parse_disruption_presence(
    property: &RawProperty,
) -> Result<HydratedDisruptionPresence, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "disruption_presence")?;
    let mut id = header_id;
    let mut location = None;
    let mut location_span = None;
    let mut resource = None;
    let mut amount = None;
    let mut threshold = None;
    let mut event_kind = None;
    let mut direction = TriggerDirection::Rising;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "location" => {
                location_span = Some(field.key.span.token_index);
                location = Some(read_scalar_text(field, "location")?);
            }
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            "amount" => amount = Some(read_scalar_f32(field, "amount")?),
            "threshold" => threshold = Some(read_scalar_f32(field, "threshold")?),
            "event_kind" => event_kind = Some(read_scalar_u32(field, "event_kind")?),
            "direction" => direction = parse_trigger_direction(field)?,
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported disruption_presence field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(HydratedDisruptionPresence {
        id: require_id(id, "disruption_presence", property)?,
        location: require_local(location, "location", property)?,
        resource: require_local(resource, "resource", property)?,
        amount: require_local(amount, "amount", property)?,
        threshold: require_local(threshold, "threshold", property)?,
        event_kind: require_local(event_kind, "event_kind", property)?,
        direction,
        location_span_token: location_span,
    })
}

fn parse_owner_policy_overlay(
    property: &RawProperty,
) -> Result<HydratedOwnerPolicyOverlay, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "owner_policy_overlay")?;
    let mut id = header_id;
    let mut owner = None;
    let mut targets_property = None;
    let mut amount_add = None;
    let mut amount_mult = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "owner" => owner = Some(read_scalar_text(field, "owner")?),
            "targets_property" => {
                targets_property = Some(read_scalar_text(field, "targets_property")?)
            }
            "amount_add" => amount_add = Some(read_scalar_f32(field, "amount_add")?),
            "amount_mult" => amount_mult = Some(read_scalar_f32(field, "amount_mult")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported owner_policy_overlay field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    let transform = match (amount_add, amount_mult) {
        (Some(add), None) => HydratedOwnerPolicyTransform::Add(add),
        (None, Some(mult)) => HydratedOwnerPolicyTransform::Multiply(mult),
        (Some(_), Some(_)) => {
            return Err(HydrateError::new_spanned(
                "owner_policy_overlay cannot specify both amount_add and amount_mult",
                Some(property.key.span.clone()),
            ));
        }
        (None, None) => {
            return Err(HydrateError::new_spanned(
                "owner_policy_overlay requires amount_add or amount_mult",
                Some(property.key.span.clone()),
            ));
        }
    };
    Ok(HydratedOwnerPolicyOverlay {
        id: require_id(id, "owner_policy_overlay", property)?,
        owner: require_local(owner, "owner", property)?,
        targets_property: require_local(targets_property, "targets_property", property)?,
        transform,
    })
}

fn parse_weight_profile(
    property: &RawProperty,
) -> Result<HydratedFieldEconomyWeightProfile, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "weight_profile")?;
    let mut id = header_id;
    let mut profile = None;
    let mut inputs = Vec::new();
    let mut output_col = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "profile" => profile = Some(read_scalar_text(field, "profile")?),
            "input" => inputs.push(parse_weight_input(field)?),
            "output_col" => output_col = Some(read_scalar_u32(field, "output_col")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported weight_profile field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    if inputs.is_empty() {
        return Err(HydrateError::new_spanned(
            "weight_profile requires at least one input",
            Some(property.key.span.clone()),
        ));
    }
    let profile = require_local(profile, "profile", property)?;
    if !ADMITTED_WEIGHT_PROFILES.contains(&profile.as_str()) {
        return Err(HydrateError::new_spanned(
            format!("unsupported weight_profile profile `{profile}`"),
            Some(property.key.span.clone()),
        ));
    }
    let id = require_id(id, "weight_profile", property)?;
    let stack = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::WeightedAccumulator {
            id: format!("{id}_weighted_accumulator"),
            input_cols: inputs.iter().map(|entry| entry.input_col).collect(),
            weight_cols: inputs.iter().map(|entry| entry.weight_col).collect(),
            output_col,
        }],
    };
    Ok(HydratedFieldEconomyWeightProfile { id, profile, stack })
}

fn parse_resource_amount(
    property: &RawProperty,
    field_name: &str,
) -> Result<ResourceAmount, HydrateError> {
    let block = require_block(property, field_name)?;
    let mut resource = None;
    let mut amount = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            "amount" => amount = Some(read_scalar_f32(field, "amount")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported {field_name} field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(ResourceAmount {
        resource: require_local(resource, "resource", property)?,
        amount: require_local(amount, "amount", property)?,
    })
}

fn parse_resource_output(property: &RawProperty) -> Result<ResourceOutput, HydrateError> {
    let block = require_block(property, "output")?;
    let mut resource = None;
    let mut coefficient = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            "coefficient" => coefficient = Some(read_scalar_f32(field, "coefficient")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported output field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(ResourceOutput {
        resource: require_local(resource, "resource", property)?,
        coefficient: require_local(coefficient, "coefficient", property)?,
    })
}

fn parse_coupling_input(
    property: &RawProperty,
    field_name: &str,
) -> Result<CouplingInput, HydrateError> {
    let block = require_block(property, field_name)?;
    let mut location = None;
    let mut resource = None;
    let mut unit_cost = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "location" => location = Some(read_scalar_text(field, "location")?),
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            "unit_cost" => unit_cost = Some(read_scalar_f32(field, "unit_cost")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported {field_name} field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(CouplingInput {
        location: require_local(location, "location", property)?,
        resource: require_local(resource, "resource", property)?,
        unit_cost: require_local(unit_cost, "unit_cost", property)?,
    })
}

fn parse_coupling_sink(property: &RawProperty) -> Result<CouplingSink, HydrateError> {
    let block = require_block(property, "sink")?;
    let mut location = None;
    let mut resource = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "location" => location = Some(read_scalar_text(field, "location")?),
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported sink field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(CouplingSink {
        location: require_local(location, "location", property)?,
        resource: require_local(resource, "resource", property)?,
    })
}

fn parse_coupling_weight(property: &RawProperty) -> Result<CouplingWeight, HydrateError> {
    let block = require_block(property, "weight")?;
    let mut owner = None;
    let mut resource = None;
    let mut unit_cost = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "owner" => owner = Some(read_scalar_text(field, "owner")?),
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            "unit_cost" => unit_cost = Some(read_scalar_f32(field, "unit_cost")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported weight field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(CouplingWeight {
        owner: require_local(owner, "owner", property)?,
        resource: require_local(resource, "resource", property)?,
        unit_cost: require_local(unit_cost, "unit_cost", property)?,
    })
}

fn parse_need_binding(property: &RawProperty) -> Result<NeedBindingSpec, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "need_binding")?;
    let mut id = header_id;
    let mut profile = String::new();
    let mut participant = None;
    let mut participant_span = None;
    let mut arena = None;
    let mut arena_span = None;
    let mut inputs = Vec::new();
    let mut weights = Vec::new();
    let mut threshold = None;
    let mut event_kind = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "profile" => profile = read_scalar_text(field, "profile")?,
            "participant" => {
                participant_span = Some(field.key.span.token_index);
                participant = Some(read_scalar_text(field, "participant")?);
            }
            "arena" => {
                arena_span = Some(field.key.span.token_index);
                arena = Some(read_scalar_text(field, "arena")?);
            }
            "input" => inputs.push(parse_semantic_locus(field, "input")?),
            "weight" => weights.push(parse_semantic_locus(field, "weight")?),
            "threshold" => threshold = Some(read_scalar_f32(field, "threshold")?),
            "event_kind" => event_kind = Some(read_scalar_u32(field, "event_kind")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported need_binding field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    let id = require_id(id, "need_binding", property)?;
    let participant = require_local(participant, "participant", property)?;
    let arena = require_local(arena, "arena", property)?;
    if arena.trim().is_empty() || arena == "default" {
        return Err(HydrateError::new_spanned(
            "need_binding.arena must be an explicit arena name (no first-arena fallback)",
            Some(property.key.span.clone()),
        ));
    }
    if inputs.is_empty() {
        return Err(HydrateError::new_spanned(
            "need_binding requires at least one input",
            Some(property.key.span.clone()),
        ));
    }
    if weights.is_empty() {
        return Err(HydrateError::new_spanned(
            "need_binding requires at least one weight",
            Some(property.key.span.clone()),
        ));
    }
    if inputs.len() != weights.len() {
        return Err(HydrateError::new_spanned(
            "need_binding input/weight arity must match",
            Some(property.key.span.clone()),
        ));
    }
    let threshold = require_local(threshold, "threshold", property)?;
    let event_kind = require_local(event_kind, "event_kind", property)?;
    Ok(NeedBindingSpec {
        id,
        profile,
        participant,
        arena,
        stack: EmlGadgetStackSpec { gadgets: vec![] },
        inputs,
        weights,
        threshold,
        event_kind,
        source_span_token: Some(property.key.span.token_index),
        participant_span_token: participant_span,
        arena_span_token: arena_span,
    })
}

fn parse_semantic_locus(
    property: &RawProperty,
    field_name: &str,
) -> Result<SemanticPropertyLocusSpec, HydrateError> {
    let block = require_block(property, field_name)?;
    let mut entity = None;
    let mut property_key = None;
    let mut role = None;
    // Prefer the most specific authored field span (entity > property > role > block).
    let mut locus_span = property.key.span.token_index;
    for field in &block.properties {
        match field.key.text.as_str() {
            "entity" => {
                locus_span = field.key.span.token_index;
                entity = Some(read_scalar_text(field, "entity")?);
            }
            "property" => {
                if entity.is_none() {
                    locus_span = field.key.span.token_index;
                }
                let raw = read_scalar_text(field, "property")?;
                let (ns, name) = raw.split_once("::").ok_or_else(|| {
                    HydrateError::new_spanned(
                        format!(
                            "need_binding.{field_name}.property must be `namespace::name`, got `{raw}`"
                        ),
                        Some(field.key.span.clone()),
                    )
                })?;
                property_key = Some(PropertyKey::new(ns, name));
            }
            "role" => {
                if entity.is_none() && property_key.is_none() {
                    locus_span = field.key.span.token_index;
                }
                let text = read_scalar_text(field, "role")?;
                role = Some(match text.as_str() {
                    "Amount" | "amount" => SubFieldRole::Amount,
                    other => {
                        return Err(HydrateError::new_spanned(
                            format!(
                                "need_binding.{field_name}.role `{other}` unsupported (Amount only in RF-5A)"
                            ),
                            Some(field.key.span.clone()),
                        ));
                    }
                });
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported need_binding.{field_name} field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(SemanticPropertyLocusSpec {
        entity: require_local(entity, "entity", property)?,
        property: require_local(property_key, "property", property)?,
        role: require_local(role, "role", property)?,
        source_span_token: Some(locus_span),
    })
}

fn parse_weight_input(property: &RawProperty) -> Result<WeightInput, HydrateError> {
    let block = require_block(property, "input")?;
    let mut input_col = None;
    let mut weight_col = None;
    for field in &block.properties {
        match field.key.text.as_str() {
            "input_col" => input_col = Some(read_scalar_u32(field, "input_col")?),
            "weight_col" => weight_col = Some(read_scalar_u32(field, "weight_col")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported weight_profile.input field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(WeightInput {
        input_col: require_local(input_col, "input_col", property)?,
        weight_col: require_local(weight_col, "weight_col", property)?,
    })
}

fn parse_trigger_direction(property: &RawProperty) -> Result<TriggerDirection, HydrateError> {
    match read_scalar_text(property, "direction")?.as_str() {
        "Rising" => Ok(TriggerDirection::Rising),
        "Falling" => Ok(TriggerDirection::Falling),
        other => Err(HydrateError::new_spanned(
            format!("unsupported trigger direction `{other}`"),
            Some(property.key.span.clone()),
        )),
    }
}

fn validate_field_economy(
    parsed: &ParsedFieldEconomy,
    root_node: &HydratedScenarioNode,
    owners: &[HydratedScenarioOwner],
    grid_metadata: &HydratedScenarioGridMetadata,
) -> Result<(), HydrateError> {
    if parsed.production_buildings.is_empty()
        && parsed.flow_couplings.is_empty()
        && parsed.stockpile_silos.is_empty()
        && parsed.field_resource_quantities.is_empty()
        && parsed.disruption_presences.is_empty()
        && parsed.owner_policy_overlays.is_empty()
        && parsed.weight_profiles.is_empty()
    {
        return Err(HydrateError::new_spanned(
            "field_economy requires at least one lowering block",
            Some(parsed.span.clone()),
        ));
    }
    if !parsed.production_buildings.is_empty() && parsed.flow_couplings.is_empty() {
        return Err(HydrateError::new_spanned(
            "field_economy with production_building requires an authored flow_coupling",
            Some(parsed.span.clone()),
        ));
    }

    let mut spatial_location_ids = Vec::new();
    let produced_loci: BTreeSet<(String, String)> = parsed
        .production_buildings
        .iter()
        .map(|building| (building.location.clone(), building.output_resource.clone()))
        .chain(parsed.field_resource_quantities.iter().map(|quantity| {
            (quantity.location.clone(), quantity.resource.clone())
        }))
        .collect();
    let pressure_loci: BTreeSet<(String, String)> = parsed
        .disruption_presences
        .iter()
        .map(|presence| (presence.location.clone(), presence.resource.clone()))
        .collect();
    for building in &parsed.production_buildings {
        validate_location_ref(&building.location, root_node, &parsed.span)?;
        validate_positive_amount(
            building.input_amount,
            "production_building.input.amount",
            &parsed.span,
        )?;
        validate_positive_amount(
            building.output_coefficient,
            "production_building.output.coefficient",
            &parsed.span,
        )?;
        if building.throttle_hint_max_per_tick == 0 {
            return Err(HydrateError::new_spanned(
                "production_building.throttle_hint_max_per_tick must be greater than zero",
                Some(parsed.span.clone()),
            ));
        }
        spatial_location_ids.push(building.location.clone());
    }
    for coupling in &parsed.flow_couplings {
        validate_location_ref(&coupling.source_location, root_node, &parsed.span)?;
        validate_location_ref(&coupling.pressure_location, root_node, &parsed.span)?;
        validate_location_ref(&coupling.sink_location, root_node, &parsed.span)?;
        validate_owner_ref(&coupling.weight_owner, owners, &parsed.span)?;
        validate_positive_amount(
            coupling.source_unit_cost,
            "flow_coupling.source.unit_cost",
            &parsed.span,
        )?;
        validate_positive_amount(
            coupling.pressure_unit_cost,
            "flow_coupling.pressure.unit_cost",
            &parsed.span,
        )?;
        validate_positive_amount(
            coupling.weight_unit_cost,
            "flow_coupling.weight.unit_cost",
            &parsed.span,
        )?;
        validate_positive_amount(
            coupling.output_coefficient,
            "flow_coupling.output_coefficient",
            &parsed.span,
        )?;
        if coupling.order_band == 0 {
            return Err(HydrateError::new_spanned(
                "flow_coupling.order_band must be greater than zero",
                Some(parsed.span.clone()),
            ));
        }
        if !produced_loci.contains(&(
            coupling.source_location.clone(),
            coupling.source_resource.clone(),
        )) {
            return Err(HydrateError::new_spanned(
                format!(
                    "flow_coupling `{}` source is not an authored production/quantity locus",
                    coupling.id
                ),
                Some(parsed.span.clone()),
            ));
        }
        if !pressure_loci.contains(&(
            coupling.pressure_location.clone(),
            coupling.pressure_resource.clone(),
        )) {
            return Err(HydrateError::new_spanned(
                format!(
                    "flow_coupling `{}` pressure is not an authored disruption locus",
                    coupling.id
                ),
                Some(parsed.span.clone()),
            ));
        }
        if !parsed.stockpile_silos.iter().any(|silo| {
            silo.owner == coupling.weight_owner && silo.resource == coupling.weight_resource
        }) {
            return Err(HydrateError::new_spanned(
                format!(
                    "flow_coupling `{}` weight is not an authored owner stockpile locus",
                    coupling.id
                ),
                Some(parsed.span.clone()),
            ));
        }
        if coupling.source_location == coupling.sink_location
            && coupling.source_resource == coupling.sink_resource
        {
            return Err(HydrateError::new_spanned(
                format!("flow_coupling `{}` source and sink must differ", coupling.id),
                Some(parsed.span.clone()),
            ));
        }
        spatial_location_ids.extend([
            coupling.source_location.clone(),
            coupling.pressure_location.clone(),
            coupling.sink_location.clone(),
        ]);
    }
    for quantity in &parsed.field_resource_quantities {
        validate_location_ref(&quantity.location, root_node, &parsed.span)?;
        validate_non_negative_amount(
            quantity.amount,
            "field_resource_quantity.amount",
            &parsed.span,
        )?;
        spatial_location_ids.push(quantity.location.clone());
    }
    for presence in &parsed.disruption_presences {
        validate_location_ref(&presence.location, root_node, &parsed.span)?;
        validate_non_negative_amount(presence.amount, "disruption_presence.amount", &parsed.span)?;
        validate_non_negative_amount(
            presence.threshold,
            "disruption_presence.threshold",
            &parsed.span,
        )?;
        spatial_location_ids.push(presence.location.clone());
    }
    validate_structural_location_placements(&spatial_location_ids, grid_metadata, &parsed.span)?;

    for silo in &parsed.stockpile_silos {
        validate_owner_ref(&silo.owner, owners, &parsed.span)?;
        validate_non_negative_amount(silo.current, "stockpile_silo.current", &parsed.span)?;
    }
    for overlay in &parsed.owner_policy_overlays {
        validate_owner_ref(&overlay.owner, owners, &parsed.span)?;
    }
    Ok(())
}

fn lower_field_economy(parsed: ParsedFieldEconomy) -> Result<FieldEconomyLowering, HydrateError> {
    let mut properties = Vec::new();
    for building in &parsed.production_buildings {
        properties.push(located_resource_property(
            &parsed.namespace,
            &building.location,
            &building.input_resource,
            "quantity",
        ));
        properties.push(located_resource_property(
            &parsed.namespace,
            &building.location,
            &building.output_resource,
            "quantity",
        ));
    }
    for coupling in &parsed.flow_couplings {
        properties.push(located_resource_property(
            &parsed.namespace,
            &coupling.sink_location,
            &coupling.sink_resource,
            "quantity",
        ));
    }
    for silo in &parsed.stockpile_silos {
        properties.push(owner_resource_property(
            &parsed.namespace,
            &silo.owner,
            &silo.resource,
            "current",
        ));
        properties.push(owner_resource_property(
            &parsed.namespace,
            &silo.owner,
            &silo.resource,
            "stockpile",
        ));
    }
    for quantity in &parsed.field_resource_quantities {
        properties.push(located_resource_property(
            &parsed.namespace,
            &quantity.location,
            &quantity.resource,
            "quantity",
        ));
    }
    for presence in &parsed.disruption_presences {
        properties.push(presence_property(
            &parsed.namespace,
            &presence.location,
            &presence.resource,
            &presence.id,
        ));
    }
    dedupe_properties(&mut properties);

    let mut recipes: Vec<ResourceRecipeSpec> = parsed
        .production_buildings
        .iter()
        .map(|building| ResourceRecipeSpec {
            id: format!("{}_recipe_{}", parsed.id, building.id),
            inputs: vec![RecipeInputSpec {
                property: located_resource_key(
                    &parsed.namespace,
                    &building.location,
                    &building.input_resource,
                    "quantity",
                ),
                role: SubFieldRole::Amount,
                unit_cost: building.input_amount,
                host_entity: Some(building.location.clone()),
                host_span_token: None,
            }],
            target: located_resource_key(
                &parsed.namespace,
                &building.location,
                &building.output_resource,
                "quantity",
            ),
            target_role: SubFieldRole::Amount,
            target_host_entity: Some(building.location.clone()),
            target_host_span_token: None,
            output_coefficient: building.output_coefficient,
            order_band: 0,
            throttle_hint_max_per_tick: building.throttle_hint_max_per_tick,
        })
        .collect();
    recipes.extend(parsed.flow_couplings.iter().map(|coupling| {
        ResourceRecipeSpec {
            id: format!("{}_coupling_{}", parsed.id, coupling.id),
            inputs: vec![
                RecipeInputSpec {
                    property: located_resource_key(
                        &parsed.namespace,
                        &coupling.source_location,
                        &coupling.source_resource,
                        "quantity",
                    ),
                    role: SubFieldRole::Amount,
                    unit_cost: coupling.source_unit_cost,
                    host_entity: Some(coupling.source_location.clone()),
                    host_span_token: None,
                },
                RecipeInputSpec {
                    property: located_resource_key(
                        &parsed.namespace,
                        &coupling.pressure_location,
                        &coupling.pressure_resource,
                        "presence",
                    ),
                    role: SubFieldRole::Amount,
                    unit_cost: coupling.pressure_unit_cost,
                    host_entity: Some(coupling.pressure_location.clone()),
                    host_span_token: None,
                },
                RecipeInputSpec {
                    property: owner_resource_key(
                        &parsed.namespace,
                        &coupling.weight_owner,
                        &coupling.weight_resource,
                        "stockpile",
                    ),
                    role: SubFieldRole::Amount,
                    unit_cost: coupling.weight_unit_cost,
                    host_entity: Some(coupling.weight_owner.clone()),
                    host_span_token: None,
                },
            ],
            target: located_resource_key(
                &parsed.namespace,
                &coupling.sink_location,
                &coupling.sink_resource,
                "quantity",
            ),
            target_role: SubFieldRole::Amount,
            target_host_entity: Some(coupling.sink_location.clone()),
            target_host_span_token: None,
            output_coefficient: coupling.output_coefficient,
            order_band: coupling.order_band,
            throttle_hint_max_per_tick: 1,
        }
    }));

    let transfers = parsed
        .stockpile_silos
        .iter()
        .enumerate()
        .map(|(index, silo)| ResourceTransferSpec {
            id: format!("{}_silo_transfer_{}", parsed.id, silo.id),
            source: owner_resource_key(&parsed.namespace, &silo.owner, &silo.resource, "current"),
            source_role: SubFieldRole::Amount,
            target: owner_resource_key(&parsed.namespace, &silo.owner, &silo.resource, "stockpile"),
            target_role: SubFieldRole::Amount,
            amount: silo.current,
            order_band: index as u32,
            source_host_entity: Some(silo.owner.clone()),
            target_host_entity: Some(silo.owner.clone()),
            source_host_span_token: silo.owner_span_token,
            target_host_span_token: silo.owner_span_token,
        })
        .collect();

    let mut emissions = Vec::new();
    for silo in &parsed.stockpile_silos {
        emissions.push(ResourceEmissionSpec {
            id: format!("{}_silo_current_{}", parsed.id, silo.id),
            source: owner_resource_key(&parsed.namespace, &silo.owner, &silo.resource, "current"),
            source_role: SubFieldRole::Amount,
            formula: EmissionFormulaSpec::Constant(silo.current),
            host_entity: Some(silo.owner.clone()),
            host_span_token: silo.owner_span_token,
        });
    }
    for quantity in &parsed.field_resource_quantities {
        emissions.push(ResourceEmissionSpec {
            id: format!("{}_quantity_emission_{}", parsed.id, quantity.id),
            source: located_resource_key(
                &parsed.namespace,
                &quantity.location,
                &quantity.resource,
                "quantity",
            ),
            source_role: SubFieldRole::Amount,
            formula: EmissionFormulaSpec::Constant(quantity.amount),
            // Location-hosted quantity: install on scenario root until location
            // entity-host seam is generalized; host is still explicit (not name-guess).
            host_entity: Some(quantity.location.clone()),
            host_span_token: quantity.location_span_token,
        });
    }
    for presence in &parsed.disruption_presences {
        emissions.push(ResourceEmissionSpec {
            id: format!("{}_presence_emission_{}", parsed.id, presence.id),
            source: presence_key(&parsed.namespace, &presence.location, &presence.resource),
            source_role: SubFieldRole::Amount,
            formula: EmissionFormulaSpec::Constant(presence.amount),
            host_entity: Some(presence.location.clone()),
            host_span_token: presence.location_span_token,
        });
    }

    let emit_on_threshold = parsed
        .disruption_presences
        .iter()
        .map(|presence| EmitOnThresholdSpec {
            id: format!("{}_presence_threshold_{}", parsed.id, presence.id),
            source: presence_key(&parsed.namespace, &presence.location, &presence.resource),
            source_role: SubFieldRole::Amount,
            threshold: presence.threshold,
            direction: presence.direction,
            event_kind: presence.event_kind,
            buffer: EmitBufferSpec::Values,
            host_entity: Some(presence.location.clone()),
            host_span_token: presence.location_span_token,
        })
        .collect();

    let mut overlays = Vec::new();
    overlays.extend(parsed.field_resource_quantities.iter().map(|quantity| {
        location_overlay(
            &parsed.id,
            "quantity",
            &quantity.id,
            &property_ref(&located_resource_key(
                &parsed.namespace,
                &quantity.location,
                &quantity.resource,
                "quantity",
            )),
            quantity.amount,
            &quantity.location,
            OverlayKind::Infrastructure,
        )
    }));
    overlays.extend(parsed.disruption_presences.iter().map(|presence| {
        location_overlay(
            &parsed.id,
            "presence",
            &presence.id,
            &property_ref(&presence_key(
                &parsed.namespace,
                &presence.location,
                &presence.resource,
            )),
            presence.amount,
            &presence.location,
            OverlayKind::Crisis,
        )
    }));
    overlays.extend(
        parsed
            .owner_policy_overlays
            .iter()
            .map(|overlay| OverlaySpec {
                id: format!("{}_owner_policy_{}", parsed.id, overlay.id),
                display_name: overlay.id.clone(),
                targets_property: overlay.targets_property.clone(),
                sub_field_deltas: vec![(
                    SubFieldRole::Amount,
                    match overlay.transform {
                        HydratedOwnerPolicyTransform::Add(amount) => TransformOp::Add(amount),
                        HydratedOwnerPolicyTransform::Multiply(amount) => {
                            TransformOp::Multiply(amount)
                        }
                    },
                )],
                lifecycle: OverlayLifecycle::Permanent,
                kind: OverlayKind::Policy,
                source: OverlaySource::Player,
                install: InstallTargetSpec::ScenarioListed {
                    target_id: overlay.owner.clone(),
                },
            }),
    );

    let need_bindings = parsed.need_bindings;
    let hydrated = HydratedFieldEconomy {
        id: parsed.id,
        namespace: parsed.namespace,
        production_buildings: parsed.production_buildings,
        flow_couplings: parsed.flow_couplings,
        stockpile_silos: parsed.stockpile_silos,
        field_resource_quantities: parsed.field_resource_quantities,
        disruption_presences: parsed.disruption_presences,
        owner_policy_overlays: parsed.owner_policy_overlays,
        weight_profiles: parsed.weight_profiles,
        need_bindings: need_bindings.clone(),
    };
    Ok(FieldEconomyLowering {
        hydrated,
        properties,
        overlays,
        resource_economy: ResourceEconomySpec {
            opt_in_mode: ResourceEconomyOptInMode::TransferAndEmission,
            transfers,
            recipes,
            emissions,
            emit_on_threshold,
        },
        need_bindings,
    })
}

fn resource_property(namespace: &str, resource: &str, suffix: &str) -> PropertySpec {
    PropertySpec {
        id: format!("{namespace}_{resource}_{suffix}"),
        namespace: namespace.to_string(),
        name: format!("{resource}_{suffix}"),
        display_name: format!("{resource} {suffix}"),
        description: "field economy resource surface".into(),
        // Keep the standard 3-lane width so authored weight_profile column indices
        // stay stable, but leave Amount Unbounded and ungoverned so production /
        // disruption can accrete above 1.0 under ordinary ticks.
        sub_fields: field_economy_resource_subfields(),
    }
}

fn located_resource_property(
    namespace: &str,
    location: &str,
    resource: &str,
    suffix: &str,
) -> PropertySpec {
    resource_property(namespace, &format!("{location}_{resource}"), suffix)
}

fn owner_resource_property(
    namespace: &str,
    owner: &str,
    resource: &str,
    suffix: &str,
) -> PropertySpec {
    resource_property(namespace, &format!("{owner}_{resource}"), suffix)
}

fn presence_property(namespace: &str, location: &str, resource: &str, id: &str) -> PropertySpec {
    PropertySpec {
        id: format!("{namespace}_{location}_{resource}_presence"),
        namespace: namespace.to_string(),
        name: format!("{location}_{resource}_presence"),
        display_name: format!("{location} {resource} presence"),
        description: format!("field economy disruption presence authored by `{id}`"),
        sub_fields: field_economy_resource_subfields(),
    }
}

fn field_economy_resource_subfields() -> Vec<SubFieldSpec> {
    vec![
        SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "amount".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        },
        SubFieldSpec {
            role: SubFieldRole::Velocity,
            width: 1,
            clamp: ClampBehavior::Bounded {
                min: -1.0,
                max: 1.0,
            },
            velocity_max: None,
            default: 0.0,
            display_name: "velocity".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        },
        SubFieldSpec {
            role: SubFieldRole::Intensity,
            width: 1,
            clamp: ClampBehavior::Bounded { min: 0.0, max: 1.0 },
            velocity_max: None,
            default: 0.0,
            display_name: "intensity".into(),
            display_range: Some((0.0, 1.0)),
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        },
    ]
}

fn resource_key(namespace: &str, resource: &str, suffix: &str) -> PropertyKey {
    PropertyKey::new(namespace, &format!("{resource}_{suffix}"))
}

fn located_resource_key(
    namespace: &str,
    location: &str,
    resource: &str,
    suffix: &str,
) -> PropertyKey {
    resource_key(namespace, &format!("{location}_{resource}"), suffix)
}

fn owner_resource_key(namespace: &str, owner: &str, resource: &str, suffix: &str) -> PropertyKey {
    resource_key(namespace, &format!("{owner}_{resource}"), suffix)
}

fn presence_key(namespace: &str, location: &str, resource: &str) -> PropertyKey {
    PropertyKey::new(namespace, &format!("{location}_{resource}_presence"))
}

fn property_ref(key: &PropertyKey) -> String {
    format!("{}::{}", key.namespace, key.name)
}

fn location_overlay(
    economy_id: &str,
    kind: &str,
    id: &str,
    targets_property: &str,
    amount: f32,
    location: &str,
    overlay_kind: OverlayKind,
) -> OverlaySpec {
    OverlaySpec {
        id: format!("{economy_id}_{kind}_location_{id}"),
        display_name: id.to_string(),
        targets_property: targets_property.to_string(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(amount))],
        lifecycle: OverlayLifecycle::Permanent,
        kind: overlay_kind,
        source: OverlaySource::System,
        install: InstallTargetSpec::ScenarioListed {
            target_id: location.to_string(),
        },
    }
}

fn dedupe_properties(properties: &mut Vec<PropertySpec>) {
    let mut by_key = BTreeMap::<(String, String), PropertySpec>::new();
    for property in properties.drain(..) {
        by_key
            .entry((property.namespace.clone(), property.name.clone()))
            .or_insert(property);
    }
    properties.extend(by_key.into_values());
}

fn validate_location_ref(
    location: &str,
    root_node: &HydratedScenarioNode,
    span: &RawSpan,
) -> Result<(), HydrateError> {
    let node = find_node(root_node, location).ok_or_else(|| {
        HydrateError::new_spanned(
            format!("field_economy references unknown location `{location}`"),
            Some(span.clone()),
        )
    })?;
    if location_participant_kind_label(&node.kind) != "Location" {
        return Err(HydrateError::new_spanned(
            format!("field_economy target `{location}` is not a Location"),
            Some(span.clone()),
        ));
    }
    Ok(())
}

fn location_participant_kind_label(kind: &SimThingKind) -> &'static str {
    if std::mem::discriminant(kind) == std::mem::discriminant(&SimThingKind::Location) {
        "Location"
    } else {
        "non-Location"
    }
}

fn validate_owner_ref(
    owner: &str,
    owners: &[HydratedScenarioOwner],
    span: &RawSpan,
) -> Result<(), HydrateError> {
    if owners.iter().any(|entry| entry.owner_key == owner) {
        return Ok(());
    }
    Err(HydrateError::new_spanned(
        format!("field_economy references unknown owner `{owner}`"),
        Some(span.clone()),
    ))
}

fn validate_structural_location_placements(
    location_ids: &[String],
    grid_metadata: &HydratedScenarioGridMetadata,
    span: &RawSpan,
) -> Result<(), HydrateError> {
    let placements: Vec<StructuralGridPlacement<'_>> = grid_metadata
        .placements
        .iter()
        .map(|placement| StructuralGridPlacement {
            location_id: placement.target_id.as_str(),
            coord: StructuralCoord::new(placement.col, placement.row),
        })
        .collect();
    let refs: Vec<&str> = location_ids.iter().map(String::as_str).collect();
    validate_location_ids_have_structural_placements(&refs, &placements).map_err(|err| {
        let PlacedParticipantValidationError { message } = err;
        HydrateError::new_spanned(message, Some(span.clone()))
    })
}

fn find_node<'a>(node: &'a HydratedScenarioNode, id: &str) -> Option<&'a HydratedScenarioNode> {
    if node.id == id {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find_node(child, id) {
            return Some(found);
        }
    }
    None
}

fn validate_positive_amount(value: f32, field: &str, span: &RawSpan) -> Result<(), HydrateError> {
    if value.is_finite() && value > 0.0 {
        return Ok(());
    }
    Err(HydrateError::new_spanned(
        format!("`{field}` must be greater than zero"),
        Some(span.clone()),
    ))
}

fn validate_non_negative_amount(
    value: f32,
    field: &str,
    span: &RawSpan,
) -> Result<(), HydrateError> {
    if value >= 0.0 {
        return Ok(());
    }
    Err(HydrateError::new_spanned(
        format!("`{field}` must be non-negative"),
        Some(span.clone()),
    ))
}

fn admit_unique_field_economy_id(
    seen_ids: &mut BTreeSet<String>,
    kind: &str,
    id: &str,
    property: &RawProperty,
) -> Result<(), HydrateError> {
    if seen_ids.insert(id.to_string()) {
        return Ok(());
    }
    Err(HydrateError::new_spanned(
        format!("duplicate field_economy id `{id}` in {kind}"),
        Some(property.key.span.clone()),
    ))
}

fn read_checked_id(property: &RawProperty, header_id: &str) -> Result<String, HydrateError> {
    let explicit_id = read_scalar_text(property, "id")?;
    if !header_id.is_empty() && header_id != explicit_id {
        return Err(HydrateError::new_spanned(
            format!("header id `{header_id}` does not match explicit id `{explicit_id}`"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(explicit_id)
}

fn require_id(id: String, block: &str, property: &RawProperty) -> Result<String, HydrateError> {
    if !id.is_empty() {
        return Ok(id);
    }
    Err(HydrateError::new_spanned(
        format!("`{block}` requires an id"),
        Some(property.key.span.clone()),
    ))
}

fn require_local<T>(
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
