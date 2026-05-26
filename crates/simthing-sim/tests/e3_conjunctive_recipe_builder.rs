//! E-3 — conjunctive recipe builder parity and conservation tests.

use simthing_core::{
    try_conjunctive_recipe, AccumulatorOpBuilderError, CombineFn, ConjunctiveRecipeInput,
    ConjunctiveRecipeRegistration, ConsumeMode, GateSpec, ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    conjunctive_recipe_registrations_to_transfer, execute_ops_cpu, plan_transfer_ops,
    set_debug_readback_allowed, AccumulatorOpGpu, AccumulatorPipelineSessions, GpuContext,
    Pipelines, WorldGpuState,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn run_cpu_recipe(values: &mut [f32], op: &simthing_core::AccumulatorOp) {
    let n_dims = values.len() as u32;
    execute_ops_cpu(values, std::slice::from_ref(op), 0, n_dims).expect("cpu recipe oracle");
}

#[test]
fn e3_builder_emits_min_across_inputs_subtract_all_inputs_shape() {
    let op = try_conjunctive_recipe(&[(0, 0, 5.0), (0, 1, 3.0), (0, 2, 10.0)], 0, 3, 4).unwrap();
    let SourceSpec::ConjunctiveCrossing { inputs } = &op.source else {
        panic!("expected ConjunctiveCrossing");
    };
    assert_eq!(inputs.len(), 3);
    assert_eq!(inputs[0].unit_cost, 5.0);
    assert_eq!(op.combine, CombineFn::MinAcrossInputs);
    assert_eq!(op.gate, GateSpec::Always);
    assert_eq!(op.scale, ScaleSpec::Identity);
    assert_eq!(op.consume, ConsumeMode::SubtractFromAllInputs);
    assert_eq!(op.targets, vec![(0, 3)]);
    op.validate().expect("valid op");
    AccumulatorOpGpu::from_op(&op).expect("encodes for GPU layout");
}

#[test]
fn e3_three_input_recipe_conserves_exactly() {
    let op = try_conjunctive_recipe(&[(0, 0, 5.0), (0, 1, 3.0), (0, 2, 10.0)], 0, 3, 4).unwrap();
    let mut values = [10.0_f32, 9.0, 100.0, 0.0];
    run_cpu_recipe(&mut values, &op);
    assert_eq!(values[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(values[1].to_bits(), 3.0_f32.to_bits());
    assert_eq!(values[2].to_bits(), 80.0_f32.to_bits());
    assert_eq!(values[3].to_bits(), 2.0_f32.to_bits());
}

#[test]
fn e3_insufficient_single_input_clamps_recipe_count() {
    let op = try_conjunctive_recipe(&[(0, 0, 5.0), (0, 1, 3.0)], 0, 2, 4).unwrap();
    let mut values = [10.0_f32, 2.0, 0.0];
    let before = values;
    run_cpu_recipe(&mut values, &op);
    assert_eq!(values[0].to_bits(), before[0].to_bits());
    assert_eq!(values[1].to_bits(), before[1].to_bits());
    assert_eq!(values[2].to_bits(), before[2].to_bits());
}

#[test]
fn e3_zero_available_input_is_noop() {
    let op = try_conjunctive_recipe(&[(0, 0, 5.0), (0, 1, 3.0)], 0, 2, 4).unwrap();
    let mut values = [0.0_f32, 0.0, 0.0];
    let before = values;
    run_cpu_recipe(&mut values, &op);
    assert_eq!(values[0].to_bits(), before[0].to_bits());
    assert_eq!(values[1].to_bits(), before[1].to_bits());
    assert_eq!(values[2].to_bits(), before[2].to_bits());
}

#[test]
fn e3_n8_recipe_validate_and_executes() {
    let inputs: Vec<(u32, u32, f32)> = (0..8).map(|c| (0, c, 2.0)).collect();
    let op = try_conjunctive_recipe(&inputs, 0, 8, 1).unwrap();
    op.validate().expect("N=8 validates");

    let mut values = [8.0_f32; 9];
    values[8] = 0.0;
    run_cpu_recipe(&mut values, &op);
    for col in 0..8 {
        assert_eq!(values[col].to_bits(), 0.0_f32.to_bits(), "col {col} debited");
    }
    assert_eq!(values[8].to_bits(), 4.0_f32.to_bits());

    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping GPU N=8 portion: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let reg = ConjunctiveRecipeRegistration {
        inputs: (0..8)
            .map(|c| ConjunctiveRecipeInput {
                slot: 0,
                col: c,
                unit_cost: 2.0,
            })
            .collect(),
        target_slot: 0,
        target_col: 8,
        throttle_hint_max_per_tick: 1,
    };
    let transfer_regs = conjunctive_recipe_registrations_to_transfer(std::slice::from_ref(&reg));
    let plan = plan_transfer_ops(&transfer_regs).unwrap();
    assert_eq!(plan.input_lists[0].len(), 8);

    use simthing_core::{
        ClampBehavior, DimensionRegistry, PropertyLayout, SimProperty, SubFieldRole, SubFieldSpec,
    };
    let mut dim = DimensionRegistry::new();
    let sub_fields: Vec<SubFieldSpec> = (0..9)
        .map(|i| SubFieldSpec {
            role: SubFieldRole::Named(format!("col{i}")),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: format!("col{i}"),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
        })
        .collect();
    dim.register(SimProperty {
        namespace: "recipe".into(),
        name: "n8".into(),
        layout: PropertyLayout { sub_fields },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    });

    let mut state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), &dim, 1);
    let mut initial = [8.0_f32; 9];
    initial[8] = 0.0;
    state.write_values(&initial);
    state
        .sync_transfer_accumulator(&transfer_regs)
        .expect("C-8c upload");

    let pipelines = Pipelines::new(&state.ctx);
    let mut transfer_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_transfer_session();
    pipelines.run_tick_pipeline_with_accumulators(
        &mut state,
        1.0,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: transfer_session.as_mut(),
            emission: None,
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_transfer_session(transfer_session);

    let gpu = state.read_values();
    for col in 0..8 {
        assert_eq!(gpu[col].to_bits(), 0.0_f32.to_bits());
    }
    assert_eq!(gpu[8].to_bits(), 4.0_f32.to_bits());
}

#[test]
fn e3_invalid_empty_inputs_rejected() {
    assert_eq!(
        try_conjunctive_recipe(&[], 0, 0, 1),
        Err(AccumulatorOpBuilderError::EmptyConjunctiveInputs)
    );
}

#[test]
fn e3_invalid_nonpositive_unit_cost_rejected() {
    assert_eq!(
        try_conjunctive_recipe(&[(0, 0, 0.0)], 0, 1, 1),
        Err(AccumulatorOpBuilderError::NonPositiveUnitCost { slot: 0, col: 0 })
    );
    assert_eq!(
        try_conjunctive_recipe(&[(0, 0, -2.0)], 0, 1, 1),
        Err(AccumulatorOpBuilderError::NonPositiveUnitCost { slot: 0, col: 0 })
    );
}

#[test]
fn e3_invalid_nonfinite_unit_cost_rejected() {
    assert_eq!(
        try_conjunctive_recipe(&[(0, 0, f32::NAN)], 0, 1, 1),
        Err(AccumulatorOpBuilderError::NonPositiveUnitCost { slot: 0, col: 0 })
    );
}

#[test]
fn e3_max_per_tick_is_metadata_not_gpu_cap() {
    // Inputs afford 4 recipe units; throttle hint says 1 — GPU/CPU still emit 4.
    let op = try_conjunctive_recipe(&[(0, 0, 2.0), (0, 1, 2.0)], 0, 2, 1).unwrap();
    let mut values = [8.0_f32, 8.0, 0.0];
    run_cpu_recipe(&mut values, &op);
    assert_eq!(values[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(values[1].to_bits(), 0.0_f32.to_bits());
    assert_eq!(values[2].to_bits(), 4.0_f32.to_bits());

    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping GPU metadata-cap portion: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let reg = ConjunctiveRecipeRegistration {
        inputs: vec![
            ConjunctiveRecipeInput {
                slot: 0,
                col: 0,
                unit_cost: 2.0,
            },
            ConjunctiveRecipeInput {
                slot: 0,
                col: 1,
                unit_cost: 2.0,
            },
        ],
        target_slot: 0,
        target_col: 2,
        throttle_hint_max_per_tick: 1,
    };
    let transfer_regs = conjunctive_recipe_registrations_to_transfer(std::slice::from_ref(&reg));

    use simthing_core::{
        ClampBehavior, DimensionRegistry, PropertyLayout, SimProperty, SubFieldRole, SubFieldSpec,
    };
    let mut dim = DimensionRegistry::new();
    let sub_fields: Vec<SubFieldSpec> = (0..3)
        .map(|i| SubFieldSpec {
            role: SubFieldRole::Named(format!("col{i}")),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: format!("col{i}"),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
        })
        .collect();
    dim.register(SimProperty {
        namespace: "recipe".into(),
        name: "meta".into(),
        layout: PropertyLayout { sub_fields },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    });

    let mut state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), &dim, 1);
    state.write_values(&[8.0, 8.0, 0.0]);
    state
        .sync_transfer_accumulator(&transfer_regs)
        .expect("C-8c upload");

    let pipelines = Pipelines::new(&state.ctx);
    let mut transfer_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_transfer_session();
    pipelines.run_tick_pipeline_with_accumulators(
        &mut state,
        1.0,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: transfer_session.as_mut(),
            emission: None,
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_transfer_session(transfer_session);

    let gpu = state.read_values();
    assert_eq!(gpu[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(gpu[1].to_bits(), 0.0_f32.to_bits());
    assert_eq!(gpu[2].to_bits(), 4.0_f32.to_bits());
}

#[test]
fn e3_invalid_throttle_hint_max_per_tick_rejected() {
    assert_eq!(
        try_conjunctive_recipe(&[(0, 0, 1.0)], 0, 1, 0),
        Err(AccumulatorOpBuilderError::InvalidThrottleHintMaxPerTick)
    );
}

#[test]
fn e3_builder_matches_c8c_transfer_planner_shape() {
    let reg = ConjunctiveRecipeRegistration {
        inputs: vec![
            ConjunctiveRecipeInput {
                slot: 0,
                col: 0,
                unit_cost: 5.0,
            },
            ConjunctiveRecipeInput {
                slot: 0,
                col: 1,
                unit_cost: 3.0,
            },
        ],
        target_slot: 0,
        target_col: 2,
        throttle_hint_max_per_tick: 4,
    };
    let builder_op = try_conjunctive_recipe(
        &[(0, 0, 5.0), (0, 1, 3.0)],
        reg.target_slot,
        reg.target_col,
        reg.throttle_hint_max_per_tick,
    )
    .unwrap();
    let transfer_regs = conjunctive_recipe_registrations_to_transfer(std::slice::from_ref(&reg));
    let plan = plan_transfer_ops(&transfer_regs).unwrap();
    let planned = &plan.ops[0];
    assert_eq!(builder_op.source, planned.source);
    assert_eq!(builder_op.combine, planned.combine);
    assert_eq!(builder_op.scale, planned.scale);
    assert_eq!(builder_op.consume, planned.consume);
    assert_eq!(builder_op.targets, planned.targets);
    assert!(matches!(planned.gate, GateSpec::OrderBand(0)));
}
