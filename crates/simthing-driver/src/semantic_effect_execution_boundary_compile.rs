//! SEMANTIC-EFFECT-EXECUTION-BOUNDARY-0 — compile plan for semantic execution boundary.

use simthing_spec::{
    evaluate_semantic_effect_execution_boundary, RuntimeTickId,
    SemanticEffectExecutionBoundaryReport, SemanticEffectExecutionSourceMode, SimThingScenarioSpec,
    SpecError,
};

use crate::semantic_local_effects_recursive_source_compile::{
    compile_semantic_local_effects_recursive_source_plan, SemanticLocalEffectsRecursiveSourcePlan,
};

/// Driver compile plan for semantic effect execution boundary integration.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticEffectExecutionBoundaryPlan {
    pub semantic_recursive_source_plan: SemanticLocalEffectsRecursiveSourcePlan,
    pub execution_report: SemanticEffectExecutionBoundaryReport,
    pub selected_source_mode: SemanticEffectExecutionSourceMode,
    pub legacy_default_preserved: bool,
    pub semantic_execution_boundary_proven: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile semantic effect execution boundary plan without altering default semantic paths.
pub fn compile_semantic_effect_execution_boundary_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: SemanticEffectExecutionSourceMode,
    replay_count: u32,
) -> Result<SemanticEffectExecutionBoundaryPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let semantic_rf_mode = match source_mode {
        SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo => {
            simthing_spec::SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable => {
            simthing_spec::SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable
        }
    };

    let semantic_recursive_source_plan = compile_semantic_local_effects_recursive_source_plan(
        scenario,
        tick_id,
        semantic_rf_mode,
        replay_count,
    )?;
    let execution_report =
        evaluate_semantic_effect_execution_boundary(scenario, tick_id, source_mode, replay_count)
            .map_err(|_| SpecError::ValidationFailed)?;

    Ok(SemanticEffectExecutionBoundaryPlan {
        selected_source_mode: execution_report.selected_source_mode,
        legacy_default_preserved: execution_report.legacy_default_preserved,
        semantic_execution_boundary_proven: execution_report.semantic_execution_boundary_proven,
        participant_property_mutation_deferred: execution_report
            .participant_property_mutation_deferred,
        scenario_authority_mutation_deferred: execution_report.scenario_authority_mutation_deferred,
        savefile_mutation_deferred: execution_report.savefile_mutation_deferred,
        persistent_history_deferred: execution_report.persistent_history_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        semantic_recursive_source_plan,
        execution_report,
    })
}
