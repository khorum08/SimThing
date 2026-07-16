//! STUDIO-DISRUPTION-READOUT-0 mapeditor bridge projection proof.

use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{studio_disruption_readout_map_from_session, StudioSession};
use simthing_spec::{
    apply_galaxy_map_metadata, apply_gridcell_role_metadata, apply_owner_entity_metadata,
    apply_scenario_metadata_to_root, make_planet_gridcell, structural_property_value_u32,
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn studio_session_with_star_system() -> StudioSession {
    let mut scenario = SimThing::new(SimThingKind::Scenario, 0);
    apply_scenario_metadata_to_root(
        &mut scenario,
        "studio_disruption_mapeditor_0",
        &SimThingScenarioProvenance::default(),
        SCENARIO_SCHEMA_VERSION,
    );

    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    let mut owner = SimThing::new(SimThingKind::Owner, 0);
    apply_owner_entity_metadata(&mut owner, "owner_a", "Owner A", "player");
    game_session.add_child(owner);

    let mut galaxy_map = SimThing::new(SimThingKind::Location, 0);
    apply_galaxy_map_metadata(&mut galaxy_map, "galaxy", "Galaxy");
    let map_raw = galaxy_map.id.raw();

    let mut system = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut system, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
    system.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(3),
    );
    system.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    system.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(2),
    );
    let system_raw = system.id.raw();
    system.add_child(make_planet_gridcell("planet", 0, 0, Some("Planet")));
    galaxy_map.add_child(system);
    game_session.add_child(galaxy_map);
    scenario.add_child(game_session);

    let spec = SimThingScenarioSpec {
        scenario_id: "studio_disruption_mapeditor_0".into(),
        root: scenario,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 1,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![SimThingStructuralGridPlacement {
                location_id: "system_3".into(),
                target_id: "system_3".into(),
                system_id: 3,
                row: 2,
                col: 1,
                simthing_id_raw: system_raw,
            }],
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance::default(),
    };
    StudioSession::from_loaded_scenario(spec, PathBuf::from("fixture.simthing-scenario.json"), None)
        .expect("loaded StudioSession")
}

/// catches: mapeditor bypassing typed spec records or keying disruption by raw SimThing ids.
#[test]
fn mapeditor_consumes_typed_disruption_snapshot_by_generated_system_id() {
    let session = studio_session_with_star_system();
    let map = studio_disruption_readout_map_from_session(&session).expect("disruption map");
    assert_eq!(map.system_count, 1);
    let record = map.by_system_id.get(&3).expect("generated system key");
    assert_eq!(record.system_id(), 3);
    assert_eq!(record.max_disruption_accreted(), 0.0);
}
