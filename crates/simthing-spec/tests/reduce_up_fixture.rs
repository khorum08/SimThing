//! Shared reduce-up scoped fixture for writeback tests.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_owner_silo_metadata, apply_participant_owner_flow_metadata,
    apply_scenario_metadata_to_root, apply_star_system_local_grid_frame_metadata,
    is_surface_gridcell, make_galaxy_map, make_owner_entity, make_planet_gridcell,
    structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};

pub fn build_planet_child_rf_reduce_up_scoped_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "OWNER-SILO-RUNTIME-WRITEBACK-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "owner_silo_runtime_writeback".into(),
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

    let mut inert = make_gridcell(GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0);
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
