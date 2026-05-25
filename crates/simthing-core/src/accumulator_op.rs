//! AccumulatorOp v2 — the unified GPU gather/combine/gate/scatter primitive.
//!
//! These are the CPU-side type definitions. The GPU-layout counterparts
//! (`AccumulatorOpGpu`, `InputSpecGpu`) live in `simthing-gpu` and must
//! be kept byte-for-byte in sync with the WGSL structs in
//! `accumulator_op.wgsl`.
//!
//! See `docs/adr_accumulator_op_v2.md` and `docs/design_v7.md` for the
//! full specification.

use serde::{Deserialize, Serialize};

// ── Source ────────────────────────────────────────────────────────────────────

/// How an AccumulatorOp reads its input value(s).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SourceSpec {
    /// A literal constant baked into the registration.
    Constant(f32),
    /// Read from a single (slot, col) location in the values buffer.
    SlotValue { slot: u32, col: u32 },
    /// Read from a contiguous range of slots; same column in each.
    /// Used for reductions (Sum, Mean, Max, Min, WeightedMean).
    SlotRange { start: u32, count: u32 },
    /// Read from up to 4 explicit (slot, col, unit_cost) inputs.
    /// Used for conjunctive production recipes.
    ConjunctiveCrossing { inputs: Vec<InputSpec> },
}

/// One input channel for a ConjunctiveCrossing or multi-target source.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputSpec {
    pub slot:      u32,
    pub col:       u32,
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
    WeightedMean { weight_col: u32 },
    /// Product of all inputs. Used for Multiply overlays.
    Product,
    /// Select the highest-priority input value. Used for Set overlays.
    /// Priority is encoded in `InputSpec::unit_cost` (higher = higher priority).
    LastByPriority,
    /// Integrate velocity into amount with clamping.
    /// Used for velocity integration migration.
    IntegrateWithClamp {
        dt:         f32,
        vel_max:    f32,
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
        value:     f32,
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

/// What happens to the source after the write.
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
    ByColumn { col: u32 },
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
    pub source:  SourceSpec,
    pub combine: CombineFn,
    pub gate:    GateSpec,
    pub scale:   ScaleSpec,
    pub consume: ConsumeMode,
    /// Write targets: up to 4 (slot, col) pairs. Must have at least one.
    pub targets: Vec<(u32, u32)>,
}

impl AccumulatorOp {
    /// Validate the registration for internal consistency.
    /// Called at registration time before the op is uploaded to the GPU.
    pub fn validate(&self) -> Result<(), AccumulatorOpError> {
        let threshold_emit = matches!(
            (&self.gate, self.consume),
            (GateSpec::Threshold { .. }, ConsumeMode::EmitEvent)
        );
        if self.targets.is_empty() && !threshold_emit {
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
            if inputs.len() > 4 {
                return Err(AccumulatorOpError::TooManyConjunctiveInputs(inputs.len()));
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
    #[error("ConjunctiveCrossing has {0} inputs; maximum is 4")]
    TooManyConjunctiveInputs(usize),
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
            source:  SourceSpec::Constant(1.0),
            combine: CombineFn::Identity,
            gate:    GateSpec::Always,
            scale:   ScaleSpec::Identity,
            consume: ConsumeMode::None,
            targets: vec![(0, 0)],
        }
    }

    #[test]
    fn valid_minimal_op_passes() {
        assert!(minimal_op().validate().is_ok());
    }

    #[test]
    fn no_targets_is_error() {
        let mut op = minimal_op();
        op.targets.clear();
        assert_eq!(op.validate(), Err(AccumulatorOpError::NoTargets));
    }

    #[test]
    fn threshold_emit_event_allows_empty_targets() {
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
        assert!(op.validate().is_ok());
    }

    #[test]
    fn five_targets_is_error() {
        let mut op = minimal_op();
        op.targets = vec![(0,0),(1,0),(2,0),(3,0),(4,0)];
        assert_eq!(op.validate(), Err(AccumulatorOpError::TooManyTargets(5)));
    }

    #[test]
    fn weighted_mean_requires_slot_range() {
        let mut op = minimal_op();
        op.combine = CombineFn::WeightedMean { weight_col: 1 };
        assert_eq!(op.validate(), Err(AccumulatorOpError::WeightedMeanRequiresSlotRange));
        op.source = SourceSpec::SlotRange { start: 0, count: 4 };
        assert!(op.validate().is_ok());
    }

    #[test]
    fn empty_slot_range_is_error() {
        let mut op = minimal_op();
        op.source = SourceSpec::SlotRange { start: 0, count: 0 };
        assert_eq!(op.validate(), Err(AccumulatorOpError::EmptySlotRange));
    }

    #[test]
    fn conjunctive_crossing_bounds() {
        let mut op = minimal_op();
        op.source = SourceSpec::ConjunctiveCrossing { inputs: vec![] };
        assert_eq!(op.validate(), Err(AccumulatorOpError::EmptyConjunctiveInputs));
        op.source = SourceSpec::ConjunctiveCrossing {
            inputs: (0u32..5).map(|i| InputSpec { slot: i, col: 0, unit_cost: 1.0 }).collect(),
        };
        assert_eq!(op.validate(), Err(AccumulatorOpError::TooManyConjunctiveInputs(5)));
        op.source = SourceSpec::ConjunctiveCrossing {
            inputs: vec![InputSpec { slot: 0, col: 0, unit_cost: 5.0 }],
        };
        assert!(op.validate().is_ok());
    }

    #[test]
    fn subtract_all_requires_conjunctive_source() {
        let mut op = minimal_op();
        op.consume = ConsumeMode::SubtractFromAllInputs;
        assert_eq!(op.validate(), Err(AccumulatorOpError::SubtractAllRequiresConjunctive));
        op.source = SourceSpec::ConjunctiveCrossing {
            inputs: vec![InputSpec { slot: 0, col: 0, unit_cost: 5.0 }],
        };
        assert!(op.validate().is_ok());
    }

    #[test]
    fn combine_fn_soft_aggregate_classification() {
        assert!(CombineFn::Mean.is_soft_aggregate());
        assert!(CombineFn::WeightedMean { weight_col: 0 }.is_soft_aggregate());
        assert!(!CombineFn::Sum.is_soft_aggregate());
        assert!(!CombineFn::Identity.is_soft_aggregate());
        assert!(!CombineFn::MinAcrossInputs.is_soft_aggregate());
    }

    #[test]
    fn serde_roundtrip_all_variants() {
        let ops = vec![
            AccumulatorOp {
                source: SourceSpec::Constant(3.14),
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::None,
                targets: vec![(1, 2)],
            },
            AccumulatorOp {
                source: SourceSpec::SlotRange { start: 0, count: 8 },
                combine: CombineFn::WeightedMean { weight_col: 3 },
                gate: GateSpec::OrderBand(2),
                scale: ScaleSpec::Constant(0.5),
                consume: ConsumeMode::None,
                targets: vec![(0, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::ConjunctiveCrossing {
                    inputs: vec![
                        InputSpec { slot: 1, col: 0, unit_cost: 5.0 },
                        InputSpec { slot: 1, col: 2, unit_cost: 3.0 },
                    ],
                },
                combine: CombineFn::MinAcrossInputs,
                gate: GateSpec::Threshold {
                    value: 1.0,
                    direction: ThresholdDirection::Upward,
                },
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::SubtractFromAllInputs,
                targets: vec![(99, 0)],
            },
            AccumulatorOp {
                source: SourceSpec::SlotValue { slot: 5, col: 0 },
                combine: CombineFn::EvalEML { tree_id: 42 },
                gate: GateSpec::LifecycleActive,
                scale: ScaleSpec::ByColumn { col: 1 },
                consume: ConsumeMode::EmitEvent,
                targets: vec![(10, 0), (10, 1)],
            },
        ];
        for op in &ops {
            let json = serde_json::to_string(op).expect("serialize");
            let roundtrip: AccumulatorOp = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(*op, roundtrip);
        }
    }
}
