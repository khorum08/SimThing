//! SCENARIO-CANDIDATE-FROM-RUNTIME-0 — candidate ScenarioSpec driver proofs.

use std::fs;
use std::path::PathBuf;

use simthing_driver::compile_scenario_candidate_from_runtime_plan_from_json_str;

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

fn compile_fixture() -> simthing_driver::ScenarioCandidateFromRuntimePlan {
    let json = load_owner_silo_fixture_json();
    compile_scenario_candidate_from_runtime_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile")
}

#[test]
fn scenario_candidate_from_runtime_compile_composes_runtime_report_chain_plan() {
    let plan = compile_fixture();
    assert!(plan.runtime_report_chain_plan.full_chain_ready);
    assert!(
        plan.runtime_report_chain_plan
            .runtime_property_view_rows_ready
    );
}

#[test]
fn scenario_candidate_from_runtime_compile_reports_candidate_ready() {
    let plan = compile_fixture();
    assert!(plan.candidate_scenario_spec_ready);
    assert!(plan.candidate_report.candidate_scenario_spec_ready);
}

#[test]
fn scenario_candidate_from_runtime_compile_reports_original_authority_preserved() {
    let plan = compile_fixture();
    assert!(plan.original_authority_preserved);
}

#[test]
fn scenario_candidate_from_runtime_compile_reports_candidate_digest_changed() {
    let plan = compile_fixture();
    assert!(plan.candidate_authority_changed);
}

#[test]
fn scenario_candidate_from_runtime_compile_reports_mutation_record_count() {
    let plan = compile_fixture();
    assert!(plan.mutation_record_count > 0);
}

#[test]
fn scenario_candidate_from_runtime_compile_reports_gpu_compatible_source_rows() {
    let plan = compile_fixture();
    assert!(plan.gpu_compatible_source_rows);
}

#[test]
fn scenario_candidate_from_runtime_compile_reports_cpu_candidate_serialization_only() {
    let plan = compile_fixture();
    assert!(plan.cpu_candidate_serialization_only);
}

#[test]
fn scenario_candidate_from_runtime_compile_defers_candidate_save() {
    let plan = compile_fixture();
    assert!(plan.candidate_save_deferred);
}

#[test]
fn scenario_candidate_from_runtime_compile_defers_savefile_history_ui_and_gpu_dispatch() {
    let plan = compile_fixture();
    assert!(plan.savefile_persistence_deferred);
    assert!(plan.persistent_history_deferred);
    assert!(plan.studio_ui_wiring_deferred);
    assert!(plan.gpu_dispatch_deferred);
}
