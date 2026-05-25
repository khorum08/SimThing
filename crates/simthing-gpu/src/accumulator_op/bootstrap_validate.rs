//! B-2 bootstrap upload validation — rejects contended op sets.
//!
//! `GateSpec::Always` is treated as a wildcard over all bands because the WGSL
//! kernel executes Always ops on every `tick(band)` call.

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
    pub col:  u32,
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

/// Reject obviously unsafe bootstrap op sets. False positives are OK.
pub fn validate_bootstrap_no_contention(
    gpu_ops: &[AccumulatorOpGpu],
) -> Result<(), BootstrapContention> {
    let mut always_writes: HashSet<(u32, u32)> = HashSet::new();
    let mut always_consumes: HashSet<(u32, u32)> = HashSet::new();
    let mut band_writes: HashSet<(u32, u32, u32)> = HashSet::new();
    let mut band_consumes: HashSet<(u32, u32, u32)> = HashSet::new();
    let mut band_write_cells: HashSet<(u32, u32)> = HashSet::new();
    let mut band_consume_cells: HashSet<(u32, u32)> = HashSet::new();

    for op in gpu_ops {
        let scope = gate_scope(op);
        if scope == GateScope::Threshold {
            continue;
        }

        for (slot, col) in op_targets(op) {
            let cell = (slot, col);
            if let Some(err) = check_write(scope, cell, &always_writes, &always_consumes, &band_writes, &band_consumes, &band_write_cells, &band_consume_cells) {
                return Err(err);
            }
            record_write(
                scope,
                cell,
                &mut always_writes,
                &mut band_writes,
                &mut band_write_cells,
            );
        }

        if op.consume == consume_kind::SUBTRACT_FROM_SOURCE {
            let cell = (op.source_slot, op.source_col);
            if let Some(err) = check_consume(scope, cell, &always_writes, &always_consumes, &band_writes, &band_consumes, &band_write_cells, &band_consume_cells) {
                return Err(err);
            }
            record_consume(
                scope,
                cell,
                &mut always_consumes,
                &mut band_consumes,
                &mut band_consume_cells,
            );
        }
    }

    Ok(())
}

fn check_write(
    scope: GateScope,
    cell: (u32, u32),
    always_writes: &HashSet<(u32, u32)>,
    always_consumes: &HashSet<(u32, u32)>,
    band_writes: &HashSet<(u32, u32, u32)>,
    band_consumes: &HashSet<(u32, u32, u32)>,
    band_write_cells: &HashSet<(u32, u32)>,
    band_consume_cells: &HashSet<(u32, u32)>,
) -> Option<BootstrapContention> {
    let (slot, col) = cell;
    match scope {
        GateScope::Threshold => {}
        GateScope::Always => {
            if always_writes.contains(&cell)
                || always_consumes.contains(&cell)
                || band_write_cells.contains(&cell)
                || band_consume_cells.contains(&cell)
            {
                return Some(BootstrapContention {
                    band: ALWAYS_BAND_SENTINEL,
                    slot,
                    col,
                });
            }
        }
        GateScope::OrderBand(band) => {
            if always_writes.contains(&cell)
                || always_consumes.contains(&cell)
                || band_writes.contains(&(band, slot, col))
                || band_consumes.contains(&(band, slot, col))
            {
                return Some(BootstrapContention {
                    band,
                    slot,
                    col,
                });
            }
        }
    }
    None
}

fn check_consume(
    scope: GateScope,
    cell: (u32, u32),
    always_writes: &HashSet<(u32, u32)>,
    always_consumes: &HashSet<(u32, u32)>,
    band_writes: &HashSet<(u32, u32, u32)>,
    band_consumes: &HashSet<(u32, u32, u32)>,
    band_write_cells: &HashSet<(u32, u32)>,
    band_consume_cells: &HashSet<(u32, u32)>,
) -> Option<BootstrapContention> {
    let (slot, col) = cell;
    match scope {
        GateScope::Threshold => {}
        GateScope::Always => {
            if always_consumes.contains(&cell)
                || always_writes.contains(&cell)
                || band_consume_cells.contains(&cell)
                || band_write_cells.contains(&cell)
            {
                return Some(BootstrapContention {
                    band: ALWAYS_BAND_SENTINEL,
                    slot,
                    col,
                });
            }
        }
        GateScope::OrderBand(band) => {
            if always_consumes.contains(&cell)
                || always_writes.contains(&cell)
                || band_consumes.contains(&(band, slot, col))
                || band_writes.contains(&(band, slot, col))
            {
                return Some(BootstrapContention {
                    band,
                    slot,
                    col,
                });
            }
        }
    }
    None
}

fn record_write(
    scope: GateScope,
    cell: (u32, u32),
    always_writes: &mut HashSet<(u32, u32)>,
    band_writes: &mut HashSet<(u32, u32, u32)>,
    band_write_cells: &mut HashSet<(u32, u32)>,
) {
    match scope {
        GateScope::Threshold => {}
        GateScope::Always => {
            always_writes.insert(cell);
        }
        GateScope::OrderBand(band) => {
            let (slot, col) = cell;
            band_writes.insert((band, slot, col));
            band_write_cells.insert(cell);
        }
    }
}

fn record_consume(
    scope: GateScope,
    cell: (u32, u32),
    always_consumes: &mut HashSet<(u32, u32)>,
    band_consumes: &mut HashSet<(u32, u32, u32)>,
    band_consume_cells: &mut HashSet<(u32, u32)>,
) {
    match scope {
        GateScope::Threshold => {}
        GateScope::Always => {
            always_consumes.insert(cell);
        }
        GateScope::OrderBand(band) => {
            let (slot, col) = cell;
            band_consumes.insert((band, slot, col));
            band_consume_cells.insert(cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::{
        combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
    };

    fn identity_op(gate_kind: u32, gate_a: u32, target: (u32, u32)) -> AccumulatorOpGpu {
        AccumulatorOpGpu {
            source_kind:  source_kind::CONSTANT,
            source_slot:  1f32.to_bits(),
            source_col:   0,
            source_count: 0,
            combine_kind: combine_kind::IDENTITY,
            combine_a:    0,
            combine_b:    0,
            combine_c:    0,
            combine_d:    0,
            gate_kind,
            gate_a,
            gate_b:       0,
            scale_kind:   scale_kind::IDENTITY,
            scale_a:      0,
            consume:      consume_kind::NONE,
            target0_slot: target.0,
            target0_col:  target.1,
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

    #[test]
    fn rejects_always_and_orderband_same_target() {
        let ops = vec![
            identity_op(gate_kind::ALWAYS, 0, (1, 0)),
            identity_op(gate_kind::ORDER_BAND, 1, (1, 0)),
        ];
        let err = validate_bootstrap_no_contention(&ops).unwrap_err();
        assert_eq!(err.slot, 1);
        assert_eq!(err.col, 0);
    }

    #[test]
    fn allows_same_target_in_different_order_bands() {
        let ops = vec![
            identity_op(gate_kind::ORDER_BAND, 0, (1, 0)),
            identity_op(gate_kind::ORDER_BAND, 1, (1, 0)),
        ];
        validate_bootstrap_no_contention(&ops).unwrap();
    }
}
