//! OWNER-SILO-DISBURSE-DOWN-0 — compile runtime writeback into disburse-down allocation plans.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, owner_silo_demand_aggregate_totals,
    owner_silo_demand_buckets_from_planet_child_rf, OwnerRef, ResourceKey,
    RuntimeOwnerSiloDemandBucket, RuntimeOwnerSiloDisburseDownResult, SimThingScenarioSpec,
    SpecError,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;
use crate::owner_silo_runtime_writeback_compile::compile_owner_silo_runtime_writeback_plan;

/// GPU aggregate proof plan for total requested demand per owner/resource channel.
#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloDemandAggregateProofPlan {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub demand_plan: CompiledAccumulatorOpPlan,
    pub source_demand_indices: Vec<usize>,
}

/// Driver plan for runtime owner-silo disburse-down from writeback availability.
#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloDisburseDownPlan {
    pub writeback_plan: crate::owner_silo_runtime_writeback_compile::OwnerSiloRuntimeWritebackPlan,
    pub demand_buckets: Vec<RuntimeOwnerSiloDemandBucket>,
    pub cpu_results: Vec<RuntimeOwnerSiloDisburseDownResult>,
    pub gpu_demand_aggregate_proof_plans: Vec<OwnerSiloDemandAggregateProofPlan>,
    /// Allocation application to local participant state remains deferred.
    pub allocation_application_deferred: bool,
    /// Scenario authority is never mutated by disburse-down oracle.
    pub scenario_authority_mutation_deferred: bool,
}

/// Compile writeback plan and demand buckets into disburse-down allocation with CPU oracle and GPU demand aggregate proof.
pub fn compile_owner_silo_disburse_down_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<OwnerSiloDisburseDownPlan, SpecError> {
    let writeback_plan = compile_owner_silo_runtime_writeback_plan(scenario)?;

    let demand_buckets = owner_silo_demand_buckets_from_planet_child_rf(scenario)
        .map_err(|_| SpecError::ValidationFailed)?;

    let cpu_results = if demand_buckets.is_empty() {
        Vec::new()
    } else {
        apply_owner_silo_runtime_disburse_down_cpu(&writeback_plan.cpu_results, &demand_buckets)
            .map_err(|_| SpecError::ValidationFailed)?
    };

    let gpu_demand_aggregate_proof_plans = compile_demand_aggregate_proof_plans(&demand_buckets)?;

    Ok(OwnerSiloDisburseDownPlan {
        writeback_plan,
        demand_buckets,
        cpu_results,
        gpu_demand_aggregate_proof_plans,
        allocation_application_deferred: true,
        scenario_authority_mutation_deferred: true,
    })
}

fn compile_demand_aggregate_proof_plans(
    demand_buckets: &[RuntimeOwnerSiloDemandBucket],
) -> Result<Vec<OwnerSiloDemandAggregateProofPlan>, SpecError> {
    if demand_buckets.is_empty() {
        return Ok(Vec::new());
    }

    let totals = owner_silo_demand_aggregate_totals(demand_buckets);
    let mut plans = Vec::with_capacity(totals.len());
    for ((owner_ref, resource_key), _total) in totals {
        let source_demand_indices = demand_buckets
            .iter()
            .enumerate()
            .filter(|(_, bucket)| {
                bucket.owner_ref == owner_ref && bucket.resource_key == resource_key
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if source_demand_indices.is_empty() {
            return Err(SpecError::ValidationFailed);
        }

        let participant_count = source_demand_indices.len() as u32;
        let demand_plan = compile_participant_channel_sum_plan(
            participant_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        plans.push(OwnerSiloDemandAggregateProofPlan {
            owner_ref,
            resource_key,
            demand_plan,
            source_demand_indices,
        });
    }
    Ok(plans)
}

pub fn owner_silo_disburse_down_demand_aggregate_tick_inputs(
    plan: &OwnerSiloDisburseDownPlan,
    proof_plan: &OwnerSiloDemandAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.demand_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &demand_index) in proof_plan.source_demand_indices.iter().enumerate() {
        values[slot] = plan.demand_buckets[demand_index].requested as f32;
    }
    values
}

pub fn owner_silo_disburse_down_demand_aggregate_slot(
    proof_plan: &OwnerSiloDemandAggregateProofPlan,
) -> usize {
    proof_plan.source_demand_indices.len()
}

pub fn owner_silo_disburse_down_cpu_demand_aggregate_total(
    plan: &OwnerSiloDisburseDownPlan,
    proof_plan: &OwnerSiloDemandAggregateProofPlan,
) -> u32 {
    proof_plan
        .source_demand_indices
        .iter()
        .map(|&index| plan.demand_buckets[index].requested)
        .fold(0u32, |acc, v| acc.saturating_add(v))
}
