//! PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 — CPU oracle/shadow reconciliation compile plan
//! between legacy planet-child RF ladder and recursive Location RF evaluator.

use simthing_spec::{
    admit_intrinsic_owner_channels,
    reconcile_planet_child_rf_with_recursive_local_rf_from_owner_view,
    RecursiveRfReconciliationReport, SimThingScenarioSpec, SpecError,
};

use crate::recursive_local_rf_compile::compile_recursive_local_rf_plan_from_owner_view;
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
    let owner_view =
        admit_intrinsic_owner_channels(scenario).map_err(|_| SpecError::ValidationFailedAt {
            site: "simthing-driver/recursive_rf_reconciliation_compile",
        })?;
    let recursive_local_rf_plan = compile_recursive_local_rf_plan_from_owner_view(&owner_view)?;
    let reconciliation_report = reconcile_planet_child_rf_with_recursive_local_rf_from_owner_view(
        &owner_view,
    )
    .map_err(|_| SpecError::ValidationFailedAt {
        site: "simthing-driver/recursive_rf_reconciliation_compile",
    })?;

    if !reconciliation_report.recursive_evaluator_preserved {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/recursive_rf_reconciliation_compile",
        });
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
