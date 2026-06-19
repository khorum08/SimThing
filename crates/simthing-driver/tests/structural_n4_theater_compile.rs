//! STRUCTURAL-N4-THEATER-COMPILE-0 — driver structural theater admission guards.

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{
    compile_structural_n4_theater, AtlasDeferralReason, StructuralTheaterAdmission,
};
use simthing_spec::{
    structural_property_value_u32, MappingExecutionProfile, SimThingScenarioGrid,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

const FORBIDDEN_COMPILE_SURFACE: &[&str] = &[
    "pathfinding",
    "predecessor",
    "came_from",
    "route_object",
    "movement_order",
    "border_service",
    "frontline_service",
    "cpu_planner",
    "semantic_wgsl",
];

fn synthetic_single_cell_scenario(width: u32, height: u32) -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut map = SimThing::new(SimThingKind::Location, 0);
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    let mut payload = SimThing::new(SimThingKind::Cohort, 0);
    payload.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    cell.add_child(payload);
    let cell_raw = cell.id.raw();
    let map_raw = map.id.raw();
    map.add_child(cell);
    root.add_child(map);
    SimThingScenarioSpec {
        scenario_id: "synthetic_oversize_theater".to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width,
                height,
                occupied_cells: 1,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![SimThingStructuralGridPlacement {
                location_id: "cell_1".to_string(),
                target_id: "cell_1".to_string(),
                system_id: 1,
                row: 0,
                col: 0,
                simthing_id_raw: cell_raw,
            }],
        },
        links: Vec::new(),
        provenance: Default::default(),
    }
}

#[test]
fn structural_n4_theater_oversize_frame_returns_typed_atlas_deferral() {
    let scenario = synthetic_single_cell_scenario(11, 11);
    let original_frame = scenario.structural_grid.frame;
    let admission =
        compile_structural_n4_theater(&scenario, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("compile oversize theater");

    match admission {
        StructuralTheaterAdmission::AtlasDeferred {
            frame_width,
            frame_height,
            occupied_cells,
            reason,
        } => {
            assert_eq!(frame_width, 11);
            assert_eq!(frame_height, 11);
            assert_eq!(occupied_cells, 1);
            assert_eq!(
                reason,
                AtlasDeferralReason::FrameExceedsStandardMaxGrid {
                    width: 11,
                    height: 11,
                    max_grid: 10,
                }
            );
        }
        StructuralTheaterAdmission::Admit(_) => {
            panic!("oversize frame must not admit to bounded first-slice execution")
        }
    }

    assert_eq!(scenario.structural_grid.frame, original_frame);
    assert_eq!(scenario.structural_grid.placements.len(), 1);
    assert_eq!(scenario.structural_grid.placements[0].col, 0);
    assert_eq!(scenario.structural_grid.placements[0].row, 0);
}

#[test]
fn structural_n4_theater_small_frame_admits_without_shrinking_layout() {
    let scenario = synthetic_single_cell_scenario(8, 8);
    let admission =
        compile_structural_n4_theater(&scenario, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("compile small theater");

    let StructuralTheaterAdmission::Admit(theater) = admission else {
        panic!("8x8 theater must admit");
    };
    assert_eq!(theater.frame_width, 8);
    assert_eq!(theater.frame_height, 8);
    assert_eq!(theater.occupied_cells.len(), 1);
    assert_eq!(scenario.structural_grid.frame.width, 8);
    assert_eq!(scenario.structural_grid.frame.height, 8);
}

#[test]
fn structural_n4_theater_compile_surface_forbidden_token_guard() {
    let compile_src = include_str!("../src/structural_n4_theater_compile.rs");
    let bridge_src = include_str!("../src/w_impedance_compose_bridge.rs");
    for token in FORBIDDEN_COMPILE_SURFACE {
        assert!(
            !compile_src.contains(token),
            "structural_n4_theater_compile must not contain `{token}`"
        );
        assert!(
            !bridge_src.contains(token),
            "w_impedance_compose_bridge must not contain `{token}`"
        );
    }
}
