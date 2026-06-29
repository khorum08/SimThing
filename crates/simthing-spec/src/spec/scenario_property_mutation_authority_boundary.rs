//! SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 — cloned ScenarioSpec candidate property mutation.
//!
//! Recursive-source runtime participant property-view rows apply to a cloned ScenarioSpec candidate
//! behind explicit source mode. CPU responsibilities: oracle/reference/shadow projection,
//! semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.

use simthing_core::{PropertyValue, SimPropertyId, SimThing};

use super::channel_key::{OwnerRef, ResourceKey};
use super::runtime_participant_property_mutation_boundary::{
    evaluate_runtime_participant_property_mutation_boundary,
    RuntimeParticipantPropertyMutationBoundaryReport, RuntimeParticipantPropertyMutationSourceMode,
    RuntimeParticipantPropertyViewRow,
};
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::{
    serialize_scenario_authority, SimThingScenarioSpec, RUNTIME_PREVIEW_APPLIED_SIM_PROPERTY_ID,
    RUNTIME_PREVIEW_SATISFIED_SIM_PROPERTY_ID, RUNTIME_PREVIEW_SHORTFALL_SIM_PROPERTY_ID,
};
use super::semantic_participant_delta_preview::{
    RUNTIME_PREVIEW_APPLIED_PROPERTY_ID, RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID,
    RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID,
};

pub const MIN_SCENARIO_PROPERTY_MUTATION_REPLAY_COUNT: u32 = 1;
pub const MAX_SCENARIO_PROPERTY_MUTATION_REPLAY_COUNT: u32 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioPropertyMutationSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveRuntimePropertyViewSelectable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioPropertyMutationRecord {
    pub source_property_view_index: u32,
    pub participant_simthing_id_raw: u32,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: Option<u32>,
    pub property_id: String,
    pub before_value: Option<f64>,
    pub runtime_property_view_value: f64,
    pub candidate_after_value: f64,
    pub candidate_property_mutation_applied: bool,
    pub input_scenario_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioPropertyMutationAuthorityBoundaryReport {
    pub source_mode: ScenarioPropertyMutationSourceMode,
    pub selected_source_mode: ScenarioPropertyMutationSourceMode,
    pub selection_allowed: bool,
    pub runtime_property_view_ready: bool,
    pub original_before_authority_digest: u64,
    pub original_after_authority_digest: u64,
    pub candidate_after_authority_digest: u64,
    pub original_scenario_unchanged: bool,
    pub candidate_scenario_mutated: bool,
    pub mutation_records: Vec<ScenarioPropertyMutationRecord>,
    pub mutation_record_count: u32,
    pub candidate_property_mutation_applied: bool,
    pub input_scenario_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioPropertyMutationAuthorityBoundaryReplayReport {
    pub replay_count: u32,
    pub replay_deterministic: bool,
    pub reference_report: ScenarioPropertyMutationAuthorityBoundaryReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioPropertyMutationAuthorityBoundaryErrorKind {
    PropertyViewRejected,
    CandidateMutationRejected,
    SelectionDenied,
    ScenarioAuthorityRejected,
    ReplayRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioPropertyMutationAuthorityBoundaryError {
    pub kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind,
    pub message: String,
}

/// Evaluate scenario property mutation authority boundary with explicit source mode.
pub fn evaluate_scenario_property_mutation_authority_boundary(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: ScenarioPropertyMutationSourceMode,
    replay_count: u32,
) -> Result<
    ScenarioPropertyMutationAuthorityBoundaryReport,
    ScenarioPropertyMutationAuthorityBoundaryError,
> {
    if tick_id.0 == 0 {
        return Err(ScenarioPropertyMutationAuthorityBoundaryError {
            kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::SelectionDenied,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let property_view_report = evaluate_runtime_participant_property_mutation_boundary(
        scenario,
        tick_id,
        map_to_property_view_source_mode(source_mode),
        replay_count,
    )
    .map_err(|e| ScenarioPropertyMutationAuthorityBoundaryError {
        kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::PropertyViewRejected,
        message: e.message,
    })?;

    let original_before_authority_digest =
        scenario_authority_digest_u64(scenario).map_err(map_authority_error)?;
    let mut candidate = scenario.clone();
    let (mutation_records, candidate_property_mutation_applied) =
        apply_property_view_rows_to_candidate(&property_view_report, &mut candidate, source_mode)?;

    let original_after_authority_digest =
        scenario_authority_digest_u64(scenario).map_err(map_authority_error)?;
    let candidate_after_authority_digest =
        scenario_authority_digest_u64(&candidate).map_err(map_authority_error)?;

    let mutation_record_count = u32::try_from(mutation_records.len()).map_err(|_| {
        ScenarioPropertyMutationAuthorityBoundaryError {
            kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::CandidateMutationRejected,
            message: "mutation_record_count exceeds u32".to_string(),
        }
    })?;

    let original_scenario_unchanged =
        original_before_authority_digest == original_after_authority_digest;
    let candidate_scenario_mutated = candidate_property_mutation_applied
        && (mutation_record_count > 0)
        && (candidate_after_authority_digest != original_before_authority_digest);

    let selected_source_mode =
        map_from_property_view_source_mode(property_view_report.selected_source_mode);
    let runtime_property_view_ready = property_view_report.runtime_state_mutation_ready
        && property_view_report.runtime_property_view_mutation_applied;

    let selection_allowed = match source_mode {
        ScenarioPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo => true,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable => {
            property_view_report.selection_allowed
                && runtime_property_view_ready
                && selected_source_mode
                    == ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable
        }
    };

    Ok(ScenarioPropertyMutationAuthorityBoundaryReport {
        source_mode,
        selected_source_mode,
        selection_allowed,
        runtime_property_view_ready,
        original_before_authority_digest,
        original_after_authority_digest,
        candidate_after_authority_digest,
        original_scenario_unchanged,
        candidate_scenario_mutated,
        mutation_records,
        mutation_record_count,
        candidate_property_mutation_applied,
        input_scenario_property_mutation_deferred: true,
        savefile_mutation_deferred: true,
        persistent_history_deferred: true,
    })
}

/// Clone loaded ScenarioSpec and apply recursive runtime property-view rows to the candidate only.
pub fn clone_scenario_candidate_with_runtime_property_view(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    replay_count: u32,
) -> Result<SimThingScenarioSpec, ScenarioPropertyMutationAuthorityBoundaryError> {
    let property_view_report = evaluate_runtime_participant_property_mutation_boundary(
        scenario,
        tick_id,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        replay_count,
    )
    .map_err(|e| ScenarioPropertyMutationAuthorityBoundaryError {
        kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::PropertyViewRejected,
        message: e.message,
    })?;

    let mut candidate = scenario.clone();
    let _ = apply_property_view_rows_to_candidate(
        &property_view_report,
        &mut candidate,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
    )?;
    Ok(candidate)
}

/// Prove input ScenarioSpec authority is unchanged after candidate-only mutation evaluation.
pub fn prove_scenario_property_mutation_boundary_preserves_original_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: ScenarioPropertyMutationSourceMode,
    replay_count: u32,
) -> Result<bool, ScenarioPropertyMutationAuthorityBoundaryError> {
    let before = scenario_authority_digest_u64(scenario).map_err(map_authority_error)?;
    let report = evaluate_scenario_property_mutation_authority_boundary(
        scenario,
        tick_id,
        source_mode,
        replay_count,
    )?;
    let after = scenario_authority_digest_u64(scenario).map_err(map_authority_error)?;
    Ok(before == after && report.original_scenario_unchanged)
}

/// Replay scenario property mutation authority boundary evaluation and verify determinism.
pub fn replay_scenario_property_mutation_authority_boundary(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: ScenarioPropertyMutationSourceMode,
    replay_count: u32,
) -> Result<
    ScenarioPropertyMutationAuthorityBoundaryReplayReport,
    ScenarioPropertyMutationAuthorityBoundaryError,
> {
    if !(MIN_SCENARIO_PROPERTY_MUTATION_REPLAY_COUNT..=MAX_SCENARIO_PROPERTY_MUTATION_REPLAY_COUNT)
        .contains(&replay_count)
    {
        return Err(ScenarioPropertyMutationAuthorityBoundaryError {
            kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::ReplayRejected,
            message: format!(
                "replay_count must be in {}..={}",
                MIN_SCENARIO_PROPERTY_MUTATION_REPLAY_COUNT,
                MAX_SCENARIO_PROPERTY_MUTATION_REPLAY_COUNT
            ),
        });
    }

    let reference =
        evaluate_scenario_property_mutation_authority_boundary(scenario, tick_id, source_mode, 1)?;
    let mut replay_deterministic = true;
    for _ in 1..replay_count {
        let replay = evaluate_scenario_property_mutation_authority_boundary(
            scenario,
            tick_id,
            source_mode,
            1,
        )?;
        if replay != reference {
            replay_deterministic = false;
            break;
        }
    }

    Ok(ScenarioPropertyMutationAuthorityBoundaryReplayReport {
        replay_count,
        replay_deterministic,
        reference_report: reference,
    })
}

fn map_to_property_view_source_mode(
    source_mode: ScenarioPropertyMutationSourceMode,
) -> RuntimeParticipantPropertyMutationSourceMode {
    match source_mode {
        ScenarioPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo => {
            RuntimeParticipantPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo
        }
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable => {
            RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable
        }
    }
}

fn map_from_property_view_source_mode(
    source_mode: RuntimeParticipantPropertyMutationSourceMode,
) -> ScenarioPropertyMutationSourceMode {
    match source_mode {
        RuntimeParticipantPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo => {
            ScenarioPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo
        }
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable => {
            ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable
        }
    }
}

fn map_authority_error(
    e: super::scenario::ScenarioSerdeError,
) -> ScenarioPropertyMutationAuthorityBoundaryError {
    ScenarioPropertyMutationAuthorityBoundaryError {
        kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::ScenarioAuthorityRejected,
        message: e.to_string(),
    }
}

fn scenario_authority_digest_u64(
    scenario: &SimThingScenarioSpec,
) -> Result<u64, super::scenario::ScenarioSerdeError> {
    let json = serialize_scenario_authority(scenario)?;
    Ok(fnv1a64_u64(&json))
}

fn fnv1a64_u64(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn apply_property_view_rows_to_candidate(
    property_view_report: &RuntimeParticipantPropertyMutationBoundaryReport,
    candidate: &mut SimThingScenarioSpec,
    source_mode: ScenarioPropertyMutationSourceMode,
) -> Result<
    (Vec<ScenarioPropertyMutationRecord>, bool),
    ScenarioPropertyMutationAuthorityBoundaryError,
> {
    if source_mode == ScenarioPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo {
        return Ok((Vec::new(), false));
    }

    if !property_view_report.runtime_property_view_mutation_applied {
        return Ok((Vec::new(), false));
    }

    let mut mutation_records =
        Vec::with_capacity(property_view_report.after_property_view_rows.len());

    for (index, row) in property_view_report
        .after_property_view_rows
        .iter()
        .enumerate()
    {
        if row.value == 0.0 {
            continue;
        }

        let record = apply_property_view_row_to_candidate(candidate, row, index)?;
        if record.candidate_property_mutation_applied {
            mutation_records.push(record);
        }
    }

    mutation_records.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.scope_id,
            a.participant_simthing_id_raw,
            &a.property_id,
            a.source_property_view_index,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.scope_id,
                b.participant_simthing_id_raw,
                &b.property_id,
                b.source_property_view_index,
            ))
    });

    for (index, record) in mutation_records.iter_mut().enumerate() {
        record.source_property_view_index = u32::try_from(index + 1).map_err(|_| {
            ScenarioPropertyMutationAuthorityBoundaryError {
                kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::CandidateMutationRejected,
                message: "sorted source_property_view_index exceeds u32".to_string(),
            }
        })?;
    }

    let candidate_property_mutation_applied = !mutation_records.is_empty()
        && mutation_records
            .iter()
            .all(|record| record.candidate_property_mutation_applied);

    Ok((mutation_records, candidate_property_mutation_applied))
}

fn apply_property_view_row_to_candidate(
    candidate: &mut SimThingScenarioSpec,
    row: &RuntimeParticipantPropertyViewRow,
    index: usize,
) -> Result<ScenarioPropertyMutationRecord, ScenarioPropertyMutationAuthorityBoundaryError> {
    validate_finite_f64(row.value, "runtime_property_view_value")?;

    let participant =
        find_simthing_by_raw_id_mut(&mut candidate.root, row.participant_simthing_id_raw)
            .ok_or_else(|| ScenarioPropertyMutationAuthorityBoundaryError {
                kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::CandidateMutationRejected,
                message: format!(
                    "participant simthing id {} not found in candidate scenario",
                    row.participant_simthing_id_raw
                ),
            })?;

    let property_id = map_preview_property_id(&row.property_id)?;
    let before_value = participant
        .properties
        .get(&property_id)
        .and_then(property_value_to_f64);

    let runtime_property_view_value = row.value;
    let candidate_after_value = runtime_property_view_value;
    participant.properties.insert(
        property_id,
        PropertyValue::from_raw_lanes(vec![candidate_after_value as f32]),
    );

    let source_property_view_index =
        u32::try_from(index + 1).map_err(|_| ScenarioPropertyMutationAuthorityBoundaryError {
            kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::CandidateMutationRejected,
            message: "source_property_view_index exceeds u32".to_string(),
        })?;

    Ok(ScenarioPropertyMutationRecord {
        source_property_view_index,
        participant_simthing_id_raw: row.participant_simthing_id_raw,
        owner_ref: row.owner_ref.clone(),
        resource_key: row.resource_key.clone(),
        scope_id: row.scope_id,
        property_id: row.property_id.clone(),
        before_value,
        runtime_property_view_value,
        candidate_after_value,
        candidate_property_mutation_applied: true,
        input_scenario_property_mutation_deferred: true,
        savefile_mutation_deferred: true,
        persistent_history_deferred: true,
    })
}

fn map_preview_property_id(
    property_id: &str,
) -> Result<SimPropertyId, ScenarioPropertyMutationAuthorityBoundaryError> {
    match property_id {
        RUNTIME_PREVIEW_APPLIED_PROPERTY_ID => Ok(RUNTIME_PREVIEW_APPLIED_SIM_PROPERTY_ID),
        RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID => Ok(RUNTIME_PREVIEW_SATISFIED_SIM_PROPERTY_ID),
        RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID => Ok(RUNTIME_PREVIEW_SHORTFALL_SIM_PROPERTY_ID),
        other => Err(ScenarioPropertyMutationAuthorityBoundaryError {
            kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::CandidateMutationRejected,
            message: format!("unsupported preview property id: {other}"),
        }),
    }
}

fn property_value_to_f64(value: &PropertyValue) -> Option<f64> {
    value
        .raw_lanes_for_serialization()
        .first()
        .copied()
        .map(f64::from)
}

fn find_simthing_by_raw_id_mut<'a>(thing: &'a mut SimThing, raw: u32) -> Option<&'a mut SimThing> {
    if thing.id.raw() == raw {
        return Some(thing);
    }
    for child in &mut thing.children {
        if let Some(found) = find_simthing_by_raw_id_mut(child, raw) {
            return Some(found);
        }
    }
    None
}

fn validate_finite_f64(
    value: f64,
    label: &str,
) -> Result<(), ScenarioPropertyMutationAuthorityBoundaryError> {
    if !value.is_finite() {
        return Err(ScenarioPropertyMutationAuthorityBoundaryError {
            kind: ScenarioPropertyMutationAuthorityBoundaryErrorKind::CandidateMutationRejected,
            message: format!("{label} must be finite"),
        });
    }
    Ok(())
}
