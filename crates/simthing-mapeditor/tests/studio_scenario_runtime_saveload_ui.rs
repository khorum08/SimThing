//! STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 — Studio runtime/candidate save-reopen UI adapter proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_mapeditor::{
    build_studio_scenario_runtime_saveload_status_from_json_str,
    canonical_json_from_loaded_scenario_authority, load_studio_session_from_scenario_path,
    reopen_candidate_scenario_for_studio, save_candidate_scenario_for_studio_create_new,
    studio_scenario_runtime_saveload_non_authority_boundary,
};
use tempfile::TempDir;

const OWNER_SILO_FIXTURE: &str = "owner_silo_disburse_down_scoped.simthing-scenario.json";

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_owner_silo_fixture_json() -> String {
    fs::read_to_string(corpus_path(OWNER_SILO_FIXTURE))
        .unwrap_or_else(|_| panic!("missing corpus {OWNER_SILO_FIXTURE}"))
}

fn status_fixture() -> simthing_mapeditor::StudioScenarioRuntimeSaveLoadStatus {
    build_studio_scenario_runtime_saveload_status_from_json_str(
        "owner_silo_corpus",
        &load_owner_silo_fixture_json(),
    )
    .expect("status")
}

#[test]
fn studio_runtime_saveload_status_reports_loaded_digest() {
    let status = status_fixture();
    assert!(status.loaded_scenario_digest.is_some());
    assert!(status.loaded_scenario_digest.unwrap() > 0);
}

#[test]
fn studio_runtime_saveload_status_reports_stead_validation_ready() {
    let status = status_fixture();
    assert!(status.stead_validation_ready);
}

#[test]
fn studio_runtime_saveload_status_reports_recursive_rf_runtime_ready() {
    let status = status_fixture();
    assert!(status.recursive_rf_runtime_ready);
}

#[test]
fn studio_runtime_saveload_status_reports_runtime_report_chain_ready() {
    let status = status_fixture();
    assert!(status.runtime_report_chain_ready);
}

#[test]
fn studio_runtime_saveload_status_reports_candidate_ready() {
    let status = status_fixture();
    assert!(status.candidate_ready);
    assert!(status.candidate_digest.is_some());
    assert!(status.candidate_save_ready);
    assert!(status.candidate_reopen_ready);
}

#[test]
fn studio_save_candidate_create_new_writes_candidate_file() {
    let json = load_owner_silo_fixture_json();
    let temp_dir = TempDir::new().expect("temp");
    let output_path = temp_dir.path().join("candidate.simthing-scenario.json");
    let result =
        save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &output_path)
            .expect("save");
    assert!(result.attempted);
    assert!(result.saved);
    assert!(result.create_new_policy);
    assert!(!result.target_existed);
    assert!(result.existing_target_preserved);
    let written = fs::read_to_string(&output_path).expect("read");
    assert!(!written.is_empty());
}

#[test]
fn studio_save_candidate_create_new_rejects_existing_target() {
    let json = load_owner_silo_fixture_json();
    let temp_dir = TempDir::new().expect("temp");
    let output_path = temp_dir.path().join("candidate.simthing-scenario.json");
    fs::write(&output_path, "seed").expect("seed");
    let result =
        save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &output_path)
            .expect("save");
    assert!(result.attempted);
    assert!(!result.saved);
    assert!(result.target_existed);
    assert!(result.create_new_policy);
    assert!(result.message.contains("create-new"));
}

#[test]
fn studio_save_candidate_existing_target_preserved() {
    let json = load_owner_silo_fixture_json();
    let temp_dir = TempDir::new().expect("temp");
    let output_path = temp_dir.path().join("candidate.simthing-scenario.json");
    let seed = "existing candidate contents";
    fs::write(&output_path, seed).expect("seed");
    let _ = save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &output_path)
        .expect("save");
    assert_eq!(fs::read_to_string(&output_path).expect("read"), seed);
}

#[test]
fn studio_reopen_candidate_loads_canonical_scenario_json() {
    let json = load_owner_silo_fixture_json();
    let temp_dir = TempDir::new().expect("temp");
    let candidate_path = temp_dir.path().join("candidate.simthing-scenario.json");
    save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &candidate_path)
        .expect("save")
        .saved;
    let result = reopen_candidate_scenario_for_studio(&candidate_path).expect("reopen");
    assert!(result.attempted);
    assert!(result.reopened);
    assert!(result.reopened_digest.is_some());
}

#[test]
fn studio_reopen_candidate_rebuilds_validation_and_projection_readiness() {
    let json = load_owner_silo_fixture_json();
    let temp_dir = TempDir::new().expect("temp");
    let candidate_path = temp_dir.path().join("candidate.simthing-scenario.json");
    save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &candidate_path)
        .expect("save");
    let result = reopen_candidate_scenario_for_studio(&candidate_path).expect("reopen");
    assert!(result.stead_validation_ready);
    assert!(result.projection_rebuild_ready);
}

#[test]
fn studio_save_reopen_failed_operation_preserves_current_session_status() {
    let session = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("session");
    let status_before = build_studio_scenario_runtime_saveload_status_from_json_str(
        "owner_silo_corpus",
        &load_owner_silo_fixture_json(),
    )
    .expect("status");
    let json =
        canonical_json_from_loaded_scenario_authority(&session.scenario_authority).expect("json");
    let temp_dir = TempDir::new().expect("temp");
    let output_path = temp_dir.path().join("candidate.simthing-scenario.json");
    fs::write(&output_path, "seed").expect("seed");
    let save_result =
        save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &output_path)
            .expect("save");
    assert!(!save_result.saved);
    let status_after = build_studio_scenario_runtime_saveload_status_from_json_str(
        "owner_silo_corpus",
        &load_owner_silo_fixture_json(),
    )
    .expect("status");
    assert_eq!(status_before, status_after);
    assert_eq!(
        session.scenario_authority.scenario_id,
        load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
            .expect("reload")
            .scenario_authority
            .scenario_id
    );
}

#[test]
fn studio_runtime_saveload_ui_does_not_write_repo_fixtures() {
    let path = corpus_path(OWNER_SILO_FIXTURE);
    if !path.exists() {
        return;
    }
    let _ = status_fixture();
    let mtime = fs::metadata(&path)
        .and_then(|m| m.modified())
        .expect("mtime");
    let age = SystemTime::now()
        .duration_since(mtime)
        .unwrap_or(Duration::from_secs(0));
    assert!(
        age.as_secs() > 5,
        "corpus fixture must not be rewritten during normal tests"
    );
}

#[test]
fn studio_runtime_saveload_ui_defers_persistent_history_and_gpu_dispatch() {
    let boundary = studio_scenario_runtime_saveload_non_authority_boundary();
    assert!(boundary.persistent_history_deferred);
    assert!(boundary.gpu_dispatch_deferred);
    assert!(!boundary.ui_state_is_authority);
    assert!(!boundary.bevy_state_is_authority);
    assert!(!boundary.runtime_reports_are_authority);
    assert!(!boundary.gpu_buffers_are_authority);
    assert!(boundary.canonical_scenario_json_only);
    assert!(boundary.no_distinct_savefile_format);
}
