//! C-3 overlay Add → AccumulatorOp planning (Add-only batches, OrderBand sequencing).

use std::collections::HashMap;

use crate::accumulator_op::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};
use crate::world_state::{OverlayDelta, SlotDeltaRange, OP_ADD};

/// Outcome of planning a C-3 overlay Add migration for one boundary sync.
#[derive(Debug, PartialEq)]
pub enum OverlayAddPlan {
    /// Every active overlay delta is `OP_ADD`; one op per delta with per-cell OrderBands.
    AllAdd {
        ops:     Vec<AccumulatorOpGpu>,
        n_bands: u32,
    },
    /// At least one Multiply or Set delta — caller must use legacy Pass 3 entirely.
    FallbackNonAdd,
}

fn make_add_op(slot: u32, col: u32, value: f32, band: u32) -> AccumulatorOpGpu {
    AccumulatorOpGpu {
        source_kind:  source_kind::CONSTANT,
        source_slot:  value.to_bits(),
        source_col:   0,
        source_count: 0,
        combine_kind: combine_kind::IDENTITY,
        combine_a:    0,
        combine_b:    0,
        combine_c:    0,
        combine_d:    0,
        gate_kind:    gate_kind::ORDER_BAND,
        gate_a:       band,
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
/// `OP_ADD`. Otherwise emits one AccumulatorOp per Add delta with an OrderBand
/// equal to that cell's local Add sequence index (legacy traversal order).
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

    let mut next_band: HashMap<(u32, u32), u32> = HashMap::new();
    let mut ops = Vec::new();
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
            debug_assert_eq!(delta.op_kind, OP_ADD);

            let cell = (slot as u32, delta.col);
            let band = *next_band.get(&cell).unwrap_or(&0);
            next_band.insert(cell, band + 1);
            n_bands = n_bands.max(band + 1);

            ops.push(make_add_op(cell.0, cell.1, delta.value, band));
        }
    }

    OverlayAddPlan::AllAdd { ops, n_bands }
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
    fn c3_same_cell_add_assigns_increasing_order_bands() {
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

        let OverlayAddPlan::AllAdd { ops, n_bands } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert_eq!(ops.len(), 3);
        assert_eq!(ops[0].gate_a, 0);
        assert_eq!(ops[1].gate_a, 1);
        assert_eq!(ops[2].gate_a, 2);
        assert_eq!(n_bands, 3);
        assert_eq!(ops[0].source_slot, 1e20_f32.to_bits());
        assert_eq!(ops[1].source_slot, 1.0_f32.to_bits());
        assert_eq!(ops[2].source_slot, (-1e20_f32).to_bits());
    }

    #[test]
    fn c3_different_cells_share_same_band() {
        let deltas = vec![
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 1.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 1,
                op_kind: OP_ADD,
                value: 2.0,
                _pad: 0,
            },
        ];
        let ranges = slot0_range(2);

        let OverlayAddPlan::AllAdd { ops, n_bands } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert_eq!(ops.len(), 2);
        assert_eq!(ops[0].gate_a, 0);
        assert_eq!(ops[1].gate_a, 0);
        assert_eq!(n_bands, 1);
    }

    #[test]
    fn c3_second_add_same_cell_only_advances_that_cell() {
        let deltas = vec![
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 1.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 1,
                op_kind: OP_ADD,
                value: 2.0,
                _pad: 0,
            },
            OverlayDelta {
                col: 0,
                op_kind: OP_ADD,
                value: 3.0,
                _pad: 0,
            },
        ];
        let ranges = slot0_range(3);

        let OverlayAddPlan::AllAdd { ops, n_bands } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert_eq!(ops.len(), 3);
        assert_eq!(ops[0].gate_a, 0);
        assert_eq!(ops[0].target0_col, 0);
        assert_eq!(ops[1].gate_a, 0);
        assert_eq!(ops[1].target0_col, 1);
        assert_eq!(ops[2].gate_a, 1);
        assert_eq!(ops[2].target0_col, 0);
        assert_eq!(n_bands, 2);
    }

    #[test]
    fn c3_add_only_batch_emits_one_op_per_add_delta() {
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

        let OverlayAddPlan::AllAdd { ops, n_bands } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert_eq!(ops.len(), 3);
        assert_eq!(n_bands, 2);
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
        let OverlayAddPlan::AllAdd { ops, n_bands } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert!(ops.is_empty());
        assert_eq!(n_bands, 0);
    }

    #[test]
    fn c3_delta_value_in_source_slot_field() {
        let deltas = vec![OverlayDelta {
            col: 2,
            op_kind: OP_ADD,
            value: 7.5,
            _pad: 0,
        }];
        let ranges = slot0_range(1);
        let OverlayAddPlan::AllAdd { ops, n_bands } =
            plan_overlay_add_accumulator(&deltas, &ranges, 1)
        else {
            panic!("expected AllAdd");
        };
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].source_slot, 7.5_f32.to_bits());
        assert_eq!(ops[0].target0_col, 2);
        assert_eq!(n_bands, 1);
    }
}
