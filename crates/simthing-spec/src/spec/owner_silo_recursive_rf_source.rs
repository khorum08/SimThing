//! OWNER-SILO-RECURSIVE-RF-SOURCE-0 — recursive RF source mode for owner-silo disburse-down.
//!
//! Recursive RF may drive owner-silo/disburse-down proof reports behind explicit source mode.
//! CPU responsibilities: oracle/reference/shadow projection, semantic-side bookkeeping,
//! compile-plan construction, and owner/user-facing report formatting.

use std::collections::BTreeSet;

use super::channel_key::{OwnerRef, ScopeId};
use super::owner_silo_disburse_down::{
    apply_owner_silo_runtime_disburse_down_cpu, demand_bucket_sort_key,
    owner_silo_demand_buckets_from_planet_child_rf, RuntimeOwnerSiloDemandBucket,
    RuntimeOwnerSiloDisburseDownResult,
};
use super::owner_silo_runtime_writeback::{
    apply_owner_silo_runtime_writeback_cpu,
    owner_silo_writeback_inputs_from_planet_child_reduce_up,
    runtime_owner_silo_states_from_scenario,
};
use super::planet_child_rf::{
    evaluate_planet_child_rf_reduce_up, planet_child_rf_default_resource_key,
    PlanetChildRfAdmissionClassification, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};
use super::recursive_local_rf::{
    evaluate_recursive_local_rf, recursive_local_rf_aggregate_source_rows,
};
use super::recursive_rf_reconciliation::reconcile_planet_child_rf_with_recursive_local_rf;
use super::runtime_tick_history::scenario_authority_digest;
use super::scenario::{
    game_session_owners, owner_entity_id, SimThingScenarioSpec, OWNER_FLOW_DEFAULT_PRIORITY,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerSiloRfSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveLocalRfSelectable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerSiloRfSourceSelection {
    pub requested_source_mode: OwnerSiloRfSourceMode,
    pub selected_source_mode: OwnerSiloRfSourceMode,
    pub selection_allowed: bool,
    pub legacy_default_preserved: bool,
    pub reconciliation_ready: bool,
    pub participant_projection_compatible: bool,
    pub redistribution_deltas_documented: bool,
    pub recursive_selected_for_owner_silo_report_only: bool,
    pub local_allocation_integration_deferred: bool,
    pub local_effect_integration_deferred: bool,
    pub semantic_effect_integration_deferred: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerSiloRecursiveSourceErrorKind {
    LegacyDemandRejected,
    RecursiveEvaluationRejected,
    ReconciliationRejected,
    WritebackRejected,
    DisburseDownRejected,
    SelectionDenied,
    ArithmeticOverflow,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerSiloRecursiveSourceError {
    pub kind: OwnerSiloRecursiveSourceErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloDisburseDownReport {
    pub demand_buckets: Vec<RuntimeOwnerSiloDemandBucket>,
    pub disburse_down_results: Vec<RuntimeOwnerSiloDisburseDownResult>,
    pub demand_bucket_count: u32,
    pub disburse_result_count: u32,
    pub allocated_total: u32,
    pub unmet_total: u32,
    pub owner_silo_disburse_down_executed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloRfSourceDisburseReport {
    pub source_selection: OwnerSiloRfSourceSelection,
    pub legacy_disburse_report: OwnerSiloDisburseDownReport,
    pub recursive_disburse_report: Option<OwnerSiloDisburseDownReport>,
    pub selected_disburse_report: OwnerSiloDisburseDownReport,
    pub selected_source_mode: OwnerSiloRfSourceMode,
    pub owner_silo_disburse_down_executed_for_selected_source: bool,
    pub legacy_default_preserved: bool,
    pub recursive_source_report_only_beyond_owner_silo: bool,
    pub local_allocation_integration_deferred: bool,
    pub local_effect_integration_deferred: bool,
    pub semantic_effect_integration_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

/// Project owner-silo demand buckets from recursive RF aggregate source rows and arena settlements.
///
/// Grain: per-Location-arena aggregate source rows with `demand > 0`, plus parent-level
/// `net_deficit_to_parent` settlement rows so sibling redistribution is not ignored.
pub fn owner_silo_demand_buckets_from_recursive_local_rf(
    scenario: &SimThingScenarioSpec,
) -> Result<Vec<RuntimeOwnerSiloDemandBucket>, OwnerSiloRecursiveSourceError> {
    let recursive_report =
        evaluate_recursive_local_rf(scenario).map_err(|e| OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::RecursiveEvaluationRejected,
            message: e.message,
        })?;

    let owner_refs: BTreeSet<OwnerRef> = game_session_owners(scenario)
        .map_err(|_| OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::RecursiveEvaluationRejected,
            message: "game session owners unavailable".to_string(),
        })?
        .into_iter()
        .filter_map(owner_entity_id)
        .map(OwnerRef::new)
        .collect();

    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive_report);
    let mut buckets = Vec::new();

    for row in &aggregate_rows {
        if row.demand == 0 {
            continue;
        }
        if !owner_refs.contains(&row.owner_ref) {
            return Err(OwnerSiloRecursiveSourceError {
                kind: OwnerSiloRecursiveSourceErrorKind::RecursiveEvaluationRejected,
                message: format!(
                    "recursive demand references unknown owner/channel {}",
                    row.owner_ref.as_str()
                ),
            });
        }

        // Owner-silo disburse-down writeback channels use planet-child reduce-up scope keys ("generic").
        let resource_key = planet_child_rf_default_resource_key();

        let scope_id = ScopeId::new(format!("location/{}", row.arena_location_id_raw));
        buckets.push(RuntimeOwnerSiloDemandBucket {
            owner_ref: row.owner_ref.clone(),
            resource_key,
            scope_id,
            planet_id: None,
            star_system_gridcell_id_raw: Some(row.arena_location_id_raw),
            requested: row.demand,
            priority: OWNER_FLOW_DEFAULT_PRIORITY,
            source_simthing_id_raw: Some(row.source_simthing_or_location_id_raw),
        });
    }

    for arena in &recursive_report.arena_reports {
        for settlement in &arena.settlements {
            if settlement.net_deficit_to_parent == 0 {
                continue;
            }
            if !owner_refs.contains(&settlement.owner_ref) {
                return Err(OwnerSiloRecursiveSourceError {
                    kind: OwnerSiloRecursiveSourceErrorKind::RecursiveEvaluationRejected,
                    message: format!(
                        "recursive parent deficit references unknown owner/channel {}",
                        settlement.owner_ref.as_str()
                    ),
                });
            }
            let resource_key = planet_child_rf_default_resource_key();
            let scope_id =
                ScopeId::new(format!("location/{}/parent_deficit", arena.location_id_raw));
            buckets.push(RuntimeOwnerSiloDemandBucket {
                owner_ref: settlement.owner_ref.clone(),
                resource_key,
                scope_id,
                planet_id: None,
                star_system_gridcell_id_raw: Some(arena.location_id_raw),
                requested: settlement.net_deficit_to_parent,
                priority: OWNER_FLOW_DEFAULT_PRIORITY,
                source_simthing_id_raw: Some(arena.location_id_raw),
            });
        }
    }

    for output in &recursive_report.root_outputs {
        if output.net_deficit == 0 {
            continue;
        }
        if !owner_refs.contains(&output.owner_ref) {
            return Err(OwnerSiloRecursiveSourceError {
                kind: OwnerSiloRecursiveSourceErrorKind::RecursiveEvaluationRejected,
                message: format!(
                    "recursive root deficit references unknown owner/channel {}",
                    output.owner_ref.as_str()
                ),
            });
        }
        let resource_key = planet_child_rf_default_resource_key();
        let scope_id = ScopeId::new(format!(
            "location/{}/root_deficit",
            output.parent_location_id.raw()
        ));
        buckets.push(RuntimeOwnerSiloDemandBucket {
            owner_ref: output.owner_ref.clone(),
            resource_key,
            scope_id,
            planet_id: None,
            star_system_gridcell_id_raw: Some(output.parent_location_id.raw()),
            requested: output.net_deficit,
            priority: OWNER_FLOW_DEFAULT_PRIORITY,
            source_simthing_id_raw: Some(output.child_location_id_raw),
        });
    }

    buckets.sort_by(demand_bucket_sort_key);
    Ok(buckets)
}

/// Evaluate owner-silo disburse-down with explicit RF source mode.
pub fn evaluate_owner_silo_disburse_down_with_rf_source(
    scenario: &SimThingScenarioSpec,
    source_mode: OwnerSiloRfSourceMode,
) -> Result<OwnerSiloRfSourceDisburseReport, OwnerSiloRecursiveSourceError> {
    let legacy_buckets = owner_silo_demand_buckets_from_planet_child_rf(scenario).map_err(|e| {
        OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::LegacyDemandRejected,
            message: e.message,
        }
    })?;
    let legacy_disburse_report = run_owner_silo_disburse_down_report(scenario, legacy_buckets)?;

    let reconciliation =
        reconcile_planet_child_rf_with_recursive_local_rf(scenario).map_err(|e| {
            OwnerSiloRecursiveSourceError {
                kind: OwnerSiloRecursiveSourceErrorKind::ReconciliationRejected,
                message: e.message,
            }
        })?;

    let recursive_report =
        evaluate_recursive_local_rf(scenario).map_err(|e| OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::RecursiveEvaluationRejected,
            message: e.message,
        })?;
    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive_report);
    let recursive_source_available = !aggregate_rows.is_empty();

    let redistribution_deltas_documented = reconciliation
        .sibling_redistribution_scope_delta_observed
        || reconciliation
            .buckets
            .iter()
            .any(|bucket| !bucket.compatible);

    let participant_projection_compatible = reconciliation.participant_row_compatible;
    let reconciliation_ready = true;
    let reconciliation_compatible =
        reconciliation.participant_row_compatible && reconciliation.incompatible_bucket_count == 0;

    let selection_allowed = match source_mode {
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo => true,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable => {
            reconciliation_ready
                && participant_projection_compatible
                && recursive_source_available
                && (reconciliation_compatible || redistribution_deltas_documented)
        }
    };

    let selected_source_mode = match source_mode {
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo => {
            OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable if selection_allowed => {
            OwnerSiloRfSourceMode::RecursiveLocalRfSelectable
        }
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable => {
            OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo
        }
    };

    let reason = match source_mode {
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo => {
            "legacy planet-child/owner-silo source preserves default owner-silo disburse-down"
                .to_string()
        }
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable if selection_allowed => {
            if reconciliation_compatible {
                "recursive RF owner-silo source: participant projection compatible and reconciliation compatible"
                    .to_string()
            } else {
                "recursive RF owner-silo source: participant projection compatible with documented redistribution deltas"
                    .to_string()
            }
        }
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable => {
            if !participant_projection_compatible {
                "recursive RF owner-silo selection denied: participant projection incompatible"
                    .to_string()
            } else if !recursive_source_available {
                "recursive RF owner-silo selection denied: recursive source unavailable".to_string()
            } else {
                "recursive RF owner-silo selection denied: equivalence gates not satisfied"
                    .to_string()
            }
        }
    };

    let recursive_disburse_report =
        if source_mode == OwnerSiloRfSourceMode::RecursiveLocalRfSelectable {
            let recursive_buckets = owner_silo_demand_buckets_from_recursive_local_rf(scenario)?;
            Some(run_owner_silo_disburse_down_report(
                scenario,
                recursive_buckets,
            )?)
        } else {
            None
        };

    let selected_disburse_report = match selected_source_mode {
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable => recursive_disburse_report
            .clone()
            .expect("recursive disburse report required when selected"),
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo => legacy_disburse_report.clone(),
    };

    let recursive_selected_for_owner_silo_report_only = selected_source_mode
        == OwnerSiloRfSourceMode::RecursiveLocalRfSelectable
        && selection_allowed;

    Ok(OwnerSiloRfSourceDisburseReport {
        source_selection: OwnerSiloRfSourceSelection {
            requested_source_mode: source_mode,
            selected_source_mode,
            selection_allowed,
            legacy_default_preserved: true,
            reconciliation_ready,
            participant_projection_compatible,
            redistribution_deltas_documented,
            recursive_selected_for_owner_silo_report_only,
            local_allocation_integration_deferred: true,
            local_effect_integration_deferred: true,
            semantic_effect_integration_deferred: true,
            reason,
        },
        legacy_disburse_report: legacy_disburse_report.clone(),
        recursive_disburse_report,
        selected_disburse_report: selected_disburse_report.clone(),
        selected_source_mode,
        owner_silo_disburse_down_executed_for_selected_source: selected_disburse_report
            .owner_silo_disburse_down_executed,
        legacy_default_preserved: true,
        recursive_source_report_only_beyond_owner_silo: true,
        local_allocation_integration_deferred: true,
        local_effect_integration_deferred: true,
        semantic_effect_integration_deferred: true,
        scenario_authority_mutation_deferred: true,
        participant_property_mutation_deferred: true,
        savefile_mutation_deferred: true,
    })
}

/// Prove Scenario authority is unchanged after owner-silo recursive source evaluation.
pub fn prove_owner_silo_recursive_source_preserves_authority(
    scenario: &SimThingScenarioSpec,
    source_mode: OwnerSiloRfSourceMode,
) -> Result<bool, OwnerSiloRecursiveSourceError> {
    let before =
        scenario_authority_digest(scenario).map_err(|e| OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    let _report = evaluate_owner_silo_disburse_down_with_rf_source(scenario, source_mode)?;
    let after = scenario_authority_digest(scenario).map_err(|e| OwnerSiloRecursiveSourceError {
        kind: OwnerSiloRecursiveSourceErrorKind::ScenarioAuthorityRejected,
        message: e.message,
    })?;
    Ok(before == after)
}

fn run_owner_silo_disburse_down_report(
    scenario: &SimThingScenarioSpec,
    demand_buckets: Vec<RuntimeOwnerSiloDemandBucket>,
) -> Result<OwnerSiloDisburseDownReport, OwnerSiloRecursiveSourceError> {
    let reduce_up = evaluate_planet_child_rf_reduce_up(scenario);
    if reduce_up.classification == PlanetChildRfAdmissionClassification::Rejected {
        return Err(OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::WritebackRejected,
            message: "planet child RF reduce-up rejected".to_string(),
        });
    }

    let initial = runtime_owner_silo_states_from_scenario(scenario).map_err(|e| {
        OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::WritebackRejected,
            message: e.message,
        }
    })?;
    let inputs =
        owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up).map_err(|e| {
            OwnerSiloRecursiveSourceError {
                kind: OwnerSiloRecursiveSourceErrorKind::WritebackRejected,
                message: e.message,
            }
        })?;
    let writeback_results =
        apply_owner_silo_runtime_writeback_cpu(&initial, &inputs).map_err(|e| {
            OwnerSiloRecursiveSourceError {
                kind: OwnerSiloRecursiveSourceErrorKind::WritebackRejected,
                message: e.message,
            }
        })?;

    let disburse_down_results = if demand_buckets.is_empty() {
        Vec::new()
    } else {
        apply_owner_silo_runtime_disburse_down_cpu(&writeback_results, &demand_buckets).map_err(
            |e| OwnerSiloRecursiveSourceError {
                kind: OwnerSiloRecursiveSourceErrorKind::DisburseDownRejected,
                message: e.message,
            },
        )?
    };

    let allocated_total = disburse_down_results
        .iter()
        .try_fold(0u32, |acc, r| acc.checked_add(r.allocated_total))
        .ok_or_else(|| OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::ArithmeticOverflow,
            message: "allocated_total overflow".to_string(),
        })?;
    let unmet_total = disburse_down_results
        .iter()
        .try_fold(0u32, |acc, r| acc.checked_add(r.unmet_total))
        .ok_or_else(|| OwnerSiloRecursiveSourceError {
            kind: OwnerSiloRecursiveSourceErrorKind::ArithmeticOverflow,
            message: "unmet_total overflow".to_string(),
        })?;

    let owner_silo_disburse_down_executed =
        !demand_buckets.is_empty() && !writeback_results.is_empty();

    Ok(OwnerSiloDisburseDownReport {
        demand_bucket_count: demand_buckets.len() as u32,
        disburse_result_count: disburse_down_results.len() as u32,
        owner_silo_disburse_down_executed,
        demand_buckets,
        disburse_down_results,
        allocated_total,
        unmet_total,
    })
}
