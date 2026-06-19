//! DRIVER-STRUCTURAL-ATLAS-PARTITION-0 — partitioned atlas -> sim scheduler integration.

use std::sync::Mutex;

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{
    compile_structural_n4_atlas, compile_structured_field_mapping_plan, StructuralAtlasAdmission,
    StructuralAtlasPartitionProfile,
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
    let ctx = GpuContext::new_blocking().expect("GPU required for atlas scheduler integration");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn synthetic_oversize_scenario() -> simthing_spec::SimThingScenarioSpec {
    use simthing_spec::{
        structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioSpec,
        SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    };

    let placements = [(1u32, 0u32, 0u32), (2u32, 10u32, 10u32)];
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
        scenario_id: "synthetic_partition_scheduler".to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 11,
                height: 11,
                occupied_cells: 2,
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
        name: "partition_slice".into(),
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
        gamma_neighbor: 0.0,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
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
    seeds: &[(u32, f32)],
) -> Vec<f32> {
    let cells = theater.frame_width * theater.frame_height;
    let mut values = vec![0.0f32; (cells * n_dims) as usize];
    for &(system_id, seed) in seeds {
        if let Some(placement) = theater.placement_for_system(system_id) {
            let slot = theater.cell_slot(simthing_driver::StructuralGridCoordinate {
                col: placement.col,
                row: placement.row,
            });
            values[idx(slot, 0, n_dims)] = seed;
        }
    }
    values
}

#[test]
fn structural_atlas_scheduler_integration_partitioned_plans_proof_and_none() {
    let scenario = synthetic_oversize_scenario();
    let original_id = scenario.scenario_id.clone();
    let profile = StructuralAtlasPartitionProfile {
        max_theater_width: REGION_FIELD_STANDARD_MAX_GRID,
        max_theater_height: REGION_FIELD_STANDARD_MAX_GRID,
        include_overlap_halo: false,
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

    let occupied_entries: Vec<_> = atlas
        .theaters
        .iter()
        .filter(|entry| !entry.theater.occupied_cells.is_empty())
        .collect();
    assert!(
        occupied_entries.len() >= 2,
        "need at least two occupied partition theaters"
    );

    let mut plans = Vec::new();
    let mut stored_values = Vec::new();
    let mut cpu_oracles = Vec::new();

    for entry in &occupied_entries {
        assert_eq!(
            entry.theater.frame_width, entry.theater.frame_height,
            "region field admission is square; partition tile must be square"
        );
        let grid = entry.theater.frame_width;
        let structured =
            compile_region_field_preview(&structured_field_spec(grid)).expect("field admission");
        let plan =
            compile_structured_field_mapping_plan(&entry.theater, &structured, 1, Vec::new(), 0)
                .expect("mapping plan");
        let seeds = entry
            .theater
            .system_placements
            .iter()
            .map(|placement| (placement.system_id, 40.0 + placement.system_id as f32))
            .collect::<Vec<_>>();
        let values = seed_values_for_theater(&entry.theater, 4, &seeds);
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
        assert_eq!(proof.theater_outputs.len(), occupied_entries.len());
        for (output, cpu) in proof.theater_outputs.iter().zip(cpu_oracles.iter()) {
            let gpu = output.proof_values.as_ref().expect("proof values");
            assert_eq!(gpu.len(), cpu.len());
            for (i, (g, c)) in gpu.iter().zip(cpu.iter()).enumerate() {
                assert!((g - c).abs() < 1e-4, "parity mismatch at {i}");
            }
        }
        assert!(!debug_readback_allowed());

        let mut none_values = Vec::new();
        for entry in &occupied_entries {
            let seeds = entry
                .theater
                .system_placements
                .iter()
                .map(|placement| (placement.system_id, 10.0))
                .collect::<Vec<_>>();
            none_values.push(seed_values_for_theater(&entry.theater, 4, &seeds));
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
            "DRIVER-STRUCTURAL-ATLAS-PARTITION-0: REAL_ADAPTER_OBSERVED (partition_scheduler)"
        );
    });

    assert_eq!(scenario.scenario_id, original_id);
    assert_eq!(scenario.structural_grid.frame.width, 11);
}
