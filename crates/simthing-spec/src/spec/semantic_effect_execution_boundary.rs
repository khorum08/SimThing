//! SEMANTIC-EFFECT-EXECUTION-BOUNDARY-0 — runtime semantic execution records without mutation.
//!
//! Recursive-source semantic local effects convert into deterministic runtime execution records
//! behind explicit source mode. CPU responsibilities: oracle/reference/shadow projection,
//! semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.

use super::channel_key::{OwnerRef, ResourceKey};
use super::runtime_tick_history::scenario_authority_digest;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::SimThingScenarioSpec;
use super::semantic_local_effects::SemanticLocalEffectKind;
use super::semantic_local_effects_recursive_rf_source::{
    evaluate_semantic_local_effects_with_rf_source, SemanticLocalEffectRfSourceMode,
    SemanticLocalEffectRfSourceReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticEffectExecutionSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveSemanticLocalEffectsSelectable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticEffectExecutionKind {
    ResourceSatisfiedExecution,
    ResourceShortfallExecution,
    RuntimeAppliedAmountExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEffectExecutionRecord {
    pub source_semantic_effect_id: u32,
    pub source_simthing_id_raw: u32,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: Option<u32>,
    pub execution_kind: SemanticEffectExecutionKind,
    pub amount: u32,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticEffectExecutionBoundaryReport {
    pub source_mode: SemanticEffectExecutionSourceMode,
    pub selected_source_mode: SemanticEffectExecutionSourceMode,
    pub selection_allowed: bool,
    pub legacy_default_preserved: bool,
    pub semantic_projection_ready: bool,
    pub execution_record_count: u32,
    pub resource_satisfied_execution_count: u32,
    pub resource_shortfall_execution_count: u32,
    pub runtime_applied_execution_count: u32,
    pub executed_amount_total: u32,
    pub execution_records: Vec<SemanticEffectExecutionRecord>,
    pub recursive_semantic_report_available: bool,
    pub recursive_execution_report_only: bool,
    pub semantic_execution_boundary_proven: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticEffectExecutionBoundaryErrorKind {
    SemanticProjectionRejected,
    ExecutionRecordRejected,
    SelectionDenied,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEffectExecutionBoundaryError {
    pub kind: SemanticEffectExecutionBoundaryErrorKind,
    pub message: String,
}

/// Evaluate semantic effect execution boundary with explicit RF source mode.
pub fn evaluate_semantic_effect_execution_boundary(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: SemanticEffectExecutionSourceMode,
    replay_count: u32,
) -> Result<SemanticEffectExecutionBoundaryReport, SemanticEffectExecutionBoundaryError> {
    if tick_id.0 == 0 {
        return Err(SemanticEffectExecutionBoundaryError {
            kind: SemanticEffectExecutionBoundaryErrorKind::SelectionDenied,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let semantic_rf_mode = match source_mode {
        SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo => {
            SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable => {
            SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable
        }
    };

    let semantic_report = evaluate_semantic_local_effects_with_rf_source(
        scenario,
        tick_id,
        semantic_rf_mode,
        replay_count,
    )
    .map_err(|e| SemanticEffectExecutionBoundaryError {
        kind: SemanticEffectExecutionBoundaryErrorKind::SemanticProjectionRejected,
        message: e.message,
    })?;

    build_execution_boundary_report(source_mode, &semantic_report)
}

/// Prove Scenario authority is unchanged after semantic execution boundary evaluation.
pub fn prove_semantic_effect_execution_boundary_preserves_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: SemanticEffectExecutionSourceMode,
    replay_count: u32,
) -> Result<bool, SemanticEffectExecutionBoundaryError> {
    let before =
        scenario_authority_digest(scenario).map_err(|e| SemanticEffectExecutionBoundaryError {
            kind: SemanticEffectExecutionBoundaryErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    let _report =
        evaluate_semantic_effect_execution_boundary(scenario, tick_id, source_mode, replay_count)?;
    let after =
        scenario_authority_digest(scenario).map_err(|e| SemanticEffectExecutionBoundaryError {
            kind: SemanticEffectExecutionBoundaryErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    Ok(before == after)
}

fn build_execution_boundary_report(
    source_mode: SemanticEffectExecutionSourceMode,
    semantic_report: &SemanticLocalEffectRfSourceReport,
) -> Result<SemanticEffectExecutionBoundaryReport, SemanticEffectExecutionBoundaryError> {
    let semantic_projection_ready =
        semantic_report.semantic_local_effects_projected_for_selected_source;
    let recursive_semantic_report_available = semantic_report
        .recursive_semantic_report
        .as_ref()
        .map(|report| report.output_count > 0 || report.runtime_applied_total > 0)
        .unwrap_or(false);

    let selection_allowed = match source_mode {
        SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo => true,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable => {
            semantic_report.source_selection.selection_allowed
                && semantic_report
                    .source_selection
                    .local_effect_recursive_source_ready
                && recursive_semantic_report_available
        }
    };

    let selected_source_mode = match source_mode {
        SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo => {
            SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo
        }
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable
            if selection_allowed =>
        {
            SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable
        }
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable => {
            SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo
        }
    };

    let selected_semantic_report = match selected_source_mode {
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable => {
            semantic_report
                .recursive_semantic_report
                .as_ref()
                .expect("recursive semantic report required when selected")
        }
        SemanticEffectExecutionSourceMode::LegacyPlanetChildOwnerSilo => {
            &semantic_report.legacy_semantic_report
        }
    };

    let execution_records =
        semantic_outputs_to_execution_records(&selected_semantic_report.outputs)?;

    let resource_satisfied_execution_count = execution_records
        .iter()
        .filter(|record| {
            record.execution_kind == SemanticEffectExecutionKind::ResourceSatisfiedExecution
        })
        .count() as u32;
    let resource_shortfall_execution_count = execution_records
        .iter()
        .filter(|record| {
            record.execution_kind == SemanticEffectExecutionKind::ResourceShortfallExecution
        })
        .count() as u32;
    let runtime_applied_execution_count = execution_records
        .iter()
        .filter(|record| {
            record.execution_kind == SemanticEffectExecutionKind::RuntimeAppliedAmountExecution
        })
        .count() as u32;
    let executed_amount_total = execution_records
        .iter()
        .map(|record| record.amount)
        .try_fold(0u32, |acc, amount| {
            acc.checked_add(amount)
                .ok_or_else(|| SemanticEffectExecutionBoundaryError {
                    kind: SemanticEffectExecutionBoundaryErrorKind::ExecutionRecordRejected,
                    message: "executed_amount_total overflow".to_string(),
                })
        })?;

    let execution_record_count = u32::try_from(execution_records.len()).map_err(|_| {
        SemanticEffectExecutionBoundaryError {
            kind: SemanticEffectExecutionBoundaryErrorKind::ExecutionRecordRejected,
            message: "execution_record_count exceeds u32".to_string(),
        }
    })?;

    let semantic_execution_boundary_proven =
        execution_record_count > 0 || selected_semantic_report.runtime_applied_total > 0;

    Ok(SemanticEffectExecutionBoundaryReport {
        source_mode,
        selected_source_mode,
        selection_allowed,
        legacy_default_preserved: true,
        semantic_projection_ready,
        execution_record_count,
        resource_satisfied_execution_count,
        resource_shortfall_execution_count,
        runtime_applied_execution_count,
        executed_amount_total,
        execution_records,
        recursive_semantic_report_available,
        recursive_execution_report_only: true,
        semantic_execution_boundary_proven,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        persistent_history_deferred: true,
    })
}

fn semantic_outputs_to_execution_records(
    outputs: &[super::semantic_local_effects::SemanticLocalEffectOutput],
) -> Result<Vec<SemanticEffectExecutionRecord>, SemanticEffectExecutionBoundaryError> {
    let mut records = Vec::with_capacity(outputs.len());
    for (index, output) in outputs.iter().enumerate() {
        if output.source_simthing_id_raw == 0 {
            return Err(SemanticEffectExecutionBoundaryError {
                kind: SemanticEffectExecutionBoundaryErrorKind::ExecutionRecordRejected,
                message: "semantic output requires non-zero source_simthing_id_raw".to_string(),
            });
        }
        records.push(SemanticEffectExecutionRecord {
            source_semantic_effect_id: u32::try_from(index + 1).map_err(|_| {
                SemanticEffectExecutionBoundaryError {
                    kind: SemanticEffectExecutionBoundaryErrorKind::ExecutionRecordRejected,
                    message: "source_semantic_effect_id exceeds u32".to_string(),
                }
            })?,
            source_simthing_id_raw: output.source_simthing_id_raw,
            owner_ref: output.owner_ref.clone(),
            resource_key: output.resource_key.clone(),
            scope_id: parse_scope_id(output.scope_id.as_str()),
            execution_kind: map_execution_kind(output.effect_kind),
            amount: output.amount,
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
            a.execution_kind,
            a.source_semantic_effect_id,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                b.scope_id,
                b.source_simthing_id_raw,
                b.execution_kind,
                b.source_semantic_effect_id,
            ))
    });

    for (index, record) in records.iter_mut().enumerate() {
        record.source_semantic_effect_id =
            u32::try_from(index + 1).map_err(|_| SemanticEffectExecutionBoundaryError {
                kind: SemanticEffectExecutionBoundaryErrorKind::ExecutionRecordRejected,
                message: "sorted source_semantic_effect_id exceeds u32".to_string(),
            })?;
    }

    Ok(records)
}

fn map_execution_kind(kind: SemanticLocalEffectKind) -> SemanticEffectExecutionKind {
    match kind {
        SemanticLocalEffectKind::ResourceSatisfied => {
            SemanticEffectExecutionKind::ResourceSatisfiedExecution
        }
        SemanticLocalEffectKind::ResourceShortfall => {
            SemanticEffectExecutionKind::ResourceShortfallExecution
        }
        SemanticLocalEffectKind::RuntimeAppliedAmount => {
            SemanticEffectExecutionKind::RuntimeAppliedAmountExecution
        }
    }
}

fn parse_scope_id(scope_id: &str) -> Option<u32> {
    scope_id.parse::<u32>().ok()
}
