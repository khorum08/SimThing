//! LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 — full recursive runtime report chain for loaded ScenarioSpec sessions.
//!
//! GPU-residency doctrine: runtime report chain stages lower to flat GPU-compatible rows/tables.
//! CPU space is limited to deterministic oracle/reference validation, semantic-side bookkeeping,
//! compile-plan construction, and owner/user-facing reports — not production simulation authority.

use crate::error::SpecError;

use super::loaded_scenario_recursive_rf_runtime::evaluate_loaded_scenario_recursive_rf_runtime_from_json_str;
use super::local_allocation_recursive_rf_source::{
    evaluate_runtime_local_allocation_with_rf_source, LocalAllocationRfSourceMode,
};
use super::local_effect_recursive_rf_source::{
    evaluate_local_effect_application_with_rf_source, LocalEffectRfSourceMode,
};
use super::owner_silo_recursive_rf_source::{
    evaluate_owner_silo_disburse_down_with_rf_source, OwnerSiloRfSourceMode,
};
use super::runtime_participant_property_mutation_boundary::{
    evaluate_runtime_participant_property_mutation_boundary,
    prove_runtime_participant_property_mutation_boundary_preserves_authority,
    RuntimeParticipantPropertyMutationSourceMode,
};
use super::runtime_participant_state_mutation::{
    evaluate_runtime_participant_state_mutation, RuntimeParticipantStateMutationSourceMode,
};
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario_canonical_io::load_scenario_spec_from_json_str;
use super::semantic_effect_execution_boundary::{
    evaluate_semantic_effect_execution_boundary, SemanticEffectExecutionSourceMode,
};
use super::semantic_local_effects_recursive_rf_source::{
    evaluate_semantic_local_effects_with_rf_source, SemanticLocalEffectRfSourceMode,
};
use super::semantic_participant_delta_preview::{
    evaluate_semantic_participant_delta_preview, ParticipantDeltaPreviewSourceMode,
};

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadedScenarioRuntimeReportChainSource {
    LoadedScenarioRecursiveRfRuntime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioRuntimeReportChainStage {
    pub stage_id: String,
    pub ready: bool,
    pub record_count: u32,
    pub report_only: bool,
    pub mutation_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioRuntimeReportChainReport {
    pub source: LoadedScenarioRuntimeReportChainSource,
    pub source_label: String,
    pub scenario_authority_digest: u64,

    pub loaded_session_envelope_ready: bool,
    pub recursive_rf_runtime_ready: bool,

    pub owner_silo_ready: bool,
    pub local_allocation_ready: bool,
    pub local_effects_ready: bool,
    pub semantic_projection_ready: bool,
    pub semantic_execution_records_ready: bool,
    pub semantic_delta_preview_ready: bool,
    pub runtime_participant_state_rows_ready: bool,
    pub runtime_property_view_rows_ready: bool,

    pub stages: Vec<LoadedScenarioRuntimeReportChainStage>,

    pub gpu_compatible_row_table_surface: bool,
    pub cpu_oracle_only: bool,
    pub explicit_runtime_report_mode_only: bool,

    pub scenario_authority_mutation_deferred: bool,
    pub runtime_mutation_deferred: bool,
    pub semantic_execution_deferred: bool,
    pub savefile_persistence_deferred: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Evaluate loaded scenario runtime report chain from canonical JSON, composing #838 recursive RF runtime.
pub fn evaluate_loaded_scenario_runtime_report_chain_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<LoadedScenarioRuntimeReportChainReport, SpecError> {
    let recursive_rf_report =
        evaluate_loaded_scenario_recursive_rf_runtime_from_json_str(source_label, json)?;
    if !recursive_rf_report.recursive_rf_runtime_ready {
        return Err(SpecError::ValidationFailed);
    }

    let (scenario, _) = load_scenario_spec_from_json_str(source_label, json)?;

    let owner_silo_report = evaluate_owner_silo_disburse_down_with_rf_source(
        &scenario,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let allocation_report = evaluate_runtime_local_allocation_with_rf_source(
        &scenario,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let local_effect_report = evaluate_local_effect_application_with_rf_source(
        &scenario,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let semantic_report = evaluate_semantic_local_effects_with_rf_source(
        &scenario,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let execution_report = evaluate_semantic_effect_execution_boundary(
        &scenario,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let delta_preview_report = evaluate_semantic_participant_delta_preview(
        &scenario,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let state_mutation_report = evaluate_runtime_participant_state_mutation(
        &scenario,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let property_view_report = evaluate_runtime_participant_property_mutation_boundary(
        &scenario,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_ONE,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let owner_silo_ready = owner_silo_report.source_selection.selection_allowed
        && owner_silo_report.owner_silo_disburse_down_executed_for_selected_source
        && owner_silo_report
            .selected_disburse_report
            .disburse_result_count
            > 0;

    let local_allocation_ready = allocation_report.source_selection.selection_allowed
        && allocation_report.local_allocation_executed_for_selected_source
        && allocation_report
            .selected_allocation_report
            .allocation_count
            > 0;

    let local_effects_ready = local_effect_report.source_selection.selection_allowed
        && local_effect_report.local_effect_application_executed_for_selected_source
        && local_effect_report
            .selected_application_report
            .application_count
            > 0;

    let semantic_projection_ready = semantic_report.source_selection.selection_allowed
        && semantic_report.semantic_local_effects_projected_for_selected_source
        && semantic_report.selected_semantic_report.output_count > 0;

    let semantic_execution_records_ready = execution_report.selection_allowed
        && execution_report.semantic_projection_ready
        && execution_report.execution_record_count > 0;

    let semantic_delta_preview_ready = delta_preview_report.selection_allowed
        && delta_preview_report.execution_boundary_ready
        && delta_preview_report.delta_preview_count > 0;

    let runtime_participant_state_rows_ready = state_mutation_report.selection_allowed
        && state_mutation_report.delta_preview_ready
        && state_mutation_report.mutation_record_count > 0;

    let runtime_property_view_rows_ready = property_view_report.selection_allowed
        && property_view_report.runtime_state_mutation_ready
        && property_view_report.mutation_record_count > 0;

    let stages = vec![
        stage(
            "loaded_session_envelope",
            recursive_rf_report.loaded_session_envelope_ready,
            u32::from(recursive_rf_report.loaded_session_envelope_ready),
        ),
        stage(
            "recursive_rf_runtime",
            recursive_rf_report.recursive_rf_runtime_ready,
            recursive_rf_report.participant_row_count,
        ),
        stage(
            "owner_silo_disburse_down",
            owner_silo_ready,
            owner_silo_report
                .selected_disburse_report
                .disburse_result_count,
        ),
        stage(
            "local_allocation",
            local_allocation_ready,
            allocation_report
                .selected_allocation_report
                .allocation_count,
        ),
        stage(
            "local_effects",
            local_effects_ready,
            local_effect_report
                .selected_application_report
                .application_count,
        ),
        stage(
            "semantic_projection",
            semantic_projection_ready,
            semantic_report.selected_semantic_report.output_count,
        ),
        stage(
            "semantic_execution_records",
            semantic_execution_records_ready,
            execution_report.execution_record_count,
        ),
        stage(
            "semantic_delta_preview",
            semantic_delta_preview_ready,
            delta_preview_report.delta_preview_count,
        ),
        stage(
            "runtime_participant_state_rows",
            runtime_participant_state_rows_ready,
            state_mutation_report.mutation_record_count,
        ),
        stage(
            "runtime_property_view_rows",
            runtime_property_view_rows_ready,
            property_view_report.mutation_record_count,
        ),
    ];

    Ok(LoadedScenarioRuntimeReportChainReport {
        source: LoadedScenarioRuntimeReportChainSource::LoadedScenarioRecursiveRfRuntime,
        source_label: source_label.to_string(),
        scenario_authority_digest: recursive_rf_report.scenario_authority_digest,
        loaded_session_envelope_ready: recursive_rf_report.loaded_session_envelope_ready,
        recursive_rf_runtime_ready: recursive_rf_report.recursive_rf_runtime_ready,
        owner_silo_ready,
        local_allocation_ready,
        local_effects_ready,
        semantic_projection_ready,
        semantic_execution_records_ready,
        semantic_delta_preview_ready,
        runtime_participant_state_rows_ready,
        runtime_property_view_rows_ready,
        stages,
        gpu_compatible_row_table_surface: recursive_rf_report.gpu_compatible_row_table_surface,
        cpu_oracle_only: true,
        explicit_runtime_report_mode_only: true,
        scenario_authority_mutation_deferred: recursive_rf_report
            .scenario_authority_mutation_deferred
            && owner_silo_report.scenario_authority_mutation_deferred
            && property_view_report.scenario_authority_mutation_deferred,
        runtime_mutation_deferred: recursive_rf_report.runtime_mutation_deferred,
        semantic_execution_deferred: recursive_rf_report.semantic_execution_deferred
            && semantic_report.semantic_execution_deferred,
        savefile_persistence_deferred: recursive_rf_report.savefile_persistence_deferred,
        persistent_history_deferred: recursive_rf_report.persistent_history_deferred,
        studio_ui_wiring_deferred: recursive_rf_report.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: recursive_rf_report.gpu_dispatch_deferred,
    })
}

/// Prove loaded scenario runtime report chain preserves ScenarioSpec authority.
pub fn prove_loaded_scenario_runtime_report_chain_preserves_authority(
    source_label: &str,
    json: &str,
) -> Result<bool, SpecError> {
    let report = evaluate_loaded_scenario_runtime_report_chain_from_json_str(source_label, json)?;
    let (scenario, _) = load_scenario_spec_from_json_str(source_label, json)?;

    let recursive_authority =
        super::loaded_scenario_recursive_rf_runtime::prove_loaded_scenario_recursive_rf_runtime_preserves_authority(
            source_label,
            json,
        )?;

    let property_boundary_authority =
        prove_runtime_participant_property_mutation_boundary_preserves_authority(
            &scenario,
            TICK_ONE,
            RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
            REPLAY_ONE,
        )
        .map_err(|_| SpecError::ValidationFailed)?;

    Ok(recursive_authority
        && property_boundary_authority
        && report.scenario_authority_mutation_deferred
        && report.explicit_runtime_report_mode_only
        && report.cpu_oracle_only
        && report.savefile_persistence_deferred
        && report.studio_ui_wiring_deferred
        && report.gpu_dispatch_deferred)
}

fn stage(stage_id: &str, ready: bool, record_count: u32) -> LoadedScenarioRuntimeReportChainStage {
    LoadedScenarioRuntimeReportChainStage {
        stage_id: stage_id.to_string(),
        ready,
        record_count,
        report_only: true,
        mutation_deferred: true,
    }
}
