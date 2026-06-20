//! SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 — runtime participant property delta previews without mutation.
//!
//! Semantic execution records convert into deterministic runtime-only participant property delta
//! preview records behind explicit source mode. CPU responsibilities: oracle/reference/shadow
//! projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report
//! formatting.

use super::runtime_tick_history::scenario_authority_digest;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::SimThingScenarioSpec;
use super::semantic_effect_execution_boundary::{
    evaluate_semantic_effect_execution_boundary, SemanticEffectExecutionBoundaryReport,
    SemanticEffectExecutionKind, SemanticEffectExecutionRecord, SemanticEffectExecutionSourceMode,
};

pub const RUNTIME_PREVIEW_APPLIED_PROPERTY_ID: &str = "runtime.preview.applied";
pub const RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID: &str = "runtime.preview.satisfied";
pub const RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID: &str = "runtime.preview.shortfall";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticipantDeltaPreviewSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveSemanticExecutionSelectable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParticipantDeltaPreviewKind {
    ResourceSatisfiedDelta,
    ResourceShortfallDelta,
    RuntimeAppliedAmountDelta,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParticipantPropertyDeltaPreviewRecord {
    pub source_execution_record_id: u32,
    pub source_simthing_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub scope_id: Option<u32>,
    pub delta_kind: ParticipantDeltaPreviewKind,
    pub amount: u32,
    pub target_property_id: String,
    pub preview_delta_value: f64,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticParticipantDeltaPreviewReport {
    pub source_mode: ParticipantDeltaPreviewSourceMode,
    pub selected_source_mode: ParticipantDeltaPreviewSourceMode,
    pub selection_allowed: bool,
    pub execution_boundary_ready: bool,
    pub delta_preview_count: u32,
    pub resource_satisfied_delta_count: u32,
    pub resource_shortfall_delta_count: u32,
    pub runtime_applied_delta_count: u32,
    pub preview_amount_total: u32,
    pub delta_records: Vec<ParticipantPropertyDeltaPreviewRecord>,
    pub runtime_delta_preview_only: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticParticipantDeltaPreviewErrorKind {
    ExecutionBoundaryRejected,
    DeltaPreviewRejected,
    SelectionDenied,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticParticipantDeltaPreviewError {
    pub kind: SemanticParticipantDeltaPreviewErrorKind,
    pub message: String,
}

/// Evaluate participant property delta preview with explicit source mode.
pub fn evaluate_semantic_participant_delta_preview(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: ParticipantDeltaPreviewSourceMode,
    replay_count: u32,
) -> Result<SemanticParticipantDeltaPreviewReport, SemanticParticipantDeltaPreviewError> {
    if tick_id.0 == 0 {
        return Err(SemanticParticipantDeltaPreviewError {
            kind: SemanticParticipantDeltaPreviewErrorKind::SelectionDenied,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let execution_report = evaluate_semantic_effect_execution_boundary(
        scenario,
        tick_id,
        map_to_execution_source_mode(source_mode),
        replay_count,
    )
    .map_err(|e| SemanticParticipantDeltaPreviewError {
        kind: SemanticParticipantDeltaPreviewErrorKind::ExecutionBoundaryRejected,
        message: e.message,
    })?;

    build_delta_preview_report(source_mode, &execution_report)
}

/// Prove Scenario authority is unchanged after participant delta preview evaluation.
pub fn prove_semantic_participant_delta_preview_preserves_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: ParticipantDeltaPreviewSourceMode,
    replay_count: u32,
) -> Result<bool, SemanticParticipantDeltaPreviewError> {
    let before =
        scenario_authority_digest(scenario).map_err(|e| SemanticParticipantDeltaPreviewError {
            kind: SemanticParticipantDeltaPreviewErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    let _report =
        evaluate_semantic_participant_delta_preview(scenario, tick_id, source_mode, replay_count)?;
    let after =
        scenario_authority_digest(scenario).map_err(|e| SemanticParticipantDeltaPreviewError {
            kind: SemanticParticipantDeltaPreviewErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    Ok(before == after)
}

fn map_to_execution_source_mode(
    source_mode: ParticipantDeltaPreviewSourceMode,
) -> SemanticEffectExecutionSourceMode {
    match source_mode {
        ParticipantDeltaPreviewSourceMode::LegacyPlanetChildOwnerSilo => {
            SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo
        }
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable => {
            SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable
        }
    }
}

fn map_from_execution_source_mode(
    source_mode: SemanticEffectExecutionSourceMode,
) -> ParticipantDeltaPreviewSourceMode {
    match source_mode {
        SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo => {
            ParticipantDeltaPreviewSourceMode::LegacyPlanetChildOwnerSilo
        }
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable => {
            ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable
        }
    }
}

fn build_delta_preview_report(
    source_mode: ParticipantDeltaPreviewSourceMode,
    execution_report: &SemanticEffectExecutionBoundaryReport,
) -> Result<SemanticParticipantDeltaPreviewReport, SemanticParticipantDeltaPreviewError> {
    let execution_boundary_ready = execution_report.semantic_projection_ready
        && execution_report.semantic_execution_boundary_proven;

    let selected_source_mode =
        map_from_execution_source_mode(execution_report.selected_source_mode);

    let selection_allowed = match source_mode {
        ParticipantDeltaPreviewSourceMode::LegacyPlanetChildOwnerSilo => true,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable => {
            execution_report.selection_allowed
                && execution_boundary_ready
                && selected_source_mode
                    == ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable
        }
    };

    let delta_records = execution_records_to_delta_previews(&execution_report.execution_records)?;

    let resource_satisfied_delta_count = delta_records
        .iter()
        .filter(|record| record.delta_kind == ParticipantDeltaPreviewKind::ResourceSatisfiedDelta)
        .count() as u32;
    let resource_shortfall_delta_count = delta_records
        .iter()
        .filter(|record| record.delta_kind == ParticipantDeltaPreviewKind::ResourceShortfallDelta)
        .count() as u32;
    let runtime_applied_delta_count = delta_records
        .iter()
        .filter(|record| {
            record.delta_kind == ParticipantDeltaPreviewKind::RuntimeAppliedAmountDelta
        })
        .count() as u32;
    let preview_amount_total =
        delta_records
            .iter()
            .map(|record| record.amount)
            .try_fold(0u32, |acc, amount| {
                acc.checked_add(amount)
                    .ok_or_else(|| SemanticParticipantDeltaPreviewError {
                        kind: SemanticParticipantDeltaPreviewErrorKind::DeltaPreviewRejected,
                        message: "preview_amount_total overflow".to_string(),
                    })
            })?;

    let delta_preview_count =
        u32::try_from(delta_records.len()).map_err(|_| SemanticParticipantDeltaPreviewError {
            kind: SemanticParticipantDeltaPreviewErrorKind::DeltaPreviewRejected,
            message: "delta_preview_count exceeds u32".to_string(),
        })?;

    Ok(SemanticParticipantDeltaPreviewReport {
        source_mode,
        selected_source_mode,
        selection_allowed,
        execution_boundary_ready,
        delta_preview_count,
        resource_satisfied_delta_count,
        resource_shortfall_delta_count,
        runtime_applied_delta_count,
        preview_amount_total,
        delta_records,
        runtime_delta_preview_only: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        persistent_history_deferred: true,
    })
}

fn execution_records_to_delta_previews(
    execution_records: &[SemanticEffectExecutionRecord],
) -> Result<Vec<ParticipantPropertyDeltaPreviewRecord>, SemanticParticipantDeltaPreviewError> {
    let mut records = Vec::with_capacity(execution_records.len());
    for execution_record in execution_records {
        if execution_record.source_simthing_id_raw == 0 {
            return Err(SemanticParticipantDeltaPreviewError {
                kind: SemanticParticipantDeltaPreviewErrorKind::DeltaPreviewRejected,
                message: "execution record requires non-zero source_simthing_id_raw".to_string(),
            });
        }
        let (delta_kind, target_property_id) =
            map_delta_kind_and_target(execution_record.execution_kind);
        records.push(ParticipantPropertyDeltaPreviewRecord {
            source_execution_record_id: execution_record.source_semantic_effect_id,
            source_simthing_id_raw: execution_record.source_simthing_id_raw,
            owner_ref: execution_record.owner_ref.clone(),
            resource_key: execution_record.resource_key.clone(),
            scope_id: execution_record.scope_id,
            delta_kind,
            amount: execution_record.amount,
            target_property_id: target_property_id.to_string(),
            preview_delta_value: f64::from(execution_record.amount),
            participant_property_mutation_deferred: true,
            scenario_authority_mutation_deferred: true,
            savefile_mutation_deferred: true,
        });
    }

    records.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            a.scope_id,
            a.source_simthing_id_raw,
            a.delta_kind,
            a.source_execution_record_id,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.scope_id,
                b.source_simthing_id_raw,
                b.delta_kind,
                b.source_execution_record_id,
            ))
    });

    for (index, record) in records.iter_mut().enumerate() {
        record.source_execution_record_id =
            u32::try_from(index + 1).map_err(|_| SemanticParticipantDeltaPreviewError {
                kind: SemanticParticipantDeltaPreviewErrorKind::DeltaPreviewRejected,
                message: "sorted source_execution_record_id exceeds u32".to_string(),
            })?;
    }

    Ok(records)
}

fn map_delta_kind_and_target(
    execution_kind: SemanticEffectExecutionKind,
) -> (ParticipantDeltaPreviewKind, &'static str) {
    match execution_kind {
        SemanticEffectExecutionKind::ResourceSatisfiedExecution => (
            ParticipantDeltaPreviewKind::ResourceSatisfiedDelta,
            RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID,
        ),
        SemanticEffectExecutionKind::ResourceShortfallExecution => (
            ParticipantDeltaPreviewKind::ResourceShortfallDelta,
            RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID,
        ),
        SemanticEffectExecutionKind::RuntimeAppliedAmountExecution => (
            ParticipantDeltaPreviewKind::RuntimeAppliedAmountDelta,
            RUNTIME_PREVIEW_APPLIED_PROPERTY_ID,
        ),
    }
}
