//! STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 — Studio UI adapter for loaded scenario runtime candidate save/reopen.
//!
//! Presentation/command surface only. ScenarioSpec remains authority; UI state, Bevy ECS, runtime
//! reports, and GPU buffers are explicitly non-authoritative.

use std::fs;
use std::path::{Path, PathBuf};

use simthing_driver::{
    compile_loaded_scenario_runtime_report_chain_plan_from_json_str,
    compile_scenario_candidate_save_reopen_plan_from_json_str,
};
use simthing_spec::{
    evaluate_loaded_scenario_studio_session_envelope_from_json_str,
    evaluate_scenario_stead_map_roundtrip_from_json_str, load_scenario_spec_from_json_str,
    save_scenario_spec_to_canonical_json, write_candidate_scenario_canonical_json_atomic,
    SpecError,
};

/// UI-visible loaded scenario runtime/candidate save-reopen status (presentation only).
#[derive(Debug, Clone, PartialEq)]
pub struct StudioScenarioRuntimeSaveLoadStatus {
    pub loaded_scenario_digest: Option<u64>,
    pub stead_validation_ready: bool,
    pub recursive_rf_runtime_ready: bool,
    pub runtime_report_chain_ready: bool,
    pub candidate_ready: bool,
    pub candidate_digest: Option<u64>,
    pub candidate_save_ready: bool,
    pub candidate_reopen_ready: bool,
    pub last_message: Option<String>,
}

/// Explicit non-authority boundary flags for Studio runtime save/load UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StudioScenarioRuntimeSaveLoadNonAuthorityBoundary {
    pub ui_state_is_authority: bool,
    pub bevy_state_is_authority: bool,
    pub runtime_reports_are_authority: bool,
    pub gpu_buffers_are_authority: bool,
    pub persistent_history_deferred: bool,
    pub gpu_dispatch_deferred: bool,
    pub canonical_scenario_json_only: bool,
    pub no_distinct_savefile_format: bool,
}

impl Default for StudioScenarioRuntimeSaveLoadNonAuthorityBoundary {
    fn default() -> Self {
        Self {
            ui_state_is_authority: false,
            bevy_state_is_authority: false,
            runtime_reports_are_authority: false,
            gpu_buffers_are_authority: false,
            persistent_history_deferred: true,
            gpu_dispatch_deferred: true,
            canonical_scenario_json_only: true,
            no_distinct_savefile_format: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioSaveCandidateResult {
    pub attempted: bool,
    pub saved: bool,
    pub output_path: Option<PathBuf>,
    pub target_existed: bool,
    pub create_new_policy: bool,
    pub existing_target_preserved: bool,
    pub candidate_digest: Option<u64>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioReopenCandidateResult {
    pub attempted: bool,
    pub reopened: bool,
    pub input_path: Option<PathBuf>,
    pub reopened_digest: Option<u64>,
    pub stead_validation_ready: bool,
    pub projection_rebuild_ready: bool,
    pub message: String,
}

/// Build UI-visible runtime/candidate status from loaded canonical ScenarioSpec JSON.
pub fn build_studio_scenario_runtime_saveload_status_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<StudioScenarioRuntimeSaveLoadStatus, SpecError> {
    let (_, load_report) = load_scenario_spec_from_json_str(source_label, json)?;
    let envelope =
        evaluate_loaded_scenario_studio_session_envelope_from_json_str(source_label, json)?;
    let runtime_chain_plan =
        compile_loaded_scenario_runtime_report_chain_plan_from_json_str(source_label, json)?;
    let save_reopen_plan =
        compile_scenario_candidate_save_reopen_plan_from_json_str(source_label, json)?;

    let candidate_ready = save_reopen_plan
        .candidate_from_runtime_plan
        .candidate_scenario_spec_ready;
    let candidate_digest = if candidate_ready {
        Some(
            save_reopen_plan
                .save_reopen_report
                .candidate_authority_digest_before_save,
        )
    } else {
        None
    };

    Ok(StudioScenarioRuntimeSaveLoadStatus {
        loaded_scenario_digest: Some(load_report.authority_digest),
        stead_validation_ready: envelope.authority.stead_map_roundtrip_ready
            && envelope.authority.stead_ids_stable
            && envelope.authority.links_stable
            && envelope.authority.spatial_tree_stable,
        recursive_rf_runtime_ready: runtime_chain_plan
            .recursive_rf_runtime_plan
            .recursive_rf_runtime_ready,
        runtime_report_chain_ready: runtime_chain_plan.full_chain_ready,
        candidate_ready,
        candidate_digest,
        candidate_save_ready: candidate_ready
            && save_reopen_plan.candidate_save_reopen_ready
            && save_reopen_plan.canonical_scenario_json_only,
        candidate_reopen_ready: save_reopen_plan.candidate_save_reopen_ready,
        last_message: None,
    })
}

/// Save candidate ScenarioSpec using hardened create-new canonical JSON writer (#845).
pub fn save_candidate_scenario_for_studio_create_new(
    source_label: &str,
    json: &str,
    output_path: &Path,
) -> Result<StudioSaveCandidateResult, SpecError> {
    let save_reopen_plan =
        compile_scenario_candidate_save_reopen_plan_from_json_str(source_label, json)?;
    let candidate_digest = Some(
        save_reopen_plan
            .save_reopen_report
            .candidate_authority_digest_before_save,
    );

    if !save_reopen_plan
        .candidate_from_runtime_plan
        .candidate_scenario_spec_ready
    {
        return Ok(StudioSaveCandidateResult {
            attempted: true,
            saved: false,
            output_path: Some(output_path.to_path_buf()),
            target_existed: output_path.exists(),
            create_new_policy: true,
            existing_target_preserved: true,
            candidate_digest,
            message: "Save Candidate failed: candidate ScenarioSpec not ready".into(),
        });
    }

    if output_path.exists() {
        return Ok(StudioSaveCandidateResult {
            attempted: true,
            saved: false,
            output_path: Some(output_path.to_path_buf()),
            target_existed: true,
            create_new_policy: true,
            existing_target_preserved: true,
            candidate_digest,
            message: format!(
                "Save Candidate refused: target already exists at {} (create-new policy; existing file preserved)",
                output_path.display()
            ),
        });
    }

    let canonical_json = save_reopen_plan
        .save_reopen_report
        .save_report
        .canonical_json
        .clone();
    match write_candidate_scenario_canonical_json_atomic(&canonical_json, output_path) {
        Ok(()) => Ok(StudioSaveCandidateResult {
            attempted: true,
            saved: true,
            output_path: Some(output_path.to_path_buf()),
            target_existed: false,
            create_new_policy: true,
            existing_target_preserved: true,
            candidate_digest,
            message: format!(
                "Save Candidate succeeded: canonical ScenarioSpec JSON written to {}",
                output_path.display()
            ),
        }),
        Err(_) => Ok(StudioSaveCandidateResult {
            attempted: true,
            saved: false,
            output_path: Some(output_path.to_path_buf()),
            target_existed: false,
            create_new_policy: true,
            existing_target_preserved: !output_path.exists(),
            candidate_digest,
            message: format!(
                "Save Candidate failed: could not write canonical JSON to {}",
                output_path.display()
            ),
        }),
    }
}

/// Reopen candidate ScenarioSpec from canonical JSON path and validate readiness.
pub fn reopen_candidate_scenario_for_studio(
    input_path: &Path,
) -> Result<StudioReopenCandidateResult, SpecError> {
    let source_label = input_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("studio_candidate_reopen");
    let json = fs::read_to_string(input_path).map_err(|_| SpecError::ValidationFailed)?;

    let (_, load_report) = load_scenario_spec_from_json_str(source_label, &json)?;
    let stead_report = evaluate_scenario_stead_map_roundtrip_from_json_str(source_label, &json)?;
    let envelope =
        evaluate_loaded_scenario_studio_session_envelope_from_json_str(source_label, &json)?;

    let stead_validation_ready = stead_report.stead_ids_stable
        && stead_report.links_stable
        && stead_report.spatial_tree_stable
        && stead_report.rf_metadata_stable
        && stead_report.digest_stable;
    let projection_rebuild_ready = envelope.authority.studio_projection_rebuild_ready;
    let reopened = load_report.loaded
        && load_report.ingestion_ready
        && stead_validation_ready
        && projection_rebuild_ready;

    let message = if reopened {
        format!(
            "Reopen Candidate succeeded: digest {} with STEAD/projection readiness",
            load_report.authority_digest
        )
    } else {
        "Reopen Candidate failed: canonical load or validation/projection readiness incomplete"
            .into()
    };

    Ok(StudioReopenCandidateResult {
        attempted: true,
        reopened,
        input_path: Some(input_path.to_path_buf()),
        reopened_digest: Some(load_report.authority_digest),
        stead_validation_ready,
        projection_rebuild_ready,
        message,
    })
}

/// Non-authority boundary flags for Studio runtime save/load presentation.
pub fn studio_scenario_runtime_saveload_non_authority_boundary(
) -> StudioScenarioRuntimeSaveLoadNonAuthorityBoundary {
    StudioScenarioRuntimeSaveLoadNonAuthorityBoundary::default()
}

/// Canonical JSON for loaded session authority (for status/save command composition).
pub fn canonical_json_from_loaded_scenario_authority(
    scenario: &simthing_spec::SimThingScenarioSpec,
) -> Result<String, SpecError> {
    Ok(save_scenario_spec_to_canonical_json(scenario)?.canonical_json)
}

/// Refresh UI status from loaded session authority without mutating session.
pub fn refresh_runtime_saveload_status_from_session(
    source_label: &str,
    scenario: &simthing_spec::SimThingScenarioSpec,
) -> Result<StudioScenarioRuntimeSaveLoadStatus, SpecError> {
    let json = canonical_json_from_loaded_scenario_authority(scenario)?;
    build_studio_scenario_runtime_saveload_status_from_json_str(source_label, &json)
}
