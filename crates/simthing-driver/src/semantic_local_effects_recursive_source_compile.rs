//! SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 — compile plan for recursive RF semantic local effects.

use simthing_spec::{
    evaluate_semantic_local_effects_with_rf_source, LocalEffectRfSourceMode, RuntimeTickId,
    SemanticLocalEffectRfSourceMode, SemanticLocalEffectRfSourceReport, SimThingScenarioSpec,
    SpecError,
};

use crate::local_effect_recursive_source_compile::compile_local_effect_recursive_source_plan;

/// Driver compile plan for recursive RF semantic local effect integration.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticLocalEffectsRecursiveSourcePlan {
    pub local_effect_recursive_source_plan:
        crate::local_effect_recursive_source_compile::LocalEffectRecursiveSourcePlan,
    pub semantic_report: SemanticLocalEffectRfSourceReport,
    pub selected_source_mode: SemanticLocalEffectRfSourceMode,
    pub legacy_default_preserved: bool,
    pub semantic_local_effects_projected_for_selected_source: bool,
    pub semantic_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile recursive RF semantic local effect plan without altering default semantic paths.
pub fn compile_semantic_local_effects_recursive_source_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: SemanticLocalEffectRfSourceMode,
    replay_count: u32,
) -> Result<SemanticLocalEffectsRecursiveSourcePlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let local_effect_mode = match source_mode {
        SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable => {
            LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable
        }
    };

    let local_effect_recursive_source_plan =
        compile_local_effect_recursive_source_plan(scenario, tick_id, local_effect_mode)?;
    let semantic_report = evaluate_semantic_local_effects_with_rf_source(
        scenario,
        tick_id,
        source_mode,
        replay_count,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    Ok(SemanticLocalEffectsRecursiveSourcePlan {
        selected_source_mode: semantic_report.selected_source_mode,
        legacy_default_preserved: semantic_report.legacy_default_preserved,
        semantic_local_effects_projected_for_selected_source: semantic_report
            .semantic_local_effects_projected_for_selected_source,
        semantic_execution_deferred: semantic_report.semantic_execution_deferred,
        participant_property_mutation_deferred: semantic_report
            .participant_property_mutation_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        local_effect_recursive_source_plan,
        semantic_report,
    })
}
