//! PLANET-CHILD-LOCATION-ADMISSION-0 — Studio planet child-location display proofs.

use std::fs;
use std::path::PathBuf;

use simthing_mapeditor::{
    load_studio_session_from_scenario_path, studio_apply_planet_child_location_command,
    StudioPlanetChildView,
};
use simthing_spec::{
    ingest_scenario_from_str, serialize_scenario_authority, studio_canonical_ingestion_profile,
    PlanetChildLocationCommand, PlanetChildLocationEditErrorKind, ScenarioIngestionClassification,
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
fn studio_displays_planet_child_under_star_system() {
    let session = load_studio_session_from_scenario_path(&admitted_fixture_path(), None)
        .expect("load admitted");
    assert_eq!(session.scenario_document.planets.len(), 1);
    let planet = &session.scenario_document.planets[0];
    assert_eq!(planet.planet_id, "terra_prime");
    assert_eq!(planet.display_name.as_deref(), Some("Terra Prime"));
    assert_eq!(planet.role, "planet");
    assert_eq!(planet.parent_location_id.as_deref(), Some("cell_b"));
}

#[test]
fn studio_planet_child_display_preserves_gridcell_projection() {
    let session =
        load_studio_session_from_scenario_path(&admitted_fixture_path(), None).expect("load");
    assert_eq!(session.scenario_document.gridcells.len(), 2);
    assert_eq!(session.structural_projection.location_indices.len(), 2);
    let planet: &StudioPlanetChildView = &session.scenario_document.planets[0];
    assert_eq!(planet.structural_col, Some(1));
    assert_eq!(planet.structural_row, Some(0));
}

#[test]
fn studio_add_planet_child_rebuilds_document_and_admission() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    let outcome = studio_apply_planet_child_location_command(
        &mut session,
        PlanetChildLocationCommand::AddPlanet {
            star_system_gridcell_id: "cell_b".into(),
            planet_id: "studio_planet".into(),
            display_name: Some("Studio Planet".into()),
        },
    )
    .expect("add");
    assert_eq!(outcome.planet_count, 1);
    assert_eq!(session.scenario_document.planets.len(), 1);
    assert!(session
        .admission_summary
        .deferrals
        .iter()
        .any(|d| d.kind == "PlanetSimulationDeferred"));
}

#[test]
fn studio_rejects_planet_under_inert_gridcell_without_partial_mutation() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    let before = serialize_scenario_authority(&session.scenario_authority).expect("snap");
    let err = studio_apply_planet_child_location_command(
        &mut session,
        PlanetChildLocationCommand::AddPlanet {
            star_system_gridcell_id: "cell_a".into(),
            planet_id: "bad".into(),
            display_name: None,
        },
    )
    .unwrap_err();
    let inner = match err {
        simthing_mapeditor::StudioPlanetChildLocationError::PlanetEdit(inner) => inner,
        _ => panic!("expected planet edit error"),
    };
    assert_eq!(
        inner.kind,
        PlanetChildLocationEditErrorKind::PlanetUnderInertGridcell
    );
    let after = serialize_scenario_authority(&session.scenario_authority).expect("snap");
    assert_eq!(before, after);
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

#[test]
fn studio_under_inert_fixture_ingestion_rejected() {
    let json = fs::read_to_string(under_inert_fixture_path()).expect("under inert corpus");
    let (result, _) =
        ingest_scenario_from_str("under_inert", &json, studio_canonical_ingestion_profile());
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
}
