//! RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 — optional side-by-side RF source comparison for tick shell.
//!
//! Legacy planet-child/owner-silo RF remains the default tick source. Recursive Location RF is
//! preview-only. CPU responsibilities: oracle/reference/shadow comparison, semantic-side bookkeeping,
//! compile-plan construction, and owner/user-facing report formatting.

use std::collections::BTreeSet;

use super::recursive_local_rf::{
    evaluate_recursive_local_rf, recursive_local_rf_aggregate_source_rows,
};
use super::recursive_rf_reconciliation::reconcile_planet_child_rf_with_recursive_local_rf;
use super::runtime_rf_tick::evaluate_runtime_rf_tick;
use super::runtime_tick_history::scenario_authority_digest;
use super::scenario::SimThingScenarioSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeRfTickSourceKind {
    LegacyPlanetChildOwnerSilo,
    RecursiveLocalRf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeRfTickSourceMode {
    LegacyDefault,
    RecursivePreview,
    SideBySideComparison,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRfTickSourceSummary {
    pub source_kind: RuntimeRfTickSourceKind,
    pub participant_or_source_count: u32,
    pub location_count: u32,
    pub owner_channel_count: u32,
    pub surplus_total: u32,
    pub demand_total: u32,
    pub net_surplus_total: u32,
    pub net_deficit_total: u32,
    pub gpu_compatible_row_count: u32,
    pub gpu_residency_doctrine_preserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimeRfTickSourceDeltaKind {
    CompatibleParticipantProjection,
    RecursiveRedistributionDelta,
    ScopeProjectionDelta,
    MissingLegacyProjection,
    MissingRecursiveProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRfTickSourceDelta {
    pub owner_ref: String,
    pub resource_key: String,
    pub legacy_surplus_total: u32,
    pub legacy_demand_total: u32,
    pub recursive_surplus_total: u32,
    pub recursive_demand_total: u32,
    pub surplus_delta: i64,
    pub demand_delta: i64,
    pub delta_kind: RuntimeRfTickSourceDeltaKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeRfTickSourceErrorKind {
    LegacyTickRejected,
    RecursiveTickRejected,
    ReconciliationRejected,
    ArithmeticOverflow,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRfTickSourceError {
    pub kind: RuntimeRfTickSourceErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeRfTickSourceComparisonReport {
    pub source_mode: RuntimeRfTickSourceMode,
    pub legacy_summary: RuntimeRfTickSourceSummary,
    pub recursive_summary: RuntimeRfTickSourceSummary,
    pub reconciliation_ready: bool,
    pub reconciliation_compatible: bool,
    pub deltas: Vec<RuntimeRfTickSourceDelta>,
    pub default_source_kind: RuntimeRfTickSourceKind,
    pub selected_source_kind: RuntimeRfTickSourceKind,
    pub recursive_source_available: bool,
    pub recursive_source_preview_only: bool,
    pub legacy_tick_source_preserved: bool,
    pub tick_shell_source_replacement_deferred: bool,
    pub semantic_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

/// Evaluate side-by-side legacy vs recursive RF tick source comparison.
pub fn evaluate_runtime_rf_tick_source_comparison(
    scenario: &SimThingScenarioSpec,
) -> Result<RuntimeRfTickSourceComparisonReport, RuntimeRfTickSourceError> {
    evaluate_runtime_rf_tick_source_preview(scenario, RuntimeRfTickSourceMode::SideBySideComparison)
}

/// Evaluate RF tick source comparison for the requested preview mode.
pub fn evaluate_runtime_rf_tick_source_preview(
    scenario: &SimThingScenarioSpec,
    source_mode: RuntimeRfTickSourceMode,
) -> Result<RuntimeRfTickSourceComparisonReport, RuntimeRfTickSourceError> {
    let legacy_tick = evaluate_runtime_rf_tick(scenario).map_err(|e| RuntimeRfTickSourceError {
        kind: RuntimeRfTickSourceErrorKind::LegacyTickRejected,
        message: e.message,
    })?;

    let recursive_report =
        evaluate_recursive_local_rf(scenario).map_err(|e| RuntimeRfTickSourceError {
            kind: RuntimeRfTickSourceErrorKind::RecursiveTickRejected,
            message: e.message,
        })?;

    let reconciliation =
        reconcile_planet_child_rf_with_recursive_local_rf(scenario).map_err(|e| {
            RuntimeRfTickSourceError {
                kind: RuntimeRfTickSourceErrorKind::ReconciliationRejected,
                message: e.message,
            }
        })?;

    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive_report);
    let owner_channels: BTreeSet<_> = aggregate_rows
        .iter()
        .map(|row| row.owner_ref.clone())
        .collect();

    let (recursive_surplus_total, recursive_demand_total) =
        recursive_settlement_totals(&recursive_report)?;
    let (recursive_net_surplus, recursive_net_deficit) =
        recursive_net_bubble_totals(&recursive_report)?;

    let legacy_summary = RuntimeRfTickSourceSummary {
        source_kind: RuntimeRfTickSourceKind::LegacyPlanetChildOwnerSilo,
        participant_or_source_count: legacy_tick.participant_count,
        location_count: legacy_tick.participant_report.planet_gridcell_count,
        owner_channel_count: legacy_tick.participant_report.owner_channel_count,
        surplus_total: legacy_tick.surplus_total,
        demand_total: legacy_tick.deficit_total,
        net_surplus_total: legacy_tick
            .surplus_total
            .saturating_sub(legacy_tick.deficit_total),
        net_deficit_total: legacy_tick
            .deficit_total
            .saturating_sub(legacy_tick.surplus_total),
        gpu_compatible_row_count: legacy_tick.participant_count,
        gpu_residency_doctrine_preserved: true,
    };

    let recursive_summary = RuntimeRfTickSourceSummary {
        source_kind: RuntimeRfTickSourceKind::RecursiveLocalRf,
        participant_or_source_count: recursive_report.participant_count,
        location_count: recursive_report.location_count,
        owner_channel_count: owner_channels.len() as u32,
        surplus_total: recursive_surplus_total,
        demand_total: recursive_demand_total,
        net_surplus_total: recursive_net_surplus,
        net_deficit_total: recursive_net_deficit,
        gpu_compatible_row_count: aggregate_rows.len() as u32,
        gpu_residency_doctrine_preserved: true,
    };

    let deltas = deltas_from_reconciliation(&reconciliation);
    let reconciliation_compatible =
        reconciliation.participant_row_compatible && reconciliation.incompatible_bucket_count == 0;

    let selected_source_kind = match source_mode {
        RuntimeRfTickSourceMode::LegacyDefault => {
            RuntimeRfTickSourceKind::LegacyPlanetChildOwnerSilo
        }
        RuntimeRfTickSourceMode::RecursivePreview => RuntimeRfTickSourceKind::RecursiveLocalRf,
        RuntimeRfTickSourceMode::SideBySideComparison => {
            RuntimeRfTickSourceKind::LegacyPlanetChildOwnerSilo
        }
    };

    Ok(RuntimeRfTickSourceComparisonReport {
        source_mode,
        legacy_summary,
        recursive_summary,
        reconciliation_ready: true,
        reconciliation_compatible,
        deltas,
        default_source_kind: RuntimeRfTickSourceKind::LegacyPlanetChildOwnerSilo,
        selected_source_kind,
        recursive_source_available: !aggregate_rows.is_empty(),
        recursive_source_preview_only: true,
        legacy_tick_source_preserved: true,
        tick_shell_source_replacement_deferred: true,
        semantic_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
    })
}

/// Prove Scenario authority is unchanged after RF source comparison evaluation.
pub fn prove_runtime_rf_tick_source_preserves_authority(
    scenario: &SimThingScenarioSpec,
) -> Result<bool, RuntimeRfTickSourceError> {
    let before = scenario_authority_digest(scenario).map_err(|e| RuntimeRfTickSourceError {
        kind: RuntimeRfTickSourceErrorKind::ScenarioAuthorityRejected,
        message: e.message,
    })?;
    let _report = evaluate_runtime_rf_tick_source_comparison(scenario)?;
    let after = scenario_authority_digest(scenario).map_err(|e| RuntimeRfTickSourceError {
        kind: RuntimeRfTickSourceErrorKind::ScenarioAuthorityRejected,
        message: e.message,
    })?;
    Ok(before == after)
}

fn recursive_settlement_totals(
    report: &super::recursive_local_rf::RecursiveLocalRfEvaluationReport,
) -> Result<(u32, u32), RuntimeRfTickSourceError> {
    let mut surplus = 0u32;
    let mut demand = 0u32;
    for arena in &report.arena_reports {
        for settlement in &arena.settlements {
            surplus = surplus
                .checked_add(settlement.total_surplus)
                .ok_or_else(|| overflow_error("recursive_surplus_total"))?;
            demand = demand
                .checked_add(settlement.total_demand)
                .ok_or_else(|| overflow_error("recursive_demand_total"))?;
        }
    }
    Ok((surplus, demand))
}

fn recursive_net_bubble_totals(
    report: &super::recursive_local_rf::RecursiveLocalRfEvaluationReport,
) -> Result<(u32, u32), RuntimeRfTickSourceError> {
    let mut net_surplus = 0u32;
    let mut net_deficit = 0u32;
    for output in &report.root_outputs {
        net_surplus = net_surplus
            .checked_add(output.net_surplus)
            .ok_or_else(|| overflow_error("recursive_net_surplus"))?;
        net_deficit = net_deficit
            .checked_add(output.net_deficit)
            .ok_or_else(|| overflow_error("recursive_net_deficit"))?;
    }
    Ok((net_surplus, net_deficit))
}

fn deltas_from_reconciliation(
    reconciliation: &super::recursive_rf_reconciliation::RecursiveRfReconciliationReport,
) -> Vec<RuntimeRfTickSourceDelta> {
    let mut deltas = Vec::with_capacity(reconciliation.buckets.len());
    for bucket in &reconciliation.buckets {
        let delta_kind = if bucket.compatible {
            RuntimeRfTickSourceDeltaKind::CompatibleParticipantProjection
        } else if bucket.planet_gridcell_id_raw.is_none()
            && bucket.star_system_gridcell_id_raw.is_some()
        {
            RuntimeRfTickSourceDeltaKind::ScopeProjectionDelta
        } else {
            RuntimeRfTickSourceDeltaKind::RecursiveRedistributionDelta
        };
        deltas.push(RuntimeRfTickSourceDelta {
            owner_ref: bucket.owner_ref.clone(),
            resource_key: bucket.resource_key.clone(),
            legacy_surplus_total: bucket.legacy_surplus_total,
            legacy_demand_total: bucket.legacy_demand_total,
            recursive_surplus_total: bucket.recursive_surplus_total,
            recursive_demand_total: bucket.recursive_demand_total,
            surplus_delta: bucket.surplus_delta,
            demand_delta: bucket.demand_delta,
            delta_kind,
        });
    }

    for mismatch in &reconciliation.mismatches {
        use super::recursive_rf_reconciliation::RecursiveRfReconciliationMismatchKind;
        let delta_kind = match mismatch.mismatch_kind {
            RecursiveRfReconciliationMismatchKind::ScopeProjectionMismatch => {
                RuntimeRfTickSourceDeltaKind::ScopeProjectionDelta
            }
            RecursiveRfReconciliationMismatchKind::MissingLegacyRowInRecursiveProjection => {
                RuntimeRfTickSourceDeltaKind::MissingRecursiveProjection
            }
            RecursiveRfReconciliationMismatchKind::UnexpectedRecursiveProjection => {
                RuntimeRfTickSourceDeltaKind::MissingLegacyProjection
            }
            _ => RuntimeRfTickSourceDeltaKind::RecursiveRedistributionDelta,
        };
        deltas.push(RuntimeRfTickSourceDelta {
            owner_ref: mismatch.owner_ref.clone(),
            resource_key: mismatch.resource_key.clone(),
            legacy_surplus_total: mismatch.legacy_value.max(0) as u32,
            legacy_demand_total: 0,
            recursive_surplus_total: mismatch.recursive_value.max(0) as u32,
            recursive_demand_total: 0,
            surplus_delta: mismatch.recursive_value - mismatch.legacy_value,
            demand_delta: 0,
            delta_kind,
        });
    }

    deltas.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.delta_kind,
            a.surplus_delta,
            a.demand_delta,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.delta_kind,
                b.surplus_delta,
                b.demand_delta,
            ))
    });
    deltas
}

fn overflow_error(field: &str) -> RuntimeRfTickSourceError {
    RuntimeRfTickSourceError {
        kind: RuntimeRfTickSourceErrorKind::ArithmeticOverflow,
        message: format!("{field} overflow"),
    }
}
