//! PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 — reconciliation/projection between legacy
//! planet-child RF ladder and recursive Location RF evaluator.
//!
//! CPU responsibilities: deterministic oracle/reference reconciliation, semantic-side bookkeeping,
//! compile-plan construction, and owner/user-facing report formatting — not production simulation authority.
//! Recursive RF remains a GPU-resident row/table target.

use std::collections::BTreeMap;

use super::channel_key::{OwnerRef, ParentLocationId, ResourceKey, ScopeId};
use super::planet_child_rf::{
    planet_child_rf_participant_inputs, scope_key_from_participant,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};
use super::recursive_local_rf::{
    evaluate_recursive_local_rf, recursive_local_rf_aggregate_source_rows,
    RecursiveLocalRfAggregateSourceKind,
};
use super::runtime_tick_history::scenario_authority_digest;
use super::scenario::SimThingScenarioSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildRfProjectionRow {
    pub source_simthing_id_raw: u32,
    pub planet_gridcell_id_raw: u32,
    pub star_system_gridcell_id_raw: u32,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub surplus: u32,
    pub demand: u32,
    pub deficit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveRfProjectionRow {
    pub source_simthing_or_location_id_raw: u32,
    pub source_kind_label: String,
    pub arena_location_id_raw: u32,
    pub parent_location_id: Option<ParentLocationId>,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub surplus: u32,
    pub demand: u32,
    pub net_surplus: u32,
    pub net_deficit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveRfReconciliationBucket {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub planet_gridcell_id_raw: Option<u32>,
    pub star_system_gridcell_id_raw: Option<u32>,
    pub legacy_surplus_total: u32,
    pub legacy_demand_total: u32,
    pub recursive_surplus_total: u32,
    pub recursive_demand_total: u32,
    pub surplus_delta: i64,
    pub demand_delta: i64,
    pub compatible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecursiveRfReconciliationMismatchKind {
    MissingLegacyRowInRecursiveProjection,
    ResourceKeyMismatch,
    OwnerRefMismatch,
    SurplusMismatch,
    DemandMismatch,
    UnexpectedRecursiveProjection,
    ScopeProjectionMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveRfReconciliationMismatch {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub source_simthing_id_raw: Option<u32>,
    pub location_id_raw: Option<u32>,
    pub mismatch_kind: RecursiveRfReconciliationMismatchKind,
    pub legacy_value: i64,
    pub recursive_value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursiveRfReconciliationErrorKind {
    LegacyProjectionRejected,
    RecursiveProjectionRejected,
    ArithmeticOverflow,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveRfReconciliationError {
    pub kind: RecursiveRfReconciliationErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursiveRfReconciliationDeferralKind {
    TickShellSourceReplacementDeferred,
    SemanticExecutionDeferred,
    ParticipantPropertyMutationDeferred,
    ScenarioAuthorityMutationDeferred,
    SavefileMutationDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveRfReconciliationDeferral {
    pub kind: RecursiveRfReconciliationDeferralKind,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveRfReconciliationReport {
    pub legacy_projection_count: u32,
    pub recursive_projection_count: u32,
    pub bucket_count: u32,
    pub compatible_bucket_count: u32,
    pub incompatible_bucket_count: u32,
    pub participant_row_compatible: bool,
    pub sibling_redistribution_scope_delta_observed: bool,
    pub buckets: Vec<RecursiveRfReconciliationBucket>,
    pub mismatches: Vec<RecursiveRfReconciliationMismatch>,
    pub previous_ladder_preserved: bool,
    pub recursive_evaluator_preserved: bool,
    pub tick_shell_source_replacement_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub semantic_execution_deferred: bool,
    pub errors: Vec<RecursiveRfReconciliationError>,
    pub deferrals: Vec<RecursiveRfReconciliationDeferral>,
}

/// Project legacy planet-child RF participant rows for reconciliation comparison.
pub fn project_planet_child_rf_ladder_rows(
    scenario: &SimThingScenarioSpec,
) -> Result<Vec<PlanetChildRfProjectionRow>, RecursiveRfReconciliationError> {
    let participants = planet_child_rf_participant_inputs(scenario).map_err(|report| {
        RecursiveRfReconciliationError {
            kind: RecursiveRfReconciliationErrorKind::LegacyProjectionRejected,
            message: format!("planet-child RF admission {:?}", report.classification),
        }
    })?;

    let mut rows = Vec::with_capacity(participants.len());
    for participant in participants {
        let scope = scope_key_from_participant(&participant);
        rows.push(PlanetChildRfProjectionRow {
            source_simthing_id_raw: participant.simthing_id_raw,
            planet_gridcell_id_raw: participant.planet_gridcell_id_raw,
            star_system_gridcell_id_raw: scope.star_system_gridcell_id_raw.unwrap_or(0),
            owner_ref: participant.owner_ref.clone(),
            resource_key: super::planet_child_rf::planet_child_rf_default_resource_key(),
            surplus: participant.surplus,
            demand: participant.deficit,
            deficit: participant.deficit,
        });
    }

    rows.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.planet_gridcell_id_raw,
            a.source_simthing_id_raw,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.planet_gridcell_id_raw,
                b.source_simthing_id_raw,
            ))
    });
    Ok(rows)
}

/// Project recursive Location RF aggregate rows and arena settlements for reconciliation.
pub fn project_recursive_local_rf_rows(
    scenario: &SimThingScenarioSpec,
) -> Result<Vec<RecursiveRfProjectionRow>, RecursiveRfReconciliationError> {
    let report =
        evaluate_recursive_local_rf(scenario).map_err(|e| RecursiveRfReconciliationError {
            kind: RecursiveRfReconciliationErrorKind::RecursiveProjectionRejected,
            message: e.message,
        })?;

    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&report);
    let mut rows = Vec::new();

    for source in &aggregate_rows {
        let arena = report
            .arena_reports
            .iter()
            .find(|arena| arena.location_id_raw == source.arena_location_id_raw);
        let parent_location_id = arena.and_then(|arena| arena.parent_location_id);
        let (source_kind_label, net_surplus, net_deficit) = match source.source_kind {
            RecursiveLocalRfAggregateSourceKind::DirectParticipant => {
                ("direct_participant".to_string(), 0, 0)
            }
            RecursiveLocalRfAggregateSourceKind::ChildLocationOutput => (
                "child_location_output".to_string(),
                source.surplus,
                source.demand,
            ),
        };
        rows.push(RecursiveRfProjectionRow {
            source_simthing_or_location_id_raw: source.source_simthing_or_location_id_raw,
            source_kind_label,
            arena_location_id_raw: source.arena_location_id_raw,
            parent_location_id,
            owner_ref: source.owner_ref.clone(),
            resource_key: source.resource_key.clone(),
            surplus: source.surplus,
            demand: source.demand,
            net_surplus,
            net_deficit,
        });
    }

    for arena in &report.arena_reports {
        for settlement in &arena.settlements {
            rows.push(RecursiveRfProjectionRow {
                source_simthing_or_location_id_raw: arena.location_id_raw,
                source_kind_label: "arena_settlement".to_string(),
                arena_location_id_raw: arena.location_id_raw,
                parent_location_id: arena.parent_location_id,
                owner_ref: settlement.owner_ref.clone(),
                resource_key: settlement.resource_key.clone(),
                surplus: settlement.total_surplus,
                demand: settlement.total_demand,
                net_surplus: settlement.net_surplus_to_parent,
                net_deficit: settlement.net_deficit_to_parent,
            });
        }
    }

    rows.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.arena_location_id_raw,
            &a.source_kind_label,
            a.source_simthing_or_location_id_raw,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.arena_location_id_raw,
                &b.source_kind_label,
                b.source_simthing_or_location_id_raw,
            ))
    });
    Ok(rows)
}

/// Reconcile legacy planet-child RF projections with recursive Location RF projections.
pub fn reconcile_planet_child_rf_with_recursive_local_rf(
    scenario: &SimThingScenarioSpec,
) -> Result<RecursiveRfReconciliationReport, RecursiveRfReconciliationError> {
    let legacy_rows = project_planet_child_rf_ladder_rows(scenario)?;
    let recursive_rows = project_recursive_local_rf_rows(scenario)?;

    let direct_recursive: Vec<_> = recursive_rows
        .iter()
        .filter(|row| row.source_kind_label == "direct_participant")
        .collect();

    let mut mismatches = Vec::new();
    let mut participant_row_compatible = true;

    for legacy in &legacy_rows {
        let matching = direct_recursive.iter().find(|recursive| {
            recursive.source_simthing_or_location_id_raw == legacy.source_simthing_id_raw
                && recursive.owner_ref == legacy.owner_ref
                && recursive.surplus == legacy.surplus
                && recursive.demand == legacy.demand
        });

        let Some(recursive) = matching else {
            participant_row_compatible = false;
            mismatches.push(RecursiveRfReconciliationMismatch {
                owner_ref: legacy.owner_ref.clone(),
                resource_key: legacy.resource_key.clone(),
                source_simthing_id_raw: Some(legacy.source_simthing_id_raw),
                location_id_raw: Some(legacy.planet_gridcell_id_raw),
                mismatch_kind:
                    RecursiveRfReconciliationMismatchKind::MissingLegacyRowInRecursiveProjection,
                legacy_value: legacy.surplus as i64,
                recursive_value: 0,
            });
            continue;
        };

        if recursive.resource_key.as_str() != legacy.resource_key.as_str()
            && legacy.resource_key.as_str() != PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY
        {
            mismatches.push(RecursiveRfReconciliationMismatch {
                owner_ref: legacy.owner_ref.clone(),
                resource_key: legacy.resource_key.clone(),
                source_simthing_id_raw: Some(legacy.source_simthing_id_raw),
                location_id_raw: Some(legacy.planet_gridcell_id_raw),
                mismatch_kind: RecursiveRfReconciliationMismatchKind::ResourceKeyMismatch,
                legacy_value: 0,
                recursive_value: 0,
            });
        } else if recursive.resource_key.as_str() != legacy.resource_key.as_str() {
            mismatches.push(RecursiveRfReconciliationMismatch {
                owner_ref: legacy.owner_ref.clone(),
                resource_key: legacy.resource_key.clone(),
                source_simthing_id_raw: Some(legacy.source_simthing_id_raw),
                location_id_raw: Some(legacy.planet_gridcell_id_raw),
                mismatch_kind: RecursiveRfReconciliationMismatchKind::ResourceKeyMismatch,
                legacy_value: 0,
                recursive_value: 0,
            });
        }
    }

    let mut bucket_map: BTreeMap<BucketKey, BucketAccumulator> = BTreeMap::new();

    for legacy in &legacy_rows {
        let key = BucketKey {
            owner_ref: legacy.owner_ref.clone(),
            resource_key: legacy.resource_key.clone(),
            planet_gridcell_id_raw: Some(legacy.planet_gridcell_id_raw),
            star_system_gridcell_id_raw: if legacy.star_system_gridcell_id_raw > 0 {
                Some(legacy.star_system_gridcell_id_raw)
            } else {
                None
            },
        };
        let entry = bucket_map.entry(key).or_default();
        entry.legacy_surplus_total = entry
            .legacy_surplus_total
            .checked_add(legacy.surplus)
            .ok_or_else(|| overflow_error("legacy_surplus_total"))?;
        entry.legacy_demand_total = entry
            .legacy_demand_total
            .checked_add(legacy.demand)
            .ok_or_else(|| overflow_error("legacy_demand_total"))?;
    }

    for recursive in &direct_recursive {
        let star_system_gridcell_id_raw = recursive
            .parent_location_id
            .map(ParentLocationId::raw)
            .filter(|id| *id > 0);
        let key = BucketKey {
            owner_ref: recursive.owner_ref.clone(),
            resource_key: recursive.resource_key.clone(),
            planet_gridcell_id_raw: Some(recursive.arena_location_id_raw),
            star_system_gridcell_id_raw,
        };
        let entry = bucket_map.entry(key).or_default();
        entry.recursive_surplus_total = entry
            .recursive_surplus_total
            .checked_add(recursive.surplus)
            .ok_or_else(|| overflow_error("recursive_surplus_total"))?;
        entry.recursive_demand_total = entry
            .recursive_demand_total
            .checked_add(recursive.demand)
            .ok_or_else(|| overflow_error("recursive_demand_total"))?;
    }

    let mut buckets = Vec::new();
    let mut compatible_bucket_count = 0u32;
    let mut incompatible_bucket_count = 0u32;

    for (key, acc) in bucket_map {
        let surplus_delta = acc.recursive_surplus_total as i64 - acc.legacy_surplus_total as i64;
        let demand_delta = acc.recursive_demand_total as i64 - acc.legacy_demand_total as i64;
        let compatible = surplus_delta == 0 && demand_delta == 0;
        if compatible {
            compatible_bucket_count = compatible_bucket_count
                .checked_add(1)
                .ok_or_else(|| overflow_error("compatible_bucket_count"))?;
        } else {
            incompatible_bucket_count = incompatible_bucket_count
                .checked_add(1)
                .ok_or_else(|| overflow_error("incompatible_bucket_count"))?;
        }
        buckets.push(RecursiveRfReconciliationBucket {
            owner_ref: key.owner_ref,
            resource_key: key.resource_key,
            planet_gridcell_id_raw: key.planet_gridcell_id_raw,
            star_system_gridcell_id_raw: key.star_system_gridcell_id_raw,
            legacy_surplus_total: acc.legacy_surplus_total,
            legacy_demand_total: acc.legacy_demand_total,
            recursive_surplus_total: acc.recursive_surplus_total,
            recursive_demand_total: acc.recursive_demand_total,
            surplus_delta,
            demand_delta,
            compatible,
        });
    }

    buckets.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.planet_gridcell_id_raw,
            a.star_system_gridcell_id_raw,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.planet_gridcell_id_raw,
                b.star_system_gridcell_id_raw,
            ))
    });

    let settlement_rows: Vec<_> = recursive_rows
        .iter()
        .filter(|row| row.source_kind_label == "arena_settlement")
        .collect();

    let mut sibling_redistribution_scope_delta_observed = false;
    for settlement in settlement_rows {
        if settlement.net_surplus > 0 || settlement.net_deficit > 0 {
            sibling_redistribution_scope_delta_observed = true;
            mismatches.push(RecursiveRfReconciliationMismatch {
                owner_ref: settlement.owner_ref.clone(),
                resource_key: settlement.resource_key.clone(),
                source_simthing_id_raw: None,
                location_id_raw: Some(settlement.arena_location_id_raw),
                mismatch_kind: RecursiveRfReconciliationMismatchKind::ScopeProjectionMismatch,
                legacy_value: (settlement.surplus - settlement.net_surplus) as i64,
                recursive_value: settlement.net_surplus as i64,
            });
        }
    }

    mismatches.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.mismatch_kind,
            a.source_simthing_id_raw,
            a.location_id_raw,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.mismatch_kind,
                b.source_simthing_id_raw,
                b.location_id_raw,
            ))
    });

    let previous_ladder_preserved = legacy_rows.iter().all(|legacy| {
        direct_recursive.iter().any(|recursive| {
            recursive.source_simthing_or_location_id_raw == legacy.source_simthing_id_raw
                && recursive.owner_ref == legacy.owner_ref
                && recursive.surplus == legacy.surplus
                && recursive.demand == legacy.demand
        })
    });

    Ok(RecursiveRfReconciliationReport {
        legacy_projection_count: legacy_rows.len() as u32,
        recursive_projection_count: recursive_rows.len() as u32,
        bucket_count: buckets.len() as u32,
        compatible_bucket_count,
        incompatible_bucket_count,
        participant_row_compatible,
        sibling_redistribution_scope_delta_observed,
        buckets,
        mismatches,
        previous_ladder_preserved,
        recursive_evaluator_preserved: !recursive_rows.is_empty(),
        tick_shell_source_replacement_deferred: true,
        scenario_authority_mutation_deferred: true,
        participant_property_mutation_deferred: true,
        semantic_execution_deferred: true,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    })
}

/// Prove Scenario authority is unchanged after reconciliation evaluation.
pub fn prove_recursive_rf_reconciliation_preserves_authority(
    scenario: &SimThingScenarioSpec,
) -> Result<bool, RecursiveRfReconciliationError> {
    let before =
        scenario_authority_digest(scenario).map_err(|e| RecursiveRfReconciliationError {
            kind: RecursiveRfReconciliationErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    let _report = reconcile_planet_child_rf_with_recursive_local_rf(scenario)?;
    let after =
        scenario_authority_digest(scenario).map_err(|e| RecursiveRfReconciliationError {
            kind: RecursiveRfReconciliationErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    Ok(before == after)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BucketKey {
    owner_ref: OwnerRef,
    resource_key: ResourceKey,
    planet_gridcell_id_raw: Option<u32>,
    star_system_gridcell_id_raw: Option<u32>,
}

#[derive(Debug, Default)]
struct BucketAccumulator {
    legacy_surplus_total: u32,
    legacy_demand_total: u32,
    recursive_surplus_total: u32,
    recursive_demand_total: u32,
}

fn overflow_error(field: &str) -> RecursiveRfReconciliationError {
    RecursiveRfReconciliationError {
        kind: RecursiveRfReconciliationErrorKind::ArithmeticOverflow,
        message: format!("{field} overflow"),
    }
}

fn default_deferrals() -> Vec<RecursiveRfReconciliationDeferral> {
    vec![
        RecursiveRfReconciliationDeferral {
            kind: RecursiveRfReconciliationDeferralKind::TickShellSourceReplacementDeferred,
            reason: "runtime tick shell still derives from planet-child/owner-silo ladder"
                .to_string(),
        },
        RecursiveRfReconciliationDeferral {
            kind: RecursiveRfReconciliationDeferralKind::SemanticExecutionDeferred,
            reason: "semantic effect execution remains deferred".to_string(),
        },
        RecursiveRfReconciliationDeferral {
            kind: RecursiveRfReconciliationDeferralKind::ParticipantPropertyMutationDeferred,
            reason: "participant SimThing properties are not mutated".to_string(),
        },
        RecursiveRfReconciliationDeferral {
            kind: RecursiveRfReconciliationDeferralKind::ScenarioAuthorityMutationDeferred,
            reason: "Scenario authority is not mutated".to_string(),
        },
        RecursiveRfReconciliationDeferral {
            kind: RecursiveRfReconciliationDeferralKind::SavefileMutationDeferred,
            reason: "savefile and persistent timeline mutation remain deferred".to_string(),
        },
    ]
}
