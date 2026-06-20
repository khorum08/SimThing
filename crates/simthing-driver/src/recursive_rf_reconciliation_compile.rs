//! PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 — CPU oracle/shadow reconciliation compile plan
//! between legacy planet-child RF ladder and recursive Location RF evaluator.

use simthing_spec::{
    reconcile_planet_child_rf_with_recursive_local_rf, RecursiveRfReconciliationReport,
    SimThingScenarioSpec, SpecError,
};

use crate::recursive_local_rf_compile::compile_recursive_local_rf_plan;
use crate::recursive_local_rf_compile::RecursiveLocalRfPlan;

/// Driver compile plan composing recursive local RF plan and reconciliation report.
#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveRfReconciliationPlan {
    pub recursive_local_rf_plan: RecursiveLocalRfPlan,
    pub reconciliation_report: RecursiveRfReconciliationReport,
    pub legacy_projection_count: u32,
    pub recursive_projection_count: u32,
    pub gpu_residency_doctrine_preserved: bool,
    pub tick_shell_source_replacement_deferred: bool,
    pub previous_ladder_preserved: bool,
}

/// Compile recursive RF reconciliation plan without altering runtime tick or semantic paths.
pub fn compile_recursive_rf_reconciliation_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<RecursiveRfReconciliationPlan, SpecError> {
    let recursive_local_rf_plan = compile_recursive_local_rf_plan(scenario)?;
    let reconciliation_report = reconcile_planet_child_rf_with_recursive_local_rf(scenario)
        .map_err(|_| SpecError::ValidationFailed)?;

    if !reconciliation_report.recursive_evaluator_preserved {
        return Err(SpecError::ValidationFailed);
    }

    Ok(RecursiveRfReconciliationPlan {
        legacy_projection_count: reconciliation_report.legacy_projection_count,
        recursive_projection_count: reconciliation_report.recursive_projection_count,
        previous_ladder_preserved: reconciliation_report.previous_ladder_preserved,
        gpu_residency_doctrine_preserved: true,
        tick_shell_source_replacement_deferred: true,
        recursive_local_rf_plan,
        reconciliation_report,
    })
}
