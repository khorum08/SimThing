//! Structural tree mutation — step 8 of the day boundary.
//!
//! Implements the real execution of every `BoundaryRequest` variant. The
//! feeder's `TreeMaintainer::execute` was scaffolded as a counter/seam;
//! this module is where the actual mutations happen at boundary time.
//!
//! ## Why this lives in `simthing-sim` and not in the feeder
//!
//! The feeder crate's job is data-plane work — drain channels, mutate the
//! values shadow, run GPU passes. The SimThing tree is the authoritative
//! semantic state, and the day-boundary protocol is the only place that
//! ever mutates it structurally. That protocol lives here; so does this
//! function. The feeder's `MaintainerOutcome` type is reused as the result
//! shape — the scaffold's fields anticipated real execution.
//!
//! ## Per-variant behavior
//!
//! ### `AddChild { parent, child }`
//! - Walks the tree to find `parent`.
//! - Allocates a slot for the new child (and recursively for its subtree,
//!   though typical use is a fresh leaf).
//! - Attaches the child as a child of `parent`.
//! - Projects the added subtree's semantic properties into the CPU shadow,
//!   zeroing each row first so absent properties do not inherit stale data.
//! - Records the new id in `MaintainerOutcome::allocated`.
//!
//! Unknown parent → `rejected_unknown_target` increment, no slot churn.
//!
//! ### `Remove { target }`
//! - Walks the tree to find `target`.
//! - Zeros and tombstones the target's slot AND every descendant slot. This is
//!   crucial: a subtree removal must release every slot it owned, or the
//!   shadow rows for descendants stay live but unreachable.
//! - Removes the subtree from its parent's children list.
//! - Records all tombstoned ids in `MaintainerOutcome::tombstoned`.
//!
//! Unknown target → `rejected_unknown_target` increment.
//!
//! ### `Reparent { child, new_parent }`
//! - Walks the tree to find both nodes.
//! - Detaches the child subtree from its current parent.
//! - Attaches it under `new_parent`. Slots are NOT churned — the entire
//!   subtree keeps its existing slot assignments. This is the whole point
//!   of slot stability: reparenting is free in GPU terms.
//! - Records the reparent count and `(child, new_parent)` pairs in
//!   `MaintainerOutcome::reparents` / `reparented`.
//!
//! Either unknown → `rejected_unknown_target` increment; tree unchanged.
//!
//! ### `AttachOverlay { target, overlay }`
//! - Walks the tree to find `target` and pushes the overlay into its
//!   `overlays` Vec. Records the overlay id in `overlays_attached`.
//!
//! Note: this overlaps with `overlay_lifecycle::attach_overlay` from step 7.
//! The boundary protocol routes AttachOverlay through THIS function for
//! consistency — all structural mutations land in one place.
//!
//! ### `AddDimension { property }`
//! - Restores the property's registry columns if they were tombstoned.
//! - Records the property id so the boundary protocol can widen the CPU
//!   shadow and rebuild `WorldGpuState` before step 9 sync.

use crate::tree_index::{detach_at_path, node_at_path, node_at_path_mut};
use simthing_core::{DimensionRegistry, OverlayId, OverlayLifecycle, SimThing, SimThingId};
use simthing_feeder::{BoundaryRequest, MaintainerOutcome};
use simthing_gpu::SlotAllocator;
use std::collections::HashMap;

/// Apply every `BoundaryRequest` to the authoritative tree + slot table.
///
/// `values_shadow` must be sized `n_slots × n_dims` where `n_slots`
/// matches the capacity the `WorldGpuState` was built with. If `AddChild`
/// pushes the allocator past that capacity, the new slot's row is written
/// to a position outside `values_shadow` — the caller must catch this
/// before flushing. For Week 3 testing the fixture pre-allocates headroom.
pub fn apply_structural_mutations(
    requests: Vec<BoundaryRequest>,
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
    registry: &mut DimensionRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> MaintainerOutcome {
    let mut out = MaintainerOutcome::default();

    for req in requests {
        match req {
            BoundaryRequest::AddChild { parent, child } => {
                apply_add_child(
                    root,
                    allocator,
                    registry,
                    values_shadow,
                    n_dims,
                    parent,
                    child,
                    node_paths,
                    &mut out,
                );
            }
            BoundaryRequest::Remove { target } => {
                apply_remove(
                    root,
                    allocator,
                    values_shadow,
                    n_dims,
                    target,
                    node_paths,
                    &mut out,
                );
            }
            BoundaryRequest::Reparent { child, new_parent } => {
                apply_reparent(root, child, new_parent, node_paths, &mut out);
            }
            BoundaryRequest::AttachOverlay { target, overlay } => {
                let oid = overlay.id;
                if attach_overlay_to_node(root, target, overlay, node_paths) {
                    out.overlays += 1;
                    out.overlays_attached.push((target, oid));
                } else {
                    out.rejected_unknown_target += 1;
                }
            }
            BoundaryRequest::ActivateOverlay { target, overlay_id } => {
                match activate_overlay(root, target, overlay_id, node_paths) {
                    OverlayTransition::Changed => {
                        out.overlay_activations += 1;
                        out.overlays_activated.push((target, overlay_id));
                    }
                    OverlayTransition::NoOp => {}
                    OverlayTransition::Missing => out.rejected_unknown_target += 1,
                }
            }
            BoundaryRequest::SuspendOverlay { target, overlay_id } => {
                match suspend_overlay(root, target, overlay_id, node_paths) {
                    OverlayTransition::Changed => {
                        out.overlay_suspensions += 1;
                        out.overlays_suspended.push((target, overlay_id));
                    }
                    OverlayTransition::NoOp => {}
                    OverlayTransition::Missing => out.rejected_unknown_target += 1,
                }
            }
            BoundaryRequest::AddDimension { property } => {
                if property.index() < registry.properties.len() {
                    registry.restore(property);
                    out.dimensions += 1;
                    out.dimensions_added.push(property);
                } else {
                    out.rejected_unknown_target += 1;
                }
            }
        }
    }

    out
}

// ── AddChild ──────────────────────────────────────────────────────────────────

fn apply_add_child(
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
    registry: &DimensionRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
    parent_id: SimThingId,
    child: SimThing,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    out: &mut MaintainerOutcome,
) {
    // Collect every id in the subtree being added (typically just the
    // child itself, but supports importing pre-built subtrees).
    let mut new_ids = Vec::new();
    collect_subtree_ids(&child, &mut new_ids);

    // Find parent first; if missing, do nothing.
    let Some(parent) = lookup_node_mut(root, parent_id, node_paths) else {
        out.rejected_unknown_target += 1;
        return;
    };

    // Attach subtree.
    parent.add_child(child);

    // Allocate slots for every new id. Re-walk the attached subtree to
    // get a stable order (root before children); the SimThing we just
    // pushed is at the end of parent's children list.
    let attached = parent.children.last().expect("just pushed");
    populate_from_subtree(allocator, attached);

    // Project the attached subtree's semantic properties into the shadow.
    // Rows are zeroed first so absent properties do not inherit stale slot data.
    if let Some(attached) = find_node(root, new_ids[0]) {
        project_subtree_to_shadow(attached, allocator, registry, values_shadow, n_dims, out);
    }
    out.adds += 1;
}

fn collect_subtree_ids(node: &SimThing, out: &mut Vec<SimThingId>) {
    out.push(node.id);
    for c in &node.children {
        collect_subtree_ids(c, out);
    }
}

fn populate_from_subtree(allocator: &mut SlotAllocator, node: &SimThing) {
    allocator.alloc(node.id);
    for c in &node.children {
        populate_from_subtree(allocator, c);
    }
}

fn project_subtree_to_shadow(
    node: &SimThing,
    allocator: &SlotAllocator,
    registry: &DimensionRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
    out: &mut MaintainerOutcome,
) {
    if let Some(slot) = allocator.slot_of(node.id) {
        let base = (slot as usize) * n_dims;
        let end = base + n_dims;
        if end <= values_shadow.len() {
            values_shadow[base..end].fill(0.0);
            for (&pid, pval) in &node.properties {
                if !registry.is_active(pid) {
                    continue;
                }
                let prop = registry.property(pid);
                let range = registry.column_range(pid);
                let src_len = prop.layout.stride().min(pval.data.len());
                let dst = base + range.start;
                if dst + src_len <= values_shadow.len() {
                    values_shadow[dst..dst + src_len].copy_from_slice(&pval.data[..src_len]);
                }
            }
        }
        out.allocated.push(node.id);
    }

    for child in &node.children {
        project_subtree_to_shadow(child, allocator, registry, values_shadow, n_dims, out);
    }
}

// ── Remove ────────────────────────────────────────────────────────────────────

fn apply_remove(
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    target: SimThingId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    out: &mut MaintainerOutcome,
) {
    // Cannot remove the root via this path; it would orphan the tree.
    if root.id == target {
        out.rejected_unknown_target += 1;
        return;
    }

    // Find the subtree, collect its ids, then detach + tombstone.
    let subtree = if let Some(paths) = node_paths {
        paths
            .get(&target)
            .and_then(|path| detach_at_path(root, path))
    } else {
        detach_subtree(root, target)
    };
    let Some(subtree) = subtree else {
        out.rejected_unknown_target += 1;
        return;
    };

    let mut subtree_ids = Vec::new();
    collect_subtree_ids(&subtree, &mut subtree_ids);

    for sid in subtree_ids {
        if let Some(slot) = allocator.slot_of(sid) {
            zero_shadow_row(values_shadow, n_dims, slot);
        }
        if allocator.tombstone(sid).is_some() {
            out.tombstoned.push(sid);
        }
    }
    out.removes += 1;
}

fn zero_shadow_row(values_shadow: &mut [f32], n_dims: usize, slot: u32) {
    let base = (slot as usize) * n_dims;
    let end = base + n_dims;
    if end <= values_shadow.len() {
        values_shadow[base..end].fill(0.0);
    }
}

/// Walk the tree, find a child with the given id, remove it from its parent's
/// children list, and return the detached subtree.
fn detach_subtree(node: &mut SimThing, target: SimThingId) -> Option<SimThing> {
    if let Some(idx) = node.children.iter().position(|c| c.id == target) {
        return Some(node.children.remove(idx));
    }
    for c in &mut node.children {
        if let Some(s) = detach_subtree(c, target) {
            return Some(s);
        }
    }
    None
}

// ── Reparent ──────────────────────────────────────────────────────────────────

fn apply_reparent(
    root: &mut SimThing,
    child: SimThingId,
    new_parent: SimThingId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    out: &mut MaintainerOutcome,
) {
    if child == new_parent || child == root.id {
        // Self-parenting and root-reparenting are no-ops; flag as rejected.
        out.rejected_unknown_target += 1;
        return;
    }

    // Verify the new parent exists *before* detaching. Otherwise a missing
    // new parent would leave us with an orphaned subtree to dispose of.
    if lookup_node(root, new_parent, node_paths).is_none() {
        out.rejected_unknown_target += 1;
        return;
    }

    // Cannot reparent a node under its own descendant — would create a cycle.
    if let Some(child_node) = lookup_node(root, child, node_paths) {
        if subtree_contains(child_node, new_parent) {
            out.rejected_unknown_target += 1;
            return;
        }
    } else {
        out.rejected_unknown_target += 1;
        return;
    }

    let subtree = if let Some(paths) = node_paths {
        paths
            .get(&child)
            .and_then(|path| detach_at_path(root, path))
    } else {
        detach_subtree(root, child)
    };
    let Some(subtree) = subtree else {
        out.rejected_unknown_target += 1;
        return;
    };
    let Some(parent) = lookup_node_mut(root, new_parent, node_paths) else {
        // Race window: someone removed new_parent between check and detach.
        // We checked first to make this practically impossible in single-
        // threaded code; defensive log + reattach to root as fallback.
        out.rejected_unknown_target += 1;
        root.add_child(subtree);
        return;
    };
    parent.add_child(subtree);
    out.reparents += 1;
    out.reparented.push((child, new_parent));
}

fn subtree_contains(node: &SimThing, target: SimThingId) -> bool {
    if node.id == target {
        return true;
    }
    node.children.iter().any(|c| subtree_contains(c, target))
}

// ── AttachOverlay ─────────────────────────────────────────────────────────────

fn attach_overlay_to_node(
    root: &mut SimThing,
    target: SimThingId,
    overlay: simthing_core::Overlay,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> bool {
    if let Some(paths) = node_paths {
        if let Some(path) = paths.get(&target) {
            if let Some(node) = node_at_path_mut(root, path) {
                node.add_overlay(overlay);
                return true;
            }
            return false;
        }
    }
    if root.id == target {
        root.add_overlay(overlay);
        return true;
    }
    for c in &mut root.children {
        if attach_overlay_to_node(c, target, overlay.clone(), None) {
            return true;
        }
    }
    false
}

// ── Tree helpers ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OverlayTransition {
    Changed,
    NoOp,
    Missing,
}

fn activate_overlay(
    root: &mut SimThing,
    target: SimThingId,
    overlay_id: OverlayId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> OverlayTransition {
    let Some(node) = lookup_node_mut(root, target, node_paths) else {
        return OverlayTransition::Missing;
    };
    let Some(overlay) = node
        .overlays
        .iter_mut()
        .find(|overlay| overlay.id == overlay_id)
    else {
        return OverlayTransition::Missing;
    };
    let OverlayLifecycle::Suspended { when_activated } = overlay.lifecycle.clone() else {
        return OverlayTransition::NoOp;
    };
    overlay.lifecycle = *when_activated;
    OverlayTransition::Changed
}

fn suspend_overlay(
    root: &mut SimThing,
    target: SimThingId,
    overlay_id: OverlayId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> OverlayTransition {
    let Some(node) = lookup_node_mut(root, target, node_paths) else {
        return OverlayTransition::Missing;
    };
    let Some(overlay) = node
        .overlays
        .iter_mut()
        .find(|overlay| overlay.id == overlay_id)
    else {
        return OverlayTransition::Missing;
    };
    if matches!(overlay.lifecycle, OverlayLifecycle::Suspended { .. }) {
        return OverlayTransition::NoOp;
    }
    let active_lifecycle = overlay.lifecycle.clone();
    overlay.lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(active_lifecycle),
    };
    OverlayTransition::Changed
}

fn lookup_node<'a>(
    root: &'a SimThing,
    id: SimThingId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> Option<&'a SimThing> {
    if let Some(paths) = node_paths {
        paths.get(&id).and_then(|path| node_at_path(root, path))
    } else {
        find_node(root, id)
    }
}

fn lookup_node_mut<'a>(
    root: &'a mut SimThing,
    id: SimThingId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> Option<&'a mut SimThing> {
    if let Some(paths) = node_paths {
        paths.get(&id).and_then(|path| node_at_path_mut(root, path))
    } else {
        find_node_mut(root, id)
    }
}

fn find_node<'a>(node: &'a SimThing, id: SimThingId) -> Option<&'a SimThing> {
    if node.id == id {
        return Some(node);
    }
    for c in &node.children {
        if let Some(n) = find_node(c, id) {
            return Some(n);
        }
    }
    None
}

fn find_node_mut<'a>(node: &'a mut SimThing, id: SimThingId) -> Option<&'a mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    for c in &mut node.children {
        if let Some(n) = find_node_mut(c, id) {
            return Some(n);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DimensionRegistry, DissolveCondition, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
        OverlaySource, PropertyTransformDelta, PropertyValue, SimProperty, SimPropertyId, SimThing,
        SimThingKind, SubFieldRole, TransformOp,
    };
    use simthing_feeder::BoundaryRequest;
    use simthing_gpu::SlotAllocator;

    fn fixture() -> (DimensionRegistry, SlotAllocator, SimThing) {
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut alloc = SlotAllocator::new();
        alloc.alloc(root.id);
        let loc = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(loc.id);
        root.add_child(loc);
        (reg, alloc, root)
    }

    #[test]
    fn add_child_allocates_slot_and_attaches() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let parent_id = root.children[0].id;
        let cohort = SimThing::new(SimThingKind::Cohort, 1);
        let cohort_id = cohort.id;

        // Pre-size shadow with headroom for one new slot.
        let mut shadow = vec![0.5f32; (alloc.capacity() + 2) * n_dims];

        let out = apply_structural_mutations(
            vec![BoundaryRequest::AddChild {
                parent: parent_id,
                child: cohort,
            }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.adds, 1);
        assert_eq!(out.allocated, vec![cohort_id]);
        assert_eq!(root.children[0].children.len(), 1);
        assert_eq!(root.children[0].children[0].id, cohort_id);
        // New slot's row was zeroed.
        let new_slot = alloc.slot_of(cohort_id).unwrap();
        let base = (new_slot as usize) * n_dims;
        assert!(shadow[base..base + n_dims].iter().all(|v| *v == 0.0));
    }

    #[test]
    fn add_child_to_unknown_parent_is_rejected() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let ghost = SimThing::new(SimThingKind::Cohort, 0).id;
        let child = SimThing::new(SimThingKind::Cohort, 1);
        let mut shadow = vec![0.0f32; (alloc.capacity() + 2) * n_dims];

        let out = apply_structural_mutations(
            vec![BoundaryRequest::AddChild {
                parent: ghost,
                child,
            }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.rejected_unknown_target, 1);
        assert_eq!(out.adds, 0);
        assert!(out.allocated.is_empty());
    }

    #[test]
    fn add_child_projects_initialized_properties_into_shadow() {
        let (mut reg, mut alloc, mut root) = fixture();
        let pid = SimPropertyId(0);
        let layout = reg.property(pid).layout.clone();
        let amount = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let velocity = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let n_dims = reg.total_columns;
        let parent_id = root.children[0].id;

        let mut child = SimThing::new(SimThingKind::Cohort, 1);
        let child_id = child.id;
        let mut pval = PropertyValue::from_layout(&layout);
        pval.data[amount] = 0.7;
        pval.data[velocity] = -0.2;
        child.add_property(pid, pval);

        let mut shadow = vec![9.0f32; (alloc.capacity() + 2) * n_dims];
        let out = apply_structural_mutations(
            vec![BoundaryRequest::AddChild {
                parent: parent_id,
                child,
            }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.adds, 1);
        let slot = alloc.slot_of(child_id).unwrap() as usize;
        let base = slot * n_dims;
        assert_eq!(shadow[base + amount], 0.7);
        assert_eq!(shadow[base + velocity], -0.2);
    }

    #[test]
    fn remove_tombstones_target_and_all_descendants() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let loc_id = root.children[0].id;
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let cohort_id = cohort.id;
        let leaf = SimThing::new(SimThingKind::Cohort, 0);
        let leaf_id = leaf.id;
        cohort.add_child(leaf);
        alloc.alloc(cohort_id);
        alloc.alloc(leaf_id);
        root.children[0].add_child(cohort);

        let mut shadow = vec![1.0f32; alloc.capacity() * n_dims];
        let loc_slot = alloc.slot_of(loc_id).unwrap() as usize;
        let cohort_slot = alloc.slot_of(cohort_id).unwrap() as usize;
        let leaf_slot = alloc.slot_of(leaf_id).unwrap() as usize;

        let out = apply_structural_mutations(
            vec![BoundaryRequest::Remove { target: loc_id }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.removes, 1);
        assert_eq!(out.tombstoned.len(), 3);
        assert!(out.tombstoned.contains(&loc_id));
        assert!(out.tombstoned.contains(&cohort_id));
        assert!(out.tombstoned.contains(&leaf_id));
        assert!(root.children.is_empty());
        assert!(!alloc.is_live(alloc.capacity() as u32 - 1));
        assert!(shadow[loc_slot * n_dims..loc_slot * n_dims + n_dims]
            .iter()
            .all(|v| *v == 0.0));
        assert!(shadow[cohort_slot * n_dims..cohort_slot * n_dims + n_dims]
            .iter()
            .all(|v| *v == 0.0));
        assert!(shadow[leaf_slot * n_dims..leaf_slot * n_dims + n_dims]
            .iter()
            .all(|v| *v == 0.0));
    }

    #[test]
    fn remove_root_is_rejected() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let root_id = root.id;
        let mut shadow = vec![0.0f32; alloc.capacity() * n_dims];

        let out = apply_structural_mutations(
            vec![BoundaryRequest::Remove { target: root_id }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.rejected_unknown_target, 1);
        assert_eq!(out.removes, 0);
    }

    #[test]
    fn reparent_moves_subtree_without_slot_churn() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let loc_id = root.children[0].id;
        let new_loc = SimThing::new(SimThingKind::Location, 0);
        let new_loc_id = new_loc.id;
        alloc.alloc(new_loc_id);
        root.add_child(new_loc);

        // Add a cohort under the original location.
        let cohort = SimThing::new(SimThingKind::Cohort, 0);
        let cohort_id = cohort.id;
        alloc.alloc(cohort_id);
        root.children[0].add_child(cohort);

        let original_cohort_slot = alloc.slot_of(cohort_id).unwrap();
        let mut shadow = vec![0.0f32; alloc.capacity() * n_dims];

        let out = apply_structural_mutations(
            vec![BoundaryRequest::Reparent {
                child: cohort_id,
                new_parent: new_loc_id,
            }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.reparents, 1);
        assert_eq!(out.rejected_unknown_target, 0);
        // Cohort moved.
        assert!(root.children[0].children.is_empty());
        let new_loc_node = root.children.iter().find(|c| c.id == new_loc_id).unwrap();
        assert_eq!(new_loc_node.children.len(), 1);
        assert_eq!(new_loc_node.children[0].id, cohort_id);
        // Slot is preserved.
        assert_eq!(alloc.slot_of(cohort_id), Some(original_cohort_slot));
        // Suppress unused warning.
        let _ = loc_id;
    }

    #[test]
    fn reparent_cycle_is_rejected() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let loc_id = root.children[0].id;
        let root_id = root.id;
        let mut shadow = vec![0.0f32; alloc.capacity() * n_dims];

        // Trying to reparent root under its own child would create a cycle.
        let out = apply_structural_mutations(
            vec![BoundaryRequest::Reparent {
                child: root_id,
                new_parent: loc_id,
            }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        // Root reparenting is caught by the root-id check before cycle detection.
        assert_eq!(out.rejected_unknown_target, 1);
    }

    #[test]
    fn attach_overlay_appends_to_target() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let loc_id = root.children[0].id;
        let pid = SimPropertyId(0);
        let overlay = Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.5))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        };
        let oid = overlay.id;
        let mut shadow = vec![0.0f32; alloc.capacity() * n_dims];

        let out = apply_structural_mutations(
            vec![BoundaryRequest::AttachOverlay {
                target: loc_id,
                overlay,
            }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.overlays, 1);
        assert_eq!(out.overlays_attached, vec![(loc_id, oid)]);
        assert_eq!(root.children[0].overlays.len(), 1);
    }

    #[test]
    fn activate_overlay_restores_parked_lifecycle() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let loc_id = root.children[0].id;
        let pid = SimPropertyId(0);
        let overlay_id = OverlayId::new();
        root.children[0].add_overlay(Overlay {
            id: overlay_id,
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.1))],
            },
            lifecycle: OverlayLifecycle::Suspended {
                when_activated: Box::new(OverlayLifecycle::Transient {
                    dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 3 }],
                }),
            },
        });
        let mut shadow = vec![0.0f32; alloc.capacity() * n_dims];

        let out = apply_structural_mutations(
            vec![BoundaryRequest::ActivateOverlay {
                target: loc_id,
                overlay_id,
            }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.overlay_activations, 1);
        assert_eq!(out.overlays_activated, vec![(loc_id, overlay_id)]);
        assert!(matches!(
            root.children[0].overlays[0].lifecycle,
            OverlayLifecycle::Transient { ref dissolution_conditions }
                if matches!(
                    dissolution_conditions.as_slice(),
                    [DissolveCondition::AfterTicks { remaining: 3 }]
                )
        ));
    }

    #[test]
    fn suspend_overlay_parks_current_lifecycle() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let loc_id = root.children[0].id;
        let pid = SimPropertyId(0);
        let overlay_id = OverlayId::new();
        root.children[0].add_overlay(Overlay {
            id: overlay_id,
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(0.9))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        });
        let mut shadow = vec![0.0f32; alloc.capacity() * n_dims];

        let out = apply_structural_mutations(
            vec![BoundaryRequest::SuspendOverlay {
                target: loc_id,
                overlay_id,
            }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.overlay_suspensions, 1);
        assert_eq!(out.overlays_suspended, vec![(loc_id, overlay_id)]);
        let OverlayLifecycle::Suspended { when_activated } =
            &root.children[0].overlays[0].lifecycle
        else {
            panic!("overlay should be suspended");
        };
        assert!(matches!(**when_activated, OverlayLifecycle::Permanent));
    }

    #[test]
    fn add_dimension_restores_property() {
        let (mut reg, mut alloc, mut root) = fixture();
        let n_dims = reg.total_columns;
        let pid = SimPropertyId(0);
        reg.tombstone(pid);
        let mut shadow = vec![0.0f32; alloc.capacity() * n_dims];

        let out = apply_structural_mutations(
            vec![BoundaryRequest::AddDimension { property: pid }],
            &mut root,
            &mut alloc,
            &mut reg,
            &mut shadow,
            n_dims,
            None,
        );

        assert_eq!(out.dimensions, 1);
        assert_eq!(out.deferred, 0);
        assert_eq!(out.dimensions_added, vec![pid]);
        assert!(reg.is_active(pid));
    }
}
