//! FrontierV1-3 — GPU flat-star Resource Flow integration (Tier-2, test-only).
//!
//! Closes the Resource Flow GPU gap from FrontierV1-2 by verifying flat-star allocation
//! through the accepted Resource Flow allocator path inside the default-off FrontierV1 fixture.

#[path = "support/e11_flat_star.rs"]
mod e11_flat_star;
#[path = "support/frontier_v1.rs"]
mod frontier_v1;

use std::sync::Mutex;

use e11_flat_star::{
    fill_explicit_participants, flat_star_cell_inputs, flat_star_game_mode, flat_star_scenario,
    leaf_slots, root_slot, try_gpu, FlatStarSession,
};
use frontier_v1::*;
use simthing_driver::{
    build_execution_plan, compiled_stencil_to_gpu_config, install_atomic, resolve_node_columns,
    run_arena_allocation_oracle, FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions,
    SimSession,
};
use simthing_gpu::{cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilConfig};
use simthing_spec::{
    compile_region_field_preview, landed_jit_kernel_descriptors, MappingExecutionProfile,
    RegionFieldSpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

pub const FRONTIER_V1_GPU_RF_REPLAY_FINGERPRINT: &str = "7bacf7921b807bee";

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping GPU assertions: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn smoke_fixture() -> (FrontierV1ScenarioSkeleton, FrontierV1FixtureConfig) {
    (
        frontier_v1_1_smoke_skeleton(),
        frontier_v1_1_fixture_config(),
    )
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn cpu_caller_managed_field(
    config: &StructuredFieldStencilConfig,
    seeds: &[(u32, u32, f32)],
) -> Vec<f32> {
    let params = params_from_config(config);
    let mut values = vec![0.0f32; config.values_len()];
    for &(row, col, value) in seeds {
        let slot = row * config.width + col;
        values[idx(slot, config.source_col, config.n_dims)] = value;
    }
    values = cpu_horizon(&values, &params, 1);
    for &(row, col, _) in seeds {
        let slot = row * config.width + col;
        values[idx(slot, config.source_col, config.n_dims)] = 0.0;
    }
    cpu_horizon(&values, &params, config.horizon)
}

fn assert_gpu_field_matches_cpu_reference(
    spec: &RegionFieldSpec,
    field_values: &[f32],
    seeds: &[(u32, u32, f32)],
) {
    let preview = compile_region_field_preview(spec).expect("mapping spec admits");
    let config = compiled_stencil_to_gpu_config(&preview.stencil);
    let expected = cpu_caller_managed_field(&config, seeds);
    assert_eq!(field_values.len(), expected.len());
    for (i, (&gpu, &cpu)) in field_values.iter().zip(expected.iter()).enumerate() {
        assert!(
            (gpu - cpu).abs() <= 0.0001,
            "GPU/CPU mapping parity mismatch at {i}: gpu={gpu} cpu={cpu}"
        );
    }
}

fn frontier_v1_gpu_seeds(config: &FrontierV1FixtureConfig) -> Vec<FirstSliceSeed> {
    vec![
        FirstSliceSeed {
            row: 0,
            col: 0,
            value: config.district_output_a as f32,
        },
        FirstSliceSeed {
            row: config.grid_size - 1,
            col: config.grid_size - 1,
            value: config.district_output_b as f32,
        },
    ]
}

fn seed_tuples(config: &FrontierV1FixtureConfig) -> Vec<(u32, u32, f32)> {
    vec![
        (0, 0, config.district_output_a as f32),
        (
            config.grid_size - 1,
            config.grid_size - 1,
            config.district_output_b as f32,
        ),
    ]
}

fn open_frontier_v1_flat_star_gpu() -> FlatStarSession {
    // Flat-star D=2: first explicit participant is the allocator root; remaining are faction leaves.
    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = flat_star_game_mode(16);
    game_mode.resource_flow.as_mut().unwrap().opt_in_mode = ResourceFlowOptInMode::FlatStarOptIn;
    game_mode.resource_flow_execution_profile = ResourceFlowExecutionProfile::FlatStarResourceFlow;
    fill_explicit_participants(&mut game_mode, &scenario);

    let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    assert!(session.proto.flags.use_accumulator_resource_flow);
    assert!(session.state.accumulator_resource_flow_active);

    let flow_id = session
        .proto
        .registry
        .id_of("core", "food_flow")
        .expect("food_flow registered");
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "food")
        .expect("column refs");
    let layout = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("execution plan")
    .arenas
    .into_iter()
    .next()
    .expect("one arena");

    FlatStarSession {
        session,
        layout,
        cols,
    }
}

fn f32_alloc_to_u32(v: f32) -> u32 {
    assert!(v.is_finite() && v >= 0.0);
    v.round() as u32
}

fn run_gpu_flat_star_allocation(
    fx: &mut FlatStarSession,
    allocator_total: u32,
) -> GpuResourceFlowAllocationSummary {
    fx.session
        .sync_resource_flow_if_enabled()
        .expect("resource flow sync");
    assert!(fx.session.state.accumulator_resource_flow_active);

    let root = root_slot(&fx.layout);
    let leaves = leaf_slots(&fx.layout);
    assert_eq!(leaves.len(), 2, "FrontierV1 smoke uses two faction leaves");

    let (weight_a, weight_b) = frontier_v1_flat_star_weights();
    let inputs = flat_star_cell_inputs(
        root,
        &leaves,
        fx.cols,
        allocator_total as f32,
        &[weight_a, weight_b],
    );

    let n_dims = fx.session.proto.registry.total_columns as u32;
    let mut flat = fx.session.state.read_values();
    for (&(slot, col), &v) in &inputs {
        flat[idx(slot, col, n_dims)] = v;
    }
    fx.session.state.write_values(&flat);

    let mut oracle = inputs.clone();
    run_arena_allocation_oracle(&fx.layout, &mut oracle, 1.0);

    fx.session
        .state
        .run_resource_flow_bands(fx.session.state.accumulator_resource_flow_bands, 1.0);

    let gpu_out = fx.session.state.read_values();
    let mut gpu_a = 0.0f32;
    let mut gpu_b = 0.0f32;
    for (i, &leaf) in leaves.iter().enumerate() {
        let cpu = oracle
            .get(&(leaf, fx.cols.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu = gpu_out[idx(leaf, fx.cols.allocated_flow_col, n_dims)];
        assert_eq!(
            cpu.to_bits(),
            gpu.to_bits(),
            "leaf {leaf} E-11 oracle/GPU bit parity"
        );
        if i == 0 {
            gpu_a = gpu;
        } else {
            gpu_b = gpu;
        }
    }

    GpuResourceFlowAllocationSummary {
        faction_a_allocation: f32_alloc_to_u32(gpu_a),
        faction_b_allocation: f32_alloc_to_u32(gpu_b),
        allocator_total,
        resource_overflow_flags: 0,
        allocator_route_code: FRONTIER_V1_ALLOCATOR_ROUTE_CODE,
    }
}

struct FrontierV1GpuFullRun {
    summary: FrontierV1GpuReplaySummary,
    cpu_output: FrontierV1FixtureOutput,
    gpu_rf: GpuResourceFlowAllocationSummary,
}

fn run_frontier_v1_gpu_full(
    ctx: &GpuContext,
    skeleton: &FrontierV1ScenarioSkeleton,
    config: &FrontierV1FixtureConfig,
) -> FrontierV1GpuFullRun {
    let admission = validate_frontier_v1_admission(skeleton);
    assert!(admission.accepted, "{:?}", admission.rejected_reasons);

    let spec = frontier_v1_mapping_field_spec();
    let mut mapping_session =
        FirstSliceMappingSession::open(ctx, MappingExecutionProfile::SparseRegionFieldV1, &spec)
            .expect("mapping session opens");
    mapping_session
        .queue_seeds(&frontier_v1_gpu_seeds(config))
        .expect("queue seeds");
    let mapping_report = mapping_session
        .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
        .expect("mapping tick");
    assert!(mapping_report.enabled);
    let field_values = mapping_report
        .field_values
        .expect("debug readback field values");
    assert_gpu_field_matches_cpu_reference(&spec, &field_values, &seed_tuples(config));
    let _ = mapping_session
        .diagnostic_readback_reduction_eml(ctx, (0.2, 0.1))
        .expect("reduction/eml readback");

    let mapping_hash = hash_gpu_field_values(&field_values);
    let cpu_output = run_frontier_v1_fixture(skeleton, config);
    let allocator_total = cpu_output
        .resource_flow
        .allocated_a
        .saturating_add(cpu_output.resource_flow.allocated_b);

    let mut fx = open_frontier_v1_flat_star_gpu();
    let gpu_rf = run_gpu_flat_star_allocation(&mut fx, allocator_total);

    assert_eq!(
        gpu_rf.faction_a_allocation, cpu_output.resource_flow.allocated_a,
        "faction_a GPU/CPU oracle parity"
    );
    assert_eq!(
        gpu_rf.faction_b_allocation, cpu_output.resource_flow.allocated_b,
        "faction_b GPU/CPU oracle parity"
    );
    assert_eq!(gpu_rf.allocator_total, allocator_total);
    assert_eq!(
        gpu_rf.allocator_route_code,
        FRONTIER_V1_ALLOCATOR_ROUTE_CODE
    );

    let rf_hash = hash_gpu_resource_flow(gpu_rf);
    let summary = build_gpu_replay_summary_with_rf(
        mapping_hash,
        rf_hash,
        &cpu_output,
        FrontierV1FieldStatus::GpuVerified,
        true,
    );

    FrontierV1GpuFullRun {
        summary,
        cpu_output,
        gpu_rf,
    }
}

#[test]
fn frontier_v1_3_happy_path_gpu_resource_flow_runs() {
    with_gpu(|ctx| {
        let (skeleton, config) = smoke_fixture();
        let run = run_frontier_v1_gpu_full(ctx, &skeleton, &config);

        assert!(run.summary.accepted);
        assert_eq!(
            run.summary.mapping_status,
            FrontierV1FieldStatus::GpuVerified
        );
        assert_eq!(
            run.summary.resource_flow_status,
            FrontierV1FieldStatus::GpuVerified
        );
        assert!(!skeleton.enabled_by_default);
        assert!(run.gpu_rf.faction_a_allocation > 0);
        assert!(run.gpu_rf.faction_b_allocation > 0);

        println!(
            "frontier_v1_3_happy: fixture_id={FRONTIER_V1_GPU_RF_FIXTURE_ID} fp={} alloc_a={} alloc_b={}",
            run.summary.combined_hex(),
            run.gpu_rf.faction_a_allocation,
            run.gpu_rf.faction_b_allocation,
        );
    });
}

#[test]
fn frontier_v1_3_gpu_resource_flow_cpu_oracle_parity() {
    with_gpu(|ctx| {
        let (skeleton, config) = smoke_fixture();
        let run = run_frontier_v1_gpu_full(ctx, &skeleton, &config);

        assert_eq!(
            run.gpu_rf.faction_a_allocation,
            run.cpu_output.resource_flow.allocated_a
        );
        assert_eq!(
            run.gpu_rf.faction_b_allocation,
            run.cpu_output.resource_flow.allocated_b
        );
        assert_eq!(
            run.gpu_rf.allocator_total,
            run.cpu_output.resource_flow.allocated_a + run.cpu_output.resource_flow.allocated_b
        );
        assert_eq!(run.gpu_rf.resource_overflow_flags, 0);
        assert_eq!(
            classify_proposal_route(ProposalKind::ResourceDispatch, &skeleton),
            ProposalRoute::ResourceFlowAllocator
        );
        assert_eq!(
            run.gpu_rf.allocator_route_code,
            FRONTIER_V1_ALLOCATOR_ROUTE_CODE
        );

        println!(
            "frontier_v1_3_parity: rf=gpu_verified route=cpu_oracle_only fp={}",
            run.summary.combined_hex()
        );
    });
}

#[test]
fn frontier_v1_3_gpu_resource_flow_replay_reproducibility() {
    with_gpu(|ctx| {
        let (skeleton, config) = smoke_fixture();
        let run_a = run_frontier_v1_gpu_full(ctx, &skeleton, &config);
        let run_b = run_frontier_v1_gpu_full(ctx, &skeleton, &config);

        assert_eq!(run_a.summary, run_b.summary);
        assert_eq!(run_a.gpu_rf, run_b.gpu_rf);
        assert_eq!(run_a.summary.combined_hex(), run_b.summary.combined_hex());
        assert_eq!(
            run_a.summary.combined_hex(),
            FRONTIER_V1_GPU_RF_REPLAY_FINGERPRINT
        );

        println!(
            "frontier_v1_3_replay: fp={} fixture_id={FRONTIER_V1_GPU_RF_FIXTURE_ID}",
            run_a.summary.combined_hex()
        );
    });
}

#[test]
fn frontier_v1_3_defaults_remain_disabled() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_eq!(
        ResourceFlowOptInMode::default(),
        ResourceFlowOptInMode::Disabled
    );
    assert_eq!(
        ResourceFlowExecutionProfile::default(),
        ResourceFlowExecutionProfile::DefaultDisabled
    );

    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    assert!(!sim_lib.contains("FrontierV1"));

    with_gpu(|ctx| {
        let mut disabled = open_frontier_v1_flat_star_gpu();
        disabled.session.proto.flags.use_accumulator_resource_flow = false;
        disabled.session.state.accumulator_resource_flow_active = false;
        let err = disabled
            .session
            .sync_resource_flow_if_enabled()
            .expect("disabled sync ok");
        let _ = err;
    });

    println!("frontier_v1_3_defaults: disabled=true fixture_id={FRONTIER_V1_GPU_RF_FIXTURE_ID}");
}

#[test]
fn frontier_v1_3_rejects_resource_flow_bypass() {
    let (skeleton, config) = smoke_fixture();
    assert_eq!(
        classify_proposal_route(ProposalKind::ResourceDispatch, &skeleton),
        ProposalRoute::ResourceFlowAllocator
    );

    let mut bypass = skeleton;
    bypass.resource_flow.parallel_fixture_economy = true;
    assert!(!validate_frontier_v1_admission(&bypass).accepted);

    let mut shared_pool = skeleton;
    shared_pool.resource_flow.shared_pool_tick_writes = true;
    assert!(!validate_frontier_v1_admission(&shared_pool).accepted);

    let mut planner = skeleton;
    planner.field_policy.cpu_planner = true;
    assert!(!validate_frontier_v1_admission(&planner).accepted);

    bypass.field_policy.resource_dispatch_via_allocator = false;
    assert_eq!(
        classify_proposal_route(ProposalKind::ResourceDispatch, &bypass),
        ProposalRoute::Rejected
    );

    let _ = config;
    println!("frontier_v1_3_bypass: rejects=true fixture_id={FRONTIER_V1_GPU_RF_FIXTURE_ID}");
}

#[test]
fn frontier_v1_3_coupling_rejects_non_frontier_profile() {
    let (mut skeleton, config) = smoke_fixture();
    skeleton.coupling.coupling_requested = true;
    assert!(validate_frontier_v1_admission(&skeleton).accepted);

    skeleton.profile_name = "OtherProfile";
    let other = validate_frontier_v1_admission(&skeleton);
    assert!(!other.coupling_ok);
    assert!(!other.accepted);

    println!(
        "frontier_v1_3_coupling: frontier_only=true fixture_id={FRONTIER_V1_GPU_RF_FIXTURE_ID}"
    );
    let _ = config;
}

#[test]
fn frontier_v1_3_deferred_features_reject() {
    let deferred: [(&str, Box<dyn Fn(&mut FrontierV1ScenarioSkeleton)>); 10] = [
        ("atlas", Box::new(|s| s.theater.request_atlas = true)),
        (
            "active_mask",
            Box::new(|s| s.theater.request_active_mask = true),
        ),
        (
            "perception",
            Box::new(|s| s.theater.request_perception = true),
        ),
        (
            "source_identity",
            Box::new(|s| s.theater.request_source_identity = true),
        ),
        (
            "nested_e11b",
            Box::new(|s| s.resource_flow.nested_e11b = true),
        ),
        (
            "e11b_5",
            Box::new(|s| s.resource_flow.e11b_5_dynamic_enrollment = true),
        ),
        (
            "d2a",
            Box::new(|s| s.resource_flow.d2a_hard_currency_ordering = true),
        ),
        (
            "act5_ladder",
            Box::new(|s| s.field_policy.pipeline_version = FieldPolicyPipelineVersion::Other),
        ),
        (
            "parallel_fixture",
            Box::new(|s| s.resource_flow.parallel_fixture_economy = true),
        ),
        (
            "cpu_planner",
            Box::new(|s| s.field_policy.cpu_planner = true),
        ),
    ];
    for (label, mutate) in deferred {
        let mut skeleton = frontier_v1_1_smoke_skeleton();
        mutate(&mut skeleton);
        assert!(
            !validate_frontier_v1_admission(&skeleton).accepted,
            "{label} should reject"
        );
    }
    println!("frontier_v1_3_deferred: rejects=true fixture_id={FRONTIER_V1_GPU_RF_FIXTURE_ID}");
}

#[test]
fn frontier_v1_3_no_simthing_sim_semantic_awareness() {
    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    for needle in [
        "FrontierV1",
        "FIELD_POLICY",
        "RegionCell",
        "ArenaRegistry",
        "proposal",
        "ResourceFlow",
    ] {
        assert!(
            !sim_lib.contains(needle),
            "simthing-sim must not contain `{needle}`"
        );
    }
    println!("frontier_v1_3_sim: semantic_free=true fixture_id={FRONTIER_V1_GPU_RF_FIXTURE_ID}");
}

#[test]
fn frontier_v1_3_no_unauthorized_gpu_primitive() {
    let frontier_descriptor = landed_jit_kernel_descriptors().into_iter().find(|d| {
        d.id.contains("frontier") || d.id.contains("FrontierV1") || d.id.contains("frontier_v1_3")
    });
    assert!(frontier_descriptor.is_none());

    let wgsl_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/wgsl");
    if wgsl_dir.is_dir() {
        for entry in std::fs::read_dir(&wgsl_dir).expect("read wgsl dir") {
            let path = entry.expect("entry").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            assert!(
                !name.contains("frontier"),
                "no Frontier WGSL: {}",
                path.display()
            );
        }
    }
    println!("frontier_v1_3_gpu: no_new_primitive=true fixture_id={FRONTIER_V1_GPU_RF_FIXTURE_ID}");
}
