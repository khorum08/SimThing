//! RUNTIME-PARTICIPANT-STATE-MUTATION-0 — compile plan for runtime participant state mutation.

use simthing_spec::{
    evaluate_runtime_participant_state_mutation, RuntimeParticipantStateMutationReport,
    RuntimeParticipantStateMutationSourceMode, RuntimeTickId, SimThingScenarioSpec, SpecError,
};

use crate::semantic_participant_delta_preview_compile::{
    compile_semantic_participant_delta_preview_plan, SemanticParticipantDeltaPreviewPlan,
};

/// Driver compile plan for runtime participant state mutation integration.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantStateMutationPlan {
    pub delta_preview_plan: SemanticParticipantDeltaPreviewPlan,
    pub mutation_report: RuntimeParticipantStateMutationReport,
    pub selected_source_mode: RuntimeParticipantStateMutationSourceMode,
    pub runtime_state_mutation_applied: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile runtime participant state mutation plan without altering default semantic paths.
pub fn compile_runtime_participant_state_mutation_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: RuntimeParticipantStateMutationSourceMode,
    replay_count: u32,
) -> Result<RuntimeParticipantStateMutationPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let delta_preview_mode = match source_mode {
        RuntimeParticipantStateMutationSourceMode::LegacyPlanetChildOwnerSilo => {
            simthing_spec::ParticipantDeltaPreviewSourceMode::LegacyPlanetChildOwnerSilo
        }
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable => {
            simthing_spec::ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable
        }
    };

    let delta_preview_plan = compile_semantic_participant_delta_preview_plan(
        scenario,
        tick_id,
        delta_preview_mode,
        replay_count,
    )?;
    let mutation_report =
        evaluate_runtime_participant_state_mutation(scenario, tick_id, source_mode, replay_count)
            .map_err(|_| SpecError::ValidationFailed)?;

    Ok(RuntimeParticipantStateMutationPlan {
        selected_source_mode: mutation_report.selected_source_mode,
        runtime_state_mutation_applied: mutation_report.runtime_state_mutation_applied,
        participant_property_mutation_deferred: mutation_report
            .participant_property_mutation_deferred,
        scenario_authority_mutation_deferred: mutation_report.scenario_authority_mutation_deferred,
        savefile_mutation_deferred: mutation_report.savefile_mutation_deferred,
        persistent_history_deferred: mutation_report.persistent_history_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        delta_preview_plan,
        mutation_report,
    })
}
