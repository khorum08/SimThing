//! AO-WGSL-0 — generic semantic-free AccumulatorOp WGSL fast-path classification
//! and compatibility checks.

use super::types::{combine_kind, gate_kind, source_kind, AccumulatorOpGpu};

/// `AccumulatorTickParams._pad1` carries band count for `execute_orderband_bands`.
pub const AO_WGSL0_N_BANDS_UNIFORM_FIELD: &str = "tick_params._pad1";

/// Named AO-WGSL-0 WGSL entry point (defined in `accumulator_op.wgsl`).
pub const AO_WGSL0_ENTRY_POINT: &str = "execute_orderband_bands";

/// Supported AO plan shapes for the AO-WGSL-0 fast path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AoWgsl0PlanShape {
    /// E-11 / A-0 Resource Flow OrderBand allocation (includes EvalEML disburse + integration).
    ResourceFlowOrderBand,
    /// B-0 / Phase T discrete transfer OrderBand path.
    TransferOrderBand,
    /// Flat-star E-11 allocation (same substrate as ResourceFlowOrderBand).
    FlatStarOrderBand,
}

/// Why a plan cannot use the AO-WGSL-0 fast path (falls back to per-band dispatch).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AoWgsl0FallbackReason {
    EmptyPlan,
    ThresholdOps,
    AffineIntentOps,
    UnsupportedCombine { kind: u32 },
    UnsupportedGate { kind: u32 },
    CrossingFormulaOps,
    ProductOps,
    LastByPriorityOps,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AoWgsl0Compatibility {
    Compatible(AoWgsl0PlanShape),
    Fallback(AoWgsl0FallbackReason),
}

/// Classify uploaded GPU ops for AO-WGSL-0 fast-path eligibility.
///
/// The fast path fuses sequential OrderBand dispatches into one kernel launch.
/// Threshold, affine-intent, and unimplemented combine families fall back.
pub fn classify_ao_wgsl0_plan(ops: &[AccumulatorOpGpu]) -> AoWgsl0Compatibility {
    if ops.is_empty() {
        return AoWgsl0Compatibility::Fallback(AoWgsl0FallbackReason::EmptyPlan);
    }

    let mut saw_transfer = false;
    let mut saw_rf = false;

    for op in ops {
        if op.gate_kind == gate_kind::THRESHOLD {
            return AoWgsl0Compatibility::Fallback(AoWgsl0FallbackReason::ThresholdOps);
        }
        if op.combine_kind == combine_kind::AFFINE_INTENT {
            return AoWgsl0Compatibility::Fallback(AoWgsl0FallbackReason::AffineIntentOps);
        }
        if op.combine_kind == combine_kind::CROSSING_FORMULA {
            return AoWgsl0Compatibility::Fallback(AoWgsl0FallbackReason::CrossingFormulaOps);
        }
        if op.combine_kind == combine_kind::PRODUCT {
            return AoWgsl0Compatibility::Fallback(AoWgsl0FallbackReason::ProductOps);
        }
        if op.combine_kind == combine_kind::LAST_BY_PRIORITY {
            return AoWgsl0Compatibility::Fallback(AoWgsl0FallbackReason::LastByPriorityOps);
        }

        match op.gate_kind {
            gate_kind::ALWAYS | gate_kind::ORDER_BAND => {}
            other => {
                return AoWgsl0Compatibility::Fallback(AoWgsl0FallbackReason::UnsupportedGate {
                    kind: other,
                });
            }
        }

        match op.combine_kind {
            combine_kind::IDENTITY
            | combine_kind::SUM
            | combine_kind::MEAN
            | combine_kind::MAX
            | combine_kind::MIN
            | combine_kind::WEIGHTED_MEAN
            | combine_kind::FIRST
            | combine_kind::INTEGRATE_CLAMP
            | combine_kind::MIN_ACROSS_INPUTS
            | combine_kind::EVAL_EML => {}
            other => {
                return AoWgsl0Compatibility::Fallback(AoWgsl0FallbackReason::UnsupportedCombine {
                    kind: other,
                });
            }
        }

        if op.source_kind == source_kind::INPUT_LIST
            && op.combine_kind == combine_kind::MIN_ACROSS_INPUTS
        {
            saw_transfer = true;
        }
        if op.combine_kind == combine_kind::EVAL_EML
            || op.combine_kind == combine_kind::INTEGRATE_CLAMP
            || (op.source_kind == source_kind::SLOT_RANGE && op.combine_kind == combine_kind::SUM)
        {
            saw_rf = true;
        }
    }

    if saw_rf {
        AoWgsl0Compatibility::Compatible(AoWgsl0PlanShape::ResourceFlowOrderBand)
    } else if saw_transfer {
        AoWgsl0Compatibility::Compatible(AoWgsl0PlanShape::TransferOrderBand)
    } else {
        AoWgsl0Compatibility::Compatible(AoWgsl0PlanShape::FlatStarOrderBand)
    }
}

/// Returns true when the plan may run through `execute_orderband_bands`.
pub fn ao_wgsl0_fast_path_compatible(ops: &[AccumulatorOpGpu]) -> bool {
    matches!(
        classify_ao_wgsl0_plan(ops),
        AoWgsl0Compatibility::Compatible(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accumulator_op::types::AccumulatorOpGpu;
    use bytemuck::Zeroable;

    fn minimal_orderband_op(band: u32) -> AccumulatorOpGpu {
        let mut op = AccumulatorOpGpu::zeroed();
        op.gate_kind = gate_kind::ORDER_BAND;
        op.gate_a = band;
        op.combine_kind = combine_kind::IDENTITY;
        op.source_kind = source_kind::SLOT_VALUE;
        op
    }

}
