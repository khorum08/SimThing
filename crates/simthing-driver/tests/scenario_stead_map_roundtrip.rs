//! SCENARIO-STEAD-MAP-ROUNDTRIP-0 — STEAD map roundtrip driver compile proofs.

use std::fs;
use std::path::PathBuf;

use simthing_driver::compile_scenario_stead_map_roundtrip_plan_from_json_str;

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

fn compile_fixture() -> simthing_driver::ScenarioSteadMapRoundtripPlan {
    let json = load_owner_silo_fixture_json();
    compile_scenario_stead_map_roundtrip_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile")
}

#[test]
fn scenario_stead_map_roundtrip_compile_composes_canonical_io_plan() {
    let plan = compile_fixture();
    assert!(
        plan.canonical_io_plan
            .roundtrip_report
            .scenario_authority_preserved
    );
    assert!(plan.canonical_io_plan.studio_import_export_ready);
}

#[test]
fn scenario_stead_map_roundtrip_compile_reports_stable_stead_ids() {
    let plan = compile_fixture();
    assert!(plan.stead_ids_stable);
    assert!(plan.stead_roundtrip_report.stead_ids_stable);
}

#[test]
fn scenario_stead_map_roundtrip_compile_reports_stable_links() {
    let plan = compile_fixture();
    assert!(plan.links_stable);
}

#[test]
fn scenario_stead_map_roundtrip_compile_reports_stable_spatial_tree() {
    let plan = compile_fixture();
    assert!(plan.spatial_tree_stable);
}

#[test]
fn scenario_stead_map_roundtrip_compile_reports_stable_rf_metadata() {
    let plan = compile_fixture();
    assert!(plan.rf_metadata_stable);
}

#[test]
fn scenario_stead_map_roundtrip_compile_reports_studio_projection_rebuild_ready() {
    let plan = compile_fixture();
    assert!(plan.studio_projection_rebuild_ready);
}

#[test]
fn scenario_stead_map_roundtrip_compile_defers_runtime_mutation() {
    let plan = compile_fixture();
    assert!(plan.runtime_mutation_deferred);
    assert!(plan.canonical_io_plan.runtime_mutation_deferred);
}

#[test]
fn scenario_stead_map_roundtrip_compile_defers_savefile_persistence() {
    let plan = compile_fixture();
    assert!(plan.savefile_persistence_deferred);
    assert!(plan.canonical_io_plan.savefile_persistence_deferred);
}
