//! STUDIO-CANDIDATE-REOPEN-ADOPT-0 — successful Reopen Candidate adopts candidate Studio session.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_mapeditor::{
    build_studio_scenario_runtime_saveload_status_from_json_str,
    canonical_json_from_loaded_scenario_authority, load_studio_session_from_scenario_path,
    refresh_runtime_saveload_status_from_session, reopen_candidate_scenario_for_studio_session,
    save_candidate_scenario_for_studio_create_new,
    studio_scenario_runtime_saveload_non_authority_boundary,
};
use simthing_spec::save_scenario_spec_to_canonical_json;
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

fn saved_candidate_path(temp_dir: &TempDir) -> PathBuf {
    let json = load_owner_silo_fixture_json();
    let candidate_path = temp_dir.path().join("candidate.simthing-scenario.json");
    assert!(
        save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &candidate_path)
            .expect("save")
            .saved
    );
    candidate_path
}

#[test]
fn studio_reopen_candidate_success_adopts_reopened_candidate_session() {
    let temp_dir = TempDir::new().expect("temp");
    let candidate_path = saved_candidate_path(&temp_dir);
    let adoption = reopen_candidate_scenario_for_studio_session(&candidate_path).expect("adopt");
    assert!(adoption.adopted);
    let session = adoption.session.expect("session");
    assert_eq!(
        save_scenario_spec_to_canonical_json(&session.scenario_authority)
            .expect("digest")
            .authority_digest,
        adoption.reopened_digest.unwrap()
    );
}

#[test]
fn studio_reopen_candidate_success_updates_loaded_digest_to_candidate_digest() {
    let original_session =
        load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
            .expect("session");
    let original_digest =
        save_scenario_spec_to_canonical_json(&original_session.scenario_authority)
            .expect("digest")
            .authority_digest;
    let temp_dir = TempDir::new().expect("temp");
    let candidate_path = saved_candidate_path(&temp_dir);
    let adoption = reopen_candidate_scenario_for_studio_session(&candidate_path).expect("adopt");
    let status = adoption.status.expect("status");
    assert_ne!(status.loaded_scenario_digest, Some(original_digest));
    assert_eq!(status.loaded_scenario_digest, adoption.reopened_digest);
}

#[test]
fn studio_reopen_candidate_success_rebuilds_runtime_saveload_status() {
    let temp_dir = TempDir::new().expect("temp");
    let candidate_path = saved_candidate_path(&temp_dir);
    let adoption = reopen_candidate_scenario_for_studio_session(&candidate_path).expect("adopt");
    let session = adoption.session.expect("session");
    let status = adoption.status.expect("status");
    let rebuilt = refresh_runtime_saveload_status_from_session(
        "studio_reopened_candidate",
        &session.scenario_authority,
    )
    .expect("rebuild");
    assert_eq!(status, rebuilt);
    assert!(status.stead_validation_ready);
    assert!(status.runtime_report_chain_ready);
}

#[test]
fn studio_reopen_candidate_success_reports_stead_validation_and_projection_ready() {
    let temp_dir = TempDir::new().expect("temp");
    let candidate_path = saved_candidate_path(&temp_dir);
    let adoption = reopen_candidate_scenario_for_studio_session(&candidate_path).expect("adopt");
    assert!(adoption.stead_validation_ready);
    assert!(adoption.projection_rebuild_ready);
}

#[test]
fn studio_reopen_candidate_failure_preserves_current_session() {
    let session = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("session");
    let scenario_id = session.scenario_authority.scenario_id.clone();
    let temp_dir = TempDir::new().expect("temp");
    let invalid_path = temp_dir.path().join("invalid.simthing-scenario.json");
    fs::write(&invalid_path, "{not canonical json").expect("write");
    let err = reopen_candidate_scenario_for_studio_session(&invalid_path).expect_err("reject");
    assert_eq!(err, simthing_spec::SpecError::ValidationFailed);
    let reloaded = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("reload");
    assert_eq!(reloaded.scenario_authority.scenario_id, scenario_id);
}

#[test]
fn studio_reopen_candidate_failure_preserves_current_runtime_status_except_message() {
    let session = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("session");
    let status_before = refresh_runtime_saveload_status_from_session(
        "owner_silo_corpus",
        &session.scenario_authority,
    )
    .expect("status");
    let temp_dir = TempDir::new().expect("temp");
    let invalid_path = temp_dir.path().join("invalid.simthing-scenario.json");
    fs::write(&invalid_path, "{not canonical json").expect("write");
    let err = reopen_candidate_scenario_for_studio_session(&invalid_path).expect_err("reject");
    assert_eq!(err, simthing_spec::SpecError::ValidationFailed);
    let status_after = refresh_runtime_saveload_status_from_session(
        "owner_silo_corpus",
        &session.scenario_authority,
    )
    .expect("status");
    assert_eq!(status_before, status_after);
}

#[test]
fn studio_reopen_candidate_rejects_noncanonical_or_invalid_json() {
    let temp_dir = TempDir::new().expect("temp");
    let invalid_path = temp_dir.path().join("invalid.simthing-scenario.json");
    fs::write(&invalid_path, "{not canonical json").expect("write");
    let err = reopen_candidate_scenario_for_studio_session(&invalid_path).expect_err("reject");
    assert_eq!(err, simthing_spec::SpecError::ValidationFailed);
}

#[test]
fn studio_reopen_candidate_does_not_write_repo_fixtures() {
    let path = corpus_path(OWNER_SILO_FIXTURE);
    if !path.exists() {
        return;
    }
    let temp_dir = TempDir::new().expect("temp");
    let candidate_path = saved_candidate_path(&temp_dir);
    let _ = reopen_candidate_scenario_for_studio_session(&candidate_path).expect("adopt");
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
fn studio_reopen_candidate_keeps_ui_bevy_runtime_gpu_non_authoritative() {
    let boundary = studio_scenario_runtime_saveload_non_authority_boundary();
    assert!(!boundary.ui_state_is_authority);
    assert!(!boundary.bevy_state_is_authority);
    assert!(!boundary.runtime_reports_are_authority);
    assert!(!boundary.gpu_buffers_are_authority);
}

#[test]
fn studio_reopen_candidate_defers_persistent_history_and_gpu_dispatch() {
    let boundary = studio_scenario_runtime_saveload_non_authority_boundary();
    assert!(boundary.persistent_history_deferred);
    assert!(boundary.gpu_dispatch_deferred);
}
