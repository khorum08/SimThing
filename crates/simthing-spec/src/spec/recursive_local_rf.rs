//! RECURSIVE-LOCAL-RF-EVALUATOR-0 — recursive Location gridcell RF evaluator nexus.
//!
//! GPU-residency doctrine: runtime RF aggregation lowers to flat GPU-compatible rows/tables.
//! CPU space is limited to deterministic oracle/reference validation, semantic-side bookkeeping,
//! compile-plan construction, and owner/user-facing reports — not production simulation authority.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{SimThing, SimThingKind};

use super::planet_child_rf::{
    evaluate_planet_child_rf_admission, planet_child_rf_participant_inputs,
    PlanetChildRfAdmissionClassification, PlanetChildRfParticipantInput,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};
use super::runtime_tick_history::scenario_authority_digest;
use super::scenario::{
    game_session_galaxy_map, game_session_owners, owner_entity_id, owner_flow_deficit,
    owner_flow_owner_ref, owner_flow_resource_key, owner_flow_surplus, SimThingScenarioSpec,
    OWNER_FLOW_DEFAULT_RESOURCE_KEY,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalRfArenaKey {
    pub owner_ref: String,
    pub resource_key: String,
    pub arena_location_id_raw: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRfParticipantRow {
    pub source_simthing_id_raw: u32,
    pub parent_location_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub surplus: u32,
    pub demand: u32,
    pub participant_kind_label: String,
}

/// Kind of row in the GPU-compatible recursive RF aggregate source table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecursiveLocalRfAggregateSourceKind {
    DirectParticipant,
    ChildLocationOutput,
}

/// Flat GPU-compatible aggregate source row for AccumulatorOp proof lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveLocalRfAggregateSourceRow {
    pub source_kind: RecursiveLocalRfAggregateSourceKind,
    pub source_simthing_or_location_id_raw: u32,
    pub arena_location_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub surplus: u32,
    pub demand: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRfChildOutputRow {
    pub child_location_id_raw: u32,
    pub parent_location_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub net_surplus: u32,
    pub net_deficit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRfArenaSettlement {
    pub arena_location_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub direct_surplus_total: u32,
    pub direct_demand_total: u32,
    pub child_surplus_total: u32,
    pub child_deficit_total: u32,
    pub total_surplus: u32,
    pub total_demand: u32,
    pub locally_matched_total: u32,
    pub net_surplus_to_parent: u32,
    pub net_deficit_to_parent: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocationRfArenaReport {
    pub location_id_raw: u32,
    pub parent_location_id_raw: Option<u32>,
    pub depth: u32,
    pub participant_count: u32,
    pub child_location_count: u32,
    pub child_output_count: u32,
    pub settlement_count: u32,
    pub participant_rows: Vec<LocalRfParticipantRow>,
    pub child_outputs: Vec<LocalRfChildOutputRow>,
    pub settlements: Vec<LocalRfArenaSettlement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursiveLocalRfErrorKind {
    MissingSpatialRoot,
    MissingOwnerChannelForActiveParticipant,
    DuplicateParticipant,
    ArithmeticOverflow,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveLocalRfError {
    pub kind: RecursiveLocalRfErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursiveLocalRfDeferralKind {
    RecursiveRfSimulationDeferred,
    SemanticExecutionDeferred,
    ParticipantPropertyMutationDeferred,
    ScenarioAuthorityMutationDeferred,
    SavefileMutationDeferred,
    StudioPresentationDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveLocalRfDeferral {
    pub kind: RecursiveLocalRfDeferralKind,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveLocalRfEvaluationReport {
    pub root_location_id_raw: u32,
    pub location_count: u32,
    pub participant_count: u32,
    pub child_output_count: u32,
    pub settlement_count: u32,
    pub arena_reports: Vec<LocationRfArenaReport>,
    pub root_outputs: Vec<LocalRfChildOutputRow>,
    pub scenario_authority_mutation_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub semantic_execution_deferred: bool,
    pub previous_rf_ladder_compatibility_preserved: bool,
    pub errors: Vec<RecursiveLocalRfError>,
    pub deferrals: Vec<RecursiveLocalRfDeferral>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveLocalRfAuthorityProof {
    pub scenario_authority_digest_before: String,
    pub scenario_authority_digest_after: String,
    pub scenario_authority_unchanged: bool,
    pub participant_property_mutation_deferred: bool,
    pub semantic_execution_deferred: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveLocalRfCompatibilityReport {
    pub planet_child_participant_count: u32,
    pub planet_child_participants_found_in_recursive: u32,
    pub owner_silo_fixture_compatible: bool,
    pub previous_rf_ladder_preserved: bool,
}

/// Evaluate recursive local RF from GalaxyMap downward through Location arenas.
pub fn evaluate_recursive_local_rf(
    scenario: &SimThingScenarioSpec,
) -> Result<RecursiveLocalRfEvaluationReport, RecursiveLocalRfError> {
    let galaxy_map = game_session_galaxy_map(scenario).map_err(|e| RecursiveLocalRfError {
        kind: RecursiveLocalRfErrorKind::MissingSpatialRoot,
        message: format!("{:?}", e),
    })?;

    let owner_refs: BTreeSet<String> = game_session_owners(scenario)
        .map_err(|e| RecursiveLocalRfError {
            kind: RecursiveLocalRfErrorKind::MissingSpatialRoot,
            message: format!("{:?}", e),
        })?
        .into_iter()
        .filter_map(owner_entity_id)
        .collect();

    let mut arena_reports = Vec::new();
    let root_outputs =
        evaluate_location_arena(galaxy_map, None, 0, &owner_refs, &mut arena_reports)?;

    let participant_count = arena_reports
        .iter()
        .map(|report| report.participant_count)
        .try_fold(0u32, |acc, v| {
            acc.checked_add(v).ok_or_else(|| RecursiveLocalRfError {
                kind: RecursiveLocalRfErrorKind::ArithmeticOverflow,
                message: "participant_count overflow".to_string(),
            })
        })?;
    let child_output_count = arena_reports
        .iter()
        .map(|report| report.child_output_count)
        .try_fold(0u32, |acc, v| {
            acc.checked_add(v).ok_or_else(|| RecursiveLocalRfError {
                kind: RecursiveLocalRfErrorKind::ArithmeticOverflow,
                message: "child_output_count overflow".to_string(),
            })
        })?;
    let settlement_count = arena_reports
        .iter()
        .map(|report| report.settlement_count)
        .try_fold(0u32, |acc, v| {
            acc.checked_add(v).ok_or_else(|| RecursiveLocalRfError {
                kind: RecursiveLocalRfErrorKind::ArithmeticOverflow,
                message: "settlement_count overflow".to_string(),
            })
        })?;

    let report = RecursiveLocalRfEvaluationReport {
        root_location_id_raw: galaxy_map.id.raw(),
        location_count: arena_reports.len() as u32,
        participant_count,
        child_output_count,
        settlement_count,
        arena_reports,
        root_outputs,
        scenario_authority_mutation_deferred: true,
        participant_property_mutation_deferred: true,
        semantic_execution_deferred: true,
        previous_rf_ladder_compatibility_preserved: false,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    };

    let compatibility = compatibility_from_report(scenario, &report)?;
    Ok(RecursiveLocalRfEvaluationReport {
        previous_rf_ladder_compatibility_preserved: compatibility.previous_rf_ladder_preserved,
        ..report
    })
}

/// Prove Scenario authority is unchanged after recursive local RF evaluation.
pub fn prove_recursive_local_rf_preserves_authority(
    scenario: &SimThingScenarioSpec,
) -> Result<RecursiveLocalRfAuthorityProof, RecursiveLocalRfError> {
    let scenario_authority_digest_before =
        scenario_authority_digest(scenario).map_err(|e| RecursiveLocalRfError {
            kind: RecursiveLocalRfErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;

    let _report = evaluate_recursive_local_rf(scenario)?;

    let scenario_authority_digest_after =
        scenario_authority_digest(scenario).map_err(|e| RecursiveLocalRfError {
            kind: RecursiveLocalRfErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;

    let scenario_authority_unchanged =
        scenario_authority_digest_before == scenario_authority_digest_after;

    Ok(RecursiveLocalRfAuthorityProof {
        scenario_authority_digest_before,
        scenario_authority_digest_after,
        scenario_authority_unchanged,
        participant_property_mutation_deferred: true,
        semantic_execution_deferred: true,
    })
}

/// Convert planet-child RF participant inputs into recursive participant rows.
pub fn recursive_local_rf_participant_rows_from_planet_child_inputs(
    inputs: &[PlanetChildRfParticipantInput],
) -> Vec<LocalRfParticipantRow> {
    inputs
        .iter()
        .map(|input| LocalRfParticipantRow {
            source_simthing_id_raw: input.simthing_id_raw,
            parent_location_id_raw: input.planet_gridcell_id_raw,
            owner_ref: input.owner_ref.clone(),
            resource_key: PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY.to_string(),
            surplus: input.surplus,
            demand: input.deficit,
            participant_kind_label: input.participant_kind_label.clone(),
        })
        .collect()
}

/// Verify planet-child RF participants are derivable from the recursive evaluator slice.
pub fn recursive_local_rf_report_matches_planet_child_compatibility_slice(
    scenario: &SimThingScenarioSpec,
) -> Result<RecursiveLocalRfCompatibilityReport, RecursiveLocalRfError> {
    let report = evaluate_recursive_local_rf(scenario)?;
    compatibility_from_report(scenario, &report)
}

fn compatibility_from_report(
    scenario: &SimThingScenarioSpec,
    report: &RecursiveLocalRfEvaluationReport,
) -> Result<RecursiveLocalRfCompatibilityReport, RecursiveLocalRfError> {
    let admission = evaluate_planet_child_rf_admission(scenario);
    let ladder_preserved =
        admission.classification != PlanetChildRfAdmissionClassification::Rejected;

    let planet_inputs = match planet_child_rf_participant_inputs(scenario) {
        Ok(inputs) => inputs,
        Err(_) => {
            return Ok(RecursiveLocalRfCompatibilityReport {
                planet_child_participant_count: 0,
                planet_child_participants_found_in_recursive: 0,
                owner_silo_fixture_compatible: ladder_preserved,
                previous_rf_ladder_preserved: ladder_preserved,
            });
        }
    };

    let adapted = recursive_local_rf_participant_rows_from_planet_child_inputs(&planet_inputs);

    let mut found = 0u32;
    for expected in &adapted {
        let present = report.arena_reports.iter().any(|arena| {
            arena.participant_rows.iter().any(|row| {
                row.source_simthing_id_raw == expected.source_simthing_id_raw
                    && row.parent_location_id_raw == expected.parent_location_id_raw
                    && row.owner_ref == expected.owner_ref
                    && row.resource_key == expected.resource_key
                    && row.surplus == expected.surplus
                    && row.demand == expected.demand
            })
        });
        if present {
            found = found.checked_add(1).ok_or_else(|| RecursiveLocalRfError {
                kind: RecursiveLocalRfErrorKind::ArithmeticOverflow,
                message: "compatibility found count overflow".to_string(),
            })?;
        }
    }

    let previous_rf_ladder_preserved = ladder_preserved && found == planet_inputs.len() as u32;

    Ok(RecursiveLocalRfCompatibilityReport {
        planet_child_participant_count: planet_inputs.len() as u32,
        planet_child_participants_found_in_recursive: found,
        owner_silo_fixture_compatible: previous_rf_ladder_preserved,
        previous_rf_ladder_preserved,
    })
}

/// Flatten recursive evaluation report into GPU-compatible aggregate source rows.
///
/// Direct participants contribute surplus/demand; child Location outputs contribute
/// net_surplus/net_deficit. This table shape is the target for GPU-resident lowering —
/// not a CPU-owned runtime evaluator.
pub fn recursive_local_rf_aggregate_source_rows(
    report: &RecursiveLocalRfEvaluationReport,
) -> Vec<RecursiveLocalRfAggregateSourceRow> {
    let mut rows = Vec::new();
    for arena in &report.arena_reports {
        for row in &arena.participant_rows {
            rows.push(RecursiveLocalRfAggregateSourceRow {
                source_kind: RecursiveLocalRfAggregateSourceKind::DirectParticipant,
                source_simthing_or_location_id_raw: row.source_simthing_id_raw,
                arena_location_id_raw: arena.location_id_raw,
                owner_ref: row.owner_ref.clone(),
                resource_key: row.resource_key.clone(),
                surplus: row.surplus,
                demand: row.demand,
            });
        }
        for child in &arena.child_outputs {
            rows.push(RecursiveLocalRfAggregateSourceRow {
                source_kind: RecursiveLocalRfAggregateSourceKind::ChildLocationOutput,
                source_simthing_or_location_id_raw: child.child_location_id_raw,
                arena_location_id_raw: arena.location_id_raw,
                owner_ref: child.owner_ref.clone(),
                resource_key: child.resource_key.clone(),
                surplus: child.net_surplus,
                demand: child.net_deficit,
            });
        }
    }
    rows.sort_by(|a, b| {
        (
            a.arena_location_id_raw,
            &a.owner_ref,
            &a.resource_key,
            a.source_kind,
            a.source_simthing_or_location_id_raw,
        )
            .cmp(&(
                b.arena_location_id_raw,
                &b.owner_ref,
                &b.resource_key,
                b.source_kind,
                b.source_simthing_or_location_id_raw,
            ))
    });
    rows
}

/// Aggregate surplus/demand totals per arena/owner/resource for GPU proof comparison.
pub fn recursive_local_rf_arena_aggregate_totals(
    report: &RecursiveLocalRfEvaluationReport,
) -> BTreeMap<(u32, String, String), (u32, u32)> {
    let mut totals: BTreeMap<(u32, String, String), (u32, u32)> = BTreeMap::new();
    for arena in &report.arena_reports {
        for settlement in &arena.settlements {
            totals.insert(
                (
                    settlement.arena_location_id_raw,
                    settlement.owner_ref.clone(),
                    settlement.resource_key.clone(),
                ),
                (settlement.total_surplus, settlement.total_demand),
            );
        }
    }
    totals
}

fn evaluate_location_arena(
    location: &SimThing,
    parent_location_id_raw: Option<u32>,
    depth: u32,
    owner_refs: &BTreeSet<String>,
    arena_reports: &mut Vec<LocationRfArenaReport>,
) -> Result<Vec<LocalRfChildOutputRow>, RecursiveLocalRfError> {
    let location_id = location.id.raw();
    let mut incoming_child_outputs = Vec::new();
    let mut child_location_count = 0u32;

    for child in &location.children {
        if child.kind == SimThingKind::Location {
            child_location_count =
                child_location_count
                    .checked_add(1)
                    .ok_or_else(|| RecursiveLocalRfError {
                        kind: RecursiveLocalRfErrorKind::ArithmeticOverflow,
                        message: "child_location_count overflow".to_string(),
                    })?;
            let child_outputs = evaluate_location_arena(
                child,
                Some(location_id),
                depth.checked_add(1).ok_or_else(|| RecursiveLocalRfError {
                    kind: RecursiveLocalRfErrorKind::ArithmeticOverflow,
                    message: "depth overflow".to_string(),
                })?,
                owner_refs,
                arena_reports,
            )?;
            incoming_child_outputs.extend(child_outputs);
        }
    }

    let mut participant_rows = Vec::new();
    let mut seen_participants = BTreeSet::new();

    collect_participant_row(
        location,
        location_id,
        location_participant_kind_label(location),
        owner_refs,
        &mut seen_participants,
        &mut participant_rows,
    )?;

    for child in &location.children {
        if child.kind == SimThingKind::Location {
            continue;
        }
        collect_participant_row(
            child,
            location_id,
            non_location_participant_kind_label(child),
            owner_refs,
            &mut seen_participants,
            &mut participant_rows,
        )?;
    }

    participant_rows.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.source_simthing_id_raw,
            &a.participant_kind_label,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.source_simthing_id_raw,
                &b.participant_kind_label,
            ))
    });

    let mut groups: BTreeMap<(String, String), ArenaAccumulator> = BTreeMap::new();
    for row in &participant_rows {
        let entry = groups
            .entry((row.owner_ref.clone(), row.resource_key.clone()))
            .or_default();
        entry.direct_surplus_total = entry
            .direct_surplus_total
            .checked_add(row.surplus)
            .ok_or_else(|| overflow_error("direct_surplus_total"))?;
        entry.direct_demand_total = entry
            .direct_demand_total
            .checked_add(row.demand)
            .ok_or_else(|| overflow_error("direct_demand_total"))?;
    }

    let mut child_outputs_for_report = Vec::new();
    for child_output in &incoming_child_outputs {
        if child_output.parent_location_id_raw != location_id {
            continue;
        }
        child_outputs_for_report.push(child_output.clone());
        let entry = groups
            .entry((
                child_output.owner_ref.clone(),
                child_output.resource_key.clone(),
            ))
            .or_default();
        entry.child_surplus_total = entry
            .child_surplus_total
            .checked_add(child_output.net_surplus)
            .ok_or_else(|| overflow_error("child_surplus_total"))?;
        entry.child_deficit_total = entry
            .child_deficit_total
            .checked_add(child_output.net_deficit)
            .ok_or_else(|| overflow_error("child_deficit_total"))?;
    }

    child_outputs_for_report.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.child_location_id_raw,
            a.net_surplus,
            a.net_deficit,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.child_location_id_raw,
                b.net_surplus,
                b.net_deficit,
            ))
    });

    let mut settlements = Vec::new();
    let mut parent_outputs = Vec::new();

    for ((owner_ref, resource_key), acc) in groups {
        let total_surplus = acc
            .direct_surplus_total
            .checked_add(acc.child_surplus_total)
            .ok_or_else(|| overflow_error("total_surplus"))?;
        let total_demand = acc
            .direct_demand_total
            .checked_add(acc.child_deficit_total)
            .ok_or_else(|| overflow_error("total_demand"))?;
        let locally_matched_total = total_surplus.min(total_demand);
        let net_surplus_to_parent = total_surplus - locally_matched_total;
        let net_deficit_to_parent = total_demand - locally_matched_total;

        settlements.push(LocalRfArenaSettlement {
            arena_location_id_raw: location_id,
            owner_ref: owner_ref.clone(),
            resource_key: resource_key.clone(),
            direct_surplus_total: acc.direct_surplus_total,
            direct_demand_total: acc.direct_demand_total,
            child_surplus_total: acc.child_surplus_total,
            child_deficit_total: acc.child_deficit_total,
            total_surplus,
            total_demand,
            locally_matched_total,
            net_surplus_to_parent,
            net_deficit_to_parent,
        });

        if net_surplus_to_parent > 0 || net_deficit_to_parent > 0 {
            parent_outputs.push(LocalRfChildOutputRow {
                child_location_id_raw: location_id,
                parent_location_id_raw: parent_location_id_raw.unwrap_or(0),
                owner_ref,
                resource_key,
                net_surplus: net_surplus_to_parent,
                net_deficit: net_deficit_to_parent,
            });
        }
    }

    settlements.sort_by(|a, b| {
        (&a.owner_ref, &a.resource_key, a.arena_location_id_raw).cmp(&(
            &b.owner_ref,
            &b.resource_key,
            b.arena_location_id_raw,
        ))
    });
    parent_outputs.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.child_location_id_raw,
            a.net_surplus,
            a.net_deficit,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.child_location_id_raw,
                b.net_surplus,
                b.net_deficit,
            ))
    });

    arena_reports.push(LocationRfArenaReport {
        location_id_raw: location_id,
        parent_location_id_raw,
        depth,
        participant_count: participant_rows.len() as u32,
        child_location_count,
        child_output_count: child_outputs_for_report.len() as u32,
        settlement_count: settlements.len() as u32,
        participant_rows,
        child_outputs: child_outputs_for_report,
        settlements,
    });

    Ok(parent_outputs)
}

fn collect_participant_row(
    node: &SimThing,
    parent_location_id_raw: u32,
    kind_label: String,
    owner_refs: &BTreeSet<String>,
    seen_participants: &mut BTreeSet<u32>,
    participant_rows: &mut Vec<LocalRfParticipantRow>,
) -> Result<(), RecursiveLocalRfError> {
    let surplus = owner_flow_surplus(node).unwrap_or(0);
    let deficit = owner_flow_deficit(node).unwrap_or(0);
    let demand = deficit;
    if surplus == 0 && deficit == 0 {
        return Ok(());
    }

    let Some(owner_ref) = owner_flow_owner_ref(node) else {
        return Err(RecursiveLocalRfError {
            kind: RecursiveLocalRfErrorKind::MissingOwnerChannelForActiveParticipant,
            message: format!(
                "active RF participant {} requires owner/channel reference",
                node.id.raw()
            ),
        });
    };
    if owner_ref.trim().is_empty() {
        return Err(RecursiveLocalRfError {
            kind: RecursiveLocalRfErrorKind::MissingOwnerChannelForActiveParticipant,
            message: format!(
                "active RF participant {} has empty owner_ref",
                node.id.raw()
            ),
        });
    }
    if !owner_refs.contains(&owner_ref) {
        return Err(RecursiveLocalRfError {
            kind: RecursiveLocalRfErrorKind::MissingOwnerChannelForActiveParticipant,
            message: format!(
                "active RF participant {} references unknown owner `{owner_ref}`",
                node.id.raw()
            ),
        });
    }
    if !seen_participants.insert(node.id.raw()) {
        return Err(RecursiveLocalRfError {
            kind: RecursiveLocalRfErrorKind::DuplicateParticipant,
            message: format!("duplicate RF participant id {}", node.id.raw()),
        });
    }

    participant_rows.push(LocalRfParticipantRow {
        source_simthing_id_raw: node.id.raw(),
        parent_location_id_raw,
        owner_ref,
        resource_key: owner_flow_resource_key(node),
        surplus,
        demand,
        participant_kind_label: kind_label,
    });
    Ok(())
}

fn location_participant_kind_label(location: &SimThing) -> String {
    if super::planet_child_location::is_planet_gridcell(location) {
        "planet_gridcell".to_string()
    } else if super::scenario::is_galaxy_map_entity(location) {
        "galaxy_map".to_string()
    } else {
        "location_gridcell".to_string()
    }
}

fn non_location_participant_kind_label(node: &SimThing) -> String {
    match &node.kind {
        SimThingKind::Custom(label) => label.clone(),
        other => format!("{other:?}"),
    }
}

#[derive(Debug, Default)]
struct ArenaAccumulator {
    direct_surplus_total: u32,
    direct_demand_total: u32,
    child_surplus_total: u32,
    child_deficit_total: u32,
}

fn overflow_error(field: &str) -> RecursiveLocalRfError {
    RecursiveLocalRfError {
        kind: RecursiveLocalRfErrorKind::ArithmeticOverflow,
        message: format!("{field} overflow"),
    }
}

fn default_deferrals() -> Vec<RecursiveLocalRfDeferral> {
    vec![
        RecursiveLocalRfDeferral {
            kind: RecursiveLocalRfDeferralKind::RecursiveRfSimulationDeferred,
            reason: "recursive local RF evaluation is proof-only; integration into tick shell remains deferred"
                .to_string(),
        },
        RecursiveLocalRfDeferral {
            kind: RecursiveLocalRfDeferralKind::SemanticExecutionDeferred,
            reason: "semantic effect execution remains deferred".to_string(),
        },
        RecursiveLocalRfDeferral {
            kind: RecursiveLocalRfDeferralKind::ParticipantPropertyMutationDeferred,
            reason: "participant SimThing properties are not mutated".to_string(),
        },
        RecursiveLocalRfDeferral {
            kind: RecursiveLocalRfDeferralKind::ScenarioAuthorityMutationDeferred,
            reason: "Scenario authority is not mutated".to_string(),
        },
        RecursiveLocalRfDeferral {
            kind: RecursiveLocalRfDeferralKind::SavefileMutationDeferred,
            reason: "savefile and persistent timeline mutation remain deferred".to_string(),
        },
        RecursiveLocalRfDeferral {
            kind: RecursiveLocalRfDeferralKind::StudioPresentationDeferred,
            reason: "Studio recursive RF presentation remains deferred".to_string(),
        },
    ]
}

pub fn recursive_local_rf_default_resource_key() -> &'static str {
    OWNER_FLOW_DEFAULT_RESOURCE_KEY
}
