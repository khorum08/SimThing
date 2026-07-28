//! Typed-plan → WGSL wire encoding boundary.
//!
//! **Door family:** `WGSL-RAW-BOUNDARY`.
//! **Promotion blocker:** `PLAN-STRUCT-TYPING-0` (0.0.8.7 rung 4.2).
//!
//! This is the single kernel module where the typed compile plan intentionally
//! drops to the raw `u32`/POD representation consumed by WGSL buffers. Runtime
//! buffer ownership and upload remain in [`crate::world_state`]; CPU planning
//! remains outside it. Rung 4.2 types plan structures up to this frontier.

use bytemuck::{Pod, Zeroable};
use simthing_core::{
    ClampBehavior, ColumnIndex, DimensionRegistry, PropertyColumnRange, PropertyLayout,
    SimPropertyId,
};

/// Drop a typed plan column onto the WGSL/`repr(C)` wire.
///
/// **Door family:** `WGSL-RAW-BOUNDARY`. Production plan/compile paths must not
/// call [`ColumnIndex::raw_u32`] directly — route through this helper.
#[inline]
pub fn encode_column(col: ColumnIndex) -> u32 {
    col.raw_u32()
}

/// Re-materialize a typed plan column from a WGSL/`repr(C)` wire field.
///
/// **Door family:** `WGSL-RAW-BOUNDARY`. Production GPU round-trip remints must
/// not call [`ColumnIndex::from_gpu_round_trip`] outside this module.
#[inline]
pub fn column_from_wire(raw: u32) -> ColumnIndex {
    ColumnIndex::from_gpu_round_trip(raw)
}

pub const CLAMP_BOUNDED: u32 = 0;
pub const CLAMP_FLOORED: u32 = 1;
pub const CLAMP_UNBOUNDED: u32 = 2;

/// WGSL wire row for one `(governed, governing)` sub-field pair.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct GovernedPair {
    pub governed_col: u32,
    pub governing_col: u32,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub vel_max: f32,
    pub clamp_kind: u32,
}

impl GovernedPair {
    fn encode_clamp(c: &ClampBehavior) -> (u32, f32, f32) {
        match c {
            ClampBehavior::Bounded { min, max } => (CLAMP_BOUNDED, *min, *max),
            ClampBehavior::Floored { min } => (CLAMP_FLOORED, *min, f32::INFINITY),
            ClampBehavior::Unbounded => (CLAMP_UNBOUNDED, f32::NEG_INFINITY, f32::INFINITY),
        }
    }
}

/// Emit one [`GovernedPair`] per sub-field with `governed_by: Some(_)`.
pub fn governed_pairs_for_property(
    range: &PropertyColumnRange,
    layout: &PropertyLayout,
) -> Vec<GovernedPair> {
    let mut pairs = Vec::new();
    for sf in &layout.sub_fields {
        let Some(gov_role) = &sf.governed_by else {
            continue;
        };
        let Some(governed_col) = range.col_for_role(&sf.role, layout) else {
            continue;
        };
        let Some(governing_col) = range.col_for_role(gov_role, layout) else {
            continue;
        };
        let (clamp_kind, clamp_min, clamp_max) = GovernedPair::encode_clamp(&sf.clamp);
        pairs.push(GovernedPair {
            governed_col: encode_column(governed_col),
            governing_col: encode_column(governing_col),
            clamp_min,
            clamp_max,
            vel_max: sf.velocity_max.unwrap_or(f32::INFINITY),
            clamp_kind,
        });
    }
    pairs
}

/// Walk every active property and emit its governed-pair WGSL rows.
pub fn build_governed_pairs(registry: &DimensionRegistry) -> Vec<GovernedPair> {
    let mut pairs = Vec::new();
    for (idx, prop) in registry.properties.iter().enumerate() {
        let id = SimPropertyId(idx as u32);
        if registry.is_active(id) {
            pairs.extend(governed_pairs_for_property(
                &registry.column_range(id),
                &prop.layout,
            ));
        }
    }
    pairs
}

pub const RULE_MEAN: u32 = 0;
pub const RULE_SUM: u32 = 1;
pub const RULE_MAX: u32 = 2;
pub const RULE_MIN: u32 = 3;
pub const RULE_FIRST: u32 = 4;
pub const RULE_WEIGHTED_MEAN: u32 = 5;

/// Sentinel in the per-column weight slot when the rule is not WeightedMean.
pub const WEIGHT_COL_NONE: u32 = u32::MAX;

pub fn encode_rule(rule: simthing_core::ReductionRule) -> u32 {
    use simthing_core::ReductionRule::*;
    match rule {
        Mean => RULE_MEAN,
        Sum => RULE_SUM,
        Max => RULE_MAX,
        Min => RULE_MIN,
        First => RULE_FIRST,
        WeightedMean { .. } => RULE_WEIGHTED_MEAN,
    }
}
