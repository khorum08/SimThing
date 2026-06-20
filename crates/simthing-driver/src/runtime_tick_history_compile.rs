//! RUNTIME-TICK-HISTORY-REPLAY-0 — compile tick history and replay proof plans.

use simthing_spec::{
    evaluate_runtime_tick_history_entry, replay_runtime_tick_history, RuntimeTickHistoryEntry,
    RuntimeTickId, RuntimeTickReplayReport, SimThingScenarioSpec, SpecError,
};

use crate::local_participant_effects_compile::compile_local_participant_effects_plan;
use crate::runtime_tick_shell_compile::compile_runtime_tick_shell_plan;

/// Driver plan composing tick shell, local effects, history entry, and replay report.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeTickHistoryPlan {
    pub tick_id: RuntimeTickId,
    pub tick_shell_plan: crate::runtime_tick_shell_compile::RuntimeTickShellPlan,
    pub local_participant_effects_plan:
        crate::local_participant_effects_compile::LocalParticipantEffectsPlan,
    pub history_entry: RuntimeTickHistoryEntry,
    pub replay_report: RuntimeTickReplayReport,
    pub gpu_stage_proof_summary_available: bool,
    pub fused_replay_kernel_present: bool,
    pub new_gpu_primitive_required: bool,
    pub persistent_history_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
}

/// Compile tick shell + local effects into history entry and replay proof.
pub fn compile_runtime_tick_history_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    replay_count: u32,
) -> Result<RuntimeTickHistoryPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let tick_shell_plan = compile_runtime_tick_shell_plan(scenario, tick_id)?;
    let local_participant_effects_plan = compile_local_participant_effects_plan(scenario, tick_id)?;

    let history_entry = evaluate_runtime_tick_history_entry(scenario, tick_id)
        .map_err(|_| SpecError::ValidationFailed)?;

    let replay_report = replay_runtime_tick_history(scenario, tick_id, replay_count)
        .map_err(|_| SpecError::ValidationFailed)?;

    let gpu_stage_proof_summary_available = tick_shell_plan
        .gpu_stage_proof_summary
        .stage_local_gpu_proofs_available;

    Ok(RuntimeTickHistoryPlan {
        tick_id,
        tick_shell_plan,
        local_participant_effects_plan,
        history_entry,
        replay_report,
        gpu_stage_proof_summary_available,
        fused_replay_kernel_present: false,
        new_gpu_primitive_required: false,
        persistent_history_deferred: true,
        scenario_authority_mutation_deferred: true,
    })
}
