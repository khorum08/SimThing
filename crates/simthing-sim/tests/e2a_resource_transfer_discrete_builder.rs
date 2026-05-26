//! E-2A — exact discrete transfer builder parity and conservation tests.

use simthing_core::{
    try_resource_transfer_discrete, AccumulatorOpBuilderError, CombineFn, ConsumeMode,
    DiscreteTransferRegistration, GateSpec, ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    discrete_transfer_registration_to_transfer, discrete_transfer_registrations_to_transfer,
    execute_ops_cpu, plan_transfer_ops, set_debug_readback_allowed, AccumulatorOpGpu,
    GpuContext, WorldGpuState,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn run_cpu_transfer(values: &mut [f32], op: &simthing_core::AccumulatorOp) {
    execute_ops_cpu(values, std::slice::from_ref(op), 0, 1).expect("cpu transfer oracle");
}

#[test]
fn e2a_builder_emits_subtract_from_source_op_shape() {
    let op = try_resource_transfer_discrete(1, 2, 3, 4, 25.0).unwrap();
    assert_eq!(
        op.source,
        SourceSpec::SlotValue {
            slot: 1,
            col: 2
        }
    );
    assert_eq!(op.combine, CombineFn::Identity);
    assert_eq!(op.gate, GateSpec::Always);
    assert_eq!(op.scale, ScaleSpec::Constant(25.0));
    assert_eq!(op.consume, ConsumeMode::SubtractFromSource);
    assert_eq!(op.targets, vec![(3, 4)]);
    op.validate().expect("valid op");
    AccumulatorOpGpu::from_op(&op).expect("encodes for GPU");
}

#[test]
fn e2a_discrete_transfer_debits_source_and_credits_target_exactly() {
    let op = try_resource_transfer_discrete(0, 0, 0, 1, 25.0).unwrap();
    let mut values = [100.0_f32, 10.0];
    let before_sum = values[0] + values[1];
    run_cpu_transfer(&mut values, &op);
    assert_eq!(values[0].to_bits(), 75.0_f32.to_bits());
    assert_eq!(values[1].to_bits(), 35.0_f32.to_bits());
    assert_eq!(values[0] + values[1], before_sum);
}

#[test]
fn e2a_discrete_transfer_clamps_to_available_source() {
    let op = try_resource_transfer_discrete(0, 0, 0, 1, 25.0).unwrap();
    let mut values = [12.0_f32, 10.0];
    run_cpu_transfer(&mut values, &op);
    assert_eq!(values[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(values[1].to_bits(), 22.0_f32.to_bits());
}

#[test]
fn e2a_zero_amount_is_noop() {
    let op = try_resource_transfer_discrete(0, 0, 0, 1, 0.0).unwrap();
    let mut values = [100.0_f32, 10.0];
    let before = values;
    run_cpu_transfer(&mut values, &op);
    assert_eq!(values[0].to_bits(), before[0].to_bits());
    assert_eq!(values[1].to_bits(), before[1].to_bits());
}

#[test]
fn e2a_negative_amount_rejected() {
    assert_eq!(
        try_resource_transfer_discrete(0, 0, 0, 1, -1.0),
        Err(AccumulatorOpBuilderError::NegativeAmount)
    );
}

#[test]
fn e2a_nan_or_infinite_amount_rejected() {
    for amount in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            try_resource_transfer_discrete(0, 0, 0, 1, amount),
            Err(AccumulatorOpBuilderError::NonFiniteAmount)
        );
    }
}

#[test]
fn e2a_builder_matches_c8c_transfer_planner_shape() {
    let reg = DiscreteTransferRegistration {
        source_slot: 0,
        source_col: 0,
        target_slot: 0,
        target_col: 1,
        amount: 25.0,
    };
    let builder_op = try_resource_transfer_discrete(
        reg.source_slot,
        reg.source_col,
        reg.target_slot,
        reg.target_col,
        reg.amount,
    )
    .unwrap();
    let transfer_regs = discrete_transfer_registrations_to_transfer(std::slice::from_ref(&reg));
    let plan = plan_transfer_ops(&transfer_regs).unwrap();
    assert_eq!(plan.ops.len(), 1);
    let planned = &plan.ops[0];
    assert_eq!(builder_op.source, planned.source);
    assert_eq!(builder_op.combine, planned.combine);
    assert_eq!(builder_op.scale, planned.scale);
    assert_eq!(builder_op.consume, planned.consume);
    assert_eq!(builder_op.targets, planned.targets);
    assert!(matches!(planned.gate, GateSpec::OrderBand(0)));
}

#[test]
fn e2a_transfer_executes_through_accumulator_op() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let reg = DiscreteTransferRegistration {
        source_slot: 0,
        source_col: 0,
        target_slot: 0,
        target_col: 1,
        amount: 25.0,
    };
    let op = try_resource_transfer_discrete(
        reg.source_slot,
        reg.source_col,
        reg.target_slot,
        reg.target_col,
        reg.amount,
    )
    .unwrap();

    let mut cpu_values = [100.0_f32, 10.0];
    run_cpu_transfer(&mut cpu_values, &op);

    let transfer_reg = discrete_transfer_registration_to_transfer(&reg);
    let mut state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), &{
        use simthing_core::{DimensionRegistry, SimProperty};
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "amount", 0));
        reg
    }, 1);
    state.write_values(&[100.0, 10.0, 0.0]);
    state
        .sync_transfer_accumulator(std::slice::from_ref(&transfer_reg))
        .expect("C-8c transfer upload");

    use simthing_gpu::{AccumulatorPipelineSessions, Pipelines};
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

    let gpu_values = state.read_values();
    assert_eq!(gpu_values[0].to_bits(), cpu_values[0].to_bits());
    assert_eq!(gpu_values[1].to_bits(), cpu_values[1].to_bits());
    assert_eq!(gpu_values[0] + gpu_values[1], cpu_values[0] + cpu_values[1]);
}
