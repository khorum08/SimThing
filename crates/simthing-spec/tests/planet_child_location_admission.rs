//! PLANET-CHILD-LOCATION-ADMISSION-0 — planet child-location admission proofs.

use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_planet_child_location_command, apply_planet_child_metadata,
    apply_scenario_metadata_to_root, evaluate_planet_child_locations, ingest_scenario,
    ingest_scenario_from_str, make_galaxy_map, make_owner_entity, make_planet_child_location,
    serialize_scenario_authority, structural_property_value_u32, validate_planet_child_locations,
    validate_scenario_root_authority, validate_stead_mapping_consistency,
    PlanetChildLocationAdmissionErrorKind, PlanetChildLocationCommand, ScenarioDeferralKind,
    ScenarioIngestionClassification, ScenarioIngestionProfile, ScenarioRootValidationMode,
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, GALAXY_CHILD_LOCATION_ROLE_MOON,
    GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, PLANET_ID_PROPERTY_ID,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

const CANONICAL_PROFILE: ScenarioIngestionProfile = ScenarioIngestionProfile {
    require_canonical_tree: true,
    admit_legacy_world_root: true,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn base_galaxymap_spec(scenario_id: &str) -> SimThingScenarioSpec {
    let mut spec = base_canonical_spec(scenario_id);
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let galaxy_map = game_session
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .expect("map");

    let mut cell_a = make_gridcell(GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0);
    let cell_a_raw = cell_a.id.raw();
    let mut cell_b = make_gridcell(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, 2, 1, 0);
    let cell_b_raw = cell_b.id.raw();
    galaxy_map.add_child(cell_a);
    galaxy_map.add_child(cell_b);
    let map_raw = galaxy_map.id.raw();
    spec.structural_grid = SimThingScenarioGrid {
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
                simthing_id_raw: cell_a_raw,
            },
            SimThingStructuralGridPlacement {
                location_id: "cell_b".into(),
                target_id: "cell_b".into(),
                system_id: 2,
                row: 0,
                col: 1,
                simthing_id_raw: cell_b_raw,
            },
        ],
    };
    spec
}

fn base_canonical_spec(scenario_id: &str) -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "PLANET-CHILD-LOCATION-ADMISSION-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "planet_child".into(),
    };
    apply_scenario_metadata_to_root(&mut root, scenario_id, &provenance, SCENARIO_SCHEMA_VERSION);
    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(make_owner_entity("owner_a", "Owner A", "player"));
    let galaxy_map = make_galaxy_map("galaxy_a", "Galaxy A");
    game_session.add_child(galaxy_map);
    root.add_child(game_session);
    let mut spec = SimThingScenarioSpec {
        scenario_id: scenario_id.to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance,
    };
    spec.sync_sidecar_from_root_metadata();
    spec
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

fn star_system_gridcell_mut(spec: &mut SimThingScenarioSpec) -> &mut SimThing {
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let galaxy_map = game_session
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .expect("map");
    galaxy_map
        .children
        .iter_mut()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .expect("star system")
}

fn inert_gridcell_mut(spec: &mut SimThingScenarioSpec) -> &mut SimThing {
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let galaxy_map = game_session
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .expect("map");
    galaxy_map
        .children
        .iter_mut()
        .find(|c| simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_INERT))
        .expect("inert")
}

fn write_json(spec: &SimThingScenarioSpec, name: &str) {
    let json = serialize_scenario_authority(spec).expect("serialize");
    fs::write(corpus_path(name), json).expect("write corpus");
}

#[test]
fn write_planet_child_location_corpus_fixtures() {
    let mut admitted = base_galaxymap_spec("planet_child_location_admitted");
    star_system_gridcell_mut(&mut admitted).add_child(make_planet_child_location(
        "terra_prime",
        Some("Terra Prime"),
    ));
    write_json(
        &admitted,
        "planet_child_location_admitted.simthing-scenario.json",
    );

    let mut under_inert = base_galaxymap_spec("planet_child_location_under_inert_rejected");
    inert_gridcell_mut(&mut under_inert).add_child(make_planet_child_location("bad_planet", None));
    write_json(
        &under_inert,
        "planet_child_location_under_inert_rejected.simthing-scenario.json",
    );

    let mut duplicate = base_galaxymap_spec("planet_child_location_duplicate_id_rejected");
    let p1 = make_planet_child_location("dup_planet", None);
    let p2 = make_planet_child_location("dup_planet", None);
    star_system_gridcell_mut(&mut duplicate).add_child(p1);
    star_system_gridcell_mut(&mut duplicate).add_child(p2);
    write_json(
        &duplicate,
        "planet_child_location_duplicate_id_rejected.simthing-scenario.json",
    );

    let mut unsupported = base_galaxymap_spec("planet_child_location_unsupported_child_deferred");
    let mut moon = SimThing::new(SimThingKind::Location, 0);
    moon.add_property(
        GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID,
        simthing_spec::scenario_metadata_string_value(GALAXY_CHILD_LOCATION_ROLE_MOON),
    );
    moon.add_property(
        PLANET_ID_PROPERTY_ID,
        simthing_spec::scenario_metadata_string_value("luna_x"),
    );
    star_system_gridcell_mut(&mut unsupported).add_child(moon);
    write_json(
        &unsupported,
        "planet_child_location_unsupported_child_deferred.simthing-scenario.json",
    );
}

#[test]
fn planet_under_star_system_gridcell_admitted() {
    let json = fs::read_to_string(corpus_path(
        "planet_child_location_admitted.simthing-scenario.json",
    ))
    .expect("admitted corpus — run write_planet_child_location_corpus_fixtures first");
    let (result, _) = ingest_scenario_from_str("admitted", &json, CANONICAL_PROFILE);
    assert!(matches!(
        result.classification,
        ScenarioIngestionClassification::Admitted
            | ScenarioIngestionClassification::PartiallyAdmitted
    ));
    let report = result.planet_child_location.expect("planet report");
    assert_eq!(report.planet_count, 1);
    assert!(report.errors.is_empty());
}

#[test]
fn planet_under_inert_gridcell_rejected() {
    let json = fs::read_to_string(corpus_path(
        "planet_child_location_under_inert_rejected.simthing-scenario.json",
    ))
    .expect("under inert corpus");
    let (result, _) = ingest_scenario_from_str("under_inert", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(result
        .errors
        .iter()
        .any(|e| e.message.contains("PlanetUnderInertGridcell")));
}

#[test]
fn planet_missing_id_rejected() {
    let mut spec = base_galaxymap_spec("missing_planet_id");
    let mut planet = SimThing::new(SimThingKind::Location, 0);
    apply_planet_child_metadata(&mut planet, "", None);
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetMissingId
        )
    }));
}

#[test]
fn duplicate_planet_id_rejected() {
    let json = fs::read_to_string(corpus_path(
        "planet_child_location_duplicate_id_rejected.simthing-scenario.json",
    ))
    .expect("duplicate corpus");
    let (result, _) = ingest_scenario_from_str("duplicate", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
}

#[test]
fn planet_not_added_to_structural_grid_placements() {
    let mut spec = base_galaxymap_spec("no_structural_pollution");
    apply_planet_child_location_command(
        &mut spec,
        PlanetChildLocationCommand::AddPlanet {
            star_system_gridcell_id: "cell_b".into(),
            planet_id: "p1".into(),
            display_name: Some("P1".into()),
        },
    )
    .expect("add planet");
    let before = spec.structural_grid.placements.len();
    assert_eq!(before, 2);
    assert!(spec
        .structural_grid
        .placements
        .iter()
        .all(|p| p.location_id != "p1"));
}

#[test]
fn unsupported_child_location_role_typed_deferred() {
    let json = fs::read_to_string(corpus_path(
        "planet_child_location_unsupported_child_deferred.simthing-scenario.json",
    ))
    .expect("unsupported corpus");
    let (result, _) = ingest_scenario_from_str("unsupported", &json, CANONICAL_PROFILE);
    assert!(result
        .deferrals
        .iter()
        .any(|d| { matches!(d.kind, ScenarioDeferralKind::UnsupportedChildLocationRole) }));
}

#[test]
fn deep_child_location_typed_deferred() {
    let mut spec = base_galaxymap_spec("deep_child");
    let mut planet = make_planet_child_location("deep_p", None);
    planet.add_child(SimThing::new(SimThingKind::Location, 0));
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            PlanetChildLocationAdmissionErrorKind::DeepChildLocationDeferred
        )
    }));
}

#[test]
fn scenario_ingestion_reports_planet_child_locations() {
    let mut spec = base_galaxymap_spec("ingestion_report");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_child_location("ingested", None));
    let result = ingest_scenario("ingestion_report", &spec, CANONICAL_PROFILE);
    let report = result.planet_child_location.expect("planet report");
    assert_eq!(report.planet_count, 1);
}

#[test]
fn valid_planet_child_no_longer_emits_blanket_planets_not_admitted() {
    let mut spec = base_galaxymap_spec("no_blanket");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_child_location("valid", None));
    let result = ingest_scenario("no_blanket", &spec, CANONICAL_PROFILE);
    assert!(!result
        .deferrals
        .iter()
        .any(|d| d.kind == ScenarioDeferralKind::PlanetsNotYetAdmitted));
    assert!(result
        .deferrals
        .iter()
        .any(|d| { d.kind == ScenarioDeferralKind::PlanetSimulationDeferred }));
}

#[test]
fn planet_child_location_preserves_canonical_validation() {
    let mut spec = base_galaxymap_spec("canonical");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_child_location("c1", None));
    validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical).expect("root");
    validate_stead_mapping_consistency(&spec).expect("stead");
    validate_planet_child_locations(&spec).expect("planet");
}
