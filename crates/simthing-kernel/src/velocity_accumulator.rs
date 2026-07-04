//! C-7 / E-7 governed integration → AccumulatorOp planner.
//!
//! Encodes property-level [`GovernedPair`] metadata into per-(slot, pair) GPU ops.
//! E-7 generalizes beyond `(Amount, Velocity)` to arbitrary `(Named, Named)` pairs
//! such as `(balance, flow)` using the same `IntegrateWithClamp` kernel branch.
//! E-7R adds band-targeted planning for E-11 integration placement.
//! `dt` is supplied at dispatch via [`crate::AccumulatorTickParams::dt_bits`], not
//! baked into uploaded ops.

use crate::accumulator_op::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};
use crate::world_state::GovernedPair;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum PlannerError {
    #[error("participant_filter is empty")]
    EmptyParticipantFilter,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VelocityAccumulatorPlan {
    pub ops: Vec<AccumulatorOpGpu>,
    pub n_bands: u32,
}

/// E-7 alias: plan governed integration for arbitrary `governed_by` pairs.
pub type GovernedIntegrationPlan = VelocityAccumulatorPlan;

/// Build one compact AccumulatorOp per governed pair. The velocity encoder
/// expands each pair across `n_slots` GPU invocations at dispatch time, avoiding
/// a host upload of `n_slots * n_pairs` materialized ops.
pub fn plan_velocity_integration(pairs: &[GovernedPair], n_slots: u32) -> VelocityAccumulatorPlan {
    let ops = pairs
        .iter()
        .map(|pair| compact_pair_to_gpu_op(pair, n_slots))
        .collect::<Vec<_>>();
    let n_bands = if ops.is_empty() { 0 } else { 1 };
    VelocityAccumulatorPlan { ops, n_bands }
}

/// E-7: compile arbitrary `governed_by` pairs to `IntegrateWithClamp` ops.
pub fn plan_governed_integration(pairs: &[GovernedPair], n_slots: u32) -> GovernedIntegrationPlan {
    let mut ops = Vec::with_capacity(pairs.len() * n_slots as usize);
    for slot in 0..n_slots {
        for pair in pairs {
            ops.push(pair_to_gpu_op(slot, pair, gate_kind::ALWAYS, 0));
        }
    }
    let n_bands = if ops.is_empty() { 0 } else { 1 };
    GovernedIntegrationPlan { ops, n_bands }
}

/// E-7R: place governed integration ops on a specific OrderBand, optionally
/// restricted to a participant slot subset (E-11 integration phase).
pub fn plan_governed_integration_at_band(
    pairs: &[GovernedPair],
    n_slots: u32,
    band: u32,
    participant_filter: Option<&[u32]>,
) -> Result<GovernedIntegrationPlan, PlannerError> {
    let slots: Vec<u32> = match participant_filter {
        None => (0..n_slots).collect(),
        Some(filter) if filter.is_empty() => return Err(PlannerError::EmptyParticipantFilter),
        Some(filter) => filter.to_vec(),
    };
    let mut ops = Vec::with_capacity(pairs.len() * slots.len());
    for slot in slots {
        for pair in pairs {
            ops.push(pair_to_gpu_op(slot, pair, gate_kind::ORDER_BAND, band));
        }
    }
    let n_bands = if ops.is_empty() {
        0
    } else {
        band.saturating_add(1)
    };
    Ok(GovernedIntegrationPlan { ops, n_bands })
}

fn pair_to_gpu_op(slot: u32, pair: &GovernedPair, gate: u32, gate_a: u32) -> AccumulatorOpGpu {
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
        gate_kind: gate,
        gate_a,
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

fn compact_pair_to_gpu_op(pair: &GovernedPair, n_slots: u32) -> AccumulatorOpGpu {
    AccumulatorOpGpu {
        source_kind: source_kind::SLOT_RANGE,
        source_slot: 0,
        source_col: pair.governing_col,
        source_count: n_slots,
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
        target0_slot: 0,
        target0_col: pair.governed_col,
        target1_slot: 0,
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

}
