//! Hand-built generic `CompiledAccumulatorOpPlan` fixtures — no scenario/driver/Studio imports.

use simthing_core::{
    AccumulatorOp, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode, GateSpec, InputSpec,
    ScaleSpec, SourceSpec, StructuralScalarChannel,
};

fn neighbor_sum_op(
    target_slot: u32,
    neighbor_slots: &[u32],
    input_col: u32,
    output_col: u32,
) -> AccumulatorOp {
    let inputs: Vec<InputSpec> = neighbor_slots
        .iter()
        .map(|&slot| InputSpec {
            slot,
            col: input_col,
            unit_cost: 1.0,
        })
        .collect();
    AccumulatorOp {
        source: SourceSpec::ConjunctiveCrossing { inputs },
        combine: CombineFn::Sum,
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::AddToTarget,
        targets: vec![(target_slot, output_col)],
    }
}

/// Two-slot vertical link gather: slot 0 ← [1], slot 1 ← [0].
pub fn two_slot_vertical_input_list_plan() -> CompiledAccumulatorOpPlan {
    let input_channel = StructuralScalarChannel(0);
    let output_channel = StructuralScalarChannel(1);
    CompiledAccumulatorOpPlan {
        slot_count: 2,
        n_dims: 2,
        input_channel,
        output_channel,
        ops: vec![
            neighbor_sum_op(0, &[1], input_channel.0, output_channel.0),
            neighbor_sum_op(1, &[0], input_channel.0, output_channel.0),
        ],
    }
}

/// Four-slot forked input-list gather matching driver-compiled fork topology:
/// slot 0 ← [1], slot 1 ← [0, 2, 3], slot 2 ← [1], slot 3 ← [1].
pub fn forked_four_slot_input_list_plan() -> CompiledAccumulatorOpPlan {
    let input_channel = StructuralScalarChannel(0);
    let output_channel = StructuralScalarChannel(1);
    CompiledAccumulatorOpPlan {
        slot_count: 4,
        n_dims: 2,
        input_channel,
        output_channel,
        ops: vec![
            neighbor_sum_op(0, &[1], input_channel.0, output_channel.0),
            neighbor_sum_op(1, &[0, 2, 3], input_channel.0, output_channel.0),
            neighbor_sum_op(2, &[1], input_channel.0, output_channel.0),
            neighbor_sum_op(3, &[1], input_channel.0, output_channel.0),
        ],
    }
}

/// Dense order oracle inputs for `forked_four_slot_input_list_plan`.
pub fn forked_four_slot_dense_inputs() -> Vec<f32> {
    vec![10.0, 20.0, 40.0, 30.0]
}
