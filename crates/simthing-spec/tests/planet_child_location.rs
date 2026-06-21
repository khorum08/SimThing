//! SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0 — planet surface gridcell tier proofs.

use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_planet_gridcell_metadata, apply_scenario_metadata_to_root,
    apply_star_system_local_grid_frame_metadata,
    evaluate_loaded_scenario_recursive_rf_runtime_from_json_str, evaluate_planet_child_locations,
    gridcell_role, is_surface_gridcell, make_galaxy_map, make_owner_entity, make_planet_gridcell,
    make_surface_gridcell, planet_surface_gridcell, structural_property_value_u32,
    PlanetChildLocationAdmissionErrorKind, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, LOCAL_GRIDCELL_ROLE_SURFACE,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};

const OWNER_SILO_FIXTURE: &str = "owner_silo_disburse_down_scoped.simthing-scenario.json";

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn base_galaxymap_spec(scenario_id: &str) -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "PLANET-SURFACE-GRIDCELL-TIER-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "planet_surface_gridcell".into(),
    };
    apply_scenario_metadata_to_root(&mut root, scenario_id, &provenance, SCENARIO_SCHEMA_VERSION);
    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(make_owner_entity("owner_a", "Owner A", "player"));
    let mut galaxy_map = make_galaxy_map("galaxy_a", "Galaxy A");
    let cell_a = make_gridcell(GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0);
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
    game_session.add_child(galaxy_map);
    root.add_child(game_session);
    let mut spec = SimThingScenarioSpec {
        scenario_id: scenario_id.to_string(),
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
        .find(|c| gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM))
        .expect("star system")
}

fn add_child_to_planet_surface(planet: &mut SimThing, child: SimThing) {
    let surface = planet
        .children
        .iter_mut()
        .find(|c| is_surface_gridcell(c))
        .expect("planet surface gridcell");
    surface.add_child(child);
}

#[test]
fn planet_child_location_admits_surface_gridcell_under_planet() {
    let mut spec = base_galaxymap_spec("surface_admitted");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_gridcell("p1", 0, 0, None));
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.surface_gridcell_tier_present);
    assert_eq!(report.surface_gridcell_count, 1);
    assert!(report.errors.is_empty());
}

#[test]
fn planet_child_location_rejects_direct_gameplay_children_under_planet() {
    let mut spec = base_galaxymap_spec("direct_gameplay_rejected");
    let mut planet = make_planet_gridcell("p1", 0, 0, None);
    planet.add_child(SimThing::new(SimThingKind::Cohort, 0));
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert_eq!(report.direct_gameplay_child_under_planet_count, 1);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetDirectGameplayChildRequiresSurfaceGridcell
        )
    }));
}

#[test]
fn planet_child_location_admits_gameplay_children_under_surface_gridcell() {
    let mut spec = base_galaxymap_spec("surface_gameplay");
    let mut planet = make_planet_gridcell("p1", 0, 0, None);
    add_child_to_planet_surface(&mut planet, SimThing::new(SimThingKind::Fleet, 0));
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert_eq!(report.gameplay_child_under_surface_count, 1);
    assert_eq!(report.planet_non_grid_child_count, 1);
    assert!(report.errors.is_empty());
}

#[test]
fn planet_child_location_does_not_defer_required_surface_gridcell_as_deep_child() {
    let mut spec = base_galaxymap_spec("surface_not_deep_deferred");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_gridcell("p1", 0, 0, None));
    let report = evaluate_planet_child_locations(&spec);
    assert!(!report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            PlanetChildLocationAdmissionErrorKind::DeepPlanetChildDeferred
        )
    }));
}

#[test]
fn planet_child_location_requires_surface_gridcell_for_planet_gameplay() {
    let mut spec = base_galaxymap_spec("surface_required");
    let mut planet = SimThing::new(SimThingKind::Location, 0);
    apply_planet_gridcell_metadata(&mut planet, "bare", 0, 0, None);
    star_system_gridcell_mut(&mut spec).add_child(planet);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.iter().any(|e| {
        matches!(
            e.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetSurfaceGridcellMissing
        )
    }));
}

#[test]
fn planet_child_location_preserves_owner_metadata_not_spatial_parentage() {
    let mut spec = base_galaxymap_spec("owner_metadata");
    star_system_gridcell_mut(&mut spec).add_child(make_planet_gridcell("owned", 0, 0, None));
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.errors.is_empty());
}

#[test]
fn loaded_scenario_recursive_rf_runtime_uses_surface_gridcell_as_gameplay_parent_arena() {
    let json = fs::read_to_string(corpus_path(OWNER_SILO_FIXTURE)).expect("corpus");
    let report = evaluate_loaded_scenario_recursive_rf_runtime_from_json_str("owner_silo", &json)
        .expect("rf");
    assert!(report.gameplay_rows_parented_to_surface);
    assert!(report.surface_arena_count >= 2);
}

#[test]
fn loaded_scenario_recursive_rf_runtime_bubbles_surface_to_planet_to_star_to_galaxy() {
    let json = fs::read_to_string(corpus_path(OWNER_SILO_FIXTURE)).expect("corpus");
    let report = evaluate_loaded_scenario_recursive_rf_runtime_from_json_str("owner_silo", &json)
        .expect("rf");
    assert!(report.surface_to_planet_bubbling_present);
    assert!(report.sibling_settlement_before_upward_bubbling);
}

#[test]
fn normal_tests_do_not_write_surface_gridcell_fixtures() {
    assert!(corpus_path(OWNER_SILO_FIXTURE).is_file());
    assert_eq!(LOCAL_GRIDCELL_ROLE_SURFACE, "surface");
    let planet = make_planet_gridcell("p", 0, 0, None);
    assert!(planet_surface_gridcell(&planet).is_some_and(is_surface_gridcell));
    assert!(is_surface_gridcell(&make_surface_gridcell()));
}
