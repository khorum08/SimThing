//! ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 — Sum-over-INPUT_LIST behavioral proofs.

use simthing_core::{
    is_exact_integer_f32, AccumulatorOp, CombineFn, ConsumeMode, GateSpec, InputSpec, ScaleSpec,
    SourceSpec, EXACT_INTEGER_F32_BOUND,
};
use simthing_gpu::execute_ops_cpu;
use simthing_gpu::{
    set_debug_readback_allowed, AccumulatorOpSession, GpuContext, PackedAccumulatorUpload,
};

fn neighbor_sum_ops_vertical_seed() -> Vec<AccumulatorOp> {
    vec![
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![InputSpec {
                    slot: 1,
                    col: 0,
                    unit_cost: 1.0,
                }],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(0, 1)],
        },
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![InputSpec {
                    slot: 0,
                    col: 0,
                    unit_cost: 1.0,
                }],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(1, 1)],
        },
    ]
}

fn neighbor_sum_ops_chain() -> Vec<AccumulatorOp> {
    vec![
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![InputSpec {
                    slot: 1,
                    col: 0,
                    unit_cost: 1.0,
                }],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(0, 1)],
        },
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![
                    InputSpec {
                        slot: 0,
                        col: 0,
                        unit_cost: 1.0,
                    },
                    InputSpec {
                        slot: 2,
                        col: 0,
                        unit_cost: 1.0,
                    },
                ],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(1, 1)],
        },
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![InputSpec {
                    slot: 1,
                    col: 0,
                    unit_cost: 1.0,
                }],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(2, 1)],
        },
    ]
}

fn neighbor_sum_ops_triangle() -> Vec<AccumulatorOp> {
    vec![
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![
                    InputSpec {
                        slot: 1,
                        col: 0,
                        unit_cost: 1.0,
                    },
                    InputSpec {
                        slot: 2,
                        col: 0,
                        unit_cost: 1.0,
                    },
                ],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(0, 1)],
        },
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![
                    InputSpec {
                        slot: 0,
                        col: 0,
                        unit_cost: 1.0,
                    },
                    InputSpec {
                        slot: 2,
                        col: 0,
                        unit_cost: 1.0,
                    },
                ],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(1, 1)],
        },
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![
                    InputSpec {
                        slot: 0,
                        col: 0,
                        unit_cost: 1.0,
                    },
                    InputSpec {
                        slot: 1,
                        col: 0,
                        unit_cost: 1.0,
                    },
                ],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(2, 1)],
        },
    ]
}

fn run_cpu_neighbor_sum(ops: &[AccumulatorOp], slot_count: usize, inputs: &[f32]) -> Vec<f32> {
    let n_dims = 2u32;
    let mut values = vec![0.0f32; slot_count * 2];
    for (slot, &input) in inputs.iter().enumerate() {
        values[slot * 2] = input;
    }
    execute_ops_cpu(&mut values, ops, 0, n_dims).expect("cpu oracle");
    (0..slot_count).map(|slot| values[slot * 2 + 1]).collect()
}

fn run_gpu_neighbor_sum(ops: &[AccumulatorOp], slot_count: u32, inputs: &[f32]) -> Vec<f32> {
    set_debug_readback_allowed(true);
    let ctx = GpuContext::new_blocking().expect("gpu context");
    let mut session = AccumulatorOpSession::new(&ctx, slot_count, 2);
    let mut values = vec![0.0f32; slot_count as usize * 2];
    for (slot, &input) in inputs.iter().enumerate() {
        values[slot * 2] = input;
    }
    session.upload_values(&ctx, &values);
    session
        .upload_packed_ops(
            &ctx,
            &PackedAccumulatorUpload::from_ops_resolving_input_lists(ops).unwrap(),
        )
        .expect("upload ops");
    session.tick(&ctx, 0).expect("tick");
    let readback = session.readback_full(&ctx).expect("readback");
    (0..slot_count as usize)
        .map(|slot| readback[slot * 2 + 1])
        .collect()
}

#[test]
fn accumulator_op_sum_over_input_list_cpu_oracle_vertical_seed() {
    let output = run_cpu_neighbor_sum(&neighbor_sum_ops_vertical_seed(), 2, &[10.0, 20.0]);
    assert_eq!(output, vec![20.0, 10.0]);
}

#[test]
fn accumulator_op_sum_over_input_list_cpu_oracle_chain() {
    let output = run_cpu_neighbor_sum(&neighbor_sum_ops_chain(), 3, &[10.0, 20.0, 30.0]);
    assert_eq!(output, vec![20.0, 40.0, 20.0]);
}

#[test]
fn accumulator_op_sum_over_input_list_cpu_oracle_triangle() {
    let output = run_cpu_neighbor_sum(&neighbor_sum_ops_triangle(), 3, &[10.0, 20.0, 30.0]);
    assert_eq!(output, vec![50.0, 40.0, 30.0]);
}

#[test]
fn accumulator_op_sum_over_input_list_rejects_invalid_input_list_range() {
    let op = AccumulatorOp {
        source: SourceSpec::ConjunctiveCrossing { inputs: vec![] },
        combine: CombineFn::Sum,
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::AddToTarget,
        targets: vec![(0, 1)],
    };
    use simthing_gpu::AccumulatorOpGpu;
    let err = AccumulatorOpGpu::from_op(&op).expect_err("empty");
    assert!(err.to_string().contains("ConjunctiveCrossing"));
}

#[test]
fn accumulator_op_sum_over_input_list_exact_integer_f32_contract() {
    for value in [
        10.0f32,
        20.0,
        EXACT_INTEGER_F32_BOUND,
        -EXACT_INTEGER_F32_BOUND,
    ] {
        assert!(is_exact_integer_f32(value), "{value}");
    }
    assert!(!is_exact_integer_f32(0.5));
    assert!(!is_exact_integer_f32((1u32 << 24) as f32 + 256.0));
}

#[test]
fn accumulator_op_sum_over_input_list_shader_contains_no_forbidden_semantic_terms() {
    const FORBIDDEN: &[&str] = &[
        "route",
        "predecessor",
        "pathfinding",
        "movement_order",
        "fleet",
        "faction",
        "owner",
        "border",
        "frontline",
        "combat",
        "economy",
        "diplomacy",
        "pirate",
    ];
    let source = include_str!("../src/shaders/accumulator_op.wgsl").to_ascii_lowercase();
    for token in FORBIDDEN {
        assert!(
            !source.contains(token),
            "forbidden token {token} in accumulator_op.wgsl"
        );
    }
}

#[test]
fn accumulator_op_sum_over_input_list_gpu_vertical_seed_real_adapter_or_partial() {
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    drop(ctx);
    let gpu = run_gpu_neighbor_sum(&neighbor_sum_ops_vertical_seed(), 2, &[10.0, 20.0]);
    assert_eq!(gpu, vec![20.0, 10.0]);
    eprintln!("ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1: REAL_ADAPTER_OBSERVED");
}
