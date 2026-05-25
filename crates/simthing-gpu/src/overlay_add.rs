//! C-3 overlay Add → AccumulatorOp builders and old Pass 3 filtering.

use crate::accumulator_op::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};
use crate::world_state::{OverlayDelta, SlotDeltaRange, OP_ADD};

/// Convert flat overlay deltas into one [`AccumulatorOpGpu`] per Add delta,
/// preserving slot/delta traversal order. Multiply and Set deltas are skipped.
pub fn build_overlay_add_ops(
    deltas: &[OverlayDelta],
    ranges: &[SlotDeltaRange],
    n_slots: u32,
) -> Vec<AccumulatorOpGpu> {
    let mut ops = Vec::new();
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
            if delta.op_kind != OP_ADD {
                continue;
            }
            ops.push(AccumulatorOpGpu {
                source_kind:  source_kind::CONSTANT,
                source_slot:  delta.value.to_bits(),
                source_col:   0,
                source_count: 0,
                combine_kind: combine_kind::IDENTITY,
                combine_a:    0,
                combine_b:    0,
                combine_c:    0,
                combine_d:    0,
                gate_kind:    gate_kind::ORDER_BAND,
                gate_a:       0,
                gate_b:       0,
                scale_kind:   scale_kind::IDENTITY,
                scale_a:      0,
                consume:      consume_kind::NONE,
                target0_slot: slot as u32,
                target0_col:  delta.col,
                target1_slot: 0,
                target1_col:  0,
                target2_slot: 0,
                target2_col:  0,
                target3_slot: 0,
                target3_col:  0,
                n_targets:    1,
                _pad:         0,
            });
        }
    }
    ops
}

/// Filter overlay deltas to Multiply and Set only for old Pass 3.
pub fn filter_multiply_set_deltas(
    deltas: &[OverlayDelta],
    ranges: &[SlotDeltaRange],
    n_slots: u32,
) -> (Vec<OverlayDelta>, Vec<SlotDeltaRange>) {
    let mut new_deltas = Vec::new();
    let mut new_ranges = vec![SlotDeltaRange::default(); n_slots as usize];
    for slot in 0..n_slots as usize {
        if slot >= ranges.len() {
            break;
        }
        let range = ranges[slot];
        let start = new_deltas.len() as u32;
        for i in range.offset as usize..(range.offset + range.length) as usize {
            if i >= deltas.len() {
                break;
            }
            let delta = deltas[i];
            if delta.op_kind != OP_ADD {
                new_deltas.push(delta);
            }
        }
        new_ranges[slot] = SlotDeltaRange {
            offset: start,
            length: new_deltas.len() as u32 - start,
        };
    }
    (new_deltas, new_ranges)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_state::{OP_MULTIPLY, OP_SET};

    #[test]
    fn c3_build_overlay_add_ops_produces_one_op_per_add_delta() {
        let deltas = vec![
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 3.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 1,
                op_kind: OP_ADD,
                value: 1.5,
                _pad: 0,
            },
            OverlayDelta {
                col: 0,
                op_kind: OP_MULTIPLY,
                value: 2.0,
                _pad: 0,
            },
        ];
        let ranges = vec![
            SlotDeltaRange {
                offset: 0,
                length: 2,
            },
            SlotDeltaRange {
                offset: 2,
                length: 1,
            },
        ];
        let ops = build_overlay_add_ops(&deltas, &ranges, 2);
        assert_eq!(ops.len(), 2);
        assert_eq!(ops[0].target0_slot, 0);
        assert_eq!(ops[0].target0_col, 0);
        assert_eq!(ops[1].target0_col, 1);
    }

    #[test]
    fn c3_filter_multiply_set_retains_non_add() {
        let deltas = vec![
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 3.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 1,
                op_kind: OP_MULTIPLY,
                value: 2.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 2,
                op_kind: OP_SET,
                value: 0.5,
                _pad: 0,
            },
        ];
        let ranges = vec![SlotDeltaRange {
            offset: 0,
            length: 3,
        }];
        let (ms_deltas, ms_ranges) = filter_multiply_set_deltas(&deltas, &ranges, 1);
        assert_eq!(ms_deltas.len(), 2);
        assert_eq!(ms_ranges[0].length, 2);
        assert!(ms_deltas.iter().all(|d| d.op_kind != OP_ADD));
    }

    #[test]
    fn c3_empty_overlay_add_produces_no_ops() {
        let deltas = vec![OverlayDelta {
            col: 0,
            op_kind: OP_MULTIPLY,
            value: 2.0,
            _pad: 0,
        }];
        let ranges = vec![SlotDeltaRange {
            offset: 0,
            length: 1,
        }];
        let ops = build_overlay_add_ops(&deltas, &ranges, 1);
        assert!(ops.is_empty());
    }

    #[test]
    fn c3_overlay_add_op_value_in_correct_field() {
        let deltas = vec![OverlayDelta {
            col: 2,
            op_kind: OP_ADD,
            value: 7.5,
            _pad: 0,
        }];
        let ranges = vec![SlotDeltaRange {
            offset: 0,
            length: 1,
        }];
        let ops = build_overlay_add_ops(&deltas, &ranges, 1);
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].source_slot, 7.5_f32.to_bits());
        assert_eq!(ops[0].target0_col, 2);
    }
}
