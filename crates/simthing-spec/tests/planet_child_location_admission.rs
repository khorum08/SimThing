//! RECURSIVE-SPATIAL-GRID-DEFAULTS-0 — recursive spatial local-grid admission proofs.

use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_local_gridcell_metadata, apply_planet_gridcell_metadata,
    apply_planet_local_grid_command, apply_scenario_metadata_to_root,
    apply_star_system_local_grid_frame_metadata, collect_local_receiver_cells,
    collect_planet_non_grid_children, default_local_grid_frame_for_spatial_gridcell,
    evaluate_planet_child_locations, ingest_scenario, ingest_scenario_from_str, is_planet_gridcell,
    is_surface_gridcell, make_galaxy_map, make_owner_entity, make_planet_gridcell,
    planet_gridcell_interior_frame, planet_owner_ref, save_scenario_spec_to_canonical_json,
    scenario_metadata_string_value, star_system_local_grid_frame, structural_property_value_u32,
    validate_planet_child_locations, validate_scenario_root_authority,
    validate_stead_mapping_consistency, LocalGridFrame, PlanetChildLocationAdmissionErrorKind,
    PlanetLocalGridCommand, ScenarioDeferralKind, ScenarioIngestionClassification,
    ScenarioIngestionProfile, ScenarioRootValidationMode, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    LOCAL_GRIDCELL_COL_PROPERTY_ID, LOCAL_GRIDCELL_ROLE_RECEIVER, LOCAL_GRIDCELL_ROW_PROPERTY_ID,
    LOCAL_GRID_DEFAULT_COLS, LOCAL_GRID_DEFAULT_ROWS, PLANET_ID_PROPERTY_ID,
    PLANET_OWNER_REF_PROPERTY_ID, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_SCHEMA_VERSION, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
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

fn add_child_to_planet_surface(planet: &mut SimThing, child: SimThing) {
    let surface = planet
        .children
        .iter_mut()
        .find(|c| is_surface_gridcell(c))
        .expect("planet surface gridcell");
    surface.add_child(child);
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
    apply_star_system_local_grid_frame_metadata(
        &mut cell_b,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
    );
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
        source: "RECURSIVE-SPATIAL-GRID-DEFAULTS-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "recursive_spatial_grid".into(),
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

fn make_receiver_local_gridcell(col: u32, row: u32) -> SimThing {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    apply_local_gridcell_metadata(&mut cell, LOCAL_GRIDCELL_ROLE_RECEIVER, col, row);
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

#[test]
fn planet_gridcell_under_star_system_admitted() {
    let json = fs::read_to_string(corpus_path(
        "planet_child_location_admitted.simthing-scenario.json",
    ))
    .expect("admitted corpus");
    let (result, _) = ingest_scenario_from_str("admitted", &json, CANONICAL_PROFILE);
    assert!(matches!(
        result.classification,
        ScenarioIngestionClassification::Admitted
            | ScenarioIngestionClassification::PartiallyAdmitted
    ));
    let report = result.planet_child_location.expect("planet report");
    assert_eq!(report.planet_gridcell_count, 1);
    assert_eq!(report.local_gridcell_count, 2);
    assert!(report.surface_gridcell_tier_present);
    assert!(report.errors.is_empty());
}

#[test]
fn generic_spatial_gridcell_defaults_to_1x1_local_frame() {
    let mut spec = base_galaxymap_spec("generic_default");
    let inert = inert_gridcell_mut(&mut spec);
    let frame = default_local_grid_frame_for_spatial_gridcell(inert);
    assert_eq!(
        frame,
        LocalGridFrame {
            cols: LOCAL_GRID_DEFAULT_COLS,
            rows: LOCAL_GRID_DEFAULT_ROWS,
        }
    );
}

#[test]
fn star_system_gridcell_defaults_to_10x10_local_frame() {
    let mut spec = base_galaxymap_spec("default_frame");
    let star = star_system_gridcell_mut(&mut spec);
    let (cols, rows) = star_system_local_grid_frame(star);
    assert_eq!(cols, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS);
    assert_eq!(rows, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS);
}

#[test]
fn planet_gridcell_defaults_to_1x1_interior_frame() {
    let planet = make_planet_gridcell("interior", 0, 0, None);
    let interior = planet_gridcell_interior_frame(&planet);
    assert_eq!(interior.cols, LOCAL_GRID_DEFAULT_COLS);
    assert_eq!(interior.rows, LOCAL_GRID_DEFAULT_ROWS);
}

#[test]
fn inert_galactic_gridcell_admits_1x1_receiver_cell() {
    let mut spec = base_galaxymap_spec("inert_receiver");
    inert_gridcell_mut(&mut spec).add_child(make_receiver_local_gridcell(0, 0));
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.is_empty());
    assert_eq!(report.receiver_cell_count, 1);
    assert_eq!(report.implicit_receiver_cell_count, 0);
    let receivers = collect_local_receiver_cells(&spec);
    assert_eq!(receivers.len(), 1);
    assert!(!receivers[0].is_implicit);
    assert_eq!(receivers[0].col, 0);
    assert_eq!(receivers[0].row, 0);
}

#[test]
fn inert_galactic_gridcell_rejects_planet_local_gridcell() {
    let json = fs::read_to_string(corpus_path(
        "planet_child_location_under_inert_rejected.simthing-scenario.json",
    ))
    .expect("under inert corpus");
    let (result, _) = ingest_scenario_from_str("under_inert", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    let report = result.planet_child_location.expect("planet report");
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::InertGridcellNonReceiverChild
        )
    }));
}

#[test]
fn inert_galactic_gridcell_rejects_receiver_coordinate_outside_1x1() {
    let mut spec = base_galaxymap_spec("receiver_oob");
    inert_gridcell_mut(&mut spec).add_child(make_receiver_local_gridcell(1, 0));
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::InertGridcellReceiverCoordinateOutOfFrame
        )
    }));
}

#[test]
fn planet_gridcell_requires_local_coordinate() {
    let mut spec = base_galaxymap_spec("missing_coord");
    let mut planet = SimThing::new(SimThingKind::Location, 0);
    apply_planet_gridcell_metadata(&mut planet, "no_coord", 0, 0, None);
    planet.properties.retain(|id, _| {
        *id != LOCAL_GRIDCELL_COL_PROPERTY_ID && *id != LOCAL_GRIDCELL_ROW_PROPERTY_ID
    });
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetLocalGridMissingCoordinate
        )
    }));
}

#[test]
fn planet_gridcell_coordinate_out_of_10x10_rejected() {
    let mut spec = base_galaxymap_spec("out_of_frame");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_gridcell("far_planet", 10, 0, None));
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetLocalGridCoordinateOutOfFrame
        )
    }));
}

#[test]
fn planet_gridcell_duplicate_local_coordinate_rejected() {
    let mut spec = base_galaxymap_spec("dup_coord");
    let star = star_system_gridcell_mut(&mut spec);
    star.add_child(make_planet_gridcell("p1", 0, 0, None));
    star.add_child(make_planet_gridcell("p2", 0, 0, None));
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetLocalGridDuplicateCoordinate
        )
    }));
}

#[test]
fn planet_gridcell_missing_planet_id_rejected() {
    let mut spec = base_galaxymap_spec("missing_planet_id");
    let mut planet = make_planet_gridcell("", 0, 0, None);
    planet
        .properties
        .retain(|id, _| *id != PLANET_ID_PROPERTY_ID);
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetGridcellMissingId
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
fn planet_gridcell_not_added_to_galaxy_structural_grid_placements() {
    let mut spec = base_galaxymap_spec("no_structural_pollution");
    apply_planet_local_grid_command(
        &mut spec,
        PlanetLocalGridCommand::AddPlanetGridcell {
            star_system_gridcell_id: "cell_b".into(),
            planet_gridcell_id: "p1".into(),
            planet_id: "p1".into(),
            col: 0,
            row: 0,
            display_name: Some("P1".into()),
        },
    )
    .expect("add planet");
    assert_eq!(spec.structural_grid.placements.len(), 2);
    assert!(spec
        .structural_grid
        .placements
        .iter()
        .all(|p| p.location_id != "p1"));
}

#[test]
fn planet_gridcell_allows_non_grid_children_such_as_cohort_fleet_infrastructure() {
    let mut spec = base_galaxymap_spec("planet_non_grid_children");
    let mut planet = make_planet_gridcell("fleet_world", 1, 0, None);
    add_child_to_planet_surface(&mut planet, SimThing::new(SimThingKind::Cohort, 0));
    add_child_to_planet_surface(&mut planet, SimThing::new(SimThingKind::Fleet, 0));
    add_child_to_planet_surface(
        &mut planet,
        SimThing::new(SimThingKind::Custom("Infrastructure".into()), 0),
    );
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.is_empty());
    assert_eq!(report.planet_gridcell_count, 1);
    assert_eq!(report.planet_non_grid_child_count, 3);
    assert_eq!(collect_planet_non_grid_children(&spec).len(), 3);
    assert!(report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildSimulationDeferred
        )
    }));
}

#[test]
fn planet_non_grid_child_rejects_local_coordinate_metadata() {
    let mut spec = base_galaxymap_spec("planet_non_grid_local_coord");
    let mut planet = make_planet_gridcell("bad_child", 0, 0, None);
    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    cohort.add_property(
        LOCAL_GRIDCELL_COL_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    add_child_to_planet_surface(&mut planet, cohort);
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildHasLocalCoordinate
        )
    }));
}

#[test]
fn planet_non_grid_child_unsupported_kind_typed_deferred() {
    let mut spec = base_galaxymap_spec("planet_non_grid_unsupported");
    let mut planet = make_planet_gridcell("unsupported_child", 0, 0, None);
    add_child_to_planet_surface(
        &mut planet,
        SimThing::new(SimThingKind::ArenaParticipant, 0),
    );
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.is_empty());
    assert_eq!(report.planet_non_grid_child_count, 0);
    assert!(report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildUnsupportedKind
        )
    }));
}

#[test]
fn scenario_ingestion_reports_planet_non_grid_child_count() {
    let mut spec = base_galaxymap_spec("ingestion_non_grid");
    let mut planet = make_planet_gridcell("terra", 0, 0, None);
    add_child_to_planet_surface(&mut planet, SimThing::new(SimThingKind::Cohort, 0));
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let result = ingest_scenario("ingestion_non_grid", &spec, CANONICAL_PROFILE);
    let report = result.planet_child_location.expect("planet report");
    assert_eq!(report.planet_non_grid_child_count, 1);
}

#[test]
fn owner_channel_metadata_does_not_require_spatial_reparenting() {
    let mut spec = base_galaxymap_spec("owner_channel");
    let mut planet = make_planet_gridcell("owned", 0, 0, None);
    planet.add_property(
        PLANET_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value("owner_a"),
    );
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.is_empty());
    assert!(report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred
        )
    }));

    let game_session = spec
        .root
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let owner = game_session
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::Owner)
        .expect("owner");
    assert!(
        owner.children.is_empty(),
        "owner channel metadata must not reparent planet under Owner"
    );

    let star = star_system_gridcell_mut(&mut spec);
    let planet_child = star
        .children
        .iter()
        .find(|c| is_planet_gridcell(c))
        .expect("planet remains under star system");
    assert_eq!(planet_owner_ref(planet_child).as_deref(), Some("owner_a"));
}

#[test]
fn unsupported_local_child_role_typed_deferred() {
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
fn valid_planet_gridcell_no_longer_emits_blanket_planets_not_admitted() {
    let mut spec = base_galaxymap_spec("no_blanket");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_gridcell("valid", 0, 0, None));
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
fn validate_planet_child_locations_rejects_rejected_reports() {
    let mut spec = base_galaxymap_spec("validate_fail_closed");
    inert_gridcell_mut(&mut spec).add_child(make_planet_gridcell("bad", 0, 0, None));
    let err = validate_planet_child_locations(&spec).expect_err("must reject");
    assert!(matches!(
        err.kind,
        PlanetChildLocationAdmissionErrorKind::InertGridcellNonReceiverChild
    ));
}

#[test]
fn normal_tests_do_not_write_corpus_fixtures() {
    for name in [
        "planet_child_location_admitted.simthing-scenario.json",
        "planet_child_location_under_inert_rejected.simthing-scenario.json",
        "planet_child_location_duplicate_id_rejected.simthing-scenario.json",
        "planet_child_location_unsupported_child_deferred.simthing-scenario.json",
        "inert_gridcell_receiver_1x1_admitted.simthing-scenario.json",
    ] {
        assert!(
            corpus_path(name).is_file(),
            "durable corpus fixture `{name}` must exist"
        );
    }
}

#[test]
fn deep_child_location_typed_deferred() {
    let mut spec = base_galaxymap_spec("deep_child");
    let mut planet = make_planet_gridcell("deep_p", 0, 0, None);
    add_child_to_planet_surface(&mut planet, SimThing::new(SimThingKind::Location, 0));
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            PlanetChildLocationAdmissionErrorKind::DeepPlanetChildDeferred
        )
    }));
}

#[test]
fn scenario_ingestion_reports_planet_local_gridcells() {
    let mut spec = base_galaxymap_spec("ingestion_report");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_gridcell("ingested", 0, 0, None));
    let result = ingest_scenario("ingestion_report", &spec, CANONICAL_PROFILE);
    let report = result.planet_child_location.expect("planet report");
    assert_eq!(report.planet_gridcell_count, 1);
    assert_eq!(report.local_gridcell_count, 2);
    assert!(report.surface_gridcell_tier_present);
}

#[test]
#[ignore = "manual corpus regeneration only"]
fn regen_planet_child_location_admitted_corpus_fixture() {
    let mut spec = base_galaxymap_spec("planet_child_location_admitted");
    spec.scenario_id = "planet_child_location_admitted".into();
    star_system_gridcell_mut(&mut spec).add_child(make_planet_gridcell("admitted_p", 0, 0, None));
    let save = save_scenario_spec_to_canonical_json(&spec).expect("save");
    fs::write(
        corpus_path("planet_child_location_admitted.simthing-scenario.json"),
        save.canonical_json,
    )
    .expect("write corpus");
}

#[test]
fn planet_child_location_preserves_canonical_validation() {
    let mut spec = base_galaxymap_spec("canonical");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_gridcell("c1", 0, 0, None));
    validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical).expect("root");
    validate_stead_mapping_consistency(&spec).expect("stead");
    validate_planet_child_locations(&spec).expect("planet");
}

fn inert_receiver_only_spec() -> SimThingScenarioSpec {
    let mut spec = base_canonical_spec("inert_gridcell_receiver_1x1_admitted");
    spec.provenance.source = "RECURSIVE-SPATIAL-GRID-DEFAULTS-0".into();
    spec.provenance.generator_shape = "inert_receiver".into();
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
    let cell_a = make_gridcell(GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0);
    let cell_a_raw = cell_a.id.raw();
    galaxy_map.add_child(cell_a);
    let map_raw = galaxy_map.id.raw();
    spec.structural_grid = SimThingScenarioGrid {
        frame: SimThingStructuralGridFrame {
            width: 8,
            height: 8,
            occupied_cells: 1,
        },
        map_container_id: map_raw.to_string(),
        placements: vec![SimThingStructuralGridPlacement {
            location_id: "cell_a".into(),
            target_id: "cell_a".into(),
            system_id: 1,
            row: 0,
            col: 0,
            simthing_id_raw: cell_a_raw,
        }],
    };
    spec.sync_sidecar_from_root_metadata();
    spec
}
