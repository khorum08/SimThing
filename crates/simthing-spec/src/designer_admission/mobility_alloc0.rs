//! Passive mobility residency records retained after allocator-policy retirement.
//!
//! The former `plan_mobility_alloc0` surface accepted arrival batches and
//! assigned the lowest free slot.  That made physical order capable of
//! selecting a recipient under oversubscription, so the planner, its block and
//! event vocabulary, and its assignment report are deliberately absent.
//!
//! Mobility re-enrollment may only carry an already-resident object's stable
//! logical slot across a structural parent change.  These two value types name
//! that recorded state; they grant nothing and expose no ordering input.
//!
//! ```compile_fail,E0432
//! use simthing_spec::{plan_mobility_alloc0, MobilityAlloc0PlanInput};
//! ```

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MobilityAlloc0ParentKey {
    pub parent_id: u64,
    pub key_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAlloc0LiveSlice {
    pub entity_id: u64,
    pub parent_key: MobilityAlloc0ParentKey,
    /// Stable logical slot retained across structural re-enrollment.
    pub slot: u32,
}
