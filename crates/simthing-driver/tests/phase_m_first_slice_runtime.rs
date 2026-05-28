//! Phase M-first-slice — opt-in mapping runtime integration tests.

use simthing_core::{manual_slot_range_sum_op, WHITELISTED_FORMULA_CLASSES};
use simthing_driver::{
    compiled_stencil_to_gpu_config, estimate_first_slice_budget, FirstSliceMappingError,
    FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions, FieldDispatchSchedule,
};
use simthing_gpu::{
    cpu_horizon, params_from_config, GpuContext, StructuredFieldExecutionOptions,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig, StructuredFieldStencilMaskMode,
    StructuredFieldStencilOp, StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_region_field_preview, deserialize_region_field_ron, estimate_region_field_budget,
    region_field_isolation_multiplier, MappingExecutionProfile, RegionFieldBudgetSpec,
    RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec, RegionFieldGridProfile,
    RegionFieldIsolationPolicyEstimate, RegionFieldOperatorSpec, RegionFieldReductionSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn first_slice_spec() -> RegionFieldSpec {
    RegionFieldSpec {
        name: "tactical_suppression".into(),
        grid_size: 10,
        n_dims: 8,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SourceCappedNormalized,
        horizon: 8,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.8,
        source_cap: Some(500.0),
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: Some(RegionFieldReductionSpec {
            child_slot_start: 0,
            child_slot_count: 100,
            child_col: 0,
            parent_slot: 100,
            parent_col: 0,
            order_band: 0,
        }),
        parent_formula: Some(RegionFieldFormulaBindingSpec {
            formula_class: "field_urgency".into(),
            tree_id: Some(1),
        }),
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
    }
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn run_caller_managed_gpu(
    ctx: &GpuContext,
    config: &StructuredFieldStencilConfig,
    seeds: &[(u32, u32, f32)],
) -> Vec<f32> {
    let op = StructuredFieldStencilOp::new(ctx, config.clone()).unwrap();
    let n_dims = config.n_dims;
    let w = config.width;
    let mut values = vec![0.0f32; op.config().values_len()];
    for &(r, c, v) in seeds {
        values[idx(r * w + c, config.source_col, n_dims)] = v;
    }
    op.upload_values(ctx, &values).unwrap();
    op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
    values = op.readback_after_ping_pong(ctx, 1);
    for &(r, c, _) in seeds {
        values[idx(r * w + c, config.source_col, n_dims)] = 0.0;
    }
    op.upload_values(ctx, &values).unwrap();
    op.execute_configured(
        ctx,
        StructuredFieldExecutionOptions {
            readback_values: true,
            collect_field_stats: false,
            steps: None,
        },
    )
    .unwrap()
    .values
    .unwrap()
}

fn run_caller_managed_cpu(
    config: &StructuredFieldStencilConfig,
    seeds: &[(u32, u32, f32)],
    hops: u32,
) -> Vec<f32> {
    run_caller_managed_cpu_ticks(config, &[seeds], hops)
}

fn run_caller_managed_cpu_ticks(
    config: &StructuredFieldStencilConfig,
    tick_seeds: &[&[(u32, u32, f32)]],
    hops: u32,
) -> Vec<f32> {
    let params = params_from_config(config);
    let n_dims = config.n_dims;
    let w = config.width;
    let mut values = vec![0.0f32; config.values_len()];
    for seeds in tick_seeds {
        for &(r, c, v) in *seeds {
            values[idx(r * w + c, config.source_col, n_dims)] = v;
        }
        values = cpu_horizon(&values, &params, 1);
        for &(r, c, _) in *seeds {
            values[idx(r * w + c, config.source_col, n_dims)] = 0.0;
        }
        values = cpu_horizon(&values, &params, hops);
    }
    values
}

fn assert_fields_near(a: &[f32], b: &[f32], tol: f32, label: &str) {
    assert_eq!(a.len(), b.len(), "{label} len");
    for (i, (&x, &y)) in a.iter().zip(b.iter()).enumerate() {
        assert!(
            (x - y).abs() <= tol,
            "{label} mismatch at {i}: {x} vs {y}"
        );
    }
}

#[test]
fn test_0_guardrail_sanity() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let spec = first_slice_spec();
    assert!(compile_region_field_preview(&spec).unwrap().reduction.is_some());

    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    assert!(!runtime_src.contains("ActiveOnlyExperimentalNoHalo"));
    assert!(!runtime_src.contains("source_mask"));

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));

    for class in ["field_pressure", "field_urgency", "field_decay", "bounded_field_update"] {
        assert!(WHITELISTED_FORMULA_CLASSES.contains(&class));
    }
}

#[test]
fn test_1_ron_roundtrip_runtime_registration() {
    let spec = first_slice_spec();
    let preview = compile_region_field_preview(&spec).unwrap();
    assert_eq!(preview.parent_formula_class.as_deref(), Some("field_urgency"));
    compiled_stencil_to_gpu_config(&preview.stencil).validate().unwrap();

    let mut bad = spec.clone();
    bad.request_atlas_batching = true;
    assert!(compile_region_field_preview(&bad).is_err());
}

#[test]
fn test_2_single_tick_gpu_execution() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session.queue_seeds(&[
            FirstSliceSeed { row: 0, col: 0, value: 80.0 },
            FirstSliceSeed { row: 0, col: 1, value: 60.0 },
        ]).unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
            .unwrap();
        assert!(report.scheduled);
        let vals = report.field_values.as_ref().unwrap();
        let interior = vals[idx(55, 0, spec.n_dims)];
        assert!(interior.is_finite() && interior > 0.0);
        assert_eq!(
            report.stencil_execution.as_ref().unwrap().report.debug.executed_horizon,
            8
        );
    });
}

#[test]
fn test_3_edge_corner_algebraic_boundary_parity() {
    with_gpu(|ctx| {
        let seed_cases: &[(&str, &[(u32, u32, f32)])] = &[
            ("corner", &[(0, 0, 100.0)]),
            ("edge", &[(0, 5, 100.0)]),
            ("center", &[(5, 5, 100.0)]),
        ];
        for &h in &[1u32, 4, 8] {
            for &(label, seeds) in seed_cases {
                let config = StructuredFieldStencilConfig {
                    width: 10,
                    height: 10,
                    n_dims: 4,
                    source_col: 0,
                    target_col: 0,
                    horizon: h,
                    alpha_self: 1.0,
                    gamma_neighbor: 0.8,
                    source_cap: Some(500.0),
                    operator: StructuredFieldStencilOperator::SourceCappedNormalized,
                    source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
                    boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
                    mask_mode: StructuredFieldStencilMaskMode::All,
                    allow_extended_horizon: false,
                };
                let cpu = run_caller_managed_cpu(&config, seeds, h);
                let gpu = run_caller_managed_gpu(ctx, &config, seeds);
                let col = 0;
                let nd = config.n_dims;
                for slot in 0..100 {
                    assert!(
                        (cpu[idx(slot, col, nd)] - gpu[idx(slot, col, nd)]).abs() <= 0.0001,
                        "{label} H={h} slot={slot}"
                    );
                }
                if label == "corner" {
                    assert!(gpu[idx(0, col, nd)] >= gpu[idx(99, col, nd)]);
                }
            }
        }
    });
}

#[test]
fn test_4_field_scheduler_integration() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session.queue_seeds(&[FirstSliceSeed { row: 5, col: 5, value: 50.0 }]).unwrap();
        let r1 = session.tick(ctx, FirstSliceTickOptions::hot_path(), (0.2, 0.1)).unwrap();
        assert!(r1.scheduled);
        assert_eq!(r1.scheduler_report.as_ref().unwrap().false_skip_count, 0);

        if let Some(region) = session.scheduler().regions().first().cloned() {
            let mut clean = region;
            clean.dirty = simthing_driver::DirtyRegionState::default();
            session.scheduler_mut().register_region(clean);
        }
        let (_, report) = session.scheduler().decide_tick(session.tick_index()).unwrap();
        assert_eq!(report.false_skip_count, 0);
    });
}

#[test]
fn test_5_layer2_sum_reduction() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session.queue_seeds(&[FirstSliceSeed { row: 4, col: 4, value: 80.0 }]).unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.0, 0.0))
            .unwrap();
        let vals = report.field_values.as_ref().unwrap();
        let reduction = spec.reduction.as_ref().unwrap();
        let expected: f32 = (0..reduction.child_slot_count)
            .map(|s| vals[idx(s, reduction.child_col, spec.n_dims)])
            .sum();
        assert!((report.reduction_parent_value.unwrap() - expected).abs() < 0.01);
        let _ = manual_slot_range_sum_op(
            reduction.child_slot_start,
            reduction.child_slot_count,
            reduction.child_col,
            reduction.parent_slot,
            reduction.parent_col,
            reduction.order_band,
        );
    });
}

#[test]
fn test_6_layer3_eval_eml() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session.queue_seeds(&[FirstSliceSeed { row: 2, col: 2, value: 90.0 }]).unwrap();
        let low = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
            .unwrap();
        session.queue_seeds(&[FirstSliceSeed { row: 2, col: 2, value: 90.0 }]).unwrap();
        let high = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.9, 0.1))
            .unwrap();
        assert!(high.eml_output.unwrap() > low.eml_output.unwrap());
    });
}

#[test]
fn test_7_no_readback_hot_path() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session.queue_seeds(&[FirstSliceSeed { row: 3, col: 3, value: 70.0 }]).unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.3, 0.2))
            .unwrap();
        assert!(report.field_values.is_none());
        assert!(report.reduction_parent_value.is_none());
        assert!(report.eml_output.is_none());
        assert!(report.reduction_executed);
        assert!(report.eml_executed);
        assert_eq!(report.source_setup_dispatches, 1);
        assert_eq!(report.propagation_dispatches, spec.horizon);
        assert_eq!(report.total_dispatches, spec.horizon + 1);
        assert!(report.stencil_execution.as_ref().unwrap().report.values.is_none());
    });
}

#[test]
fn test_8_default_off_enforcement() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut disabled = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::Disabled,
            &spec,
        )
        .unwrap();
        disabled.queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 100.0 }]).unwrap();
        assert!(!disabled.tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1)).unwrap().scheduled);

        let mut enabled = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        enabled.queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 100.0 }]).unwrap();
        assert!(enabled.tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1)).unwrap().scheduled);
    });
}

#[test]
fn test_9_deterministic_replay() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let seeds = [
            FirstSliceSeed { row: 1, col: 1, value: 55.0 },
            FirstSliceSeed { row: 8, col: 8, value: 40.0 },
        ];
        let run = || {
            let mut s = FirstSliceMappingSession::open(
                ctx,
                MappingExecutionProfile::SparseRegionFieldV1,
                &spec,
            )
            .unwrap();
            s.queue_seeds(&seeds).unwrap();
            s.tick(ctx, FirstSliceTickOptions::debug_readback(), (0.5, 0.2))
                .unwrap()
        };
        let a = run();
        let b = run();
        assert!((a.reduction_parent_value.unwrap() - b.reduction_parent_value.unwrap()).abs() < 1e-4);
        assert!((a.eml_output.unwrap() - b.eml_output.unwrap()).abs() < 1e-4);
    });
}

#[test]
fn test_10_region_field_budget_estimator() {
    let spec = first_slice_spec();
    let single = estimate_first_slice_budget(
        &spec,
        RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
    )
    .unwrap();
    assert!((single.isolation_multiplier - 1.0).abs() < 1e-9);

    let gutter_mult = region_field_isolation_multiplier(
        RegionFieldIsolationPolicyEstimate::PhysicalGutter { gutter: 8, horizon: 8 },
        10,
    );
    assert!((gutter_mult - 6.76).abs() < 0.01);

    let err = estimate_region_field_budget(&RegionFieldBudgetSpec {
        grid_size: 32,
        column_count: 16,
        buffer_multiplier: 2.0,
        copy_multiplier: 1.0,
        tile_count: 1,
        isolation_policy: RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
        max_region_field_vram_bytes: Some(4096),
    })
    .unwrap_err();
    assert!(err.requested_bytes > err.allowed_bytes);
}

// --- M-first-slice-R1: no-readback correctness hardening ---

#[test]
fn test_r1_a_hot_path_matches_debug_with_diagnostic_readback() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let seeds = [
            FirstSliceSeed { row: 0, col: 0, value: 80.0 },
            FirstSliceSeed { row: 0, col: 1, value: 60.0 },
        ];
        let weights = (0.2, 0.1);

        let mut debug = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        debug.queue_seeds(&seeds).unwrap();
        let debug_report = debug
            .tick(ctx, FirstSliceTickOptions::debug_readback(), weights)
            .unwrap();

        let mut hot = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        hot.queue_seeds(&seeds).unwrap();
        let hot_report = hot
            .tick(ctx, FirstSliceTickOptions::hot_path(), weights)
            .unwrap();
        let hot_field = hot.readback_canonical_field(ctx);
        let (hot_threat, hot_eml) = hot.diagnostic_readback_reduction_eml(ctx, weights).unwrap();

        assert_fields_near(
            debug_report.field_values.as_ref().unwrap(),
            &hot_field,
            1e-4,
            "hot vs debug field",
        );
        assert!(
            (debug_report.reduction_parent_value.unwrap() - hot_threat).abs() < 0.01,
            "reduction mismatch"
        );
        assert!(
            (debug_report.eml_output.unwrap() - hot_eml).abs() < 1e-4,
            "eml mismatch"
        );
        assert!(hot_report.field_values.is_none());
    });
}

#[test]
fn test_r1_b_no_readback_preserves_first_hop_propagation() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let seeds = [(0u32, 0u32, 100.0f32)];
        let config = compiled_stencil_to_gpu_config(
            &compile_region_field_preview(&spec).unwrap().stencil,
        );
        let cpu = run_caller_managed_cpu(&config, &seeds, spec.horizon);

        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 100.0 }])
            .unwrap();
        session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.0, 0.0))
            .unwrap();
        let gpu = session.readback_canonical_field(ctx);
        let nd = spec.n_dims;
        let col = spec.source_col;

        assert!(gpu[idx(1, col, nd)] > 0.0, "neighbor must receive first-hop propagation");
        for slot in 0..100 {
            assert!(
                (cpu[idx(slot, col, nd)] - gpu[idx(slot, col, nd)]).abs() <= 0.0001,
                "slot={slot}"
            );
        }
    });
}

#[test]
fn test_r1_c_no_readback_two_tick_persistence() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let config = compiled_stencil_to_gpu_config(
            &compile_region_field_preview(&spec).unwrap().stencil,
        );
        let seeds_a = [(1u32, 1u32, 55.0f32)];
        let seeds_b = [(8u32, 8u32, 40.0f32)];
        let cpu = run_caller_managed_cpu_ticks(&config, &[&seeds_a, &seeds_b], spec.horizon);

        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 1, col: 1, value: 55.0 }])
            .unwrap();
        session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.0, 0.0))
            .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 8, col: 8, value: 40.0 }])
            .unwrap();
        session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.0, 0.0))
            .unwrap();
        let gpu = session.readback_canonical_field(ctx);
        let col = spec.source_col;
        let nd = spec.n_dims;
        for slot in 0..100 {
            assert!(
                (cpu[idx(slot, col, nd)] - gpu[idx(slot, col, nd)]).abs() <= 0.0001,
                "two-tick slot={slot}"
            );
        }
    });
}

#[test]
fn test_r1_d_seed_only_clear_gpu_resident() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let seeds = [(0u32, 0u32, 100.0f32)];
        let config = compiled_stencil_to_gpu_config(
            &compile_region_field_preview(&spec).unwrap().stencil,
        );
        let cpu = run_caller_managed_cpu(&config, &seeds, spec.horizon);

        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 100.0 }])
            .unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.0, 0.0))
            .unwrap();
        let vals = report.field_values.as_ref().unwrap();
        let col = spec.source_col;
        let nd = spec.n_dims;
        assert!(vals[idx(1, col, nd)] > 0.0, "propagated neighbor preserved");
        assert!(vals[idx(10, col, nd)] > 0.0, "non-seed propagated cell preserved");
        for slot in 0..100 {
            assert!(
                (cpu[idx(slot, col, nd)] - vals[idx(slot, col, nd)]).abs() <= 0.0001,
                "seed-only clear protocol slot={slot}"
            );
        }
    });
}

#[test]
fn test_r1_e_hot_path_report_no_fake_values() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 3, col: 3, value: 70.0 }])
            .unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.3, 0.2))
            .unwrap();
        assert!(report.field_values.is_none());
        assert!(report.reduction_parent_value.is_none());
        assert!(report.eml_output.is_none());
        assert!(report.reduction_executed);
        assert!(report.eml_executed);
    });
}

#[test]
fn test_r1_f_debug_readback_still_returns_values() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 2, col: 2, value: 90.0 }])
            .unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
            .unwrap();
        assert!(report.field_values.is_some());
        assert!(report.reduction_parent_value.is_some());
        assert!(report.eml_output.is_some());
    });
}

#[test]
fn test_r1_g_invalid_seed_rejected() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();

        let oob_row = session.queue_seeds(&[FirstSliceSeed {
            row: 10,
            col: 0,
            value: 1.0,
        }]);
        assert!(matches!(oob_row, Err(FirstSliceMappingError::InvalidSeed { .. })));

        let oob_col = session.queue_seeds(&[FirstSliceSeed {
            row: 0,
            col: 10,
            value: 1.0,
        }]);
        assert!(matches!(oob_col, Err(FirstSliceMappingError::InvalidSeed { .. })));

        let nan = session.queue_seeds(&[FirstSliceSeed {
            row: 0,
            col: 0,
            value: f32::NAN,
        }]);
        assert!(matches!(
            nan,
            Err(FirstSliceMappingError::NonFiniteSeedValue { .. })
        ));

        let inf = session.queue_seeds(&[FirstSliceSeed {
            row: 0,
            col: 0,
            value: f32::INFINITY,
        }]);
        assert!(matches!(
            inf,
            Err(FirstSliceMappingError::NonFiniteSeedValue { .. })
        ));
    });
}

#[test]
fn test_r1_h_dispatch_count_honesty() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 50.0 }])
            .unwrap();
        let with_seeds = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.0, 0.0))
            .unwrap();
        assert_eq!(with_seeds.source_setup_dispatches, 1);
        assert_eq!(with_seeds.propagation_dispatches, spec.horizon);
        assert_eq!(with_seeds.total_dispatches, spec.horizon + 1);

        let mut every_n = first_slice_spec();
        every_n.cadence = RegionFieldCadenceSpec::EveryN(2);
        let mut skip_session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &every_n,
        )
        .unwrap();
        skip_session
            .queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 50.0 }])
            .unwrap();
        skip_session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.0, 0.0))
            .unwrap();
        let skipped = skip_session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.0, 0.0))
            .unwrap();
        assert!(!skipped.scheduled);
        assert_eq!(skipped.source_setup_dispatches, 0);
        assert_eq!(skipped.propagation_dispatches, 0);
        assert_eq!(skipped.total_dispatches, 0);
    });
}

#[test]
fn test_r1_j_posture_preserved() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    assert!(!runtime_src.contains("ActiveOnlyExperimentalNoHalo"));
    assert!(!runtime_src.contains("source_mask"));
    assert!(!runtime_src.contains("request_atlas_batching"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
}

// --- M-first-slice-R2: GPU-resident Layer 1→2→3 bridge ---

#[test]
fn test_r2_a_hot_path_no_hidden_reduction_readback() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 3, col: 3, value: 70.0 }])
            .unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.3, 0.2))
            .unwrap();
        assert!(report.field_values.is_none());
        assert!(report.reduction_parent_value.is_none());
        assert!(report.eml_output.is_none());
        assert!(report.reduction_executed);
        assert!(report.eml_executed);
        assert_eq!(report.reduction_stencil_readbacks, 0);
    });
}

#[test]
fn test_r2_b_gpu_bridge_matches_debug_semantics() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let seeds = [FirstSliceSeed { row: 4, col: 4, value: 80.0 }];
        let weights = (0.2, 0.1);

        let mut debug = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        debug.queue_seeds(&seeds).unwrap();
        let debug_report = debug
            .tick(ctx, FirstSliceTickOptions::debug_readback(), weights)
            .unwrap();

        let mut hot = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        hot.queue_seeds(&seeds).unwrap();
        hot.tick(ctx, FirstSliceTickOptions::hot_path(), weights)
            .unwrap();
        let hot_field = hot.readback_canonical_field(ctx);
        let (hot_threat, hot_eml) = hot.diagnostic_readback_reduction_eml(ctx, weights).unwrap();

        assert_fields_near(
            debug_report.field_values.as_ref().unwrap(),
            &hot_field,
            1e-4,
            "hot vs debug field",
        );
        assert!(
            (debug_report.reduction_parent_value.unwrap() - hot_threat).abs() < 0.01,
            "reduction mismatch"
        );
        assert!(
            (debug_report.eml_output.unwrap() - hot_eml).abs() < 1e-4,
            "eml mismatch"
        );
    });
}

#[test]
fn test_r2_c_two_tick_gpu_bridge_persistence() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let weights = (0.5, 0.2);
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 1, col: 1, value: 55.0 }])
            .unwrap();
        session
            .tick(ctx, FirstSliceTickOptions::hot_path(), weights)
            .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 8, col: 8, value: 40.0 }])
            .unwrap();
        session
            .tick(ctx, FirstSliceTickOptions::hot_path(), weights)
            .unwrap();

        let config = compiled_stencil_to_gpu_config(
            &compile_region_field_preview(&spec).unwrap().stencil,
        );
        let cpu = run_caller_managed_cpu_ticks(
            &config,
            &[&[(1, 1, 55.0)], &[(8, 8, 40.0)]],
            spec.horizon,
        );
        let gpu = session.readback_canonical_field(ctx);
        let col = spec.source_col;
        let nd = spec.n_dims;
        for slot in 0..100 {
            assert!(
                (cpu[idx(slot, col, nd)] - gpu[idx(slot, col, nd)]).abs() <= 0.0001,
                "field slot={slot}"
            );
        }

        let (threat, eml) = session.diagnostic_readback_reduction_eml(ctx, weights).unwrap();
        assert!(threat.is_finite() && threat > 0.0);
        assert!(eml.is_finite());
    });
}

#[test]
fn test_r2_g_posture_preserved() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    assert!(!runtime_src.contains("ActiveOnlyExperimentalNoHalo"));
    assert!(!runtime_src.contains("source_mask"));
    assert!(!runtime_src.contains("request_atlas_batching"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
}

// --- M-first-slice-R3: readiness / observability parking ---

#[test]
fn test_r3_a_readiness_report_hot_path_shape() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 80.0 }])
            .unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.2, 0.1))
            .unwrap();
        let r = &report.readiness;

        assert!(report.scheduled);
        assert!(report.reduction_executed);
        assert!(report.eml_executed);
        assert_eq!(report.reduction_stencil_readbacks, 0);
        assert!(report.field_values.is_none());
        assert!(report.reduction_parent_value.is_none());
        assert!(report.eml_output.is_none());
        assert_eq!(report.source_setup_dispatches, 1);
        assert_eq!(report.propagation_dispatches, spec.horizon);
        assert_eq!(report.total_dispatches, spec.horizon + 1);

        assert!(r.mapping_enabled);
        assert_eq!(r.grid_size, 10);
        assert_eq!(r.cell_count, 100);
        assert_eq!(r.n_dims, spec.n_dims);
        assert_eq!(r.horizon, spec.horizon);
        assert_eq!(r.operator, "source_capped_normalized");
        assert_eq!(r.source_policy, "caller_managed_one_shot_seed_then_zero");
        assert_eq!(r.boundary_mode, "zero");
        assert_eq!(r.gpu_bridge_bytes_copied, (r.cell_count * r.n_dims * 4) as u64);
        assert_eq!(r.gpu_bridge_slot_col_writes, r.cell_count + 2);
        assert!(r.budget_estimate_bytes.unwrap() > 0);
        assert!(r.hot_path_wall_ms_observed.unwrap() > 0.0);
    });
}

#[test]
fn test_r3_b_debug_readback_explicit() {
    with_gpu(|ctx| {
        let spec = first_slice_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed { row: 2, col: 2, value: 90.0 }])
            .unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
            .unwrap();
        let r = &report.readiness;

        assert!(report.field_values.is_some());
        assert!(report.reduction_parent_value.is_some());
        assert!(report.eml_output.is_some());
        assert_eq!(report.reduction_stencil_readbacks, 0);
        assert!(r.field_values_present);
        assert!(r.parent_reduction_present);
        assert!(r.eml_output_present);
        assert!(r.hot_path_wall_ms_observed.is_none());
    });
}

#[test]
fn test_r3_c_budget_readiness_summary() {
    let spec = first_slice_spec();
    let budget = estimate_first_slice_budget(
        &spec,
        RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
    )
    .unwrap();
    assert!((budget.isolation_multiplier - 1.0).abs() < 1e-9);
    assert!(budget.estimated_bytes > 0);

    let mut over = spec.clone();
    over.max_region_field_vram_bytes = Some(4096);
    let err = estimate_region_field_budget(&RegionFieldBudgetSpec {
        grid_size: over.grid_size,
        column_count: over.n_dims,
        buffer_multiplier: 2.0,
        copy_multiplier: 1.0,
        tile_count: 1,
        isolation_policy: RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
        max_region_field_vram_bytes: over.max_region_field_vram_bytes,
    })
    .unwrap_err();
    assert!(err.requested_bytes > err.allowed_bytes);
}

#[test]
fn test_r3_d_no_accidental_feature_expansion() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    assert!(!runtime_src.contains("ActiveOnlyExperimentalNoHalo"));
    assert!(!runtime_src.contains("source_mask"));
    assert!(!runtime_src.contains("request_atlas_batching"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
}
