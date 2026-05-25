//! CPU reference executor for Pass B ops (B-2 parity tests).

use simthing_core::{
    AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec,
};

use super::types::EmissionRecord;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum CpuOracleError {
    #[error("unsupported op in CPU oracle: {0}")]
    Unsupported(&'static str),
}

/// Execute one band of AccumulatorOp registrations against a flat values buffer.
pub fn execute_ops_cpu(
    values: &mut [f32],
    ops: &[AccumulatorOp],
    band: u32,
    n_dims: u32,
) -> Result<(), CpuOracleError> {
    execute_ops_cpu_with_emissions(values, ops, band, n_dims).map(|_| ())
}

/// Execute one band and collect compact emission records (B-2 EmitEvent parity).
pub fn execute_ops_cpu_with_emissions(
    values: &mut [f32],
    ops: &[AccumulatorOp],
    band: u32,
    n_dims: u32,
) -> Result<Vec<EmissionRecord>, CpuOracleError> {
    let mut records = Vec::new();
    for (op_idx, op) in ops.iter().enumerate() {
        if !gate_matches(&op.gate, band) {
            continue;
        }
        let write_value = gather_and_combine(values, op, n_dims)?;
        let write_value = clamp_transfer(values, op, write_value, n_dims)?;
        apply_targets(values, op, write_value, n_dims)?;
        apply_consume(values, op, write_value, n_dims)?;
        maybe_emit_event(op_idx, op, write_value, &mut records);
    }
    Ok(records)
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    slot as usize * n_dims as usize + col as usize
}

fn gate_matches(gate: &GateSpec, band: u32) -> bool {
    match gate {
        GateSpec::Always => true,
        GateSpec::OrderBand(current) => *current == band,
        _ => false,
    }
}

fn apply_scale(value: f32, scale: &ScaleSpec) -> f32 {
    match scale {
        ScaleSpec::Identity => value,
        ScaleSpec::Constant(factor) => value * factor,
        ScaleSpec::ByColumn { .. } => value,
    }
}

fn clamped_transfer(values: &[f32], op: &AccumulatorOp, n_dims: u32) -> Result<f32, CpuOracleError> {
    let SourceSpec::SlotValue { slot, col } = op.source else {
        return Err(CpuOracleError::Unsupported("transfer without SlotValue"));
    };
    let ScaleSpec::Constant(requested) = op.scale else {
        return Err(CpuOracleError::Unsupported("transfer without Constant scale"));
    };
    let available = values[idx(slot, col, n_dims)];
    Ok(requested.max(0.0).min(available.max(0.0)))
}

fn clamp_transfer(
    values: &[f32],
    op: &AccumulatorOp,
    write_value: f32,
    n_dims: u32,
) -> Result<f32, CpuOracleError> {
    if op.consume == ConsumeMode::SubtractFromSource {
        let SourceSpec::SlotValue { slot, col } = op.source else {
            return Ok(write_value);
        };
        let available = values[idx(slot, col, n_dims)];
        Ok(write_value.max(0.0).min(available.max(0.0)))
    } else {
        Ok(write_value)
    }
}

fn gather_and_combine(
    values: &[f32],
    op: &AccumulatorOp,
    n_dims: u32,
) -> Result<f32, CpuOracleError> {
    match &op.combine {
        CombineFn::Identity => {
            if op.consume == ConsumeMode::SubtractFromSource {
                return clamped_transfer(values, op, n_dims);
            }
            let raw = read_source(values, &op.source, n_dims)?;
            Ok(apply_scale(raw, &op.scale))
        }
        CombineFn::Sum => match &op.source {
            SourceSpec::SlotRange { start, count } => {
                let col = op.targets.first().map(|(_, c)| *c).ok_or(
                    CpuOracleError::Unsupported("Sum without target col"),
                )?;
                let mut sum = 0.0f32;
                for offset in 0..*count {
                    sum += values[idx(start + offset, col, n_dims)];
                }
                Ok(sum)
            }
            _ => Err(CpuOracleError::Unsupported("Sum without SlotRange")),
        },
        _ => Err(CpuOracleError::Unsupported("combine")),
    }
}

fn read_source(
    values: &[f32],
    source: &SourceSpec,
    n_dims: u32,
) -> Result<f32, CpuOracleError> {
    match source {
        SourceSpec::Constant(value) => Ok(*value),
        SourceSpec::SlotValue { slot, col } => Ok(values[idx(*slot, *col, n_dims)]),
        SourceSpec::SlotRange { .. } | SourceSpec::ConjunctiveCrossing { .. } => {
            Err(CpuOracleError::Unsupported("source for Identity"))
        }
    }
}

fn apply_targets(
    values: &mut [f32],
    op: &AccumulatorOp,
    write_value: f32,
    n_dims: u32,
) -> Result<(), CpuOracleError> {
    for (slot, col) in &op.targets {
        let i = idx(*slot, *col, n_dims);
        match op.combine {
            CombineFn::Identity => values[i] += write_value,
            CombineFn::Sum => values[i] = write_value,
            _ => return Err(CpuOracleError::Unsupported("combine target write")),
        }
    }
    Ok(())
}

fn apply_consume(
    values: &mut [f32],
    op: &AccumulatorOp,
    write_value: f32,
    n_dims: u32,
) -> Result<(), CpuOracleError> {
    match op.consume {
        ConsumeMode::None | ConsumeMode::EmitEvent => Ok(()),
        ConsumeMode::SubtractFromSource => match op.source {
            SourceSpec::SlotValue { slot, col } => {
                values[idx(slot, col, n_dims)] -= write_value;
                Ok(())
            }
            _ => Err(CpuOracleError::Unsupported(
                "SubtractFromSource without SlotValue",
            )),
        },
        _ => Err(CpuOracleError::Unsupported("consume")),
    }
}

fn maybe_emit_event(
    op_idx: usize,
    op: &AccumulatorOp,
    write_value: f32,
    records: &mut Vec<EmissionRecord>,
) {
    if op.consume != ConsumeMode::EmitEvent {
        return;
    }
    let emit_count = write_value.max(0.0).floor() as u32;
    if emit_count > 0 {
        records.push(EmissionRecord {
            reg_idx:    op_idx as u32,
            emit_count,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};

    #[test]
    fn accumulator_scale_constant_zero_writes_zero() {
        let mut values = vec![10.0, 7.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(0.0),
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };
        execute_ops_cpu(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(values[1], 7.0);
    }

    #[test]
    fn accumulator_transfer_clamps_to_available_source() {
        let mut values = vec![5.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(10.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(1, 0)],
        };
        execute_ops_cpu(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(values[1], 5.0);
        assert_eq!(values[0], 0.0);

        let mut values = vec![10.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(3.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(1, 0)],
        };
        execute_ops_cpu(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(values[1], 3.0);
        assert_eq!(values[0], 7.0);
    }

    #[test]
    fn accumulator_transfer_rejects_negative_requested_transfer() {
        let mut values = vec![5.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(-3.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(1, 0)],
        };
        execute_ops_cpu(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(values[1], 0.0);
        assert_eq!(values[0], 5.0);
    }

    #[test]
    fn b2_emit_event_writes_compact_record_cpu() {
        let mut values = vec![0.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::Constant(3.7),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::EmitEvent,
            targets: vec![(0, 0)],
        };
        let records =
            execute_ops_cpu_with_emissions(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(records, vec![EmissionRecord { reg_idx: 0, emit_count: 3 }]);
        assert_eq!(values[0], 3.7);
    }
}
