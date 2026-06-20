//! SCENARIO-CANDIDATE-SAVE-REOPEN-0 — candidate ScenarioSpec save/reopen proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_spec::{
    evaluate_scenario_candidate_from_runtime_from_json_str,
    evaluate_scenario_candidate_save_reopen_from_json_str, load_scenario_spec_from_json_str,
    prove_scenario_candidate_save_reopen_digest_stability,
    write_candidate_scenario_canonical_json_atomic, ScenarioCandidateSaveReopenSource, SpecError,
};

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

fn evaluate_fixture() -> simthing_spec::ScenarioCandidateSaveReopenReport {
    let json = load_owner_silo_fixture_json();
    evaluate_scenario_candidate_save_reopen_from_json_str("owner_silo_corpus", &json)
        .expect("evaluate")
}

#[test]
fn scenario_candidate_save_reopen_composes_candidate_from_runtime() {
    let json = load_owner_silo_fixture_json();
    let candidate =
        evaluate_scenario_candidate_from_runtime_from_json_str("owner_silo_corpus", &json)
            .expect("candidate");
    let report = evaluate_fixture();
    assert!(report.candidate_from_runtime_ready);
    assert!(candidate.candidate_scenario_spec_ready);
    assert_eq!(
        report.source,
        ScenarioCandidateSaveReopenSource::ScenarioCandidateFromRuntime
    );
}

#[test]
fn scenario_candidate_save_reopen_serializes_candidate_canonical_json() {
    let report = evaluate_fixture();
    assert!(!report.save_report.canonical_json.is_empty());
    assert!(report.save_report.byte_len > 0);
    assert!(report.save_report.deterministic);
    assert!(report.save_report.candidate_authority_digest > 0);
}

#[test]
fn scenario_candidate_save_reopen_writes_temp_file_atomically() {
    let report = evaluate_fixture();
    let temp_dir = std::env::temp_dir().join(format!(
        "simthing_candidate_save_reopen_{}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("candidate.simthing-scenario.json");
    write_candidate_scenario_canonical_json_atomic(
        &report.save_report.canonical_json,
        &output_path,
    )
    .expect("atomic write");
    let written = fs::read_to_string(&output_path).expect("read");
    assert_eq!(written, report.save_report.canonical_json);
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn scenario_candidate_save_reopen_reopens_candidate() {
    let report = evaluate_fixture();
    let (_, load_report) =
        load_scenario_spec_from_json_str("owner_silo_corpus", &report.save_report.canonical_json)
            .expect("reopen");
    assert!(load_report.loaded);
    assert!(load_report.ingestion_ready);
    assert!(report.reopen_report.canonical_io_ready);
}

#[test]
fn scenario_candidate_save_reopen_candidate_digest_stable_after_reopen() {
    let report = evaluate_fixture();
    assert!(report.candidate_digest_stable_after_reopen);
    assert!(report.reopen_report.digest_matches_saved_candidate);
    assert_eq!(
        report.candidate_authority_digest_before_save,
        report.reopen_report.reopened_authority_digest
    );
    assert!(prove_scenario_candidate_save_reopen_digest_stability(
        "owner_silo_corpus",
        &load_owner_silo_fixture_json()
    )
    .expect("prove"));
}

#[test]
fn scenario_candidate_save_reopen_preserves_original_authority() {
    let report = evaluate_fixture();
    assert!(report.original_authority_preserved);
}

#[test]
fn scenario_candidate_save_reopen_preserves_stead_ids() {
    let report = evaluate_fixture();
    assert!(report.reopen_report.stead_ids_preserved);
}

#[test]
fn scenario_candidate_save_reopen_preserves_links() {
    let report = evaluate_fixture();
    assert!(report.reopen_report.links_preserved);
}

#[test]
fn scenario_candidate_save_reopen_preserves_spatial_tree() {
    let report = evaluate_fixture();
    assert!(report.reopen_report.spatial_tree_preserved);
}

#[test]
fn scenario_candidate_save_reopen_preserves_rf_metadata() {
    let report = evaluate_fixture();
    assert!(report.reopen_report.rf_metadata_preserved);
}

#[test]
fn scenario_candidate_save_reopen_preserves_owner_metadata_not_spatial_parentage() {
    let report = evaluate_fixture();
    assert!(report.reopen_report.owner_metadata_not_spatial_parentage);
}

#[test]
fn scenario_candidate_save_reopen_reports_projection_rebuild_ready() {
    let report = evaluate_fixture();
    assert!(report.reopen_report.studio_projection_rebuild_ready);
    assert!(report.candidate_stead_tree_projection_ready);
}

#[test]
fn scenario_candidate_save_reopen_uses_canonical_scenario_json_only() {
    let report = evaluate_fixture();
    assert!(report.canonical_scenario_json_only);
    assert!(report.save_report.atomic_write_ready);
}

#[test]
fn scenario_candidate_save_reopen_does_not_introduce_distinct_savefile_format() {
    let report = evaluate_fixture();
    assert!(report.no_distinct_savefile_format_introduced);
}

#[test]
fn scenario_candidate_save_reopen_defers_persistent_history_ui_and_gpu_dispatch() {
    let report = evaluate_fixture();
    assert!(report.persistent_history_deferred);
    assert!(report.studio_ui_wiring_deferred);
    assert!(report.gpu_dispatch_deferred);
}

#[test]
fn scenario_candidate_save_reopen_rejects_invalid_json() {
    let err = evaluate_scenario_candidate_save_reopen_from_json_str("invalid", "{not json")
        .expect_err("reject");
    assert_eq!(err, SpecError::ValidationFailed);
}

#[test]
fn normal_tests_do_not_write_candidate_save_reopen_fixtures() {
    let path = corpus_path(OWNER_SILO_FIXTURE);
    if !path.exists() {
        return;
    }
    let _ = evaluate_fixture();
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
