//! SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 — compile plan for participant delta preview.

use simthing_spec::{
    evaluate_semantic_participant_delta_preview, ParticipantDeltaPreviewSourceMode, RuntimeTickId,
    SemanticParticipantDeltaPreviewReport, SimThingScenarioSpec, SpecError,
};

use crate::semantic_effect_execution_boundary_compile::{
    compile_semantic_effect_execution_boundary_plan, SemanticEffectExecutionBoundaryPlan,
};

/// Driver compile plan for semantic participant delta preview integration.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticParticipantDeltaPreviewPlan {
    pub execution_boundary_plan: SemanticEffectExecutionBoundaryPlan,
    pub delta_preview_report: SemanticParticipantDeltaPreviewReport,
    pub selected_source_mode: ParticipantDeltaPreviewSourceMode,
    pub delta_preview_boundary_proven: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile semantic participant delta preview plan without altering default semantic paths.
pub fn compile_semantic_participant_delta_preview_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: ParticipantDeltaPreviewSourceMode,
    replay_count: u32,
) -> Result<SemanticParticipantDeltaPreviewPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let execution_boundary_mode = match source_mode {
        ParticipantDeltaPreviewSourceMode::LegacyPlanetChildOwnerSilo => {
            simthing_spec::SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo
        }
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable => {
            simthing_spec::SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable
        }
    };

    let execution_boundary_plan = compile_semantic_effect_execution_boundary_plan(
        scenario,
        tick_id,
        execution_boundary_mode,
        replay_count,
    )?;
    let delta_preview_report =
        evaluate_semantic_participant_delta_preview(scenario, tick_id, source_mode, replay_count)
            .map_err(|_| SpecError::ValidationFailed)?;

    let delta_preview_boundary_proven = delta_preview_report.delta_preview_count > 0
        || delta_preview_report.preview_amount_total > 0;

    Ok(SemanticParticipantDeltaPreviewPlan {
        selected_source_mode: delta_preview_report.selected_source_mode,
        delta_preview_boundary_proven,
        participant_property_mutation_deferred: delta_preview_report
            .participant_property_mutation_deferred,
        scenario_authority_mutation_deferred: delta_preview_report
            .scenario_authority_mutation_deferred,
        savefile_mutation_deferred: delta_preview_report.savefile_mutation_deferred,
        persistent_history_deferred: delta_preview_report.persistent_history_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        execution_boundary_plan,
        delta_preview_report,
    })
}
