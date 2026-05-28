//! StructuredFieldStencilOp GPU parity and stability tests (V7.6 promotion).

use simthing_gpu::{
    cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilError, StructuredFieldStencilMaskMode,
    StructuredFieldStencilOp, StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
    DEFAULT_HORIZON_CAP, EXTENDED_HORIZON_CAP,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for structured field stencil tests");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn get(v: &[f32], slot: u32, col: u32, n_dims: u32) -> f32 {
    v[idx(slot, col, n_dims)]
}

fn normalized_config(w: u32, h: u32, horizon: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon,
        alpha_self: 1.0,
        gamma_neighbor: 0.8,
        source_cap: None,
        operator: StructuredFieldStencilOperator::Normalized,
        source_policy: StructuredFieldStencilSourcePolicy::OneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: horizon > DEFAULT_HORIZON_CAP,
    }
}

fn source_capped_config(w: u32, h: u32, horizon: u32, cap: f32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon,
        alpha_self: 1.0,
        gamma_neighbor: 0.8,
        source_cap: Some(cap),
        operator: StructuredFieldStencilOperator::SourceCappedNormalized,
        source_policy: StructuredFieldStencilSourcePolicy::OneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: horizon > DEFAULT_HORIZON_CAP,
    }
}
#[test]
fn test_a_wgsl_compile_and_3x3_correctness() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 1);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(4, 0, 4)] = 100.0;
        op.upload_values(ctx, &values).unwrap();
        let (gpu, _) = op.run_ping_pong(ctx, 1);
        let params = params_from_config(op.config());
        let cpu = cpu_horizon(&values, &params, 1);
        let mut max_err = 0.0f32;
        for i in 0..values.len() {
            max_err = max_err.max((cpu[i] - gpu[i]).abs());
        }
        assert!(max_err < 1e-4, "3x3 center source max_err={max_err}");
        let n = get(&gpu, 1, 0, 4);
        let s = get(&gpu, 7, 0, 4);
        let e = get(&gpu, 5, 0, 4);
        let w = get(&gpu, 3, 0, 4);
        assert!(n > 0.0 && s > 0.0 && e > 0.0 && w > 0.0);
        assert!((n - s).abs() < 1e-3 && (n - e).abs() < 1e-3 && (n - w).abs() < 1e-3);
    });
}

#[test]
fn test_b_pingpong_correctness() {
    with_gpu(|ctx| {
        for &(w, h) in &[(3u32, 3u32), (10u32, 10u32)] {
            let config = normalized_config(w, h, 8);
            let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
            let mut values = vec![0.0f32; op.config().values_len()];
            if w == 3 {
                values[idx(4, 0, 4)] = 100.0;
            } else {
                values[idx(0, 0, 4)] = 80.0;
                values[idx(1, 0, 4)] = 60.0;
                values[idx(10, 0, 4)] = 60.0;
                values[idx(11, 0, 4)] = 40.0;
            }
            let params = params_from_config(op.config());
            for &steps in &[1u32, 2, 4, 8] {
                op.upload_values(ctx, &values).unwrap();
                let (gpu, _) = op.run_ping_pong(ctx, steps);
                let cpu = cpu_horizon(&values, &params, steps);
                let mut max_err = 0.0f32;
                for i in 0..values.len() {
                    max_err = max_err.max((cpu[i] - gpu[i]).abs());
                }
                assert!(max_err < 1e-3, "grid {w}x{h} H={steps} max_err={max_err}");
            }
        }
    });
}

#[test]
fn test_c_10x10_h8_tactical_horizon() {
    with_gpu(|ctx| {
        let config = normalized_config(10, 10, 8);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let cluster = [(0u32, 0u32, 80.0f32), (0, 1, 60.0), (1, 0, 60.0), (1, 1, 40.0)];
        let mut values = vec![0.0f32; op.config().values_len()];
        for &(r, c, v) in &cluster {
            values[idx(r * 10 + c, 0, 4)] = v;
        }
        op.upload_values(ctx, &values).unwrap();
        op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
        let mut v = op.readback_after_ping_pong(ctx, 1);
        for &(r, c, _) in &cluster {
            v[idx(r * 10 + c, 0, 4)] = 0.0;
        }
        op.upload_values(ctx, &v).unwrap();
        let (out, _) = op.run_ping_pong(ctx, 8);
        let t44 = get(&out, 44, 0, 4);
        let gx = (get(&out, 45, 0, 4) - get(&out, 43, 0, 4)) / 2.0;
        let gy = (get(&out, 54, 0, 4) - get(&out, 34, 0, 4)) / 2.0;
        let mut max_v = 0.0f32;
        for s in 0..100 {
            max_v = max_v.max(get(&out, s, 0, 4).abs());
        }
        assert!(t44 > 0.01, "t44={t44}");
        assert!(gx < 0.0 && gy < 0.0, "grad=({gx},{gy})");
        assert!(max_v < 1_000_000.0, "max={max_v}");
    });
}

#[test]
fn test_d_source_cap_and_horizon_cap() {
    let mut config = normalized_config(10, 10, 16);
    config.allow_extended_horizon = true;
    assert!(config.validate().is_ok());

    let capped = source_capped_config(10, 10, 16, 500.0);
    capped.validate().unwrap();
    let params = params_from_config(&capped);
    let mut values = vec![0.0f32; capped.values_len()];
    values[0] = 80.0;
    values[1] = 60.0;
    values[10] = 60.0;
    values[11] = 40.0;
    let out = cpu_horizon(&values, &params, 16);
    let mut max_v = 0.0f32;
    for s in 0..100 {
        max_v = max_v.max(get(&out, s, 0, 4));
    }
    assert!(max_v <= 500.0 + 1e-3, "capped max={max_v}");

    let uncapped_params = params_from_config(&config);
    let uncapped = cpu_horizon(&values, &uncapped_params, 16);
    let mut uncapped_max = 0.0f32;
    for s in 0..100 {
        uncapped_max = uncapped_max.max(get(&uncapped, s, 0, 4));
    }
    assert!(
        uncapped_max > 500.0,
        "normalized without cap should amplify: max={uncapped_max}"
    );

    let mut h16 = normalized_config(10, 10, 16);
    h16.allow_extended_horizon = false;
    assert_eq!(
        h16.validate().unwrap_err(),
        StructuredFieldStencilError::HorizonCapExceeded {
            horizon: 16,
            cap: DEFAULT_HORIZON_CAP
        }
    );
    h16.allow_extended_horizon = true;
    assert!(h16.validate().is_ok());
    assert!(EXTENDED_HORIZON_CAP >= 16);
}

#[test]
fn guard_no_production_pipeline_integration() {
    let lib = include_str!("../src/lib.rs");
    assert!(!lib.contains("StructuredFieldStencilOp::new(&ctx"));
    let passes = include_str!("../src/passes.rs");
    assert!(!passes.contains("structured_field_stencil"));
}
