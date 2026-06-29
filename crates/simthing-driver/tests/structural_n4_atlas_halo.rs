//! DRIVER-STRUCTURAL-ATLAS-HALO-0 — one-cell structural halo admission proofs.

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{
    compile_structural_n4_atlas, StructuralAtlasAdmission, StructuralAtlasPartitionProfile,
    StructuralTheaterCellRole, StructuralTheaterCompileError,
};
use simthing_spec::{
    MappingExecutionProfile, SimThingScenarioGrid, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, REGION_FIELD_STANDARD_MAX_GRID,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

const FORBIDDEN_HALO_TOKENS: &[&str] = &[
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

fn halo_disabled_profile() -> StructuralAtlasPartitionProfile {
    StructuralAtlasPartitionProfile {
        max_theater_width: REGION_FIELD_STANDARD_MAX_GRID,
        max_theater_height: REGION_FIELD_STANDARD_MAX_GRID,
        include_overlap_halo: false,
    }
}

fn halo_enabled_profile() -> StructuralAtlasPartitionProfile {
    // East one-cell halo on a 10-wide owned tile needs an 11-wide bounded theater.
    StructuralAtlasPartitionProfile {
        max_theater_width: REGION_FIELD_STANDARD_MAX_GRID + 1,
        max_theater_height: REGION_FIELD_STANDARD_MAX_GRID + 1,
        include_overlap_halo: true,
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
            simthing_spec::structural_property_value_u32(system_id),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            simthing_spec::structural_property_value_u32(col),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            simthing_spec::structural_property_value_u32(row),
        );
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        payload.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            simthing_spec::structural_property_value_u32(system_id),
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
        scenario_id: "synthetic_atlas_halo".to_string(),
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

fn cross_partition_corner_scenario() -> SimThingScenarioSpec {
    // 12x12 with max-10 tiles: (9,0) and (10,0) are cross-partition N4 neighbors.
    synthetic_scenario(12, 12, &[(1, 9, 0), (2, 10, 0)])
}

#[test]
fn structural_atlas_halo_disabled_preserves_partition_deferral_behavior() {
    let scenario = cross_partition_corner_scenario();
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        halo_disabled_profile(),
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };
    assert_eq!(atlas.deferred_cross_partition_edges.len(), 1);
    assert!(atlas.halo_coverage.is_empty());
    for entry in &atlas.theaters {
        assert!(entry.halo_cells.is_empty());
    }
}

#[test]
fn structural_atlas_halo_enabled_adds_one_cell_structural_n4_halo() {
    let scenario = cross_partition_corner_scenario();
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        halo_enabled_profile(),
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };
    assert_eq!(atlas.deferred_cross_partition_edges.len(), 1);
    assert_eq!(atlas.halo_coverage.len(), 1);

    let halo_theaters: Vec<_> = atlas
        .theaters
        .iter()
        .filter(|entry| !entry.halo_cells.is_empty())
        .collect();
    assert!(
        halo_theaters.len() >= 2,
        "adjacent partitions should admit halo cells"
    );
    for entry in &halo_theaters {
        for halo in &entry.halo_cells {
            let tentative_col = halo.global_coord.col() as i64 - entry.origin.col as i64;
            let tentative_row = halo.global_coord.row() as i64 - entry.origin.row as i64;
            assert!(
                tentative_col.abs() <= 1 || tentative_row.abs() <= 1,
                "halo must be one structural N4 step from owned tile"
            );
        }
    }
}

#[test]
fn structural_atlas_halo_distinguishes_owned_from_halo_cells() {
    let scenario = cross_partition_corner_scenario();
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        halo_enabled_profile(),
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };

    let mut saw_owned = false;
    let mut saw_halo = false;
    for entry in &atlas.theaters {
        for placement in &entry.theater.system_placements {
            let local = simthing_driver::StructuralCoord::new(placement.col, placement.row);
            assert_eq!(entry.cell_role(local), StructuralTheaterCellRole::Owned);
            saw_owned = true;
        }
        for halo in &entry.halo_cells {
            assert_eq!(
                entry.cell_role(halo.local_coord),
                StructuralTheaterCellRole::Halo
            );
            assert!(
                !entry.theater.system_placements.iter().any(|placement| {
                    placement.col == halo.local_coord.col()
                        && placement.row == halo.local_coord.row()
                }),
                "halo cells must not appear in system_placements"
            );
            saw_halo = true;
        }
    }
    assert!(saw_owned);
    assert!(saw_halo);
}

#[test]
fn structural_atlas_halo_preserves_original_frame_metadata() {
    let scenario = cross_partition_corner_scenario();
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        halo_enabled_profile(),
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };
    assert_eq!(atlas.original_frame_width, 12);
    assert_eq!(atlas.original_frame_height, 12);
    assert_eq!(scenario.structural_grid.frame.width, 12);
    assert_eq!(scenario.structural_grid.frame.height, 12);
}

#[test]
fn structural_atlas_halo_does_not_mutate_scenario_authority() {
    let scenario = cross_partition_corner_scenario();
    let before = scenario.structural_grid.frame.clone();
    let placements_before: Vec<_> = scenario
        .structural_grid
        .placements
        .iter()
        .map(|placement| (placement.system_id, placement.col, placement.row))
        .collect();
    let _ = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        halo_enabled_profile(),
    )
    .expect("atlas compile");
    assert_eq!(scenario.structural_grid.frame, before);
    let placements_after: Vec<_> = scenario
        .structural_grid
        .placements
        .iter()
        .map(|placement| (placement.system_id, placement.col, placement.row))
        .collect();
    assert_eq!(placements_after, placements_before);
}

#[test]
fn structural_atlas_halo_rejects_or_defers_if_halo_exceeds_cap() {
    let scenario = synthetic_scenario(2, 1, &[(1, 0, 0), (2, 1, 0)]);
    let tight_profile = StructuralAtlasPartitionProfile {
        max_theater_width: 1,
        max_theater_height: 1,
        include_overlap_halo: true,
    };
    let err = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        tight_profile,
    )
    .expect_err("halo expansion should exceed 10x10 cap for cross-partition edge");
    assert!(matches!(
        err,
        StructuralTheaterCompileError::HaloExceedsTheaterCap { .. }
    ));
}

#[test]
fn structural_atlas_halo_global_local_coordinate_recovery() {
    let scenario = cross_partition_corner_scenario();
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        halo_enabled_profile(),
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };

    let expected_globals = [
        (1u32, simthing_driver::StructuralCoord::new(9, 0)),
        (2u32, simthing_driver::StructuralCoord::new(10, 0)),
    ];
    for entry in &atlas.theaters {
        for placement in &entry.theater.system_placements {
            let local = simthing_driver::StructuralCoord::new(placement.col, placement.row);
            let global = entry.global_from_local(local);
            let expected = expected_globals
                .iter()
                .find(|(system_id, _)| *system_id == placement.system_id)
                .map(|(_, global)| *global)
                .expect("owned placement global");
            assert_eq!(global, expected);
        }
        for halo in &entry.halo_cells {
            assert!(
                halo.global_coord == atlas.halo_coverage[0].edge.global_a
                    || halo.global_coord == atlas.halo_coverage[0].edge.global_b,
                "halo global coord must match a deferred cross-partition endpoint"
            );
            let recovered = entry.global_from_local(halo.local_coord);
            assert_eq!(recovered, halo.global_coord);
        }
    }
}

#[test]
fn structural_atlas_halo_forbidden_token_guard() {
    let source = include_str!("../src/structural_n4_atlas_partition.rs");
    for token in FORBIDDEN_HALO_TOKENS {
        assert!(
            !source.contains(token),
            "structural_n4_atlas_partition must not contain `{token}`"
        );
    }
    assert!(
        !source.contains("simthing_sim"),
        "halo/partition module must not call sim scheduler"
    );
}
