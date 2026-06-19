//! SIM-MAPPING-ATLAS-SCHEDULER-0 — driver→sim atlas scheduler integration proof.

use std::sync::Mutex;

use simthing_driver::{
    compile_mapping_plan_from_admitted_theater, compile_structural_n4_theater,
    MappingPlanCompileSpec, StructuralGridCoordinate, StructuralTheaterAdmission,
};
use simthing_gpu::{debug_readback_allowed, max_d_field_error, GpuContext, MIN_PLUS_INF};
use simthing_sim::{
    cpu_min_plus_d_from_composed_interleaved, cpu_structured_field_horizon, CompiledMappingAtlas,
    CompiledMappingPlan, CompiledMappingStep, MappingAtlasTickInputs, MappingTheaterSlot,
    SimGpuMappingAtlasScheduler, SimGpuMappingReadbackPolicy,
};
use simthing_spec::{
    compile_region_field_preview, compile_w_impedance_compose_preview,
    deserialize_scenario_authority, validate_scenario_links, validate_stead_mapping_consistency,
    MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldSourcePolicySpec, RegionFieldSpec, SimThingScenarioSpec,
    WImpedanceComposeProfileSpec, WImpedanceComposeSpec,
};

const TERRAN_PIRATE_SKELETON_SCENARIO_JSON: &str =
    include_str!("../../../scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json");

const SATURATING_FLUX_HOPS: u32 = 4;
const MIN_PLUS_ITERATIONS: u32 = 16;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn canonical_skeleton_scenario() -> SimThingScenarioSpec {
    let scenario = deserialize_scenario_authority(TERRAN_PIRATE_SKELETON_SCENARIO_JSON)
        .expect("deserialize canonical skeleton");
    validate_stead_mapping_consistency(&scenario).expect("STEAD valid");
    validate_scenario_links(&scenario).expect("links valid");
    scenario
}

fn admit_structural_theater(
    spec: &SimThingScenarioSpec,
) -> simthing_driver::CompiledStructuralN4Theater {
    match compile_structural_n4_theater(spec, MappingExecutionProfile::SparseRegionFieldV1)
        .expect("compile structural theater")
    {
        StructuralTheaterAdmission::Admit(theater) => theater,
        StructuralTheaterAdmission::AtlasDeferred { reason, .. } => {
            panic!("expected admission, got atlas deferral: {reason:?}")
        }
    }
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn terran_pirate_guyang_field_spec(grid_size: u32) -> RegionFieldSpec {
    RegionFieldSpec {
        name: "terran_pirate_guyang_first_slice".into(),
        grid_size,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: 2.0,
            chi: 0.25,
            choke_output_col: Some(1),
        },
        horizon: SATURATING_FLUX_HOPS,
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

fn terran_pirate_w_compose_spec(grid_size: u32) -> WImpedanceComposeSpec {
    WImpedanceComposeSpec {
        width: grid_size,
        height: grid_size,
        n_dims: 5,
        base_w_col: 0,
        choke_a_col: 1,
        choke_b_col: 2,
        profiles: vec![WImpedanceComposeProfileSpec {
            weight_a: 1.0,
            weight_b: 0.25,
            output_w_col: 3,
        }],
    }
}

fn terran_pirate_mapping_plan_compile_spec(
    theater: &simthing_driver::CompiledStructuralN4Theater,
) -> MappingPlanCompileSpec {
    let grid = theater.frame_width;
    let hub = theater.coord_for_system(1).expect("hub");
    MappingPlanCompileSpec {
        structured_field: compile_region_field_preview(&terran_pirate_guyang_field_spec(grid))
            .expect("guyang admission"),
        structured_hops: SATURATING_FLUX_HOPS,
        structured_to_interleaved_writes: vec![(1, 1)],
        w_compose: compile_w_impedance_compose_preview(&terran_pirate_w_compose_spec(grid))
            .expect("w compose admission"),
        min_plus_profile_index: 0,
        min_plus_dest: StructuralGridCoordinate {
            col: hub.col,
            row: hub.row,
        },
        min_plus_d_col: 4,
        min_plus_iterations: MIN_PLUS_ITERATIONS,
        min_plus_inf: MIN_PLUS_INF,
    }
}

fn seed_guyang_values(
    theater: &simthing_driver::CompiledStructuralN4Theater,
    n_dims: u32,
) -> Vec<f32> {
    let cells = theater.frame_width * theater.frame_height;
    let mut values = vec![0.0f32; (cells * n_dims) as usize];
    let hub = theater.coord_for_system(1).expect("hub placement");
    let corridor = theater.coord_for_system(2).expect("corridor placement");
    let branch = theater.coord_for_system(3).expect("branch placement");
    values[idx(theater.cell_slot(hub), 0, n_dims)] = 80.0;
    values[idx(theater.cell_slot(corridor), 0, n_dims)] = 20.0;
    values[idx(theater.cell_slot(branch), 0, n_dims)] = 10.0;
    values
}

fn seed_interleaved_base(
    theater: &simthing_driver::CompiledStructuralN4Theater,
    n_dims: u32,
) -> Vec<f32> {
    let cells = theater.frame_width * theater.frame_height;
    let mut values = vec![0.0f32; (cells * n_dims) as usize];
    for slot in 0..cells {
        values[idx(slot, 0, n_dims)] = 1.0;
        values[idx(slot, 2, n_dims)] = 0.0;
    }
    values
}

/// Second theater: small generic structured-field-only plan (no scenario authority).
fn synthetic_structured_field_plan() -> (
    CompiledMappingPlan,
    simthing_gpu::StructuredFieldStencilConfig,
    Vec<f32>,
) {
    use simthing_gpu::{
        StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
        StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
        StructuredFieldStencilSourcePolicy,
    };
    let config = StructuredFieldStencilConfig {
        width: 3,
        height: 3,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon: 1,
        alpha_self: 1.0,
        gamma_neighbor: 0.0,
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.0,
        weight_west: 0.0,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 2.0,
            chi: 0.25,
            choke_output_col: Some(1),
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    };
    let plan = CompiledMappingPlan {
        steps: vec![CompiledMappingStep::StructuredFieldStencil {
            config: config.clone(),
            hops: 1,
            interleaved_column_writes: Vec::new(),
        }],
        interleaved_width: 0,
        interleaved_height: 0,
        interleaved_n_dims: 0,
    };
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(0, 0, config.n_dims)] = 50.0;
    values[idx(1, 0, config.n_dims)] = 15.0;
    (plan, config, values)
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for atlas scheduler proof");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

#[test]
fn terran_pirate_mapping_atlas_scheduler_two_theater_driver_compile_to_sim() {
    let spec = canonical_skeleton_scenario();
    let original_id = spec.scenario_id.clone();
    let theater = admit_structural_theater(&spec);
    let compile_spec = terran_pirate_mapping_plan_compile_spec(&theater);
    let terran_plan =
        compile_mapping_plan_from_admitted_theater(&theater, compile_spec).expect("mapping plan");

    let guyang_config = match &terran_plan.steps[0] {
        CompiledMappingStep::StructuredFieldStencil { config, .. } => config.clone(),
        _ => panic!("expected structured field"),
    };
    let guyang_values = seed_guyang_values(&theater, guyang_config.n_dims);
    let interleaved = seed_interleaved_base(&theater, terran_plan.interleaved_n_dims);

    let guyang_cpu =
        cpu_structured_field_horizon(&guyang_values, &guyang_config, SATURATING_FLUX_HOPS);
    let mut cpu_interleaved = interleaved.clone();
    for slot in 0..(terran_plan.interleaved_width * terran_plan.interleaved_height) {
        cpu_interleaved[idx(slot, 1, terran_plan.interleaved_n_dims)] =
            guyang_cpu[idx(slot, 1, guyang_config.n_dims)];
    }
    let w_config = match &terran_plan.steps[1] {
        CompiledMappingStep::WImpedanceCompose { config } => config.clone(),
        _ => panic!("expected w compose"),
    };
    cpu_interleaved = simthing_gpu::cpu_w_impedance_compose_oracle(&cpu_interleaved, &w_config);
    let stencil = match &terran_plan.steps[2] {
        CompiledMappingStep::MinPlusStencil { config, .. } => config.clone(),
        _ => panic!("expected min plus"),
    };
    let cpu_d =
        cpu_min_plus_d_from_composed_interleaved(&cpu_interleaved, &stencil, MIN_PLUS_ITERATIONS)
            .expect("cpu d");

    let (synthetic_plan, synthetic_config, synthetic_values) = synthetic_structured_field_plan();
    let synthetic_cpu = cpu_structured_field_horizon(&synthetic_values, &synthetic_config, 1);

    let interleaved_n_dims = terran_plan.interleaved_n_dims;
    let atlas = CompiledMappingAtlas {
        plans: vec![terran_plan, synthetic_plan],
    };

    with_gpu(|ctx| {
        let mut scheduler = SimGpuMappingAtlasScheduler::new(ctx, atlas).expect("scheduler");
        assert_eq!(scheduler.theater_count(), 2);

        let proof = scheduler
            .tick(
                ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        simthing_sim::MappingTickInputs {
                            structured_field_values: &[guyang_values.clone()],
                            interleaved_values: Some(&interleaved),
                        },
                        simthing_sim::MappingTickInputs {
                            structured_field_values: &[synthetic_values.clone()],
                            interleaved_values: None,
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");

        assert_eq!(proof.theater_outputs.len(), 2);
        let gpu_d = proof.theater_outputs[0]
            .proof_values
            .as_ref()
            .expect("terran proof d");
        assert!(
            max_d_field_error(&cpu_d, gpu_d) < 1e-4,
            "Terran Pirate D-field parity through atlas scheduler"
        );
        let gpu_synthetic = proof.theater_outputs[1]
            .proof_values
            .as_ref()
            .expect("synthetic proof");
        assert_eq!(gpu_synthetic.len(), synthetic_cpu.len());
        for (i, (g, c)) in gpu_synthetic.iter().zip(synthetic_cpu.iter()).enumerate() {
            assert!((g - c).abs() < 1e-4, "synthetic theater mismatch at {i}");
        }
        assert!(!debug_readback_allowed());
        assert_eq!(
            scheduler
                .theater_resident_tick_count(MappingTheaterSlot(0))
                .expect("slot 0"),
            1
        );
        assert_eq!(
            scheduler
                .theater_resident_tick_count(MappingTheaterSlot(1))
                .expect("slot 1"),
            1
        );

        let none = scheduler
            .tick(
                ctx,
                MappingAtlasTickInputs {
                    theater_inputs: vec![
                        simthing_sim::MappingTickInputs {
                            structured_field_values: &[seed_guyang_values(
                                &theater,
                                guyang_config.n_dims,
                            )],
                            interleaved_values: Some(&seed_interleaved_base(
                                &theater,
                                interleaved_n_dims,
                            )),
                        },
                        simthing_sim::MappingTickInputs {
                            structured_field_values: &[synthetic_values],
                            interleaved_values: None,
                        },
                    ],
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");

        for theater_out in &none.theater_outputs {
            assert!(theater_out.proof_values.is_none());
        }
        assert!(!debug_readback_allowed());
        assert_eq!(scheduler.resident_tick_count(), 2);
        eprintln!("SIM-MAPPING-ATLAS-SCHEDULER-0: REAL_ADAPTER_OBSERVED (driver_atlas)");
    });

    let reloaded = canonical_skeleton_scenario();
    assert_eq!(reloaded.scenario_id, original_id);
    assert_eq!(reloaded.structural_grid.frame.width, 8);
}
