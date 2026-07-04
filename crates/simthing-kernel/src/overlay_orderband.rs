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

}
