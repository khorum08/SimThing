//! RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 — compile disburse-down into runtime allocation application plans.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    apply_runtime_local_allocations_from_disburse_down, runtime_local_allocation_aggregate_totals,
    OwnerRef, ResourceKey, RuntimeLocalAllocationApplicationReport, SimThingScenarioSpec,
    SpecError,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;
use crate::owner_silo_disburse_down_compile::compile_owner_silo_disburse_down_plan;

/// GPU aggregate proof plan for total allocated amount per owner/resource channel.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeLocalAllocationAggregateProofPlan {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub allocation_plan: CompiledAccumulatorOpPlan,
    pub source_allocation_indices: Vec<usize>,
}

/// Driver plan for runtime-local allocation application from disburse-down results.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeLocalAllocationApplicationPlan {
    pub disburse_down_plan: crate::owner_silo_disburse_down_compile::OwnerSiloDisburseDownPlan,
    pub application_report: RuntimeLocalAllocationApplicationReport,
    pub gpu_allocation_aggregate_proof_plans: Vec<RuntimeLocalAllocationAggregateProofPlan>,
    pub economy_execution_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
}

/// Compile disburse-down plan into runtime-local allocation application with CPU oracle and GPU aggregate proof.
pub fn compile_runtime_local_allocation_application_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<RuntimeLocalAllocationApplicationPlan, SpecError> {
    let disburse_down_plan = compile_owner_silo_disburse_down_plan(scenario)?;

    let application_report =
        apply_runtime_local_allocations_from_disburse_down(&disburse_down_plan.cpu_results)
            .map_err(|_| SpecError::ValidationFailed)?;

    let gpu_allocation_aggregate_proof_plans =
        compile_allocation_aggregate_proof_plans(&application_report)?;

    Ok(RuntimeLocalAllocationApplicationPlan {
        disburse_down_plan,
        application_report,
        gpu_allocation_aggregate_proof_plans,
        economy_execution_deferred: true,
        scenario_authority_mutation_deferred: true,
    })
}

fn compile_allocation_aggregate_proof_plans(
    report: &RuntimeLocalAllocationApplicationReport,
) -> Result<Vec<RuntimeLocalAllocationAggregateProofPlan>, SpecError> {
    if report.states.is_empty() {
        return Ok(Vec::new());
    }

    let totals = runtime_local_allocation_aggregate_totals(report);
    let mut plans = Vec::with_capacity(totals.len());
    for ((owner_ref, resource_key), _total) in totals {
        let source_allocation_indices = report
            .states
            .iter()
            .enumerate()
            .filter(|(_, state)| state.owner_ref == owner_ref && state.resource_key == resource_key)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if source_allocation_indices.is_empty() {
            return Err(SpecError::ValidationFailed);
        }

        let participant_count = source_allocation_indices.len() as u32;
        let allocation_plan = compile_participant_channel_sum_plan(
            participant_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        plans.push(RuntimeLocalAllocationAggregateProofPlan {
            owner_ref,
            resource_key,
            allocation_plan,
            source_allocation_indices,
        });
    }
    Ok(plans)
}

pub fn runtime_local_allocation_aggregate_tick_inputs(
    plan: &RuntimeLocalAllocationApplicationPlan,
    proof_plan: &RuntimeLocalAllocationAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.allocation_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &state_index) in proof_plan.source_allocation_indices.iter().enumerate() {
        values[slot] = plan.application_report.states[state_index].allocated as f32;
    }
    values
}

pub fn runtime_local_allocation_aggregate_slot(
    proof_plan: &RuntimeLocalAllocationAggregateProofPlan,
) -> usize {
    proof_plan.source_allocation_indices.len()
}

pub fn runtime_local_allocation_cpu_aggregate_total(
    plan: &RuntimeLocalAllocationApplicationPlan,
    proof_plan: &RuntimeLocalAllocationAggregateProofPlan,
) -> u32 {
    proof_plan
        .source_allocation_indices
        .iter()
        .map(|&index| plan.application_report.states[index].allocated)
        .fold(0u32, |acc, v| acc.saturating_add(v))
}
