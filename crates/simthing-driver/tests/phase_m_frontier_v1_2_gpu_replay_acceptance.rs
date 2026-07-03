//! FrontierV1-2 — GPU-resident execution and replay acceptance (Tier-2, test-only).
//!
//! Executes accepted first-slice RegionCell mapping on GPU with CPU-oracle Resource Flow
//! routing and replay fingerprint. Resource Flow GPU integration remains pending (FrontierV1-3).

#[path = "support/frontier_v1.rs"]
mod frontier_v1;

use std::sync::Mutex;

use frontier_v1::*;
use simthing_driver::{
    compiled_stencil_to_gpu_config, FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions,
};
use simthing_gpu::{cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilConfig};
use simthing_spec::{
    compile_region_field_preview, landed_jit_kernel_descriptors, MappingExecutionProfile,
    RegionFieldSpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

pub const FRONTIER_V1_GPU_REPLAY_FINGERPRINT: &str = "42b0455e4d0b59ac";

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

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

struct GpuFixtureRun {
    summary: FrontierV1GpuReplaySummary,
    cpu_output: FrontierV1FixtureOutput,
    field_values: Vec<f32>,
    threat: f32,
    urgency: f32,
}

fn run_gpu_frontier_fixture(
    ctx: &GpuContext,
    skeleton: &FrontierV1ScenarioSkeleton,
    config: &FrontierV1FixtureConfig,
) -> GpuFixtureRun {
    let spec = frontier_v1_mapping_field_spec();
    let admission = validate_frontier_v1_admission(skeleton);
    assert!(admission.accepted, "{:?}", admission.rejected_reasons);

    let seeds = frontier_v1_gpu_seeds(config);
    let mut session =
        FirstSliceMappingSession::open(ctx, MappingExecutionProfile::SparseRegionFieldV1, &spec)
            .expect("mapping session opens");
    session.queue_seeds(&seeds).expect("queue seeds");

    let report = session
        .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
        .expect("mapping tick");
    assert!(report.enabled);
    assert!(report.scheduled);
    assert!(report.reduction_executed);
    assert!(report.eml_executed);

    let field_values = report.field_values.expect("debug readback field values");
    assert_gpu_field_matches_cpu_reference(&spec, &field_values, &seed_tuples(config));

    let (threat, urgency) = session
        .diagnostic_readback_reduction_eml(ctx, (0.2, 0.1))
        .expect("reduction/eml readback");
    assert!(threat.is_finite());
    assert!(urgency.is_finite());

    let mapping_hash = hash_gpu_field_values(&field_values);
    let cpu_output = run_frontier_v1_fixture(skeleton, config);
    let summary = build_gpu_replay_summary(mapping_hash, &cpu_output, true);

    GpuFixtureRun {
        summary,
        cpu_output,
        field_values,
        threat,
        urgency,
    }
}

#[test]
fn frontier_v1_2_happy_path_gpu_fixture_runs() {
    with_gpu(|ctx| {
        let (skeleton, config) = smoke_fixture();
        let run = run_gpu_frontier_fixture(ctx, &skeleton, &config);

        assert!(run.summary.accepted);
        assert_eq!(skeleton.profile_name, FRONTIER_V1_PROFILE_NAME);
        assert!(!skeleton.enabled_by_default);
        assert_eq!(
            skeleton.mapping_execution_profile,
            MappingExecutionProfile::SparseRegionFieldV1
        );
        assert_eq!(
            run.summary.mapping_status,
            FrontierV1FieldStatus::GpuVerified
        );
        assert_eq!(
            run.summary.resource_flow_status,
            FrontierV1FieldStatus::CpuOracleOnly
        );
        assert!(run.summary.gpu_reduction_eml_executed);
        assert!(!run.field_values.is_empty());

        println!(
            "frontier_v1_2_happy: fixture_id={FRONTIER_V1_GPU_FIXTURE_ID} fp={} threat={} urgency={}",
            run.summary.combined_hex(),
            run.threat,
            run.urgency,
        );
    });
}

#[test]
fn frontier_v1_2_defaults_remain_disabled() {
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
        let spec = frontier_v1_mapping_field_spec();
        let mut session =
            FirstSliceMappingSession::open(ctx, MappingExecutionProfile::Disabled, &spec)
                .expect("disabled session opens");
        session
            .queue_seeds(&[FirstSliceSeed {
                row: 0,
                col: 0,
                value: 120.0,
            }])
            .unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
            .unwrap();
        assert!(!report.enabled);
        assert!(!report.scheduled);
    });

    println!("frontier_v1_2_defaults: disabled=true fixture_id={FRONTIER_V1_GPU_FIXTURE_ID}");
}

#[test]
fn frontier_v1_2_gpu_cpu_oracle_parity() {
    with_gpu(|ctx| {
        let (skeleton, config) = smoke_fixture();
        let run = run_gpu_frontier_fixture(ctx, &skeleton, &config);

        assert_eq!(
            run.summary.resource_flow_summary_hash,
            run.cpu_output.fingerprint.resource_flow_summary_hash
        );
        assert_eq!(
            run.summary.proposal_summary_hash,
            run.cpu_output.fingerprint.proposal_summary_hash
        );
        assert_eq!(
            run.summary.route_summary_hash,
            run.cpu_output.fingerprint.route_summary_hash
        );
        assert_eq!(run.summary.overflow_flags, 0);
        assert_eq!(run.cpu_output.routes.resource_route_count, 1);
        assert_eq!(run.cpu_output.routes.structural_route_count, 1);
        assert_eq!(run.cpu_output.routes.movement_route_count, 1);
        assert_eq!(
            run.summary.mapping_status,
            FrontierV1FieldStatus::GpuVerified
        );
        assert_eq!(
            run.summary.resource_flow_status,
            FrontierV1FieldStatus::CpuOracleOnly
        );

        println!(
            "frontier_v1_2_parity: mapping=gpu_verified rf=cpu_oracle_only fp={}",
            run.summary.combined_hex()
        );
    });
}

#[test]
fn frontier_v1_2_gpu_replay_reproducibility() {
    with_gpu(|ctx| {
        let (skeleton, config) = smoke_fixture();
        let run_a = run_gpu_frontier_fixture(ctx, &skeleton, &config);
        let run_b = run_gpu_frontier_fixture(ctx, &skeleton, &config);

        assert_eq!(run_a.summary, run_b.summary);
        assert_eq!(run_a.field_values, run_b.field_values);
        assert_eq!(run_a.threat.to_bits(), run_b.threat.to_bits());
        assert_eq!(run_a.urgency.to_bits(), run_b.urgency.to_bits());
        assert_eq!(run_a.summary.combined_hex(), run_b.summary.combined_hex());
        assert_eq!(
            run_a.summary.combined_hex(),
            FRONTIER_V1_GPU_REPLAY_FINGERPRINT
        );

        println!(
            "frontier_v1_2_replay: fp={} fixture_id={FRONTIER_V1_GPU_FIXTURE_ID}",
            run_a.summary.combined_hex()
        );
    });
}

#[test]
fn frontier_v1_2_resource_dispatch_routes_through_allocator() {
    let (skeleton, config) = smoke_fixture();
    let cpu = run_frontier_v1_fixture(&skeleton, &config);
    assert_eq!(
        classify_proposal_route(ProposalKind::ResourceDispatch, &skeleton),
        ProposalRoute::ResourceFlowAllocator
    );
    assert_eq!(cpu.routes.resource_route_count, 1);
    assert!(!skeleton.resource_flow.parallel_fixture_economy);
    assert!(!skeleton.resource_flow.shared_pool_tick_writes);
    assert!(!skeleton.field_policy.cpu_planner);

    with_gpu(|ctx| {
        let run = run_gpu_frontier_fixture(ctx, &skeleton, &config);
        assert_eq!(run.cpu_output.routes.resource_route_count, 1);
    });

    println!("frontier_v1_2_resource_route: ResourceFlowAllocator fixture_id={FRONTIER_V1_GPU_FIXTURE_ID}");
}
#[test]
fn frontier_v1_2_no_simthing_sim_semantic_awareness() {
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
    println!("frontier_v1_2_sim: semantic_free=true fixture_id={FRONTIER_V1_GPU_FIXTURE_ID}");
}

#[test]
fn frontier_v1_2_no_unauthorized_gpu_primitive() {
    let frontier_descriptor = landed_jit_kernel_descriptors().into_iter().find(|d| {
        d.id.contains("frontier") || d.id.contains("FrontierV1") || d.id.contains("frontier_v1_2")
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
    println!("frontier_v1_2_gpu: no_new_primitive=true fixture_id={FRONTIER_V1_GPU_FIXTURE_ID}");
}
