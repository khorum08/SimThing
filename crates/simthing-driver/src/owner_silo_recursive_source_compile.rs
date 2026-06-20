//! OWNER-SILO-RECURSIVE-RF-SOURCE-0 — compile plan for recursive RF owner-silo disburse-down.

use simthing_spec::{
    evaluate_owner_silo_disburse_down_with_rf_source, OwnerSiloRfSourceDisburseReport,
    OwnerSiloRfSourceMode, RuntimeRfTickSourceMode, RuntimeTickId, SimThingScenarioSpec, SpecError,
};

use crate::recursive_local_rf_compile::compile_recursive_local_rf_plan;
use crate::recursive_local_rf_compile::RecursiveLocalRfPlan;
use crate::recursive_rf_reconciliation_compile::compile_recursive_rf_reconciliation_plan;
use crate::recursive_rf_reconciliation_compile::RecursiveRfReconciliationPlan;
use crate::runtime_rf_tick_source_select_compile::{
    compile_runtime_rf_tick_source_selection_plan, RuntimeRfTickSourceSelectionPlan,
};

/// Driver compile plan for recursive RF owner-silo disburse-down integration.
#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloRecursiveSourcePlan {
    pub source_selection_plan: RuntimeRfTickSourceSelectionPlan,
    pub recursive_local_rf_plan: RecursiveLocalRfPlan,
    pub reconciliation_plan: RecursiveRfReconciliationPlan,
    pub disburse_report: OwnerSiloRfSourceDisburseReport,
    pub selected_source_mode: OwnerSiloRfSourceMode,
    pub legacy_default_preserved: bool,
    pub owner_silo_disburse_down_executed_for_selected_source: bool,
    pub local_allocation_integration_deferred: bool,
    pub local_effect_integration_deferred: bool,
    pub semantic_effect_integration_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile owner-silo recursive RF source plan without altering default tick-shell paths.
pub fn compile_owner_silo_recursive_source_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: OwnerSiloRfSourceMode,
) -> Result<OwnerSiloRecursiveSourcePlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let selection_mode = match source_mode {
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo => {
            RuntimeRfTickSourceMode::LegacyDefault
        }
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable => {
            RuntimeRfTickSourceMode::RecursiveSelectable
        }
    };

    let source_selection_plan =
        compile_runtime_rf_tick_source_selection_plan(scenario, tick_id, selection_mode)?;
    let recursive_local_rf_plan = compile_recursive_local_rf_plan(scenario)?;
    let reconciliation_plan = compile_recursive_rf_reconciliation_plan(scenario)?;
    let disburse_report = evaluate_owner_silo_disburse_down_with_rf_source(scenario, source_mode)
        .map_err(|_| SpecError::ValidationFailed)?;

    Ok(OwnerSiloRecursiveSourcePlan {
        selected_source_mode: disburse_report.selected_source_mode,
        legacy_default_preserved: disburse_report.legacy_default_preserved,
        owner_silo_disburse_down_executed_for_selected_source: disburse_report
            .owner_silo_disburse_down_executed_for_selected_source,
        local_allocation_integration_deferred: disburse_report
            .local_allocation_integration_deferred,
        local_effect_integration_deferred: disburse_report.local_effect_integration_deferred,
        semantic_effect_integration_deferred: disburse_report
            .semantic_effect_integration_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        source_selection_plan,
        recursive_local_rf_plan,
        reconciliation_plan,
        disburse_report,
    })
}