//! First-class builders for AccumulatorOp registrations (E-1 and successors).
//!
//! These compile designer/spec intent into flat [`AccumulatorOp`] registrations
//! for upload by `simthing-sim` without semantic branching at runtime.

use serde::{Deserialize, Serialize};

use crate::{
    AccumulatorOp, CombineFn, ConsumeMode, GateSpec, InputSpec, ScaleSpec, SourceSpec,
    ThresholdDirection,
};

/// Errors returned by AccumulatorOp registration builders (E-2A, E-3, …).
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum AccumulatorOpBuilderError {
    #[error("discrete transfer amount must be finite")]
    NonFiniteAmount,
    #[error("discrete transfer amount must be >= 0")]
    NegativeAmount,
    #[error("discrete transfer source and target cells must differ")]
    SameSourceAndTarget,
    #[error("conjunctive recipe requires at least one input")]
    EmptyConjunctiveInputs,
    #[error("conjunctive recipe unit cost must be finite and > 0 at slot {slot} col {col}")]
    NonPositiveUnitCost { slot: u32, col: u32 },
    #[error("conjunctive recipe max_per_tick must be > 0")]
    InvalidMaxPerTick,
}

/// One input channel for a conjunctive production recipe (E-3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConjunctiveRecipeInput {
    pub slot: u32,
    pub col: u32,
    pub unit_cost: f32,
}

/// One conjunctive recipe registration compiled by the E-3 builder surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConjunctiveRecipeRegistration {
    pub inputs: Vec<ConjunctiveRecipeInput>,
    pub target_slot: u32,
    pub target_col: u32,
    /// Session/boundary throttle hint; the op uses C-8c Identity scaling today.
    /// Zero is rejected at registration time.
    pub max_per_tick: u32,
}

/// One exact discrete source-debit transfer registration (E-2A).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscreteTransferRegistration {
    pub source_slot: u32,
    pub source_col: u32,
    pub target_slot: u32,
    pub target_col: u32,
    pub amount: f32,
}

fn validate_discrete_transfer_amount(amount: f32) -> Result<(), AccumulatorOpBuilderError> {
    if !amount.is_finite() {
        return Err(AccumulatorOpBuilderError::NonFiniteAmount);
    }
    if amount < 0.0 {
        return Err(AccumulatorOpBuilderError::NegativeAmount);
    }
    Ok(())
}

fn validate_discrete_transfer_cells(
    source_slot: u32,
    source_col: u32,
    target_slot: u32,
    target_col: u32,
) -> Result<(), AccumulatorOpBuilderError> {
    if source_slot == target_slot && source_col == target_col {
        return Err(AccumulatorOpBuilderError::SameSourceAndTarget);
    }
    Ok(())
}

/// Which GPU buffer a threshold registration observes for crossing detection.
///
/// The buffer selector is preserved by the GPU bridge
/// (`emit_on_threshold_registrations_to_gpu`) and by `upload_threshold_ops`,
/// which writes it into `AccumulatorOpGpu.source_count`. Plain [`AccumulatorOp`]
/// values do not carry this selector.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmitOnThresholdBuffer {
    /// `values` / `previous_values` (default C-1 path).
    #[default]
    Values,
    /// Post-reduction `output_vectors` / `previous_output_vectors`.
    Output,
}

/// One threshold-emission registration compiled by the E-1 builder surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmitOnThresholdRegistration {
    pub slot: u32,
    pub col: u32,
    pub threshold: f32,
    pub direction: ThresholdDirection,
    pub event_kind: u32,
    #[serde(default)]
    pub buffer: EmitOnThresholdBuffer,
}

/// Canonical builder for AccumulatorOp registrations.
pub struct AccumulatorOpBuilder;

impl AccumulatorOpBuilder {
    /// Build a C-1 threshold + `EmitEvent` op over a single `(slot, col)` source.
    ///
    /// The op has empty targets; event identity is carried by the parallel
    /// `event_kind` list produced alongside ops at upload time.
    pub fn emit_on_threshold(
        source_slot: u32,
        source_col: u32,
        threshold: f32,
        direction: ThresholdDirection,
    ) -> AccumulatorOp {
        AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: source_slot,
                col: source_col,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Threshold {
                value: threshold,
                direction,
            },
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::EmitEvent,
            targets: vec![],
        }
    }

    /// Build an exact discrete source-debit transfer (E-2A / C-8c single-source path).
    ///
    /// `transfer_amount = min(max(amount, 0), max(source_value, 0))` at execution time;
    /// source is debited and target is credited by that amount.
    pub fn resource_transfer_discrete(
        source_slot: u32,
        source_col: u32,
        target_slot: u32,
        target_col: u32,
        amount: f32,
    ) -> Result<AccumulatorOp, AccumulatorOpBuilderError> {
        validate_discrete_transfer_amount(amount)?;
        validate_discrete_transfer_cells(source_slot, source_col, target_slot, target_col)?;
        Ok(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: source_slot,
                col: source_col,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(amount),
            consume: ConsumeMode::SubtractFromSource,
            targets: vec![(target_slot, target_col)],
        })
    }

    /// Build an exact conjunctive production recipe (E-3 / C-8c conjunctive path).
    ///
    /// Recipe count is `floor(min(input_i / unit_cost_i))` at execution time; all
    /// inputs are debited and the target is credited by that count (Identity scale).
    /// `max_per_tick` is stored on [`ConjunctiveRecipeRegistration`] for boundary
    /// throttling policy; the GPU op does not cap recipe count yet.
    pub fn conjunctive_recipe(
        inputs: &[(u32, u32, f32)],
        target_slot: u32,
        target_col: u32,
        max_per_tick: u32,
    ) -> Result<AccumulatorOp, AccumulatorOpBuilderError> {
        if inputs.is_empty() {
            return Err(AccumulatorOpBuilderError::EmptyConjunctiveInputs);
        }
        if max_per_tick == 0 {
            return Err(AccumulatorOpBuilderError::InvalidMaxPerTick);
        }
        let mut input_specs = Vec::with_capacity(inputs.len());
        for &(slot, col, unit_cost) in inputs {
            if !unit_cost.is_finite() || unit_cost <= 0.0 {
                return Err(AccumulatorOpBuilderError::NonPositiveUnitCost { slot, col });
            }
            input_specs.push(InputSpec {
                slot,
                col,
                unit_cost,
            });
        }
        Ok(AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: input_specs,
            },
            combine: CombineFn::MinAcrossInputs,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::SubtractFromAllInputs,
            targets: vec![(target_slot, target_col)],
        })
    }
}

/// Convenience alias for [`AccumulatorOpBuilder::emit_on_threshold`].
pub fn emit_on_threshold(
    source_slot: u32,
    source_col: u32,
    threshold: f32,
    direction: ThresholdDirection,
) -> AccumulatorOp {
    AccumulatorOpBuilder::emit_on_threshold(source_slot, source_col, threshold, direction)
}

/// Exact discrete source-debit transfer builder (E-2A).
pub fn try_resource_transfer_discrete(
    source_slot: u32,
    source_col: u32,
    target_slot: u32,
    target_col: u32,
    amount: f32,
) -> Result<AccumulatorOp, AccumulatorOpBuilderError> {
    AccumulatorOpBuilder::resource_transfer_discrete(
        source_slot,
        source_col,
        target_slot,
        target_col,
        amount,
    )
}

/// Exact discrete source-debit transfer builder (E-2A).
///
/// Prefer [`try_resource_transfer_discrete`] when handling invalid inputs.
pub fn resource_transfer_discrete(
    source_slot: u32,
    source_col: u32,
    target_slot: u32,
    target_col: u32,
    amount: f32,
) -> AccumulatorOp {
    try_resource_transfer_discrete(
        source_slot,
        source_col,
        target_slot,
        target_col,
        amount,
    )
    .expect("invalid discrete transfer registration")
}

/// Exact conjunctive recipe builder (E-3).
pub fn try_conjunctive_recipe(
    inputs: &[(u32, u32, f32)],
    target_slot: u32,
    target_col: u32,
    max_per_tick: u32,
) -> Result<AccumulatorOp, AccumulatorOpBuilderError> {
    AccumulatorOpBuilder::conjunctive_recipe(inputs, target_slot, target_col, max_per_tick)
}

/// Compile one conjunctive recipe registration into its AccumulatorOp shape.
pub fn conjunctive_recipe_registration_to_op(
    reg: &ConjunctiveRecipeRegistration,
) -> Result<AccumulatorOp, AccumulatorOpBuilderError> {
    if reg.max_per_tick == 0 {
        return Err(AccumulatorOpBuilderError::InvalidMaxPerTick);
    }
    let inputs: Vec<(u32, u32, f32)> = reg
        .inputs
        .iter()
        .map(|i| (i.slot, i.col, i.unit_cost))
        .collect();
    AccumulatorOpBuilder::conjunctive_recipe(
        &inputs,
        reg.target_slot,
        reg.target_col,
        reg.max_per_tick,
    )
}

/// Session-open / boundary refresh: compile conjunctive recipe registrations.
pub fn rebuild_conjunctive_recipe_ops(
    regs: &[ConjunctiveRecipeRegistration],
) -> Result<Vec<AccumulatorOp>, AccumulatorOpBuilderError> {
    regs.iter().map(conjunctive_recipe_registration_to_op).collect()
}

/// Compile one discrete transfer registration into its AccumulatorOp shape.
pub fn discrete_transfer_registration_to_op(
    reg: &DiscreteTransferRegistration,
) -> Result<AccumulatorOp, AccumulatorOpBuilderError> {
    AccumulatorOpBuilder::resource_transfer_discrete(
        reg.source_slot,
        reg.source_col,
        reg.target_slot,
        reg.target_col,
        reg.amount,
    )
}

/// Session-open / boundary refresh: compile discrete transfer registrations.
pub fn rebuild_discrete_transfer_ops(
    regs: &[DiscreteTransferRegistration],
) -> Result<Vec<AccumulatorOp>, AccumulatorOpBuilderError> {
    regs.iter().map(discrete_transfer_registration_to_op).collect()
}

/// Compile one registration into its AccumulatorOp shape.
///
/// Only the threshold gate fields are represented; [`EmitOnThresholdBuffer`] is
/// not encoded. For `Output`, use `emit_on_threshold_registrations_to_gpu` and
/// `AccumulatorOpSession::upload_threshold_ops`.
pub fn emit_on_threshold_registration_to_op(reg: &EmitOnThresholdRegistration) -> AccumulatorOp {
    AccumulatorOpBuilder::emit_on_threshold(reg.slot, reg.col, reg.threshold, reg.direction)
}

/// Session-open / boundary refresh: compile Values-buffer threshold-emission registrations.
///
/// Plain [`AccumulatorOp`] values do not encode the buffer selector. Registrations
/// with [`EmitOnThresholdBuffer::Output`] must be uploaded through
/// `emit_on_threshold_registrations_to_gpu` and `upload_threshold_ops`.
pub fn rebuild_emit_on_threshold_ops(regs: &[EmitOnThresholdRegistration]) -> Vec<AccumulatorOp> {
    regs.iter()
        .map(emit_on_threshold_registration_to_op)
        .collect()
}

/// Parallel `event_kind` list aligned with [`rebuild_emit_on_threshold_ops`].
pub fn rebuild_emit_on_threshold_event_kinds(regs: &[EmitOnThresholdRegistration]) -> Vec<u32> {
    regs.iter().map(|r| r.event_kind).collect()
}

/// Debt-band next downward threshold: `-((queued_count - 1) * unit_cost)`.
///
/// Used by boundary re-registration after discrete emission; see design v7 §5.2.
pub fn debt_band_next_threshold(queued_count: u32, unit_cost: f32) -> f32 {
    -((queued_count.saturating_sub(1)) as f32 * unit_cost)
}

/// Re-register one debt-band threshold after emission decrements `queued_count`.
pub fn refresh_emit_on_threshold_debt_band(
    reg: &EmitOnThresholdRegistration,
    new_queued_count: u32,
    unit_cost: f32,
) -> EmitOnThresholdRegistration {
    EmitOnThresholdRegistration {
        threshold: debt_band_next_threshold(new_queued_count, unit_cost),
        direction: ThresholdDirection::Downward,
        ..reg.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conjunctive_recipe_validates() {
        let op = try_conjunctive_recipe(&[(0, 0, 5.0), (0, 1, 3.0)], 0, 2, 4).unwrap();
        op.validate().expect("conjunctive recipe validates");
    }

    #[test]
    fn conjunctive_recipe_rejects_empty_inputs() {
        assert_eq!(
            try_conjunctive_recipe(&[], 0, 0, 1),
            Err(AccumulatorOpBuilderError::EmptyConjunctiveInputs)
        );
    }

    #[test]
    fn conjunctive_recipe_accepts_eight_inputs() {
        let inputs: Vec<(u32, u32, f32)> = (0..8).map(|c| (0, c, 1.0)).collect();
        let op = try_conjunctive_recipe(&inputs, 0, 8, 1).unwrap();
        op.validate().expect("N=8 conjunctive validates");
    }

    #[test]
    fn resource_transfer_discrete_validates() {
        let op = try_resource_transfer_discrete(0, 0, 0, 1, 5.0).unwrap();
        op.validate().expect("discrete transfer validates");
    }

    #[test]
    fn resource_transfer_discrete_rejects_negative_amount() {
        assert_eq!(
            try_resource_transfer_discrete(0, 0, 0, 1, -1.0),
            Err(AccumulatorOpBuilderError::NegativeAmount)
        );
    }

    #[test]
    fn resource_transfer_discrete_rejects_same_cell() {
        assert_eq!(
            try_resource_transfer_discrete(0, 0, 0, 0, 1.0),
            Err(AccumulatorOpBuilderError::SameSourceAndTarget)
        );
    }

    #[test]
    fn rebuild_discrete_transfer_ops_preserves_order() {
        let regs = [
            DiscreteTransferRegistration {
                source_slot: 0,
                source_col: 0,
                target_slot: 0,
                target_col: 1,
                amount: 3.0,
            },
            DiscreteTransferRegistration {
                source_slot: 1,
                source_col: 0,
                target_slot: 2,
                target_col: 0,
                amount: 7.0,
            },
        ];
        let ops = rebuild_discrete_transfer_ops(&regs).unwrap();
        assert_eq!(ops.len(), 2);
        assert_eq!(
            ops[0].scale,
            ScaleSpec::Constant(3.0),
            "order preserved"
        );
    }

    #[test]
    fn emit_on_threshold_validates() {
        let op = emit_on_threshold(0, 0, 0.5, ThresholdDirection::Upward);
        op.validate().expect("threshold emit validates");
    }

    #[test]
    fn rebuild_preserves_order() {
        let regs = [
            EmitOnThresholdRegistration {
                slot: 1,
                col: 0,
                threshold: 0.5,
                direction: ThresholdDirection::Upward,
                event_kind: 10,
                buffer: EmitOnThresholdBuffer::Values,
            },
            EmitOnThresholdRegistration {
                slot: 2,
                col: 1,
                threshold: -20.0,
                direction: ThresholdDirection::Downward,
                event_kind: 11,
                buffer: EmitOnThresholdBuffer::Values,
            },
        ];
        let ops = rebuild_emit_on_threshold_ops(&regs);
        let kinds = rebuild_emit_on_threshold_event_kinds(&regs);
        assert_eq!(ops.len(), 2);
        assert_eq!(kinds, vec![10, 11]);
    }

    #[test]
    fn debt_band_refresh_advances_threshold() {
        let reg = EmitOnThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: debt_band_next_threshold(10, 20.0),
            direction: ThresholdDirection::Downward,
            event_kind: 1,
            buffer: EmitOnThresholdBuffer::Values,
        };
        let next = refresh_emit_on_threshold_debt_band(&reg, 8, 20.0);
        assert_eq!(next.threshold, debt_band_next_threshold(8, 20.0));
        assert_eq!(next.event_kind, reg.event_kind);
    }
}
