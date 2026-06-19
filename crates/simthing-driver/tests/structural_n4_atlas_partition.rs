//! DRIVER-STRUCTURAL-ATLAS-PARTITION-0 — structural atlas partition/admission proofs.

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{
    compile_structural_n4_atlas, compile_structural_n4_theater, StructuralAtlasAdmission,
    StructuralAtlasPartitionProfile, StructuralTheaterAdmission,
};
use simthing_spec::{
    deserialize_scenario_authority, structural_property_value_u32, validate_scenario_links,
    validate_stead_mapping_consistency, MappingExecutionProfile, SimThingScenarioGrid,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    REGION_FIELD_STANDARD_MAX_GRID, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

const TERRAN_PIRATE_SKELETON_SCENARIO_JSON: &str =
    include_str!("../../../scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json");

const FORBIDDEN_PARTITION_TOKENS: &[&str] = &[
    "pathfinding",
    "predecessor",
    "came_from",
    "route_object",
    "movement_order",
    "border_service",
    "frontline_service",
    "cpu_planner",
    "semantic_wgsl",
    "SimGpuMappingAtlasScheduler",
    "StructuredFieldStencilOp",
    "simthing_mapeditor",
];

fn default_partition_profile() -> StructuralAtlasPartitionProfile {
    StructuralAtlasPartitionProfile {
        max_theater_width: REGION_FIELD_STANDARD_MAX_GRID,
        max_theater_height: REGION_FIELD_STANDARD_MAX_GRID,
        include_overlap_halo: false,
    }
}

fn synthetic_scenario(
    width: u32,
    height: u32,
    placements: &[(u32, u32, u32)],
) -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut map = SimThing::new(SimThingKind::Location, 0);
    let map_raw = map.id.raw();
    let mut grid_placements = Vec::new();

    for &(system_id, col, row) in placements {
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
        map.add_child(cell);
        grid_placements.push(SimThingStructuralGridPlacement {
            location_id: format!("cell_{system_id}"),
            target_id: format!("cell_{system_id}"),
            system_id,
            row,
            col,
            simthing_id_raw: cell_raw,
        });
    }

    root.add_child(map);
    SimThingScenarioSpec {
        scenario_id: "synthetic_atlas_partition".to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width,
                height,
                occupied_cells: placements.len() as u64,
            },
            map_container_id: map_raw.to_string(),
            placements: grid_placements,
        },
        links: Vec::new(),
        provenance: Default::default(),
    }
}

#[test]
fn structural_atlas_partition_terran_pirate_remains_single_bounded_theater() {
    let scenario = deserialize_scenario_authority(TERRAN_PIRATE_SKELETON_SCENARIO_JSON)
        .expect("deserialize canonical skeleton");
    validate_stead_mapping_consistency(&scenario).expect("STEAD valid");
    validate_scenario_links(&scenario).expect("links valid");
    let original_frame = scenario.structural_grid.frame;

    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        default_partition_profile(),
    )
    .expect("atlas compile");

    match admission {
        StructuralAtlasAdmission::Single(theater) => {
            assert_eq!(theater.frame_width, 8);
            assert_eq!(theater.frame_height, 8);
            assert_eq!(theater.occupied_cells.len(), 4);
        }
        StructuralAtlasAdmission::Partitioned(atlas) => {
            assert_eq!(atlas.theaters.len(), 1);
            assert_eq!(atlas.theaters[0].theater.frame_width, 8);
        }
        StructuralAtlasAdmission::Deferred { .. } => {
            panic!("Terran Pirate must not defer under atlas partition")
        }
    }

    assert_eq!(scenario.structural_grid.frame, original_frame);
}

#[test]
fn structural_atlas_partition_oversize_11x11_returns_partitioned_atlas() {
    let scenario = synthetic_scenario(11, 11, &[(1, 0, 0)]);
    let original_frame = scenario.structural_grid.frame;

    let legacy =
        compile_structural_n4_theater(&scenario, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("legacy compile");
    assert!(matches!(
        legacy,
        StructuralTheaterAdmission::AtlasDeferred { .. }
    ));

    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        default_partition_profile(),
    )
    .expect("atlas compile");

    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("11x11 must partition");
    };
    assert_eq!(atlas.original_frame_width, 11);
    assert_eq!(atlas.original_frame_height, 11);
    assert!(atlas.theaters.len() >= 2);
    assert_eq!(scenario.structural_grid.frame, original_frame);
}

#[test]
fn structural_atlas_partition_preserves_original_frame_metadata() {
    let scenario = synthetic_scenario(11, 11, &[(1, 0, 0), (2, 10, 10)]);
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        default_partition_profile(),
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };
    assert_eq!(atlas.original_frame_width, 11);
    assert_eq!(atlas.original_frame_height, 11);
    assert_eq!(scenario.structural_grid.frame.width, 11);
    assert_eq!(scenario.structural_grid.frame.height, 11);
}

#[test]
fn structural_atlas_partition_theaters_respect_max_grid_cap() {
    let scenario = synthetic_scenario(11, 11, &[(1, 5, 5)]);
    let profile = default_partition_profile();
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        profile,
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };
    for entry in &atlas.theaters {
        assert!(entry.theater.frame_width <= profile.max_theater_width);
        assert!(entry.theater.frame_height <= profile.max_theater_height);
    }
}

#[test]
fn structural_atlas_partition_does_not_shrink_or_mutate_scenario_authority() {
    let scenario = synthetic_scenario(11, 11, &[(1, 0, 0)]);
    let before = scenario.structural_grid.frame;
    let _ = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        default_partition_profile(),
    )
    .expect("atlas compile");
    assert_eq!(scenario.structural_grid.frame, before);
    assert_eq!(scenario.structural_grid.placements[0].col, 0);
    assert_eq!(scenario.structural_grid.placements[0].row, 0);
}

#[test]
fn structural_atlas_partition_records_or_defers_cross_partition_edges() {
    let scenario = synthetic_scenario(11, 11, &[(1, 9, 0), (2, 10, 0)]);
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        default_partition_profile(),
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };
    assert_eq!(atlas.deferred_cross_partition_edges.len(), 1);
    let deferred = &atlas.deferred_cross_partition_edges[0];
    assert_eq!(deferred.global_a.col, 9);
    assert_eq!(deferred.global_b.col, 10);
    assert_ne!(deferred.partition_index_a, deferred.partition_index_b);
}

#[test]
fn structural_atlas_partition_forbidden_token_guard() {
    let source = include_str!("../src/structural_n4_atlas_partition.rs");
    for token in FORBIDDEN_PARTITION_TOKENS {
        assert!(
            !source.contains(token),
            "structural_n4_atlas_partition must not contain `{token}`"
        );
    }
    assert!(
        !source.contains("simthing_sim"),
        "partition module must not call sim scheduler"
    );
}
