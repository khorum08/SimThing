//! RUNTIME-PARTICIPANT-STATE-MUTATION-0 — runtime-only participant state mutation without persistence.
//!
//! Semantic delta preview records apply to ephemeral runtime participant state rows behind explicit
//! source mode. CPU responsibilities: oracle/reference/shadow projection, semantic-side bookkeeping,
//! compile-plan construction, and owner/user-facing report formatting.

use std::collections::BTreeMap;

use super::runtime_tick_history::scenario_authority_digest;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::SimThingScenarioSpec;
use super::semantic_participant_delta_preview::{
    evaluate_semantic_participant_delta_preview, ParticipantDeltaPreviewKind,
    ParticipantDeltaPreviewSourceMode, ParticipantPropertyDeltaPreviewRecord,
    SemanticParticipantDeltaPreviewReport,
};

pub const MIN_RUNTIME_PARTICIPANT_STATE_REPLAY_COUNT: u32 = 1;
pub const MAX_RUNTIME_PARTICIPANT_STATE_REPLAY_COUNT: u32 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeParticipantStateMutationSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveDeltaPreviewSelectable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimeParticipantStateMutationKind {
    ApplyResourceSatisfiedDelta,
    ApplyResourceShortfallDelta,
    ApplyRuntimeAppliedAmountDelta,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantStateRow {
    pub participant_simthing_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub scope_id: Option<u32>,
    pub property_id: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantStateMutationRecord {
    pub source_delta_preview_id: u32,
    pub participant_simthing_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub scope_id: Option<u32>,
    pub mutation_kind: RuntimeParticipantStateMutationKind,
    pub property_id: String,
    pub before_value: f64,
    pub delta_value: f64,
    pub after_value: f64,
    pub runtime_state_mutation_applied: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantStateMutationReport {
    pub source_mode: RuntimeParticipantStateMutationSourceMode,
    pub selected_source_mode: RuntimeParticipantStateMutationSourceMode,
    pub selection_allowed: bool,
    pub delta_preview_ready: bool,
    pub before_rows: Vec<RuntimeParticipantStateRow>,
    pub mutation_records: Vec<RuntimeParticipantStateMutationRecord>,
    pub after_rows: Vec<RuntimeParticipantStateRow>,
    pub mutation_record_count: u32,
    pub applied_runtime_delta_total: f64,
    pub runtime_state_mutation_applied: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantStateMutationReplayReport {
    pub replay_count: u32,
    pub replay_deterministic: bool,
    pub reference_report: RuntimeParticipantStateMutationReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeParticipantStateMutationErrorKind {
    DeltaPreviewRejected,
    StateMutationRejected,
    SelectionDenied,
    ScenarioAuthorityRejected,
    ReplayRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeParticipantStateMutationError {
    pub kind: RuntimeParticipantStateMutationErrorKind,
    pub message: String,
}

/// Evaluate runtime participant state mutation with explicit source mode.
pub fn evaluate_runtime_participant_state_mutation(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: RuntimeParticipantStateMutationSourceMode,
    replay_count: u32,
) -> Result<RuntimeParticipantStateMutationReport, RuntimeParticipantStateMutationError> {
    if tick_id.0 == 0 {
        return Err(RuntimeParticipantStateMutationError {
            kind: RuntimeParticipantStateMutationErrorKind::SelectionDenied,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let delta_preview_report = evaluate_semantic_participant_delta_preview(
        scenario,
        tick_id,
        map_to_delta_preview_source_mode(source_mode),
        replay_count,
    )
    .map_err(|e| RuntimeParticipantStateMutationError {
        kind: RuntimeParticipantStateMutationErrorKind::DeltaPreviewRejected,
        message: e.message,
    })?;

    build_state_mutation_report(source_mode, &delta_preview_report)
}

/// Prove Scenario authority is unchanged after runtime participant state mutation evaluation.
pub fn prove_runtime_participant_state_mutation_preserves_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: RuntimeParticipantStateMutationSourceMode,
    replay_count: u32,
) -> Result<bool, RuntimeParticipantStateMutationError> {
    let before =
        scenario_authority_digest(scenario).map_err(|e| RuntimeParticipantStateMutationError {
            kind: RuntimeParticipantStateMutationErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    let _report =
        evaluate_runtime_participant_state_mutation(scenario, tick_id, source_mode, replay_count)?;
    let after =
        scenario_authority_digest(scenario).map_err(|e| RuntimeParticipantStateMutationError {
            kind: RuntimeParticipantStateMutationErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    Ok(before == after)
}

/// Replay runtime participant state mutation and verify deterministic outputs.
pub fn replay_runtime_participant_state_mutation(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: RuntimeParticipantStateMutationSourceMode,
    replay_count: u32,
) -> Result<RuntimeParticipantStateMutationReplayReport, RuntimeParticipantStateMutationError> {
    if !(MIN_RUNTIME_PARTICIPANT_STATE_REPLAY_COUNT..=MAX_RUNTIME_PARTICIPANT_STATE_REPLAY_COUNT)
        .contains(&replay_count)
    {
        return Err(RuntimeParticipantStateMutationError {
            kind: RuntimeParticipantStateMutationErrorKind::ReplayRejected,
            message: format!(
                "replay_count must be in {}..={}",
                MIN_RUNTIME_PARTICIPANT_STATE_REPLAY_COUNT,
                MAX_RUNTIME_PARTICIPANT_STATE_REPLAY_COUNT
            ),
        });
    }

    let reference = evaluate_runtime_participant_state_mutation(scenario, tick_id, source_mode, 1)?;
    let mut replay_deterministic = true;
    for _ in 1..replay_count {
        let replay =
            evaluate_runtime_participant_state_mutation(scenario, tick_id, source_mode, 1)?;
        if replay != reference {
            replay_deterministic = false;
            break;
        }
    }

    Ok(RuntimeParticipantStateMutationReplayReport {
        replay_count,
        replay_deterministic,
        reference_report: reference,
    })
}

fn map_to_delta_preview_source_mode(
    source_mode: RuntimeParticipantStateMutationSourceMode,
) -> ParticipantDeltaPreviewSourceMode {
    match source_mode {
        RuntimeParticipantStateMutationSourceMode::LegacyPlanetChildOwnerSilo => {
            ParticipantDeltaPreviewSourceMode::LegacyPlanetChildOwnerSilo
        }
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable => {
            ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable
        }
    }
}

fn map_from_delta_preview_source_mode(
    source_mode: ParticipantDeltaPreviewSourceMode,
) -> RuntimeParticipantStateMutationSourceMode {
    match source_mode {
        ParticipantDeltaPreviewSourceMode::LegacyPlanetChildOwnerSilo => {
            RuntimeParticipantStateMutationSourceMode::LegacyPlanetChildOwnerSilo
        }
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable => {
            RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable
        }
    }
}

fn build_state_mutation_report(
    source_mode: RuntimeParticipantStateMutationSourceMode,
    delta_preview_report: &SemanticParticipantDeltaPreviewReport,
) -> Result<RuntimeParticipantStateMutationReport, RuntimeParticipantStateMutationError> {
    let delta_preview_ready = delta_preview_report.execution_boundary_ready
        && (delta_preview_report.delta_preview_count > 0
            || delta_preview_report.preview_amount_total > 0);

    let selected_source_mode =
        map_from_delta_preview_source_mode(delta_preview_report.selected_source_mode);

    let selection_allowed = match source_mode {
        RuntimeParticipantStateMutationSourceMode::LegacyPlanetChildOwnerSilo => true,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable => {
            delta_preview_report.selection_allowed
                && delta_preview_ready
                && selected_source_mode
                    == RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable
        }
    };

    let (before_rows, mutation_records, after_rows, applied_runtime_delta_total) =
        apply_delta_previews_to_runtime_state(&delta_preview_report.delta_records)?;

    let mutation_record_count = u32::try_from(mutation_records.len()).map_err(|_| {
        RuntimeParticipantStateMutationError {
            kind: RuntimeParticipantStateMutationErrorKind::StateMutationRejected,
            message: "mutation_record_count exceeds u32".to_string(),
        }
    })?;

    let runtime_state_mutation_applied = mutation_record_count > 0
        && mutation_records
            .iter()
            .all(|r| r.runtime_state_mutation_applied);

    Ok(RuntimeParticipantStateMutationReport {
        source_mode,
        selected_source_mode,
        selection_allowed,
        delta_preview_ready,
        before_rows,
        mutation_records,
        after_rows,
        mutation_record_count,
        applied_runtime_delta_total,
        runtime_state_mutation_applied,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        persistent_history_deferred: true,
    })
}

type StateKey = (u32, String, String, Option<u32>, String);

fn apply_delta_previews_to_runtime_state(
    delta_records: &[ParticipantPropertyDeltaPreviewRecord],
) -> Result<
    (
        Vec<RuntimeParticipantStateRow>,
        Vec<RuntimeParticipantStateMutationRecord>,
        Vec<RuntimeParticipantStateRow>,
        f64,
    ),
    RuntimeParticipantStateMutationError,
> {
    let mut state: BTreeMap<StateKey, f64> = BTreeMap::new();

    for delta in delta_records {
        let key = state_key_from_delta(delta);
        state.entry(key).or_insert(0.0);
    }

    let before_rows = state_map_to_rows(&state);

    let mut mutation_records = Vec::with_capacity(delta_records.len());
    let mut applied_runtime_delta_total = 0.0f64;

    for delta in delta_records {
        let key = state_key_from_delta(delta);
        let before_value = *state.get(&key).unwrap_or(&0.0);
        validate_finite_f64(before_value, "before_value")?;
        validate_finite_f64(delta.preview_delta_value, "delta_value")?;

        let after_value = before_value + delta.preview_delta_value;
        validate_finite_f64(after_value, "after_value")?;

        applied_runtime_delta_total += delta.preview_delta_value;
        validate_finite_f64(applied_runtime_delta_total, "applied_runtime_delta_total")?;

        state.insert(key.clone(), after_value);

        mutation_records.push(RuntimeParticipantStateMutationRecord {
            source_delta_preview_id: delta.source_execution_record_id,
            participant_simthing_id_raw: delta.source_simthing_id_raw,
            owner_ref: delta.owner_ref.clone(),
            resource_key: delta.resource_key.clone(),
            scope_id: delta.scope_id,
            mutation_kind: map_mutation_kind(delta.delta_kind),
            property_id: delta.target_property_id.clone(),
            before_value,
            delta_value: delta.preview_delta_value,
            after_value,
            runtime_state_mutation_applied: true,
            participant_property_mutation_deferred: true,
            scenario_authority_mutation_deferred: true,
            savefile_mutation_deferred: true,
            persistent_history_deferred: true,
        });
    }

    mutation_records.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.scope_id,
            a.participant_simthing_id_raw,
            &a.property_id,
            a.mutation_kind,
            a.source_delta_preview_id,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.scope_id,
                b.participant_simthing_id_raw,
                &b.property_id,
                b.mutation_kind,
                b.source_delta_preview_id,
            ))
    });

    for (index, record) in mutation_records.iter_mut().enumerate() {
        record.source_delta_preview_id =
            u32::try_from(index + 1).map_err(|_| RuntimeParticipantStateMutationError {
                kind: RuntimeParticipantStateMutationErrorKind::StateMutationRejected,
                message: "sorted source_delta_preview_id exceeds u32".to_string(),
            })?;
    }

    let after_rows = state_map_to_rows(&state);

    Ok((
        before_rows,
        mutation_records,
        after_rows,
        applied_runtime_delta_total,
    ))
}

fn state_key_from_delta(delta: &ParticipantPropertyDeltaPreviewRecord) -> StateKey {
    (
        delta.source_simthing_id_raw,
        delta.owner_ref.clone(),
        delta.resource_key.clone(),
        delta.scope_id,
        delta.target_property_id.clone(),
    )
}

fn state_map_to_rows(state: &BTreeMap<StateKey, f64>) -> Vec<RuntimeParticipantStateRow> {
    state
        .iter()
        .map(
            |(
                (participant_simthing_id_raw, owner_ref, resource_key, scope_id, property_id),
                value,
            )| {
                RuntimeParticipantStateRow {
                    participant_simthing_id_raw: *participant_simthing_id_raw,
                    owner_ref: owner_ref.clone(),
                    resource_key: resource_key.clone(),
                    scope_id: *scope_id,
                    property_id: property_id.clone(),
                    value: *value,
                }
            },
        )
        .collect()
}

fn map_mutation_kind(
    delta_kind: ParticipantDeltaPreviewKind,
) -> RuntimeParticipantStateMutationKind {
    match delta_kind {
        ParticipantDeltaPreviewKind::ResourceSatisfiedDelta => {
            RuntimeParticipantStateMutationKind::ApplyResourceSatisfiedDelta
        }
        ParticipantDeltaPreviewKind::ResourceShortfallDelta => {
            RuntimeParticipantStateMutationKind::ApplyResourceShortfallDelta
        }
        ParticipantDeltaPreviewKind::RuntimeAppliedAmountDelta => {
            RuntimeParticipantStateMutationKind::ApplyRuntimeAppliedAmountDelta
        }
    }
}

fn validate_finite_f64(
    value: f64,
    label: &str,
) -> Result<(), RuntimeParticipantStateMutationError> {
    if !value.is_finite() {
        return Err(RuntimeParticipantStateMutationError {
            kind: RuntimeParticipantStateMutationErrorKind::StateMutationRejected,
            message: format!("{label} must be finite"),
        });
    }
    Ok(())
}
