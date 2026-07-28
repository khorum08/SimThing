//! Hand-built generic `CompiledAccumulatorOpPlan` fixtures — no scenario/driver/Studio imports.

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode, GateSpec,
    InputSpec, ScaleSpec, SlotIndex, SourceSpec, StructuralScalarChannel,
};

fn neighbor_sum_op(
    target_slot: u32,
    neighbor_slots: &[u32],
    input_col: ColumnIndex,
    output_col: ColumnIndex,
) -> AccumulatorOp {
    let inputs: Vec<InputSpec> = neighbor_slots
        .iter()
        .map(|&slot| InputSpec {
            slot: SlotIndex::new(slot),
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
        targets: vec![(SlotIndex::new(target_slot), output_col)],
    }
}

/// Two-slot vertical link gather: slot 0 ← [1], slot 1 ← [0].
pub fn two_slot_vertical_input_list_plan() -> CompiledAccumulatorOpPlan {
    let input_channel = StructuralScalarChannel::INPUT;
    let output_channel = StructuralScalarChannel::OUTPUT;
    let input_col = input_channel.into_plan_column();
    let output_col = output_channel.into_plan_column();
    CompiledAccumulatorOpPlan {
        slot_count: 2,
        n_dims: 2,
        input_channel,
        output_channel,
        ops: vec![
            neighbor_sum_op(0, &[1], input_col, output_col),
            neighbor_sum_op(1, &[0], input_col, output_col),
        ],
    }
}

/// Four-slot forked input-list gather matching driver-compiled fork topology:
/// slot 0 ← [1], slot 1 ← [0, 2, 3], slot 2 ← [1], slot 3 ← [1].
pub fn forked_four_slot_input_list_plan() -> CompiledAccumulatorOpPlan {
    let input_channel = StructuralScalarChannel::INPUT;
    let output_channel = StructuralScalarChannel::OUTPUT;
    let input_col = input_channel.into_plan_column();
    let output_col = output_channel.into_plan_column();
    CompiledAccumulatorOpPlan {
        slot_count: 4,
        n_dims: 2,
        input_channel,
        output_channel,
        ops: vec![
            neighbor_sum_op(0, &[1], input_col, output_col),
            neighbor_sum_op(1, &[0, 2, 3], input_col, output_col),
            neighbor_sum_op(2, &[1], input_col, output_col),
            neighbor_sum_op(3, &[1], input_col, output_col),
        ],
    }
}

/// Dense order oracle inputs for `forked_four_slot_input_list_plan`.
pub fn forked_four_slot_dense_inputs() -> Vec<f32> {
    vec![10.0, 20.0, 40.0, 30.0]
}
