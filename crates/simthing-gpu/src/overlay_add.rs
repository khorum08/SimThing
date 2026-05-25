//! C-3 overlay Add → AccumulatorOp planning (Add-only batches, per-cell fold).

use crate::accumulator_op::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};
use crate::world_state::{OverlayDelta, SlotDeltaRange, OP_ADD};

/// Outcome of planning a C-3 overlay Add migration for one boundary sync.
#[derive(Debug, PartialEq)]
pub enum OverlayAddPlan {
    /// Every active overlay delta is `OP_ADD`; ops are folded per target cell.
    AllAdd { ops: Vec<AccumulatorOpGpu> },
    /// At least one Multiply or Set delta — caller must use legacy Pass 3 entirely.
    FallbackNonAdd,
}

fn make_folded_add_op(slot: u32, col: u32, folded_value: f32) -> AccumulatorOpGpu {
    AccumulatorOpGpu {
        source_kind:  source_kind::CONSTANT,
        source_slot:  folded_value.to_bits(),
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
        target0_slot: slot,
        target0_col:  col,
        target1_slot: 0,
        target1_col:  0,
        target2_slot: 0,
        target2_col:  0,
        target3_slot: 0,
        target3_col:  0,
        n_targets:    1,
        _pad:         0,
    }
}

/// Plan C-3 overlay Add migration for the current overlay delta batch.
///
/// Returns [`OverlayAddPlan::FallbackNonAdd`] when any active delta is not
/// `OP_ADD`. Otherwise folds Add deltas per `(slot, col)` in legacy delta order
/// and emits one AccumulatorOp registration per distinct cell.
pub fn plan_overlay_add_accumulator(
    deltas: &[OverlayDelta],
    ranges: &[SlotDeltaRange],
    n_slots: u32,
) -> OverlayAddPlan {
    for slot in 0..n_slots as usize {
        if slot >= ranges.len() {
            break;
        }
        let range = ranges[slot];
        for i in range.offset as usize..(range.offset + range.length) as usize {
            if i >= deltas.len() {
                break;
            }
            if deltas[i].op_kind != OP_ADD {
                return OverlayAddPlan::FallbackNonAdd;
            }
        }
    }

    let mut ordered_cells: Vec<(u32, u32)> = Vec::new();
    let mut folded: Vec<f32> = Vec::new();

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
            debug_assert_eq!(delta.op_kind, OP_ADD);

            let cell = (slot as u32, delta.col);
            if let Some(idx) = ordered_cells.iter().position(|&c| c == cell) {
                folded[idx] += delta.value;
            } else {
                ordered_cells.push(cell);
                folded.push(delta.value);
            }
        }
    }

    let ops = ordered_cells
        .into_iter()
        .zip(folded)
        .map(|((slot, col), value)| make_folded_add_op(slot, col, value))
        .collect();

    OverlayAddPlan::AllAdd { ops }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_state::{OP_MULTIPLY, OP_SET};

    fn slot0_range(len: u32) -> Vec<SlotDeltaRange> {
        vec![SlotDeltaRange { offset: 0, length: len }]
    }

    #[test]
    fn c3_mixed_add_multiply_falls_back_to_legacy_overlay() {
        let deltas = vec![
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 10.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 0,
                op_kind: OP_MULTIPLY,
                value: 2.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 1.0,
                _pad: 0,
            },
        ];
        let ranges = slot0_range(3);
        assert_eq!(
            plan_overlay_add_accumulator(&deltas, &ranges, 1),
            OverlayAddPlan::FallbackNonAdd,
        );
    }

    #[test]
    fn c3_same_cell_add_folds_in_legacy_order() {
        let deltas = vec![
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 1e20,
                _pad: 0,
            },
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 1.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: -1e20,
                _pad: 0,
            },
        ];
        let ranges = slot0_range(3);
        let expected = ((0.0f32 + 1e20f32) + 1.0f32) + (-1e20f32);

        let OverlayAddPlan::AllAdd { ops } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert_eq!(ops.len(), 1);
        assert_eq!(f32::from_bits(ops[0].source_slot), expected);
        assert_eq!(ops[0].target0_col, 0);
    }

    #[test]
    fn c3_add_only_batch_emits_one_op_per_target_cell() {
        let deltas = vec![
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 1.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 2.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 1,
                op_kind: OP_ADD,
                value: 3.0,
                _pad: 0,
            },
        ];
        let ranges = slot0_range(3);

        let OverlayAddPlan::AllAdd { ops } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert_eq!(ops.len(), 2);
        assert_eq!(f32::from_bits(ops[0].source_slot), 3.0);
        assert_eq!(ops[0].target0_col, 0);
        assert_eq!(f32::from_bits(ops[1].source_slot), 3.0);
        assert_eq!(ops[1].target0_col, 1);
    }

    #[test]
    fn c3_multiply_only_falls_back() {
        let deltas = vec![OverlayDelta {
            col: 0,
            op_kind: OP_MULTIPLY,
            value: 2.0,
            _pad: 0,
        }];
        let ranges = slot0_range(1);
        assert_eq!(
            plan_overlay_add_accumulator(&deltas, &ranges, 1),
            OverlayAddPlan::FallbackNonAdd,
        );
    }

    #[test]
    fn c3_set_only_falls_back() {
        let deltas = vec![OverlayDelta {
            col: 0,
            op_kind: OP_SET,
            value: 0.5,
            _pad: 0,
        }];
        let ranges = slot0_range(1);
        assert_eq!(
            plan_overlay_add_accumulator(&deltas, &ranges, 1),
            OverlayAddPlan::FallbackNonAdd,
        );
    }

    #[test]
    fn c3_empty_add_batch_emits_no_ops() {
        let deltas: Vec<OverlayDelta> = vec![];
        let ranges = slot0_range(0);
        let OverlayAddPlan::AllAdd { ops } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert!(ops.is_empty());
    }

    #[test]
    fn c3_folded_value_in_source_slot_field() {
        let deltas = vec![OverlayDelta {
            col: 2,
            op_kind: OP_ADD,
            value: 7.5,
            _pad: 0,
        }];
        let ranges = slot0_range(1);
        let OverlayAddPlan::AllAdd { ops } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].source_slot, 7.5_f32.to_bits());
        assert_eq!(ops[0].target0_col, 2);
    }
}
