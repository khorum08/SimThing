//! PLANET-CHILD-RF-REDUCE-UP-0 — scoped reduce-up admission proofs.

use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_owner_silo_metadata, apply_participant_owner_flow_metadata,
    apply_scenario_metadata_to_root, apply_star_system_local_grid_frame_metadata,
    deserialize_scenario_authority, evaluate_planet_child_locations,
    evaluate_planet_child_rf_reduce_up, is_surface_gridcell, make_galaxy_map, make_owner_entity,
    make_planet_gridcell, serialize_scenario_authority, structural_property_value_u32,
    PlanetChildRfAdmissionClassification, PlanetChildRfAdmissionErrorKind, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY, PLANET_ID_PROPERTY_ID,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn make_gridcell(role: &str, system_id: u32, col: u32, row: u32) -> SimThing {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut cell, role);
    cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(col),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(row),
    );
    cell.add_child(SimThing::new(SimThingKind::Cohort, 0));
    cell
}

fn build_planet_child_rf_reduce_up_scoped_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "PLANET-CHILD-RF-REDUCE-UP-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "planet_child_rf_reduce_up".into(),
    };
    apply_scenario_metadata_to_root(
        &mut root,
        "planet_child_rf_reduce_up_scoped",
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );

    let mut owner_a = make_owner_entity("owner_a", "Owner A", "player");
    apply_owner_silo_metadata(&mut owner_a, 50, Some(100));
    let mut owner_b = make_owner_entity("owner_b", "Owner B", "player");
    apply_owner_silo_metadata(&mut owner_b, 40, Some(80));

    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(owner_a);
    game_session.add_child(owner_b);

    let mut galaxy_map = make_galaxy_map("galaxy_a", "Galaxy A");

    let inert = make_gridcell(GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0);
    let inert_raw = inert.id.raw();

    let mut star_system = make_gridcell(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, 2, 1, 0);
    apply_star_system_local_grid_frame_metadata(
        &mut star_system,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
    );
    let star_raw = star_system.id.raw();

    let mut terra_prime = make_planet_gridcell("terra_prime", 0, 0, Some("Terra Prime"));
    let mut terra_cohort = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut terra_cohort, "owner_a", 15, 0);
    let mut terra_fleet = SimThing::new(SimThingKind::Fleet, 0);
    apply_participant_owner_flow_metadata(&mut terra_fleet, "owner_a", 0, 8);
    let mut terra_infra = SimThing::new(SimThingKind::Custom("Infrastructure".into()), 0);
    apply_participant_owner_flow_metadata(&mut terra_infra, "owner_a", 5, 0);
    terra_prime
        .children
        .iter_mut()
        .find(|c| is_surface_gridcell(c))
        .expect("surface gridcell")
        .add_child(terra_cohort);
    terra_prime
        .children
        .iter_mut()
        .find(|c| is_surface_gridcell(c))
        .expect("surface gridcell")
        .add_child(terra_fleet);
    terra_prime
        .children
        .iter_mut()
        .find(|c| is_surface_gridcell(c))
        .expect("surface gridcell")
        .add_child(terra_infra);

    let mut border_moon = make_planet_gridcell("border_moon", 1, 0, Some("Border Moon"));
    let mut moon_cohort = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut moon_cohort, "owner_b", 7, 2);
    border_moon
        .children
        .iter_mut()
        .find(|c| is_surface_gridcell(c))
        .expect("surface gridcell")
        .add_child(moon_cohort);

    star_system.add_child(terra_prime);
    star_system.add_child(border_moon);

    galaxy_map.add_child(inert);
    galaxy_map.add_child(star_system);
    let map_raw = galaxy_map.id.raw();
    game_session.add_child(galaxy_map);
    root.add_child(game_session);

    let mut spec = SimThingScenarioSpec {
        scenario_id: "planet_child_rf_reduce_up_scoped".into(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 2,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![
                SimThingStructuralGridPlacement {
                    location_id: "cell_a".into(),
                    target_id: "cell_a".into(),
                    system_id: 1,
                    row: 0,
                    col: 0,
                    simthing_id_raw: inert_raw,
                },
                SimThingStructuralGridPlacement {
                    location_id: "cell_b".into(),
                    target_id: "cell_b".into(),
                    system_id: 2,
                    row: 0,
                    col: 1,
                    simthing_id_raw: star_raw,
                },
            ],
        },
        links: Vec::new(),
        provenance,
    };
    spec.sync_sidecar_from_root_metadata();
    spec
}

fn terra_prime_bucket(
    report: &simthing_spec::PlanetChildRfReduceUpReport,
) -> &simthing_spec::PlanetChildRfReduceUpBucket {
    report
        .buckets
        .iter()
        .find(|b| b.scope.planet_id.as_deref() == Some("terra_prime"))
        .expect("terra_prime bucket")
}

fn border_moon_bucket(
    report: &simthing_spec::PlanetChildRfReduceUpReport,
) -> &simthing_spec::PlanetChildRfReduceUpBucket {
    report
        .buckets
        .iter()
        .find(|b| b.scope.planet_id.as_deref() == Some("border_moon"))
        .expect("border_moon bucket")
}

#[test]
fn planet_child_rf_reduce_up_rejects_rejected_participant_admission() {
    let mut spec = build_planet_child_rf_reduce_up_scoped_spec();
    let star = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .unwrap();
    let planet = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap();
    planet.properties.remove(&PLANET_ID_PROPERTY_ID);

    let report = evaluate_planet_child_rf_reduce_up(&spec);
    assert_eq!(
        report.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
}

#[test]
fn planet_child_rf_reduce_up_groups_by_owner_channel() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let report = evaluate_planet_child_rf_reduce_up(&spec);
    assert_eq!(report.bucket_count, 2);
    assert!(report
        .buckets
        .iter()
        .all(|b| b.scope.resource_key.as_str() == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn planet_child_rf_reduce_up_keeps_two_owners_separate_in_same_star_system() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let report = evaluate_planet_child_rf_reduce_up(&spec);
    let owners: Vec<_> = report
        .buckets
        .iter()
        .map(|b| b.scope.owner_ref.as_str())
        .collect();
    assert!(owners.contains(&"owner_a"));
    assert!(owners.contains(&"owner_b"));
    assert_eq!(report.star_system_scope_count, 1);
}

#[test]
fn planet_child_rf_reduce_up_groups_by_planet_scope() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let report = evaluate_planet_child_rf_reduce_up(&spec);
    assert_eq!(report.planet_scope_count, 2);
    let terra = terra_prime_bucket(&report);
    let moon = border_moon_bucket(&report);
    assert_eq!(
        terra.scope.local_scope_id.as_ref().map(|s| s.as_str()),
        Some("terra_prime")
    );
    assert_eq!(
        moon.scope.local_scope_id.as_ref().map(|s| s.as_str()),
        Some("border_moon")
    );
}

#[test]
fn planet_child_rf_reduce_up_computes_net_surplus_and_net_deficit() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let report = evaluate_planet_child_rf_reduce_up(&spec);
    let terra = terra_prime_bucket(&report);
    assert_eq!(terra.participant_count, 3);
    assert_eq!(terra.surplus_total, 20);
    assert_eq!(terra.deficit_total, 8);
    assert_eq!(terra.net_surplus, 12);
    assert_eq!(terra.net_deficit, 0);

    let moon = border_moon_bucket(&report);
    assert_eq!(moon.surplus_total, 7);
    assert_eq!(moon.deficit_total, 2);
    assert_eq!(moon.net_surplus, 5);
    assert_eq!(moon.net_deficit, 0);
}

#[test]
fn planet_child_rf_reduce_up_preserves_spatial_parentage() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let location_report = evaluate_planet_child_locations(&spec);
    assert_eq!(location_report.planet_gridcell_count, 2);
    assert_eq!(location_report.planet_non_grid_child_count, 4);
    assert_eq!(spec.structural_grid.placements.len(), 2);
}

#[test]
fn planet_child_rf_reduce_up_does_not_mutate_scenario_authority() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _report = evaluate_planet_child_rf_reduce_up(&spec);
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn planet_child_rf_reduce_up_empty_participants_defer_not_panic() {
    let mut spec = build_planet_child_rf_reduce_up_scoped_spec();
    let star = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .unwrap();
    for planet in star
        .children
        .iter_mut()
        .filter(|c| simthing_spec::is_planet_gridcell(c))
    {
        planet.children.clear();
    }

    let report = evaluate_planet_child_rf_reduce_up(&spec);
    assert_eq!(report.participant_count, 0);
    assert_eq!(report.bucket_count, 0);
    assert!(report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            simthing_spec::PlanetChildRfDeferralKind::PlanetChildRfNoParticipants
        )
    }));
}

#[test]
fn planet_child_rf_reduce_up_rejects_overflow_if_checked_math_fails() {
    let mut spec = build_planet_child_rf_reduce_up_scoped_spec();
    let star = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .unwrap();
    let terra = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::planet_id(c).as_deref() == Some("terra_prime"))
        .unwrap();
    for child in terra.children.iter_mut() {
        child.add_property(
            simthing_spec::OWNER_FLOW_SURPLUS_PROPERTY_ID,
            structural_property_value_u32(u32::MAX),
        );
    }

    let report = evaluate_planet_child_rf_reduce_up(&spec);
    assert_eq!(
        report.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(report
        .errors
        .iter()
        .any(|e| { e.kind == PlanetChildRfAdmissionErrorKind::InvalidPlanetChildRfAmount }));
}

#[test]
fn planet_child_rf_reduce_up_fixture_roundtrips() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let roundtrip = deserialize_scenario_authority(&json).expect("deserialize");
    let report = evaluate_planet_child_rf_reduce_up(&roundtrip);
    assert_eq!(report.bucket_count, 2);
    assert_eq!(report.participant_count, 4);
}

#[test]
fn normal_tests_do_not_write_reduce_up_corpus_fixture() {
    let path = corpus_path("planet_child_rf_reduce_up_scoped.simthing-scenario.json");
    if !path.exists() {
        return;
    }
    let mtime = fs::metadata(&path)
        .and_then(|m| m.modified())
        .expect("mtime");
    let age = mtime.elapsed().expect("elapsed");
    assert!(
        age.as_secs() > 5,
        "corpus fixture must not be rewritten during normal tests"
    );
}

#[test]
#[ignore = "manual corpus regeneration only"]
fn write_planet_child_rf_reduce_up_corpus_fixture() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let json = serialize_scenario_authority(&spec).expect("serialize");
    fs::write(
        corpus_path("planet_child_rf_reduce_up_scoped.simthing-scenario.json"),
        json,
    )
    .expect("write corpus");
}
