//! Scenario-agnostic field-economy hydration.
//!
//! This lowers authoring blocks onto existing spec surfaces only: `PropertySpec`,
//! `OverlaySpec`, `ResourceEconomySpec`, and `EmlGadgetStackSpec`.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use simthing_core::{
    OverlayKind, OverlayLifecycle, OverlaySource, PlacedParticipantValidationError, SimThingKind,
    StructuralCoord, StructuralGridPlacement, SubFieldRole, TransformOp,
    validate_location_ids_have_structural_placements,
};
use simthing_spec::spec::eml_gadget::{EmlGadgetInstanceSpec, EmlGadgetStackSpec};
use simthing_spec::spec::install_target::InstallTargetSpec;
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
    pub stockpile_silos: Vec<HydratedStockpileSilo>,
    pub field_resource_quantities: Vec<HydratedFieldResourceQuantity>,
    pub disruption_presences: Vec<HydratedDisruptionPresence>,
    pub owner_policy_overlays: Vec<HydratedOwnerPolicyOverlay>,
    pub weight_profiles: Vec<HydratedFieldEconomyWeightProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedProductionBuilding {
    pub id: String,
    pub location: String,
    pub input_resource: String,
    pub input_amount: f32,
    pub output_resource: String,
    pub output_amount: f32,
    pub throttle_hint_max_per_tick: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedStockpileSilo {
    pub id: String,
    pub owner: String,
    pub resource: String,
    pub capacity: f32,
    pub current: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HydratedFieldResourceQuantity {
    pub id: String,
    pub location: String,
    pub resource: String,
    pub amount: f32,
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
}

#[derive(Debug, Clone)]
struct ParsedFieldEconomy {
    id: String,
    namespace: String,
    production_buildings: Vec<HydratedProductionBuilding>,
    stockpile_silos: Vec<HydratedStockpileSilo>,
    field_resource_quantities: Vec<HydratedFieldResourceQuantity>,
    disruption_presences: Vec<HydratedDisruptionPresence>,
    owner_policy_overlays: Vec<HydratedOwnerPolicyOverlay>,
    weight_profiles: Vec<HydratedFieldEconomyWeightProfile>,
    span: RawSpan,
}

#[derive(Debug, Clone)]
struct ResourceAmount {
    resource: String,
    amount: f32,
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
    let mut stockpile_silos = Vec::new();
    let mut field_resource_quantities = Vec::new();
    let mut disruption_presences = Vec::new();
    let mut owner_policy_overlays = Vec::new();
    let mut weight_profiles = Vec::new();
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
    Ok(ParsedFieldEconomy {
        id,
        namespace,
        production_buildings,
        stockpile_silos,
        field_resource_quantities,
        disruption_presences,
        owner_policy_overlays,
        weight_profiles,
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
            "output" => output = Some(parse_resource_amount(field, "output")?),
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
        output_amount: output.amount,
        throttle_hint_max_per_tick: require_local(
            throttle_hint_max_per_tick,
            "throttle_hint_max_per_tick",
            property,
        )?,
    })
}

fn parse_stockpile_silo(property: &RawProperty) -> Result<HydratedStockpileSilo, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "stockpile_silo")?;
    let mut id = header_id;
    let mut owner = None;
    let mut resource = None;
    let mut capacity = None;
    let mut current = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "owner" => owner = Some(read_scalar_text(field, "owner")?),
            "resource" => resource = Some(read_scalar_text(field, "resource")?),
            "capacity" => capacity = Some(read_scalar_f32(field, "capacity")?),
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
        capacity: require_local(capacity, "capacity", property)?,
        current: require_local(current, "current", property)?,
    })
}

fn parse_field_resource_quantity(
    property: &RawProperty,
) -> Result<HydratedFieldResourceQuantity, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "field_resource_quantity")?;
    let mut id = header_id;
    let mut location = None;
    let mut resource = None;
    let mut amount = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "location" => location = Some(read_scalar_text(field, "location")?),
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
    })
}

fn parse_disruption_presence(
    property: &RawProperty,
) -> Result<HydratedDisruptionPresence, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "disruption_presence")?;
    let mut id = header_id;
    let mut location = None;
    let mut resource = None;
    let mut amount = None;
    let mut threshold = None;
    let mut event_kind = None;
    let mut direction = TriggerDirection::Rising;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = read_checked_id(field, &id)?,
            "location" => location = Some(read_scalar_text(field, "location")?),
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

    let mut spatial_location_ids = Vec::new();
    for building in &parsed.production_buildings {
        validate_location_ref(&building.location, root_node, &parsed.span)?;
        validate_positive_amount(
            building.input_amount,
            "production_building.input.amount",
            &parsed.span,
        )?;
        validate_positive_amount(
            building.output_amount,
            "production_building.output.amount",
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
        validate_positive_amount(silo.capacity, "stockpile_silo.capacity", &parsed.span)?;
        validate_non_negative_amount(silo.current, "stockpile_silo.current", &parsed.span)?;
        if silo.current > silo.capacity {
            return Err(HydrateError::new_spanned(
                format!(
                    "stockpile_silo `{}` current {} exceeds capacity {}",
                    silo.id, silo.current, silo.capacity
                ),
                Some(parsed.span.clone()),
            ));
        }
    }
    for overlay in &parsed.owner_policy_overlays {
        validate_owner_ref(&overlay.owner, owners, &parsed.span)?;
    }
    Ok(())
}

fn lower_field_economy(parsed: ParsedFieldEconomy) -> Result<FieldEconomyLowering, HydrateError> {
    let mut resources = BTreeSet::new();
    for building in &parsed.production_buildings {
        resources.insert(building.input_resource.clone());
        resources.insert(building.output_resource.clone());
    }
    for silo in &parsed.stockpile_silos {
        resources.insert(silo.resource.clone());
    }
    for quantity in &parsed.field_resource_quantities {
        resources.insert(quantity.resource.clone());
    }
    for presence in &parsed.disruption_presences {
        resources.insert(presence.resource.clone());
    }

    let mut properties = Vec::new();
    for resource in &resources {
        properties.push(resource_property(&parsed.namespace, resource, "quantity"));
        properties.push(resource_property(&parsed.namespace, resource, "stockpile"));
    }
    for presence in &parsed.disruption_presences {
        properties.push(PropertySpec {
            id: format!("{}_{}_presence", parsed.id, presence.id),
            namespace: parsed.namespace.clone(),
            name: format!("{}_presence", presence.resource),
            display_name: format!("{} presence", presence.resource),
            description: format!(
                "field economy disruption presence authored by `{}`",
                presence.id
            ),
            sub_fields: Vec::new(),
        });
    }
    dedupe_properties(&mut properties);

    let recipes = parsed
        .production_buildings
        .iter()
        .map(|building| ResourceRecipeSpec {
            id: format!("{}_recipe_{}", parsed.id, building.id),
            inputs: vec![RecipeInputSpec {
                property: resource_key(&parsed.namespace, &building.input_resource, "quantity"),
                role: SubFieldRole::Amount,
                unit_cost: building.input_amount,
            }],
            target: resource_key(&parsed.namespace, &building.output_resource, "quantity"),
            target_role: SubFieldRole::Amount,
            throttle_hint_max_per_tick: building.throttle_hint_max_per_tick,
        })
        .collect();

    let transfers = parsed
        .stockpile_silos
        .iter()
        .enumerate()
        .map(|(index, silo)| ResourceTransferSpec {
            id: format!("{}_silo_transfer_{}", parsed.id, silo.id),
            source: resource_key(&parsed.namespace, &silo.resource, "quantity"),
            source_role: SubFieldRole::Amount,
            target: resource_key(&parsed.namespace, &silo.resource, "stockpile"),
            target_role: SubFieldRole::Amount,
            amount: silo.current,
            order_band: index as u32,
        })
        .collect();

    let mut emissions = Vec::new();
    for quantity in &parsed.field_resource_quantities {
        emissions.push(ResourceEmissionSpec {
            id: format!("{}_quantity_emission_{}", parsed.id, quantity.id),
            source: resource_key(&parsed.namespace, &quantity.resource, "quantity"),
            source_role: SubFieldRole::Amount,
            formula: EmissionFormulaSpec::Constant(quantity.amount),
        });
    }
    for presence in &parsed.disruption_presences {
        emissions.push(ResourceEmissionSpec {
            id: format!("{}_presence_emission_{}", parsed.id, presence.id),
            source: PropertyKey::new(
                &parsed.namespace,
                &format!("{}_presence", presence.resource),
            ),
            source_role: SubFieldRole::Amount,
            formula: EmissionFormulaSpec::Constant(presence.amount),
        });
    }

    let emit_on_threshold = parsed
        .disruption_presences
        .iter()
        .map(|presence| EmitOnThresholdSpec {
            id: format!("{}_presence_threshold_{}", parsed.id, presence.id),
            source: PropertyKey::new(
                &parsed.namespace,
                &format!("{}_presence", presence.resource),
            ),
            source_role: SubFieldRole::Amount,
            threshold: presence.threshold,
            direction: presence.direction,
            event_kind: presence.event_kind,
            buffer: EmitBufferSpec::Values,
        })
        .collect();

    let overlays = parsed
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
                    HydratedOwnerPolicyTransform::Multiply(amount) => TransformOp::Multiply(amount),
                },
            )],
            lifecycle: OverlayLifecycle::Permanent,
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            install: InstallTargetSpec::ScenarioListed {
                target_id: overlay.owner.clone(),
            },
        })
        .collect();

    let hydrated = HydratedFieldEconomy {
        id: parsed.id,
        namespace: parsed.namespace,
        production_buildings: parsed.production_buildings,
        stockpile_silos: parsed.stockpile_silos,
        field_resource_quantities: parsed.field_resource_quantities,
        disruption_presences: parsed.disruption_presences,
        owner_policy_overlays: parsed.owner_policy_overlays,
        weight_profiles: parsed.weight_profiles,
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
    })
}

fn resource_property(namespace: &str, resource: &str, suffix: &str) -> PropertySpec {
    PropertySpec {
        id: format!("{namespace}_{resource}_{suffix}"),
        namespace: namespace.to_string(),
        name: format!("{resource}_{suffix}"),
        display_name: format!("{resource} {suffix}"),
        description: "field economy resource surface".into(),
        sub_fields: Vec::new(),
    }
}

fn resource_key(namespace: &str, resource: &str, suffix: &str) -> PropertyKey {
    PropertyKey::new(namespace, &format!("{resource}_{suffix}"))
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
    if node.kind != SimThingKind::Location {
        return Err(HydrateError::new_spanned(
            format!("field_economy target `{location}` is not a Location"),
            Some(span.clone()),
        ));
    }
    Ok(())
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
    if value > 0.0 {
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
