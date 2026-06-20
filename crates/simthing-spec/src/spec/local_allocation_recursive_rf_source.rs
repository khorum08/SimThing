//! LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 — recursive RF source mode for runtime local allocation.
//!
//! Recursive-source owner-silo/disburse-down feeds runtime local allocation proof reports
//! behind explicit source mode. CPU responsibilities: oracle/reference/shadow projection,
//! semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.

use super::owner_silo_recursive_rf_source::{
    evaluate_owner_silo_disburse_down_with_rf_source, OwnerSiloDisburseDownReport,
    OwnerSiloRfSourceDisburseReport, OwnerSiloRfSourceMode,
};
use super::runtime_local_allocation::{
    apply_runtime_local_allocations_from_disburse_down, RuntimeLocalAllocationApplicationError,
    RuntimeLocalAllocationApplicationReport,
};
use super::runtime_tick_history::scenario_authority_digest;
use super::scenario::SimThingScenarioSpec;

/// Runtime local allocation report (alias for application report shape).
pub type RuntimeLocalAllocationReport = RuntimeLocalAllocationApplicationReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalAllocationRfSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveOwnerSiloSelectable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAllocationRfSourceSelection {
    pub requested_source_mode: LocalAllocationRfSourceMode,
    pub selected_source_mode: LocalAllocationRfSourceMode,
    pub selection_allowed: bool,
    pub legacy_default_preserved: bool,
    pub owner_silo_recursive_source_ready: bool,
    pub owner_silo_disburse_report_available: bool,
    pub recursive_selected_for_local_allocation_report_only: bool,
    pub local_effect_integration_deferred: bool,
    pub semantic_effect_integration_deferred: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalAllocationRecursiveSourceErrorKind {
    OwnerSiloSourceRejected,
    AllocationRejected,
    SelectionDenied,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAllocationRecursiveSourceError {
    pub kind: LocalAllocationRecursiveSourceErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeLocalAllocationRfSourceReport {
    pub source_selection: LocalAllocationRfSourceSelection,
    pub legacy_allocation_report: RuntimeLocalAllocationReport,
    pub recursive_owner_silo_disburse_report: Option<OwnerSiloRfSourceDisburseReport>,
    pub recursive_allocation_report: Option<RuntimeLocalAllocationReport>,
    pub selected_allocation_report: RuntimeLocalAllocationReport,
    pub selected_source_mode: LocalAllocationRfSourceMode,
    pub local_allocation_executed_for_selected_source: bool,
    pub legacy_default_preserved: bool,
    pub recursive_source_report_only_beyond_local_allocation: bool,
    pub local_effect_integration_deferred: bool,
    pub semantic_effect_integration_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

/// Map owner-silo disburse-down report rows into runtime local allocation report.
pub fn runtime_local_allocation_from_owner_silo_disburse_report(
    disburse_report: &OwnerSiloDisburseDownReport,
) -> Result<RuntimeLocalAllocationReport, LocalAllocationRecursiveSourceError> {
    apply_runtime_local_allocations_from_disburse_down(&disburse_report.disburse_down_results)
        .map_err(map_allocation_error)
}

/// Evaluate runtime local allocation with explicit RF source mode.
pub fn evaluate_runtime_local_allocation_with_rf_source(
    scenario: &SimThingScenarioSpec,
    source_mode: LocalAllocationRfSourceMode,
) -> Result<RuntimeLocalAllocationRfSourceReport, LocalAllocationRecursiveSourceError> {
    let owner_silo_mode = match source_mode {
        LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo => {
            OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable => {
            OwnerSiloRfSourceMode::RecursiveLocalRfSelectable
        }
    };

    let owner_silo_report =
        evaluate_owner_silo_disburse_down_with_rf_source(scenario, owner_silo_mode).map_err(
            |e| LocalAllocationRecursiveSourceError {
                kind: LocalAllocationRecursiveSourceErrorKind::OwnerSiloSourceRejected,
                message: e.message,
            },
        )?;

    let legacy_allocation_report = runtime_local_allocation_from_owner_silo_disburse_report(
        &owner_silo_report.legacy_disburse_report,
    )?;

    let recursive_owner_silo_disburse_report =
        if source_mode == LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable {
            Some(owner_silo_report.clone())
        } else {
            None
        };

    let owner_silo_recursive_source_ready = owner_silo_report.source_selection.reconciliation_ready;
    let owner_silo_disburse_report_available = owner_silo_report
        .recursive_disburse_report
        .as_ref()
        .map(|report| report.owner_silo_disburse_down_executed)
        .unwrap_or(false);

    let selection_allowed = match source_mode {
        LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo => true,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable => {
            owner_silo_recursive_source_ready
                && owner_silo_disburse_report_available
                && owner_silo_report.source_selection.selection_allowed
        }
    };

    let selected_source_mode = match source_mode {
        LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo => {
            LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable if selection_allowed => {
            LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable
        }
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable => {
            LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo
        }
    };

    let reason = match source_mode {
        LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo => {
            "legacy planet-child/owner-silo/local-allocation path preserves default behavior"
                .to_string()
        }
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable if selection_allowed => {
            "recursive owner-silo disburse report feeds runtime local allocation report".to_string()
        }
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable => {
            if !owner_silo_recursive_source_ready {
                "recursive local allocation denied: owner-silo recursive source not ready"
                    .to_string()
            } else if !owner_silo_disburse_report_available {
                "recursive local allocation denied: recursive owner-silo disburse report unavailable"
                    .to_string()
            } else {
                "recursive local allocation denied: owner-silo selection gates not satisfied"
                    .to_string()
            }
        }
    };

    let recursive_allocation_report =
        if source_mode == LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable {
            owner_silo_report
                .recursive_disburse_report
                .as_ref()
                .map(runtime_local_allocation_from_owner_silo_disburse_report)
                .transpose()?
        } else {
            None
        };

    let selected_allocation_report = match selected_source_mode {
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable => recursive_allocation_report
            .clone()
            .expect("recursive allocation report required when selected"),
        LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo => legacy_allocation_report.clone(),
    };

    let recursive_selected_for_local_allocation_report_only = selected_source_mode
        == LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable
        && selection_allowed;

    let local_allocation_executed_for_selected_source = selected_allocation_report.allocation_count
        > 0
        || selected_allocation_report.allocated_total > 0;

    Ok(RuntimeLocalAllocationRfSourceReport {
        source_selection: LocalAllocationRfSourceSelection {
            requested_source_mode: source_mode,
            selected_source_mode,
            selection_allowed,
            legacy_default_preserved: true,
            owner_silo_recursive_source_ready,
            owner_silo_disburse_report_available,
            recursive_selected_for_local_allocation_report_only,
            local_effect_integration_deferred: true,
            semantic_effect_integration_deferred: true,
            reason,
        },
        legacy_allocation_report,
        recursive_owner_silo_disburse_report,
        recursive_allocation_report,
        selected_allocation_report: selected_allocation_report.clone(),
        selected_source_mode,
        local_allocation_executed_for_selected_source,
        legacy_default_preserved: true,
        recursive_source_report_only_beyond_local_allocation: true,
        local_effect_integration_deferred: true,
        semantic_effect_integration_deferred: true,
        scenario_authority_mutation_deferred: true,
        participant_property_mutation_deferred: true,
        savefile_mutation_deferred: true,
    })
}

/// Prove Scenario authority is unchanged after recursive local allocation evaluation.
pub fn prove_local_allocation_recursive_source_preserves_authority(
    scenario: &SimThingScenarioSpec,
    source_mode: LocalAllocationRfSourceMode,
) -> Result<bool, LocalAllocationRecursiveSourceError> {
    let before =
        scenario_authority_digest(scenario).map_err(|e| LocalAllocationRecursiveSourceError {
            kind: LocalAllocationRecursiveSourceErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    let _report = evaluate_runtime_local_allocation_with_rf_source(scenario, source_mode)?;
    let after =
        scenario_authority_digest(scenario).map_err(|e| LocalAllocationRecursiveSourceError {
            kind: LocalAllocationRecursiveSourceErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    Ok(before == after)
}

fn map_allocation_error(
    error: RuntimeLocalAllocationApplicationError,
) -> LocalAllocationRecursiveSourceError {
    LocalAllocationRecursiveSourceError {
        kind: LocalAllocationRecursiveSourceErrorKind::AllocationRejected,
        message: error.message,
    }
}
