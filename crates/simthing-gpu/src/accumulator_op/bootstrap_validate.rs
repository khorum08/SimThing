//! B-1 bootstrap upload validation — rejects contended same-band op sets.

use std::collections::HashSet;

use super::types::{consume_kind, gate_kind, AccumulatorOpGpu};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BootstrapContention {
    pub band: u32,
    pub slot: u32,
    pub col:  u32,
}

pub fn op_band_key(op: &AccumulatorOpGpu) -> u32 {
    if op.gate_kind == gate_kind::ORDER_BAND {
        op.gate_a
    } else {
        0
    }
}

fn op_targets(op: &AccumulatorOpGpu) -> impl Iterator<Item = (u32, u32)> + '_ {
    [
        (op.target0_slot, op.target0_col),
        (op.target1_slot, op.target1_col),
        (op.target2_slot, op.target2_col),
        (op.target3_slot, op.target3_col),
    ]
    .into_iter()
    .take(op.n_targets as usize)
}

/// Reject obviously unsafe bootstrap op sets: duplicate same-band writes or
/// consumes, or write/consume aliasing within a band. False positives are OK.
pub fn validate_bootstrap_no_contention(
    gpu_ops: &[AccumulatorOpGpu],
) -> Result<(), BootstrapContention> {
    let mut writes: HashSet<(u32, u32, u32)> = HashSet::new();
    let mut consumes: HashSet<(u32, u32, u32)> = HashSet::new();

    for op in gpu_ops {
        let band = op_band_key(op);

        for (slot, col) in op_targets(op) {
            let write_key = (band, slot, col);
            if writes.contains(&write_key) {
                return Err(BootstrapContention { band, slot, col });
            }
            if consumes.contains(&write_key) {
                return Err(BootstrapContention { band, slot, col });
            }
            writes.insert(write_key);
        }

        if op.consume == consume_kind::SUBTRACT_FROM_SOURCE {
            let consume_key = (band, op.source_slot, op.source_col);
            if consumes.contains(&consume_key) {
                return Err(BootstrapContention {
                    band,
                    slot: op.source_slot,
                    col:  op.source_col,
                });
            }
            if writes.contains(&consume_key) {
                return Err(BootstrapContention {
                    band,
                    slot: op.source_slot,
                    col:  op.source_col,
                });
            }
            consumes.insert(consume_key);
        }
    }

    Ok(())
}
