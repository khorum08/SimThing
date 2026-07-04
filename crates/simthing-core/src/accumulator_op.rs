//! AccumulatorOp v2 — the unified GPU gather/combine/gate/scatter primitive.
//!
//! These are the CPU-side type definitions. The GPU-layout counterparts
//! (`AccumulatorOpGpu`, `InputSpecGpu`) live in `simthing-gpu` and must
//! be kept byte-for-byte in sync with the WGSL structs in
//! `accumulator_op.wgsl`.
//!
//! See `docs/adr_accumulator_op_v2.md` and `docs/design_v7.md` for the
//! full specification.
//!
//! Raw integer targets are uncompilable:
//!
//! ```compile_fail
//! use simthing_core::{AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
//!
//! fn accumulator_op_rejects_raw_integer_target_slot_compile_fail() {
//!     let _ = AccumulatorOp {
//!         source: SourceSpec::Constant(1.0),
//!         combine: CombineFn::Identity,
//!         gate: GateSpec::Always,
//!         scale: ScaleSpec::Identity,
//!         consume: ConsumeMode::None,
//!         targets: vec![(0u32, ColumnIndex::new(0))],
//!     };
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::{ColumnIndex, SlotIndex};

// ── Source ────────────────────────────────────────────────────────────────────

/// How an AccumulatorOp reads its input value(s).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SourceSpec {
    /// A literal constant baked into the registration.
    Constant(f32),
    /// Read from a single (slot, col) location in the values buffer.
    SlotValue { slot: SlotIndex, col: ColumnIndex },
    /// Read from a contiguous range of slots; same column in each.
    /// Used for reductions (Sum, Mean, Max, Min, WeightedMean).
    SlotRange {
        start: SlotIndex,
        count: u32,
        col: ColumnIndex,
    },
    /// Read from up to 4 explicit (slot, col, unit_cost) inputs.
    /// Used for conjunctive production recipes.
    ConjunctiveCrossing { inputs: Vec<InputSpec> },
}

/// One input channel for a ConjunctiveCrossing or multi-target source.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputSpec {
    pub slot: SlotIndex,
    pub col: ColumnIndex,
    /// Cost per unit for emission calculations; 1.0 for non-emission uses.
    pub unit_cost: f32,
}

// ── CombineFn ─────────────────────────────────────────────────────────────────

/// How N gathered inputs collapse to a single write value.
///
/// # Semantic classes
///
/// **Exact** (conservation-critical, never soft): `Identity`, `Sum`, `Max`,
/// `Min`, `IntegrateWithClamp`, `CrossingFormula`, `MinAcrossInputs`,
/// `Product`, `LastByPriority`.
///
/// **Soft aggregate** (tolerance policy applies — see ADR §Semantic scope):
/// `Mean`, `WeightedMean`. These are GPU-to-GPU deterministic but ~3e-6 off
/// the CPU oracle. Must not drive hard structural transitions without a
/// `SoftAggregateGuard`.
///
/// **EML** (whitelist required): `EvalEML`. Only for formulas with no
/// transcendentals, ≤16 nodes, and an entry in `EmlExpressionRegistry`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CombineFn {
    /// Pass the source value through unchanged.
    Identity,
    /// Sum all inputs. Used for Sum reductions and accumulating transfers.
    Sum,
    /// Arithmetic mean of all inputs. **Soft aggregate.**
    Mean,
    /// Maximum of all inputs.
    Max,
    /// Minimum of all inputs.
    Min,
    /// Weighted mean: `Σ(value × weight) / Σ(weight)`.
    /// `weight_col` is the column to read for weights on each input slot.
    /// **Soft aggregate.**
    WeightedMean { weight_col: ColumnIndex },
    /// Product of all inputs. Used for Multiply overlays.
    Product,
    /// Select the highest-priority input value. Used for Set overlays.
    /// Priority is encoded in `InputSpec::unit_cost` (higher = higher priority).
    LastByPriority,
    /// Integrate velocity into amount with clamping.
    /// Used for velocity integration migration.
    IntegrateWithClamp {
        dt: f32,
        vel_max: f32,
        amount_min: f32,
        amount_max: f32,
    },
    /// Debt-band emission formula: `floor((queued_count * unit_cost + value) / unit_cost)`.
    CrossingFormula { unit_cost: f32 },
    /// Conjunctive production: `floor(min(input_i / unit_cost_i))`.
    /// Emits one unit when all channels have accumulated enough.
    MinAcrossInputs,
    /// Evaluate a designer EML expression tree on GPU.
    /// Requires a whitelist entry in `EmlExpressionRegistry`.
    /// Only valid for formulas with no transcendentals and ≤16 nodes.
    EvalEML { tree_id: u32 },
}

impl CombineFn {
    /// Returns true if this combine function is a soft aggregate (tolerant to
    /// ~3e-6 error vs CPU oracle). Soft aggregates must not drive hard
    /// structural transitions without a `SoftAggregateGuard`.
    pub fn is_soft_aggregate(&self) -> bool {
        matches!(self, CombineFn::Mean | CombineFn::WeightedMean { .. })
    }
}

// ── GateSpec ──────────────────────────────────────────────────────────────────

/// When the AccumulatorOp write fires.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GateSpec {
    /// Always fires every tick.
    Always,
    /// Fires when the source column crosses `value` in `direction`.
    Threshold {
        value: f32,
        direction: ThresholdDirection,
    },
    /// Fires only when the overlay associated with this registration is active.
    LifecycleActive,
    /// Fires only when the target slot has been marked dirty this tick.
    DirtyOnly,
    /// Fires only when the dispatcher is processing this band number.
    /// Band 0 = first (e.g. Add overlays, basic threshold).
    /// Band 1 = second (e.g. Multiply/Set overlays).
    /// Depth-bucket order for reductions uses ascending bands from leaf → root.
    OrderBand(u32),
}

/// Direction of threshold crossing that triggers a `GateSpec::Threshold`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThresholdDirection {
    /// Fires when value crosses upward (value >= threshold having been below).
    Upward,
    /// Fires when value crosses downward (value <= threshold having been above).
    Downward,
    /// Fires on either upward or downward strict crossing.
    Either,
}

// ── ConsumeMode ───────────────────────────────────────────────────────────────

/// What happens during and after the target write.
///
/// `None`, `ResetTarget`, `ScaleTarget`, and `AddToTarget` form the
/// write-to-target axis. `SubtractFromSource`, `SubtractFromAllInputs`, and
/// `EmitEvent` are source/event side effects.
///
/// `SubtractFromSource` and `SubtractFromAllInputs` are the only mechanisms
/// for resource transfer. `TransformOp::Add` on two separate slots for the
/// same logical transfer is a violation of the AccumulatorOp invariant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumeMode {
    /// Nothing happens to the source.
    None,
    /// Subtract the written value from the source slot/col.
    /// Used for single-channel transfer and debt-band emission.
    SubtractFromSource,
    /// Subtract `emit_count × unit_cost_i` from each input in a
    /// `ConjunctiveCrossing`. Used for multi-channel production recipes.
    SubtractFromAllInputs,
    /// Overwrite the target slot/col with the computed value (rather than
    /// adding). Used for Set overlays and clamp operations.
    ResetTarget,
    /// Multiply the target slot/col by the computed value. Used for
    /// Multiply overlays when the target is being scaled in place.
    ScaleTarget,
    /// Write a compact `EmissionRecord` to the GPU emission buffer and
    /// increment the atomic emission counter. Used for threshold-gated events.
    EmitEvent,
    /// Add the computed value to the target slot/col. Used for Add overlays.
    AddToTarget,
}

// ── ScaleSpec ─────────────────────────────────────────────────────────────────

/// How the combined value is scaled before writing.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScaleSpec {
    /// Write the value as-is.
    Identity,
    /// Multiply by a constant scale factor.
    Constant(f32),
    /// Multiply by the value in a secondary column (for weighted operations).
    ByColumn { col: ColumnIndex },
}

// ── SoftAggregateGuard ────────────────────────────────────────────────────────

/// Guard policy for sub-fields using soft-aggregate combine functions.
///
/// Required on any `SubFieldSpec` whose `reduction_override` is
/// `WeightedMean` or `Mean` and that feeds a threshold registration.
/// Validated at registration time by `assert_no_hard_trigger_on_soft_aggregate`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SoftAggregateGuard {
    /// No guard — field is explicitly approved as unguarded. Only valid
    /// for fields that never feed threshold registrations.
    Unguarded,
    /// Quantize the value to the nearest `step` before using as a threshold
    /// input. Eliminates sub-step drift from triggering re-evaluation.
    Quantized { step: f32 },
    /// Apply a hysteresis band of `band` around the last committed value.
    /// The threshold fires only when the value exits the band.
    Hysteresis { band: f32 },
}

// ── AccumulatorOp ─────────────────────────────────────────────────────────────

/// The unified CPU-side description of one AccumulatorOp registration.
///
/// Each instance describes one GPU write: gather from `source`, collapse via
/// `combine`, check `gate`, scale by `scale`, write to `targets`, optionally
/// consuming from the source via `consume`.
///
/// The GPU-layout counterpart is `AccumulatorOpGpu` in `simthing-gpu`.
/// The two types must represent the same operation. Use
/// `AccumulatorOpGpu::from_op(&op, slot_resolver)` to convert.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccumulatorOp {
    pub source: SourceSpec,
    pub combine: CombineFn,
    pub gate: GateSpec,
    pub scale: ScaleSpec,
    pub consume: ConsumeMode,
    /// Write targets: up to 4 (slot, col) pairs. Must have at least one.
    pub targets: Vec<(SlotIndex, ColumnIndex)>,
}

impl AccumulatorOp {
    /// Validate the registration for internal consistency.
    /// Called at registration time before the op is uploaded to the GPU.
    pub fn validate(&self) -> Result<(), AccumulatorOpError> {
        let emit_event_no_targets = matches!(self.consume, ConsumeMode::EmitEvent)
            && matches!(
                &self.gate,
                GateSpec::Threshold { .. } | GateSpec::OrderBand(_)
            );
        if self.targets.is_empty() && !emit_event_no_targets {
            return Err(AccumulatorOpError::NoTargets);
        }
        if self.targets.len() > 4 {
            return Err(AccumulatorOpError::TooManyTargets(self.targets.len()));
        }
        if let CombineFn::WeightedMean { .. } = &self.combine {
            if !matches!(&self.source, SourceSpec::SlotRange { .. }) {
                return Err(AccumulatorOpError::WeightedMeanRequiresSlotRange);
            }
        }
        if let SourceSpec::ConjunctiveCrossing { inputs } = &self.source {
            if inputs.is_empty() {
                return Err(AccumulatorOpError::EmptyConjunctiveInputs);
            }
        }
        if let SourceSpec::SlotRange { count, .. } = &self.source {
            if *count == 0 {
                return Err(AccumulatorOpError::EmptySlotRange);
            }
        }
        if let CombineFn::EvalEML { tree_id } = &self.combine {
            if *tree_id == u32::MAX {
                return Err(AccumulatorOpError::InvalidEmlTreeId(*tree_id));
            }
        }
        if self.consume == ConsumeMode::SubtractFromAllInputs
            && !matches!(&self.source, SourceSpec::ConjunctiveCrossing { .. })
        {
            return Err(AccumulatorOpError::SubtractAllRequiresConjunctive);
        }
        Ok(())
    }
}

/// Errors returned by `AccumulatorOp::validate()`.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum AccumulatorOpError {
    #[error("AccumulatorOp must have at least one target")]
    NoTargets,
    #[error("AccumulatorOp has {0} targets; maximum is 4")]
    TooManyTargets(usize),
    #[error("WeightedMean combine requires a SlotRange source")]
    WeightedMeanRequiresSlotRange,
    #[error("ConjunctiveCrossing source requires at least one input")]
    EmptyConjunctiveInputs,
    #[error("SlotRange source must have count > 0")]
    EmptySlotRange,
    #[error("EvalEML tree_id {0} is invalid")]
    InvalidEmlTreeId(u32),
    #[error("SubtractFromAllInputs requires a ConjunctiveCrossing source")]
    SubtractAllRequiresConjunctive,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_op() -> AccumulatorOp {
        AccumulatorOp {
            source: SourceSpec::Constant(1.0),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(SlotIndex::new(0), ColumnIndex::new(0))],
        }
    }

}
