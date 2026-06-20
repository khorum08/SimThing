//! SCENARIO-CANDIDATE-SAVE-REOPEN-0 — candidate ScenarioSpec save/reopen driver proofs.

use std::fs;
use std::path::PathBuf;

use simthing_driver::compile_scenario_candidate_save_reopen_plan_from_json_str;

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

fn compile_fixture() -> simthing_driver::ScenarioCandidateSaveReopenPlan {
    let json = load_owner_silo_fixture_json();
    compile_scenario_candidate_save_reopen_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile")
}

#[test]
fn scenario_candidate_save_reopen_compile_composes_candidate_from_runtime_plan() {
    let plan = compile_fixture();
    assert!(
        plan.candidate_from_runtime_plan
            .candidate_scenario_spec_ready
    );
    assert!(plan.save_reopen_report.candidate_from_runtime_ready);
}

#[test]
fn scenario_candidate_save_reopen_compile_reports_ready() {
    let plan = compile_fixture();
    assert!(plan.candidate_save_reopen_ready);
    assert!(plan.save_reopen_report.candidate_save_reopen_ready);
}

#[test]
fn scenario_candidate_save_reopen_compile_reports_original_authority_preserved() {
    let plan = compile_fixture();
    assert!(plan.original_authority_preserved);
}

#[test]
fn scenario_candidate_save_reopen_compile_reports_candidate_digest_stable_after_reopen() {
    let plan = compile_fixture();
    assert!(plan.candidate_digest_stable_after_reopen);
    assert!(
        plan.save_reopen_report
            .reopen_report
            .digest_matches_saved_candidate
    );
}

#[test]
fn scenario_candidate_save_reopen_compile_reports_stead_tree_projection_ready() {
    let plan = compile_fixture();
    assert!(plan.stead_tree_projection_ready);
    assert!(
        plan.save_reopen_report
            .reopen_report
            .studio_projection_rebuild_ready
    );
}

#[test]
fn scenario_candidate_save_reopen_compile_reports_canonical_scenario_json_only() {
    let plan = compile_fixture();
    assert!(plan.canonical_scenario_json_only);
}

#[test]
fn scenario_candidate_save_reopen_compile_reports_no_distinct_savefile_format() {
    let plan = compile_fixture();
    assert!(plan.no_distinct_savefile_format_introduced);
}

#[test]
fn scenario_candidate_save_reopen_compile_defers_history_ui_and_gpu_dispatch() {
    let plan = compile_fixture();
    assert!(plan.persistent_history_deferred);
    assert!(plan.studio_ui_wiring_deferred);
    assert!(plan.gpu_dispatch_deferred);
}
