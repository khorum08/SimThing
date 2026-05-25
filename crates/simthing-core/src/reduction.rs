//! Reduction rules — how child sub-field values aggregate into a parent.
//!
//! Reduction operates per-column. Every column in the GPU `output_vectors`
//! buffer is produced by reducing its children's corresponding column values
//! using the rule resolved from that sub-field's `SubFieldSpec`.
//!
//! ## Semantics
//!
//! Leaves: `output_vectors[slot] = values[slot]` (post-Pass 3 state).
//! Inner nodes: each column `c` is reduced across children using `rule(c)`:
//!
//! - `Mean` — arithmetic mean. Zero children → 0.0.
//! - `Sum`  — algebraic sum. Identity element 0.0.
//! - `Max`  — maximum. Identity element f32::NEG_INFINITY (zero children → 0.0).
//! - `Min`  — minimum. Identity element f32::INFINITY  (zero children → 0.0).
//! - `First`— value from the first child in canonical (slot) order. Used when
//!           aggregation is not semantically meaningful and the first child
//!           acts as a representative.
//! - `WeightedMean { by }` — `sum(child_value * weight) / sum(weight)` where
//!           `weight` is each child's `Amount` sub-field of property `by`.
//!           Zero total weight → 0.0.
//!
//! Floating-point determinism: both the CPU oracle and the GPU shader iterate
//! children in canonical slot order. Sum/Mean are not associative in float, so
//! reordering children would diverge — `SlotAllocator` ordering is the
//! contract.
//!
//! ## Role defaults
//!
//! Most properties never specify a reduction rule. The default is derived from
//! the sub-field's `SubFieldRole`:
//!
//! - `Amount`    → `Mean`  (a region's loyalty is the average of its cohorts')
//! - `Velocity`  → `Mean`  (rate-of-change averages, not sums)
//! - `Intensity` → `Max`   (the loudest child voice surfaces at the parent)
//! - `Named(_)`  → `Mean`  (designer-defined; override if Sum/Max needed)
//! - `Custom(_)` → `Mean`
//!
//! Designers override on a per-sub-field basis via
//! `SubFieldSpec::reduction_override`.

use serde::{Deserialize, Serialize};

use crate::property::SubFieldRole;

/// How a sub-field's values combine when reducing children into a parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReductionRule {
    Mean,
    Sum,
    Max,
    Min,
    /// First child's value in canonical slot order.
    First,
    /// Arithmetic mean weighted by another property's `Amount` on each child.
    WeightedMean { by: crate::ids::SimPropertyId },
}

impl ReductionRule {
    /// Default rule for a sub-field with this role when no override is set.
    pub fn default_for_role(role: &SubFieldRole) -> Self {
        match role {
            SubFieldRole::Amount    => ReductionRule::Mean,
            SubFieldRole::Velocity  => ReductionRule::Mean,
            SubFieldRole::Intensity => ReductionRule::Max,
            SubFieldRole::Named(_)  => ReductionRule::Mean,
            SubFieldRole::Custom(_) => ReductionRule::Mean,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_resolves_via_subfield_spec() {
        use crate::property::{ClampBehavior, SubFieldSpec};
        let mut spec = SubFieldSpec {
            role:          SubFieldRole::Amount,
            width:         1,
            clamp:         ClampBehavior::Unbounded,
            velocity_max:  None,
            default:       0.0,
            display_name:  "x".into(),
            display_range: None,
            governed_by:   None,
            reduction_override: None,
            soft_aggregate_guard: None,
        };
        assert_eq!(spec.resolved_reduction(), ReductionRule::Mean);
        spec.reduction_override = Some(ReductionRule::Sum);
        assert_eq!(spec.resolved_reduction(), ReductionRule::Sum);
    }

    #[test]
    fn role_defaults() {
        assert_eq!(ReductionRule::default_for_role(&SubFieldRole::Amount),    ReductionRule::Mean);
        assert_eq!(ReductionRule::default_for_role(&SubFieldRole::Velocity),  ReductionRule::Mean);
        assert_eq!(ReductionRule::default_for_role(&SubFieldRole::Intensity), ReductionRule::Max);
        assert_eq!(
            ReductionRule::default_for_role(&SubFieldRole::Named("axis_drift".into())),
            ReductionRule::Mean,
        );
        assert_eq!(
            ReductionRule::default_for_role(&SubFieldRole::Custom("mod_metric".into())),
            ReductionRule::Mean,
        );
    }
}
