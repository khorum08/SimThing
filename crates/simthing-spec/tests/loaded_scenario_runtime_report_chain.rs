//! LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 — loaded scenario runtime report chain proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_spec::{
    evaluate_loaded_scenario_recursive_rf_runtime_from_json_str,
    evaluate_loaded_scenario_runtime_report_chain_from_json_str,
    prove_loaded_scenario_runtime_report_chain_preserves_authority,
    LoadedScenarioRuntimeReportChainSource, SpecError,
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

fn evaluate_fixture() -> simthing_spec::LoadedScenarioRuntimeReportChainReport {
    let json = load_owner_silo_fixture_json();
    evaluate_loaded_scenario_runtime_report_chain_from_json_str("owner_silo_corpus", &json)
        .expect("evaluate")
}

fn stage<'a>(
    report: &'a simthing_spec::LoadedScenarioRuntimeReportChainReport,
    stage_id: &str,
) -> &'a simthing_spec::LoadedScenarioRuntimeReportChainStage {
    report
        .stages
        .iter()
        .find(|stage| stage.stage_id == stage_id)
        .unwrap_or_else(|| panic!("missing stage {stage_id}"))
}

#[test]
fn loaded_scenario_runtime_report_chain_composes_recursive_rf_runtime() {
    let json = load_owner_silo_fixture_json();
    let recursive =
        evaluate_loaded_scenario_recursive_rf_runtime_from_json_str("owner_silo_corpus", &json)
            .expect("recursive");
    let chain = evaluate_fixture();
    assert!(chain.recursive_rf_runtime_ready);
    assert_eq!(
        chain.scenario_authority_digest,
        recursive.scenario_authority_digest
    );
    assert_eq!(
        chain.source,
        LoadedScenarioRuntimeReportChainSource::LoadedScenarioRecursiveRfRuntime
    );
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_authority_digest() {
    let chain = evaluate_fixture();
    assert!(chain.scenario_authority_digest > 0);
    assert!(chain.loaded_session_envelope_ready);
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_owner_silo_stage_ready() {
    let chain = evaluate_fixture();
    assert!(chain.owner_silo_ready);
    let stage = stage(&chain, "owner_silo_disburse_down");
    assert!(stage.ready);
    assert!(stage.record_count > 0);
    assert!(stage.report_only);
    assert!(stage.mutation_deferred);
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_local_allocation_stage_ready() {
    let chain = evaluate_fixture();
    assert!(chain.local_allocation_ready);
    assert!(stage(&chain, "local_allocation").ready);
    assert!(stage(&chain, "local_allocation").record_count > 0);
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_local_effects_stage_ready() {
    let chain = evaluate_fixture();
    assert!(chain.local_effects_ready);
    assert!(stage(&chain, "local_effects").ready);
    assert!(stage(&chain, "local_effects").record_count > 0);
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_semantic_projection_stage_ready() {
    let chain = evaluate_fixture();
    assert!(chain.semantic_projection_ready);
    assert!(stage(&chain, "semantic_projection").ready);
    assert!(stage(&chain, "semantic_projection").record_count > 0);
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_semantic_execution_records_stage_ready() {
    let chain = evaluate_fixture();
    assert!(chain.semantic_execution_records_ready);
    assert!(stage(&chain, "semantic_execution_records").ready);
    assert!(stage(&chain, "semantic_execution_records").record_count > 0);
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_semantic_delta_preview_stage_ready() {
    let chain = evaluate_fixture();
    assert!(chain.semantic_delta_preview_ready);
    assert!(stage(&chain, "semantic_delta_preview").ready);
    assert!(stage(&chain, "semantic_delta_preview").record_count > 0);
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_runtime_participant_state_rows_ready() {
    let chain = evaluate_fixture();
    assert!(chain.runtime_participant_state_rows_ready);
    assert!(stage(&chain, "runtime_participant_state_rows").ready);
    assert!(stage(&chain, "runtime_participant_state_rows").record_count > 0);
}

#[test]
fn loaded_scenario_runtime_report_chain_reports_runtime_property_view_rows_ready() {
    let chain = evaluate_fixture();
    assert!(chain.runtime_property_view_rows_ready);
    assert!(stage(&chain, "runtime_property_view_rows").ready);
    assert!(stage(&chain, "runtime_property_view_rows").record_count > 0);
}

#[test]
fn loaded_scenario_runtime_report_chain_is_explicit_report_mode_only() {
    let chain = evaluate_fixture();
    assert!(chain.explicit_runtime_report_mode_only);
    assert!(chain
        .stages
        .iter()
        .all(|s| s.report_only && s.mutation_deferred));
}

#[test]
fn loaded_scenario_runtime_report_chain_emits_gpu_compatible_surface() {
    let chain = evaluate_fixture();
    assert!(chain.gpu_compatible_row_table_surface);
}

#[test]
fn loaded_scenario_runtime_report_chain_preserves_scenario_authority() {
    assert!(
        prove_loaded_scenario_runtime_report_chain_preserves_authority(
            "owner_silo_corpus",
            &load_owner_silo_fixture_json()
        )
        .expect("prove")
    );
}

#[test]
fn loaded_scenario_runtime_report_chain_defers_candidate_mutation() {
    let chain = evaluate_fixture();
    assert!(chain.scenario_authority_mutation_deferred);
    assert!(chain.runtime_mutation_deferred);
    assert!(chain.semantic_execution_deferred);
}

#[test]
fn loaded_scenario_runtime_report_chain_defers_savefile_history_ui_and_gpu_dispatch() {
    let chain = evaluate_fixture();
    assert!(chain.savefile_persistence_deferred);
    assert!(chain.persistent_history_deferred);
    assert!(chain.studio_ui_wiring_deferred);
    assert!(chain.gpu_dispatch_deferred);
}

#[test]
fn normal_tests_do_not_write_loaded_scenario_runtime_chain_fixtures() {
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
