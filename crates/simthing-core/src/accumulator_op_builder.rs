//! First-class builders for AccumulatorOp registrations (E-1 and successors).
//!
//! These compile designer/spec intent into flat [`AccumulatorOp`] registrations
//! for upload by `simthing-sim` without semantic branching at runtime.

use serde::{Deserialize, Serialize};

use crate::{
    AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec, ThresholdDirection,
};

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
