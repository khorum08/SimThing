//! LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 — loaded scenario recursive RF runtime proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_spec::{
    evaluate_loaded_scenario_recursive_rf_runtime_from_json_str,
    evaluate_loaded_scenario_studio_session_envelope_from_json_str,
    prove_loaded_scenario_recursive_rf_runtime_preserves_authority,
    LoadedScenarioRecursiveRfRuntimeSource,
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

fn evaluate_fixture() -> simthing_spec::LoadedScenarioRecursiveRfRuntimeReport {
    let json = load_owner_silo_fixture_json();
    evaluate_loaded_scenario_recursive_rf_runtime_from_json_str("owner_silo_corpus", &json)
        .expect("evaluate")
}

#[test]
fn loaded_scenario_recursive_rf_runtime_composes_session_envelope() {
    let json = load_owner_silo_fixture_json();
    let envelope =
        evaluate_loaded_scenario_studio_session_envelope_from_json_str("owner_silo_corpus", &json)
            .expect("envelope");
    let report = evaluate_fixture();
    assert!(report.loaded_session_envelope_ready);
    assert_eq!(
        report.scenario_authority_digest,
        envelope.authority.scenario_authority_digest
    );
}

#[test]
fn loaded_scenario_recursive_rf_runtime_reports_authority_digest() {
    let report = evaluate_fixture();
    assert!(report.scenario_authority_digest > 0);
    assert_eq!(
        report.source,
        LoadedScenarioRecursiveRfRuntimeSource::LoadedScenarioStudioSessionEnvelope
    );
}

#[test]
fn loaded_scenario_recursive_rf_runtime_extracts_participant_rows() {
    let report = evaluate_fixture();
    assert!(!report.participant_rows.is_empty());
    assert_eq!(
        report.participant_row_count as usize,
        report.participant_rows.len()
    );
    assert!(report.participant_rows.iter().all(|row| row
        .owner_ref
        .as_ref()
        .is_some_and(|owner| !owner.as_str().is_empty())));
}

#[test]
fn loaded_scenario_recursive_rf_runtime_extracts_parent_location_arenas() {
    let report = evaluate_fixture();
    assert!(!report.parent_arena_rows.is_empty());
    assert_eq!(
        report.parent_location_arena_count as usize,
        report.parent_arena_rows.len()
    );
    assert!(report.parent_arena_rows.iter().all(|row| row.depth > 0 || {
        report
            .parent_arena_rows
            .iter()
            .any(|arena| arena.parent_location_id == row.parent_location_id)
    }));
}

#[test]
fn loaded_scenario_recursive_rf_runtime_resolves_parent_arena_locally_first() {
    let report = evaluate_fixture();
    assert!(report.local_parent_node_resolution_first);
    assert!(report
        .parent_arena_rows
        .iter()
        .all(|row| row.local_settlement_applied_before_upward_bubbling));
}

#[test]
fn loaded_scenario_recursive_rf_runtime_settles_siblings_before_upward_bubbling() {
    let report = evaluate_fixture();
    assert!(report.sibling_settlement_before_upward_bubbling);
    for channel in &report.channel_rows {
        let expected_match = channel.available_total.min(channel.requested_total);
        assert_eq!(channel.satisfied_total, expected_match);
        assert_eq!(
            channel.surplus_total,
            channel.available_total - channel.satisfied_total
        );
        assert_eq!(
            channel.unmet_total,
            channel.requested_total - channel.satisfied_total
        );
        assert_eq!(
            channel.net_upward_delta,
            channel.surplus_total - channel.unmet_total
        );
    }
}

#[test]
fn loaded_scenario_recursive_rf_runtime_reduces_upward_by_owner_resource_scope() {
    let report = evaluate_fixture();
    assert!(!report.channel_rows.is_empty());
    assert_eq!(report.channel_row_count as usize, report.channel_rows.len());
    assert!(report.channel_rows.iter().all(|row| {
        row.owner_ref.is_some() && row.resource_key.is_some() && row.scope_id.is_some()
    }));
}

#[test]
fn loaded_scenario_recursive_rf_runtime_preserves_owner_scope_not_spatial_parentage() {
    let report = evaluate_fixture();
    assert!(report.owner_scope_not_spatial_parentage);
    for row in &report.participant_rows {
        let owner = row.owner_ref.as_ref().map(|owner| owner.as_str()).unwrap_or("");
        assert_ne!(owner, row.parent_location_id.raw().to_string());
        assert_ne!(owner, row.simthing_id_raw.to_string());
    }
}

#[test]
fn loaded_scenario_recursive_rf_runtime_emits_gpu_compatible_rows() {
    let report = evaluate_fixture();
    assert!(report.gpu_compatible_row_table_surface);
    assert!(!report.participant_rows.is_empty());
    assert!(!report.parent_arena_rows.is_empty());
    assert!(!report.channel_rows.is_empty());
}

#[test]
fn loaded_scenario_recursive_rf_runtime_cpu_oracle_only() {
    let report = evaluate_fixture();
    assert!(report.cpu_oracle_only);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_defers_scenario_and_runtime_mutation() {
    let report = evaluate_fixture();
    assert!(report.scenario_authority_mutation_deferred);
    assert!(report.runtime_mutation_deferred);
    assert!(
        prove_loaded_scenario_recursive_rf_runtime_preserves_authority(
            "owner_silo_corpus",
            &load_owner_silo_fixture_json()
        )
        .expect("prove")
    );
}

#[test]
fn loaded_scenario_recursive_rf_runtime_defers_semantic_execution() {
    let report = evaluate_fixture();
    assert!(report.semantic_execution_deferred);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_defers_savefile_history_ui_and_gpu_dispatch() {
    let report = evaluate_fixture();
    assert!(report.savefile_persistence_deferred);
    assert!(report.persistent_history_deferred);
    assert!(report.studio_ui_wiring_deferred);
    assert!(report.gpu_dispatch_deferred);
}

#[test]
fn normal_tests_do_not_write_loaded_scenario_recursive_rf_fixtures() {
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
