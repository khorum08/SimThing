//! SESSION-GALAXYMAP-WORLDSTATE-0 — canonical GalaxyMap structural N4 theater admission.

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{
    compile_structural_n4_theater, AtlasDeferralReason, StructuralTheaterAdmission,
};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_scenario_metadata_to_root, make_galaxy_map,
    make_owner_entity, structural_property_value_u32, MappingExecutionProfile,
    SimThingScenarioGrid, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn canonical_galaxymap_single_cell_scenario() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    apply_scenario_metadata_to_root(
        &mut root,
        "canonical_galaxymap_compile",
        &Default::default(),
        SCENARIO_SCHEMA_VERSION,
    );
    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(make_owner_entity(
        "compile_owner",
        "Compile Owner",
        "player",
    ));
    let mut galaxy_map = make_galaxy_map("compile_galaxy", "Compile Galaxy");
    let map_raw = galaxy_map.id.raw();

    let mut cell = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut cell, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
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
    galaxy_map.add_child(cell);
    game_session.add_child(galaxy_map);
    root.add_child(game_session);

    SimThingScenarioSpec {
        scenario_id: "canonical_galaxymap_compile".into(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 1,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![SimThingStructuralGridPlacement {
                location_id: "cell_1".into(),
                target_id: "cell_1".into(),
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
fn canonical_galaxymap_fixture_admits_structural_n4_theater() {
    let scenario = canonical_galaxymap_single_cell_scenario();
    let admission =
        compile_structural_n4_theater(&scenario, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("compile canonical galaxymap theater");

    match admission {
        StructuralTheaterAdmission::Admit(theater) => {
            assert_eq!(theater.frame_width, 8);
            assert_eq!(theater.frame_height, 8);
            assert_eq!(theater.occupied_cells.len(), 1);
        }
        StructuralTheaterAdmission::AtlasDeferred { reason, .. } => {
            panic!("8x8 single-cell canonical galaxymap must admit, got {reason:?}");
        }
    }
}

#[test]
fn canonical_galaxymap_oversize_frame_returns_atlas_deferral() {
    let mut scenario = canonical_galaxymap_single_cell_scenario();
    scenario.structural_grid.frame.width = 11;
    scenario.structural_grid.frame.height = 11;
    let admission =
        compile_structural_n4_theater(&scenario, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("compile oversize");

    match admission {
        StructuralTheaterAdmission::AtlasDeferred { reason, .. } => {
            assert!(matches!(
                reason,
                AtlasDeferralReason::FrameExceedsStandardMaxGrid { .. }
            ));
        }
        StructuralTheaterAdmission::Admit(_) => panic!("oversize must defer"),
    }
}
