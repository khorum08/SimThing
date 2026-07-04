//! Tree Maintainer — day-boundary-only structural mutations.
//!
//! Per design_v4.md §11: "handles slot allocation, deallocation,
//! reparenting, and `AddDimension` events." This module is currently a
//! **scaffold**: it accepts `BoundaryRequest`s and produces an outcome
//! record describing what *would* have been done, but the tree mutations,
//! slot allocations, registry edits, and GPU buffer resizing live in
//! follow-up Week 3 work (the upcoming `simthing-sim` crate).
//!
//! The scaffolding here is real, not a stub-out: it owns the types and the
//! request dispatch surface, so when the day-boundary protocol lands it
//! only has to plug execution into the existing methods rather than
//! redesign the seam.
//!
//! ## Why the seam matters
//!
//! Invariant I7 ("structural mutations only at the day boundary") is the
//! contract that lets within-day code skip every form of structural
//! defensive checking. The Patcher trusts that the `SlotAllocator` is
//! stable across a day. Pass 7 trusts that no threshold-targeted slot
//! suddenly disappears mid-tick. Pass 3 trusts that `slot_delta_ranges`
//! still points at valid `overlay_deltas` rows. Maintainer code that
//! tried to execute mid-day would break all of those quietly.
//!
//! The dispatch surface here enforces that contract by **only being
//! callable from a boundary context**: the caller hands over the parked
//! request Vec from `TransformPatcher::take_boundary_requests` *between*
//! the §10 day-end and day-start phases. The maintainer never sees the
//! channel directly.

use crate::work::BoundaryRequest;
use simthing_core::{OverlayId, SimPropertyId, SimThingId};

// ── Outcome ───────────────────────────────────────────────────────────────────

/// Summary of one boundary-cycle's worth of structural work. Mirrors the
/// counters on `PatcherStats` — diagnostic, not authoritative state.
/// The authoritative outputs (new slot ids, allocator deltas, dimension
/// expansions) flow back to the caller via the maintainer's mutable
/// arguments once execution lands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MaintainerOutcome {
    /// `AddChild` requests acknowledged (slot will be allocated at exec).
    pub adds: u32,
    /// `Remove` requests acknowledged (slot will be tombstoned at exec).
    pub removes: u32,
    /// `Reparent` requests acknowledged.
    pub reparents: u32,
    /// `AttachOverlay` requests acknowledged.
    pub overlays: u32,
    /// `ActivateOverlay` requests acknowledged.
    pub overlay_activations: u32,
    /// `SuspendOverlay` requests acknowledged.
    pub overlay_suspensions: u32,
    /// `AddDimension` requests acknowledged.
    pub dimensions: u32,
    /// Requests rejected because the target id was not found in the tree.
    pub rejected_unknown_target: u32,
    /// Requests deferred because they reference state not yet implemented
    /// (everything, currently — see TODO list at module top).
    pub deferred: u32,
    /// New `SimThingId`s allocated this cycle (one per `AddChild`).
    pub allocated: Vec<SimThingId>,
    /// `SimThingId`s tombstoned this cycle.
    pub tombstoned: Vec<SimThingId>,
    /// Each successful overlay attach: `(target_sim_thing_id, overlay_id)`.
    /// Used by the delta log to resolve the full `Overlay` from the live tree
    /// for replay serialization.
    pub overlays_attached: Vec<(SimThingId, OverlayId)>,
    /// Each successful overlay activation: `(target_sim_thing_id, overlay_id)`.
    pub overlays_activated: Vec<(SimThingId, OverlayId)>,
    /// Each successful overlay suspension: `(target_sim_thing_id, overlay_id)`.
    pub overlays_suspended: Vec<(SimThingId, OverlayId)>,
    /// New `SimPropertyId`s admitted this cycle.
    pub dimensions_added: Vec<SimPropertyId>,
    /// Each successful reparent: `(child_id, new_parent_id)`.
    pub reparented: Vec<(SimThingId, SimThingId)>,
}

// ── Maintainer ────────────────────────────────────────────────────────────────

/// Owner of boundary-only structural mutation logic. Holds no state today —
/// every input comes through `execute` arguments. Once `simthing-sim` lands
/// the day-boundary protocol, this struct will gain references to the
/// authoritative SimThing tree root, the `SlotAllocator`, the
/// `DimensionRegistry`, and the `WorldGpuState` so that boundary execution
/// can resize GPU buffers in one place.
#[derive(Debug, Default)]
pub struct TreeMaintainer;

impl TreeMaintainer {
    pub fn new() -> Self {
        Self
    }

    /// Drain a boundary batch. Today this records every request as
    /// `deferred` and returns the count summary; the actual mutations are
    /// follow-up Week 3 work. The method signature is the final one — when
    /// execution lands, only the body changes.
    ///
    /// Called by the day-boundary protocol between §10 step 5 ("property
    /// expiry resolves") and step 9 ("feeder patches GPU buffers"). The
    /// requests are in arrival order; execution should preserve that order
    /// to keep replay logs deterministic.
    pub fn execute(&mut self, requests: Vec<BoundaryRequest>) -> MaintainerOutcome {
        let mut out = MaintainerOutcome::default();
        for req in requests {
            // Classify, then defer. The classification half is wired so
            // that the count fields tell the truth about what the
            // simulation queued, even before execution exists.
            match req {
                BoundaryRequest::AddChild { .. } => {
                    out.adds += 1;
                }
                BoundaryRequest::Remove { .. } => {
                    out.removes += 1;
                }
                BoundaryRequest::Reparent { .. } => {
                    out.reparents += 1;
                }
                BoundaryRequest::AttachOverlay { .. } => {
                    out.overlays += 1;
                }
                BoundaryRequest::ActivateOverlay { .. } => {
                    out.overlay_activations += 1;
                }
                BoundaryRequest::SuspendOverlay { .. } => {
                    out.overlay_suspensions += 1;
                }
                BoundaryRequest::AddDimension { .. } => {
                    out.dimensions += 1;
                }
            }
            out.deferred += 1;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::work::BoundaryRequest;
    use simthing_core::{SimThing, SimThingKind};

}
