//! Encode CPU-side [`AccumulatorOp`] registrations into GPU layout structs.

use simthing_core::{
    AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec,
};

use super::types::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum EncodeError {
    #[error("unsupported AccumulatorOp variant for Pass B bootstrap: {0}")]
    Unsupported(&'static str),
    #[error("AccumulatorOp failed CPU validation: {0}")]
    Validation(#[from] simthing_core::AccumulatorOpError),
}

impl AccumulatorOpGpu {
    pub fn from_op(op: &AccumulatorOp) -> Result<Self, EncodeError> {
        op.validate()?;

        let (source_kind, source_slot, source_col, source_count) = encode_source(op)?;
        let (combine_kind, combine_a, combine_b, combine_c, combine_d) = encode_combine(op)?;
        let (gate_kind, gate_a, gate_b) = encode_gate(&op.gate)?;
        let (scale_kind, scale_a) = encode_scale(&op.scale)?;
        let consume = encode_consume(op.consume);
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
        other => Err(EncodeError::Unsupported(other_name_gate(other))),
    }
}

fn encode_scale(scale: &ScaleSpec) -> Result<(u32, u32), EncodeError> {
    match scale {
        ScaleSpec::Identity => Ok((scale_kind::IDENTITY, 0)),
        ScaleSpec::Constant(value) => Ok((scale_kind::IDENTITY, value.to_bits())),
        ScaleSpec::ByColumn { .. } => Err(EncodeError::Unsupported("ScaleSpec::ByColumn")),
    }
}

fn encode_consume(consume: ConsumeMode) -> u32 {
    match consume {
        ConsumeMode::None => consume_kind::NONE,
        ConsumeMode::SubtractFromSource => consume_kind::SUBTRACT_FROM_SOURCE,
        _ => consume_kind::NONE,
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
