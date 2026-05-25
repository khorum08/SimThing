//! C-4 overlay Add/Multiply/Set -> AccumulatorOp OrderBand planning.
//!
//! This planner consumes the canonical `build_overlay_deltas` output. It does
//! not walk the tree or reinterpret overlay lifecycle state.

use std::collections::HashMap;

use crate::accumulator_op::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};
use crate::world_state::{OverlayDelta, SlotDeltaRange, OP_ADD, OP_MULTIPLY, OP_SET};

#[derive(Debug, PartialEq)]
pub struct OverlayOrderBandPlan {
    pub ops: Vec<AccumulatorOpGpu>,
    pub n_bands: u32,
}

fn make_overlay_op(slot: u32, col: u32, value: f32, op_kind: u32, band: u32) -> AccumulatorOpGpu {
    let consume = match op_kind {
        OP_ADD => consume_kind::ADD_TO_TARGET,
        OP_MULTIPLY => consume_kind::SCALE_TARGET,
        OP_SET => consume_kind::RESET_TARGET,
        other => panic!("unsupported overlay op kind {other}"),
    };

    AccumulatorOpGpu {
        source_kind: source_kind::CONSTANT,
        source_slot: value.to_bits(),
        source_col: 0,
        source_count: 0,
        combine_kind: combine_kind::IDENTITY,
        combine_a: 0,
        combine_b: 0,
        combine_c: 0,
        combine_d: 0,
        gate_kind: gate_kind::ORDER_BAND,
        gate_a: band,
        gate_b: 0,
        scale_kind: scale_kind::IDENTITY,
        scale_a: 0,
        consume,
        target0_slot: slot,
        target0_col: col,
        target1_slot: 0,
        target1_col: 0,
        target2_slot: 0,
        target2_col: 0,
        target3_slot: 0,
        target3_col: 0,
        n_targets: 1,
        _pad: 0,
    }
}

pub fn plan_overlay_orderband(
    deltas: &[OverlayDelta],
    ranges: &[SlotDeltaRange],
    n_slots: u32,
) -> OverlayOrderBandPlan {
    let mut next_band: HashMap<(u32, u32), u32> = HashMap::new();
    let mut ops = Vec::with_capacity(deltas.len());
    let mut n_bands = 0u32;

    for slot in 0..n_slots as usize {
        if slot >= ranges.len() {
            break;
        }
        let range = ranges[slot];
        for i in range.offset as usize..(range.offset + range.length) as usize {
            if i >= deltas.len() {
                break;
            }
            let delta = deltas[i];
            let cell = (slot as u32, delta.col);
            let band = *next_band.get(&cell).unwrap_or(&0);
            next_band.insert(cell, band + 1);
            n_bands = n_bands.max(band + 1);
            ops.push(make_overlay_op(
                cell.0,
                cell.1,
                delta.value,
                delta.op_kind,
                band,
            ));
        }
    }

    debug_assert_no_duplicate_band_slot_col(&ops);

    OverlayOrderBandPlan { ops, n_bands }
}

fn debug_assert_no_duplicate_band_slot_col(ops: &[AccumulatorOpGpu]) {
    #[cfg(debug_assertions)]
    {
        use std::collections::HashSet;

        let mut seen: HashSet<(u32, u32, u32)> = HashSet::new();
        for op in ops {
            let key = (op.gate_a, op.target0_slot, op.target0_col);
            assert!(
                seen.insert(key),
                "overlay OrderBand planner produced duplicate (band, slot, col): {key:?}"
            );
        }
    }
    #[cfg(not(debug_assertions))]
    let _ = ops;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range(offset: u32, length: u32) -> SlotDeltaRange {
        SlotDeltaRange { offset, length }
    }

    fn delta(col: u32, op_kind: u32, value: f32) -> OverlayDelta {
        OverlayDelta {
            col,
            op_kind,
            value,
            _pad: 0,
        }
    }

    #[test]
    fn c4_add_only_matches_c3_planner_output() {
        let deltas = vec![
            delta(0, OP_ADD, 1.0),
            delta(0, OP_ADD, 2.0),
            delta(1, OP_ADD, 3.0),
        ];
        let plan = plan_overlay_orderband(&deltas, &[range(0, 3)], 1);
        assert_eq!(plan.ops.len(), 3);
        assert_eq!(plan.n_bands, 2);
        assert_eq!(plan.ops[0].consume, consume_kind::ADD_TO_TARGET);
        assert_eq!(plan.ops[0].gate_a, 0);
        assert_eq!(plan.ops[1].gate_a, 1);
        assert_eq!(plan.ops[2].gate_a, 0);
    }

    #[test]
    fn c4_same_cell_add_mul_set_assigns_increasing_bands() {
        let deltas = vec![
            delta(0, OP_ADD, 5.0),
            delta(0, OP_MULTIPLY, 2.0),
            delta(0, OP_SET, 0.0),
        ];
        let plan = plan_overlay_orderband(&deltas, &[range(0, 3)], 1);
        assert_eq!(plan.n_bands, 3);
        assert_eq!(
            plan.ops.iter().map(|op| op.gate_a).collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        assert_eq!(plan.ops[0].consume, consume_kind::ADD_TO_TARGET);
        assert_eq!(plan.ops[1].consume, consume_kind::SCALE_TARGET);
        assert_eq!(plan.ops[2].consume, consume_kind::RESET_TARGET);
    }

    #[test]
    fn c4_different_cells_share_same_band_across_op_kinds() {
        let deltas = vec![
            delta(0, OP_ADD, 1.0),
            delta(1, OP_MULTIPLY, 2.0),
            delta(0, OP_SET, 3.0),
        ];
        let ranges = vec![range(0, 2), range(2, 1)];
        let plan = plan_overlay_orderband(&deltas, &ranges, 2);
        assert_eq!(plan.n_bands, 1);
        assert!(plan.ops.iter().all(|op| op.gate_a == 0));
    }

    #[test]
    fn c4_ancestor_then_local_correct_band_order() {
        let deltas = vec![delta(0, OP_MULTIPLY, 0.9), delta(0, OP_ADD, -0.05)];
        let plan = plan_overlay_orderband(&deltas, &[range(0, 2)], 1);
        assert_eq!(plan.ops[0].consume, consume_kind::SCALE_TARGET);
        assert_eq!(plan.ops[0].gate_a, 0);
        assert_eq!(plan.ops[1].consume, consume_kind::ADD_TO_TARGET);
        assert_eq!(plan.ops[1].gate_a, 1);
    }

    #[test]
    fn c4_empty_batch_emits_no_ops_zero_bands() {
        let plan = plan_overlay_orderband(&[], &[range(0, 0)], 1);
        assert!(plan.ops.is_empty());
        assert_eq!(plan.n_bands, 0);
    }

    #[test]
    fn c4_planner_no_duplicate_band_slot_col() {
        let deltas = vec![
            delta(0, OP_ADD, 1.0),
            delta(1, OP_MULTIPLY, 2.0),
            delta(0, OP_SET, 3.0),
        ];
        let _ = plan_overlay_orderband(&deltas, &[range(0, 3)], 1);
    }
}
