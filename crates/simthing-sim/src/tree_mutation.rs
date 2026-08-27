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
//! - Resolves the overlay's required origin and target, walks origin -> common
//!   ancestor -> target, and terminates in the target's existing `overlays` Vec.
//! - Intermediate policy/governance overlays filter an instruction along that
//!   route; no second inbox, transport, or scheduler is introduced.
//! - Records the overlay id in `overlays_attached`.
//!
//! Note: this overlaps with `overlay_lifecycle::attach_overlay` from step 7.
//! The boundary protocol routes AttachOverlay through THIS function for
//! consistency — all structural mutations land in one place.
//!
//! ### `AddDimension { property }`
//! - Restores the property's registry columns if they were tombstoned.
//! - Records the property id so the boundary protocol can widen the CPU
//!   shadow and rebuild `WorldGpuState` before step 9 sync.

use crate::grant_disbursement::{is_protected_grant_overlay, GrantDisbursementBoundaryAuthority};
use crate::growth_entitlement::VerifiedGrowthResidencyCommit;
use crate::overlay_lifecycle::OverlayLifecycleAdmissionState;
use crate::sim_runtime_tree::SimRuntimeTree;
use crate::tree_index::{detach_at_path, node_at_path, node_at_path_mut};
use simthing_core::{
    prepare_fission_clone_sources_subtree, DimensionRegistry, GenerationStamp,
    ObjectResidencyRequest, OverlayId, OverlayLifecycle, SimThing, SimThingId,
};
use simthing_feeder::{BoundaryRequest, FeederError, FeederSender, MaintainerOutcome};
use simthing_gpu::SlotAllocator;
use simthing_kernel::StructuralCommitment;
use std::collections::{BTreeMap, HashMap};
use thiserror::Error;

/// Session-frozen application door from a sealed structural commitment to an
/// already-admitted boundary request. The numeric commitment fields are never
/// inspected: sealed `event_kind` identity selects one fixed request, and the
/// existing feeder/boundary authority applies it later.
#[derive(Clone, Debug)]
pub struct StructuralCommitmentApplicationDoor {
    requests_by_event_kind: BTreeMap<u32, BoundaryRequest>,
}

impl StructuralCommitmentApplicationDoor {
    pub fn from_pre_admitted_requests(
        requests: Vec<(u32, BoundaryRequest)>,
    ) -> Result<Self, StructuralCommitmentApplicationError> {
        let mut requests_by_event_kind = BTreeMap::new();
        for (event_kind, request) in requests {
            if requests_by_event_kind.insert(event_kind, request).is_some() {
                return Err(StructuralCommitmentApplicationError::DuplicateEventKind(
                    event_kind,
                ));
            }
        }
        Ok(Self {
            requests_by_event_kind,
        })
    }

    pub fn submit_committed(
        &self,
        commitments: &[StructuralCommitment],
        boundary: &FeederSender,
    ) -> Result<usize, StructuralCommitmentApplicationError> {
        let requests = commitments
            .iter()
            .map(|commitment| {
                self.requests_by_event_kind
                    .get(&commitment.event_kind())
                    .ok_or(StructuralCommitmentApplicationError::UnknownEventKind(
                        commitment.event_kind(),
                    ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        for request in requests {
            boundary.submit_boundary(request.clone())?;
        }
        Ok(commitments.len())
    }

    /// Read-only lookup of the pre-admitted structural request for a sealed
    /// event_kind (7.5 semantic projection). Does not mint or alter authority.
    pub fn request_for_event_kind(&self, event_kind: u32) -> Option<&BoundaryRequest> {
        self.requests_by_event_kind.get(&event_kind)
    }
}

#[derive(Debug, Error)]
pub enum StructuralCommitmentApplicationError {
    #[error("duplicate pre-admitted structural event kind {0}")]
    DuplicateEventKind(u32),
    #[error("structural commitment has no pre-admitted request for event kind {0}")]
    UnknownEventKind(u32),
    #[error(transparent)]
    Feeder(#[from] FeederError),
}

/// Apply every `BoundaryRequest` to the authoritative tree + slot table.
///
/// `values_shadow` must be sized `n_slots × n_dims` where `n_slots`
/// matches the capacity the `WorldGpuState` was built with. If `AddChild`
/// pushes the allocator past that capacity, the new slot's row is written
/// to a position outside `values_shadow` — the caller must catch this
/// before flushing. For Week 3 testing the fixture pre-allocates headroom.
pub fn apply_structural_mutations(
    requests: Vec<BoundaryRequest>,
    root: &mut SimRuntimeTree,
    allocator: &mut SlotAllocator,
    registry: &mut DimensionRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    destination_generation: GenerationStamp,
    lifecycle_admission: &mut OverlayLifecycleAdmissionState,
    growth_commits: &BTreeMap<SimThingId, VerifiedGrowthResidencyCommit>,
) -> MaintainerOutcome {
    let mut grant_authority = GrantDisbursementBoundaryAuthority::default();
    apply_structural_mutations_with_grant_authority(
        requests,
        root,
        allocator,
        registry,
        values_shadow,
        n_dims,
        node_paths,
        destination_generation,
        lifecycle_admission,
        growth_commits,
        &mut grant_authority,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn apply_structural_mutations_with_grant_authority(
    requests: Vec<BoundaryRequest>,
    root: &mut SimRuntimeTree,
    allocator: &mut SlotAllocator,
    registry: &mut DimensionRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    destination_generation: GenerationStamp,
    lifecycle_admission: &mut OverlayLifecycleAdmissionState,
    growth_commits: &BTreeMap<SimThingId, VerifiedGrowthResidencyCommit>,
    grant_authority: &mut GrantDisbursementBoundaryAuthority,
) -> MaintainerOutcome {
    let mut out = MaintainerOutcome::default();
    let root = root.inner_mut();

    for req in requests {
        match req {
            BoundaryRequest::AddChild { parent, child } => {
                let commit = growth_commits.get(&child.id).copied();
                apply_add_child(
                    root,
                    allocator,
                    registry,
                    values_shadow,
                    n_dims,
                    parent,
                    child,
                    node_paths,
                    commit,
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
                apply_reparent(root, allocator, child, new_parent, node_paths, &mut out);
            }
            BoundaryRequest::AttachOverlay {
                target,
                overlay,
                source_generation,
            } => {
                if is_protected_grant_overlay(registry, &overlay)
                    && !grant_authority.consume_attach(target, &overlay)
                {
                    out.rejected_grant_lane_authority += 1;
                    continue;
                }
                let oid = overlay.id;
                if lifecycle_admission
                    .admit_routed_overlay(
                        target,
                        oid,
                        &overlay.lifecycle,
                        source_generation,
                        destination_generation,
                    )
                    .is_err()
                {
                    out.rejected_overlay_lifecycle += 1;
                    continue;
                }
                match simthing_core::deliver_routed_overlay(root, target, overlay) {
                    Ok(_) => {
                        out.overlays += 1;
                        out.overlays_attached.push((target, oid));
                    }
                    Err(_) => out.rejected_unknown_target += 1,
                }
            }
            BoundaryRequest::ActivateOverlay { target, overlay_id } => {
                if lookup_node(root, target, node_paths)
                    .and_then(|node| {
                        node.overlays
                            .iter()
                            .find(|overlay| overlay.id == overlay_id)
                    })
                    .is_some_and(|overlay| is_protected_grant_overlay(registry, overlay))
                {
                    out.rejected_grant_lane_authority += 1;
                    continue;
                }
                match activate_overlay(
                    root,
                    target,
                    overlay_id,
                    node_paths,
                    destination_generation,
                    lifecycle_admission,
                ) {
                    OverlayTransition::Changed => {
                        out.overlay_activations += 1;
                        out.overlays_activated.push((target, overlay_id));
                    }
                    OverlayTransition::NoOp => {}
                    OverlayTransition::Missing => out.rejected_unknown_target += 1,
                    OverlayTransition::RejectedLifecycle => {
                        out.rejected_overlay_lifecycle += 1;
                    }
                }
            }
            BoundaryRequest::SuspendOverlay { target, overlay_id } => {
                if lookup_node(root, target, node_paths)
                    .and_then(|node| {
                        node.overlays
                            .iter()
                            .find(|overlay| overlay.id == overlay_id)
                    })
                    .is_some_and(|overlay| is_protected_grant_overlay(registry, overlay))
                    && !grant_authority.consume_suspend(target, overlay_id)
                {
                    out.rejected_grant_lane_authority += 1;
                    continue;
                }
                match suspend_overlay(
                    root,
                    target,
                    overlay_id,
                    node_paths,
                    destination_generation,
                    lifecycle_admission,
                ) {
                    OverlayTransition::Changed => {
                        out.overlay_suspensions += 1;
                        out.overlays_suspended.push((target, overlay_id));
                    }
                    OverlayTransition::NoOp => {}
                    OverlayTransition::Missing => out.rejected_unknown_target += 1,
                    OverlayTransition::RejectedLifecycle => {
                        out.rejected_overlay_lifecycle += 1;
                    }
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
    mut child: SimThing,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    commit: Option<VerifiedGrowthResidencyCommit>,
    out: &mut MaintainerOutcome,
) {
    let Some(commit) = commit else {
        out.rejected_growth_entitlement += 1;
        return;
    };
    prepare_fission_clone_sources_subtree(&mut child, registry);

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
    if allocator
        .realize_growth_subtree(parent, attached, commit.commit())
        .is_err()
    {
        parent.children.pop();
        out.rejected_unknown_target += 1;
        return;
    }
    let attached_request = parent
        .attached_child_residency_request(attached)
        .expect("just-pushed subtree is an attached direct child");

    // Project the attached subtree's semantic properties into the shadow.
    // Rows are zeroed first so absent properties do not inherit stale slot data.
    if let Some(attached) = find_node(root, new_ids[0]) {
        project_subtree_to_shadow(
            attached,
            attached_request,
            allocator,
            registry,
            values_shadow,
            n_dims,
            out,
        );
    }
    out.adds += 1;
}

fn collect_subtree_ids(node: &SimThing, out: &mut Vec<SimThingId>) {
    out.push(node.id);
    for c in &node.children {
        collect_subtree_ids(c, out);
    }
}

fn project_subtree_to_shadow(
    node: &SimThing,
    request: ObjectResidencyRequest,
    allocator: &SlotAllocator,
    registry: &DimensionRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
    out: &mut MaintainerOutcome,
) {
    if let Some(residency) = allocator.residency_for(&request) {
        let slot = residency.slot();
        let base = slot.as_usize() * n_dims;
        let end = base + n_dims;
        if end <= values_shadow.len() {
            values_shadow[base..end].fill(0.0);
            for (&pid, pval) in &node.properties {
                if !registry.is_active(pid) {
                    continue;
                }
                let prop = registry.property(pid);
                let range = registry.column_range(pid);
                let src_len = prop.layout.stride().min(pval.lane_count());
                let dst = base + range.start;
                if dst + src_len <= values_shadow.len() {
                    values_shadow[dst..dst + src_len]
                        .copy_from_slice(&pval.raw_lanes_for_serialization()[..src_len]);
                }
            }
        }
        out.allocated.push(node.id);
    }

    for child in &node.children {
        let request = node
            .attached_child_residency_request(child)
            .expect("tree traversal holds the attached direct child");
        project_subtree_to_shadow(
            child,
            request,
            allocator,
            registry,
            values_shadow,
            n_dims,
            out,
        );
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

    let mut removed_ids = Vec::new();
    collect_subtree_ids(&subtree, &mut removed_ids);
    for slot in allocator.release_subtree(&subtree) {
        zero_shadow_row(values_shadow, n_dims, slot.raw());
    }
    out.tombstoned.extend(removed_ids);
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
    allocator: &mut SlotAllocator,
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
    let Some(simthing_core::ObjectResidencyRelation::ChildOf(old_parent)) =
        allocator.relation_of(child)
    else {
        out.rejected_unknown_target += 1;
        return;
    };

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
    let Some(parent) = lookup_node_mut(root, new_parent, None) else {
        // Restore the exact old attachment if the preflighted destination
        // disappeared or its cached path was invalidated by detachment.
        out.rejected_unknown_target += 1;
        find_node_mut(root, old_parent)
            .expect("resident child's old parent remains in the tree")
            .add_child(subtree);
        return;
    };
    parent.add_child(subtree);
    let attached = parent.children.last().expect("reparented child attached");
    let request = parent
        .attached_child_residency_request(attached)
        .expect("reparented subtree is attached before residency rebind");
    if allocator.reparent_residency(request).is_err() {
        let subtree = detach_subtree(root, child).expect("failed reparent remains attached");
        find_node_mut(root, old_parent)
            .expect("failed reparent restores the old attachment")
            .add_child(subtree);
        out.rejected_unknown_target += 1;
        return;
    }
    out.reparents += 1;
    out.reparented.push((child, new_parent));
}

fn subtree_contains(node: &SimThing, target: SimThingId) -> bool {
    if node.id == target {
        return true;
    }
    node.children.iter().any(|c| subtree_contains(c, target))
}

// ── Tree helpers ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OverlayTransition {
    Changed,
    NoOp,
    Missing,
    RejectedLifecycle,
}

fn activate_overlay(
    root: &mut SimThing,
    target: SimThingId,
    overlay_id: OverlayId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    destination_generation: GenerationStamp,
    lifecycle_admission: &mut OverlayLifecycleAdmissionState,
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
    if lifecycle_admission
        .activate_overlay(target, overlay_id, &when_activated, destination_generation)
        .is_err()
    {
        return OverlayTransition::RejectedLifecycle;
    }
    overlay.lifecycle = *when_activated;
    OverlayTransition::Changed
}

fn suspend_overlay(
    root: &mut SimThing,
    target: SimThingId,
    overlay_id: OverlayId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    destination_generation: GenerationStamp,
    lifecycle_admission: &mut OverlayLifecycleAdmissionState,
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
    lifecycle_admission.suspend_overlay(
        target,
        overlay_id,
        &active_lifecycle,
        destination_generation,
    );
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
    use crate::sim_runtime_tree::SimRuntimeTree;
    use simthing_core::{
        prepare_fission_clone_sources_subtree, DimensionRegistry, Direction, DissolveCondition,
        FissionTemplate, FissionThreshold, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
        OverlaySource, PropertyTransformDelta, PropertyValue, SimProperty, SimPropertyId, SimThing,
        SimThingKind, SimThingKindTag, SlotIndex, SubFieldRole, TransformOp,
        FISSION_CLONE_SOURCE_PROPERTY_ID,
    };
    use simthing_feeder::BoundaryRequest;
    use simthing_gpu::{ProvisionalResidencyEntitlement, ResidencyExtent, SlotAllocator};

    fn fixture() -> (DimensionRegistry, SlotAllocator, SimRuntimeTree) {
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut root = SimThing::new(SimThingKind::World, 0);
        let loc = SimThing::new(SimThingKind::Location, 0);
        root.add_child(loc);
        let mut alloc = SlotAllocator::new();
        alloc.install_initial_tree(&root);
        (reg, alloc, SimRuntimeTree::admit(root))
    }

    #[test]
    fn row_slot_object_semantics_structural_routes_preserve_one_authority() {
        let mut registry = DimensionRegistry::new();
        registry.register(SimProperty::simple("core", "loyalty", 0));

        let mut root = SimThing::new(SimThingKind::World, 0);
        let parent_a = SimThing::new(SimThingKind::Location, 0);
        let parent_a_id = parent_a.id;
        let parent_b = SimThing::new(SimThingKind::Location, 0);
        let parent_b_id = parent_b.id;
        root.add_child(parent_a);
        root.add_child(parent_b);

        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&root);
        let mut runtime = SimRuntimeTree::admit(root);
        let n_dims = registry.total_columns;
        let mut shadow = vec![0.0; 16 * n_dims];
        let mut lifecycle_admission = OverlayLifecycleAdmissionState::default();

        let child = SimThing::new(SimThingKind::Cohort, 1);
        let child_id = child.id;
        allocator
            .declare_root_residency_extent(
                runtime.inner().id,
                ResidencyExtent::try_new(0, 16).unwrap(),
            )
            .unwrap();
        let mut schedule = simthing_core::IntegrationSchedule::new();
        let commit = allocator
            .realize_unattached_growth_residency(
                ProvisionalResidencyEntitlement::try_new(
                    runtime.inner().id,
                    child_id,
                    1,
                    1,
                    GenerationStamp::new(0),
                )
                .unwrap(),
                parent_a_id,
                GenerationStamp::new(0),
                &mut schedule,
            )
            .unwrap();
        let growth_commits = BTreeMap::from([(
            child_id,
            VerifiedGrowthResidencyCommit::unchecked_for_test(commit),
        )]);
        let added = apply_structural_mutations(
            vec![BoundaryRequest::AddChild {
                parent: parent_a_id,
                child,
            }],
            &mut runtime,
            &mut allocator,
            &mut registry,
            &mut shadow,
            n_dims,
            None,
            GenerationStamp::new(0),
            &mut lifecycle_admission,
            &growth_commits,
        );
        assert_eq!(added.allocated, vec![child_id]);
        let stable_slot = allocator.slot_of(child_id).unwrap();
        assert_eq!(
            allocator.relation_of(child_id),
            Some(simthing_core::ObjectResidencyRelation::ChildOf(parent_a_id))
        );

        let reparented = apply_structural_mutations(
            vec![BoundaryRequest::Reparent {
                child: child_id,
                new_parent: parent_b_id,
            }],
            &mut runtime,
            &mut allocator,
            &mut registry,
            &mut shadow,
            n_dims,
            None,
            GenerationStamp::new(0),
            &mut lifecycle_admission,
            &BTreeMap::new(),
        );
        assert_eq!(reparented.reparented, vec![(child_id, parent_b_id)]);
        assert_eq!(allocator.slot_of(child_id), Some(stable_slot));
        assert_eq!(
            allocator.relation_of(child_id),
            Some(simthing_core::ObjectResidencyRelation::ChildOf(parent_b_id))
        );

        let removed = apply_structural_mutations(
            vec![BoundaryRequest::Remove { target: child_id }],
            &mut runtime,
            &mut allocator,
            &mut registry,
            &mut shadow,
            n_dims,
            None,
            GenerationStamp::new(0),
            &mut lifecycle_admission,
            &BTreeMap::new(),
        );
        assert_eq!(removed.tombstoned, vec![child_id]);
        assert!(allocator.slot_of(child_id).is_none());
        assert!(allocator.relation_of(child_id).is_none());
        assert!(!allocator.is_live(stable_slot));
    }
}
