//! CPU reference executor for Pass B ops (B-2 parity tests).

use simthing_core::{eml_opcode, EmlNodeGpu};

pub use crate::cpu_oracle::{
    execute_ops_cpu, execute_ops_cpu_with_emissions, execute_threshold_ops_cpu, CpuOracleError,
};

use crate::world_state::IntentDelta;

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

/// Apply folded intent deltas on CPU (C-2 parity reference).
pub fn execute_intent_deltas_cpu(values: &mut [f32], deltas: &[IntentDelta], n_dims: u32) {
    for d in deltas {
        let i = idx(d.slot, d.col, n_dims);
        values[i] = values[i] * d.mul + d.add;
    }
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    slot as usize * n_dims as usize + col as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex,
        SourceSpec, ThresholdDirection,
    };

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
