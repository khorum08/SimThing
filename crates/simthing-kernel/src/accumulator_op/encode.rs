//! Encode CPU-side [`AccumulatorOp`] registrations into GPU layout structs.

use simthing_core::{
    eml_nodes::execution_class_to_u32, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode,
    EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlExecutionClass, EmlExpressionRegistry,
    EmlTreeId, GateSpec, ScaleSpec, SlotIndex, SourceSpec, ThresholdDirection,
};

use crate::registration::{
    ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, THRESH_BUF_OUTPUT,
    THRESH_BUF_VALUES,
};

use crate::world_state::IntentDelta;

use super::bootstrap_validate::{validate_no_contention, BootstrapContention};
use super::input_list_table::InputListRange;
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
    BootstrapContention { band: u32, slot: u32, col: u32 },
    #[error("duplicate folded intent cell: slot={slot}, col={col}")]
    DuplicateIntentCell { slot: u32, col: u32 },
    #[error("EML tree {tree_id:?} is not uploaded to the GPU program table")]
    EmlTreeNotUploaded { tree_id: EmlTreeId },
    #[error(
        "EML formula {tree_id:?} execution class {class:?} is not production-admissible in C-8a"
    )]
    EmlExecutionClassNotAdmissible {
        tree_id: EmlTreeId,
        class: EmlExecutionClass,
    },
    #[error("EML registry error: {0}")]
    EmlRegistry(#[from] simthing_core::EmlRegistryError),
}

impl From<BootstrapContention> for EncodeError {
    fn from(value: BootstrapContention) -> Self {
        Self::BootstrapContention {
            band: value.band,
            slot: value.slot,
            col: value.col,
        }
    }
}

impl AccumulatorOpGpu {
    pub fn from_op(op: &AccumulatorOp) -> Result<Self, EncodeError> {
        Self::from_op_with_eml(op, None)
    }

    pub fn from_op_with_eml(
        op: &AccumulatorOp,
        eml: Option<&EmlExpressionRegistry>,
    ) -> Result<Self, EncodeError> {
        op.validate()?;
        validate_bootstrap_op(op)?;

        let (source_kind, source_slot, source_col, source_count) = encode_source(op)?;
        let (combine_kind, combine_a, combine_b, combine_c, combine_d) = encode_combine(op, eml)?;
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
            target0_col: targets[0].1,
            target1_slot: targets[1].0,
            target1_col: targets[1].1,
            target2_slot: targets[2].0,
            target2_col: targets[2].1,
            target3_slot: targets[3].0,
            target3_col: targets[3].1,
            n_targets,
            _pad: 0,
        })
    }

    pub fn from_op_with_input_list(
        op: &AccumulatorOp,
        range: InputListRange,
    ) -> Result<Self, EncodeError> {
        let mut gpu = Self::from_op(op)?;
        gpu.source_kind = source_kind::INPUT_LIST;
        gpu.source_slot = range.offset;
        gpu.source_col = 0;
        gpu.source_count = range.count;
        Ok(gpu)
    }

    /// Encode and validate a full bootstrap op set, including same-band contention checks.
    pub fn encode_bootstrap_set(ops: &[AccumulatorOp]) -> Result<Vec<Self>, EncodeError> {
        Self::encode_bootstrap_set_with_eml(ops, None)
    }

    /// Encode bootstrap ops, resolving `EvalEML` tree IDs via the uploaded registry.
    pub fn encode_bootstrap_set_with_eml(
        ops: &[AccumulatorOp],
        eml: Option<&EmlExpressionRegistry>,
    ) -> Result<Vec<Self>, EncodeError> {
        let gpu_ops: Vec<Self> = ops
            .iter()
            .map(|op| Self::from_op_with_eml(op, eml))
            .collect::<Result<_, _>>()?;
        validate_no_contention(&gpu_ops)?;
        Ok(gpu_ops)
    }

    /// Encode folded intent deltas as affine GPU ops (C-2 intent migration path).
    pub fn encode_intent_deltas(deltas: &[IntentDelta]) -> Result<Vec<Self>, EncodeError> {
        validate_intent_deltas_no_duplicate_cells(deltas)?;
        Ok(deltas.iter().map(intent_delta_to_gpu).collect())
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

/// Reject duplicate `(slot, col)` rows — the CPU fold must collapse them first.
pub fn validate_intent_deltas_no_duplicate_cells(
    deltas: &[IntentDelta],
) -> Result<(), EncodeError> {
    let mut seen = std::collections::HashSet::new();
    for d in deltas {
        if !seen.insert((d.slot, d.col)) {
            return Err(EncodeError::DuplicateIntentCell {
                slot: d.slot,
                col: d.col,
            });
        }
    }
    Ok(())
}

fn intent_delta_to_gpu(delta: &IntentDelta) -> AccumulatorOpGpu {
    AccumulatorOpGpu {
        source_kind: source_kind::SLOT_VALUE,
        source_slot: delta.slot,
        source_col: delta.col,
        source_count: 0,
        combine_kind: combine_kind::AFFINE_INTENT,
        combine_a: delta.mul.to_bits(),
        combine_b: delta.add.to_bits(),
        combine_c: 0,
        combine_d: 0,
        gate_kind: gate_kind::ALWAYS,
        gate_a: 0,
        gate_b: 0,
        scale_kind: scale_kind::IDENTITY,
        scale_a: 0,
        consume: consume_kind::NONE,
        target0_slot: delta.slot,
        target0_col: delta.col,
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

/// Convert E-1 builder registrations into GPU threshold registrations.
///
/// Canonical bridge for both [`EmitOnThresholdBuffer::Values`] and
/// [`EmitOnThresholdBuffer::Output`]. Upload the result via
/// `AccumulatorOpSession::upload_threshold_ops` so `ThresholdRegistration.buffer`
/// is written into `AccumulatorOpGpu.source_count`.
pub fn emit_on_threshold_registrations_to_gpu(
    regs: &[EmitOnThresholdRegistration],
) -> Vec<ThresholdRegistration> {
    regs.iter()
        .map(|r| ThresholdRegistration {
            slot: r.slot.raw(),
            col: r.col.raw_u32(),
            threshold: r.threshold,
            direction: threshold_direction_to_u32(r.direction),
            event_kind: r.event_kind,
            buffer: match r.buffer {
                EmitOnThresholdBuffer::Values => THRESH_BUF_VALUES,
                EmitOnThresholdBuffer::Output => THRESH_BUF_OUTPUT,
            },
        })
        .collect()
}

/// Compile E-1 Values-buffer registrations into plain AccumulatorOps.
///
/// Plain [`AccumulatorOp`] values do not encode the threshold buffer selector.
/// Output-buffer registrations must use [`emit_on_threshold_registrations_to_gpu`]
/// and `upload_threshold_ops` instead.
pub fn emit_on_threshold_registrations_to_ops(
    regs: &[EmitOnThresholdRegistration],
) -> Result<(Vec<AccumulatorOp>, Vec<u32>), EncodeError> {
    if regs
        .iter()
        .any(|r| r.buffer == EmitOnThresholdBuffer::Output)
    {
        return Err(EncodeError::Unsupported(
            "EmitOnThreshold Output-buffer registrations must be uploaded through ThresholdRegistration / upload_threshold_ops so source_count preserves THRESH_BUF_OUTPUT",
        ));
    }
    let gpu_regs = emit_on_threshold_registrations_to_gpu(regs);
    threshold_registrations_to_ops(&gpu_regs)
}

/// Convert GPU threshold registrations into AccumulatorOp threshold scan ops.
pub fn threshold_registrations_to_ops(
    regs: &[ThresholdRegistration],
) -> Result<(Vec<AccumulatorOp>, Vec<u32>), EncodeError> {
    let mut ops = Vec::with_capacity(regs.len());
    let mut event_kinds = Vec::with_capacity(regs.len());
    for r in regs {
        debug_assert!(r.buffer == THRESH_BUF_VALUES || r.buffer == THRESH_BUF_OUTPUT);
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(r.slot),
                col: ColumnIndex::new(r.col as usize),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: r.threshold,
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
        (GateSpec::Threshold { .. }, ConsumeMode::None) => {
            if !matches!(
                op.source,
                SourceSpec::SlotValue { .. } | SourceSpec::Constant(_)
            ) {
                return Err(EncodeError::Unsupported(
                    "Threshold+None requires SlotValue or Constant source",
                ));
            }
            Ok(())
        }
        (GateSpec::Threshold { .. }, _) => Err(EncodeError::Unsupported(
            "Threshold gate with this consume mode is not yet implemented",
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
    } else if op.consume == ConsumeMode::SubtractFromAllInputs {
        match (&op.source, &op.combine) {
            (SourceSpec::ConjunctiveCrossing { inputs }, CombineFn::MinAcrossInputs) => {
                if inputs.is_empty() {
                    Err(EncodeError::Unsupported(
                        "SubtractFromAllInputs requires non-empty ConjunctiveCrossing",
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Err(EncodeError::Unsupported(
                "SubtractFromAllInputs requires ConjunctiveCrossing + MinAcrossInputs",
            )),
        }
    } else {
        Ok(())
    }
}

fn encode_source(op: &AccumulatorOp) -> Result<(u32, u32, u32, u32), EncodeError> {
    match &op.source {
        SourceSpec::Constant(value) => Ok((source_kind::CONSTANT, value.to_bits(), 0, 0)),
        SourceSpec::SlotValue { slot, col } => {
            Ok((source_kind::SLOT_VALUE, slot.raw(), col.raw_u32(), 0))
        }
        SourceSpec::SlotRange { start, count, col } => {
            Ok((source_kind::SLOT_RANGE, start.raw(), col.raw_u32(), *count))
        }
        SourceSpec::ConjunctiveCrossing { inputs } => {
            if inputs.is_empty() {
                return Err(EncodeError::Unsupported(
                    "ConjunctiveCrossing with zero inputs",
                ));
            }
            Ok((source_kind::INPUT_LIST, 0, 0, inputs.len() as u32))
        }
    }
}

fn encode_combine(
    op: &AccumulatorOp,
    eml: Option<&EmlExpressionRegistry>,
) -> Result<(u32, u32, u32, u32, u32), EncodeError> {
    match &op.combine {
        CombineFn::Identity => Ok((combine_kind::IDENTITY, 0, 0, 0, 0)),
        CombineFn::Sum => Ok((combine_kind::SUM, 0, 0, 0, 0)),
        CombineFn::Product => Ok((combine_kind::PRODUCT, 0, 0, 0, 0)),
        CombineFn::LastByPriority => Ok((combine_kind::LAST_BY_PRIORITY, 0, 0, 0, 0)),
        CombineFn::Mean => Ok((combine_kind::MEAN, 0, 0, 0, 0)),
        CombineFn::Max => Ok((combine_kind::MAX, 0, 0, 0, 0)),
        CombineFn::Min => Ok((combine_kind::MIN, 0, 0, 0, 0)),
        CombineFn::WeightedMean { weight_col } => {
            Ok((combine_kind::WEIGHTED_MEAN, weight_col.raw_u32(), 0, 0, 0))
        }
        CombineFn::IntegrateWithClamp {
            dt: _,
            vel_max,
            amount_min,
            amount_max,
        } => Ok((
            combine_kind::INTEGRATE_CLAMP,
            vel_max.to_bits(),
            amount_min.to_bits(),
            amount_max.to_bits(),
            0,
        )),
        CombineFn::CrossingFormula { unit_cost } => {
            Ok((combine_kind::CROSSING_FORMULA, unit_cost.to_bits(), 0, 0, 0))
        }
        CombineFn::MinAcrossInputs => Ok((combine_kind::MIN_ACROSS_INPUTS, 0, 0, 0, 0)),
        CombineFn::EvalEML { tree_id } => {
            let tree_id = EmlTreeId(*tree_id);
            let Some(registry) = eml else {
                return Err(EncodeError::EmlTreeNotUploaded { tree_id });
            };
            let meta = registry
                .get(tree_id)
                .ok_or(EncodeError::EmlTreeNotUploaded { tree_id })?;
            if meta.execution_class != EmlExecutionClass::ExactDeterministic {
                return Err(EncodeError::EmlExecutionClassNotAdmissible {
                    tree_id,
                    class: meta.execution_class,
                });
            }
            let range_index = registry
                .tree_range_index(tree_id)
                .ok_or(EncodeError::EmlTreeNotUploaded { tree_id })?;
            Ok((
                combine_kind::EVAL_EML,
                range_index,
                0,
                0,
                execution_class_to_u32(meta.execution_class),
            ))
        }
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

fn encode_consume(consume: ConsumeMode, _gate: &GateSpec) -> Result<u32, EncodeError> {
    match consume {
        ConsumeMode::None => Ok(consume_kind::NONE),
        ConsumeMode::SubtractFromSource => Ok(consume_kind::SUBTRACT_FROM_SOURCE),
        ConsumeMode::SubtractFromAllInputs => Ok(consume_kind::SUBTRACT_FROM_ALL_INPUTS),
        ConsumeMode::ResetTarget => Ok(consume_kind::RESET_TARGET),
        ConsumeMode::ScaleTarget => Ok(consume_kind::SCALE_TARGET),
        ConsumeMode::EmitEvent => Ok(consume_kind::EMIT_EVENT),
        ConsumeMode::AddToTarget => Ok(consume_kind::ADD_TO_TARGET),
    }
}

fn encode_targets(
    targets: &[(simthing_core::SlotIndex, simthing_core::ColumnIndex)],
) -> ([(u32, u32); 4], u32) {
    let mut out = [(0u32, 0u32); 4];
    for (idx, target) in targets.iter().take(4).enumerate() {
        out[idx] = (target.0.raw(), target.1.raw_u32());
    }
    (out, targets.len() as u32)
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
    use crate::registration::THRESH_BUF_OUTPUT;
    use simthing_core::{
        ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex, SourceSpec,
        ThresholdDirection,
    };

    #[test]
    fn c2_intent_delta_encodes_affine_params() {
        let delta = IntentDelta {
            slot: 3,
            col: 2,
            mul: 1.5,
            add: -0.25,
        };
        let gpu = AccumulatorOpGpu::encode_intent_deltas(std::slice::from_ref(&delta))
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(gpu.source_slot, 3);
        assert_eq!(gpu.source_col, 2);
        assert_eq!(gpu.combine_kind, combine_kind::AFFINE_INTENT);
        assert_eq!(gpu.combine_a, 1.5f32.to_bits());
        assert_eq!(gpu.combine_b, (-0.25f32).to_bits());
        assert_eq!(gpu.n_targets, 1);
        assert_eq!(gpu.target0_slot, 3);
        assert_eq!(gpu.target0_col, 2);
    }

    #[test]
    fn c2_empty_intent_set_encodes_to_empty() {
        assert!(AccumulatorOpGpu::encode_intent_deltas(&[])
            .unwrap()
            .is_empty());
    }

    #[test]
    fn encodes_transfer_op() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(1),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(2.0),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(SlotIndex::new(2), ColumnIndex::new(0))],
        };
        let gpu = AccumulatorOpGpu::from_op(&op).unwrap();
        assert_eq!(gpu.scale_kind, scale_kind::CONSTANT);
        assert_eq!(gpu.consume, consume_kind::SUBTRACT_FROM_SOURCE);
    }

    #[test]
    fn c1_threshold_gate_emit_event_validator_accepts() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
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
    fn threshold_with_none_consume_encodes_ok() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: 0.5,
                direction: ThresholdDirection::Upward,
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(SlotIndex::new(1), ColumnIndex::new(0))],
        };
        let result = AccumulatorOpGpu::from_op(&op);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn conjunctive_crossing_encodes_without_error() {
        use simthing_core::InputSpec;
        let op = AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![
                    InputSpec {
                        slot: SlotIndex::new(1),
                        col: ColumnIndex::new(0),
                        unit_cost: 5.0,
                    },
                    InputSpec {
                        slot: SlotIndex::new(1),
                        col: ColumnIndex::new(2),
                        unit_cost: 3.0,
                    },
                ],
            },
            combine: CombineFn::MinAcrossInputs,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::SubtractFromAllInputs,
            targets: vec![(SlotIndex::new(99), ColumnIndex::new(0))],
        };
        let result = AccumulatorOpGpu::from_op(&op);
        assert!(
            result.is_ok(),
            "ConjunctiveCrossing must encode: {result:?}"
        );
        let gpu = result.unwrap();
        assert_eq!(gpu.source_kind, source_kind::CONJUNCTIVE_CROSSING);
        assert_eq!(gpu.source_count, 2);
    }

    fn encode_combine_fn_only(
        combine: &CombineFn,
        eml: Option<&EmlExpressionRegistry>,
    ) -> Result<(u32, u32, u32, u32, u32), EncodeError> {
        let op = AccumulatorOp {
            source: SourceSpec::Constant(0.0),
            combine: combine.clone(),
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(SlotIndex::new(0), ColumnIndex::new(0))],
        };
        let _ = op.validate();
        encode_combine(&op, eml)
    }

    #[test]
    fn c1_threshold_output_buffer_registrations_encode() {
        let regs = [ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 0.5,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_OUTPUT,
        }];
        let (ops, kinds) = threshold_registrations_to_ops(&regs).unwrap();
        assert_eq!(ops.len(), 1);
        assert_eq!(kinds, vec![1]);
    }

    #[test]
    fn accumulator_op_gpu_encoding_preserved_after_index_newtypes() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: SlotIndex::new(4),
                count: 3,
                col: ColumnIndex::new(11),
            },
            combine: CombineFn::WeightedMean {
                weight_col: ColumnIndex::new(5),
            },
            gate: GateSpec::OrderBand(2),
            scale: ScaleSpec::Constant(1.25),
            consume: ConsumeMode::ResetTarget,
            targets: vec![
                (SlotIndex::new(7), ColumnIndex::new(11)),
                (SlotIndex::new(8), ColumnIndex::new(12)),
            ],
        };
        let gpu = AccumulatorOpGpu::from_op(&op).unwrap();
        assert_eq!(gpu.source_kind, source_kind::SLOT_RANGE);
        assert_eq!(gpu.source_slot, 4);
        assert_eq!(gpu.source_col, 11);
        assert_eq!(gpu.source_count, 3);
        assert_eq!(gpu.combine_kind, combine_kind::WEIGHTED_MEAN);
        assert_eq!(gpu.combine_a, 5);
        assert_eq!(gpu.gate_kind, gate_kind::ORDER_BAND);
        assert_eq!(gpu.gate_a, 2);
        assert_eq!(gpu.scale_kind, scale_kind::CONSTANT);
        assert_eq!(gpu.scale_a, 1.25f32.to_bits());
        assert_eq!(gpu.consume, consume_kind::RESET_TARGET);
        assert_eq!(gpu.n_targets, 2);
        assert_eq!(gpu.target0_slot, 7);
        assert_eq!(gpu.target0_col, 11);
        assert_eq!(gpu.target1_slot, 8);
        assert_eq!(gpu.target1_col, 12);
    }
}
