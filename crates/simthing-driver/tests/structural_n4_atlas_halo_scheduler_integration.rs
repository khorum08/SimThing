//! DRIVER-STRUCTURAL-ATLAS-HALO-0 — halo-augmented partition -> sim scheduler integration.

use std::sync::Mutex;

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{
    compile_structural_n4_atlas, compile_structured_field_mapping_plan, StructuralAtlasAdmission,
    StructuralAtlasPartitionProfile, StructuralGridCoordinate,
};
use simthing_gpu::{debug_readback_allowed, GpuContext};
use simthing_sim::{
    cpu_structured_field_horizon, CompiledMappingAtlas, MappingAtlasTickInputs,
    SimGpuMappingAtlasScheduler, SimGpuMappingReadbackPolicy,
};
use simthing_spec::{
    compile_region_field_preview, MappingExecutionProfile, RegionFieldCadenceSpec,
    RegionFieldGridProfile, RegionFieldOperatorSpec, RegionFieldSourcePolicySpec, RegionFieldSpec,
    REGION_FIELD_STANDARD_MAX_GRID,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for halo scheduler integration");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn synthetic_cross_partition_scenario() -> simthing_spec::SimThingScenarioSpec {
    use simthing_spec::{
        structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioSpec,
        SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    };

    let placements = [(1u32, 9u32, 9u32), (2u32, 10u32, 9u32), (3u32, 9u32, 10u32)];
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut map = SimThing::new(SimThingKind::Location, 0);
    let map_raw = map.id.raw();
    let mut grid_placements = Vec::new();

    for &(system_id, col, row) in &placements {
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
        scenario_id: "synthetic_halo_scheduler".to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 12,
                height: 12,
                occupied_cells: 3,
            },
            map_container_id: map_raw.to_string(),
            placements: grid_placements,
        },
        links: Vec::new(),
        provenance: Default::default(),
    }
}

fn structured_field_spec(grid_size: u32) -> RegionFieldSpec {
    RegionFieldSpec {
        name: "halo_partition_slice".into(),
        grid_size,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: 2.0,
            chi: 0.25,
            choke_output_col: Some(1),
        },
        horizon: 1,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.5,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::ExtendedSquare,
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    }
}

fn seed_values_for_theater(
    theater: &simthing_driver::CompiledStructuralN4Theater,
    n_dims: u32,
    owned_seeds: &[(u32, f32)],
    halo_seeds: &[(StructuralGridCoordinate, f32)],
) -> Vec<f32> {
    let cells = theater.frame_width * theater.frame_height;
    let mut values = vec![0.0f32; (cells * n_dims) as usize];
    for &(system_id, seed) in owned_seeds {
        if let Some(placement) = theater.placement_for_system(system_id) {
            let slot = theater.cell_slot(StructuralGridCoordinate {
                col: placement.col,
                row: placement.row,
            });
            values[idx(slot, 0, n_dims)] = seed;
        }
    }
    for &(coord, seed) in halo_seeds {
        let slot = theater.cell_slot(coord);
        values[idx(slot, 0, n_dims)] = seed;
    }
    values
}

#[test]
fn structural_atlas_halo_scheduler_integration_proof_and_none() {
    let scenario = synthetic_cross_partition_scenario();
    let original_id = scenario.scenario_id.clone();
    let profile = StructuralAtlasPartitionProfile {
        max_theater_width: REGION_FIELD_STANDARD_MAX_GRID + 1,
        max_theater_height: REGION_FIELD_STANDARD_MAX_GRID + 1,
        include_overlap_halo: true,
    };
    let admission = compile_structural_n4_atlas(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
        profile,
    )
    .expect("atlas compile");
    let StructuralAtlasAdmission::Partitioned(atlas) = admission else {
        panic!("expected partitioned atlas");
    };

    let halo_entries: Vec<_> = atlas
        .theaters
        .iter()
        .filter(|entry| {
            !entry.halo_cells.is_empty() && entry.theater.frame_width == entry.theater.frame_height
        })
        .collect();
    assert!(
        !halo_entries.is_empty(),
        "at least one square halo-augmented theater must be schedulable"
    );

    let occupied_entries: Vec<_> = atlas
        .theaters
        .iter()
        .filter(|entry| !entry.theater.system_placements.is_empty())
        .collect();
    assert!(
        occupied_entries.len() >= 2,
        "need owned cells in both adjacent partitions"
    );

    let mut plans = Vec::new();
    let mut stored_values = Vec::new();
    let mut cpu_oracles = Vec::new();

    let halo_count = halo_entries.len();
    for entry in &halo_entries {
        let grid = entry.theater.frame_width;
        assert_eq!(grid, entry.theater.frame_height);
        let structured =
            compile_region_field_preview(&structured_field_spec(grid)).expect("field admission");
        let plan =
            compile_structured_field_mapping_plan(&entry.theater, &structured, 1, Vec::new(), 0)
                .expect("mapping plan");

        let owned_seeds: Vec<_> = entry
            .theater
            .system_placements
            .iter()
            .map(|placement| (placement.system_id, 40.0 + placement.system_id as f32))
            .collect();
        let halo_seeds: Vec<_> = entry
            .halo_cells
            .iter()
            .map(|halo| (halo.local_coord, 100.0))
            .collect();
        let values = seed_values_for_theater(&entry.theater, 4, &owned_seeds, &halo_seeds);
        let config = simthing_driver::compiled_stencil_to_gpu_config(&structured.stencil);
        cpu_oracles.push(cpu_structured_field_horizon(&values, &config, 1));
        plans.push(plan);
        stored_values.push(values);
    }

    let proof_inputs: Vec<_> = stored_values
        .iter()
        .map(|values| simthing_sim::MappingTickInputs {
            structured_field_values: std::slice::from_ref(values),
            interleaved_values: None,
        })
        .collect();

    let mapping_atlas = CompiledMappingAtlas { plans };

    with_gpu(|ctx| {
        let mut scheduler =
            SimGpuMappingAtlasScheduler::new(ctx, mapping_atlas).expect("scheduler");
        let proof = scheduler
            .tick(
                ctx,
                MappingAtlasTickInputs {
                    theater_inputs: proof_inputs,
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");
        assert_eq!(proof.theater_outputs.len(), halo_count);
        for (output, cpu) in proof.theater_outputs.iter().zip(cpu_oracles.iter()) {
            let gpu = output.proof_values.as_ref().expect("proof values");
            assert_eq!(gpu.len(), cpu.len());
            for (i, (g, c)) in gpu.iter().zip(cpu.iter()).enumerate() {
                assert!((g - c).abs() < 1e-4, "parity mismatch at {i}");
            }
        }
        assert!(!debug_readback_allowed());

        let mut none_values = Vec::new();
        for entry in &halo_entries {
            let owned_seeds: Vec<_> = entry
                .theater
                .system_placements
                .iter()
                .map(|placement| (placement.system_id, 10.0))
                .collect();
            let values = seed_values_for_theater(&entry.theater, 4, &owned_seeds, &[]);
            none_values.push(values);
        }
        let none_inputs: Vec<_> = none_values
            .iter()
            .map(|values| simthing_sim::MappingTickInputs {
                structured_field_values: std::slice::from_ref(values),
                interleaved_values: None,
            })
            .collect();

        let none = scheduler
            .tick(
                ctx,
                MappingAtlasTickInputs {
                    theater_inputs: none_inputs,
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");
        for output in &none.theater_outputs {
            assert!(output.proof_values.is_none());
        }
        assert!(!debug_readback_allowed());
        eprintln!(
            "DRIVER-STRUCTURAL-ATLAS-HALO-0: REAL_ADAPTER_OBSERVED (halo_partition_scheduler)"
        );
    });

    assert_eq!(scenario.scenario_id, original_id);
    assert_eq!(scenario.structural_grid.frame.width, 12);
}
