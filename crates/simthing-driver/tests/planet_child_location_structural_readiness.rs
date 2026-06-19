//! PLANET-CHILD-LOCATION-ADMISSION-0 — driver structural readiness with planet children.

use std::fs;
use std::path::PathBuf;

use simthing_driver::{
    compile_structural_n4_theater, evaluate_scenario_compile_readiness, StructuralTheaterAdmission,
};
use simthing_spec::{
    deserialize_scenario_authority, evaluate_planet_child_locations, ingest_scenario,
    ingest_scenario_from_str, MappingExecutionProfile, ScenarioIngestionClassification,
    ScenarioIngestionProfile, SimThingScenarioSpec,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_admitted() -> SimThingScenarioSpec {
    let json = fs::read_to_string(corpus_path(
        "planet_child_location_admitted.simthing-scenario.json",
    ))
    .expect("admitted corpus");
    deserialize_scenario_authority(&json).expect("parse")
}

#[test]
fn planet_child_scenario_reaches_structural_n4_admission() {
    let spec = load_admitted();
    let readiness = evaluate_scenario_compile_readiness(&spec);
    assert!(readiness.structural_n4_ready);
    let admission =
        compile_structural_n4_theater(&spec, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("compile");
    assert!(matches!(admission, StructuralTheaterAdmission::Admit(_)));
}

#[test]
fn planet_child_not_counted_as_structural_gridcell() {
    let spec = load_admitted();
    assert_eq!(spec.structural_grid.placements.len(), 2);
    let report = evaluate_planet_child_locations(&spec);
    assert_eq!(report.planet_count, 1);
    assert!(report.errors.is_empty());
}

#[test]
fn invalid_planet_under_inert_gridcell_does_not_reach_driver_compile() {
    let json = fs::read_to_string(corpus_path(
        "planet_child_location_under_inert_rejected.simthing-scenario.json",
    ))
    .expect("under inert corpus");
    let profile = ScenarioIngestionProfile {
        require_canonical_tree: true,
        admit_legacy_world_root: true,
    };
    let (result, spec) = ingest_scenario_from_str("under_inert", &json, profile);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    let report = result.planet_child_location.expect("planet report");
    assert!(report.errors.iter().any(|e| e.message.contains("inert")));
}

#[test]
fn planet_child_locations_do_not_expand_structural_grid_placements() {
    let spec = load_admitted();
    let before = spec.structural_grid.placements.len();
    let _ = ingest_scenario(
        "planet_grid",
        &spec,
        ScenarioIngestionProfile {
            require_canonical_tree: true,
            admit_legacy_world_root: true,
        },
    );
    assert_eq!(spec.structural_grid.placements.len(), before);
}
