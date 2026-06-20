//! SCENARIO-CANDIDATE-SAVE-REOPEN-0 — save and reopen candidate ScenarioSpec with stable authority digest.
//!
//! GPU-residency doctrine: runtime property-view rows are GPU-compatible source data.
//! CPU space is limited to candidate canonical serialization/bookkeeping — not production simulation authority.

use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::SpecError;

use super::loaded_scenario_studio_session_envelope::evaluate_loaded_scenario_studio_session_envelope_from_json_str;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario_candidate_from_runtime::{
    evaluate_scenario_candidate_from_runtime_from_json_str,
    prove_scenario_candidate_from_runtime_preserves_original_authority,
};
use super::scenario_canonical_io::{
    load_scenario_spec_from_json_str, save_scenario_spec_to_canonical_json,
};
use super::scenario_property_mutation_authority_boundary::clone_scenario_candidate_with_runtime_property_view;
use super::scenario_stead_map_roundtrip::{
    evaluate_scenario_stead_map_roundtrip_from_json_str, extract_scenario_rf_metadata_rows,
    extract_scenario_spatial_tree_rows, extract_scenario_stead_id_rows,
    extract_scenario_stead_link_rows,
};

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioCandidateSaveReopenSource {
    ScenarioCandidateFromRuntime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateScenarioWritePolicy {
    CreateNew,
    ReplaceExisting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioCandidateWritePolicyReport {
    pub policy: CandidateScenarioWritePolicy,
    pub same_directory_temp_file: bool,
    pub existing_target_preserved_on_error: bool,
    pub removes_existing_target_before_successful_write: bool,
    pub temp_cleanup_attempted_on_error: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCandidateSaveReport {
    pub canonical_json: String,
    pub candidate_authority_digest: u64,
    pub byte_len: u32,
    pub deterministic: bool,
    pub atomic_write_ready: bool,
    pub wrote_to_temp_path_only: bool,
    pub write_policy: ScenarioCandidateWritePolicyReport,
    pub existing_target_preserved_on_error: bool,
    pub same_directory_temp_file: bool,
    pub removes_existing_target_before_successful_write: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCandidateReopenReport {
    pub reopened_authority_digest: u64,
    pub digest_matches_saved_candidate: bool,
    pub canonical_io_ready: bool,
    pub stead_ids_preserved: bool,
    pub links_preserved: bool,
    pub spatial_tree_preserved: bool,
    pub rf_metadata_preserved: bool,
    pub owner_metadata_not_spatial_parentage: bool,
    pub studio_projection_rebuild_ready: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCandidateSaveReopenReport {
    pub source: ScenarioCandidateSaveReopenSource,
    pub source_label: String,

    pub candidate_from_runtime_ready: bool,
    pub original_authority_preserved: bool,
    pub candidate_authority_digest_before_save: u64,

    pub save_report: ScenarioCandidateSaveReport,
    pub reopen_report: ScenarioCandidateReopenReport,

    pub candidate_save_reopen_ready: bool,
    pub candidate_digest_stable_after_reopen: bool,
    pub candidate_stead_tree_projection_ready: bool,

    pub canonical_scenario_json_only: bool,
    pub no_distinct_savefile_format_introduced: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Evaluate candidate ScenarioSpec save/reopen report from canonical JSON, composing #842 candidate path.
pub fn evaluate_scenario_candidate_save_reopen_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<ScenarioCandidateSaveReopenReport, SpecError> {
    let candidate_report =
        evaluate_scenario_candidate_from_runtime_from_json_str(source_label, json)?;
    if !candidate_report.candidate_scenario_spec_ready {
        return Err(SpecError::ValidationFailed);
    }

    let (original_scenario, _) = load_scenario_spec_from_json_str(source_label, json)?;
    let candidate = clone_scenario_candidate_with_runtime_property_view(
        &original_scenario,
        TICK_ONE,
        REPLAY_ONE,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let canonical_save = save_scenario_spec_to_canonical_json(&candidate)?;
    let write_policy = candidate_scenario_write_policy_report();
    let save_report = ScenarioCandidateSaveReport {
        canonical_json: canonical_save.canonical_json.clone(),
        candidate_authority_digest: canonical_save.authority_digest,
        byte_len: canonical_save.byte_len,
        deterministic: canonical_save.deterministic,
        atomic_write_ready: true,
        wrote_to_temp_path_only: false,
        existing_target_preserved_on_error: write_policy.existing_target_preserved_on_error,
        same_directory_temp_file: write_policy.same_directory_temp_file,
        removes_existing_target_before_successful_write: write_policy
            .removes_existing_target_before_successful_write,
        write_policy,
    };

    let candidate_authority_digest_before_save = save_report.candidate_authority_digest;

    let (reopened_scenario, reopen_load) =
        load_scenario_spec_from_json_str(source_label, &save_report.canonical_json)?;
    let stead_report = evaluate_scenario_stead_map_roundtrip_from_json_str(
        source_label,
        &save_report.canonical_json,
    )?;
    let envelope = evaluate_loaded_scenario_studio_session_envelope_from_json_str(
        source_label,
        &save_report.canonical_json,
    )?;

    let candidate_stead_ids = extract_scenario_stead_id_rows(&candidate)?;
    let reopened_stead_ids = extract_scenario_stead_id_rows(&reopened_scenario)?;
    let candidate_links = extract_scenario_stead_link_rows(&candidate)?;
    let reopened_links = extract_scenario_stead_link_rows(&reopened_scenario)?;
    let candidate_spatial_tree = extract_scenario_spatial_tree_rows(&candidate)?;
    let reopened_spatial_tree = extract_scenario_spatial_tree_rows(&reopened_scenario)?;
    let candidate_rf_metadata = extract_scenario_rf_metadata_rows(&candidate)?;
    let reopened_rf_metadata = extract_scenario_rf_metadata_rows(&reopened_scenario)?;

    let reopened_authority_digest = reopen_load.authority_digest;
    let digest_matches_saved_candidate =
        candidate_authority_digest_before_save == reopened_authority_digest;

    let reopen_report = ScenarioCandidateReopenReport {
        reopened_authority_digest,
        digest_matches_saved_candidate,
        canonical_io_ready: reopen_load.loaded
            && reopen_load.ingestion_ready
            && digest_matches_saved_candidate,
        stead_ids_preserved: candidate_stead_ids == reopened_stead_ids
            && stead_report.stead_ids_stable,
        links_preserved: candidate_links == reopened_links && stead_report.links_stable,
        spatial_tree_preserved: candidate_spatial_tree == reopened_spatial_tree
            && stead_report.spatial_tree_stable,
        rf_metadata_preserved: candidate_rf_metadata == reopened_rf_metadata
            && stead_report.rf_metadata_stable,
        owner_metadata_not_spatial_parentage: stead_report.owner_metadata_not_spatial_parentage,
        studio_projection_rebuild_ready: envelope.authority.studio_projection_rebuild_ready,
    };

    let original_authority_preserved =
        prove_scenario_candidate_from_runtime_preserves_original_authority(source_label, json)?;

    let candidate_digest_stable_after_reopen =
        digest_matches_saved_candidate && stead_report.digest_stable;
    let candidate_stead_tree_projection_ready = reopen_report.studio_projection_rebuild_ready
        && reopen_report.stead_ids_preserved
        && reopen_report.links_preserved
        && reopen_report.spatial_tree_preserved;

    let candidate_save_reopen_ready = candidate_report.candidate_scenario_spec_ready
        && save_report.deterministic
        && reopen_report.canonical_io_ready
        && reopen_report.digest_matches_saved_candidate
        && reopen_report.stead_ids_preserved
        && reopen_report.links_preserved
        && reopen_report.spatial_tree_preserved
        && reopen_report.rf_metadata_preserved
        && reopen_report.owner_metadata_not_spatial_parentage
        && reopen_report.studio_projection_rebuild_ready
        && original_authority_preserved;

    Ok(ScenarioCandidateSaveReopenReport {
        source: ScenarioCandidateSaveReopenSource::ScenarioCandidateFromRuntime,
        source_label: source_label.to_string(),
        candidate_from_runtime_ready: candidate_report.candidate_scenario_spec_ready,
        original_authority_preserved,
        candidate_authority_digest_before_save,
        save_report,
        reopen_report,
        candidate_save_reopen_ready,
        candidate_digest_stable_after_reopen,
        candidate_stead_tree_projection_ready,
        canonical_scenario_json_only: true,
        no_distinct_savefile_format_introduced: true,
        persistent_history_deferred: candidate_report.persistent_history_deferred,
        studio_ui_wiring_deferred: candidate_report.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: candidate_report.gpu_dispatch_deferred,
    })
}

/// Create-new policy report for hardened candidate canonical JSON writes.
pub fn candidate_scenario_write_policy_report() -> ScenarioCandidateWritePolicyReport {
    ScenarioCandidateWritePolicyReport {
        policy: CandidateScenarioWritePolicy::CreateNew,
        same_directory_temp_file: true,
        existing_target_preserved_on_error: true,
        removes_existing_target_before_successful_write: false,
        temp_cleanup_attempted_on_error: true,
    }
}

/// Derive a same-directory temp path from the output filename with a unique suffix.
pub fn candidate_scenario_write_temp_path(output_path: &Path) -> PathBuf {
    let parent = output_path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = output_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "candidate.simthing-scenario.json".to_string());
    let unique = format!(
        "{file_name}.write.{}.{}.tmp",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0)
    );
    parent.join(unique)
}

/// Atomically create candidate canonical ScenarioSpec JSON via same-directory temp-to-rename.
///
/// Create-new only: returns an error when `output_path` already exists and leaves the existing
/// target unchanged. Overwrite/replace semantics are deferred to a future explicit helper.
pub fn write_candidate_scenario_canonical_json_atomic(
    canonical_json: &str,
    output_path: &Path,
) -> Result<(), SpecError> {
    if output_path.exists() {
        return Err(SpecError::ValidationFailed);
    }

    let tmp = candidate_scenario_write_temp_path(output_path);
    let mut file = std::fs::File::create(&tmp).map_err(|_| SpecError::ValidationFailed)?;
    if file
        .write_all(canonical_json.as_bytes())
        .and_then(|_| file.sync_all())
        .is_err()
    {
        cleanup_candidate_write_temp(&tmp);
        return Err(SpecError::ValidationFailed);
    }

    if std::fs::rename(&tmp, output_path).is_err() {
        cleanup_candidate_write_temp(&tmp);
        return Err(SpecError::ValidationFailed);
    }

    Ok(())
}

fn cleanup_candidate_write_temp(temp_path: &Path) {
    let _ = std::fs::remove_file(temp_path);
}

/// Prove candidate ScenarioSpec authority digest is stable after canonical save/reopen.
pub fn prove_scenario_candidate_save_reopen_digest_stability(
    source_label: &str,
    json: &str,
) -> Result<bool, SpecError> {
    let report = evaluate_scenario_candidate_save_reopen_from_json_str(source_label, json)?;
    Ok(report.candidate_save_reopen_ready
        && report.candidate_digest_stable_after_reopen
        && report.reopen_report.digest_matches_saved_candidate
        && report.save_report.deterministic)
}
