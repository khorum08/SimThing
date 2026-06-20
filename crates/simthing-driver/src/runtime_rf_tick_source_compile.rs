//! RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 — optional side-by-side RF tick source comparison compile plan.

use simthing_spec::{
    evaluate_runtime_rf_tick_source_comparison, RuntimeRfTickSourceComparisonReport,
    RuntimeRfTickSourceKind, RuntimeTickId, SimThingScenarioSpec, SpecError,
};

use crate::recursive_rf_reconciliation_compile::{
    compile_recursive_rf_reconciliation_plan, RecursiveRfReconciliationPlan,
};
use crate::runtime_rf_tick_compile::{compile_runtime_rf_tick_plan, RuntimeRfTickPlan};
use crate::runtime_tick_shell_compile::{compile_runtime_tick_shell_plan, RuntimeTickShellPlan};

/// Driver plan composing legacy, recursive, and reconciliation tick-source proof surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeRfTickSourceComparisonPlan {
    pub legacy_tick_plan: RuntimeRfTickPlan,
    pub recursive_local_rf_plan: crate::recursive_local_rf_compile::RecursiveLocalRfPlan,
    pub reconciliation_plan: RecursiveRfReconciliationPlan,
    pub comparison_report: RuntimeRfTickSourceComparisonReport,
    pub default_source_kind: RuntimeRfTickSourceKind,
    pub selected_source_kind: RuntimeRfTickSourceKind,
    pub recursive_source_preview_only: bool,
    pub tick_shell_source_replacement_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Tick-shell wrapper carrying optional side-by-side RF source comparison (preview/report only).
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeTickShellRfSourceComparisonPlan {
    pub tick_id: RuntimeTickId,
    pub tick_shell_plan: RuntimeTickShellPlan,
    pub rf_source_comparison_plan: RuntimeRfTickSourceComparisonPlan,
}

/// Compile side-by-side legacy vs recursive RF tick source comparison without altering default paths.
pub fn compile_runtime_rf_tick_source_comparison_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<RuntimeRfTickSourceComparisonPlan, SpecError> {
    let legacy_tick_plan = compile_runtime_rf_tick_plan(scenario)?;
    let reconciliation_plan = compile_recursive_rf_reconciliation_plan(scenario)?;
    let comparison_report = evaluate_runtime_rf_tick_source_comparison(scenario)
        .map_err(|_| SpecError::ValidationFailed)?;

    Ok(RuntimeRfTickSourceComparisonPlan {
        default_source_kind: comparison_report.default_source_kind,
        selected_source_kind: comparison_report.selected_source_kind,
        recursive_source_preview_only: comparison_report.recursive_source_preview_only,
        tick_shell_source_replacement_deferred: comparison_report
            .tick_shell_source_replacement_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        recursive_local_rf_plan: reconciliation_plan.recursive_local_rf_plan.clone(),
        reconciliation_plan,
        legacy_tick_plan,
        comparison_report,
    })
}

/// Compile tick shell with optional side-by-side RF source comparison wrapper (preview only).
pub fn compile_runtime_tick_shell_with_rf_source_comparison_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
) -> Result<RuntimeTickShellRfSourceComparisonPlan, SpecError> {
    let tick_shell_plan = compile_runtime_tick_shell_plan(scenario, tick_id)?;
    let rf_source_comparison_plan = compile_runtime_rf_tick_source_comparison_plan(scenario)?;

    Ok(RuntimeTickShellRfSourceComparisonPlan {
        tick_id,
        tick_shell_plan,
        rf_source_comparison_plan,
    })
}
