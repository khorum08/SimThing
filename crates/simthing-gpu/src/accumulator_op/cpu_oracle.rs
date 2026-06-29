//! CPU reference executor for Pass B ops (B-2 parity tests).

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlNodeGpu, GateSpec, ScaleSpec, SourceSpec,
    ThresholdDirection,
};

use crate::world_state::IntentDelta;

use super::types::{EmissionRecord, ThresholdEmission};

/// Test-only CPU mirror of the WGSL `eml_eval` stack machine (ExactDeterministic opcodes).
pub fn eval_eml_cpu(
    nodes: &[EmlNodeGpu],
    eval_slot: u32,
    values: &[f32],
    n_dims: u32,
    params: [f32; 4],
) -> f32 {
    let mut stack = [0.0f32; 32];
    let mut sp: usize = 0;

    for node in nodes {
        match node.opcode {
            eml_opcode::LITERAL_F32 => {
                stack[sp] = f32::from_bits(node.a);
                sp += 1;
            }
            eml_opcode::SLOT_VALUE => {
                let i = idx(eval_slot, node.a, n_dims);
                stack[sp] = values[i];
                sp += 1;
            }
            eml_opcode::PARAM => {
                stack[sp] = params[node.a as usize];
                sp += 1;
            }
            eml_opcode::ADD => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs + rhs;
                sp -= 1;
            }
            eml_opcode::SUB => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs - rhs;
                sp -= 1;
            }
            eml_opcode::MUL => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs * rhs;
                sp -= 1;
            }
            eml_opcode::NEG => {
                stack[sp - 1] = -stack[sp - 1];
            }
            eml_opcode::DIV => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs / rhs;
                sp -= 1;
            }
            eml_opcode::MIN => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs.min(rhs);
                sp -= 1;
            }
            eml_opcode::MAX => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs.max(rhs);
                sp -= 1;
            }
            eml_opcode::CLAMP_BOUNDED => {
                let v = stack[sp - 1];
                stack[sp - 1] = v.clamp(f32::from_bits(node.a), f32::from_bits(node.b));
            }
            eml_opcode::CLAMP_FLOORED => {
                let v = stack[sp - 1];
                stack[sp - 1] = v.max(f32::from_bits(node.a));
            }
            eml_opcode::ABS => {
                stack[sp - 1] = stack[sp - 1].abs();
            }
            eml_opcode::FLOOR => {
                stack[sp - 1] = stack[sp - 1].floor();
            }
            eml_opcode::CMP_LT => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs < rhs { 1.0 } else { 0.0 };
                sp -= 1;
            }
            eml_opcode::CMP_LE => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs <= rhs { 1.0 } else { 0.0 };
                sp -= 1;
            }
            eml_opcode::CMP_GT => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs > rhs { 1.0 } else { 0.0 };
                sp -= 1;
            }
            eml_opcode::CMP_GE => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs >= rhs { 1.0 } else { 0.0 };
                sp -= 1;
            }
            eml_opcode::CMP_EQ => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs == rhs { 1.0 } else { 0.0 };
                sp -= 1;
            }
            eml_opcode::SELECT => {
                let f_val = stack[sp - 1];
                let t_val = stack[sp - 2];
                let cond = stack[sp - 3] != 0.0;
                stack[sp - 3] = if cond { t_val } else { f_val };
                sp -= 2;
            }
            eml_opcode::RETURN_TOP => {
                return stack[sp - 1];
            }
            _ => return 0.0,
        }
    }
    stack[sp - 1]
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum CpuOracleError {
    #[error("unsupported op in CPU oracle: {0}")]
    Unsupported(&'static str),
}

/// Apply folded intent deltas on CPU (C-2 parity reference).
pub fn execute_intent_deltas_cpu(values: &mut [f32], deltas: &[IntentDelta], n_dims: u32) {
    for d in deltas {
        let i = idx(d.slot, d.col, n_dims);
        values[i] = values[i] * d.mul + d.add;
    }
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
        if matches!(
            (&op.gate, op.consume),
            (GateSpec::Threshold { .. }, ConsumeMode::EmitEvent)
        ) {
            continue;
        }
        let write_value = gather_and_combine(values, op, n_dims)?;
        let write_value = clamp_transfer(values, op, write_value, n_dims)?;
        let target_value = if matches!(op.combine, CombineFn::MinAcrossInputs) {
            apply_scale(write_value, &op.scale)
        } else {
            write_value
        };
        apply_targets(values, op, target_value, n_dims)?;
        apply_consume(values, op, write_value, n_dims)?;
        maybe_emit_event(op_idx, op, write_value, &mut records);
    }
    Ok(records)
}

/// Execute threshold-gated ops against previous/current value buffers.
pub fn execute_threshold_ops_cpu(
    previous_values: &[f32],
    values: &mut [f32],
    ops: &[AccumulatorOp],
    n_dims: u32,
) -> Result<Vec<ThresholdEmission>, CpuOracleError> {
    let mut records = Vec::new();

    for (op_idx, op) in ops.iter().enumerate() {
        let GateSpec::Threshold {
            value: threshold,
            direction,
        } = op.gate
        else {
            continue;
        };

        match op.consume {
            ConsumeMode::EmitEvent => {
                let SourceSpec::SlotValue { slot, col } = op.source else {
                    return Err(CpuOracleError::Unsupported(
                        "Threshold+EmitEvent requires SlotValue source",
                    ));
                };
                let prev = previous_values[idx(slot.raw(), col.raw_u32(), n_dims)];
                let curr = values[idx(slot.raw(), col.raw_u32(), n_dims)];
                if threshold_crossed_cpu(prev, curr, threshold, direction) {
                    records.push(ThresholdEmission {
                        reg_idx: op_idx as u32,
                        slot: slot.raw(),
                        col: col.raw_u32(),
                        value: curr,
                    });
                }
            }
            ConsumeMode::None => match &op.source {
                SourceSpec::SlotValue { slot, col } => {
                    let i = idx(slot.raw(), col.raw_u32(), n_dims);
                    let prev = previous_values[i];
                    let curr = values[i];
                    if threshold_crossed_cpu(prev, curr, threshold, direction) {
                        let write_value = gather_and_combine(values, op, n_dims)?;
                        apply_targets(values, op, write_value, n_dims)?;
                    }
                }
                SourceSpec::Constant(value) => {
                    let prev = *value;
                    let curr = *value;
                    if threshold_crossed_cpu(prev, curr, threshold, direction) {
                        let write_value = gather_and_combine(values, op, n_dims)?;
                        apply_targets(values, op, write_value, n_dims)?;
                    }
                }
                _ => {
                    return Err(CpuOracleError::Unsupported(
                        "Threshold+None requires SlotValue or Constant source",
                    ));
                }
            },
            _ => {
                return Err(CpuOracleError::Unsupported(
                    "Threshold gate with this consume mode is not implemented in CPU oracle",
                ));
            }
        }
    }

    Ok(records)
}

fn threshold_crossed_cpu(
    prev: f32,
    curr: f32,
    threshold: f32,
    direction: ThresholdDirection,
) -> bool {
    let up = prev <= threshold && curr > threshold;
    let down = prev >= threshold && curr < threshold;
    match direction {
        ThresholdDirection::Upward => up,
        ThresholdDirection::Downward => down,
        ThresholdDirection::Either => up || down,
    }
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    slot as usize * n_dims as usize + col as usize
}

fn gate_matches(gate: &GateSpec, band: u32) -> bool {
    match gate {
        GateSpec::Always | GateSpec::Threshold { .. } => true,
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

fn clamped_transfer(
    values: &[f32],
    op: &AccumulatorOp,
    n_dims: u32,
) -> Result<f32, CpuOracleError> {
    let SourceSpec::SlotValue { slot, col } = op.source else {
        return Err(CpuOracleError::Unsupported("transfer without SlotValue"));
    };
    let ScaleSpec::Constant(requested) = op.scale else {
        return Err(CpuOracleError::Unsupported(
            "transfer without Constant scale",
        ));
    };
    let available = values[idx(slot.raw(), col.raw_u32(), n_dims)];
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
        let available = values[idx(slot.raw(), col.raw_u32(), n_dims)];
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
            SourceSpec::SlotRange { start, count, col } => {
                let mut sum = 0.0f32;
                for offset in 0..*count {
                    sum += values[idx(start.saturating_add(offset).raw(), col.raw_u32(), n_dims)];
                }
                Ok(sum)
            }
            SourceSpec::ConjunctiveCrossing { inputs } => {
                let mut sum = 0.0f32;
                for input in inputs {
                    sum += values[idx(input.slot.raw(), input.col.raw_u32(), n_dims)];
                }
                Ok(sum)
            }
            _ => Err(CpuOracleError::Unsupported(
                "Sum without SlotRange or InputList",
            )),
        },
        CombineFn::MinAcrossInputs => {
            let SourceSpec::ConjunctiveCrossing { inputs } = &op.source else {
                return Err(CpuOracleError::Unsupported(
                    "MinAcrossInputs without ConjunctiveCrossing",
                ));
            };
            let mut amount = f32::MAX;
            for input in inputs {
                if input.unit_cost <= 0.0 {
                    return Ok(0.0);
                }
                let available = values[idx(input.slot.raw(), input.col.raw_u32(), n_dims)];
                amount = amount.min(available / input.unit_cost);
            }
            if inputs.is_empty() {
                Ok(0.0)
            } else {
                Ok(amount.max(0.0).floor())
            }
        }
        _ => Err(CpuOracleError::Unsupported("combine")),
    }
}

fn read_source(values: &[f32], source: &SourceSpec, n_dims: u32) -> Result<f32, CpuOracleError> {
    match source {
        SourceSpec::Constant(value) => Ok(*value),
        SourceSpec::SlotValue { slot, col } => Ok(values[idx(slot.raw(), col.raw_u32(), n_dims)]),
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
        let i = idx(slot.raw(), col.raw_u32(), n_dims);
        match op.consume {
            ConsumeMode::AddToTarget => values[i] += write_value,
            ConsumeMode::ScaleTarget => values[i] *= write_value,
            ConsumeMode::ResetTarget => values[i] = write_value,
            _ => match op.combine {
                CombineFn::Identity | CombineFn::Sum | CombineFn::MinAcrossInputs => {
                    if matches!(
                        op.consume,
                        ConsumeMode::SubtractFromSource | ConsumeMode::SubtractFromAllInputs
                    ) {
                        values[i] += write_value;
                    } else {
                        values[i] = write_value;
                    }
                }
                _ => return Err(CpuOracleError::Unsupported("combine target write")),
            },
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
        ConsumeMode::None
        | ConsumeMode::EmitEvent
        | ConsumeMode::AddToTarget
        | ConsumeMode::ScaleTarget
        | ConsumeMode::ResetTarget => Ok(()),
        ConsumeMode::SubtractFromSource => match op.source {
            SourceSpec::SlotValue { slot, col } => {
                values[idx(slot.raw(), col.raw_u32(), n_dims)] -= write_value;
                Ok(())
            }
            _ => Err(CpuOracleError::Unsupported(
                "SubtractFromSource without SlotValue",
            )),
        },
        ConsumeMode::SubtractFromAllInputs => {
            let SourceSpec::ConjunctiveCrossing { inputs } = &op.source else {
                return Err(CpuOracleError::Unsupported(
                    "SubtractFromAllInputs without ConjunctiveCrossing",
                ));
            };
            let unit_count = write_value;
            for input in inputs {
                let i = idx(input.slot.raw(), input.col.raw_u32(), n_dims);
                values[i] = (values[i] - unit_count * input.unit_cost).max(0.0);
            }
            Ok(())
        }
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
            reg_idx: op_idx as u32,
            emit_count,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex, SourceSpec,
        ThresholdDirection,
    };

    #[test]
    fn c2_intent_affine_cpu_oracle() {
        let mut values = vec![10.0, 7.0];
        let deltas = [IntentDelta {
            slot: 0,
            col: 0,
            mul: 2.0,
            add: 3.0,
        }];
        execute_intent_deltas_cpu(&mut values, &deltas, 1);
        assert_eq!(values[0], 23.0);
    }

    #[test]
    fn accumulator_scale_constant_zero_writes_zero() {
        let mut values = vec![10.0, 7.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(0.0),
            consume: ConsumeMode::None,
            targets: vec![(SlotIndex::new(1), ColumnIndex::new(0))],
        };
        execute_ops_cpu(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(values[1], 0.0);
    }

    #[test]
    fn accumulator_transfer_clamps_to_available_source() {
        let mut values = vec![5.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(10.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(SlotIndex::new(1), ColumnIndex::new(0))],
        };
        execute_ops_cpu(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(values[1], 5.0);
        assert_eq!(values[0], 0.0);

        let mut values = vec![10.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(3.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(SlotIndex::new(1), ColumnIndex::new(0))],
        };
        execute_ops_cpu(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(values[1], 3.0);
        assert_eq!(values[0], 7.0);
    }

    #[test]
    fn accumulator_transfer_rejects_negative_requested_transfer() {
        let mut values = vec![5.0, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(-3.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(SlotIndex::new(1), ColumnIndex::new(0))],
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
            targets: vec![(SlotIndex::new(0), ColumnIndex::new(0))],
        };
        let records =
            execute_ops_cpu_with_emissions(&mut values, std::slice::from_ref(&op), 0, 1).unwrap();
        assert_eq!(
            records,
            vec![EmissionRecord {
                reg_idx: 0,
                emit_count: 3
            }]
        );
        assert_eq!(values[0], 3.7);
    }

    #[test]
    fn threshold_none_cpu_writes_target_on_crossing() {
        let previous = vec![0.2, 0.0];
        let mut values = vec![0.5, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: 0.3,
                direction: ThresholdDirection::Upward,
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(SlotIndex::new(1), ColumnIndex::new(0))],
        };
        let records =
            execute_threshold_ops_cpu(&previous, &mut values, std::slice::from_ref(&op), 1)
                .unwrap();
        assert!(records.is_empty());
        assert!((values[1] - 0.5).abs() < 1e-5, "target: {}", values[1]);
    }

    #[test]
    fn threshold_none_cpu_no_write_when_not_crossing() {
        let previous = vec![0.4, 0.0];
        let mut values = vec![0.5, 0.0];
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: 0.3,
                direction: ThresholdDirection::Upward,
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(SlotIndex::new(1), ColumnIndex::new(0))],
        };
        let records =
            execute_threshold_ops_cpu(&previous, &mut values, std::slice::from_ref(&op), 1)
                .unwrap();
        assert!(records.is_empty());
        assert!(
            (values[1] - 0.0).abs() < 1e-5,
            "should not write: {}",
            values[1]
        );
    }
}
