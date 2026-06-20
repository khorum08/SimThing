//! RUNTIME-TICK-RECURSIVE-RF-SELECTABLE-SOURCE-0 — explicit selectable RF report source compile plan.

use simthing_spec::{
    evaluate_runtime_rf_tick_source_selection, RuntimeRfTickSelectedSourceReport,
    RuntimeRfTickSourceKind, RuntimeRfTickSourceSelectionMode, RuntimeTickId, SimThingScenarioSpec,
    SpecError,
};

use crate::runtime_rf_tick_compile::compile_runtime_rf_tick_plan;
use crate::runtime_rf_tick_compile::RuntimeRfTickPlan;
use crate::runtime_rf_tick_source_compile::{
    compile_runtime_rf_tick_source_comparison_plan, RuntimeRfTickSourceComparisonPlan,
};
use crate::runtime_tick_shell_compile::{compile_runtime_tick_shell_plan, RuntimeTickShellPlan};

/// Driver plan for explicit RF tick source selection (report-only in this rung).
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeRfTickSourceSelectionPlan {
    pub comparison_plan: RuntimeRfTickSourceComparisonPlan,
    pub selected_source_report: RuntimeRfTickSelectedSourceReport,
    pub default_tick_plan: RuntimeRfTickPlan,
    pub default_tick_shell_plan: RuntimeTickShellPlan,
    pub selected_source_kind: RuntimeRfTickSourceKind,
    pub recursive_selected_for_rf_report_only: bool,
    pub legacy_default_preserved: bool,
    pub owner_silo_integration_deferred: bool,
    pub local_allocation_integration_deferred: bool,
    pub semantic_effect_integration_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Tick-shell wrapper with selectable RF report source (default shell unchanged).
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeTickShellSelectableRfSourcePlan {
    pub tick_id: RuntimeTickId,
    pub default_tick_shell_plan: RuntimeTickShellPlan,
    pub source_selection_plan: RuntimeRfTickSourceSelectionPlan,
    pub default_tick_shell_preserved: bool,
    pub selected_source_report_only: bool,
}

/// Compile explicit RF source selection without altering default tick-shell paths.
pub fn compile_runtime_rf_tick_source_selection_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    selection_mode: RuntimeRfTickSourceSelectionMode,
) -> Result<RuntimeRfTickSourceSelectionPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let comparison_plan = compile_runtime_rf_tick_source_comparison_plan(scenario)?;
    let default_tick_plan = compile_runtime_rf_tick_plan(scenario)?;
    let default_tick_shell_plan = compile_runtime_tick_shell_plan(scenario, tick_id)?;
    let selected_source_report =
        evaluate_runtime_rf_tick_source_selection(scenario, selection_mode)
            .map_err(|_| SpecError::ValidationFailed)?;

    Ok(RuntimeRfTickSourceSelectionPlan {
        selected_source_kind: selected_source_report.selection_gate.selected_source_kind,
        recursive_selected_for_rf_report_only: selected_source_report
            .recursive_source_selected_for_rf_report_only,
        legacy_default_preserved: selected_source_report
            .selection_gate
            .legacy_default_preserved,
        owner_silo_integration_deferred: selected_source_report.owner_silo_integration_deferred,
        local_allocation_integration_deferred: selected_source_report
            .local_allocation_integration_deferred,
        semantic_effect_integration_deferred: selected_source_report
            .semantic_effect_integration_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        comparison_plan,
        selected_source_report,
        default_tick_plan,
        default_tick_shell_plan,
    })
}

/// Compile tick shell wrapper with selectable RF report source.
pub fn compile_runtime_tick_shell_with_selectable_rf_source_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    selection_mode: RuntimeRfTickSourceSelectionMode,
) -> Result<RuntimeTickShellSelectableRfSourcePlan, SpecError> {
    let default_tick_shell_plan = compile_runtime_tick_shell_plan(scenario, tick_id)?;
    let source_selection_plan =
        compile_runtime_rf_tick_source_selection_plan(scenario, tick_id, selection_mode)?;

    Ok(RuntimeTickShellSelectableRfSourcePlan {
        tick_id,
        default_tick_shell_preserved: true,
        selected_source_report_only: true,
        default_tick_shell_plan,
        source_selection_plan,
    })
}
