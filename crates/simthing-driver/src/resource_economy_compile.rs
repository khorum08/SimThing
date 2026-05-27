//! Phase T-3 — Materialize T-2 [`CompiledResourceEconomy`] into existing registration shapes.
//!
//! Maps spec-owned compiled rows to `simthing-core` / `simthing-gpu` registration
//! vectors. Session upload and boundary refresh are T-4.

use std::collections::BTreeMap;

use simthing_core::{
    discrete_transfer_registration_to_op, rebuild_conjunctive_recipe_ops,
    rebuild_discrete_transfer_ops, rebuild_emit_on_threshold_ops, AccumulatorOpBuilderError,
    ConjunctiveRecipeInput, ConjunctiveRecipeRegistration, DimensionRegistry,
    DiscreteTransferRegistration, EmitOnThresholdBuffer, EmitOnThresholdRegistration,
    EmlExpressionRegistry, SimPropertyId, SimThing,
};
use simthing_gpu::{plan_emission_ops, EmissionFormula, EmissionPlanError, EmissionRegistration, SlotAllocator};
use simthing_spec::{
    CompiledEmissionFormula, CompiledResourceEconomy, CompiledResourceEmission, EmitBufferSpec,
};

/// Driver-owned materialized registration bundle (T-3 output).
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceEconomyRegistrations {
    pub transfers: Vec<DiscreteTransferRegistration>,
    pub recipes: Vec<ConjunctiveRecipeRegistration>,
    pub emissions: Vec<EmissionRegistration>,
    pub emit_on_threshold: Vec<EmitOnThresholdRegistration>,
    pub report: ResourceEconomyMaterializationReport,
}

/// Deterministic materialization counts and stable emission identity mapping.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceEconomyMaterializationReport {
    pub transfer_count: usize,
    pub recipe_count: usize,
    pub recipe_input_count: usize,
    pub emission_count: usize,
    pub threshold_emit_count: usize,
    pub eval_eml_emission_count: usize,
    /// Stable emission `reg_idx` keyed by authoring id (sorted id order, not vector order).
    pub emission_reg_idx_by_id: BTreeMap<String, u32>,
    /// Authored transfer ids in compiled order (diagnostics / T-4 refresh scaffold).
    pub transfer_ids: Vec<String>,
    /// Authored recipe ids in compiled order.
    pub recipe_ids: Vec<String>,
    /// Authored threshold-emit ids in compiled order.
    pub threshold_emit_ids: Vec<String>,
    /// OrderBand gate identity per transfer authoring id.
    pub transfer_order_band_by_id: BTreeMap<String, u32>,
}

/// Subtree-scoped refresh scaffold (generation bump wiring lands in T-4).
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceEconomyRegistry {
    pub registrations: ResourceEconomyRegistrations,
    pub generation: u64,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ResourceEconomyCompileError {
    #[error("resource economy references unknown property id {0}")]
    UnknownProperty(u32),

    #[error("resource economy property id {property_id} is not present on the session tree")]
    UnknownPropertyOwner { property_id: u32 },

    #[error("resource economy property id {property_id} owner has no allocated slot")]
    UnknownPropertySlot { property_id: u32 },

    #[error("resource economy transfer `{id}` source and target cells must differ")]
    SameSourceAndTarget { id: String },

    #[error(transparent)]
    TransferBuilder(#[from] AccumulatorOpBuilderError),

    #[error(transparent)]
    EmissionPlan(#[from] EmissionPlanError),
}

/// Materialize compiled resource economy rows into existing registration structs.
///
/// Uses the T-3 flat convention `SimPropertyId.0` as slot (unit tests only).
/// Production session install uses [`materialize_resource_economy_registry_for_session`].
pub fn materialize_resource_economy_registrations(
    compiled: &CompiledResourceEconomy,
    registry: &DimensionRegistry,
    eml_registry: &EmlExpressionRegistry,
) -> Result<ResourceEconomyRegistrations, ResourceEconomyCompileError> {
    materialize_resource_economy_registrations_with_slots(
        compiled,
        registry,
        eml_registry,
        &|property_id| Ok(flat_property_slot(property_id)),
    )
}

/// Materialize with an explicit property→slot resolver (live allocator in T-4).
pub fn materialize_resource_economy_registrations_with_slots(
    compiled: &CompiledResourceEconomy,
    registry: &DimensionRegistry,
    eml_registry: &EmlExpressionRegistry,
    resolve_slot: &dyn Fn(SimPropertyId) -> Result<u32, ResourceEconomyCompileError>,
) -> Result<ResourceEconomyRegistrations, ResourceEconomyCompileError> {
    let emission_reg_idx_by_id = assign_emission_reg_indices(&compiled.emissions);

    let mut transfers = Vec::with_capacity(compiled.transfers.len());
    let mut transfer_ids = Vec::with_capacity(compiled.transfers.len());
    let mut transfer_order_band_by_id = BTreeMap::new();

    for transfer in &compiled.transfers {
        ensure_property_known(registry, transfer.source_property)?;
        ensure_property_known(registry, transfer.target_property)?;

        let source_slot = resolve_slot(transfer.source_property)?;
        let target_slot = resolve_slot(transfer.target_property)?;
        if source_slot == target_slot && transfer.source_col == transfer.target_col {
            return Err(ResourceEconomyCompileError::SameSourceAndTarget {
                id: transfer.id.clone(),
            });
        }

        let reg = DiscreteTransferRegistration {
            source_slot,
            source_col: transfer.source_col,
            target_slot,
            target_col: transfer.target_col,
            amount: transfer.amount,
        };
        discrete_transfer_registration_to_op(&reg)?;
        transfers.push(reg);
        transfer_ids.push(transfer.id.clone());
        transfer_order_band_by_id.insert(transfer.id.clone(), transfer.order_band);
    }

    let mut recipes = Vec::with_capacity(compiled.recipes.len());
    let mut recipe_ids = Vec::with_capacity(compiled.recipes.len());
    for recipe in &compiled.recipes {
        ensure_property_known(registry, recipe.target_property)?;
        let inputs = recipe
            .inputs
            .iter()
            .map(|input| {
                ensure_property_known(registry, input.property)?;
                Ok(ConjunctiveRecipeInput {
                    slot: resolve_slot(input.property)?,
                    col: input.col,
                    unit_cost: input.unit_cost,
                })
            })
            .collect::<Result<Vec<_>, ResourceEconomyCompileError>>()?;

        let reg = ConjunctiveRecipeRegistration {
            inputs,
            target_slot: resolve_slot(recipe.target_property)?,
            target_col: recipe.target_col,
            throttle_hint_max_per_tick: recipe.throttle_hint_max_per_tick,
        };
        rebuild_conjunctive_recipe_ops(std::slice::from_ref(&reg))?;
        recipes.push(reg);
        recipe_ids.push(recipe.id.clone());
    }

    let mut emissions = Vec::with_capacity(compiled.emissions.len());
    for emission in &compiled.emissions {
        ensure_property_known(registry, emission.source_property)?;
        let reg_idx = *emission_reg_idx_by_id
            .get(&emission.id)
            .expect("every compiled emission id receives a reg_idx");
        emissions.push(materialize_emission(
            emission,
            reg_idx,
            eml_registry,
            registry,
            resolve_slot,
        )?);
    }

    let mut emit_on_threshold = Vec::with_capacity(compiled.emit_on_threshold.len());
    let mut threshold_emit_ids = Vec::with_capacity(compiled.emit_on_threshold.len());
    for emit in &compiled.emit_on_threshold {
        ensure_property_known(registry, emit.source_property)?;
        let reg = EmitOnThresholdRegistration {
            slot: resolve_slot(emit.source_property)?,
            col: emit.source_col,
            threshold: emit.threshold,
            direction: emit.direction,
            event_kind: emit.event_kind,
            buffer: map_emit_buffer(emit.buffer),
        };
        rebuild_emit_on_threshold_ops(std::slice::from_ref(&reg));
        emit_on_threshold.push(reg);
        threshold_emit_ids.push(emit.id.clone());
    }

    rebuild_discrete_transfer_ops(&transfers)?;
    rebuild_conjunctive_recipe_ops(&recipes)?;
    plan_emission_ops(&emissions, Some(eml_registry))?;

    let recipe_input_count: usize = recipes.iter().map(|r| r.inputs.len()).sum();
    let eval_eml_emission_count = emissions
        .iter()
        .filter(|e| matches!(e.formula, EmissionFormula::EvalEml { .. }))
        .count();

    let report = ResourceEconomyMaterializationReport {
        transfer_count: transfers.len(),
        recipe_count: recipes.len(),
        recipe_input_count,
        emission_count: emissions.len(),
        threshold_emit_count: emit_on_threshold.len(),
        eval_eml_emission_count,
        emission_reg_idx_by_id,
        transfer_ids,
        recipe_ids,
        threshold_emit_ids,
        transfer_order_band_by_id,
    };

    Ok(ResourceEconomyRegistrations {
        transfers,
        recipes,
        emissions,
        emit_on_threshold,
        report,
    })
}

/// Materialize for production session install using live tree + allocator slots.
pub fn materialize_resource_economy_registry_for_session(
    compiled: &CompiledResourceEconomy,
    registry: &DimensionRegistry,
    eml_registry: &EmlExpressionRegistry,
    root: &SimThing,
    allocator: &SlotAllocator,
) -> Result<ResourceEconomyRegistry, ResourceEconomyCompileError> {
    let resolve = |property_id: SimPropertyId| {
        resolve_live_property_slot(property_id, root, allocator)
    };
    Ok(ResourceEconomyRegistry {
        registrations: materialize_resource_economy_registrations_with_slots(
            compiled,
            registry,
            eml_registry,
            &resolve,
        )?,
        generation: 1,
    })
}

/// Wrap materialized registrations in the subtree-refresh registry scaffold.
pub fn materialize_resource_economy_registry(
    compiled: &CompiledResourceEconomy,
    registry: &DimensionRegistry,
    eml_registry: &EmlExpressionRegistry,
) -> Result<ResourceEconomyRegistry, ResourceEconomyCompileError> {
    Ok(ResourceEconomyRegistry {
        registrations: materialize_resource_economy_registrations(
            compiled,
            registry,
            eml_registry,
        )?,
        generation: 1,
    })
}

fn assign_emission_reg_indices(
    emissions: &[CompiledResourceEmission],
) -> BTreeMap<String, u32> {
    let mut ids: Vec<&str> = emissions.iter().map(|e| e.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    ids.into_iter()
        .enumerate()
        .map(|(idx, id)| (id.to_string(), idx as u32))
        .collect()
}

fn materialize_emission(
    emission: &CompiledResourceEmission,
    reg_idx: u32,
    _eml_registry: &EmlExpressionRegistry,
    registry: &DimensionRegistry,
    resolve_slot: &dyn Fn(SimPropertyId) -> Result<u32, ResourceEconomyCompileError>,
) -> Result<EmissionRegistration, ResourceEconomyCompileError> {
    ensure_property_known(registry, emission.source_property)?;
    let (formula, tree_id) = match &emission.formula {
        CompiledEmissionFormula::IdentityFloor => (EmissionFormula::IdentityFloor, None),
        CompiledEmissionFormula::Constant(value) => (
            EmissionFormula::Constant { value: *value },
            None,
        ),
        CompiledEmissionFormula::EvalEml { tree_id, .. } => (
            EmissionFormula::EvalEml { tree_id: *tree_id },
            Some(*tree_id),
        ),
    };

    Ok(EmissionRegistration {
        source_slot: resolve_slot(emission.source_property)?,
        source_col: emission.source_col,
        tree_id,
        formula,
        max_emit: None,
        reg_idx,
    })
}

fn map_emit_buffer(buffer: EmitBufferSpec) -> EmitOnThresholdBuffer {
    match buffer {
        EmitBufferSpec::Values => EmitOnThresholdBuffer::Values,
        EmitBufferSpec::Output => EmitOnThresholdBuffer::Output,
    }
}

fn ensure_property_known(
    registry: &DimensionRegistry,
    property_id: SimPropertyId,
) -> Result<(), ResourceEconomyCompileError> {
    if registry.try_property(property_id).is_none() {
        return Err(ResourceEconomyCompileError::UnknownProperty(property_id.0));
    }
    Ok(())
}

/// Resolve a property to the live GPU slot of its owning SimThing node.
pub fn resolve_live_property_slot(
    property_id: SimPropertyId,
    root: &SimThing,
    allocator: &SlotAllocator,
) -> Result<u32, ResourceEconomyCompileError> {
    let owner = find_property_owner(root, property_id).ok_or(
        ResourceEconomyCompileError::UnknownPropertyOwner {
            property_id: property_id.0,
        },
    )?;
    allocator
        .slot_of(owner)
        .ok_or(ResourceEconomyCompileError::UnknownPropertySlot {
            property_id: property_id.0,
        })
}

pub fn find_property_owner(root: &SimThing, property_id: SimPropertyId) -> Option<simthing_core::SimThingId> {
    if root.properties.contains_key(&property_id) {
        return Some(root.id);
    }
    for child in &root.children {
        if let Some(id) = find_property_owner(child, property_id) {
            return Some(id);
        }
    }
    None
}

/// T-3 flat slot convention for unit tests without a live session tree.
fn flat_property_slot(property_id: SimPropertyId) -> u32 {
    property_id.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{ClampBehavior, PropertyLayout, SimProperty, SubFieldRole, SubFieldSpec};
    use simthing_spec::{
        compile_resource_economy, EmissionFormulaSpec, PropertyKey, ResourceEconomySpec,
        ResourceEmissionSpec,
    };

    fn register_amount(reg: &mut DimensionRegistry, name: &str) -> SimPropertyId {
        reg.register(SimProperty {
            namespace: "core".into(),
            name: name.into(),
            layout: PropertyLayout {
                sub_fields: vec![SubFieldSpec {
                    role: SubFieldRole::Named("amount".into()),
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
                }],
            },
            decay: None,
            intensity_behavior: None,
            fission_templates: vec![],
            fusion_templates: vec![],
            on_expire: None,
            description: String::new(),
            intensity_labels: vec![],
        })
    }

    #[test]
    fn emission_reg_idx_sorted_by_authoring_id() {
        let mut reg = DimensionRegistry::new();
        register_amount(&mut reg, "food");
        let eml = EmlExpressionRegistry::new();
        let spec = ResourceEconomySpec {
            emissions: vec![
                ResourceEmissionSpec {
                    id: "z_last".into(),
                    source: PropertyKey::new("core", "food"),
                    source_role: SubFieldRole::Named("amount".into()),
                    formula: EmissionFormulaSpec::IdentityFloor,
                },
                ResourceEmissionSpec {
                    id: "a_first".into(),
                    source: PropertyKey::new("core", "food"),
                    source_role: SubFieldRole::Named("amount".into()),
                    formula: EmissionFormulaSpec::Constant(1.0),
                },
            ],
            ..Default::default()
        };
        let compiled = compile_resource_economy(&spec, &reg, &eml).unwrap();
        let materialized =
            materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();
        assert_eq!(
            materialized.report.emission_reg_idx_by_id.get("a_first"),
            Some(&0)
        );
        assert_eq!(
            materialized.report.emission_reg_idx_by_id.get("z_last"),
            Some(&1)
        );
    }
}
