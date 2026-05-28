//! Phase M-first-slice — opt-in mapping runtime integration tests.

use simthing_core::{manual_slot_range_sum_op, WHITELISTED_FORMULA_CLASSES};
use simthing_driver::{
    compiled_stencil_to_gpu_config, estimate_first_slice_budget, FirstSliceMappingSession,
    FirstSliceSeed, FirstSliceTickOptions, FieldDispatchSchedule,
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
    let params = params_from_config(config);
    let n_dims = config.n_dims;
    let w = config.width;
    let mut values = vec![0.0f32; config.values_len()];
    for &(r, c, v) in seeds {
        values[idx(r * w + c, config.source_col, n_dims)] = v;
    }
    values = cpu_horizon(&values, &params, 1);
    for &(r, c, _) in seeds {
        values[idx(r * w + c, config.source_col, n_dims)] = 0.0;
    }
    cpu_horizon(&values, &params, hops)
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
        ]);
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
        session.queue_seeds(&[FirstSliceSeed { row: 5, col: 5, value: 50.0 }]);
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
        session.queue_seeds(&[FirstSliceSeed { row: 4, col: 4, value: 80.0 }]);
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
        session.queue_seeds(&[FirstSliceSeed { row: 2, col: 2, value: 90.0 }]);
        let low = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
            .unwrap();
        session.queue_seeds(&[FirstSliceSeed { row: 2, col: 2, value: 90.0 }]);
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
        session.queue_seeds(&[FirstSliceSeed { row: 3, col: 3, value: 70.0 }]);
        let report = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.3, 0.2))
            .unwrap();
        assert!(report.field_values.is_none());
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
        disabled.queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 100.0 }]);
        assert!(!disabled.tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1)).unwrap().scheduled);

        let mut enabled = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        enabled.queue_seeds(&[FirstSliceSeed { row: 0, col: 0, value: 100.0 }]);
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
            s.queue_seeds(&seeds);
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
