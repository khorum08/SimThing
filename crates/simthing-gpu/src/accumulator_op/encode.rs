//! Encode CPU-side [`AccumulatorOp`] registrations into GPU layout structs.

use simthing_core::{
    AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec, ThresholdDirection,
};

use crate::world_state::{
    ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, THRESH_BUF_OUTPUT,
    THRESH_BUF_VALUES,
};

use super::bootstrap_validate::{validate_no_contention, BootstrapContention};
use super::types::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum EncodeError {
    #[error("unsupported AccumulatorOp variant for Pass B bootstrap: {0}")]
    Unsupported(&'static str),
    #[error("AccumulatorOp failed CPU validation: {0}")]
    Validation(#[from] simthing_core::AccumulatorOpError),
    #[error(
        "AccumulatorOp bootstrap contains same-band contended cell: band={band}, slot={slot}, col={col}"
    )]
    BootstrapContention {
        band: u32,
        slot: u32,
        col:  u32,
    },
}

impl From<BootstrapContention> for EncodeError {
    fn from(value: BootstrapContention) -> Self {
        Self::BootstrapContention {
            band: value.band,
            slot: value.slot,
            col:  value.col,
        }
    }
}

impl AccumulatorOpGpu {
    pub fn from_op(op: &AccumulatorOp) -> Result<Self, EncodeError> {
        op.validate()?;
        validate_bootstrap_op(op)?;

        let (source_kind, source_slot, source_col, source_count) = encode_source(op)?;
        let (combine_kind, combine_a, combine_b, combine_c, combine_d) = encode_combine(op)?;
        let (gate_kind, gate_a, gate_b) = encode_gate(&op.gate)?;
        let (scale_kind, scale_a) = encode_scale(&op.scale)?;
        let consume = encode_consume(op.consume, &op.gate)?;
        let (targets, n_targets) = encode_targets(&op.targets);

        Ok(Self {
            source_kind,
            source_slot,
            source_col,
            source_count,
            combine_kind,
            combine_a,
            combine_b,
            combine_c,
            combine_d,
            gate_kind,
            gate_a,
            gate_b,
            scale_kind,
            scale_a,
            consume,
            target0_slot: targets[0].0,
            target0_col:  targets[0].1,
            target1_slot: targets[1].0,
            target1_col:  targets[1].1,
            target2_slot: targets[2].0,
            target2_col:  targets[2].1,
            target3_slot: targets[3].0,
            target3_col:  targets[3].1,
            n_targets,
            _pad: 0,
        })
    }

    /// Encode and validate a full bootstrap op set, including same-band contention checks.
    pub fn encode_bootstrap_set(ops: &[AccumulatorOp]) -> Result<Vec<Self>, EncodeError> {
        let gpu_ops: Vec<Self> = ops.iter().map(Self::from_op).collect::<Result<_, _>>()?;
        validate_no_contention(&gpu_ops)?;
        Ok(gpu_ops)
    }

    /// Encode threshold-gated EmitEvent ops (C-1 Pass 7 migration path).
    pub fn encode_threshold_set(ops: &[AccumulatorOp]) -> Result<Vec<Self>, EncodeError> {
        for op in ops {
            validate_threshold_op(op)?;
        }
        let gpu_ops: Vec<Self> = ops.iter().map(Self::from_op).collect::<Result<_, _>>()?;
        validate_no_contention(&gpu_ops)?;
        Ok(gpu_ops)
    }
}

/// Convert GPU threshold registrations into AccumulatorOp threshold scan ops.
pub fn threshold_registrations_to_ops(
    regs: &[ThresholdRegistration],
) -> Result<(Vec<AccumulatorOp>, Vec<u32>), EncodeError> {
    let mut ops = Vec::with_capacity(regs.len());
    let mut event_kinds = Vec::with_capacity(regs.len());
    for r in regs {
        if r.buffer == THRESH_BUF_OUTPUT {
            return Err(EncodeError::Unsupported(
                "THRESH_BUF_OUTPUT blocked until C-5/C-6 reduction migration",
            ));
        }
        debug_assert_eq!(r.buffer, THRESH_BUF_VALUES);
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: r.slot,
                col:  r.col,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value:     r.threshold,
                direction: direction_u32_to_threshold_direction(r.direction),
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::EmitEvent,
            targets: vec![],
        });
        event_kinds.push(r.event_kind);
    }
    Ok((ops, event_kinds))
}

fn direction_u32_to_threshold_direction(d: u32) -> ThresholdDirection {
    match d {
        DIR_DOWNWARD => ThresholdDirection::Downward,
        DIR_EITHER => ThresholdDirection::Either,
        _ => ThresholdDirection::Upward,
    }
}

fn threshold_direction_to_u32(direction: ThresholdDirection) -> u32 {
    match direction {
        ThresholdDirection::Upward => DIR_UPWARD,
        ThresholdDirection::Downward => DIR_DOWNWARD,
        ThresholdDirection::Either => DIR_EITHER,
    }
}

fn validate_threshold_op(op: &AccumulatorOp) -> Result<(), EncodeError> {
    match (&op.gate, op.consume) {
        (GateSpec::Threshold { .. }, ConsumeMode::EmitEvent) => {
            if !matches!(op.source, SourceSpec::SlotValue { .. }) {
                return Err(EncodeError::Unsupported(
                    "Threshold EmitEvent requires SlotValue source",
                ));
            }
            if op.combine != CombineFn::Identity {
                return Err(EncodeError::Unsupported(
                    "Threshold EmitEvent requires Identity combine",
                ));
            }
            if op.scale != ScaleSpec::Identity {
                return Err(EncodeError::Unsupported(
                    "Threshold EmitEvent requires Identity scale",
                ));
            }
            Ok(())
        }
        (GateSpec::Threshold { .. }, _) => Err(EncodeError::Unsupported(
            "Threshold gate requires EmitEvent consume until later phases",
        )),
        _ => Err(EncodeError::Unsupported("not a threshold op")),
    }
}

fn validate_bootstrap_op(op: &AccumulatorOp) -> Result<(), EncodeError> {
    if matches!(&op.gate, GateSpec::Threshold { .. }) {
        return validate_threshold_op(op);
    }
    if op.consume == ConsumeMode::SubtractFromSource {
        match (&op.source, &op.scale) {
            (SourceSpec::SlotValue { .. }, ScaleSpec::Constant(_)) => Ok(()),
            _ => Err(EncodeError::Unsupported(
                "SubtractFromSource requires SlotValue source and Constant scale",
            )),
        }
    } else {
        Ok(())
    }
}

fn encode_source(op: &AccumulatorOp) -> Result<(u32, u32, u32, u32), EncodeError> {
    match &op.source {
        SourceSpec::Constant(value) => Ok((
            source_kind::CONSTANT,
            value.to_bits(),
            0,
            0,
        )),
        SourceSpec::SlotValue { slot, col } => Ok((
            source_kind::SLOT_VALUE,
            *slot,
            *col,
            0,
        )),
        SourceSpec::SlotRange { start, count } => {
            let col = op
                .targets
                .first()
                .map(|(_, col)| *col)
                .ok_or(EncodeError::Unsupported("SlotRange without target col"))?;
            Ok((source_kind::SLOT_RANGE, *start, col, *count))
        }
        SourceSpec::ConjunctiveCrossing { .. } => {
            Err(EncodeError::Unsupported("ConjunctiveCrossing"))
        }
    }
}

fn encode_combine(op: &AccumulatorOp) -> Result<(u32, u32, u32, u32, u32), EncodeError> {
    match &op.combine {
        CombineFn::Identity => Ok((combine_kind::IDENTITY, 0, 0, 0, 0)),
        CombineFn::Sum => Ok((combine_kind::SUM, 0, 0, 0, 0)),
        other => Err(EncodeError::Unsupported(other_name(other))),
    }
}

fn encode_gate(gate: &GateSpec) -> Result<(u32, u32, u32), EncodeError> {
    match gate {
        GateSpec::Always => Ok((gate_kind::ALWAYS, 0, 0)),
        GateSpec::OrderBand(band) => Ok((gate_kind::ORDER_BAND, *band, 0)),
        GateSpec::Threshold { value, direction } => Ok((
            gate_kind::THRESHOLD,
            threshold_direction_to_u32(*direction),
            value.to_bits(),
        )),
        other => Err(EncodeError::Unsupported(other_name_gate(other))),
    }
}

fn encode_scale(scale: &ScaleSpec) -> Result<(u32, u32), EncodeError> {
    match scale {
        ScaleSpec::Identity => Ok((scale_kind::IDENTITY, 0)),
        ScaleSpec::Constant(value) => Ok((scale_kind::CONSTANT, value.to_bits())),
        ScaleSpec::ByColumn { .. } => Err(EncodeError::Unsupported("ScaleSpec::ByColumn")),
    }
}

fn encode_consume(consume: ConsumeMode, gate: &GateSpec) -> Result<u32, EncodeError> {
    match consume {
        ConsumeMode::None => {
            if matches!(gate, GateSpec::Threshold { .. }) {
                Err(EncodeError::Unsupported(
                    "Threshold gate requires EmitEvent consume until later phases",
                ))
            } else {
                Ok(consume_kind::NONE)
            }
        }
        ConsumeMode::SubtractFromSource => Ok(consume_kind::SUBTRACT_FROM_SOURCE),
        ConsumeMode::EmitEvent => Ok(consume_kind::EMIT_EVENT),
        ConsumeMode::SubtractFromAllInputs => {
            Err(EncodeError::Unsupported("SubtractFromAllInputs"))
        }
        ConsumeMode::ResetTarget => Err(EncodeError::Unsupported("ResetTarget")),
        ConsumeMode::ScaleTarget => Err(EncodeError::Unsupported("ScaleTarget")),
    }
}

fn encode_targets(targets: &[(u32, u32)]) -> ([(u32, u32); 4], u32) {
    let mut out = [(0u32, 0u32); 4];
    for (idx, target) in targets.iter().take(4).enumerate() {
        out[idx] = *target;
    }
    (out, targets.len() as u32)
}

fn other_name(combine: &CombineFn) -> &'static str {
    match combine {
        CombineFn::Mean => "Mean",
        CombineFn::Max => "Max",
        CombineFn::Min => "Min",
        CombineFn::WeightedMean { .. } => "WeightedMean",
        CombineFn::Product => "Product",
        CombineFn::LastByPriority => "LastByPriority",
        CombineFn::IntegrateWithClamp { .. } => "IntegrateWithClamp",
        CombineFn::CrossingFormula { .. } => "CrossingFormula",
        CombineFn::MinAcrossInputs => "MinAcrossInputs",
        CombineFn::EvalEML { .. } => "EvalEML",
        CombineFn::Identity | CombineFn::Sum => "Identity/Sum",
    }
}

fn other_name_gate(gate: &GateSpec) -> &'static str {
    match gate {
        GateSpec::Threshold { .. } => "Threshold",
        GateSpec::LifecycleActive => "LifecycleActive",
        GateSpec::DirtyOnly => "DirtyOnly",
        GateSpec::Always | GateSpec::OrderBand(_) => "Always/OrderBand",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};

    #[test]
    fn encodes_transfer_op() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 1, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(2.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(2, 0)],
        };
        let gpu = AccumulatorOpGpu::from_op(&op).unwrap();
        assert_eq!(gpu.scale_kind, scale_kind::CONSTANT);
        assert_eq!(gpu.consume, consume_kind::SUBTRACT_FROM_SOURCE);
    }

    #[test]
    fn c1_threshold_gate_emit_event_validator_accepts() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: 0.5,
                direction: ThresholdDirection::Upward,
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::EmitEvent,
            targets: vec![],
        };
        AccumulatorOpGpu::encode_threshold_set(std::slice::from_ref(&op)).unwrap();
    }

    #[test]
    fn c1_threshold_gate_non_emit_consume_validator_rejects() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 0 },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: 1.0,
                direction: ThresholdDirection::Upward,
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(1, 0)],
        };
        assert!(matches!(
            AccumulatorOpGpu::encode_threshold_set(std::slice::from_ref(&op)),
            Err(EncodeError::Unsupported(_))
        ));
    }

    #[test]
    fn c1_threshold_output_buffer_validator_rejects() {
        let regs = [ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 0.5,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_OUTPUT,
        }];
        assert!(matches!(
            threshold_registrations_to_ops(&regs),
            Err(EncodeError::Unsupported(_))
        ));
    }
}
