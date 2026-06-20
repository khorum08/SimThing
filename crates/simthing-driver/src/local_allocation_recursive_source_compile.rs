//! LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 — compile plan for recursive RF local allocation.

use simthing_spec::{
    evaluate_runtime_local_allocation_with_rf_source, LocalAllocationRfSourceMode,
    OwnerSiloRfSourceMode, RuntimeLocalAllocationRfSourceReport, RuntimeTickId,
    SimThingScenarioSpec, SpecError,
};

use crate::owner_silo_recursive_source_compile::{
    compile_owner_silo_recursive_source_plan, OwnerSiloRecursiveSourcePlan,
};

/// Driver compile plan for recursive RF local allocation integration.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalAllocationRecursiveSourcePlan {
    pub owner_silo_recursive_source_plan: OwnerSiloRecursiveSourcePlan,
    pub allocation_report: RuntimeLocalAllocationRfSourceReport,
    pub selected_source_mode: LocalAllocationRfSourceMode,
    pub legacy_default_preserved: bool,
    pub local_allocation_executed_for_selected_source: bool,
    pub local_effect_integration_deferred: bool,
    pub semantic_effect_integration_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile recursive RF local allocation plan without altering default tick-shell paths.
pub fn compile_local_allocation_recursive_source_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: LocalAllocationRfSourceMode,
) -> Result<LocalAllocationRecursiveSourcePlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let owner_silo_mode = match source_mode {
        LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo => {
            OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable => {
            OwnerSiloRfSourceMode::RecursiveLocalRfSelectable
        }
    };

    let owner_silo_recursive_source_plan =
        compile_owner_silo_recursive_source_plan(scenario, tick_id, owner_silo_mode)?;
    let allocation_report = evaluate_runtime_local_allocation_with_rf_source(scenario, source_mode)
        .map_err(|_| SpecError::ValidationFailed)?;

    Ok(LocalAllocationRecursiveSourcePlan {
        selected_source_mode: allocation_report.selected_source_mode,
        legacy_default_preserved: allocation_report.legacy_default_preserved,
        local_allocation_executed_for_selected_source: allocation_report
            .local_allocation_executed_for_selected_source,
        local_effect_integration_deferred: allocation_report.local_effect_integration_deferred,
        semantic_effect_integration_deferred: allocation_report
            .semantic_effect_integration_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        owner_silo_recursive_source_plan,
        allocation_report,
    })
}
