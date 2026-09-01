//! Consumer-only inspection surface for the final-home resident clearing plan.
//!
//! Construction remains in `simthing-kernel`; GPU storage remains in
//! `simthing-gpu` and is exercised by the integration witness. This module
//! intentionally contains no dictionary, layout, scoring, or allocation code.

use simthing_kernel::{ResidentClearingPlan, SemanticPlanDigest};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentClearingPlanObservation {
    pub owner_count: u32,
    pub resource_count: u32,
    pub scope_count: u32,
    pub draw_count: u32,
    pub row_count: u32,
    pub canonical_bytes: usize,
    pub digest: SemanticPlanDigest,
}

/// Observe the immutable final-home plan through its public kernel door.
pub fn observe_resident_clearing_plan(
    plan: &ResidentClearingPlan,
) -> ResidentClearingPlanObservation {
    let ranges = plan.ranges();
    ResidentClearingPlanObservation {
        owner_count: ranges.owners.len(),
        resource_count: ranges.resources.len(),
        scope_count: ranges.scopes.len(),
        draw_count: ranges.draws.len(),
        row_count: ranges.rows.len(),
        canonical_bytes: plan.canonical_bytes().len(),
        digest: plan.digest(),
    }
}
