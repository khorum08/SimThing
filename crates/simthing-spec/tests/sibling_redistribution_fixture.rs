//! Sibling surplus/deficit redistribution fixture for recursive local RF tests.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_owner_silo_metadata, apply_participant_owner_flow_metadata,
    apply_participant_owner_flow_resource_key_metadata, apply_scenario_metadata_to_root,
    apply_star_system_local_grid_frame_metadata, is_surface_gridcell, make_galaxy_map, make_owner_entity,
    make_planet_gridcell, structural_property_value_u32, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, SCENARIO_SCHEMA_VERSION, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};

pub fn build_sibling_redistribution_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "RECURSIVE-LOCAL-RF-EVALUATOR-0".into(),
        generator_seed: 0x0003_4567_89AB_CDEF,
        generator_shape: "recursive_local_rf_sibling".into(),
        ..SimThingScenarioProvenance::default()
    };
    apply_scenario_metadata_to_root(
        &mut root,
        "recursive_local_rf_sibling_redistribution",
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );

    let mut owner_a = make_owner_entity("owner_a", "Owner A", "player");
    apply_owner_silo_metadata(&mut owner_a, 50, Some(100));

    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(owner_a);

    let mut galaxy_map = make_galaxy_map("galaxy_a", "Galaxy A");

    let mut star_system = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut star_system, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
    apply_star_system_local_grid_frame_metadata(
        &mut star_system,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
    );
    star_system.add_property(
        simthing_spec::SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(2),
    );
    star_system.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    star_system.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );

    let mut planet_a = make_planet_gridcell("planet_a", 0, 0, Some("Planet A"));
    let mut planet_a_cohort = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut planet_a_cohort, "owner_a", 30, 0);
    apply_participant_owner_flow_resource_key_metadata(&mut planet_a_cohort, "food");
    planet_a
        .children
        .iter_mut()
        .find(|child| is_surface_gridcell(child))
        .expect("planet_a surface")
        .add_child(planet_a_cohort);

    let mut planet_b = make_planet_gridcell("planet_b", 1, 0, Some("Planet B"));
    let mut planet_b_cohort = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut planet_b_cohort, "owner_a", 0, 20);
    apply_participant_owner_flow_resource_key_metadata(&mut planet_b_cohort, "food");
    planet_b
        .children
        .iter_mut()
        .find(|child| is_surface_gridcell(child))
        .expect("planet_b surface")
        .add_child(planet_b_cohort);

    star_system.add_child(planet_a);
    star_system.add_child(planet_b);
    galaxy_map.add_child(star_system);
    let map_raw = galaxy_map.id.raw();
    game_session.add_child(galaxy_map);
    root.add_child(game_session);

    let mut spec = SimThingScenarioSpec {
        scenario_id: "recursive_local_rf_sibling_redistribution".into(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 1,
            },
            map_container_id: map_raw.to_string(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance,
    };
    spec.sync_sidecar_from_root_metadata();
    spec
}

pub fn star_system_id_raw(spec: &SimThingScenarioSpec) -> u32 {
    spec.root
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::Location)
        .unwrap()
        .children
        .iter()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .unwrap()
        .id
        .raw()
}
