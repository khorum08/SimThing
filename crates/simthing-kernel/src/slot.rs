//! SlotAllocator — stable mapping between `SimThingId` and a dense slot index.
//!
//! Every SimThing that lives in the GPU buffer occupies a row at some
//! `slot_idx`. Slot assignments are append-only within a session; when a
//! SimThing dissolves, its slot is tombstoned and made available for the
//! next alloc, mirroring the column tombstone strategy in `DimensionRegistry`.
//!
//! Slot indices are stable for the lifetime of a SimThing — once allocated,
//! a SimThing's slot does not change. This is what lets transform-matrix
//! patches be delta uploads rather than full rewrites.
//!
//! Public slot parameters use [`SlotIndex`] — bare `u32` slot identity is
//! uncompilable at this boundary:
//!
//! ```compile_fail
//! use simthing_core::SimThingId;
//! use simthing_gpu::SlotAllocator;
//!
//! fn slot_allocator_rejects_raw_integer_slot_compile_fail(
//!     alloc: &SlotAllocator,
//!     slot: u32,
//! ) {
//!     let _ = alloc.owner_of(slot);
//! }
//! ```

use simthing_core::{
    ObjectResidencyRelation, ObjectResidencyRelease, ObjectResidencyRequest, SimThing, SimThingId,
    SlotIndex,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SlotAllocError {
    #[error("cannot reserve adjacent gap at slot {slot:?}: occupied by live SimThing")]
    AdjacentOccupied { slot: SlotIndex },
    #[error("contiguous slot extension at {slot:?} blocked by exclusive reserved gap slot")]
    ContiguityBlockedByGap { slot: SlotIndex },
    #[error("child {object:?} requested residency under non-resident parent {parent:?}")]
    MissingResidentParent {
        object: SimThingId,
        parent: SimThingId,
    },
    #[error("object {object:?} is not the attached direct child of {parent:?}")]
    ChildNotAttached {
        object: SimThingId,
        parent: SimThingId,
    },
    #[error("allocator already admits root {existing:?}; cannot admit second root {requested:?}")]
    RootAlreadyResident {
        existing: SimThingId,
        requested: SimThingId,
    },
    #[error(
        "object {object:?} is already resident as {existing:?}, not requested relation {requested:?}"
    )]
    RelationConflict {
        object: SimThingId,
        existing: ObjectResidencyRelation,
        requested: ObjectResidencyRelation,
    },
    #[error("object {object:?} has a slot but no admitted object residency relation")]
    UnboundSidecarSlot { object: SimThingId },
    #[error("object {object:?} is not resident")]
    ObjectNotResident { object: SimThingId },
    #[error("reparent request for {object:?} must carry ChildOf, not Root")]
    ReparentRequiresParent { object: SimThingId },
}

/// Kernel-minted proof that one object relation owns one dense row.
///
/// Fields are private: callers can inspect the admitted relation and slot but
/// cannot manufacture a production residency proof from a sidecar slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectResidency {
    object: SimThingId,
    relation: ObjectResidencyRelation,
    slot: SlotIndex,
}

impl ObjectResidency {
    pub fn object(self) -> SimThingId {
        self.object
    }

    pub fn relation(self) -> ObjectResidencyRelation {
        self.relation
    }

    pub fn slot(self) -> SlotIndex {
        self.slot
    }
}

#[derive(Clone, Debug, Default)]
pub struct SlotAllocator {
    /// Owner of each slot index. `None` = tombstoned, available for reuse.
    slot_owners: Vec<Option<SimThingId>>,
    /// Reverse lookup: SimThingId → slot index.
    by_id: HashMap<SimThingId, u32>,
    /// LIFO stack of tombstoned slots awaiting reuse.
    free: Vec<u32>,
    /// Tombstoned slots held for arena-participant gap pools — excluded from `free`.
    exclusive_reserved: HashSet<u32>,
    /// Admitted object relation for each production-resident row.
    ///
    /// A test-only injected unbound row intentionally does not populate this
    /// table and therefore cannot satisfy [`Self::residency_for`].
    relations: HashMap<SimThingId, ObjectResidencyRelation>,
}

impl SlotAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    fn alloc_slot(&mut self, id: SimThingId) -> SlotIndex {
        if let Some(&existing) = self.by_id.get(&id) {
            return SlotIndex::new(existing);
        }
        let slot = match self.free.pop() {
            Some(s) => s,
            None => {
                let s = self.slot_owners.len() as u32;
                self.slot_owners.push(None);
                s
            }
        };
        self.slot_owners[slot as usize] = Some(id);
        self.by_id.insert(id, slot);
        SlotIndex::new(slot)
    }

    #[cfg(test)]
    fn inject_unbound_row_for_escaped_bug_referee(&mut self, id: SimThingId) -> SlotIndex {
        self.alloc_slot(id)
    }

    /// Execute one object-issued root/child relation and mint its stable row.
    pub fn execute_residency(
        &mut self,
        request: ObjectResidencyRequest,
    ) -> Result<ObjectResidency, SlotAllocError> {
        let object = request.object();
        let relation = request.relation();
        self.validate_residency_request(object, relation)?;
        let slot = self.alloc_slot(object);
        self.relations.insert(object, relation);
        Ok(ObjectResidency {
            object,
            relation,
            slot,
        })
    }

    /// Rebind an already-resident object to a new structural parent while
    /// preserving its stable row identity.
    pub fn reparent_residency(
        &mut self,
        request: ObjectResidencyRequest,
    ) -> Result<ObjectResidency, SlotAllocError> {
        let object = request.object();
        let relation = request.relation();
        let ObjectResidencyRelation::ChildOf(parent) = relation else {
            return Err(SlotAllocError::ReparentRequiresParent { object });
        };
        if !self.relations.contains_key(&parent) {
            return Err(SlotAllocError::MissingResidentParent { object, parent });
        }
        let Some(slot) = self.slot_of(object) else {
            return Err(SlotAllocError::ObjectNotResident { object });
        };
        if !self.relations.contains_key(&object) {
            return Err(SlotAllocError::UnboundSidecarSlot { object });
        }
        self.relations.insert(object, relation);
        Ok(ObjectResidency {
            object,
            relation,
            slot,
        })
    }

    /// Resolve a request only when both object identity and parent relation
    /// match the kernel's admitted residency table.
    pub fn residency_for(&self, request: &ObjectResidencyRequest) -> Option<ObjectResidency> {
        let object = request.object();
        let relation = request.relation();
        if self.relations.get(&object).copied()? != relation {
            return None;
        }
        Some(ObjectResidency {
            object,
            relation,
            slot: self.slot_of(object)?,
        })
    }

    /// Resolve the allocator's current proof for an already-admitted object.
    ///
    /// Unlike [`Self::slot_of`], this rejects raw sidecar allocations because
    /// they have no object-issued relation.
    pub fn residency_of(&self, object: SimThingId) -> Option<ObjectResidency> {
        let relation = self.relations.get(&object).copied()?;
        Some(ObjectResidency {
            object,
            relation,
            slot: self.slot_of(object)?,
        })
    }

    pub fn relation_of(&self, object: SimThingId) -> Option<ObjectResidencyRelation> {
        self.relations.get(&object).copied()
    }

    /// Execute the object's release request and retire both relation and row.
    fn release_residency(&mut self, request: ObjectResidencyRelease) -> Option<ObjectResidency> {
        let object = request.object();
        let relation = self.relations.remove(&object)?;
        let slot = self.tombstone_slot(object)?;
        Some(ObjectResidency {
            object,
            relation,
            slot,
        })
    }

    /// Tombstone the slot held by `id`. Returns the freed slot index, or
    /// `None` if the id was not allocated. The slot remains indexed in the
    /// GPU buffer but is marked available; its row's float values are not
    /// auto-cleared — callers that care about residue should zero it.
    fn tombstone_slot(&mut self, id: SimThingId) -> Option<SlotIndex> {
        let slot = self.by_id.remove(&id)?;
        self.slot_owners[slot as usize] = None;
        self.free.push(slot);
        Some(SlotIndex::new(slot))
    }

    fn validate_residency_request(
        &self,
        object: SimThingId,
        relation: ObjectResidencyRelation,
    ) -> Result<(), SlotAllocError> {
        match relation {
            ObjectResidencyRelation::Root => {
                if let Some((&existing, _)) = self
                    .relations
                    .iter()
                    .find(|(_, relation)| **relation == ObjectResidencyRelation::Root)
                {
                    if existing != object {
                        return Err(SlotAllocError::RootAlreadyResident {
                            existing,
                            requested: object,
                        });
                    }
                }
            }
            ObjectResidencyRelation::ChildOf(parent) => {
                if !self.relations.contains_key(&parent) {
                    return Err(SlotAllocError::MissingResidentParent { object, parent });
                }
            }
        }
        if self.by_id.contains_key(&object) && !self.relations.contains_key(&object) {
            return Err(SlotAllocError::UnboundSidecarSlot { object });
        }
        if let Some(existing) = self.relations.get(&object).copied() {
            if existing != relation {
                return Err(SlotAllocError::RelationConflict {
                    object,
                    existing,
                    requested: relation,
                });
            }
        }
        Ok(())
    }

    pub fn slot_of(&self, id: SimThingId) -> Option<SlotIndex> {
        self.by_id.get(&id).copied().map(SlotIndex::new)
    }

    pub fn owner_of(&self, slot: SlotIndex) -> Option<SimThingId> {
        let raw = slot.raw();
        self.slot_owners.get(raw as usize).copied().flatten()
    }

    /// High-water mark — number of slots ever allocated. This is the value
    /// to pass to `WorldGpuState::new(.., n_slots)`.
    pub fn capacity(&self) -> usize {
        self.slot_owners.len()
    }

    /// Currently-live slot count (excludes tombstoned).
    pub fn live_count(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_live(&self, slot: SlotIndex) -> bool {
        let raw = slot.raw();
        self.slot_owners
            .get(raw as usize)
            .map(|o| o.is_some())
            .unwrap_or(false)
    }

    /// Recursively allocate slots for every node in a SimThing tree
    /// (depth-first, root before children). Existing allocations are
    /// preserved by the object-residency door's idempotency.
    pub fn populate_from_tree(&mut self, root: &SimThing) {
        let mut newly_admitted = Vec::new();
        let result = self.populate_requested_subtree(
            root,
            root.root_residency_request(),
            &mut newly_admitted,
        );
        if let Err(error) = result {
            self.rollback_population(newly_admitted);
            panic!("root object residency must admit: {error}");
        }
    }

    /// Admit an attached subtree beneath an already-resident parent.
    pub fn populate_subtree(
        &mut self,
        parent: &SimThing,
        root: &SimThing,
    ) -> Result<(), SlotAllocError> {
        let request = parent.attached_child_residency_request(root).ok_or(
            SlotAllocError::ChildNotAttached {
                object: root.id,
                parent: parent.id,
            },
        )?;
        let mut newly_admitted = Vec::new();
        if let Err(error) = self.populate_requested_subtree(root, request, &mut newly_admitted) {
            self.rollback_population(newly_admitted);
            return Err(error);
        }
        Ok(())
    }

    fn populate_requested_subtree(
        &mut self,
        root: &SimThing,
        request: ObjectResidencyRequest,
        newly_admitted: &mut Vec<SimThingId>,
    ) -> Result<(), SlotAllocError> {
        let was_resident = self.relations.contains_key(&root.id);
        self.execute_residency(request)?;
        if !was_resident {
            newly_admitted.push(root.id);
        }
        for child in &root.children {
            let request = root.attached_child_residency_request(child).ok_or(
                SlotAllocError::ChildNotAttached {
                    object: child.id,
                    parent: root.id,
                },
            )?;
            self.populate_requested_subtree(child, request, newly_admitted)?;
        }
        Ok(())
    }

    fn rollback_population(&mut self, newly_admitted: Vec<SimThingId>) {
        for object in newly_admitted.into_iter().rev() {
            self.relations.remove(&object);
            self.tombstone_slot(object);
        }
    }

    /// Release every row in a detached subtree through object-issued release
    /// requests. Traversal is pre-order to preserve the historical free-list
    /// and tombstone reporting order exactly.
    pub fn release_subtree(&mut self, root: &SimThing) -> Vec<SlotIndex> {
        let mut released = Vec::new();
        self.release_subtree_into(root, &mut released);
        released
    }

    fn release_subtree_into(&mut self, root: &SimThing, released: &mut Vec<SlotIndex>) {
        if let Some(residency) = self.release_residency(root.residency_release_request()) {
            released.push(residency.slot());
        } else if let Some(recovered_unbound) = self.tombstone_slot(root.id) {
            // Defensive escaped-bug recovery: an old/test-injected row that
            // lacks a relation must still be retired when its object subtree
            // detaches. Production cannot create this state.
            released.push(recovered_unbound);
        }
        for child in &root.children {
            self.release_subtree_into(child, released);
        }
    }

    /// True when `slot` is tombstoned and held for a parent's reserved gap pool.
    pub fn is_exclusive_reserved(&self, slot: SlotIndex) -> bool {
        self.exclusive_reserved.contains(&slot.raw())
    }

    /// Extend the high-water mark with `count` exclusively reserved tombstoned
    /// slots (arena-local gap bookkeeping). Returns ascending slot ids. The
    /// block creates no live row and has no relationless public claim door.
    pub fn reserve_exclusive_gap_block(&mut self, count: u32) -> Vec<SlotIndex> {
        if count == 0 {
            return Vec::new();
        }
        let mut slots = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let slot = self.capacity() as u32;
            self.slot_owners.push(None);
            self.exclusive_reserved.insert(slot);
            slots.push(SlotIndex::new(slot));
        }
        slots
    }

    /// Extend the buffer with exclusively reserved tombstoned slots immediately
    /// after `parent_slot`. Prefer [`Self::reserve_exclusive_gap_block`] when
    /// sibling participants occupy the slots after `parent_slot`.
    pub fn reserve_adjacent_gaps_after(
        &mut self,
        parent_slot: SlotIndex,
        count: u32,
    ) -> Result<Vec<SlotIndex>, SlotAllocError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let parent_raw = parent_slot.raw();
        let mut slots = Vec::with_capacity(count as usize);
        for i in 1..=count {
            let slot = parent_raw.saturating_add(i);
            while self.capacity() as u32 <= slot {
                self.slot_owners.push(None);
            }
            if self.is_live(SlotIndex::new(slot)) {
                return Err(SlotAllocError::AdjacentOccupied {
                    slot: SlotIndex::new(slot),
                });
            }
            if let Some(pos) = self.free.iter().position(|&s| s == slot) {
                self.free.remove(pos);
            }
            self.slot_owners[slot as usize] = None;
            self.exclusive_reserved.insert(slot);
            slots.push(SlotIndex::new(slot));
        }
        Ok(slots)
    }

    /// Read-only arena-plan preflight for one contiguous slot after `after_slot`.
    pub fn can_alloc_contiguous_after(
        &self,
        after_slot: SlotIndex,
    ) -> Result<SlotIndex, SlotAllocError> {
        let target = after_slot.raw().saturating_add(1);
        if self.capacity() as u32 <= target {
            return Ok(SlotIndex::new(target));
        }
        if self.is_live(SlotIndex::new(target)) {
            return Err(SlotAllocError::AdjacentOccupied {
                slot: SlotIndex::new(target),
            });
        }
        if self.exclusive_reserved.contains(&target) {
            return Err(SlotAllocError::ContiguityBlockedByGap {
                slot: SlotIndex::new(target),
            });
        }
        Ok(SlotIndex::new(target))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{build_topology, project_tree_to_values};
    use simthing_core::{
        DimensionRegistry, ObjectResidencyRelation, PropertyValue, SimProperty, SimThing,
        SimThingKind, SubFieldRole,
    };

    fn collect_ids(node: &SimThing, ids: &mut Vec<SimThingId>) {
        ids.push(node.id);
        for child in &node.children {
            collect_ids(child, ids);
        }
    }

    fn canonical_fixture() -> SimThing {
        let mut root = SimThing::new(SimThingKind::GameSession, 0);
        let mut world = SimThing::new(SimThingKind::World, 0);
        world.add_child(SimThing::new(SimThingKind::Location, 0));
        world.add_child(SimThing::new(SimThingKind::Location, 0));
        root.add_child(world);
        root.add_child(SimThing::new(SimThingKind::Owner, 0));
        root
    }

    fn uneven_fixture() -> SimThing {
        let mut root = SimThing::new(SimThingKind::Scenario, 0);
        let mut session = SimThing::new(SimThingKind::GameSession, 0);
        let mut location = SimThing::new(SimThingKind::Location, 0);
        let mut fleet = SimThing::new(SimThingKind::Fleet, 0);
        fleet.add_child(SimThing::new(SimThingKind::Cohort, 0));
        location.add_child(fleet);
        session.add_child(location);
        root.add_child(session);
        root
    }

    #[test]
    fn row_slot_object_semantics_layout_identity_oracle_parity() {
        for root in [canonical_fixture(), uneven_fixture()] {
            let mut derived = SlotAllocator::new();
            derived.populate_from_tree(&root);

            let mut ids = Vec::new();
            collect_ids(&root, &mut ids);
            assert_eq!(derived.capacity(), ids.len());
            assert_eq!(derived.live_count(), ids.len());
            for (expected_slot, id) in ids.into_iter().enumerate() {
                assert_eq!(
                    derived.slot_of(id).map(SlotIndex::as_usize),
                    Some(expected_slot)
                );
                assert!(derived.relation_of(id).is_some());
            }
        }
    }

    #[test]
    fn row_slot_object_semantics_sidecar_cannot_project_or_join_topology() {
        let property = SimProperty::simple("referee", "sidecar", 0);
        let layout = property.layout.clone();
        let mut registry = DimensionRegistry::new();
        let property_id = registry.register(property);

        let mut root = SimThing::new(SimThingKind::GameSession, 0);
        let mut child = SimThing::new(SimThingKind::Location, 0);
        let mut value = PropertyValue::from_layout(&layout);
        value.set_role(&SubFieldRole::Amount, &layout, 7.0);
        child.add_property(property_id, value);
        root.add_child(child);

        let mut allocator = SlotAllocator::new();
        let root_residency = allocator
            .execute_residency(root.root_residency_request())
            .unwrap();
        let sidecar_slot =
            allocator.inject_unbound_row_for_escaped_bug_referee(root.children[0].id);
        let child_request = root
            .attached_child_residency_request(&root.children[0])
            .expect("fixture child is attached");

        assert_eq!(root_residency.relation(), ObjectResidencyRelation::Root);
        assert_eq!(
            allocator.execute_residency(child_request),
            Err(SlotAllocError::UnboundSidecarSlot {
                object: root.children[0].id
            })
        );
        let child_request = root
            .attached_child_residency_request(&root.children[0])
            .expect("fixture child remains attached");
        assert!(allocator.residency_for(&child_request).is_none());

        let topology = build_topology(&root, &allocator);
        let root_slot = root_residency.slot().as_usize();
        assert_eq!(
            topology.child_starts[root_slot],
            topology.child_starts[root_slot + 1]
        );
        assert!(topology
            .depth_buckets
            .iter()
            .all(|bucket| { !bucket.contains(&sidecar_slot.raw()) }));

        let n_dims = registry.total_columns;
        let mut values = vec![0.0; allocator.capacity() * n_dims];
        project_tree_to_values(&root, &registry, &allocator, n_dims, &mut values);
        let sidecar_row =
            &values[sidecar_slot.as_usize() * n_dims..(sidecar_slot.as_usize() + 1) * n_dims];
        assert!(sidecar_row.iter().all(|value| value.to_bits() == 0));
    }

    #[test]
    fn row_slot_unattached_child_cannot_mint_or_leak_residency() {
        let property = SimProperty::simple("referee", "unattached", 0);
        let layout = property.layout.clone();
        let mut registry = DimensionRegistry::new();
        let property_id = registry.register(property);

        let root = SimThing::new(SimThingKind::GameSession, 0);
        let mut unattached = SimThing::new(SimThingKind::Location, 0);
        let mut value = PropertyValue::from_layout(&layout);
        value.set_role(&SubFieldRole::Amount, &layout, 11.0);
        unattached.add_property(property_id, value);

        let mut allocator = SlotAllocator::new();
        let root_residency = allocator
            .execute_residency(root.root_residency_request())
            .unwrap();
        let unbound_slot = allocator.inject_unbound_row_for_escaped_bug_referee(unattached.id);

        assert!(root.attached_child_residency_request(&unattached).is_none());
        assert!(allocator.residency_of(unattached.id).is_none());

        let topology = build_topology(&root, &allocator);
        assert!(topology
            .child_indices
            .iter()
            .all(|slot| *slot != unbound_slot.raw()));
        assert_eq!(
            topology.depth_buckets,
            vec![vec![root_residency.slot().raw()]]
        );

        let n_dims = registry.total_columns;
        let mut values = vec![0.0; allocator.capacity() * n_dims];
        project_tree_to_values(&root, &registry, &allocator, n_dims, &mut values);
        let sidecar_row =
            &values[unbound_slot.as_usize() * n_dims..(unbound_slot.as_usize() + 1) * n_dims];
        assert!(sidecar_row.iter().all(|lane| lane.to_bits() == 0));

        assert_eq!(allocator.release_subtree(&unattached), vec![unbound_slot]);
        assert!(allocator.slot_of(unattached.id).is_none());
        assert!(!allocator.is_live(unbound_slot));
    }
}
