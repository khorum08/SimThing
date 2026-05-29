//! Phase M-5C-gradient — product-facing need/routing signal reference fixture.
//!
//! RON/test-only: demonstrates generic scarcity/opportunity/cost routing pattern over
//! landed M-5A/M-5B substrate. Meaning lives in RON/spec; no production bridge.

use simthing_core::{ColumnAwareReductionCombine, ColumnAwareReductionSpec};
use simthing_driver::compiled_stencil_to_gpu_config;
use simthing_gpu::{cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilOperator};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_eml_gadget_stack, compile_region_field_preview, deserialize_eml_gadget_stack_ron,
    deserialize_region_field_ron, eval_eml_postfix, oracle_ema, oracle_weighted_accumulator,
    compile_region_field_frame_preview,
    CompiledGradientAxis, CompiledRegionFieldOperator, EmlGadgetCompileOptions, EmlGadgetKind,
    MappingExecutionProfile, RegionFieldOperatorSpec,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const SCALAR_FIELD_RON: &str = include_str!("fixtures/m5c_need_signal_scalar_field.ron");
const GRADIENT_X_FIELD_RON: &str = include_str!("fixtures/m5c_need_signal_gradient_x_field.ron");
const GRADIENT_Y_FIELD_RON: &str = include_str!("fixtures/m5c_need_signal_gradient_y_field.ron");
const L3_STACK_RON: &str = include_str!("fixtures/m5c_need_signal_l3_stack.ron");

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;
const ROUTING_WEIGHTS: [f32; 3] = [0.6, 0.25, 0.15];

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

fn build_asymmetric_need_grid(n_dims: u32, grid_size: u32, parent_slot: u32) -> Vec<f32> {
    let mut base = vec![0.0f32; ((parent_slot + 1) * n_dims) as usize];
    for row in 0..grid_size {
        for col in 0..grid_size {
            let slot = row * grid_size + col;
            // Scarcity/unmet demand rises east; labor opportunity rises south.
            base[idx(slot, 0, n_dims)] = 8.0 + col as f32 * 4.0 + row as f32 * 2.0;
        }
    }
    base
}

#[test]
fn m5c_frame_gradient_sink_validation_admits() {
    let scalar = deserialize_region_field_ron(SCALAR_FIELD_RON).expect("scalar RON");
    let gx = deserialize_region_field_ron(GRADIENT_X_FIELD_RON).expect("gx RON");
    let gy = deserialize_region_field_ron(GRADIENT_Y_FIELD_RON).expect("gy RON");
    let previews = compile_region_field_frame_preview(&[&scalar, &gx, &gy])
        .expect("M-5C fixture frame admits under gradient strict-sink rule");
    assert_eq!(previews.len(), 3);
}

#[test]
fn m5c_need_signal_fields_admit_with_single_target_gradients() {
    let scalar = deserialize_region_field_ron(SCALAR_FIELD_RON).expect("scalar RON");
    let gx = deserialize_region_field_ron(GRADIENT_X_FIELD_RON).expect("gx RON");
    let gy = deserialize_region_field_ron(GRADIENT_Y_FIELD_RON).expect("gy RON");

    assert_eq!(scalar.name, "unmet_demand_field");
    assert_eq!(gx.name, "price_differential_gradient_x");
    assert_eq!(gy.name, "labor_opportunity_gradient_y");

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

    let fixture_blob = format!("{SCALAR_FIELD_RON}{GRADIENT_X_FIELD_RON}{GRADIENT_Y_FIELD_RON}{L3_STACK_RON}");
    assert!(!fixture_blob.contains("GradientXY"));
    assert!(!fixture_blob.contains("output_col_x"));
    assert!(!fixture_blob.contains("output_col_y"));
}

#[test]
fn m5c_routing_signal_l3_stack_admits_with_ema_and_weighted_accumulator() {
    let stack = deserialize_eml_gadget_stack_ron(L3_STACK_RON).expect("L3 stack RON");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("L3 stack compiles");

    assert_eq!(compiled.gadgets.len(), 4);
    assert_eq!(
        compiled.report.gadget_ids,
        vec![
            "ema_unmet_demand",
            "ema_price_gradient_x",
            "ema_labor_opportunity_y",
            "routing_signal",
        ]
    );
    assert_eq!(compiled.gadgets[0].kind, EmlGadgetKind::Ema);
    assert_eq!(compiled.gadgets[3].kind, EmlGadgetKind::WeightedAccumulator);
}

#[test]
fn m5c_integrated_need_routing_signal_is_finite_and_deterministic() {
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

    let base = build_asymmetric_need_grid(n_dims, grid_size, parent_slot);
    let scalar_out = run_field_cpu_oracle(&base, &scalar_preview);
    let gx_out = run_field_cpu_oracle(&base, &gx_preview);
    let gy_out = run_field_cpu_oracle(&base, &gy_preview);

    let mut merged = base.clone();
    for slot in 0..slot_count {
        merged[idx(slot, 0, n_dims)] = scalar_out[idx(slot, 0, n_dims)];
        merged[idx(slot, 1, n_dims)] = gx_out[idx(slot, 1, n_dims)];
        merged[idx(slot, 2, n_dims)] = gy_out[idx(slot, 2, n_dims)];
    }

    let parent_need = slot_range_sum(
        &merged,
        n_dims,
        scalar_preview.reduction.as_ref().expect("scalar reduction"),
    );
    let parent_gx = slot_range_sum(
        &merged,
        n_dims,
        gx_preview.reduction.as_ref().expect("gx reduction"),
    );
    let parent_gy = slot_range_sum(
        &merged,
        n_dims,
        gy_preview.reduction.as_ref().expect("gy reduction"),
    );

    assert!(parent_need.is_finite());
    assert!(parent_gx.is_finite());
    assert!(parent_gy.is_finite());
    assert!(
        parent_gx.abs() > 1e-6,
        "price differential gradient parent should be nonzero, got {parent_gx}"
    );
    assert!(
        parent_gy.abs() > 1e-6,
        "labor opportunity gradient parent should be nonzero, got {parent_gy}"
    );

    let mut values = vec![0.0f32; (N_DIMS * (EVAL_SLOT + 1)) as usize];
    set_col(&mut values, 3, parent_need);
    set_col(&mut values, 4, parent_gx);
    set_col(&mut values, 5, parent_gy);
    set_col(&mut values, 20, ROUTING_WEIGHTS[0]);
    set_col(&mut values, 21, ROUTING_WEIGHTS[1]);
    set_col(&mut values, 22, ROUTING_WEIGHTS[2]);
    set_col(&mut values, 13, 0.0);

    let ema_need = eval_eml_postfix(&compiled_l3.gadgets[0].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_need - oracle_ema(parent_need, 0.0, 0.85)).abs() < 1e-5);

    set_col(&mut values, 13, ema_need);
    set_col(&mut values, 14, 0.0);
    let ema_gx = eval_eml_postfix(&compiled_l3.gadgets[1].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_gx - oracle_ema(parent_gx, 0.0, 0.9)).abs() < 1e-5);

    set_col(&mut values, 14, ema_gx);
    set_col(&mut values, 15, 0.0);
    let ema_gy = eval_eml_postfix(&compiled_l3.gadgets[2].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_gy - oracle_ema(parent_gy, 0.0, 0.9)).abs() < 1e-5);

    set_col(&mut values, 15, ema_gy);
    let routing_signal =
        eval_eml_postfix(&compiled_l3.gadgets[3].nodes, EVAL_SLOT, &values, N_DIMS);
    let oracle_routing =
        oracle_weighted_accumulator(&[ema_need, ema_gx, ema_gy], &ROUTING_WEIGHTS);
    assert!((routing_signal - oracle_routing).abs() < 1e-5);
    assert!(routing_signal.is_finite());

    let repeat = eval_eml_postfix(&compiled_l3.gadgets[3].nodes, EVAL_SLOT, &values, N_DIMS);
    assert_eq!(routing_signal, repeat);

    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn m5c_gradient_fields_gpu_parity_single_target() {
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
                values[idx(5, 0, config.n_dims)] = 12.0;
                values[idx(3, 0, config.n_dims)] = 2.0;
            } else {
                values[idx(7, 0, config.n_dims)] = 9.0;
                values[idx(1, 0, config.n_dims)] = 1.0;
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
fn m5c_posture_no_cpu_commitment_or_new_substrate() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("GradientXY"));
    assert!(!sim_lib.contains("ResourceEconomySpec"));

    let sources = [
        SCALAR_FIELD_RON,
        GRADIENT_X_FIELD_RON,
        GRADIENT_Y_FIELD_RON,
        L3_STACK_RON,
        include_str!("../../simthing-gpu/src/shaders/structured_field_stencil.wgsl"),
    ];
    for src in sources {
        assert!(!src.contains("source_mask"));
        assert!(!src.contains("GradientXY"));
        assert!(!src.contains("sqrt"));
    }
}
