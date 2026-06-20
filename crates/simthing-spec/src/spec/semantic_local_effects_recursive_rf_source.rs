//! SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 — recursive RF source mode for semantic local effects.
//!
//! Recursive-source local effect application feeds semantic local effect projection proof reports
//! behind explicit source mode. CPU responsibilities: oracle/reference/shadow projection,
//! semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.

use super::local_effect_recursive_rf_source::{
    evaluate_local_effect_application_with_rf_source, LocalEffectApplicationRfSourceReport,
    LocalEffectRfSourceMode,
};
use super::runtime_tick_history::scenario_authority_digest;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::SimThingScenarioSpec;
use super::semantic_local_effects::{
    evaluate_semantic_local_effects, semantic_local_effects_from_application,
    SemanticLocalEffectError, SemanticLocalEffectReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticLocalEffectRfSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveLocalEffectSelectable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLocalEffectRfSourceSelection {
    pub requested_source_mode: SemanticLocalEffectRfSourceMode,
    pub selected_source_mode: SemanticLocalEffectRfSourceMode,
    pub selection_allowed: bool,
    pub legacy_default_preserved: bool,
    pub local_effect_recursive_source_ready: bool,
    pub recursive_application_report_available: bool,
    pub recursive_selected_for_semantic_projection_only: bool,
    pub semantic_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticLocalEffectRecursiveSourceErrorKind {
    LocalEffectSourceRejected,
    SemanticProjectionRejected,
    SelectionDenied,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLocalEffectRecursiveSourceError {
    pub kind: SemanticLocalEffectRecursiveSourceErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticLocalEffectRfSourceReport {
    pub source_selection: SemanticLocalEffectRfSourceSelection,
    pub legacy_semantic_report: SemanticLocalEffectReport,
    pub recursive_local_effect_report: Option<LocalEffectApplicationRfSourceReport>,
    pub recursive_semantic_report: Option<SemanticLocalEffectReport>,
    pub selected_semantic_report: SemanticLocalEffectReport,
    pub selected_source_mode: SemanticLocalEffectRfSourceMode,
    pub semantic_local_effects_projected_for_selected_source: bool,
    pub legacy_default_preserved: bool,
    pub recursive_source_projection_only: bool,
    pub semantic_execution_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

/// Convert local effect application report rows into semantic local effect projection.
pub fn semantic_local_effects_from_local_effect_application_report(
    application_report: &super::local_effect_application::RuntimeLocalEffectApplicationReport,
) -> Result<SemanticLocalEffectReport, SemanticLocalEffectRecursiveSourceError> {
    semantic_local_effects_from_application(application_report).map_err(map_semantic_error)
}

/// Evaluate semantic local effects with explicit RF source mode.
pub fn evaluate_semantic_local_effects_with_rf_source(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: SemanticLocalEffectRfSourceMode,
    replay_count: u32,
) -> Result<SemanticLocalEffectRfSourceReport, SemanticLocalEffectRecursiveSourceError> {
    if tick_id.0 == 0 {
        return Err(SemanticLocalEffectRecursiveSourceError {
            kind: SemanticLocalEffectRecursiveSourceErrorKind::SelectionDenied,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let legacy_semantic_report = evaluate_semantic_local_effects(scenario, tick_id, replay_count)
        .map_err(map_semantic_error)?;

    let local_effect_mode = match source_mode {
        SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable => {
            LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable
        }
    };

    let local_effect_report =
        evaluate_local_effect_application_with_rf_source(scenario, tick_id, local_effect_mode)
            .map_err(|e| SemanticLocalEffectRecursiveSourceError {
                kind: SemanticLocalEffectRecursiveSourceErrorKind::LocalEffectSourceRejected,
                message: e.message,
            })?;

    let recursive_local_effect_report =
        if source_mode == SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable {
            Some(local_effect_report.clone())
        } else {
            None
        };

    let local_effect_recursive_source_ready = local_effect_report
        .source_selection
        .local_allocation_recursive_source_ready;
    let recursive_application_report_available = local_effect_report
        .recursive_application_report
        .as_ref()
        .map(|report| report.application_count > 0 || report.runtime_applied_total > 0)
        .unwrap_or(false);

    let selection_allowed = match source_mode {
        SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => true,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable => {
            local_effect_recursive_source_ready
                && recursive_application_report_available
                && local_effect_report.source_selection.selection_allowed
        }
    };

    let selected_source_mode = match source_mode {
        SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable if selection_allowed => {
            SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable
        }
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable => {
            SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
        }
    };

    let reason = match source_mode {
        SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            "legacy planet-child/owner-silo/local-allocation/local-effect/semantic path preserves default"
                .to_string()
        }
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable if selection_allowed => {
            "recursive local effect application feeds semantic local effect projection reports"
                .to_string()
        }
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable => {
            if !local_effect_recursive_source_ready {
                "recursive semantic projection denied: local effect recursive source not ready"
                    .to_string()
            } else if !recursive_application_report_available {
                "recursive semantic projection denied: recursive application report unavailable"
                    .to_string()
            } else {
                "recursive semantic projection denied: local effect selection gates not satisfied"
                    .to_string()
            }
        }
    };

    let recursive_semantic_report =
        if source_mode == SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable {
            local_effect_report
                .recursive_application_report
                .as_ref()
                .map(semantic_local_effects_from_local_effect_application_report)
                .transpose()?
        } else {
            None
        };

    let selected_semantic_report = match selected_source_mode {
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable => {
            recursive_semantic_report
                .clone()
                .expect("recursive semantic report required when selected")
        }
        SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            legacy_semantic_report.clone()
        }
    };

    let recursive_selected_for_semantic_projection_only = selected_source_mode
        == SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable
        && selection_allowed;

    let semantic_local_effects_projected_for_selected_source =
        selected_semantic_report.output_count > 0
            || selected_semantic_report.runtime_applied_total > 0;

    Ok(SemanticLocalEffectRfSourceReport {
        source_selection: SemanticLocalEffectRfSourceSelection {
            requested_source_mode: source_mode,
            selected_source_mode,
            selection_allowed,
            legacy_default_preserved: true,
            local_effect_recursive_source_ready,
            recursive_application_report_available,
            recursive_selected_for_semantic_projection_only,
            semantic_execution_deferred: true,
            participant_property_mutation_deferred: true,
            reason,
        },
        legacy_semantic_report,
        recursive_local_effect_report,
        recursive_semantic_report,
        selected_semantic_report: selected_semantic_report.clone(),
        selected_source_mode,
        semantic_local_effects_projected_for_selected_source,
        legacy_default_preserved: true,
        recursive_source_projection_only: true,
        semantic_execution_deferred: true,
        scenario_authority_mutation_deferred: true,
        participant_property_mutation_deferred: true,
        savefile_mutation_deferred: true,
    })
}

/// Prove Scenario authority is unchanged after recursive semantic local effect evaluation.
pub fn prove_semantic_local_effects_recursive_source_preserves_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: SemanticLocalEffectRfSourceMode,
    replay_count: u32,
) -> Result<bool, SemanticLocalEffectRecursiveSourceError> {
    let before = scenario_authority_digest(scenario).map_err(|e| {
        SemanticLocalEffectRecursiveSourceError {
            kind: SemanticLocalEffectRecursiveSourceErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        }
    })?;
    let _report = evaluate_semantic_local_effects_with_rf_source(
        scenario,
        tick_id,
        source_mode,
        replay_count,
    )?;
    let after = scenario_authority_digest(scenario).map_err(|e| {
        SemanticLocalEffectRecursiveSourceError {
            kind: SemanticLocalEffectRecursiveSourceErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        }
    })?;
    Ok(before == after)
}

fn map_semantic_error(error: SemanticLocalEffectError) -> SemanticLocalEffectRecursiveSourceError {
    SemanticLocalEffectRecursiveSourceError {
        kind: SemanticLocalEffectRecursiveSourceErrorKind::SemanticProjectionRejected,
        message: error.message,
    }
}
