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
