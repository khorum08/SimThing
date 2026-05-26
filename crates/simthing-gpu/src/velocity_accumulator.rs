//! C-7 / E-7 governed integration → AccumulatorOp planner.
//!
//! Encodes property-level [`GovernedPair`] metadata into per-(slot, pair) GPU ops.
//! E-7 generalizes beyond `(Amount, Velocity)` to arbitrary `(Named, Named)` pairs
//! such as `(balance, flow)` using the same `IntegrateWithClamp` kernel branch.
//! `dt` is supplied at dispatch via [`crate::AccumulatorTickParams::dt_bits`], not
//! baked into uploaded ops.

use crate::accumulator_op::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};
use crate::world_state::GovernedPair;

#[derive(Clone, Debug, PartialEq)]
pub struct VelocityAccumulatorPlan {
    pub ops: Vec<AccumulatorOpGpu>,
    pub n_bands: u32,
}

/// E-7 alias: plan governed integration for arbitrary `governed_by` pairs.
pub type GovernedIntegrationPlan = VelocityAccumulatorPlan;

/// Build one AccumulatorOp per `(slot, governed pair)` matching legacy Pass 1
/// dispatch topology (`n_slots * n_pairs` threads).
pub fn plan_velocity_integration(pairs: &[GovernedPair], n_slots: u32) -> VelocityAccumulatorPlan {
    plan_governed_integration(pairs, n_slots)
}

/// E-7: compile arbitrary `governed_by` pairs to `IntegrateWithClamp` ops.
pub fn plan_governed_integration(
    pairs: &[GovernedPair],
    n_slots: u32,
) -> GovernedIntegrationPlan {
    let mut ops = Vec::with_capacity(pairs.len() * n_slots as usize);
    for slot in 0..n_slots {
        for pair in pairs {
            ops.push(pair_to_gpu_op(slot, pair));
        }
    }
    let n_bands = if ops.is_empty() { 0 } else { 1 };
    GovernedIntegrationPlan { ops, n_bands }
}

fn pair_to_gpu_op(slot: u32, pair: &GovernedPair) -> AccumulatorOpGpu {
    AccumulatorOpGpu {
        source_kind: source_kind::SLOT_VALUE,
        source_slot: slot,
        source_col: pair.governing_col,
        source_count: 0,
        combine_kind: combine_kind::INTEGRATE_CLAMP,
        combine_a: pair.vel_max.to_bits(),
        combine_b: pair.clamp_min.to_bits(),
        combine_c: pair.clamp_max.to_bits(),
        combine_d: pair.clamp_kind,
        gate_kind: gate_kind::ALWAYS,
        gate_a: 0,
        gate_b: 0,
        scale_kind: scale_kind::IDENTITY,
        scale_a: 0,
        consume: consume_kind::NONE,
        target0_slot: slot,
        target0_col: pair.governed_col,
        target1_slot: slot,
        target1_col: pair.governing_col,
        target2_slot: 0,
        target2_col: 0,
        target3_slot: 0,
        target3_col: 0,
        n_targets: 2,
        _pad: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_state::{CLAMP_BOUNDED, CLAMP_FLOORED};

    #[test]
    fn plan_emits_slot_pair_ops() {
        let pairs = vec![GovernedPair {
            governed_col: 0,
            governing_col: 1,
            clamp_min: 0.0,
            clamp_max: 1.0,
            vel_max: 0.5,
            clamp_kind: CLAMP_BOUNDED,
        }];
        let plan = plan_velocity_integration(&pairs, 3);
        assert_eq!(plan.ops.len(), 3);
        assert_eq!(plan.n_bands, 1);
        assert_eq!(plan.ops[0].target0_slot, 0);
        assert_eq!(plan.ops[1].target0_slot, 1);
        assert_eq!(plan.ops[2].target0_slot, 2);
        assert_eq!(plan.ops[0].combine_kind, combine_kind::INTEGRATE_CLAMP);
    }

    #[test]
    fn empty_pairs_yields_zero_bands() {
        let plan = plan_velocity_integration(&[], 4);
        assert!(plan.ops.is_empty());
        assert_eq!(plan.n_bands, 0);
    }

    #[test]
    fn encodes_clamp_kind_in_combine_d() {
        let pairs = vec![GovernedPair {
            governed_col: 0,
            governing_col: 1,
            clamp_min: -1.0,
            clamp_max: f32::INFINITY,
            vel_max: f32::INFINITY,
            clamp_kind: CLAMP_FLOORED,
        }];
        let plan = plan_governed_integration(&pairs, 1);
        assert_eq!(plan.ops[0].combine_d, CLAMP_FLOORED);
    }

    #[test]
    fn plan_governed_integration_emits_integrate_with_clamp_for_named_pair() {
        let pairs = vec![GovernedPair {
            governed_col: 11,
            governing_col: 10,
            clamp_min: 0.0,
            clamp_max: 1000.0,
            vel_max: 5.0,
            clamp_kind: CLAMP_BOUNDED,
        }];
        let plan = plan_governed_integration(&pairs, 2);
        assert_eq!(plan.ops.len(), 2);
        let op = &plan.ops[0];
        assert_eq!(op.combine_kind, combine_kind::INTEGRATE_CLAMP);
        assert_eq!(op.target0_col, 11);
        assert_eq!(op.target1_col, 10);
        assert_eq!(op.n_targets, 2);
        assert_eq!(op.consume, consume_kind::NONE);
    }
}
