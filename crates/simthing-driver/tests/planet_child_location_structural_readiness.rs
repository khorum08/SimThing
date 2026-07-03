//! RECURSIVE-SPATIAL-GRID-DEFAULTS-0 — driver structural readiness with recursive spatial grids.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use simthing_driver::{
    compile_structural_n4_theater, evaluate_scenario_compile_readiness, StructuralTheaterAdmission,
};
use simthing_spec::{
    collect_local_receiver_cells, deserialize_scenario_authority, evaluate_planet_child_locations,
    ingest_scenario, ingest_scenario_from_str, MappingExecutionProfile,
    PlanetChildLocationAdmissionErrorKind, ScenarioIngestionClassification,
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
fn planet_local_gridcell_not_counted_as_galaxy_structural_gridcell() {
    let spec = load_admitted();
    assert_eq!(spec.structural_grid.placements.len(), 2);
    let report = evaluate_planet_child_locations(&spec);
    assert_eq!(report.planet_gridcell_count, 1);
    assert_eq!(report.local_gridcell_count, 1);
    assert!(report.errors.is_empty());
}

#[test]
fn inert_receiver_cell_not_counted_as_galaxy_structural_gridcell() {
    let spec = load_admitted();
    let placement_ids: BTreeSet<u32> = spec
        .structural_grid
        .placements
        .iter()
        .map(|p| p.simthing_id_raw)
        .collect();
    assert_eq!(placement_ids.len(), 2);

    let receivers = collect_local_receiver_cells(&spec);
    assert_eq!(receivers.len(), 1);
    assert!(receivers[0].is_implicit);
    if let Some(raw) = receivers[0].materialized_simthing_id_raw {
        assert!(
            !placement_ids.contains(&raw),
            "materialized receiver cells must not pollute GalaxyMap structural_grid"
        );
    }

    let report = evaluate_planet_child_locations(&spec);
    assert_eq!(report.receiver_cell_count, 1);
    assert_eq!(report.implicit_receiver_cell_count, 1);
    assert!(report.errors.is_empty());
}

#[test]
fn star_system_local_grid_operator_deferred_without_new_gpu_primitive() {
    let ingestion_compile = include_str!("../src/scenario_ingestion_compile.rs");
    let theater_compile = include_str!("../src/structural_n4_theater_compile.rs");
    let mapping_compile = include_str!("../src/mapping_plan_compile.rs");
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");

    assert!(
        ingestion_compile.contains("no new GPU primitives"),
        "driver compile readiness must document deferred GPU primitives"
    );
    assert!(
        ingestion_compile.contains("structural N4 theater compile surfaces only"),
        "driver must remain on structural N4 theater surfaces only"
    );

    for forbidden in [
        "star_system_local_grid_operator",
        "compile_star_system_local_grid",
        "LocalGridOperator",
        "receiver_cell_operator",
        "collect_local_receiver_cells",
    ] {
        assert!(
            !theater_compile.contains(forbidden) && !mapping_compile.contains(forbidden),
            "driver compile surface must not implement `{forbidden}` yet"
        );
    }

    for forbidden in [
        "star_system_local_grid",
        "receiver_cell_shader",
        "local_grid_operator",
    ] {
        assert!(
            !gpu_lib.contains(forbidden),
            "simthing-gpu must not add `{forbidden}` primitive for deferred local-grid operator"
        );
    }
}
#[test]
fn planet_local_gridcells_do_not_expand_structural_grid_placements() {
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
