//! Phase M-5B-gradient — L3 Strategic Pressure Composition reference fixture.
//!
//! RON/test-only: demonstrates multi-field L1 + L2 + L3 pattern over landed M-5A substrate.
//! No new runtime wiring; no production economy→mapping bridge.

use simthing_core::{ColumnAwareReductionCombine, ColumnAwareReductionSpec};
use simthing_driver::{
    compiled_stencil_to_gpu_config, FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions,
};
use simthing_gpu::{cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilOperator};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_eml_gadget_stack, compile_first_slice_scenario_preview, compile_region_field_preview,
    deserialize_eml_gadget_stack_ron, deserialize_first_slice_scenario_ron,
    deserialize_region_field_ron, eval_eml_postfix, oracle_ema, oracle_weighted_accumulator,
    CompiledGradientAxis, CompiledRegionFieldOperator, EmlGadgetCompileOptions, EmlGadgetKind,
    MappingExecutionProfile, RegionFieldOperatorSpec,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const SCALAR_FIELD_RON: &str = include_str!("fixtures/m5b_scalar_pressure_field.ron");
const GRADIENT_X_FIELD_RON: &str = include_str!("fixtures/m5b_gradient_x_field.ron");
const GRADIENT_Y_FIELD_RON: &str = include_str!("fixtures/m5b_gradient_y_field.ron");
const L3_STACK_RON: &str = include_str!("fixtures/m5b_l3_composition_gadget_stack.ron");
const REFERENCE_SCENARIO_RON: &str = include_str!("fixtures/m5b_reference_scenario.ron");

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn set_col(values: &mut [f32], col: u32, v: f32) {
    values[(EVAL_SLOT * N_DIMS + col) as usize] = v;
}

fn slot_range_sum(values: &[f32], n_dims: u32, reduction: &ColumnAwareReductionSpec) -> f32 {
    (0..reduction.child_slot_count)
        .map(|offset| {
            values[idx(
                reduction.child_slot_start + offset,
                reduction.child_col,
                n_dims,
            )]
        })
        .sum()
}

fn run_field_cpu_oracle(
    base: &[f32],
    preview: &simthing_spec::CompiledRegionFieldPreview,
) -> Vec<f32> {
    let config = compiled_stencil_to_gpu_config(&preview.stencil);
    let params = params_from_config(&config);
    cpu_horizon(base, &params, config.horizon)
}

#[test]
fn m5b_integrated_parent_columns_feed_l3_composite() {
    let scalar_preview =
        compile_region_field_preview(&deserialize_region_field_ron(SCALAR_FIELD_RON).unwrap())
            .expect("scalar admits");
    let gx_preview =
        compile_region_field_preview(&deserialize_region_field_ron(GRADIENT_X_FIELD_RON).unwrap())
            .expect("gx admits");
    let gy_preview =
        compile_region_field_preview(&deserialize_region_field_ron(GRADIENT_Y_FIELD_RON).unwrap())
            .expect("gy admits");
    let stack = deserialize_eml_gadget_stack_ron(L3_STACK_RON).expect("L3 stack RON");
    let compiled_l3 = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("L3 stack compiles");

    let n_dims = scalar_preview.stencil.n_dims;
    let grid_size = scalar_preview.grid_size;
    let slot_count = grid_size * grid_size;
    let parent_slot = 100u32;
    assert_eq!(gx_preview.stencil.n_dims, n_dims);
    assert_eq!(gy_preview.stencil.n_dims, n_dims);

    let mut base = vec![0.0f32; ((parent_slot + 1) * n_dims) as usize];
    for row in 0..grid_size {
        for col in 0..grid_size {
            let slot = row * grid_size + col;
            base[idx(slot, 0, n_dims)] = 5.0 + col as f32 * 3.0 + row as f32 * 1.5;
        }
    }

    let scalar_out = run_field_cpu_oracle(&base, &scalar_preview);
    let gx_out = run_field_cpu_oracle(&base, &gx_preview);
    let gy_out = run_field_cpu_oracle(&base, &gy_preview);

    let mut merged = base.clone();
    for slot in 0..slot_count {
        merged[idx(slot, 0, n_dims)] = scalar_out[idx(slot, 0, n_dims)];
        merged[idx(slot, 1, n_dims)] = gx_out[idx(slot, 1, n_dims)];
        merged[idx(slot, 2, n_dims)] = gy_out[idx(slot, 2, n_dims)];
    }

    let scalar_reduction = scalar_preview.reduction.as_ref().expect("scalar reduction");
    let gx_reduction = gx_preview.reduction.as_ref().expect("gx reduction");
    let gy_reduction = gy_preview.reduction.as_ref().expect("gy reduction");

    let parent_scalar = slot_range_sum(&merged, n_dims, scalar_reduction);
    let parent_gx = slot_range_sum(&merged, n_dims, gx_reduction);
    let parent_gy = slot_range_sum(&merged, n_dims, gy_reduction);

    assert!(parent_scalar.is_finite());
    assert!(parent_gx.is_finite());
    assert!(parent_gy.is_finite());
    assert!(
        parent_gx.abs() > 1e-6,
        "gradient_x parent reduction should be nonzero on asymmetric grid, got {parent_gx}"
    );
    assert!(
        parent_gy.abs() > 1e-6,
        "gradient_y parent reduction should be nonzero on asymmetric grid, got {parent_gy}"
    );

    assert_eq!(scalar_reduction.parent_col, 3);
    assert_eq!(gx_reduction.parent_col, 4);
    assert_eq!(gy_reduction.parent_col, 5);
    assert_eq!(scalar_reduction.parent_slot, parent_slot);

    let mut values = vec![0.0f32; (N_DIMS * (EVAL_SLOT + 1)) as usize];
    set_col(&mut values, 3, parent_scalar);
    set_col(&mut values, 4, parent_gx);
    set_col(&mut values, 5, parent_gy);
    set_col(&mut values, 20, 0.5);
    set_col(&mut values, 21, 0.3);
    set_col(&mut values, 22, 0.2);
    set_col(&mut values, 13, 0.0);

    let ema_scalar = eval_eml_postfix(&compiled_l3.gadgets[0].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_scalar - oracle_ema(parent_scalar, 0.0, 0.8)).abs() < 1e-5);

    set_col(&mut values, 13, ema_scalar);
    set_col(&mut values, 14, 0.0);
    let ema_gx = eval_eml_postfix(&compiled_l3.gadgets[1].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_gx - oracle_ema(parent_gx, 0.0, 0.9)).abs() < 1e-5);

    set_col(&mut values, 14, ema_gx);
    set_col(&mut values, 15, 0.0);
    let ema_gy = eval_eml_postfix(&compiled_l3.gadgets[2].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_gy - oracle_ema(parent_gy, 0.0, 0.9)).abs() < 1e-5);

    set_col(&mut values, 15, ema_gy);
    let composite = eval_eml_postfix(&compiled_l3.gadgets[3].nodes, EVAL_SLOT, &values, N_DIMS);
    let oracle_composite =
        oracle_weighted_accumulator(&[ema_scalar, ema_gx, ema_gy], &[0.5, 0.3, 0.2]);
    assert!((composite - oracle_composite).abs() < 1e-5);
    assert!(composite.is_finite());

    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn m5b_field_rons_admit_with_single_target_gradients() {
    let scalar = deserialize_region_field_ron(SCALAR_FIELD_RON).expect("scalar RON");
    let gx = deserialize_region_field_ron(GRADIENT_X_FIELD_RON).expect("gx RON");
    let gy = deserialize_region_field_ron(GRADIENT_Y_FIELD_RON).expect("gy RON");

    assert!(matches!(
        scalar.operator,
        RegionFieldOperatorSpec::SourceCappedNormalized
    ));
    assert!(matches!(
        gx.operator,
        RegionFieldOperatorSpec::Gradient { .. }
    ));
    assert!(matches!(
        gy.operator,
        RegionFieldOperatorSpec::Gradient { .. }
    ));

    let scalar_preview = compile_region_field_preview(&scalar).expect("scalar admits");
    let gx_preview = compile_region_field_preview(&gx).expect("gx admits");
    let gy_preview = compile_region_field_preview(&gy).expect("gy admits");

    assert_eq!(
        scalar_preview.stencil.operator,
        CompiledRegionFieldOperator::SourceCappedNormalized
    );
    assert_eq!(
        gx_preview.stencil.operator,
        CompiledRegionFieldOperator::Gradient {
            axis: CompiledGradientAxis::X
        }
    );
    assert_eq!(
        gy_preview.stencil.operator,
        CompiledRegionFieldOperator::Gradient {
            axis: CompiledGradientAxis::Y
        }
    );

    for preview in [&scalar_preview, &gx_preview, &gy_preview] {
        let reduction = preview.reduction.as_ref().expect("reduction");
        assert_eq!(reduction.combine, ColumnAwareReductionCombine::Sum);
        assert_eq!(reduction.parent_slot, 100);
    }

    assert_eq!(gx_preview.stencil.weight_east, 0.5);
    assert_eq!(gx_preview.stencil.weight_west, -0.5);
    assert_eq!(gy_preview.stencil.weight_north, -0.5);
    assert_eq!(gy_preview.stencil.weight_south, 0.5);

    let fixture_blob = format!("{SCALAR_FIELD_RON}{GRADIENT_X_FIELD_RON}{GRADIENT_Y_FIELD_RON}{L3_STACK_RON}");
    assert!(!fixture_blob.contains("GradientXY"));
    assert!(!fixture_blob.contains("output_col_x"));
    assert!(!fixture_blob.contains("output_col_y"));
}

#[test]
fn m5b_l3_gadget_stack_admits_with_ema_and_weighted_accumulator() {
    let stack = deserialize_eml_gadget_stack_ron(L3_STACK_RON).expect("L3 stack RON");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("L3 stack compiles");

    assert_eq!(compiled.gadgets.len(), 4);
    assert_eq!(
        compiled.report.gadget_ids,
        vec![
            "ema_scalar",
            "ema_gradient_x",
            "ema_gradient_y",
            "composite_signal",
        ]
    );
    assert_eq!(compiled.gadgets[0].kind, EmlGadgetKind::Ema);
    assert_eq!(compiled.gadgets[1].kind, EmlGadgetKind::Ema);
    assert_eq!(compiled.gadgets[2].kind, EmlGadgetKind::Ema);
    assert_eq!(compiled.gadgets[3].kind, EmlGadgetKind::WeightedAccumulator);
}

#[test]
fn m5b_l3_composition_oracle_is_deterministic_and_finite() {
    let stack = deserialize_eml_gadget_stack_ron(L3_STACK_RON).expect("stack");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("compiles");

    let mut values = vec![0.0f32; (N_DIMS * (EVAL_SLOT + 1)) as usize];
    set_col(&mut values, 3, 100.0);
    set_col(&mut values, 4, 4.0);
    set_col(&mut values, 5, -2.0);
    set_col(&mut values, 20, 0.5);
    set_col(&mut values, 21, 0.3);
    set_col(&mut values, 22, 0.2);

    set_col(&mut values, 13, 0.0);
    let ema_scalar = eval_eml_postfix(&compiled.gadgets[0].nodes, EVAL_SLOT, &values, N_DIMS);
    let oracle_scalar = oracle_ema(100.0, 0.0, 0.8);
    assert!((ema_scalar - oracle_scalar).abs() < 1e-5);

    set_col(&mut values, 13, ema_scalar);
    set_col(&mut values, 14, 0.0);
    let ema_gx = eval_eml_postfix(&compiled.gadgets[1].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_gx - oracle_ema(4.0, 0.0, 0.9)).abs() < 1e-5);

    set_col(&mut values, 14, ema_gx);
    set_col(&mut values, 15, 0.0);
    let ema_gy = eval_eml_postfix(&compiled.gadgets[2].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_gy - oracle_ema(-2.0, 0.0, 0.9)).abs() < 1e-5);

    set_col(&mut values, 15, ema_gy);
    let composite = eval_eml_postfix(&compiled.gadgets[3].nodes, EVAL_SLOT, &values, N_DIMS);
    let oracle_composite =
        oracle_weighted_accumulator(&[ema_scalar, ema_gx, ema_gy], &[0.5, 0.3, 0.2]);
    assert!((composite - oracle_composite).abs() < 1e-5);
    assert!(composite.is_finite());
}

#[test]
fn m5b_reference_scenario_admits_and_default_profile_disabled() {
    let scenario =
        deserialize_first_slice_scenario_ron(REFERENCE_SCENARIO_RON).expect("scenario RON");
    assert_eq!(
        scenario.mapping_execution_profile,
        MappingExecutionProfile::SparseRegionFieldV1
    );
    let preview = compile_first_slice_scenario_preview(&scenario).expect("scenario admits");
    assert!(preview.region_field.commitment.is_some());
    assert_eq!(
        preview.region_field.parent_formula_class.as_deref(),
        Some("field_urgency")
    );
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn m5b_gradient_fields_gpu_parity_single_target() {
    with_gpu(|ctx| {
        for (ron, expected_op) in [
            (
                GRADIENT_X_FIELD_RON,
                StructuredFieldStencilOperator::GradientX,
            ),
            (
                GRADIENT_Y_FIELD_RON,
                StructuredFieldStencilOperator::GradientY,
            ),
        ] {
            let spec = deserialize_region_field_ron(ron).expect("parse");
            let preview = compile_region_field_preview(&spec).expect("admit");
            let config = compiled_stencil_to_gpu_config(&preview.stencil);
            assert_eq!(config.operator, expected_op);
            assert_eq!(config.horizon, 1);

            let op = simthing_gpu::StructuredFieldStencilOp::new(ctx, config.clone()).unwrap();
            let mut values = vec![0.0f32; op.config().values_len()];
            if matches!(expected_op, StructuredFieldStencilOperator::GradientX) {
                values[idx(5, 0, config.n_dims)] = 10.0;
                values[idx(3, 0, config.n_dims)] = 0.0;
            } else {
                values[idx(7, 0, config.n_dims)] = 8.0;
                values[idx(1, 0, config.n_dims)] = 0.0;
            }
            op.upload_values(ctx, &values).unwrap();
            let (gpu, _) = op.run_ping_pong(ctx, 1).unwrap();
            let params = params_from_config(&config);
            let cpu = cpu_horizon(&values, &params, 1);
            for (i, (&g, &c)) in gpu.iter().zip(cpu.iter()).enumerate() {
                assert!((g - c).abs() <= 1e-4, "mismatch at {i}: gpu={g} cpu={c}");
            }
        }
    });
}

#[test]
fn m5b_reference_scenario_gpu_commitment_path_no_cpu_emission() {
    with_gpu(|ctx| {
        let scenario =
            deserialize_first_slice_scenario_ron(REFERENCE_SCENARIO_RON).expect("scenario");
        let preview = compile_first_slice_scenario_preview(&scenario).expect("admit");
        let commitment = preview
            .region_field
            .commitment
            .as_ref()
            .expect("commitment");
        let threshold = commitment.threshold;

        let mut session =
            FirstSliceMappingSession::open_from_scenario_preview(ctx, &preview).unwrap();
        session
            .queue_seeds(&[FirstSliceSeed {
                row: 5,
                col: 5,
                value: 100.0,
            }])
            .unwrap();

        let low = session
            .tick_with_commitment_spec_fixture(
                ctx,
                FirstSliceTickOptions::hot_path(),
                (0.2, 0.1),
                commitment,
            )
            .unwrap();
        let (_, low_urgency) = session
            .diagnostic_readback_reduction_eml(ctx, (0.2, 0.1))
            .unwrap();
        assert!(low_urgency.is_finite());
        assert!(low_urgency < threshold);
        assert!(low.threshold_events.is_empty());
        assert_eq!(low.mapping.reduction_stencil_readbacks, 0);

        session
            .queue_seeds(&[FirstSliceSeed {
                row: 5,
                col: 5,
                value: 100.0,
            }])
            .unwrap();
        let high = session
            .tick_with_commitment_spec_fixture(
                ctx,
                FirstSliceTickOptions::hot_path(),
                (0.9, 0.1),
                commitment,
            )
            .unwrap();
        let (_, high_urgency) = session
            .diagnostic_readback_reduction_eml(ctx, (0.9, 0.1))
            .unwrap();
        assert!(high_urgency > threshold);
        assert_eq!(high.threshold_events.len(), 1);
        assert_eq!(high.mapping.reduction_stencil_readbacks, 0);
    });
}

#[test]
fn m5b_posture_no_new_substrate() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("GradientXY"));

    let sources = [
        SCALAR_FIELD_RON,
        GRADIENT_X_FIELD_RON,
        GRADIENT_Y_FIELD_RON,
        L3_STACK_RON,
        REFERENCE_SCENARIO_RON,
        include_str!("../../simthing-gpu/src/shaders/structured_field_stencil.wgsl"),
    ];
    for src in sources {
        assert!(!src.contains("source_mask"));
        assert!(!src.contains("GradientXY"));
        assert!(!src.contains("sqrt"));
    }
}
