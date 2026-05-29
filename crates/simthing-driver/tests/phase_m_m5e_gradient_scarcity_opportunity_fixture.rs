//! Phase M-5E-gradient — scarcity/opportunity/logistics composite product fixture.
//!
//! RON/test-only: full-grid composite over landed M-5A/B/C/D substrate. Meaning lives in
//! RON/spec/test; no production bridge or new GPU substrate.

use simthing_core::{ColumnAwareReductionCombine, ColumnAwareReductionSpec};
use simthing_driver::compiled_stencil_to_gpu_config;
use simthing_gpu::{cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilOperator};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_eml_gadget_stack, compile_region_field_frame_preview, compile_region_field_preview,
    deserialize_eml_gadget_stack_ron, deserialize_region_field_ron, eval_eml_postfix, oracle_ema,
    oracle_weighted_accumulator, CompiledGradientAxis, CompiledRegionFieldOperator,
    EmlGadgetCompileOptions, EmlGadgetKind, MappingExecutionProfile, RegionFieldOperatorSpec,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const SCALAR_FIELD_RON: &str = include_str!("fixtures/m5e_scarcity_opportunity_scalar_field.ron");
const PRICE_GX_RON: &str =
    include_str!("fixtures/m5e_scarcity_opportunity_price_gradient_x_field.ron");
const LABOR_GY_RON: &str =
    include_str!("fixtures/m5e_scarcity_opportunity_labor_gradient_y_field.ron");
const LOGISTICS_GX_RON: &str =
    include_str!("fixtures/m5e_scarcity_opportunity_logistics_gradient_x_field.ron");
const L3_STACK_RON: &str = include_str!("fixtures/m5e_scarcity_opportunity_l3_stack.ron");

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;
const PRESSURE_WEIGHTS: [f32; 4] = [0.45, 0.25, 0.15, 0.15];

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

fn build_asymmetric_supply_grid(n_dims: u32, grid_size: u32, parent_slot: u32) -> Vec<f32> {
    let mut base = vec![0.0f32; ((parent_slot + 1) * n_dims) as usize];
    for row in 0..grid_size {
        for col in 0..grid_size {
            let slot = row * grid_size + col;
            // Scarcity/unmet demand rises east; labor opportunity rises south; supply reach rises east-west differential.
            base[idx(slot, 0, n_dims)] = 10.0 + col as f32 * 5.0 + row as f32 * 2.5;
        }
    }
    base
}

fn load_m5e_specs() -> [simthing_spec::RegionFieldSpec; 4] {
    [
        deserialize_region_field_ron(SCALAR_FIELD_RON).expect("scalar RON"),
        deserialize_region_field_ron(PRICE_GX_RON).expect("price gx RON"),
        deserialize_region_field_ron(LABOR_GY_RON).expect("labor gy RON"),
        deserialize_region_field_ron(LOGISTICS_GX_RON).expect("logistics gx RON"),
    ]
}

fn compile_m5e_frame() -> Vec<simthing_spec::CompiledRegionFieldPreview> {
    let specs = load_m5e_specs();
    compile_region_field_frame_preview(&[
        &specs[0], &specs[1], &specs[2], &specs[3],
    ])
    .expect("M-5E frame admits under gradient strict-sink rule")
}

struct ParentColumns {
    scarcity: f32,
    price_gx: f32,
    labor_gy: f32,
    logistics_gx: f32,
}

fn run_m5e_parent_columns(
    base: &[f32],
    previews: &[simthing_spec::CompiledRegionFieldPreview],
) -> ParentColumns {
    let n_dims = previews[0].stencil.n_dims;
    let grid_size = previews[0].grid_size;
    let slot_count = grid_size * grid_size;

    let scalar_out = run_field_cpu_oracle(base, &previews[0]);
    let price_out = run_field_cpu_oracle(base, &previews[1]);
    let labor_out = run_field_cpu_oracle(base, &previews[2]);
    let logistics_out = run_field_cpu_oracle(base, &previews[3]);

    let mut merged = base.to_vec();
    for slot in 0..slot_count {
        merged[idx(slot, 0, n_dims)] = scalar_out[idx(slot, 0, n_dims)];
        merged[idx(slot, 1, n_dims)] = price_out[idx(slot, 1, n_dims)];
        merged[idx(slot, 2, n_dims)] = labor_out[idx(slot, 2, n_dims)];
        merged[idx(slot, 3, n_dims)] = logistics_out[idx(slot, 3, n_dims)];
    }

    ParentColumns {
        scarcity: slot_range_sum(
            &merged,
            n_dims,
            previews[0].reduction.as_ref().expect("scalar reduction"),
        ),
        price_gx: slot_range_sum(
            &merged,
            n_dims,
            previews[1].reduction.as_ref().expect("price reduction"),
        ),
        labor_gy: slot_range_sum(
            &merged,
            n_dims,
            previews[2].reduction.as_ref().expect("labor reduction"),
        ),
        logistics_gx: slot_range_sum(
            &merged,
            n_dims,
            previews[3].reduction.as_ref().expect("logistics reduction"),
        ),
    }
}

fn eval_m5e_pressure(parents: &ParentColumns) -> f32 {
    let stack = deserialize_eml_gadget_stack_ron(L3_STACK_RON).expect("L3 stack RON");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("L3 stack compiles");

    let mut values = vec![0.0f32; (N_DIMS * (EVAL_SLOT + 1)) as usize];
    set_col(&mut values, 3, parents.scarcity);
    set_col(&mut values, 4, parents.price_gx);
    set_col(&mut values, 5, parents.labor_gy);
    set_col(&mut values, 6, parents.logistics_gx);
    for (i, &w) in PRESSURE_WEIGHTS.iter().enumerate() {
        set_col(&mut values, 20 + i as u32, w);
    }
    set_col(&mut values, 13, 0.0);
    set_col(&mut values, 14, 0.0);
    set_col(&mut values, 15, 0.0);
    set_col(&mut values, 16, 0.0);

    let ema_scarcity =
        eval_eml_postfix(&compiled.gadgets[0].nodes, EVAL_SLOT, &values, N_DIMS);
    set_col(&mut values, 13, ema_scarcity);
    let ema_price = eval_eml_postfix(&compiled.gadgets[1].nodes, EVAL_SLOT, &values, N_DIMS);
    set_col(&mut values, 14, ema_price);
    let ema_labor = eval_eml_postfix(&compiled.gadgets[2].nodes, EVAL_SLOT, &values, N_DIMS);
    set_col(&mut values, 15, ema_labor);
    let ema_logistics = eval_eml_postfix(&compiled.gadgets[3].nodes, EVAL_SLOT, &values, N_DIMS);
    set_col(&mut values, 16, ema_logistics);

    eval_eml_postfix(&compiled.gadgets[4].nodes, EVAL_SLOT, &values, N_DIMS)
}

#[test]
fn m5e_ron_fixtures_load_and_frame_admits() {
    let specs = load_m5e_specs();
    assert_eq!(specs[0].name, "scarcity_unmet_demand_field");
    assert_eq!(specs[1].name, "price_differential_gradient_x");
    assert_eq!(specs[2].name, "labor_opportunity_gradient_y");
    assert_eq!(specs[3].name, "supply_reach_logistics_gradient_x");

    let previews = compile_m5e_frame();
    assert_eq!(previews.len(), 4);
}

#[test]
fn m5e_fields_use_single_target_gradients_and_slotrange_sum() {
    let specs = load_m5e_specs();
    let previews = compile_m5e_frame();

    assert!(matches!(
        specs[0].operator,
        RegionFieldOperatorSpec::SourceCappedNormalized
    ));
    assert!(matches!(
        specs[1].operator,
        RegionFieldOperatorSpec::Gradient { .. }
    ));
    assert!(matches!(
        specs[2].operator,
        RegionFieldOperatorSpec::Gradient { .. }
    ));
    assert!(matches!(
        specs[3].operator,
        RegionFieldOperatorSpec::Gradient { .. }
    ));

    assert_eq!(
        previews[1].stencil.operator,
        CompiledRegionFieldOperator::Gradient {
            axis: CompiledGradientAxis::X
        }
    );
    assert_eq!(
        previews[2].stencil.operator,
        CompiledRegionFieldOperator::Gradient {
            axis: CompiledGradientAxis::Y
        }
    );
    assert_eq!(
        previews[3].stencil.operator,
        CompiledRegionFieldOperator::Gradient {
            axis: CompiledGradientAxis::X
        }
    );

    for preview in &previews {
        let reduction = preview.reduction.as_ref().expect("reduction");
        assert_eq!(reduction.combine, ColumnAwareReductionCombine::Sum);
        assert_eq!(reduction.parent_slot, 100);
    }

    let source_cols: Vec<u32> = specs.iter().map(|s| s.source_col).collect();
    let gradient_outputs: Vec<u32> = specs
        .iter()
        .filter_map(|s| {
            if let RegionFieldOperatorSpec::Gradient { output_col, .. } = s.operator {
                Some(output_col)
            } else {
                None
            }
        })
        .collect();
    for &out_col in &gradient_outputs {
        assert!(
            !source_cols.contains(&out_col),
            "strict-sink: no field may use gradient output_col {out_col} as source_col"
        );
    }

    let fixture_blob = format!(
        "{SCALAR_FIELD_RON}{PRICE_GX_RON}{LABOR_GY_RON}{LOGISTICS_GX_RON}{L3_STACK_RON}"
    );
    assert!(!fixture_blob.contains("GradientXY"));
    assert!(!fixture_blob.contains("output_col_x"));
    assert!(!fixture_blob.contains("output_col_y"));
}

#[test]
fn m5e_l3_stack_admits_with_ema_and_weighted_accumulator() {
    let stack = deserialize_eml_gadget_stack_ron(L3_STACK_RON).expect("L3 stack RON");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("L3 stack compiles");

    assert_eq!(compiled.gadgets.len(), 5);
    assert_eq!(
        compiled.report.gadget_ids,
        vec![
            "ema_scarcity_unmet_demand",
            "ema_price_differential_x",
            "ema_labor_opportunity_y",
            "ema_supply_reach_logistics_x",
            "scarcity_opportunity_logistics_pressure",
        ]
    );
    assert_eq!(compiled.gadgets[0].kind, EmlGadgetKind::Ema);
    assert_eq!(compiled.gadgets[4].kind, EmlGadgetKind::WeightedAccumulator);
}

#[test]
fn m5e_integrated_pressure_signal_is_finite_and_deterministic() {
    let previews = compile_m5e_frame();
    let n_dims = previews[0].stencil.n_dims;
    let grid_size = previews[0].grid_size;
    let parent_slot = 100u32;

    let base = build_asymmetric_supply_grid(n_dims, grid_size, parent_slot);
    let parents = run_m5e_parent_columns(&base, &previews);

    assert!(parents.scarcity.is_finite());
    assert!(parents.price_gx.is_finite());
    assert!(parents.labor_gy.is_finite());
    assert!(parents.logistics_gx.is_finite());
    assert!(
        parents.price_gx.abs() > 1e-6,
        "price gradient parent should be nonzero, got {}",
        parents.price_gx
    );
    assert!(
        parents.labor_gy.abs() > 1e-6,
        "labor gradient parent should be nonzero, got {}",
        parents.labor_gy
    );
    assert!(
        parents.logistics_gx.abs() > 1e-6,
        "logistics gradient parent should be nonzero, got {}",
        parents.logistics_gx
    );

    let stack = deserialize_eml_gadget_stack_ron(L3_STACK_RON).expect("L3 stack RON");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("L3 stack compiles");

    let mut values = vec![0.0f32; (N_DIMS * (EVAL_SLOT + 1)) as usize];
    set_col(&mut values, 3, parents.scarcity);
    set_col(&mut values, 4, parents.price_gx);
    set_col(&mut values, 5, parents.labor_gy);
    set_col(&mut values, 6, parents.logistics_gx);
    for (i, &w) in PRESSURE_WEIGHTS.iter().enumerate() {
        set_col(&mut values, 20 + i as u32, w);
    }
    set_col(&mut values, 13, 0.0);
    set_col(&mut values, 14, 0.0);
    set_col(&mut values, 15, 0.0);
    set_col(&mut values, 16, 0.0);

    let ema_scarcity =
        eval_eml_postfix(&compiled.gadgets[0].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_scarcity - oracle_ema(parents.scarcity, 0.0, 0.85)).abs() < 1e-5);
    set_col(&mut values, 13, ema_scarcity);

    let ema_price = eval_eml_postfix(&compiled.gadgets[1].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_price - oracle_ema(parents.price_gx, 0.0, 0.9)).abs() < 1e-5);
    set_col(&mut values, 14, ema_price);

    let ema_labor = eval_eml_postfix(&compiled.gadgets[2].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_labor - oracle_ema(parents.labor_gy, 0.0, 0.9)).abs() < 1e-5);
    set_col(&mut values, 15, ema_labor);

    let ema_logistics = eval_eml_postfix(&compiled.gadgets[3].nodes, EVAL_SLOT, &values, N_DIMS);
    assert!((ema_logistics - oracle_ema(parents.logistics_gx, 0.0, 0.88)).abs() < 1e-5);
    set_col(&mut values, 16, ema_logistics);

    let pressure =
        eval_eml_postfix(&compiled.gadgets[4].nodes, EVAL_SLOT, &values, N_DIMS);
    let oracle_pressure = oracle_weighted_accumulator(
        &[ema_scarcity, ema_price, ema_labor, ema_logistics],
        &PRESSURE_WEIGHTS,
    );
    assert!((pressure - oracle_pressure).abs() < 1e-5);
    assert!(pressure.is_finite());

    let repeat = eval_eml_postfix(&compiled.gadgets[4].nodes, EVAL_SLOT, &values, N_DIMS);
    assert_eq!(pressure, repeat);

    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn m5e_pressure_rises_with_higher_scarcity_seed() {
    let previews = compile_m5e_frame();
    let n_dims = previews[0].stencil.n_dims;
    let grid_size = previews[0].grid_size;
    let parent_slot = 100u32;

    let low_base = build_asymmetric_supply_grid(n_dims, grid_size, parent_slot);
    let mut high_base = low_base.clone();
    for row in 0..grid_size {
        for col in 0..grid_size {
            let slot = row * grid_size + col;
            high_base[idx(slot, 0, n_dims)] *= 2.0;
        }
    }

    let low_pressure = eval_m5e_pressure(&run_m5e_parent_columns(&low_base, &previews));
    let high_pressure = eval_m5e_pressure(&run_m5e_parent_columns(&high_base, &previews));

    assert!(low_pressure.is_finite());
    assert!(high_pressure.is_finite());
    assert!(
        high_pressure > low_pressure,
        "pressure should rise monotonically with scarcity seed: low={low_pressure} high={high_pressure}"
    );
}

#[test]
fn m5e_gradient_fields_gpu_parity_single_target() {
    with_gpu(|ctx| {
        for (ron, expected_op) in [
            (
                PRICE_GX_RON,
                StructuredFieldStencilOperator::GradientX,
            ),
            (
                LABOR_GY_RON,
                StructuredFieldStencilOperator::GradientY,
            ),
            (
                LOGISTICS_GX_RON,
                StructuredFieldStencilOperator::GradientX,
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
                values[idx(5, 0, config.n_dims)] = 14.0;
                values[idx(3, 0, config.n_dims)] = 4.0;
            } else {
                values[idx(7, 0, config.n_dims)] = 11.0;
                values[idx(1, 0, config.n_dims)] = 3.0;
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
fn m5e_posture_no_cpu_commitment_or_new_substrate() {
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
        PRICE_GX_RON,
        LABOR_GY_RON,
        LOGISTICS_GX_RON,
        L3_STACK_RON,
        include_str!("../../simthing-gpu/src/shaders/structured_field_stencil.wgsl"),
    ];
    for src in sources {
        assert!(!src.contains("source_mask"));
        assert!(!src.contains("GradientXY"));
        assert!(!src.contains("sqrt"));
        assert!(!src.contains("ActiveOnlyExperimentalNoHalo"));
    }
}
