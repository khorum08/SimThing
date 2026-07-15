//! STUDIO-FLEET-PRESENCE-READOUT-0 mapeditor bridge projection proof.

use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{studio_fleet_presence_map_from_session, StudioSession};
use simthing_spec::{
    apply_galaxy_map_metadata, apply_gridcell_role_metadata, apply_owner_entity_metadata,
    apply_participant_owner_flow_metadata, apply_scenario_metadata_to_root, make_planet_gridcell,
    structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_SCHEMA_VERSION, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn studio_session_with_fleet() -> StudioSession {
    let mut scenario = SimThing::new(SimThingKind::Scenario, 0);
    apply_scenario_metadata_to_root(
        &mut scenario,
        "studio_fleet_presence_mapeditor_0",
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
    let mut planet = make_planet_gridcell("planet", 0, 0, Some("Planet"));
    let surface = planet.children.first_mut().expect("surface");
    let mut fleet = SimThing::new(SimThingKind::Fleet, 0);
    apply_participant_owner_flow_metadata(&mut fleet, "owner_a", 0, 0);
    surface.add_child(fleet);
    system.add_child(planet);
    galaxy_map.add_child(system);
    game_session.add_child(galaxy_map);
    scenario.add_child(game_session);

    let spec = SimThingScenarioSpec {
        scenario_id: "studio_fleet_presence_mapeditor_0".into(),
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

/// catches: mapeditor reading raw fleet property ids instead of the typed helper snapshot.
#[test]
fn mapeditor_consumes_typed_fleet_snapshot_by_generated_system_id() {
    let session = studio_session_with_fleet();
    let map = studio_fleet_presence_map_from_session(&session).expect("fleet map");
    assert_eq!(map.total_fleets, 1);
    assert_eq!(map.transit_fleets, 0);
    let fleets = map.by_system_id.get(&3).expect("system key");
    assert_eq!(fleets.len(), 1);
    assert_eq!(
        fleets[0].owner_ref.as_ref().map(|owner| owner.as_str()),
        Some("owner_a")
    );
    assert!(fleets[0].posture.is_none());
}
