//! CPU reference executor for Pass B ops (B-2 parity tests).

use simthing_core::{eml_opcode, EmlNodeGpu, EmlResourceClass};

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
    let mut depth = 0u32;
    let mut peak_stack = 0u32;
    for node in nodes {
        match node.opcode {
            eml_opcode::LITERAL_F32 | eml_opcode::SLOT_VALUE | eml_opcode::PARAM => depth += 1,
            eml_opcode::ADD
            | eml_opcode::SUB
            | eml_opcode::MUL
            | eml_opcode::DIV
            | eml_opcode::MIN
            | eml_opcode::MAX
            | eml_opcode::CMP_LT
            | eml_opcode::CMP_LE
            | eml_opcode::CMP_GT
            | eml_opcode::CMP_GE
            | eml_opcode::CMP_EQ => depth = depth.saturating_sub(1),
            eml_opcode::SELECT => depth = depth.saturating_sub(2),
            _ => {}
        }
        peak_stack = peak_stack.max(depth);
    }
    let resource_class = EmlResourceClass::smallest_fitting(nodes.len() as u32, peak_stack)
        .expect("CPU oracle requires registry-admitted EML");
    let slots = resource_class.stack_slots() as usize;
    let mut stack = vec![0.0f32; slots];
    let mut mul_a = vec![0.0f32; slots];
    let mut mul_b = vec![0.0f32; slots];
    let mut is_mul = vec![false; slots];
    let mut sp: usize = 0;

    let push = |stack: &mut [f32],
                _mul_a: &mut [f32],
                _mul_b: &mut [f32],
                is_mul: &mut [bool],
                sp: &mut usize,
                value: f32| {
        stack[*sp] = value;
        is_mul[*sp] = false;
        *sp += 1;
    };
    let clear_top_mul = |is_mul: &mut [bool], sp: usize| {
        if sp > 0 {
            is_mul[sp - 1] = false;
        }
    };

    for node in nodes {
        match node.opcode {
            eml_opcode::LITERAL_F32 => {
                push(&mut stack, &mut mul_a, &mut mul_b, &mut is_mul, &mut sp, f32::from_bits(node.a));
            }
            eml_opcode::SLOT_VALUE => {
                let i = idx(eval_slot, node.a, n_dims);
                push(&mut stack, &mut mul_a, &mut mul_b, &mut is_mul, &mut sp, values[i]);
            }
            eml_opcode::PARAM => {
                push(
                    &mut stack,
                    &mut mul_a,
                    &mut mul_b,
                    &mut is_mul,
                    &mut sp,
                    params[node.a as usize],
                );
            }
            eml_opcode::ADD | eml_opcode::SUB => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                let rhs_mul = is_mul[sp - 1].then_some((mul_a[sp - 1], mul_b[sp - 1]));
                let lhs_mul = is_mul[sp - 2].then_some((mul_a[sp - 2], mul_b[sp - 2]));
                stack[sp - 2] = crate::eml_uniqueness::uniqueness_add_sub(
                    node.opcode == eml_opcode::SUB,
                    lhs,
                    rhs,
                    lhs_mul,
                    rhs_mul,
                );
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::MUL => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs * rhs;
                mul_a[sp - 2] = lhs;
                mul_b[sp - 2] = rhs;
                is_mul[sp - 2] = true;
                sp -= 1;
            }
            eml_opcode::NEG => {
                stack[sp - 1] = -stack[sp - 1];
                clear_top_mul(&mut is_mul, sp);
            }
            eml_opcode::DIV => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs / rhs;
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::MIN => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs.min(rhs);
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::MAX => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs.max(rhs);
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::CLAMP_BOUNDED => {
                let v = stack[sp - 1];
                stack[sp - 1] = v.clamp(f32::from_bits(node.a), f32::from_bits(node.b));
                clear_top_mul(&mut is_mul, sp);
            }
            eml_opcode::CLAMP_FLOORED => {
                let v = stack[sp - 1];
                stack[sp - 1] = v.max(f32::from_bits(node.a));
                clear_top_mul(&mut is_mul, sp);
            }
            eml_opcode::ABS => {
                stack[sp - 1] = stack[sp - 1].abs();
                clear_top_mul(&mut is_mul, sp);
            }
            eml_opcode::FLOOR => {
                stack[sp - 1] = stack[sp - 1].floor();
                clear_top_mul(&mut is_mul, sp);
            }
            eml_opcode::EXP => {
                stack[sp - 1] = simthing_core::eml_exp_pinned_f32(stack[sp - 1]);
                clear_top_mul(&mut is_mul, sp);
            }
            eml_opcode::LN => {
                stack[sp - 1] = simthing_core::eml_ln::eml_ln_pinned_f32(stack[sp - 1]);
                clear_top_mul(&mut is_mul, sp);
            }
            eml_opcode::CMP_LT => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs < rhs { 1.0 } else { 0.0 };
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::CMP_LE => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs <= rhs { 1.0 } else { 0.0 };
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::CMP_GT => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs > rhs { 1.0 } else { 0.0 };
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::CMP_GE => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs >= rhs { 1.0 } else { 0.0 };
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::CMP_EQ => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if lhs == rhs { 1.0 } else { 0.0 };
                is_mul[sp - 2] = false;
                sp -= 1;
            }
            eml_opcode::SELECT => {
                let f_val = stack[sp - 1];
                let t_val = stack[sp - 2];
                let cond = stack[sp - 3] != 0.0;
                stack[sp - 3] = if cond { t_val } else { f_val };
                is_mul[sp - 3] = false;
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
}
