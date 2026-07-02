//! SCENARIO-CANDIDATE-FROM-RUNTIME-0 — candidate ScenarioSpec from loaded runtime proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_spec::{
    clone_scenario_candidate_with_runtime_property_view,
    evaluate_loaded_scenario_runtime_report_chain_from_json_str, evaluate_planet_child_locations,
    evaluate_scenario_candidate_from_runtime_from_json_str, load_scenario_spec_from_json_str,
    prove_scenario_candidate_from_runtime_preserves_original_authority,
    serialize_scenario_authority, ScenarioCandidateFromRuntimeSource, SpecError,
    RUNTIME_PREVIEW_APPLIED_PROPERTY_ID, RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID,
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

fn evaluate_fixture() -> simthing_spec::ScenarioCandidateFromRuntimeReport {
    let json = load_owner_silo_fixture_json();
    evaluate_scenario_candidate_from_runtime_from_json_str("owner_silo_corpus", &json)
        .expect("evaluate")
}

#[test]
fn scenario_candidate_from_runtime_composes_loaded_runtime_report_chain() {
    let json = load_owner_silo_fixture_json();
    let chain =
        evaluate_loaded_scenario_runtime_report_chain_from_json_str("owner_silo_corpus", &json)
            .expect("chain");
    let report = evaluate_fixture();
    assert!(report.runtime_report_chain_ready);
    assert!(chain.runtime_property_view_rows_ready);
    assert_eq!(
        report.source,
        ScenarioCandidateFromRuntimeSource::LoadedScenarioRuntimeReportChain
    );
}

#[test]
fn scenario_candidate_from_runtime_reports_original_authority_digest() {
    let report = evaluate_fixture();
    assert!(report.original_authority_digest_before > 0);
    assert_eq!(
        report.original_authority_digest_before,
        report.original_authority_digest_after
    );
}

#[test]
fn scenario_candidate_from_runtime_preserves_original_authority() {
    assert!(
        prove_scenario_candidate_from_runtime_preserves_original_authority(
            "owner_silo_corpus",
            &load_owner_silo_fixture_json()
        )
        .expect("prove")
    );
}

#[test]
fn scenario_candidate_from_runtime_clones_candidate_scenario() {
    let json = load_owner_silo_fixture_json();
    let (scenario, _) =
        simthing_spec::load_scenario_spec_from_json_str("owner_silo_corpus", &json).expect("load");
    let candidate = clone_scenario_candidate_with_runtime_property_view(
        &scenario,
        simthing_spec::RuntimeTickId(1),
        1,
    )
    .expect("clone");
    let report = evaluate_fixture();
    assert!(report.candidate_scenario_spec_ready);
    assert_ne!(
        serialize_scenario_authority(&candidate).expect("candidate"),
        serialize_scenario_authority(&scenario).expect("original")
    );
}

#[test]
fn scenario_candidate_from_runtime_applies_property_view_rows_to_candidate_only() {
    let json = load_owner_silo_fixture_json();
    let before = json.clone();
    let report = evaluate_fixture();
    let after = load_owner_silo_fixture_json();
    assert_eq!(before, after);
    assert!(report.original_authority_preserved);
    assert!(report.mutation_record_count > 0);
}

#[test]
fn scenario_candidate_from_runtime_candidate_digest_changes_when_records_exist() {
    let report = evaluate_fixture();
    assert!(report.mutation_record_count > 0);
    assert!(report.candidate_authority_changed);
    assert_ne!(
        report.candidate_authority_digest_before,
        report.candidate_authority_digest_after
    );
    assert_eq!(
        report.candidate_authority_digest_before,
        report.original_authority_digest_before
    );
}

#[test]
fn scenario_candidate_from_runtime_records_owner_resource_scope_metadata() {
    let report = evaluate_fixture();
    assert!(!report.mutation_records.is_empty());
    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.owner_ref.is_some() && record.resource_key.is_some()));
    assert!(report.mutation_records.iter().any(|record| record
        .owner_ref
        .as_ref()
        .map(|o| o.as_str())
        == Some("owner_a")));
    assert!(report.mutation_records.iter().any(|record| {
        record.property_id == RUNTIME_PREVIEW_APPLIED_PROPERTY_ID
            || record.property_id == RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID
    }));
}

#[test]
fn scenario_candidate_from_runtime_preserves_candidate_stead_ids() {
    let report = evaluate_fixture();
    assert!(report.candidate_stead_ids_preserved);
}

#[test]
fn scenario_candidate_from_runtime_preserves_candidate_links() {
    let report = evaluate_fixture();
    assert!(report.candidate_links_preserved);
}

#[test]
fn scenario_candidate_from_runtime_preserves_candidate_spatial_tree() {
    let report = evaluate_fixture();
    assert!(report.candidate_spatial_tree_preserved);
}

#[test]
fn scenario_candidate_from_runtime_preserves_owner_metadata_not_spatial_parentage() {
    let report = evaluate_fixture();
    assert!(report.owner_metadata_not_spatial_parentage);
}

#[test]
fn scenario_candidate_from_runtime_uses_gpu_compatible_source_rows() {
    let report = evaluate_fixture();
    assert!(report.gpu_compatible_source_rows);
    assert!(report.runtime_property_view_rows_ready);
}

#[test]
fn scenario_candidate_from_runtime_cpu_candidate_serialization_only() {
    let report = evaluate_fixture();
    assert!(report.cpu_candidate_serialization_only);
}

#[test]
fn scenario_candidate_from_runtime_defers_candidate_save() {
    let report = evaluate_fixture();
    assert!(report.candidate_save_deferred);
}

#[test]
fn scenario_candidate_from_runtime_defers_savefile_history_ui_and_gpu_dispatch() {
    let report = evaluate_fixture();
    assert!(report.savefile_persistence_deferred);
    assert!(report.persistent_history_deferred);
    assert!(report.studio_ui_wiring_deferred);
    assert!(report.gpu_dispatch_deferred);
}

#[test]
fn normal_tests_do_not_write_scenario_candidate_fixtures() {
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

#[test]
fn scenario_candidate_from_runtime_preserves_surface_gridcell_tier() {
    let json = load_owner_silo_fixture_json();
    let (spec, _) = load_scenario_spec_from_json_str("orig", &json).expect("load");
    assert!(evaluate_planet_child_locations(&spec).surface_gridcell_tier_present);
    let report = evaluate_fixture();
    assert!(report.candidate_spatial_tree_preserved);
    assert!(report.candidate_stead_ids_preserved);
}
