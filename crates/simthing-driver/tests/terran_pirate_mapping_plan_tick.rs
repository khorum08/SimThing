//! DRIVER-MAPPING-PLAN-COMPILE-0 + SIM-MAPPING-PLAN-TICK-SEAM-0 +
//! SIM-MAPPING-READBACK-POLICY-HARDEN-0 — driver→sim integration proof.

use std::sync::Mutex;

use simthing_core::{CombineFn, SourceSpec, StructuralScalarChannel};
use simthing_driver::{
    compile_mapping_plan_from_admitted_theater, compile_structural_link_neighbor_sum_plan,
    compile_structural_n4_theater, compile_structured_field_mapping_plan,
    compiled_stencil_to_gpu_config, MappingPlanCompileSpec, StructuralGridCoordinate,
    StructuralTheaterAdmission,
};
use simthing_gpu::{debug_readback_allowed, max_d_field_error, GpuContext, MIN_PLUS_INF};
use simthing_sim::{
    cpu_min_plus_d_from_composed_interleaved, cpu_structured_field_horizon, CompiledMappingStep,
    MappingTickInputs, SimGpuMappingReadbackPolicy, SimGpuMappingTickState,
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

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for mapping plan tick proof");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

#[test]
fn terran_pirate_mapping_plan_tick_structural_n4_and_link_separation_preserved() {
    let spec = canonical_skeleton_scenario();
    let theater = admit_structural_theater(&spec);
    assert_eq!(theater.frame_width, 8);
    assert_eq!(theater.occupied_cells.len(), 4);
    assert_eq!(theater.n4_edges.len(), 3);

    let hub = theater.coord_for_system(1).expect("hub");
    let branch = theater.coord_for_system(3).expect("branch");
    assert!(!theater.has_n4_edge(hub, branch));

    let plan = compile_structural_link_neighbor_sum_plan(
        &spec,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("link gather compile");
    let corridor = plan
        .ops
        .iter()
        .find(|op| op.targets[0].0 == 1)
        .expect("corridor op");
    let SourceSpec::ConjunctiveCrossing { inputs } = &corridor.source else {
        panic!("expected input list gather");
    };
    assert_eq!(corridor.combine, CombineFn::Sum);
    let mut slots: Vec<_> = inputs.iter().map(|input| input.slot).collect();
    slots.sort_unstable();
    assert_eq!(slots, vec![0, 2, 3]);

    let mut link_only = spec.clone();
    link_only.links.clear();
    let theater_without_links = admit_structural_theater(&link_only);
    assert_eq!(theater.n4_edges, theater_without_links.n4_edges);
}

#[test]
fn terran_pirate_mapping_plan_tick_structured_field_proof_matches_cpu_oracle() {
    let spec = canonical_skeleton_scenario();
    let original_id = spec.scenario_id.clone();
    let theater = admit_structural_theater(&spec);
    let guyang_spec = terran_pirate_guyang_field_spec(theater.frame_width);
    let guyang_preview = compile_region_field_preview(&guyang_spec).expect("guyang admission");
    let guyang_config = compiled_stencil_to_gpu_config(&guyang_preview.stencil);
    let guyang_values = seed_guyang_values(&theater, guyang_config.n_dims);
    let cpu = cpu_structured_field_horizon(&guyang_values, &guyang_config, SATURATING_FLUX_HOPS);

    let plan = compile_structured_field_mapping_plan(
        &theater,
        &guyang_preview,
        SATURATING_FLUX_HOPS,
        Vec::new(),
        5,
    )
    .expect("structured field mapping plan");

    with_gpu(|ctx| {
        let mut state = SimGpuMappingTickState::new(ctx, plan).expect("state");
        let output = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[guyang_values],
                    interleaved_values: Some(&seed_interleaved_base(&theater, 5)),
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("tick");
        let gpu = output.proof_values.expect("proof values");
        assert_eq!(gpu.len(), cpu.len());
        for (i, (g, c)) in gpu.iter().zip(cpu.iter()).enumerate() {
            assert!(
                (g - c).abs() < 1e-4,
                "gpu/cpu mismatch at {i}: gpu={g} cpu={c}"
            );
        }
        eprintln!("DRIVER-MAPPING-PLAN-COMPILE-0: REAL_ADAPTER_OBSERVED (structured_field)");
    });

    let reloaded = canonical_skeleton_scenario();
    assert_eq!(reloaded.scenario_id, original_id);
    assert_eq!(reloaded.structural_grid.frame.width, 8);
}

#[test]
fn terran_pirate_mapping_plan_tick_full_chain_d_field_matches_cpu_oracle() {
    let spec = canonical_skeleton_scenario();
    let theater = admit_structural_theater(&spec);
    let compile_spec = terran_pirate_mapping_plan_compile_spec(&theater);
    let plan =
        compile_mapping_plan_from_admitted_theater(&theater, compile_spec).expect("mapping plan");

    let guyang_values = seed_guyang_values(
        &theater,
        match &plan.steps[0] {
            CompiledMappingStep::StructuredFieldStencil { config, .. } => config.n_dims,
            _ => panic!("expected structured field first step"),
        },
    );
    let interleaved = seed_interleaved_base(&theater, plan.interleaved_n_dims);

    let guyang_config = match &plan.steps[0] {
        CompiledMappingStep::StructuredFieldStencil { config, .. } => config.clone(),
        _ => panic!("expected structured field"),
    };
    let guyang_cpu =
        cpu_structured_field_horizon(&guyang_values, &guyang_config, SATURATING_FLUX_HOPS);
    let mut cpu_interleaved = interleaved.clone();
    for slot in 0..(plan.interleaved_width * plan.interleaved_height) {
        cpu_interleaved[idx(slot, 1, plan.interleaved_n_dims)] =
            guyang_cpu[idx(slot, 1, guyang_config.n_dims)];
    }
    let w_config = match &plan.steps[1] {
        CompiledMappingStep::WImpedanceCompose { config } => config.clone(),
        _ => panic!("expected w compose"),
    };
    cpu_interleaved = simthing_gpu::cpu_w_impedance_compose_oracle(&cpu_interleaved, &w_config);
    let stencil = match &plan.steps[2] {
        CompiledMappingStep::MinPlusStencil { config, .. } => config.clone(),
        _ => panic!("expected min plus"),
    };
    let cpu_d =
        cpu_min_plus_d_from_composed_interleaved(&cpu_interleaved, &stencil, MIN_PLUS_ITERATIONS)
            .expect("cpu d");

    let interleaved_n_dims = plan.interleaved_n_dims;
    with_gpu(|ctx| {
        let mut state = SimGpuMappingTickState::new(ctx, plan).expect("state");
        let output = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[guyang_values],
                    interleaved_values: Some(&interleaved),
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("tick");
        let gpu_d = output.proof_values.expect("proof d");
        assert!(
            max_d_field_error(&cpu_d, &gpu_d) < 1e-4,
            "D field GPU/CPU parity through driver compile -> sim mapping tick seam"
        );
        let none = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[seed_guyang_values(&theater, guyang_config.n_dims)],
                    interleaved_values: Some(&seed_interleaved_base(&theater, interleaved_n_dims)),
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick");
        assert!(none.proof_values.is_none());
        assert!(!debug_readback_allowed());
        assert_eq!(state.resident_tick_count(), 2);
        eprintln!("DRIVER-MAPPING-PLAN-COMPILE-0: REAL_ADAPTER_OBSERVED (full_chain)");
    });
}

#[test]
fn terran_pirate_mapping_plan_tick_readback_policy_sequencing() {
    let spec = canonical_skeleton_scenario();
    let original_id = spec.scenario_id.clone();
    let theater = admit_structural_theater(&spec);
    let compile_spec = terran_pirate_mapping_plan_compile_spec(&theater);
    let plan =
        compile_mapping_plan_from_admitted_theater(&theater, compile_spec).expect("mapping plan");

    let guyang_config = match &plan.steps[0] {
        CompiledMappingStep::StructuredFieldStencil { config, .. } => config.clone(),
        _ => panic!("expected structured field"),
    };
    let interleaved_n_dims = plan.interleaved_n_dims;

    with_gpu(|ctx| {
        let mut state = SimGpuMappingTickState::new(ctx, plan).expect("state");
        let guyang_values = seed_guyang_values(&theater, guyang_config.n_dims);
        let interleaved = seed_interleaved_base(&theater, interleaved_n_dims);

        let none1 = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[guyang_values.clone()],
                    interleaved_values: Some(&interleaved),
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick 1");
        assert!(none1.proof_values.is_none());
        assert!(!debug_readback_allowed());
        assert_eq!(state.resident_tick_count(), 1);

        let proof = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[guyang_values.clone()],
                    interleaved_values: Some(&interleaved),
                },
                SimGpuMappingReadbackPolicy::ProofReadback,
            )
            .expect("proof tick");
        assert!(proof.proof_values.is_some());
        assert!(!debug_readback_allowed());
        assert_eq!(state.resident_tick_count(), 2);

        let none2 = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[guyang_values.clone()],
                    interleaved_values: Some(&interleaved),
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick 2");
        assert!(none2.proof_values.is_none());
        assert!(!debug_readback_allowed());
        assert_eq!(state.resident_tick_count(), 3);

        let none3 = state
            .tick(
                ctx,
                MappingTickInputs {
                    structured_field_values: &[guyang_values],
                    interleaved_values: Some(&seed_interleaved_base(&theater, interleaved_n_dims)),
                },
                SimGpuMappingReadbackPolicy::None,
            )
            .expect("none tick 3");
        assert!(none3.proof_values.is_none());
        assert!(!debug_readback_allowed());
        assert_eq!(state.resident_tick_count(), 4);
        eprintln!("SIM-MAPPING-READBACK-POLICY-HARDEN-0: REAL_ADAPTER_OBSERVED (driver_readback_sequencing)");
    });

    let reloaded = canonical_skeleton_scenario();
    assert_eq!(reloaded.scenario_id, original_id);
    assert_eq!(reloaded.structural_grid.frame.width, 8);
}
