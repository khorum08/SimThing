//! StructuredFieldStencilOp GPU parity and stability tests (V7.6 promotion + hardening).

use simthing_gpu::{
    cpu_horizon, params_from_config, GpuContext, StructuredFieldExecutionOptions,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig, StructuredFieldStencilError,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, DEFAULT_HORIZON_CAP, EXTENDED_HORIZON_CAP,
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
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.0,
        weight_west: 0.0,
        source_cap: None,
        operator: StructuredFieldStencilOperator::Normalized,
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
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
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.0,
        weight_west: 0.0,
        source_cap: Some(cap),
        operator: StructuredFieldStencilOperator::SourceCappedNormalized,
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
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
        let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
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
                let (gpu, _) = op.run_ping_pong(ctx, steps).unwrap();
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
        let cluster = [
            (0u32, 0u32, 80.0f32),
            (0, 1, 60.0),
            (1, 0, 60.0),
            (1, 1, 40.0),
        ];
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
        let (out, _) = op.run_configured_horizon(ctx).unwrap();
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
    let source_col = capped.source_col;
    let mut values = vec![0.0f32; capped.values_len()];
    values[idx(0, source_col, 4)] = 80.0;
    values[idx(1, source_col, 4)] = 60.0;
    values[idx(10, source_col, 4)] = 60.0;
    values[idx(11, source_col, 4)] = 40.0;
    for slot in [0u32, 1, 10, 11] {
        assert!(get(&values, slot, source_col, 4) > 0.0);
        for col in 0..4 {
            if col != source_col {
                assert_eq!(get(&values, slot, col, 4), 0.0);
            }
        }
    }
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
fn structured_field_stencil_horizon_execution_rejects_steps_above_config() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 4);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        assert_eq!(
            op.run_ping_pong(ctx, 8).unwrap_err(),
            StructuredFieldStencilError::ExecutionHorizonExceedsConfig {
                steps: 8,
                horizon: 4
            }
        );
        assert_eq!(
            op.dispatch_ping_pong(ctx, 5).unwrap_err(),
            StructuredFieldStencilError::ExecutionHorizonExceedsConfig {
                steps: 5,
                horizon: 4
            }
        );
    });
}

#[test]
fn structured_field_stencil_source_policy_documented_or_enforced() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 2);
        assert_eq!(
            config.source_policy,
            StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero
        );
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(4, 0, 4)] = 100.0;
        op.upload_values(ctx, &values).unwrap();
        let (after_one, _) = op.run_ping_pong(ctx, 1).unwrap();
        assert!(
            get(&after_one, 4, 0, 4) > 0.0,
            "center retains propagated value"
        );
        op.upload_values(ctx, &after_one).unwrap();
        let (after_two, _) = op.run_ping_pong(ctx, 1).unwrap();
        assert!(
            get(&after_two, 4, 0, 4) > get(&after_one, 4, 0, 4),
            "primitive does not auto-zero source; value grows without caller clearing"
        );
    });
}

#[test]
fn structured_field_stencil_source_cap_cluster_indices_correct() {
    let capped = source_capped_config(10, 10, 8, 100.0);
    let source_col = capped.source_col;
    let mut values = vec![0.0f32; capped.values_len()];
    values[idx(0, source_col, 4)] = 80.0;
    values[idx(1, source_col, 4)] = 60.0;
    values[idx(10, source_col, 4)] = 60.0;
    values[idx(11, source_col, 4)] = 40.0;
    assert_eq!(get(&values, 0, source_col, 4), 80.0);
    assert_eq!(get(&values, 1, source_col, 4), 60.0);
    assert_eq!(get(&values, 10, source_col, 4), 60.0);
    assert_eq!(get(&values, 11, source_col, 4), 40.0);
}

#[test]
fn structured_field_stencil_clamp_boundary_gpu_cpu_parity() {
    with_gpu(|ctx| {
        let mut config = normalized_config(3, 3, 1);
        config.boundary_mode = StructuredFieldStencilBoundaryMode::Clamp;
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(0, 0, 4)] = 50.0;
        op.upload_values(ctx, &values).unwrap();
        let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
        let params = params_from_config(op.config());
        let cpu = cpu_horizon(&values, &params, 1);
        let mut max_err = 0.0f32;
        for i in 0..values.len() {
            max_err = max_err.max((cpu[i] - gpu[i]).abs());
        }
        assert!(max_err < 1e-4, "clamp boundary max_err={max_err}");
        assert!(get(&gpu, 1, 0, 4) > 0.0, "corner clamp feeds neighbor");
    });
}

#[test]
fn structured_field_stencil_active_mask_provisional() {
    let mode = StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo;
    let config = StructuredFieldStencilConfig {
        mask_mode: mode,
        ..normalized_config(3, 3, 1)
    };
    assert!(matches!(
        config.mask_mode,
        StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo
    ));
    let mode_name = format!("{mode:?}");
    assert!(
        mode_name.contains("Experimental") && mode_name.contains("NoHalo"),
        "active mask must be explicitly provisional: {mode_name}"
    );
}

#[test]
fn structured_field_stencil_inert_by_default() {
    let passes = include_str!("../src/passes.rs");
    assert!(!passes.contains("StructuredFieldStencilOp"));
    assert!(!passes.contains("structured_field_stencil"));

    let gpu_lib = include_str!("../src/lib.rs");
    assert!(!gpu_lib.contains("StructuredFieldStencilOp::new(&ctx"));

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("StructuredFieldStencilOp"));
    assert!(!sim_lib.contains("structured_field_stencil"));

    let driver_session = include_str!("../../simthing-driver/src/session.rs");
    assert!(!driver_session.contains("StructuredFieldStencilOp"));
    assert!(!driver_session.contains("structured_field_stencil"));
}

#[test]
fn guard_no_production_pipeline_integration() {
    structured_field_stencil_inert_by_default();
}

#[test]
fn test_m1_execute_configured_uses_horizon() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 4);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(4, 0, 4)] = 100.0;
        op.upload_values(ctx, &values).unwrap();
        let report = op
            .execute_configured(
                ctx,
                StructuredFieldExecutionOptions {
                    collect_field_stats: false,
                    readback_values: true,
                    steps: None,
                },
            )
            .unwrap();
        assert_eq!(report.debug.dispatch_count, 4);
        assert_eq!(report.debug.configured_horizon, 4);
        assert_eq!(report.debug.executed_horizon, 4);
        assert!(report.debug.field_max.is_none());
        let gpu = report.values.expect("readback_values requested");
        let params = params_from_config(op.config());
        let cpu = cpu_horizon(&values, &params, 4);
        let mut max_err = 0.0f32;
        for i in 0..values.len() {
            max_err = max_err.max((cpu[i] - gpu[i]).abs());
        }
        assert!(
            max_err < 1e-3,
            "execute_configured parity max_err={max_err}"
        );
    });
}

#[test]
fn test_m1_execute_configured_rejects_steps_above_horizon() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 4);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let err = op
            .execute_configured(
                ctx,
                StructuredFieldExecutionOptions {
                    collect_field_stats: false,
                    readback_values: false,
                    steps: Some(8),
                },
            )
            .unwrap_err();
        assert_eq!(
            err,
            StructuredFieldStencilError::ExecutionHorizonExceedsConfig {
                steps: 8,
                horizon: 4
            }
        );
    });
}

#[test]
fn test_m1_debug_report_with_stats_requires_readback() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 2);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(4, 0, 4)] = 100.0;
        op.upload_values(ctx, &values).unwrap();

        let no_stats = op
            .execute_configured(
                ctx,
                StructuredFieldExecutionOptions {
                    collect_field_stats: false,
                    readback_values: false,
                    steps: None,
                },
            )
            .unwrap();
        assert!(no_stats.values.is_none());
        assert!(no_stats.debug.field_max.is_none());
        assert!(no_stats.debug.field_l1_norm.is_none());
        assert!(no_stats.debug.active_mask_ratio.is_none());

        op.upload_values(ctx, &values).unwrap();
        let readback_only = op
            .execute_configured(
                ctx,
                StructuredFieldExecutionOptions {
                    collect_field_stats: false,
                    readback_values: true,
                    steps: None,
                },
            )
            .unwrap();
        assert!(readback_only.values.is_some());
        assert!(readback_only.debug.field_max.is_none());
        assert!(readback_only.debug.field_l1_norm.is_none());
        assert!(readback_only.debug.active_mask_ratio.is_none());

        op.upload_values(ctx, &values).unwrap();
        let with_stats = op
            .execute_configured(
                ctx,
                StructuredFieldExecutionOptions {
                    collect_field_stats: true,
                    readback_values: false,
                    steps: None,
                },
            )
            .unwrap();
        let d = &with_stats.debug;
        assert!(with_stats.values.is_some(), "stats path readback-derived");
        assert_eq!(d.dispatch_count, 2);
        assert_eq!(d.configured_horizon, 2);
        assert_eq!(d.executed_horizon, 2);
        assert_eq!(d.operator, StructuredFieldStencilOperator::Normalized);
        assert_eq!(
            d.source_policy,
            StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero
        );
        assert_eq!(d.mask_mode, StructuredFieldStencilMaskMode::All);
        assert_eq!(d.cell_count, 9);
        assert_eq!(d.values_len, 36);
        assert!(d.field_max.unwrap().is_finite());
        assert!(d.field_l1_norm.unwrap().is_finite());
        assert_eq!(d.active_mask_ratio, Some(1.0));
    });
}

#[test]
fn test_m1_1_execute_configured_no_readback_default() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 4);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let values = vec![0.0f32; op.config().values_len()];
        op.upload_values(ctx, &values).unwrap();

        let report = op
            .execute_configured(ctx, StructuredFieldExecutionOptions::default())
            .unwrap();
        assert_eq!(report.debug.dispatch_count, 4);
        assert_eq!(report.debug.executed_horizon, 4);
        assert!(report.values.is_none());
        assert!(report.debug.field_max.is_none());
        assert!(report.debug.field_l1_norm.is_none());
        assert!(report.debug.active_mask_ratio.is_none());
    });
}

#[test]
fn test_m1_1_horizon_guard_on_no_readback_and_readback_paths() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 4);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let values = vec![0.0f32; op.config().values_len()];
        op.upload_values(ctx, &values).unwrap();

        for readback_values in [false, true] {
            op.upload_values(ctx, &values).unwrap();
            let err = op
                .execute_configured(
                    ctx,
                    StructuredFieldExecutionOptions {
                        collect_field_stats: false,
                        readback_values,
                        steps: Some(8),
                    },
                )
                .unwrap_err();
            assert_eq!(
                err,
                StructuredFieldStencilError::ExecutionHorizonExceedsConfig {
                    steps: 8,
                    horizon: 4
                }
            );
        }
    });
}

#[test]
fn test_r1_gpu_buffer_copy_and_cell_write_helpers() {
    with_gpu(|ctx| {
        let config = normalized_config(4, 4, 1);
        let op = StructuredFieldStencilOp::new(ctx, config.clone()).unwrap();
        let n_dims = config.n_dims;
        let zeros = vec![0.0f32; config.values_len()];
        op.upload_values(ctx, &zeros).unwrap();

        let writes = [(5u32, 0u32, 42.0f32), (10u32, 0u32, 17.0f32)];
        op.write_cell_values(ctx, &op.input_buffer, &writes)
            .unwrap();
        let seeded = op.readback_input_buffer(ctx);
        assert!((seeded[idx(5, 0, n_dims)] - 42.0).abs() < 1e-6);
        assert!((seeded[idx(10, 0, n_dims)] - 17.0).abs() < 1e-6);

        op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
        op.zero_cell_values(ctx, &op.output_buffer, &[(5, 0), (10, 0)])
            .unwrap();
        op.copy_output_to_input(ctx);
        let after = op.readback_input_buffer(ctx);
        assert!(after[idx(5, 0, n_dims)].abs() < 1e-6);
        assert!(after[idx(10, 0, n_dims)].abs() < 1e-6);
        assert!(
            after[idx(6, 0, n_dims)] > 0.0,
            "neighbor propagation preserved"
        );

        op.upload_values(ctx, &zeros).unwrap();
        op.write_cell_values(ctx, &op.input_buffer, &[(0, 0, 100.0)])
            .unwrap();
        for _ in 0..3 {
            op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
            op.copy_output_to_input(ctx);
        }
        let canonical = op.readback_input_buffer(ctx);
        let params = params_from_config(&config);
        let mut cpu = zeros;
        cpu[idx(0, 0, n_dims)] = 100.0;
        cpu = cpu_horizon(&cpu, &params, 3);
        assert_fields_near_helper(&cpu, &canonical, 1e-4);
    });
}

fn assert_fields_near_helper(a: &[f32], b: &[f32], tol: f32) {
    assert_eq!(a.len(), b.len());
    for (i, (&x, &y)) in a.iter().zip(b.iter()).enumerate() {
        assert!((x - y).abs() <= tol, "mismatch at {i}: {x} vs {y}");
    }
}

fn gradient_x_config(w: u32, h: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
        n_dims: 4,
        source_col: 0,
        target_col: 1,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.5,
        weight_west: -0.5,
        source_cap: None,
        operator: StructuredFieldStencilOperator::GradientX,
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

fn gradient_y_config(w: u32, h: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
        n_dims: 4,
        source_col: 0,
        target_col: 1,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: -0.5,
        weight_south: 0.5,
        weight_east: 0.0,
        weight_west: 0.0,
        source_cap: None,
        operator: StructuredFieldStencilOperator::GradientY,
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

#[test]
fn m5a_cpu_oracle_isotropic_weights_match_legacy_gamma() {
    let config = normalized_config(3, 3, 1);
    let (wn, _, we, _) = config.resolved_directional_weights();
    assert!((wn - 0.2).abs() < 1e-6);
    assert!((we - 0.2).abs() < 1e-6);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(4, 0, 4)] = 100.0;
    let cpu = cpu_horizon(&values, &params, 1);
    assert!((cpu[idx(1, 0, 4)] - 20.0).abs() < 1e-5);
}

#[test]
fn m5a_cpu_oracle_gradient_x_on_small_grid() {
    let config = gradient_x_config(3, 3);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(5, 0, 4)] = 10.0;
    values[idx(3, 0, 4)] = 0.0;
    let cpu = cpu_horizon(&values, &params, 1);
    let gx = cpu[idx(4, 1, 4)];
    assert!((gx - 5.0).abs() < 1e-5, "GradientX center got {gx}");
}

#[test]
fn m5a_cpu_oracle_gradient_y_on_small_grid() {
    let config = gradient_y_config(3, 3);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    values[idx(7, 0, 4)] = 8.0;
    values[idx(1, 0, 4)] = 0.0;
    let cpu = cpu_horizon(&values, &params, 1);
    let gy = cpu[idx(4, 1, 4)];
    assert!((gy - 4.0).abs() < 1e-5, "GradientY center got {gy}");
}

#[test]
fn m5a_gpu_parity_gradient_x() {
    with_gpu(|ctx| {
        let config = gradient_x_config(3, 3);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(5, 0, 4)] = 10.0;
        values[idx(3, 0, 4)] = 0.0;
        op.upload_values(ctx, &values).unwrap();
        let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
        let params = params_from_config(op.config());
        let cpu = cpu_horizon(&values, &params, 1);
        assert_fields_near_helper(&cpu, &gpu, 1e-4);
        assert!((get(&gpu, 4, 1, 4) - 5.0).abs() < 1e-3);
    });
}

#[test]
fn m5a_gpu_parity_gradient_y() {
    with_gpu(|ctx| {
        let config = gradient_y_config(3, 3);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(7, 0, 4)] = 8.0;
        values[idx(1, 0, 4)] = 0.0;
        op.upload_values(ctx, &values).unwrap();
        let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
        let params = params_from_config(op.config());
        let cpu = cpu_horizon(&values, &params, 1);
        assert_fields_near_helper(&cpu, &gpu, 1e-4);
        assert!((get(&gpu, 4, 1, 4) - 4.0).abs() < 1e-3);
    });
}

fn gradient_xy_config(w: u32, h: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: w,
        height: h,
        n_dims: 4,
        source_col: 0,
        target_col: 1, // axis-X output
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: -0.5,
        weight_south: 0.5,
        weight_east: 0.5,
        weight_west: -0.5,
        source_cap: None,
        operator: StructuredFieldStencilOperator::GradientXY { target_col_y: 2 }, // axis-Y output
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

#[test]
fn gradient_xy_cpu_oracle_writes_both_axes_one_pass() {
    let config = gradient_xy_config(3, 3);
    let params = params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    // east of center (slot 5) = 10, west (slot 3) = 0 -> gx = 5
    values[idx(5, 0, 4)] = 10.0;
    values[idx(3, 0, 4)] = 0.0;
    // south of center (slot 7) = 8, north (slot 1) = 0 -> gy = 4
    values[idx(7, 0, 4)] = 8.0;
    values[idx(1, 0, 4)] = 0.0;
    let cpu = cpu_horizon(&values, &params, 1);
    let gx = cpu[idx(4, 1, 4)];
    let gy = cpu[idx(4, 2, 4)];
    assert!((gx - 5.0).abs() < 1e-5, "GradientXY axis-X center got {gx}");
    assert!((gy - 4.0).abs() < 1e-5, "GradientXY axis-Y center got {gy}");
    // source column is untouched.
    assert_eq!(cpu[idx(4, 0, 4)], 0.0);
}

#[test]
fn gradient_xy_cpu_oracle_matches_two_single_axis_passes() {
    // Dual-output GradientXY must equal running GradientX then GradientY into separate columns.
    let mut values = vec![0.0f32; 3 * 3 * 4];
    values[idx(5, 0, 4)] = 10.0;
    values[idx(3, 0, 4)] = 2.0;
    values[idx(7, 0, 4)] = 8.0;
    values[idx(1, 0, 4)] = 1.0;

    let xy = cpu_horizon(&values, &params_from_config(&gradient_xy_config(3, 3)), 1);
    let gx_only = cpu_horizon(&values, &params_from_config(&gradient_x_config(3, 3)), 1);
    let gy_only = cpu_horizon(&values, &params_from_config(&gradient_y_config(3, 3)), 1);

    for slot in 0..9u32 {
        // GradientX writes col 1; GradientXY writes axis-X to col 1.
        assert!((xy[idx(slot, 1, 4)] - gx_only[idx(slot, 1, 4)]).abs() < 1e-6);
        // GradientY writes col 1; GradientXY writes axis-Y to col 2.
        assert!((xy[idx(slot, 2, 4)] - gy_only[idx(slot, 1, 4)]).abs() < 1e-6);
    }
}

#[test]
fn gradient_xy_aliased_output_columns_rejected() {
    let mut config = gradient_xy_config(3, 3);
    config.operator = StructuredFieldStencilOperator::GradientXY { target_col_y: 1 }; // == target_col
    assert_eq!(
        config.validate(),
        Err(StructuredFieldStencilError::GradientXyAliasedOutputs {
            target_col: 1,
            target_col_y: 1,
        })
    );
}

#[test]
fn gradient_xy_target_y_out_of_range_rejected() {
    let mut config = gradient_xy_config(3, 3);
    config.operator = StructuredFieldStencilOperator::GradientXY { target_col_y: 4 }; // n_dims = 4
    assert_eq!(
        config.validate(),
        Err(StructuredFieldStencilError::GradientXyTargetYOutOfRange {
            target_col_y: 4,
            n_dims: 4,
        })
    );
}

#[test]
fn gradient_xy_gpu_parity_both_axes() {
    with_gpu(|ctx| {
        let config = gradient_xy_config(3, 3);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(5, 0, 4)] = 10.0;
        values[idx(3, 0, 4)] = 0.0;
        values[idx(7, 0, 4)] = 8.0;
        values[idx(1, 0, 4)] = 0.0;
        op.upload_values(ctx, &values).unwrap();
        let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
        let params = params_from_config(op.config());
        let cpu = cpu_horizon(&values, &params, 1);
        assert_fields_near_helper(&cpu, &gpu, 1e-4);
        assert!((get(&gpu, 4, 1, 4) - 5.0).abs() < 1e-3, "axis-X");
        assert!((get(&gpu, 4, 2, 4) - 4.0).abs() < 1e-3, "axis-Y");
    });
}

#[test]
fn m5a_gpu_parity_normalized_after_directional_weight_refactor() {
    with_gpu(|ctx| {
        let config = normalized_config(3, 3, 1);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(4, 0, 4)] = 100.0;
        op.upload_values(ctx, &values).unwrap();
        let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
        let params = params_from_config(op.config());
        let cpu = cpu_horizon(&values, &params, 1);
        assert_fields_near_helper(&cpu, &gpu, 1e-4);
    });
}

#[test]
fn m5a_gpu_parity_source_capped_after_directional_weight_refactor() {
    with_gpu(|ctx| {
        let config = source_capped_config(3, 3, 1, 50.0);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(4, 0, 4)] = 100.0;
        op.upload_values(ctx, &values).unwrap();
        let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
        let params = params_from_config(op.config());
        let cpu = cpu_horizon(&values, &params, 1);
        assert_fields_near_helper(&cpu, &gpu, 1e-4);
    });
}

#[test]
fn m5a_single_target_output_contract_preserved() {
    with_gpu(|ctx| {
        let config = gradient_x_config(3, 3);
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![1.0f32; op.config().values_len()];
        values[idx(5, 0, 4)] = 10.0;
        op.upload_values(ctx, &values).unwrap();
        let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
        assert!(
            (get(&gpu, 4, 0, 4) - 1.0).abs() < 1e-4,
            "source col unchanged"
        );
        assert!((get(&gpu, 4, 1, 4)).abs() > 0.0, "target col written");
        assert!(
            (get(&gpu, 4, 2, 4) - 1.0).abs() < 1e-4,
            "other cols passthrough"
        );
    });
}
