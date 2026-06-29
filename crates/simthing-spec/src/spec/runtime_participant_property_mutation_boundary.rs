//! RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 — runtime property view mutation without Scenario persistence.
//!
//! Runtime participant state after-rows apply to an ephemeral runtime participant property view behind
//! explicit source mode. CPU responsibilities: oracle/reference/shadow projection, semantic-side
//! bookkeeping, compile-plan construction, and owner/user-facing report formatting.

use std::collections::BTreeMap;

use super::channel_key::{OwnerRef, ResourceKey};
use super::runtime_participant_state_mutation::{
    evaluate_runtime_participant_state_mutation, RuntimeParticipantStateMutationReport,
    RuntimeParticipantStateMutationSourceMode, RuntimeParticipantStateRow,
};
use super::runtime_tick_history::scenario_authority_digest;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::SimThingScenarioSpec;

pub const MIN_RUNTIME_PARTICIPANT_PROPERTY_MUTATION_REPLAY_COUNT: u32 = 1;
pub const MAX_RUNTIME_PARTICIPANT_PROPERTY_MUTATION_REPLAY_COUNT: u32 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeParticipantPropertyMutationSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveRuntimeStateSelectable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantPropertyViewRow {
    pub participant_simthing_id_raw: u32,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: Option<u32>,
    pub property_id: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantPropertyMutationBoundaryRecord {
    pub source_runtime_state_mutation_index: u32,
    pub participant_simthing_id_raw: u32,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: Option<u32>,
    pub property_id: String,
    pub before_value: f64,
    pub runtime_state_value: f64,
    pub after_value: f64,
    pub runtime_property_view_mutation_applied: bool,
    pub scenario_simthing_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantPropertyMutationBoundaryReport {
    pub source_mode: RuntimeParticipantPropertyMutationSourceMode,
    pub selected_source_mode: RuntimeParticipantPropertyMutationSourceMode,
    pub selection_allowed: bool,
    pub runtime_state_mutation_ready: bool,
    pub before_property_view_rows: Vec<RuntimeParticipantPropertyViewRow>,
    pub mutation_records: Vec<RuntimeParticipantPropertyMutationBoundaryRecord>,
    pub after_property_view_rows: Vec<RuntimeParticipantPropertyViewRow>,
    pub mutation_record_count: u32,
    pub runtime_property_view_mutation_applied: bool,
    pub scenario_simthing_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantPropertyMutationBoundaryReplayReport {
    pub replay_count: u32,
    pub replay_deterministic: bool,
    pub reference_report: RuntimeParticipantPropertyMutationBoundaryReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeParticipantPropertyMutationBoundaryErrorKind {
    StateMutationRejected,
    PropertyViewRejected,
    SelectionDenied,
    ScenarioAuthorityRejected,
    ReplayRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeParticipantPropertyMutationBoundaryError {
    pub kind: RuntimeParticipantPropertyMutationBoundaryErrorKind,
    pub message: String,
}

/// Evaluate runtime participant property mutation boundary with explicit source mode.
pub fn evaluate_runtime_participant_property_mutation_boundary(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: RuntimeParticipantPropertyMutationSourceMode,
    replay_count: u32,
) -> Result<
    RuntimeParticipantPropertyMutationBoundaryReport,
    RuntimeParticipantPropertyMutationBoundaryError,
> {
    if tick_id.0 == 0 {
        return Err(RuntimeParticipantPropertyMutationBoundaryError {
            kind: RuntimeParticipantPropertyMutationBoundaryErrorKind::SelectionDenied,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let state_mutation_report = evaluate_runtime_participant_state_mutation(
        scenario,
        tick_id,
        map_to_state_mutation_source_mode(source_mode),
        replay_count,
    )
    .map_err(|e| RuntimeParticipantPropertyMutationBoundaryError {
        kind: RuntimeParticipantPropertyMutationBoundaryErrorKind::StateMutationRejected,
        message: e.message,
    })?;

    build_property_mutation_boundary_report(source_mode, &state_mutation_report)
}

/// Prove Scenario authority is unchanged after property mutation boundary evaluation.
pub fn prove_runtime_participant_property_mutation_boundary_preserves_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: RuntimeParticipantPropertyMutationSourceMode,
    replay_count: u32,
) -> Result<bool, RuntimeParticipantPropertyMutationBoundaryError> {
    let before = scenario_authority_digest(scenario).map_err(|e| {
        RuntimeParticipantPropertyMutationBoundaryError {
            kind: RuntimeParticipantPropertyMutationBoundaryErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        }
    })?;
    let _report = evaluate_runtime_participant_property_mutation_boundary(
        scenario,
        tick_id,
        source_mode,
        replay_count,
    )?;
    let after = scenario_authority_digest(scenario).map_err(|e| {
        RuntimeParticipantPropertyMutationBoundaryError {
            kind: RuntimeParticipantPropertyMutationBoundaryErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        }
    })?;
    Ok(before == after)
}

/// Replay property mutation boundary evaluation and verify deterministic outputs.
pub fn replay_runtime_participant_property_mutation_boundary(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: RuntimeParticipantPropertyMutationSourceMode,
    replay_count: u32,
) -> Result<
    RuntimeParticipantPropertyMutationBoundaryReplayReport,
    RuntimeParticipantPropertyMutationBoundaryError,
> {
    if !(MIN_RUNTIME_PARTICIPANT_PROPERTY_MUTATION_REPLAY_COUNT
        ..=MAX_RUNTIME_PARTICIPANT_PROPERTY_MUTATION_REPLAY_COUNT)
        .contains(&replay_count)
    {
        return Err(RuntimeParticipantPropertyMutationBoundaryError {
            kind: RuntimeParticipantPropertyMutationBoundaryErrorKind::ReplayRejected,
            message: format!(
                "replay_count must be in {}..={}",
                MIN_RUNTIME_PARTICIPANT_PROPERTY_MUTATION_REPLAY_COUNT,
                MAX_RUNTIME_PARTICIPANT_PROPERTY_MUTATION_REPLAY_COUNT
            ),
        });
    }

    let reference =
        evaluate_runtime_participant_property_mutation_boundary(scenario, tick_id, source_mode, 1)?;
    let mut replay_deterministic = true;
    for _ in 1..replay_count {
        let replay = evaluate_runtime_participant_property_mutation_boundary(
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

    Ok(RuntimeParticipantPropertyMutationBoundaryReplayReport {
        replay_count,
        replay_deterministic,
        reference_report: reference,
    })
}

fn map_to_state_mutation_source_mode(
    source_mode: RuntimeParticipantPropertyMutationSourceMode,
) -> RuntimeParticipantStateMutationSourceMode {
    match source_mode {
        RuntimeParticipantPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo => {
            RuntimeParticipantStateMutationSourceMode::LegacyPlanetChildOwnerSilo
        }
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable => {
            RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable
        }
    }
}

fn map_from_state_mutation_source_mode(
    source_mode: RuntimeParticipantStateMutationSourceMode,
) -> RuntimeParticipantPropertyMutationSourceMode {
    match source_mode {
        RuntimeParticipantStateMutationSourceMode::LegacyPlanetChildOwnerSilo => {
            RuntimeParticipantPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo
        }
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable => {
            RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable
        }
    }
}

fn build_property_mutation_boundary_report(
    source_mode: RuntimeParticipantPropertyMutationSourceMode,
    state_mutation_report: &RuntimeParticipantStateMutationReport,
) -> Result<
    RuntimeParticipantPropertyMutationBoundaryReport,
    RuntimeParticipantPropertyMutationBoundaryError,
> {
    let runtime_state_mutation_ready = state_mutation_report.delta_preview_ready
        && state_mutation_report.runtime_state_mutation_applied;

    let selected_source_mode =
        map_from_state_mutation_source_mode(state_mutation_report.selected_source_mode);

    let selection_allowed = match source_mode {
        RuntimeParticipantPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo => true,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable => {
            state_mutation_report.selection_allowed
                && runtime_state_mutation_ready
                && selected_source_mode
                    == RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable
        }
    };

    let (before_property_view_rows, mutation_records, after_property_view_rows) =
        apply_runtime_state_to_property_view(state_mutation_report)?;

    let mutation_record_count = u32::try_from(mutation_records.len()).map_err(|_| {
        RuntimeParticipantPropertyMutationBoundaryError {
            kind: RuntimeParticipantPropertyMutationBoundaryErrorKind::PropertyViewRejected,
            message: "mutation_record_count exceeds u32".to_string(),
        }
    })?;

    let runtime_property_view_mutation_applied = mutation_record_count > 0
        && mutation_records
            .iter()
            .all(|record| record.runtime_property_view_mutation_applied);

    Ok(RuntimeParticipantPropertyMutationBoundaryReport {
        source_mode,
        selected_source_mode,
        selection_allowed,
        runtime_state_mutation_ready,
        before_property_view_rows,
        mutation_records,
        after_property_view_rows,
        mutation_record_count,
        runtime_property_view_mutation_applied,
        scenario_simthing_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        persistent_history_deferred: true,
    })
}

type PropertyViewKey = (u32, OwnerRef, ResourceKey, Option<u32>, String);

fn apply_runtime_state_to_property_view(
    state_mutation_report: &RuntimeParticipantStateMutationReport,
) -> Result<
    (
        Vec<RuntimeParticipantPropertyViewRow>,
        Vec<RuntimeParticipantPropertyMutationBoundaryRecord>,
        Vec<RuntimeParticipantPropertyViewRow>,
    ),
    RuntimeParticipantPropertyMutationBoundaryError,
> {
    let mut property_view: BTreeMap<PropertyViewKey, f64> = BTreeMap::new();

    for row in &state_mutation_report.before_rows {
        let key = property_view_key_from_state_row(row);
        property_view.entry(key).or_insert(0.0);
    }

    let before_property_view_rows = property_view_map_to_rows(&property_view);

    let mut mutation_records = Vec::with_capacity(state_mutation_report.mutation_records.len());

    for state_record in &state_mutation_report.mutation_records {
        let key = (
            state_record.participant_simthing_id_raw,
            state_record.owner_ref.clone(),
            state_record.resource_key.clone(),
            state_record.scope_id,
            state_record.property_id.clone(),
        );
        let before_value = *property_view.get(&key).unwrap_or(&0.0);
        validate_finite_f64(before_value, "before_value")?;
        validate_finite_f64(state_record.after_value, "runtime_state_value")?;

        let runtime_state_value = state_record.after_value;
        let after_value = runtime_state_value;
        validate_finite_f64(after_value, "after_value")?;

        property_view.insert(key, after_value);

        mutation_records.push(RuntimeParticipantPropertyMutationBoundaryRecord {
            source_runtime_state_mutation_index: state_record.source_delta_preview_id,
            participant_simthing_id_raw: state_record.participant_simthing_id_raw,
            owner_ref: state_record.owner_ref.clone(),
            resource_key: state_record.resource_key.clone(),
            scope_id: state_record.scope_id,
            property_id: state_record.property_id.clone(),
            before_value,
            runtime_state_value,
            after_value,
            runtime_property_view_mutation_applied: true,
            scenario_simthing_property_mutation_deferred: true,
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
            a.source_runtime_state_mutation_index,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.scope_id,
                b.participant_simthing_id_raw,
                &b.property_id,
                b.source_runtime_state_mutation_index,
            ))
    });

    for (index, record) in mutation_records.iter_mut().enumerate() {
        record.source_runtime_state_mutation_index = u32::try_from(index + 1).map_err(|_| {
            RuntimeParticipantPropertyMutationBoundaryError {
                kind: RuntimeParticipantPropertyMutationBoundaryErrorKind::PropertyViewRejected,
                message: "sorted source_runtime_state_mutation_index exceeds u32".to_string(),
            }
        })?;
    }

    let after_property_view_rows =
        state_rows_to_property_view_rows(&state_mutation_report.after_rows);

    Ok((
        before_property_view_rows,
        mutation_records,
        after_property_view_rows,
    ))
}

fn property_view_key_from_state_row(row: &RuntimeParticipantStateRow) -> PropertyViewKey {
    (
        row.participant_simthing_id_raw,
        row.owner_ref.clone(),
        row.resource_key.clone(),
        row.scope_id,
        row.property_id.clone(),
    )
}

fn property_view_map_to_rows(
    property_view: &BTreeMap<PropertyViewKey, f64>,
) -> Vec<RuntimeParticipantPropertyViewRow> {
    property_view
        .iter()
        .map(
            |(
                (participant_simthing_id_raw, owner_ref, resource_key, scope_id, property_id),
                value,
            )| RuntimeParticipantPropertyViewRow {
                participant_simthing_id_raw: *participant_simthing_id_raw,
                owner_ref: owner_ref.clone(),
                resource_key: resource_key.clone(),
                scope_id: *scope_id,
                property_id: property_id.clone(),
                value: *value,
            },
        )
        .collect()
}

fn state_rows_to_property_view_rows(
    rows: &[RuntimeParticipantStateRow],
) -> Vec<RuntimeParticipantPropertyViewRow> {
    rows.iter()
        .map(|row| RuntimeParticipantPropertyViewRow {
            participant_simthing_id_raw: row.participant_simthing_id_raw,
            owner_ref: row.owner_ref.clone(),
            resource_key: row.resource_key.clone(),
            scope_id: row.scope_id,
            property_id: row.property_id.clone(),
            value: row.value,
        })
        .collect()
}

fn validate_finite_f64(
    value: f64,
    label: &str,
) -> Result<(), RuntimeParticipantPropertyMutationBoundaryError> {
    if !value.is_finite() {
        return Err(RuntimeParticipantPropertyMutationBoundaryError {
            kind: RuntimeParticipantPropertyMutationBoundaryErrorKind::PropertyViewRejected,
            message: format!("{label} must be finite"),
        });
    }
    Ok(())
}
