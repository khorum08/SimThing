//! RUNTIME-TICK-EXECUTION-SHELL-0 — scheduler/report shell over composed RF tick plan.

use simthing_spec::{
    evaluate_runtime_tick_shell, RuntimeTickExecutionReport, RuntimeTickId, SimThingScenarioSpec,
    SpecError,
};

use crate::runtime_rf_tick_compile::compile_runtime_rf_tick_plan;

/// Summary of stage-local GPU proof availability for the tick shell boundary.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeTickShellGpuProofSummary {
    pub stage_local_gpu_proofs_available: bool,
    pub fused_tick_kernel_present: bool,
    pub new_gpu_primitive_required: bool,
}

/// Driver plan composing RF tick plan into a scheduler/report shell.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeTickShellPlan {
    pub tick_id: RuntimeTickId,
    pub runtime_rf_tick_plan: crate::runtime_rf_tick_compile::RuntimeRfTickPlan,
    pub execution_report: RuntimeTickExecutionReport,
    pub gpu_stage_proof_summary: RuntimeTickShellGpuProofSummary,
    pub economy_execution_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub local_effect_application_deferred: bool,
}

/// Compile the composed RF tick plan and evaluate the runtime tick execution shell.
pub fn compile_runtime_tick_shell_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
) -> Result<RuntimeTickShellPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let runtime_rf_tick_plan = compile_runtime_rf_tick_plan(scenario)?;

    let mut execution_report =
        evaluate_runtime_tick_shell(scenario, tick_id).map_err(|_| SpecError::ValidationFailed)?;

    let gpu_summary = &runtime_rf_tick_plan.gpu_proof_summary;
    let stage_local_gpu_proofs_available = gpu_summary.participant_surplus_plan_ready
        && gpu_summary.participant_deficit_plan_ready
        && gpu_summary.reduce_up_bucket_plan_count > 0
        && gpu_summary.writeback_aggregate_plan_count > 0;

    execution_report.gpu_stage_proof_available = stage_local_gpu_proofs_available;

    let gpu_stage_proof_summary = RuntimeTickShellGpuProofSummary {
        stage_local_gpu_proofs_available,
        fused_tick_kernel_present: false,
        new_gpu_primitive_required: false,
    };

    Ok(RuntimeTickShellPlan {
        tick_id,
        runtime_rf_tick_plan,
        execution_report,
        gpu_stage_proof_summary,
        economy_execution_deferred: false,
        scenario_authority_mutation_deferred: true,
        local_effect_application_deferred: true,
    })
}
