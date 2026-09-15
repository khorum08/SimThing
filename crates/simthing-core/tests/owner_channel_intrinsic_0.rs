//! OWNER-CHANNEL-INTRINSIC-0 (rung 6.0) — deliverables (a) and (b).
//!
//! Input is constructed INLINE. Ownership resolution is a law over ARBITRARY trees, so the
//! smallest tree exhibiting each property is a complete witness; no scenario, corpus, or
//! shipped fixture is required or permitted (Transient Fixture Law).

use simthing_core::owner_channel::{
    bind_owner, declared_owner, is_ownership_crossing, resolve_owner, resolve_owners_in_order,
    unbind_owner, unowned, AuthoredOwnerRefError, OwnerRef, OwnerResolutionError,
    OWNER_CHANNEL_PROPERTY_ID,
};
use simthing_core::simthing::{SimThing, SimThingKind};
use simthing_core::PropertyValue;

use simthing_core::ids::SimThingId;

fn node() -> SimThing {
    // Ids are auto-assigned by SimThing::new; the u32 argument is the spawned generation.
    SimThing::new(SimThingKind::Location, 0)
}

fn resolved(root: &SimThing, id: SimThingId) -> OwnerRef {
    resolve_owner(root, id).expect("valid admitted tree member must resolve")
}

/// root -> mid -> leaf, three levels, nothing bound. Ids returned in depth order.
fn chain() -> (SimThing, [SimThingId; 3]) {
    let mut root = node();
    let mut mid = node();
    let leaf = node();
    let ids = [root.id, mid.id, leaf.id];
    mid.add_child(leaf);
    root.add_child(mid);
    (root, ids)
}

// ---------------------------------------------------------------- (b) totality

// ------------------------------------------------------- (b) inheritance depth

// ------------------------------------------------------------------- (b) fission

// ------------------------------------------------------------- (a) inert by default

// ------------------------------------------- (c) single ownership, multi-owner containers

// ----------------------------------------------------------------- (e) crossings

