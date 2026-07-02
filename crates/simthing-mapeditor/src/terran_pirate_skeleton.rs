//! Horizon terran-pirate scenario skeleton as `SimThingScenarioSpec` authority.
//!
//! Four gridcell Locations with a forked hyperlane link graph (hub → corridor → branch/choke).
//! Provenance: `TERRAN-PIRATE-SCENARIO-SKELETON-0`. No execution, render, or domain engines.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioLink,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

pub const TERRAN_PIRATE_SKELETON_SCENARIO_ID: &str = "terran_pirate_skeleton";
pub const TERRAN_PIRATE_SKELETON_PROVENANCE_SOURCE: &str = "TERRAN-PIRATE-SCENARIO-SKELETON-0";

fn add_gridcell(
    map: &mut SimThing,
    system_id: u32,
    row: u32,
    col: u32,
) -> SimThingStructuralGridPlacement {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
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
    let mut payload = SimThing::new(SimThingKind::Cohort, 0);
    payload.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    cell.add_child(payload);
    let cell_raw = cell.id.raw();
    let placement = SimThingStructuralGridPlacement {
        location_id: format!("skeleton_cell_{system_id}"),
        target_id: format!("skeleton_cell_{system_id}"),
        system_id,
        row,
        col,
        simthing_id_raw: cell_raw,
    };
    map.add_child(cell);
    placement
}

/// Canonical horizon skeleton authority: forked link graph over four gridcell Locations.
pub fn terran_pirate_skeleton_scenario_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut map = SimThing::new(SimThingKind::Location, 0);
    let map_raw = map.id.raw();

    // Hub (1) — corridor (2) — choke (4); corridor forks to branch (3).
    let placement_hub = add_gridcell(&mut map, 1, 0, 0);
    let placement_corridor = add_gridcell(&mut map, 2, 0, 1);
    let placement_choke = add_gridcell(&mut map, 4, 0, 2);
    let placement_branch = add_gridcell(&mut map, 3, 1, 1);
    root.add_child(map);

    SimThingScenarioSpec {
        scenario_id: TERRAN_PIRATE_SKELETON_SCENARIO_ID.to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 4,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![
                placement_hub,
                placement_corridor,
                placement_choke,
                placement_branch,
            ],
        },
        links: vec![
            SimThingScenarioLink {
                from_system_id: "1".to_string(),
                to_system_id: "2".to_string(),
            },
            SimThingScenarioLink {
                from_system_id: "2".to_string(),
                to_system_id: "3".to_string(),
            },
            SimThingScenarioLink {
                from_system_id: "2".to_string(),
                to_system_id: "4".to_string(),
            },
        ],
        provenance: SimThingScenarioProvenance {
            source: TERRAN_PIRATE_SKELETON_PROVENANCE_SOURCE.to_string(),
            generator_seed: 0,
            generator_shape: "terran_pirate_skeleton".to_string(),
            ..SimThingScenarioProvenance::default()
        },
    }
}

/// Exact f32 integer inputs in dense placement order (hub, corridor, choke, branch).
pub fn terran_pirate_skeleton_dense_inputs() -> Vec<f32> {
    vec![10.0, 20.0, 40.0, 30.0]
}
