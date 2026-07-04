//! Resource Flow compile-time metadata (E-8).
//!
//! `AccumulatorRole` and related types are spec/session metadata only. They
//! compile away into concrete combine/gate/consume choices before GPU upload;
//! `simthing-sim` must never branch on these at runtime.

use serde::{Deserialize, Serialize};

use crate::ids::SimPropertyId;
use crate::property::SubFieldRole;

pub type ArenaName = String;

/// Logging tier override for a resource-flow sub-field. Default: [`LogTier::Summary`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogTier {
    /// Slot summary checksum tier (production default).
    #[default]
    Summary,
    /// Compact emission/audit record tier.
    Compact,
    /// Full values readback (debug only).
    FullReadback,
}

/// Accumulator-substrate metadata for a sub-field participating in resource flow.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccumulatorSpec {
    pub role: AccumulatorRole,
    pub log_tier: LogTier,
}

/// Compile-time resource-flow role. Not a runtime participant taxonomy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AccumulatorRole {
    /// Signed rate signal contributing to upward Sum reduction.
    IntrinsicFlow,
    /// Per-arena allocated flow from a parent allocator downward sweep.
    AllocatedFlow { arena: ArenaName },
    /// Balance/Need ledger integrated via `governed_by` from total flow.
    Balance(BalanceSpec),
    /// Weight column for an intermediate allocator child split.
    AllocatorWeight { arena: ArenaName },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BalanceSpec {
    pub unit_cost: Option<f32>,
    pub num_count_source: Option<NumCountSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NumCountSource {
    Static(u32),
    Column {
        property_id: SimPropertyId,
        role: SubFieldRole,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::{ClampBehavior, PropertyLayout, SubFieldSpec};

    fn minimal_subfield() -> SubFieldSpec {
        SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "amount".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        }
    }

}
