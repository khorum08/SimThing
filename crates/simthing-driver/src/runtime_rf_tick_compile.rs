//! RUNTIME-RF-TICK-INTEGRATION-0 — compose RF stage plans into one runtime tick boundary.

use simthing_spec::{
    evaluate_runtime_rf_tick, RuntimeRfTickReport, SimThingScenarioSpec, SpecError,
};

use crate::owner_silo_disburse_down_compile::compile_owner_silo_disburse_down_plan;
use crate::owner_silo_runtime_writeback_compile::compile_owner_silo_runtime_writeback_plan;
use crate::planet_child_rf_accumulator_compile::compile_planet_child_rf_gpu_tick_plan;
use crate::planet_child_rf_reduce_up_compile::compile_planet_child_rf_reduce_up_gpu_proof_plan;
use crate::runtime_local_allocation_compile::compile_runtime_local_allocation_application_plan;

/// Summary of stage-local GPU proof plans constructed for the tick boundary.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeRfTickGpuProofSummary {
    pub participant_surplus_plan_ready: bool,
    pub participant_deficit_plan_ready: bool,
    pub reduce_up_bucket_plan_count: u32,
    pub writeback_aggregate_plan_count: u32,
    pub disburse_demand_aggregate_plan_count: u32,
    pub local_allocation_aggregate_plan_count: u32,
}

/// Driver plan composing all RF stage compile surfaces into one tick boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeRfTickPlan {
    pub participant_plan: crate::planet_child_rf_accumulator_compile::PlanetChildRfGpuTickPlan,
    pub reduce_up_plan: crate::planet_child_rf_reduce_up_compile::PlanetChildRfReduceUpGpuProofPlan,
    pub owner_silo_writeback_plan:
        crate::owner_silo_runtime_writeback_compile::OwnerSiloRuntimeWritebackPlan,
    pub owner_silo_disburse_down_plan:
        crate::owner_silo_disburse_down_compile::OwnerSiloDisburseDownPlan,
    pub runtime_local_allocation_plan:
        crate::runtime_local_allocation_compile::RuntimeLocalAllocationApplicationPlan,
    pub tick_report: RuntimeRfTickReport,
    pub gpu_proof_summary: RuntimeRfTickGpuProofSummary,
    pub economy_execution_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub local_effect_application_deferred: bool,
}

/// Compile all RF stage plans and evaluate the composed runtime tick report.
pub fn compile_runtime_rf_tick_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<RuntimeRfTickPlan, SpecError> {
    let participant_plan = compile_planet_child_rf_gpu_tick_plan(scenario)?;
    let reduce_up_plan = compile_planet_child_rf_reduce_up_gpu_proof_plan(scenario)?;
    let owner_silo_writeback_plan = compile_owner_silo_runtime_writeback_plan(scenario)?;
    let owner_silo_disburse_down_plan = compile_owner_silo_disburse_down_plan(scenario)?;
    let runtime_local_allocation_plan =
        compile_runtime_local_allocation_application_plan(scenario)?;

    let tick_report =
        evaluate_runtime_rf_tick(scenario).map_err(|_| SpecError::ValidationFailed)?;

    let gpu_proof_summary = RuntimeRfTickGpuProofSummary {
        participant_surplus_plan_ready: !participant_plan.surplus_plan.ops.is_empty(),
        participant_deficit_plan_ready: !participant_plan.deficit_plan.ops.is_empty(),
        reduce_up_bucket_plan_count: reduce_up_plan.bucket_plans.len() as u32,
        writeback_aggregate_plan_count: owner_silo_writeback_plan.gpu_aggregate_proof_plans.len()
            as u32,
        disburse_demand_aggregate_plan_count: owner_silo_disburse_down_plan
            .gpu_demand_aggregate_proof_plans
            .len() as u32,
        local_allocation_aggregate_plan_count: runtime_local_allocation_plan
            .gpu_allocation_aggregate_proof_plans
            .len() as u32,
    };

    Ok(RuntimeRfTickPlan {
        participant_plan,
        reduce_up_plan,
        owner_silo_writeback_plan,
        owner_silo_disburse_down_plan,
        runtime_local_allocation_plan,
        tick_report,
        gpu_proof_summary,
        economy_execution_deferred: false,
        scenario_authority_mutation_deferred: true,
        local_effect_application_deferred: true,
    })
}
