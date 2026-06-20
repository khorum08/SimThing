//! LOCAL-EFFECT-APPLICATION-RECURSIVE-RF-SOURCE-0 — compile plan for recursive RF local effects.

use simthing_spec::{
    evaluate_local_effect_application_with_rf_source, LocalAllocationRfSourceMode,
    LocalEffectApplicationRfSourceReport, LocalEffectRfSourceMode, RuntimeTickId,
    SimThingScenarioSpec, SpecError,
};

use crate::local_allocation_recursive_source_compile::{
    compile_local_allocation_recursive_source_plan, LocalAllocationRecursiveSourcePlan,
};

/// Driver compile plan for recursive RF local effect integration.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalEffectRecursiveSourcePlan {
    pub local_allocation_recursive_source_plan: LocalAllocationRecursiveSourcePlan,
    pub local_effect_report: LocalEffectApplicationRfSourceReport,
    pub selected_source_mode: LocalEffectRfSourceMode,
    pub legacy_default_preserved: bool,
    pub local_participant_effects_executed_for_selected_source: bool,
    pub local_effect_application_executed_for_selected_source: bool,
    pub semantic_effect_integration_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile recursive RF local effect plan without altering default semantic paths.
pub fn compile_local_effect_recursive_source_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: LocalEffectRfSourceMode,
) -> Result<LocalEffectRecursiveSourcePlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let allocation_mode = match source_mode {
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable => {
            LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable
        }
    };

    let local_allocation_recursive_source_plan =
        compile_local_allocation_recursive_source_plan(scenario, tick_id, allocation_mode)?;
    let local_effect_report =
        evaluate_local_effect_application_with_rf_source(scenario, tick_id, source_mode)
            .map_err(|_| SpecError::ValidationFailed)?;

    Ok(LocalEffectRecursiveSourcePlan {
        selected_source_mode: local_effect_report.selected_source_mode,
        legacy_default_preserved: local_effect_report.legacy_default_preserved,
        local_participant_effects_executed_for_selected_source: local_effect_report
            .local_participant_effects_executed_for_selected_source,
        local_effect_application_executed_for_selected_source: local_effect_report
            .local_effect_application_executed_for_selected_source,
        semantic_effect_integration_deferred: local_effect_report
            .semantic_effect_integration_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        local_allocation_recursive_source_plan,
        local_effect_report,
    })
}
