//! LOCAL-EFFECT-APPLICATION-RECURSIVE-RF-SOURCE-0 — recursive RF source mode for local effects.
//!
//! Recursive-source local allocation feeds local participant effect previews and local effect
//! application proof reports behind explicit source mode. CPU responsibilities: oracle/reference/
//! shadow projection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing
//! report formatting.

use super::local_allocation_recursive_rf_source::{
    evaluate_runtime_local_allocation_with_rf_source, LocalAllocationRfSourceMode,
    RuntimeLocalAllocationReport, RuntimeLocalAllocationRfSourceReport,
};
use super::local_effect_application::{
    apply_runtime_local_effect_records, evaluate_runtime_local_effect_application,
    LocalEffectApplicationError, RuntimeLocalEffectApplicationReport,
};
use super::local_participant_effects::{
    evaluate_local_participant_effects, local_participant_effects_from_allocations,
    LocalParticipantEffectsError, LocalParticipantEffectsReport,
};
use super::runtime_tick_history::scenario_authority_digest;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::SimThingScenarioSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalEffectRfSourceMode {
    LegacyPlanetChildOwnerSilo,
    RecursiveLocalAllocationSelectable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalEffectRfSourceSelection {
    pub requested_source_mode: LocalEffectRfSourceMode,
    pub selected_source_mode: LocalEffectRfSourceMode,
    pub selection_allowed: bool,
    pub legacy_default_preserved: bool,
    pub local_allocation_recursive_source_ready: bool,
    pub recursive_allocation_report_available: bool,
    pub recursive_selected_for_local_effect_report_only: bool,
    pub semantic_effect_integration_deferred: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalEffectRecursiveSourceErrorKind {
    LocalAllocationSourceRejected,
    ParticipantEffectsRejected,
    EffectApplicationRejected,
    SelectionDenied,
    ScenarioAuthorityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalEffectRecursiveSourceError {
    pub kind: LocalEffectRecursiveSourceErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalEffectApplicationRfSourceReport {
    pub source_selection: LocalEffectRfSourceSelection,
    pub legacy_local_participant_effects_report: LocalParticipantEffectsReport,
    pub legacy_application_report: RuntimeLocalEffectApplicationReport,
    pub recursive_local_allocation_report: Option<RuntimeLocalAllocationRfSourceReport>,
    pub recursive_local_participant_effects_report: Option<LocalParticipantEffectsReport>,
    pub recursive_application_report: Option<RuntimeLocalEffectApplicationReport>,
    pub selected_application_report: RuntimeLocalEffectApplicationReport,
    pub selected_source_mode: LocalEffectRfSourceMode,
    pub local_participant_effects_executed_for_selected_source: bool,
    pub local_effect_application_executed_for_selected_source: bool,
    pub legacy_default_preserved: bool,
    pub recursive_source_report_only_beyond_local_effects: bool,
    pub semantic_effect_integration_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

/// Convert runtime local allocation report rows into participant effect previews.
pub fn local_participant_effects_from_runtime_local_allocation_report(
    allocation_report: &RuntimeLocalAllocationReport,
) -> Result<LocalParticipantEffectsReport, LocalEffectRecursiveSourceError> {
    local_participant_effects_from_allocations(&allocation_report.states)
        .map_err(map_participant_effects_error)
}

/// Convert participant effect previews into local effect application report.
pub fn local_effect_application_from_participant_effects_report(
    effects_report: &LocalParticipantEffectsReport,
) -> Result<RuntimeLocalEffectApplicationReport, LocalEffectRecursiveSourceError> {
    apply_runtime_local_effect_records(&effects_report.effects)
        .map_err(map_effect_application_error)
}

/// Evaluate local participant effects and application with explicit RF source mode.
pub fn evaluate_local_effect_application_with_rf_source(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: LocalEffectRfSourceMode,
) -> Result<LocalEffectApplicationRfSourceReport, LocalEffectRecursiveSourceError> {
    if tick_id.0 == 0 {
        return Err(LocalEffectRecursiveSourceError {
            kind: LocalEffectRecursiveSourceErrorKind::SelectionDenied,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let legacy_local_participant_effects_report =
        evaluate_local_participant_effects(scenario, tick_id)
            .map_err(map_participant_effects_error)?;
    let legacy_application_report = evaluate_runtime_local_effect_application(scenario, tick_id)
        .map_err(map_effect_application_error)?;

    let allocation_mode = match source_mode {
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable => {
            LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable
        }
    };

    let allocation_report =
        evaluate_runtime_local_allocation_with_rf_source(scenario, allocation_mode).map_err(
            |e| LocalEffectRecursiveSourceError {
                kind: LocalEffectRecursiveSourceErrorKind::LocalAllocationSourceRejected,
                message: e.message,
            },
        )?;

    let recursive_local_allocation_report =
        if source_mode == LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable {
            Some(allocation_report.clone())
        } else {
            None
        };

    let local_allocation_recursive_source_ready = allocation_report
        .source_selection
        .owner_silo_recursive_source_ready;
    let recursive_allocation_report_available = allocation_report
        .recursive_allocation_report
        .as_ref()
        .map(|report| report.allocation_count > 0 || report.allocated_total > 0)
        .unwrap_or(false);

    let selection_allowed = match source_mode {
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => true,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable => {
            local_allocation_recursive_source_ready
                && recursive_allocation_report_available
                && allocation_report.source_selection.selection_allowed
        }
    };

    let selected_source_mode = match source_mode {
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
        }
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable if selection_allowed => {
            LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable
        }
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable => {
            LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
        }
    };

    let reason = match source_mode {
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            "legacy planet-child/owner-silo/local-allocation/local-effect path preserves default"
                .to_string()
        }
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable if selection_allowed => {
            "recursive local allocation feeds local participant effects and application reports"
                .to_string()
        }
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable => {
            if !local_allocation_recursive_source_ready {
                "recursive local effect denied: local allocation recursive source not ready"
                    .to_string()
            } else if !recursive_allocation_report_available {
                "recursive local effect denied: recursive allocation report unavailable".to_string()
            } else {
                "recursive local effect denied: allocation selection gates not satisfied"
                    .to_string()
            }
        }
    };

    let recursive_local_participant_effects_report =
        if source_mode == LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable {
            allocation_report
                .recursive_allocation_report
                .as_ref()
                .map(local_participant_effects_from_runtime_local_allocation_report)
                .transpose()?
        } else {
            None
        };

    let recursive_application_report =
        if source_mode == LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable {
            recursive_local_participant_effects_report
                .as_ref()
                .map(local_effect_application_from_participant_effects_report)
                .transpose()?
        } else {
            None
        };

    let selected_application_report = match selected_source_mode {
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable => recursive_application_report
            .clone()
            .expect("recursive application report required when selected"),
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => legacy_application_report.clone(),
    };

    let recursive_selected_for_local_effect_report_only = selected_source_mode
        == LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable
        && selection_allowed;

    let local_participant_effects_executed_for_selected_source = match selected_source_mode {
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable => {
            recursive_local_participant_effects_report
                .as_ref()
                .map(|report| report.effect_count > 0)
                .unwrap_or(false)
        }
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo => {
            legacy_local_participant_effects_report.effect_count > 0
        }
    };

    let local_effect_application_executed_for_selected_source =
        selected_application_report.application_count > 0
            || selected_application_report.runtime_applied_total > 0;

    Ok(LocalEffectApplicationRfSourceReport {
        source_selection: LocalEffectRfSourceSelection {
            requested_source_mode: source_mode,
            selected_source_mode,
            selection_allowed,
            legacy_default_preserved: true,
            local_allocation_recursive_source_ready,
            recursive_allocation_report_available,
            recursive_selected_for_local_effect_report_only,
            semantic_effect_integration_deferred: true,
            reason,
        },
        legacy_local_participant_effects_report,
        legacy_application_report,
        recursive_local_allocation_report,
        recursive_local_participant_effects_report,
        recursive_application_report,
        selected_application_report: selected_application_report.clone(),
        selected_source_mode,
        local_participant_effects_executed_for_selected_source,
        local_effect_application_executed_for_selected_source,
        legacy_default_preserved: true,
        recursive_source_report_only_beyond_local_effects: true,
        semantic_effect_integration_deferred: true,
        scenario_authority_mutation_deferred: true,
        participant_property_mutation_deferred: true,
        savefile_mutation_deferred: true,
    })
}

/// Prove Scenario authority is unchanged after recursive local effect evaluation.
pub fn prove_local_effect_recursive_source_preserves_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: LocalEffectRfSourceMode,
) -> Result<bool, LocalEffectRecursiveSourceError> {
    let before =
        scenario_authority_digest(scenario).map_err(|e| LocalEffectRecursiveSourceError {
            kind: LocalEffectRecursiveSourceErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    let _report = evaluate_local_effect_application_with_rf_source(scenario, tick_id, source_mode)?;
    let after =
        scenario_authority_digest(scenario).map_err(|e| LocalEffectRecursiveSourceError {
            kind: LocalEffectRecursiveSourceErrorKind::ScenarioAuthorityRejected,
            message: e.message,
        })?;
    Ok(before == after)
}

fn map_participant_effects_error(
    error: LocalParticipantEffectsError,
) -> LocalEffectRecursiveSourceError {
    LocalEffectRecursiveSourceError {
        kind: LocalEffectRecursiveSourceErrorKind::ParticipantEffectsRejected,
        message: error.message,
    }
}

fn map_effect_application_error(
    error: LocalEffectApplicationError,
) -> LocalEffectRecursiveSourceError {
    LocalEffectRecursiveSourceError {
        kind: LocalEffectRecursiveSourceErrorKind::EffectApplicationRejected,
        message: error.message,
    }
}
