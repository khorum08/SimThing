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
        ..SimThingScenarioProvenance::default()
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
