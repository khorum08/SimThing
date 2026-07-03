//! RECURSIVE-SPATIAL-GRID-DEFAULTS-0 — Studio recursive spatial local-grid display proofs.

use std::fs;
use std::path::PathBuf;

use simthing_mapeditor::{
    load_studio_session_from_scenario_path, studio_apply_planet_child_location_command,
    StudioPlanetChildView,
};
use simthing_spec::{
    ingest_scenario_from_str, serialize_scenario_authority, studio_canonical_ingestion_profile,
    PlanetChildLocationEditErrorKind, PlanetLocalGridCommand, ScenarioIngestionClassification,
    LOCAL_GRIDCELL_ROLE_RECEIVER, LOCAL_GRID_DEFAULT_COLS, LOCAL_GRID_DEFAULT_ROWS,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};

fn admitted_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/planet_child_location_admitted.simthing-scenario.json")
}

fn under_inert_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../scenarios/corpus/planet_child_location_under_inert_rejected.simthing-scenario.json",
    )
}

fn galaxymap_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json")
}

#[test]
fn studio_displays_planet_as_star_system_local_gridcell() {
    let session = load_studio_session_from_scenario_path(&admitted_fixture_path(), None)
        .expect("load admitted");
    assert_eq!(session.scenario_document.planets.len(), 1);
    let planet = &session.scenario_document.planets[0];
    assert_eq!(planet.planet_id, "terra_prime");
    assert_eq!(planet.display_name.as_deref(), Some("Terra Prime"));
    assert_eq!(planet.local_role, "planet");
    assert_eq!(
        planet.parent_star_system_location_id.as_deref(),
        Some("cell_b")
    );
}

#[test]
fn studio_displays_star_system_local_10x10_frame() {
    let session =
        load_studio_session_from_scenario_path(&admitted_fixture_path(), None).expect("load");
    let planet: &StudioPlanetChildView = &session.scenario_document.planets[0];
    assert_eq!(planet.local_frame_cols, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS);
    assert_eq!(planet.local_frame_rows, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS);
}

#[test]
fn studio_displays_planet_non_grid_children() {
    let session = load_studio_session_from_scenario_path(&admitted_fixture_path(), None)
        .expect("load admitted");
    assert_eq!(session.scenario_document.planet_non_grid_children.len(), 3);
    let kinds: Vec<&str> = session
        .scenario_document
        .planet_non_grid_children
        .iter()
        .map(|c| c.child_kind_label.as_str())
        .collect();
    assert!(kinds.contains(&"cohort"));
    assert!(kinds.contains(&"fleet"));
    assert!(kinds.contains(&"Infrastructure"));
    assert!(session
        .scenario_document
        .planet_non_grid_children
        .iter()
        .all(|c| c.planet_id == "terra_prime"));
}

#[test]
fn studio_displays_planet_default_1x1_interior_grid() {
    let session =
        load_studio_session_from_scenario_path(&admitted_fixture_path(), None).expect("load");
    let planet = &session.scenario_document.planets[0];
    assert_eq!(planet.interior_frame_cols, LOCAL_GRID_DEFAULT_COLS);
    assert_eq!(planet.interior_frame_rows, LOCAL_GRID_DEFAULT_ROWS);
}

#[test]
fn studio_displays_inert_gridcell_1x1_receiver_cell() {
    let session =
        load_studio_session_from_scenario_path(&admitted_fixture_path(), None).expect("load");
    let receivers = &session.scenario_document.receiver_cells;
    assert_eq!(receivers.len(), 1);
    let inert_receiver = receivers
        .iter()
        .find(|r| r.parent_location_id.as_deref() == Some("cell_a"))
        .expect("implicit receiver for admitted fixture inert cell_a");
    assert_eq!(
        inert_receiver.parent_local_frame_cols,
        LOCAL_GRID_DEFAULT_COLS
    );
    assert_eq!(
        inert_receiver.parent_local_frame_rows,
        LOCAL_GRID_DEFAULT_ROWS
    );
    assert_eq!(inert_receiver.local_col, 0);
    assert_eq!(inert_receiver.local_row, 0);
    assert_eq!(inert_receiver.local_role, LOCAL_GRIDCELL_ROLE_RECEIVER);
    assert!(inert_receiver.is_implicit_receiver);
    assert!(inert_receiver.materialized_simthing_id_raw.is_none());
}

#[test]
fn studio_displays_planet_local_col_row() {
    let session =
        load_studio_session_from_scenario_path(&admitted_fixture_path(), None).expect("load");
    let planet = &session.scenario_document.planets[0];
    assert_eq!(planet.local_col, Some(0));
    assert_eq!(planet.local_row, Some(0));
}
#[test]
fn studio_planet_display_does_not_dispatch_gpu() {
    let src = include_str!("../src/studio_planet_child_location.rs");
    let lib = include_str!("../src/lib.rs");
    for forbidden in [
        "SimGpuAccumulatorTickState",
        "compile_owner_silo_gpu_tick_plan",
        "gpu_context_blocking",
        "AccumulatorOpSession",
    ] {
        assert!(
            !src.contains(forbidden) && !lib.contains(forbidden),
            "{forbidden}"
        );
    }
}

#[test]
fn studio_planet_display_does_not_call_sim_tick() {
    let src = include_str!("../src/studio_planet_child_location.rs");
    for forbidden in [
        "simthing_sim",
        "SimTickError",
        "execute_accumulator_plan_tick",
    ] {
        assert!(!src.contains(forbidden), "{forbidden}");
    }
}
