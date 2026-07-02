//! LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 — loaded scenario session envelope proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_spec::{
    evaluate_loaded_scenario_studio_session_envelope_from_json_str,
    evaluate_scenario_stead_map_roundtrip_from_json_str,
    prove_loaded_scenario_session_envelope_preserves_authority_boundaries,
    prove_scenario_canonical_load_save_roundtrip, LoadedScenarioSessionSource, SpecError,
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

fn evaluate_fixture() -> simthing_spec::LoadedScenarioStudioSessionEnvelope {
    let json = load_owner_silo_fixture_json();
    evaluate_loaded_scenario_studio_session_envelope_from_json_str("owner_silo_corpus", &json)
        .expect("evaluate")
}

#[test]
fn loaded_scenario_session_envelope_composes_canonical_io() {
    let json = load_owner_silo_fixture_json();
    let canonical = prove_scenario_canonical_load_save_roundtrip("owner_silo_corpus", &json)
        .expect("canonical");
    let envelope = evaluate_fixture();
    assert!(envelope.authority.canonical_io_ready);
    assert!(canonical.scenario_authority_preserved);
    assert_eq!(
        envelope.authority.scenario_authority_digest,
        canonical.initial_digest
    );
}

#[test]
fn loaded_scenario_session_envelope_composes_stead_map_roundtrip() {
    let json = load_owner_silo_fixture_json();
    let stead = evaluate_scenario_stead_map_roundtrip_from_json_str("owner_silo_corpus", &json)
        .expect("stead");
    let envelope = evaluate_fixture();
    assert!(envelope.authority.stead_map_roundtrip_ready);
    assert_eq!(envelope.authority.stead_ids_stable, stead.stead_ids_stable);
    assert_eq!(envelope.authority.links_stable, stead.links_stable);
    assert_eq!(
        envelope.authority.spatial_tree_stable,
        stead.spatial_tree_stable
    );
    assert_eq!(
        envelope.authority.rf_metadata_stable,
        stead.rf_metadata_stable
    );
}

#[test]
fn loaded_scenario_session_envelope_reports_authority_digest() {
    let envelope = evaluate_fixture();
    assert!(envelope.authority.scenario_authority_digest > 0);
    assert_eq!(
        envelope.authority.scenario_id.as_deref(),
        Some("owner_silo_disburse_down_scoped")
    );
    assert_eq!(
        envelope.authority.source,
        LoadedScenarioSessionSource::CanonicalScenarioJson
    );
}

#[test]
fn loaded_scenario_session_envelope_reports_import_export_ready() {
    let envelope = evaluate_fixture();
    assert!(envelope.authority.scenario_import_ready);
    assert!(envelope.authority.scenario_export_ready);
}

#[test]
fn loaded_scenario_session_envelope_reports_projection_rebuild_ready() {
    let envelope = evaluate_fixture();
    assert!(envelope.authority.studio_projection_rebuild_ready);
}

#[test]
fn loaded_scenario_session_envelope_reports_recursive_rf_prerequisites_ready() {
    let envelope = evaluate_fixture();
    assert!(envelope.authority.recursive_rf_prerequisites_ready);
    assert!(envelope.runtime_sidecar.recursive_rf_runtime_ready);
}

#[test]
fn loaded_scenario_session_envelope_marks_non_authority_surfaces_false() {
    let envelope = evaluate_fixture();
    assert!(!envelope.studio_config_is_authority);
    assert!(!envelope.bevy_state_is_authority);
    assert!(!envelope.gpu_buffers_are_authority);
    assert!(!envelope.runtime_reports_are_authority);
    assert!(
        prove_loaded_scenario_session_envelope_preserves_authority_boundaries(
            "owner_silo_corpus",
            &load_owner_silo_fixture_json()
        )
        .expect("prove")
    );
}

#[test]
fn loaded_scenario_session_envelope_defers_runtime_execution_and_mutation() {
    let envelope = evaluate_fixture();
    assert!(envelope.runtime_sidecar.runtime_tick_execution_deferred);
    assert!(envelope.runtime_sidecar.runtime_mutation_deferred);
    assert!(envelope.runtime_sidecar.semantic_execution_deferred);
    assert!(envelope.runtime_sidecar.studio_ui_wiring_deferred);
    assert!(envelope.runtime_sidecar.gpu_dispatch_deferred);
}

#[test]
fn loaded_scenario_session_envelope_defers_savefile_persistence_and_history() {
    let envelope = evaluate_fixture();
    assert!(envelope.runtime_sidecar.savefile_persistence_deferred);
    assert!(envelope.runtime_sidecar.persistent_history_deferred);
}

#[test]
fn normal_tests_do_not_write_loaded_scenario_session_fixtures() {
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
