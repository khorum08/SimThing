//! Phase M-1: generic structured-field execution API and column-aware reduction.

use simthing_core::{
    column_aware_reduction_op, manual_slot_range_sum_op, AccumulatorOp, ColumnAwareReductionCombine,
    ColumnAwareReductionSpec, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession, GpuContext,
    StructuredFieldExecutionOptions, StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy,
};
use simthing_sim::PipelineFlags;
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const N_DIMS: u32 = 4;
const CHILD_SLOTS: u32 = 9;
const PARENT_SLOT: u32 = 100;

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32) -> usize {
    (slot * N_DIMS + col) as usize
}

fn get(v: &[f32], slot: u32, col: u32) -> f32 {
    v[idx(slot, col)]
}

fn run_sum_ops(ctx: &GpuContext, ops: &[AccumulatorOp], values: &[f32], n_slots: u32) -> Vec<f32> {
    set_debug_readback_allowed(true);
    let mut session = AccumulatorOpSession::new(ctx, n_slots, N_DIMS);
    session.upload_values(ctx, values);
    session.upload_ops(ctx, ops).unwrap();
    session.tick(ctx, 0).unwrap();
    session.readback_full(ctx).unwrap()
}

#[test]
fn test_a_execution_api_horizon_guard() {
    with_gpu(|ctx| {
        let config = StructuredFieldStencilConfig {
            width: 3,
            height: 3,
            n_dims: N_DIMS,
            source_col: 0,
            target_col: 0,
            horizon: 4,
            alpha_self: 1.0,
            gamma_neighbor: 0.8,
            source_cap: None,
            operator: StructuredFieldStencilOperator::Normalized,
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        };
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let values = vec![0.0f32; op.config().values_len()];
        op.upload_values(ctx, &values).unwrap();

        let report = op
            .execute_configured(
                ctx,
                StructuredFieldExecutionOptions {
                    collect_field_stats: false,
                    steps: None,
                },
            )
            .unwrap();
        assert_eq!(report.debug.executed_horizon, 4);
        assert_eq!(report.debug.dispatch_count, 4);

        op.upload_values(ctx, &values).unwrap();
        let err = op
            .execute_configured(
                ctx,
                StructuredFieldExecutionOptions {
                    collect_field_stats: false,
                    steps: Some(8),
                },
            )
            .unwrap_err();
        assert!(format!("{err}").contains("exceed configured horizon"));
    });
}

#[test]
fn test_b_debug_report_fields_10x10() {
    with_gpu(|ctx| {
        let config = StructuredFieldStencilConfig {
            width: 10,
            height: 10,
            n_dims: N_DIMS,
            source_col: 0,
            target_col: 0,
            horizon: 4,
            alpha_self: 1.0,
            gamma_neighbor: 0.8,
            source_cap: None,
            operator: StructuredFieldStencilOperator::Normalized,
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        };
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[idx(44, 0)] = 50.0;
        op.upload_values(ctx, &values).unwrap();

        let report = op
            .execute_configured(
                ctx,
                StructuredFieldExecutionOptions {
                    collect_field_stats: true,
                    steps: None,
                },
            )
            .unwrap();
        let d = &report.debug;
        assert_eq!(d.dispatch_count, 4);
        assert_eq!(d.configured_horizon, 4);
        assert_eq!(d.executed_horizon, 4);
        assert_eq!(d.operator, StructuredFieldStencilOperator::Normalized);
        assert_eq!(
            d.source_policy,
            StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero
        );
        assert_eq!(d.boundary_mode, StructuredFieldStencilBoundaryMode::Zero);
        assert_eq!(d.mask_mode, StructuredFieldStencilMaskMode::All);
        assert_eq!(d.cell_count, 100);
        assert_eq!(d.values_len, 400);
        assert!(d.field_max.unwrap().is_finite());
        assert!(d.field_l1_norm.unwrap().is_finite());
        assert_eq!(d.active_mask_ratio, Some(1.0));
    });
}

#[test]
fn test_d_column_aware_reduction_matches_manual_slot_range_sum() {
    with_gpu(|ctx| {
        const CHILD_COL: u32 = 2;
        const PARENT_COL: u32 = 1;
        let n_slots = PARENT_SLOT + 1;

        let mut values = vec![0.0f32; (n_slots * N_DIMS) as usize];
        for s in 0..CHILD_SLOTS {
            values[idx(s, CHILD_COL)] = (s + 1) as f32;
        }

        let spec = ColumnAwareReductionSpec {
            child_slot_start: 0,
            child_slot_count: CHILD_SLOTS,
            child_col: CHILD_COL,
            parent_slot: PARENT_SLOT,
            parent_col: PARENT_COL,
            combine: ColumnAwareReductionCombine::Sum,
            order_band: 0,
        };
        let helper_op = column_aware_reduction_op(spec).unwrap();
        let manual_op = manual_slot_range_sum_op(0, CHILD_SLOTS, CHILD_COL, PARENT_SLOT, PARENT_COL, 0);
        assert_eq!(helper_op, manual_op);

        let helper_out = run_sum_ops(ctx, &[helper_op.clone()], &values, n_slots);
        let manual_out = run_sum_ops(ctx, &[manual_op], &values, n_slots);

        let expected_sum: f32 = (1..=CHILD_SLOTS).map(|i| i as f32).sum();
        assert!((get(&helper_out, PARENT_SLOT, PARENT_COL) - expected_sum).abs() < 1e-4);
        assert!((get(&manual_out, PARENT_SLOT, PARENT_COL) - expected_sum).abs() < 1e-4);
        assert!(
            (get(&helper_out, PARENT_SLOT, PARENT_COL) - get(&manual_out, PARENT_SLOT, PARENT_COL))
                .abs()
                < 1e-6,
            "helper vs manual parent col mismatch"
        );

        assert!(matches!(helper_op.combine, CombineFn::Sum));
        assert!(matches!(
            helper_op.source,
            SourceSpec::SlotRange { start: 0, count: 9, col: 2 }
        ));
        assert_eq!(helper_op.consume, ConsumeMode::ResetTarget);
        assert_eq!(helper_op.gate, GateSpec::OrderBand(0));
        assert_eq!(helper_op.scale, ScaleSpec::Identity);
    });
}

#[test]
fn test_e_no_production_pass_graph_wiring() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let passes = include_str!("../../simthing-gpu/src/passes.rs");
    assert!(!passes.contains("StructuredFieldStencilOp"));
    assert!(!passes.contains("execute_configured"));
    assert!(!passes.contains("RegionField"));

    let session = include_str!("../../simthing-driver/src/session.rs");
    assert!(!session.contains("StructuredFieldStencilOp"));
    assert!(!session.contains("execute_configured"));
    assert!(!session.contains("RegionField"));

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("StructuredFieldStencilOp"));
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("Mapping"));
}
