//! LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 — loaded scenario session envelope driver proofs.

use std::fs;
use std::path::PathBuf;

use simthing_driver::compile_loaded_scenario_studio_session_envelope_plan_from_json_str;

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

fn compile_fixture() -> simthing_driver::LoadedScenarioStudioSessionEnvelopePlan {
    let json = load_owner_silo_fixture_json();
    compile_loaded_scenario_studio_session_envelope_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile")
}

#[test]
fn loaded_scenario_session_envelope_compile_composes_canonical_io_plan() {
    let plan = compile_fixture();
    assert!(
        plan.canonical_io_plan
            .roundtrip_report
            .scenario_authority_preserved
    );
    assert!(plan.canonical_io_plan.studio_import_export_ready);
}

#[test]
fn loaded_scenario_session_envelope_compile_composes_stead_map_plan() {
    let plan = compile_fixture();
    assert!(plan.stead_map_roundtrip_plan.stead_ids_stable);
    assert!(plan.stead_map_roundtrip_plan.spatial_tree_stable);
}

#[test]
fn loaded_scenario_session_envelope_compile_reports_import_export_ready() {
    let plan = compile_fixture();
    assert!(plan.scenario_import_ready);
    assert!(plan.scenario_export_ready);
    assert!(plan.session_envelope.authority.scenario_import_ready);
}

#[test]
fn loaded_scenario_session_envelope_compile_reports_projection_rebuild_ready() {
    let plan = compile_fixture();
    assert!(plan.studio_projection_rebuild_ready);
}

#[test]
fn loaded_scenario_session_envelope_compile_reports_recursive_rf_prerequisites_ready() {
    let plan = compile_fixture();
    assert!(plan.recursive_rf_prerequisites_ready);
}

#[test]
fn loaded_scenario_session_envelope_compile_defers_runtime_mutation() {
    let plan = compile_fixture();
    assert!(plan.runtime_mutation_deferred);
    assert!(plan.runtime_tick_execution_deferred);
}

#[test]
fn loaded_scenario_session_envelope_compile_defers_savefile_persistence() {
    let plan = compile_fixture();
    assert!(plan.savefile_persistence_deferred);
    assert!(plan.persistent_history_deferred);
}

#[test]
fn loaded_scenario_session_envelope_compile_preserves_non_authority_surface_flags() {
    let plan = compile_fixture();
    assert!(!plan.session_envelope.studio_config_is_authority);
    assert!(!plan.session_envelope.bevy_state_is_authority);
    assert!(!plan.session_envelope.gpu_buffers_are_authority);
    assert!(!plan.session_envelope.runtime_reports_are_authority);
    assert!(plan.studio_ui_wiring_deferred);
    assert!(plan.gpu_dispatch_deferred);
}
