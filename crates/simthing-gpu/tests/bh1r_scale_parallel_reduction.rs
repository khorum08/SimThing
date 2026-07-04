//! BH-1R-SCALE — parallel GPU choke threshold reduction tests.

use simthing_gpu::{
    cpu_choke_threshold_oracle, cpu_stencil_step, params_from_config, pass1_workgroup_count,
    GpuContext, SaturatingFluxChokeThresholdConfig, SaturatingFluxChokeThresholdOp,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, CHOKE_THRESHOLD_COMPACT_FLOATS,
    CHOKE_THRESHOLD_REDUCE_WORKGROUP_SIZE,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const BH_HOT_PATH_FORBIDDEN: &[&str] = &[
    "sqrt",
    "length(",
    "distance",
    "normalize",
    "hypot",
    "magnitude",
    "norm(",
];

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-1R-SCALE tests");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn choke_stencil_config(w: u32, h: u32, choke_col: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
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
            u_sat: 1.0,
            chi: 0.25,
            choke_output_col: Some(choke_col),
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

fn threshold_config(
    stencil: &StructuredFieldStencilConfig,
    choke_col: u32,
    threshold: f32,
) -> SaturatingFluxChokeThresholdConfig {
    SaturatingFluxChokeThresholdConfig {
        width: stencil.width,
        height: stencil.height,
        n_dims: stencil.n_dims,
        choke_col,
        threshold,
    }
}

fn crowded_values(config: &StructuredFieldStencilConfig) -> Vec<f32> {
    let mut values = vec![0.0f32; config.values_len()];
    let w = config.width;
    let center = (config.height / 2) * w + w / 2;
    values[idx(center, 0, 4)] = 2.0;
    if center >= w {
        values[idx(center - w, 0, 4)] = 2.0;
    }
    if center + w < config.cells() {
        values[idx(center + w, 0, 4)] = 2.0;
    }
    if center % w > 0 {
        values[idx(center - 1, 0, 4)] = 2.0;
    }
    if center % w + 1 < w {
        values[idx(center + 1, 0, 4)] = 2.0;
    }
    values
}

fn run_gpu_pipeline(
    ctx: &GpuContext,
    stencil_config: &StructuredFieldStencilConfig,
    values: &[f32],
    threshold: f32,
) -> (simthing_gpu::SaturatingFluxChokeThresholdResult, usize) {
    let stencil = StructuredFieldStencilOp::new(ctx, stencil_config.clone()).expect("stencil");
    stencil.upload_values(ctx, values).expect("upload");
    stencil.dispatch_ping_pong(ctx, 1).expect("dispatch");
    let resident = &stencil.output_buffer;

    let consumer = SaturatingFluxChokeThresholdOp::new(ctx);
    let cfg = threshold_config(stencil_config, 1, threshold);
    let gpu = consumer
        .reduce_resident_field(ctx, resident, &cfg)
        .expect("reduce");
    (
        gpu,
        CHOKE_THRESHOLD_COMPACT_FLOATS as usize * std::mem::size_of::<f32>(),
    )
}

fn scan_for_forbidden_tokens(source: &str, label: &str) {
    let lower = source.to_ascii_lowercase();
    for token in BH_HOT_PATH_FORBIDDEN {
        assert!(
            !lower.contains(token),
            "{label} contains forbidden BH hot-path token `{token}`"
        );
    }
}

#[test]
fn bh1r_parallel_choke_threshold_gpu_matches_cpu_oracle() {
    with_gpu(|ctx| {
        let stencil_config = choke_stencil_config(32, 32, 1);
        assert!(stencil_config.cells() > CHOKE_THRESHOLD_REDUCE_WORKGROUP_SIZE);
        let values = crowded_values(&stencil_config);
        let params = params_from_config(&stencil_config);
        let cpu_field = cpu_stencil_step(&values, &params);
        let cfg = threshold_config(&stencil_config, 1, 0.5);
        let oracle = cpu_choke_threshold_oracle(&cpu_field, &cfg);

        let (gpu, _) = run_gpu_pipeline(ctx, &stencil_config, &values, 0.5);
        assert!((gpu.sum_choke - oracle.sum_choke).abs() < 1e-3);
        assert!((gpu.max_choke - oracle.max_choke).abs() < 1e-4);
        assert_eq!(gpu.count_above_threshold, oracle.count_above_threshold);
        assert_eq!(gpu.crossed_threshold, oracle.crossed_threshold);
    });
}
