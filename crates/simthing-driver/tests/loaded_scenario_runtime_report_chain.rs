//! LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 — loaded scenario runtime report chain driver proofs.

use std::fs;
use std::path::PathBuf;

use simthing_driver::compile_loaded_scenario_runtime_report_chain_plan_from_json_str;

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

fn compile_fixture() -> simthing_driver::LoadedScenarioRuntimeReportChainPlan {
    let json = load_owner_silo_fixture_json();
    compile_loaded_scenario_runtime_report_chain_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile")
}

#[test]
fn loaded_scenario_runtime_report_chain_compile_composes_recursive_rf_runtime_plan() {
    let plan = compile_fixture();
    assert!(plan.recursive_rf_runtime_plan.recursive_rf_runtime_ready);
    assert!(
        plan.recursive_rf_runtime_plan
            .recursive_rf_runtime_report
            .recursive_rf_runtime_ready
    );
}

#[test]
fn loaded_scenario_runtime_report_chain_compile_reports_full_chain_ready() {
    let plan = compile_fixture();
    assert!(plan.full_chain_ready);
}

#[test]
fn loaded_scenario_runtime_report_chain_compile_reports_owner_silo_ready() {
    let plan = compile_fixture();
    assert!(plan.owner_silo_ready);
}

#[test]
fn loaded_scenario_runtime_report_chain_compile_reports_all_runtime_stages_ready() {
    let plan = compile_fixture();
    assert!(plan.local_allocation_ready);
    assert!(plan.local_effects_ready);
    assert!(plan.semantic_projection_ready);
    assert!(plan.semantic_execution_records_ready);
    assert!(plan.semantic_delta_preview_ready);
    assert!(plan.runtime_participant_state_rows_ready);
    assert!(plan.runtime_property_view_rows_ready);
}

#[test]
fn loaded_scenario_runtime_report_chain_compile_reports_gpu_compatible_surface() {
    let plan = compile_fixture();
    assert!(plan.gpu_compatible_row_table_surface);
}

#[test]
fn loaded_scenario_runtime_report_chain_compile_reports_report_mode_only() {
    let plan = compile_fixture();
    assert!(plan.explicit_runtime_report_mode_only);
}

#[test]
fn loaded_scenario_runtime_report_chain_compile_defers_candidate_mutation() {
    let plan = compile_fixture();
    assert!(plan.scenario_authority_mutation_deferred);
    assert!(plan.runtime_mutation_deferred);
}

#[test]
fn loaded_scenario_runtime_report_chain_compile_defers_savefile_history_ui_and_gpu_dispatch() {
    let plan = compile_fixture();
    assert!(plan.savefile_persistence_deferred);
    assert!(plan.persistent_history_deferred);
    assert!(plan.studio_ui_wiring_deferred);
    assert!(plan.gpu_dispatch_deferred);
}
