//! C-8c transfer substrate planner → AccumulatorOp.

use simthing_core::{
    AccumulatorOp, CombineFn, ConsumeMode, EmlTreeId, GateSpec, InputSpec, ScaleSpec, SourceSpec,
};

use crate::{
    AccumulatorInputGpu, AccumulatorOpGpu, EncodeError, InputListRange,
};

#[derive(Clone, Debug, PartialEq)]
pub struct TransferInputRef {
    pub slot: u32,
    pub col: u32,
    pub unit_cost: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransferRegistration {
    pub inputs: Vec<TransferInputRef>,
    pub target_slot: u32,
    pub target_col: u32,
    pub output_scale: f32,
    /// Single-source fixed transfer cap (Identity + SubtractFromSource path).
    pub max_transfer: Option<f32>,
    pub tree_id: Option<EmlTreeId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransferPlan {
    pub ops: Vec<AccumulatorOp>,
    pub input_lists: Vec<Vec<AccumulatorInputGpu>>,
    pub n_bands: u32,
}

fn input_to_gpu(input: &TransferInputRef) -> AccumulatorInputGpu {
    AccumulatorInputGpu {
        slot: input.slot,
        col: input.col,
        unit_cost_bits: input.unit_cost.to_bits(),
        flags: 0,
    }
}

/// Build logical transfer ops and parallel input-list payloads.
pub fn plan_transfer_ops(registrations: &[TransferRegistration]) -> TransferPlan {
    let mut ops = Vec::with_capacity(registrations.len());
    let mut input_lists = Vec::with_capacity(registrations.len());
    for reg in registrations {
        assert!(!reg.inputs.is_empty(), "transfer registration needs inputs");
        if reg.inputs.len() == 1 && reg.max_transfer.is_some() {
            let inp = &reg.inputs[0];
            ops.push(AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: inp.slot,
                    col: inp.col,
                },
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Constant(reg.max_transfer.unwrap()),
                consume: ConsumeMode::SubtractFromSource,
                targets: vec![(reg.target_slot, reg.target_col)],
            });
            input_lists.push(Vec::new());
        } else {
            let inputs: Vec<InputSpec> = reg
                .inputs
                .iter()
                .map(|i| InputSpec {
                    slot: i.slot,
                    col: i.col,
                    unit_cost: i.unit_cost,
                })
                .collect();
            ops.push(AccumulatorOp {
                source: SourceSpec::ConjunctiveCrossing { inputs },
                combine: CombineFn::MinAcrossInputs,
                gate: GateSpec::OrderBand(0),
                scale: if reg.output_scale == 1.0 {
                    ScaleSpec::Identity
                } else {
                    ScaleSpec::Constant(reg.output_scale)
                },
                consume: ConsumeMode::SubtractFromAllInputs,
                targets: vec![(reg.target_slot, reg.target_col)],
            });
            input_lists.push(reg.inputs.iter().map(input_to_gpu).collect());
        }
    }
    TransferPlan {
        n_bands: if ops.is_empty() { 0 } else { 1 },
        ops,
        input_lists,
    }
}

/// Encode transfer ops after input-list ranges are resolved at boundary upload.
pub fn encode_transfer_plan(
    plan: &TransferPlan,
    ranges: &[InputListRange],
) -> Result<Vec<AccumulatorOpGpu>, EncodeError> {
    let mut range_idx = 0;
    let mut gpu_ops = Vec::with_capacity(plan.ops.len());
    for (op, list) in plan.ops.iter().zip(plan.input_lists.iter()) {
        let gpu = if list.is_empty() {
            AccumulatorOpGpu::from_op(op)?
        } else {
            let range = ranges.get(range_idx).ok_or(EncodeError::Unsupported(
                "missing input-list range for conjunctive transfer",
            ))?;
            range_idx += 1;
            AccumulatorOpGpu::from_op_with_input_list(op, *range)?
        };
        gpu_ops.push(gpu);
    }
    Ok(gpu_ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_single_source_fixed_transfer() {
        let regs = vec![TransferRegistration {
            inputs: vec![TransferInputRef {
                slot: 0,
                col: 0,
                unit_cost: 1.0,
            }],
            target_slot: 0,
            target_col: 1,
            output_scale: 1.0,
            max_transfer: Some(3.0),
            tree_id: None,
        }];
        let plan = plan_transfer_ops(&regs);
        assert_eq!(plan.ops.len(), 1);
        assert!(plan.input_lists[0].is_empty());
        assert!(matches!(plan.ops[0].consume, ConsumeMode::SubtractFromSource));
    }

    #[test]
    fn plan_conjunctive_transfer() {
        let regs = vec![TransferRegistration {
            inputs: vec![
                TransferInputRef {
                    slot: 0,
                    col: 0,
                    unit_cost: 5.0,
                },
                TransferInputRef {
                    slot: 0,
                    col: 1,
                    unit_cost: 3.0,
                },
            ],
            target_slot: 0,
            target_col: 2,
            output_scale: 1.0,
            max_transfer: None,
            tree_id: None,
        }];
        let plan = plan_transfer_ops(&regs);
        assert_eq!(plan.input_lists[0].len(), 2);
        assert!(matches!(plan.ops[0].combine, CombineFn::MinAcrossInputs));
    }
}
