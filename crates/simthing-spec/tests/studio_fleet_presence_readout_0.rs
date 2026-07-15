//! STUDIO-FLEET-PRESENCE-READOUT-0 typed snapshot contract proof.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_galaxy_map_metadata, apply_gridcell_role_metadata, apply_owner_entity_metadata,
    apply_participant_owner_flow_metadata, apply_scenario_metadata_to_root,
    fleet_presence_snapshot, fleet_presence_snapshot_with_transit, make_planet_gridcell,
    scenario_metadata_string_value, structural_property_value_u32, FleetPresenceLocation,
    FleetPresenceSnapshotError, FleetPresenceTransitOverride, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, SCENARIO_SCHEMA_VERSION,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, TP_FLEET_HOME_SYSTEM_PROPERTY_ID,
    TP_FLEET_POSTURE_PROPERTY_ID,
};

fn compact_fleet_spec() -> (SimThingScenarioSpec, u32) {
    let mut scenario = SimThing::new(SimThingKind::Scenario, 0);
    apply_scenario_metadata_to_root(
        &mut scenario,
        "studio_fleet_presence_readout_0",
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
        structural_property_value_u32(7),
    );
    system.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(3),
    );
    system.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(2),
    );
    let system_raw = system.id.raw();

    let mut planet = make_planet_gridcell("planet_a", 0, 0, Some("Planet A"));
    let surface = planet.children.first_mut().expect("surface gridcell");
    let mut fleet = SimThing::new(SimThingKind::Fleet, 0);
    apply_participant_owner_flow_metadata(&mut fleet, "owner_a", 0, 0);
    fleet.add_property(
        TP_FLEET_POSTURE_PROPERTY_ID,
        scenario_metadata_string_value("border"),
    );
    fleet.add_property(
        TP_FLEET_HOME_SYSTEM_PROPERTY_ID,
        scenario_metadata_string_value("7"),
    );
    let fleet_raw = fleet.id.raw();
    surface.add_child(fleet);
    system.add_child(planet);
    galaxy_map.add_child(system);

    let mut destination = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut destination, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
    destination.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(8),
    );
    destination.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(4),
    );
    destination.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(2),
    );
    let destination_raw = destination.id.raw();
    destination.add_child(make_planet_gridcell(
        "planet_b",
        0,
        0,
        Some("Planet B"),
    ));
    galaxy_map.add_child(destination);
    game_session.add_child(galaxy_map);
    scenario.add_child(game_session);

    let spec = SimThingScenarioSpec {
        scenario_id: "studio_fleet_presence_readout_0".into(),
        root: scenario,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 2,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![
                SimThingStructuralGridPlacement {
                    location_id: "system_7".into(),
                    target_id: "system_7".into(),
                    system_id: 7,
                    row: 2,
                    col: 3,
                    simthing_id_raw: system_raw,
                },
                SimThingStructuralGridPlacement {
                    location_id: "system_8".into(),
                    target_id: "system_8".into(),
                    system_id: 8,
                    row: 2,
                    col: 4,
                    simthing_id_raw: destination_raw,
                },
            ],
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance::default(),
    };
    (spec, fleet_raw)
}

/// catches: fleet readout flattening to raw ids, dropping transit contract, or mutating authority.
#[test]
fn studio_fleet_presence_snapshot_is_typed_read_only_and_transit_capable() {
    let (spec, fleet_raw) = compact_fleet_spec();
    let before = spec.clone();

    let anchored = fleet_presence_snapshot(&spec).expect("anchored snapshot");
    assert_eq!(anchored.records.len(), 1);
    let record = &anchored.records[0];
    assert_eq!(record.fleet_simthing_id_raw, fleet_raw);
    assert_eq!(record.owner_ref.as_ref().map(|owner| owner.as_str()), Some("owner_a"));
    assert_eq!(record.posture.as_deref(), Some("border"));
    assert_eq!(record.location, FleetPresenceLocation::Anchored(7));
    assert_eq!(anchored.by_system_id().keys().copied().collect::<Vec<_>>(), vec![7]);
    assert_eq!(spec.root, before.root, "snapshot must not mutate ScenarioSpec authority");

    let transit = fleet_presence_snapshot_with_transit(
        &spec,
        [FleetPresenceTransitOverride {
            fleet_simthing_id_raw: fleet_raw,
            source_system_id: 7,
            dest_system_id: 8,
        }],
    )
    .expect("typed transit fixture");
    assert_eq!(
        transit.records[0].location,
        FleetPresenceLocation::InTransit {
            source_system_id: 7,
            dest_system_id: 8,
        }
    );

    let err = fleet_presence_snapshot_with_transit(
        &spec,
        [FleetPresenceTransitOverride {
            fleet_simthing_id_raw: fleet_raw + 1000,
            source_system_id: 7,
            dest_system_id: 8,
        }],
    )
    .expect_err("unknown transit fleet must fail loud");
    assert!(matches!(
        err,
        FleetPresenceSnapshotError::UnknownTransitFleet(_)
    ));
}
