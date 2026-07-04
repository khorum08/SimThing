//! Op-set contention validation. Rejects patterns unsafe even with atomic
//! writes to the values buffer.
//!
//! `GateSpec::Threshold` ops never write to `values_buffer` and are exempt.

use std::collections::HashSet;

use super::types::{consume_kind, AccumulatorOpGpu};

/// Band value in [`BootstrapContention`] when the conflict involves `Always`.
pub const ALWAYS_BAND_SENTINEL: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GateScope {
    Always,
    OrderBand(u32),
    /// Threshold + EmitEvent ops do not write to `values_buffer`.
    Threshold,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BootstrapContention {
    pub band: u32,
    pub slot: u32,
    pub col: u32,
}

fn gate_scope(op: &AccumulatorOpGpu) -> GateScope {
    if op.gate_kind == super::types::gate_kind::THRESHOLD {
        GateScope::Threshold
    } else if op.gate_kind == super::types::gate_kind::ORDER_BAND {
        GateScope::OrderBand(op.gate_a)
    } else {
        GateScope::Always
    }
}

/// Validate that the op set does not contain patterns that are unsafe even
/// with atomic writes.
///
/// Multiple Identity/Sum writes to the same `(slot, col)` are allowed.
pub fn validate_no_contention(gpu_ops: &[AccumulatorOpGpu]) -> Result<(), BootstrapContention> {
    let mut always_consumes: HashSet<(u32, u32)> = HashSet::new();
    let mut band_consumes: HashSet<(u32, u32, u32)> = HashSet::new();

    for op in gpu_ops {
        let scope = gate_scope(op);
        if scope == GateScope::Threshold {
            continue;
        }

        if op.consume == consume_kind::SUBTRACT_FROM_SOURCE {
            let cell = (op.source_slot, op.source_col);
            match scope {
                GateScope::Threshold => {}
                GateScope::Always => {
                    if always_consumes.contains(&cell) {
                        return Err(BootstrapContention {
                            band: ALWAYS_BAND_SENTINEL,
                            slot: op.source_slot,
                            col: op.source_col,
                        });
                    }
                    always_consumes.insert(cell);
                }
                GateScope::OrderBand(band) => {
                    let key = (band, op.source_slot, op.source_col);
                    if band_consumes.contains(&key) {
                        return Err(BootstrapContention {
                            band,
                            slot: op.source_slot,
                            col: op.source_col,
                        });
                    }
                    band_consumes.insert(key);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::types::{
        combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
    };
    use super::*;

    fn identity_op(gate_kind_val: u32, gate_a: u32, target: (u32, u32)) -> AccumulatorOpGpu {
        AccumulatorOpGpu {
            source_kind: source_kind::CONSTANT,
            source_slot: 1f32.to_bits(),
            source_col: 0,
            source_count: 0,
            combine_kind: combine_kind::IDENTITY,
            combine_a: 0,
            combine_b: 0,
            combine_c: 0,
            combine_d: 0,
            gate_kind: gate_kind_val,
            gate_a,
            gate_b: 0,
            scale_kind: scale_kind::IDENTITY,
            scale_a: 0,
            consume: consume_kind::NONE,
            target0_slot: target.0,
            target0_col: target.1,
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

    fn subtract_op(
        gate_kind_val: u32,
        gate_a: u32,
        source: (u32, u32),
        target: (u32, u32),
    ) -> AccumulatorOpGpu {
        AccumulatorOpGpu {
            source_kind: source_kind::SLOT_VALUE,
            source_slot: source.0,
            source_col: source.1,
            source_count: 0,
            combine_kind: combine_kind::IDENTITY,
            combine_a: 0,
            combine_b: 0,
            combine_c: 0,
            combine_d: 0,
            gate_kind: gate_kind_val,
            gate_a,
            gate_b: 0,
            scale_kind: scale_kind::CONSTANT,
            scale_a: 1f32.to_bits(),
            consume: consume_kind::SUBTRACT_FROM_SOURCE,
            target0_slot: target.0,
            target0_col: target.1,
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

}
