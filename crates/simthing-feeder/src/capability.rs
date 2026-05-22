//! Capability unlock registration — the bridge between `simthing-spec`'s
//! `CapabilityTreeBuilder` output and the simulation's Pass 7 threshold
//! infrastructure.
//!
//! Lives in `simthing-feeder` (not `simthing-spec` or `simthing-sim`) because
//! the feeder already mediates the spec → sim boundary: the spec layer emits
//! a `Vec<CapabilityUnlockRegistration>` at session-init time, the feeder
//! / session coordinator hands it to `ThresholdBuilder::build_with_capability_unlocks`,
//! and `simthing-sim` translates it to `ThresholdRegistration`s on the GPU
//! Pass 7 buffer plus matching `ThresholdSemantic::CapabilityUnlock` entries
//! on the CPU registry.

use serde::{Deserialize, Serialize};
use simthing_core::{SimPropertyId, SimThingId, SubFieldRole};

/// Authored-side request: when this `(sim_thing, property, sub_field)` triple's
/// value crosses `threshold` upward, fire a Pass 7 event that the boundary
/// handler resolves as a capability unlock.
///
/// Emitted by `CapabilityTreeBuilder::build` — one per `ActivationMode::Threshold`
/// entry. `PlayerSelection` and runtime-only `OnPrereqMet` entries produce none.
///
/// The threshold value is the entry's authored `research_cost`. The direction
/// is implicitly `Upward` — progress accumulates from 0 toward `research_cost`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityUnlockRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id:  SimPropertyId,
    pub sub_field:    SubFieldRole,
    pub threshold:    f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The PR 3 builder and the PR 4 sim-side `ThresholdBuilder::build_with_capability_unlocks`
    /// must agree on this type. Both depend on `simthing-feeder` and use the
    /// fully-qualified path. This test exists to fail loudly if the type ever
    /// leaves the feeder crate's public surface.
    #[test]
    fn capability_unlock_registration_in_feeder_is_public() {
        let _reg: CapabilityUnlockRegistration = CapabilityUnlockRegistration {
            sim_thing_id: SimThingId::new(),
            property_id:  SimPropertyId(0),
            sub_field:    SubFieldRole::Amount,
            threshold:    1.0,
        };
        // Also reachable via `simthing_feeder::CapabilityUnlockRegistration`
        // (the crate-root re-export):
        let _alias: crate::CapabilityUnlockRegistration = _reg.clone();
        assert_eq!(_reg, _alias);
    }
}
