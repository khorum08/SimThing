//! TP-FLEETS-SHIPS-0 table-driven fleet/ship authoring proof.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario, parse_raw_document, HydratedFleetShipPayload, HydratedScenarioPack,
};
use simthing_core::{SimPropertyId, SimThing, SimThingKind};
use simthing_spec::{
    evaluate_planet_child_locations, evaluate_planet_child_rf_admission,
    evaluate_planet_child_rf_reduce_up, game_session_child, game_session_galaxy_map,
    game_session_owners, is_galaxy_map_entity, is_surface_gridcell, owner_entity_id,
    owner_flow_deficit, owner_flow_owner_ref, planet_surface_gridcell,
    scenario_metadata_string, star_system_gridcells, PlanetChildRfAdmissionClassification,
};

const TP_FLEET_POSTURE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_301_500);
const TP_FLEET_HOME_SYSTEM_PROPERTY_ID: SimPropertyId = SimPropertyId(8_301_501);
const TP_SHIP_CLASS_PROPERTY_ID: SimPropertyId = SimPropertyId(8_301_502);
const TP_SHIP_HULL_PROPERTY_ID: SimPropertyId = SimPropertyId(8_301_503);
const TP_SHIP_WEAPON_DAMAGE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_301_504);
const TP_SHIP_UPKEEP_PROPERTY_ID: SimPropertyId = SimPropertyId(8_301_505);

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

fn fixture_path_text() -> String {
    fixture_path().to_string_lossy().replace('\\', "/")
}

fn combined_clause() -> String {
    format!(
        r#"
scenario = tp_fleets_ships_0 {{
    metadata = {{
        display_name = "TP Fleets Ships 0"
        runtime_owner = "scenario-container"
    }}
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{
        owner_key = "terran"
        display_name = "Terran Compact"
        archetype = "settler_policy"
    }}
    owner = pirate {{
        owner_key = "pirate"
        display_name = "Pirate Cartel"
        archetype = "raider_policy"
    }}
    ownership_volume = terran_core {{
        owner = "terran"
        count = 200
        selection = chebyshev_contiguous
        seed = 770421
        anchor_row = 199
        anchor_col = 80
    }}
    ownership_volume = pirate_border {{
        owner = "pirate"
        count = 50
        selection = chebyshev_contiguous
        adjacent_to = "terran_core"
        seed = 770421
    }}
    planet_surface_payload = owned_system_payload {{
        applies_to = owned_systems
        planets_per_system_min = 1
        surface_grid = "1x1"
        factory_min = 1
        cohort_min = 1
        category_map = {{
            pop_factory = {{ kind = Cohort depth = 3 }}
        }}
        resource = {{
            id = "tp_minerals"
            namespace = "tp"
            name = "minerals"
            display_name = "Minerals"
        }}
        modifier = {{
            pop_factory_minerals_produces_mult = 0.10
            pop_factory_minerals_upkeep_add = 1
        }}
    }}
    planet_surface_payload = neutral_system_payload {{
        applies_to = neutral_systems
        planets_per_system_min = 1
        surface_grid = "1x1"
        factory_min = 0
        cohort_min = 0
    }}
    fleet_ship_payload = terran_fleets {{
        owner = "terran"
        ownership_volume = "terran_core"
        enemy_ownership_volume = "pirate_border"
        fleet_count = 10
        ships_per_fleet = 20
        border_fleet_count = 6
        ship_class = "corvette"
        hull_seed = 100
        weapon_damage_seed = 25
        upkeep_per_ship = 2
        resource = {{
            id = "tp_energy"
            namespace = "tp"
            name = "energy"
            display_name = "Energy"
        }}
    }}
    fleet_ship_payload = pirate_fleets {{
        owner = "pirate"
        ownership_volume = "pirate_border"
        enemy_ownership_volume = "terran_core"
        fleet_count = 10
        ships_per_fleet = 40
        border_fleet_count = 8
        ship_class = "corvette"
        hull_seed = 80
        weapon_damage_seed = 30
        upkeep_per_ship = 3
        resource = {{
            id = "tp_energy"
            namespace = "tp"
            name = "energy"
            display_name = "Energy"
        }}
    }}
}}
"#,
        fixture_path_text()
    )
}

fn hydrate_pack() -> HydratedScenarioPack {
    let document = parse_raw_document(combined_clause().as_bytes()).expect("parse combined clause");
    hydrate_scenario(&document).expect("hydrate fleets/ships clause")
}

fn authority_root(pack: &HydratedScenarioPack) -> SimThing {
    pack.authority_root
        .clone()
        .expect("fleets/ships pack carries authority root")
}

fn scenario_from_root(root: SimThing) -> simthing_spec::SimThingScenarioSpec {
    simthing_spec::SimThingScenarioSpec {
        scenario_id: "tp_fleets_ships_0".to_string(),
        root,
        structural_grid: simthing_spec::SimThingScenarioGrid {
            frame: simthing_spec::SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance: simthing_spec::SimThingScenarioProvenance::default(),
    }
}

fn property_f32(thing: &SimThing, property_id: SimPropertyId) -> Option<f32> {
    thing
        .properties
        .get(&property_id)
        .and_then(|value| value.raw_lanes().first().copied())
}

fn property_string(thing: &SimThing, property_id: SimPropertyId) -> Option<String> {
    scenario_metadata_string(thing, property_id)
}

#[derive(Debug, Clone)]
struct FleetRecord {
    owner: String,
    posture: String,
    home_system: String,
    ships: Vec<SimThing>,
}

fn collect_fleets(root: &SimThing) -> Vec<FleetRecord> {
    let mut fleets = Vec::new();
    let scenario = scenario_from_root(root.clone());
    let systems = star_system_gridcells(&scenario).expect("star systems");
    for system in systems {
        let home = system_target_hint(system);
        for planet in &system.children {
            let Some(surface) = planet_surface_gridcell(planet) else {
                continue;
            };
            for child in &surface.children {
                if child.kind != SimThingKind::Fleet {
                    continue;
                }
                fleets.push(FleetRecord {
                    owner: owner_flow_owner_ref(child).expect("fleet owner ref"),
                    posture: property_string(child, TP_FLEET_POSTURE_PROPERTY_ID)
                        .expect("fleet posture"),
                    home_system: property_string(child, TP_FLEET_HOME_SYSTEM_PROPERTY_ID)
                        .unwrap_or_else(|| home.clone()),
                    ships: child.children.clone(),
                });
            }
        }
    }
    fleets
}

fn system_target_hint(system: &SimThing) -> String {
    format!(
        "row{}_col{}",
        system
            .properties
            .get(&simthing_spec::SCENARIO_STRUCTURAL_ROW_PROPERTY_ID)
            .and_then(|v| v.raw_lanes().first().copied())
            .unwrap_or(0.0) as u32,
        system
            .properties
            .get(&simthing_spec::SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
            .and_then(|v| v.raw_lanes().first().copied())
            .unwrap_or(0.0) as u32
    )
}

fn enemy_coords(pack: &HydratedScenarioPack, _own_owner: &str, enemy_owner: &str) -> BTreeSet<(u32, u32)> {
    pack.ownership_volumes
        .iter()
        .find(|volume| volume.owner == enemy_owner)
        .map(|volume| {
            volume
                .assigned_systems
                .iter()
                .map(|system| (system.row, system.col))
                .collect()
        })
        .unwrap_or_default()
}

fn is_adjacent_to_enemy(row: u32, col: u32, enemy_coords: &BTreeSet<(u32, u32)>) -> bool {
    for dr in -1_i32..=1 {
        for dc in -1_i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let Some(next_row) = offset_u32(row, dr) else {
                continue;
            };
            let Some(next_col) = offset_u32(col, dc) else {
                continue;
            };
            if enemy_coords.contains(&(next_row, next_col)) {
                return true;
            }
        }
    }
    false
}

fn offset_u32(value: u32, delta: i32) -> Option<u32> {
    if delta.is_negative() {
        value.checked_sub(delta.unsigned_abs())
    } else {
        value.checked_add(delta as u32)
    }
}

fn payload_by_owner<'a>(
    pack: &'a HydratedScenarioPack,
    owner: &str,
) -> &'a HydratedFleetShipPayload {
    pack.fleet_ship_payloads
        .iter()
        .find(|payload| payload.owner == owner)
        .expect("payload exists")
}

#[test]
fn tp_fleets_ships_0_table() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let spec = scenario_from_root(root.clone());
    let fleets = collect_fleets(&root);

    assert_eq!(pack.fleet_ship_payloads.len(), 2);
    assert_eq!(fleets.len(), 20, "total fleet count");

    let terran_fleets: Vec<_> = fleets.iter().filter(|fleet| fleet.owner == "terran").collect();
    let pirate_fleets: Vec<_> = fleets.iter().filter(|fleet| fleet.owner == "pirate").collect();
    assert_eq!(terran_fleets.len(), 10);
    assert_eq!(pirate_fleets.len(), 10);

    let terran_ships: usize = terran_fleets.iter().map(|fleet| fleet.ships.len()).sum();
    let pirate_ships: usize = pirate_fleets.iter().map(|fleet| fleet.ships.len()).sum();
    assert_eq!(terran_ships, 200);
    assert_eq!(pirate_ships, 400);

    let terran_payload = payload_by_owner(&pack, "terran");
    let pirate_payload = payload_by_owner(&pack, "pirate");
    assert_eq!(terran_payload.border_fleet_count, 6);
    assert_eq!(pirate_payload.border_fleet_count, 8);
    assert_eq!(
        terran_fleets
            .iter()
            .filter(|fleet| fleet.posture == "border")
            .count(),
        6
    );
    assert_eq!(
        terran_fleets
            .iter()
            .filter(|fleet| fleet.posture == "interior")
            .count(),
        4
    );
    assert_eq!(
        pirate_fleets
            .iter()
            .filter(|fleet| fleet.posture == "raid")
            .count(),
        8
    );
    assert_eq!(
        pirate_fleets
            .iter()
            .filter(|fleet| fleet.posture == "garrison")
            .count(),
        2
    );

    let terran_enemy_coords = enemy_coords(&pack, "terran", "pirate");
    let pirate_enemy_coords = enemy_coords(&pack, "pirate", "terran");
    for placement in terran_payload
        .placements
        .iter()
        .filter(|placement| placement.posture == "border")
    {
        assert!(is_adjacent_to_enemy(
            placement.row,
            placement.col,
            &terran_enemy_coords
        ));
    }
    for placement in pirate_payload
        .placements
        .iter()
        .filter(|placement| placement.posture == "raid")
    {
        assert!(is_adjacent_to_enemy(
            placement.row,
            placement.col,
            &pirate_enemy_coords
        ));
    }

    let owners: BTreeSet<_> = game_session_owners(&spec)
        .expect("owners")
        .into_iter()
        .filter_map(owner_entity_id)
        .collect();
    for fleet in &fleets {
        assert!(owners.contains(&fleet.owner));
        assert_eq!(fleet.ships.len() as u32, if fleet.owner == "terran" { 20 } else { 40 });
        for ship in &fleet.ships {
            assert_eq!(ship.kind, SimThingKind::Cohort);
            assert_eq!(
                owner_flow_owner_ref(ship).as_deref(),
                Some(fleet.owner.as_str())
            );
            assert!(property_f32(ship, TP_SHIP_HULL_PROPERTY_ID).is_some());
            assert!(property_f32(ship, TP_SHIP_WEAPON_DAMAGE_PROPERTY_ID).is_some());
            assert!(property_f32(ship, TP_SHIP_UPKEEP_PROPERTY_ID).is_some());
            assert_eq!(
                property_string(ship, TP_SHIP_CLASS_PROPERTY_ID).as_deref(),
                Some("corvette")
            );
            assert!(
                owner_flow_deficit(ship).unwrap_or(0) > 0,
                "ship upkeep RF deficit"
            );
        }
    }

    let game_session = game_session_child(&spec).expect("GameSession");
    assert!(game_session
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::Owner)
        .all(|owner| owner.children.is_empty()));

    let location = evaluate_planet_child_locations(&spec);
    assert_ne!(
        location.classification,
        simthing_spec::PlanetChildLocationAdmissionClassification::Rejected
    );

    let admission = evaluate_planet_child_rf_admission(&spec);
    assert_ne!(
        admission.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(admission.deficit_total >= 200 * 2 + 400 * 3);
    let reduce_up = evaluate_planet_child_rf_reduce_up(&spec);
    assert_ne!(
        reduce_up.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(reduce_up.bucket_count >= 2);

    let mut reparented = root.clone();
    let scenario = scenario_from_root(reparented.clone());
    let systems = star_system_gridcells(&scenario).expect("systems");
    let terran_with_fleet = systems
        .iter()
        .filter(|system| owner_flow_owner_ref(system).as_deref() == Some("terran"))
        .find(|system| {
            system.children.iter().any(|planet| {
                planet_surface_gridcell(planet)
                    .map(|surface| {
                        surface
                            .children
                            .iter()
                            .any(|child| child.kind == SimThingKind::Fleet)
                    })
                    .unwrap_or(false)
            })
        })
        .expect("terran system with fleet");
    let destination = systems
        .iter()
        .filter(|system| owner_flow_owner_ref(system).as_deref() == Some("terran"))
        .find(|system| system.id != terran_with_fleet.id)
        .expect("second terran system");
    let source = terran_with_fleet;
    let ship_count = source
        .children
        .iter()
        .find_map(|planet| planet_surface_gridcell(planet))
        .and_then(|surface| {
            surface
                .children
                .iter()
                .find(|child| child.kind == SimThingKind::Fleet)
        })
        .expect("fleet on source surface")
        .children
        .len();

    let source_id = source.id;
    let destination_id = destination.id;
    let game_session = game_session_child_mut(&mut reparented).expect("GameSession");
    let galaxy_map = game_session_galaxy_map_mut(game_session).expect("GalaxyMap");
    let moved_fleet = {
        let source_system = galaxy_map
            .children
            .iter_mut()
            .find(|child| child.id == source_id)
            .expect("source system");
        let source_surface = source_system
            .children
            .iter_mut()
            .find_map(|planet| {
                planet
                    .children
                    .iter_mut()
                    .find(|child| is_surface_gridcell(child))
            })
            .expect("source surface mutable");
        let fleet_index = source_surface
            .children
            .iter()
            .position(|child| child.kind == SimThingKind::Fleet)
            .expect("fleet index");
        source_surface.children.remove(fleet_index)
    };
    {
        let destination_system = galaxy_map
            .children
            .iter_mut()
            .find(|child| child.id == destination_id)
            .expect("destination system");
        let destination_surface = destination_system
            .children
            .iter_mut()
            .find_map(|planet| {
                planet
                    .children
                    .iter_mut()
                    .find(|child| is_surface_gridcell(child))
            })
            .expect("destination surface mutable");
        destination_surface.add_child(moved_fleet);
    }

    let reparent_spec = scenario_from_root(reparented.clone());
    let destination_system = star_system_gridcells(&reparent_spec)
        .expect("systems")
        .into_iter()
        .find(|system| system.id == destination_id)
        .expect("destination after reparent");
    let moved = destination_system
        .children
        .iter()
        .find_map(|planet| planet_surface_gridcell(planet))
        .and_then(|surface| surface.children.iter().find(|child| child.kind == SimThingKind::Fleet))
        .expect("fleet reparented");
    assert_eq!(moved.children.len(), ship_count);
    assert!(moved
        .children
        .iter()
        .all(|ship| ship.kind == SimThingKind::Cohort));
    let reparent_admission = evaluate_planet_child_rf_admission(&reparent_spec);
    assert_ne!(
        reparent_admission.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
}

fn game_session_child_mut(root: &mut SimThing) -> Option<&mut SimThing> {
    root.children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
}

fn game_session_galaxy_map_mut<'a>(session: &'a mut SimThing) -> Option<&'a mut SimThing> {
    session
        .children
        .iter_mut()
        .find(|child| is_galaxy_map_entity(child))
}

#[test]
fn unsupported_fleet_ship_payload_fields_hard_error_with_span() {
    let source = combined_clause().replace(
        "border_fleet_count = 6",
        "border_fleet_count = 6\n        unknown_fleet_field = true",
    );
    let document = parse_raw_document(source.as_bytes()).expect("parse unsupported field source");
    let err = hydrate_scenario(&document).expect_err("unsupported field must hard-error");
    assert!(
        err.to_string().contains("unsupported fleet_ship_payload field"),
        "{err}"
    );
    assert!(err.span.is_some());
}