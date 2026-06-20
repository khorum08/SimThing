//! SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 — canonical ScenarioSpec I/O driver proofs.

use std::fs;
use std::path::PathBuf;

use simthing_driver::compile_scenario_canonical_io_plan_from_json_str;

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

#[test]
fn scenario_canonical_io_compile_loads_and_roundtrips_fixture() {
    let json = load_owner_silo_fixture_json();
    let plan = compile_scenario_canonical_io_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile");

    assert!(plan.roundtrip_report.scenario_authority_preserved);
    assert!(plan.roundtrip_report.initial_load.loaded);
    assert!(plan.roundtrip_report.roundtrip_load.loaded);
}

#[test]
fn scenario_canonical_io_compile_reports_studio_import_export_ready() {
    let json = load_owner_silo_fixture_json();
    let plan = compile_scenario_canonical_io_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile");

    assert!(plan.studio_import_export_ready);
}

#[test]
fn scenario_canonical_io_compile_defers_runtime_mutation() {
    let json = load_owner_silo_fixture_json();
    let plan = compile_scenario_canonical_io_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile");

    assert!(plan.runtime_mutation_deferred);
}

#[test]
fn scenario_canonical_io_compile_defers_savefile_persistence() {
    let json = load_owner_silo_fixture_json();
    let plan = compile_scenario_canonical_io_plan_from_json_str("owner_silo_driver", &json)
        .expect("compile");

    assert!(plan.savefile_persistence_deferred);
}
