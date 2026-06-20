//! LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 — loaded scenario recursive RF runtime driver proofs.

use std::fs;
use std::path::PathBuf;

use simthing_driver::compile_loaded_scenario_recursive_rf_runtime_plan_from_json_str;

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

fn compile_fixture() -> simthing_driver::LoadedScenarioRecursiveRfRuntimePlan {
    let json = load_owner_silo_fixture_json();
    compile_loaded_scenario_recursive_rf_runtime_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile")
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_composes_session_envelope_plan() {
    let plan = compile_fixture();
    assert!(
        plan.session_envelope_plan
            .session_envelope
            .authority
            .scenario_import_ready
    );
    assert!(plan.session_envelope_plan.recursive_rf_prerequisites_ready);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_reports_recursive_rf_ready() {
    let plan = compile_fixture();
    assert!(plan.recursive_rf_runtime_ready);
    assert!(plan.recursive_rf_runtime_report.recursive_rf_runtime_ready);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_reports_local_parent_node_resolution_first() {
    let plan = compile_fixture();
    assert!(plan.local_parent_node_resolution_first);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_reports_sibling_settlement_before_upward_bubbling()
{
    let plan = compile_fixture();
    assert!(plan.sibling_settlement_before_upward_bubbling);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_reports_owner_scope_not_spatial_parentage() {
    let plan = compile_fixture();
    assert!(plan.owner_scope_not_spatial_parentage);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_reports_gpu_compatible_rows() {
    let plan = compile_fixture();
    assert!(plan.gpu_compatible_row_table_surface);
    assert!(!plan.recursive_rf_runtime_report.participant_rows.is_empty());
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_reports_cpu_oracle_only() {
    let plan = compile_fixture();
    assert!(plan.cpu_oracle_only);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_defers_runtime_mutation() {
    let plan = compile_fixture();
    assert!(plan.runtime_mutation_deferred);
    assert!(plan.scenario_authority_mutation_deferred);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_compile_defers_savefile_history_ui_and_gpu_dispatch() {
    let plan = compile_fixture();
    assert!(plan.savefile_persistence_deferred);
    assert!(plan.persistent_history_deferred);
    assert!(plan.studio_ui_wiring_deferred);
    assert!(plan.gpu_dispatch_deferred);
}
