//! Minimal prior-runtime-vertical-test-shaped `SimThingScenarioSpec` seed fixture.
//!
//! Encodes structural authority only: world root, map container, two gridcell Locations with
//! cohort payload children, canonical adjacency link, and provenance marking
//! `VERTICAL-TEST-SCENARIO-SEED-0`. No execution state, render metadata, or GPU buffers.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioLink,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

pub const RUNTIME_VERTICAL_SEED_SCENARIO_ID: &str = "runtime_vertical_seed";
pub const RUNTIME_VERTICAL_SEED_PROVENANCE_SOURCE: &str = "VERTICAL-TEST-SCENARIO-SEED-0";

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
        location_id: format!("vertical_cell_{system_id}"),
        target_id: format!("vertical_cell_{system_id}"),
        system_id,
        row,
        col,
        simthing_id_raw: cell_raw,
    };
    map.add_child(cell);
    placement
}

/// Canonical minimal vertical-test seed authority object for save/load and Studio projection.
pub fn runtime_vertical_seed_scenario_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut map = SimThing::new(SimThingKind::Location, 0);
    let map_raw = map.id.raw();
    let placement_a = add_gridcell(&mut map, 1, 2, 3);
    let placement_b = add_gridcell(&mut map, 2, 2, 4);
    root.add_child(map);
    SimThingScenarioSpec {
        scenario_id: RUNTIME_VERTICAL_SEED_SCENARIO_ID.to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 2,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![placement_a, placement_b],
        },
        links: vec![SimThingScenarioLink {
            from_system_id: "1".to_string(),
            to_system_id: "2".to_string(),
        }],
        provenance: SimThingScenarioProvenance {
            source: RUNTIME_VERTICAL_SEED_PROVENANCE_SOURCE.to_string(),
            generator_seed: 0,
            generator_shape: "vertical_test_seed".to_string(),
            ..SimThingScenarioProvenance::default()
        },
    }
}
