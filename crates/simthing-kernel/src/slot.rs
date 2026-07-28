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
    #[error("slot {slot:?} is not exclusively reserved for gap consumption")]
    NotExclusiveReserved { slot: SlotIndex },
    #[error("slot {slot:?} is live")]
    SlotLive { slot: SlotIndex },
    #[error("cannot reserve adjacent gap at slot {slot:?}: occupied by live SimThing")]
    AdjacentOccupied { slot: SlotIndex },
    #[error("contiguous slot extension at {slot:?} blocked by exclusive reserved gap slot")]
    ContiguityBlockedByGap { slot: SlotIndex },
    #[error("child {object:?} requested residency under non-resident parent {parent:?}")]
    MissingResidentParent {
        object: SimThingId,
        parent: SimThingId,
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
    /// A raw compatibility allocation intentionally does not populate this
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

    /// Compatibility allocation for test/support code that needs an isolated
    /// row without an object relation.
    ///
    /// This is not the production residency door: it intentionally does not
    /// mint [`ObjectResidency`]. Production tree paths must execute an
    /// object-issued request through [`Self::execute_residency`].
    #[doc(hidden)]
    pub fn alloc_for_oracle_or_rehearsal(&mut self, id: SimThingId) -> SlotIndex {
        self.alloc_slot(id)
    }

    /// Execute one object-issued root/child relation and mint its stable row.
    pub fn execute_residency(
        &mut self,
        request: ObjectResidencyRequest,
    ) -> Result<ObjectResidency, SlotAllocError> {
        let object = request.object();
        let relation = request.relation();
        if let ObjectResidencyRelation::ChildOf(parent) = relation {
            if !self.relations.contains_key(&parent) {
                return Err(SlotAllocError::MissingResidentParent { object, parent });
            }
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
    pub fn residency_for(&self, request: ObjectResidencyRequest) -> Option<ObjectResidency> {
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
    pub fn release_residency(
        &mut self,
        request: ObjectResidencyRelease,
    ) -> Option<ObjectResidency> {
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
    pub fn tombstone(&mut self, id: SimThingId) -> Option<SlotIndex> {
        self.relations.remove(&id);
        self.tombstone_slot(id)
    }

    fn tombstone_slot(&mut self, id: SimThingId) -> Option<SlotIndex> {
        let slot = self.by_id.remove(&id)?;
        self.slot_owners[slot as usize] = None;
        self.free.push(slot);
        Some(SlotIndex::new(slot))
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
        self.execute_residency(root.root_residency_request())
            .expect("root object residency must admit");
        self.populate_children(root);
    }

    /// Admit an attached subtree beneath an already-resident parent.
    pub fn populate_subtree(&mut self, parent: &SimThing, root: &SimThing) {
        self.execute_residency(parent.child_residency_request(root))
            .expect("child object residency must admit");
        self.populate_children(root);
    }

    fn populate_children(&mut self, parent: &SimThing) {
        for child in &parent.children {
            self.execute_residency(parent.child_residency_request(child))
                .expect("child object residency must admit");
            self.populate_children(child);
        }
    }

    /// Release every row in a detached subtree through object-issued release
    /// requests. Traversal is pre-order to preserve the historical free-list
    /// and tombstone reporting order exactly.
    pub fn release_subtree(&mut self, root: &SimThing) -> Vec<ObjectResidency> {
        let mut released = Vec::new();
        self.release_subtree_into(root, &mut released);
        released
    }

    fn release_subtree_into(&mut self, root: &SimThing, released: &mut Vec<ObjectResidency>) {
        if let Some(residency) = self.release_residency(root.residency_release_request()) {
            released.push(residency);
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
    /// slots (arena-local gap block). Returns ascending slot ids. Not placed on
    /// the global LIFO `free` stack until claimed via [`Self::claim_exclusive_slot`].
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

    /// Read-only preflight for [`Self::try_alloc_contiguous_after`].
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

    /// Allocate `id` at exactly `after_slot + 1` for arena-root sibling append.
    ///
    /// Rejects when the target slot is live, exclusively reserved (gap block),
    /// or otherwise unavailable — never falls back to the rehearsal allocator.
    pub fn try_alloc_contiguous_after(
        &mut self,
        after_slot: SlotIndex,
        id: SimThingId,
    ) -> Result<SlotIndex, SlotAllocError> {
        if let Some(&existing) = self.by_id.get(&id) {
            return Ok(SlotIndex::new(existing));
        }
        let target = after_slot.raw().saturating_add(1);
        while self.capacity() as u32 <= target {
            self.slot_owners.push(None);
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
        if let Some(pos) = self.free.iter().position(|&s| s == target) {
            self.free.remove(pos);
        }
        self.slot_owners[target as usize] = Some(id);
        self.by_id.insert(id, target);
        Ok(SlotIndex::new(target))
    }

    /// Assign `id` to an exclusively reserved tombstoned slot.
    pub fn claim_exclusive_slot(
        &mut self,
        slot: SlotIndex,
        id: SimThingId,
    ) -> Result<(), SlotAllocError> {
        if self.by_id.contains_key(&id) {
            return Ok(());
        }
        let raw = slot.raw();
        if !self.exclusive_reserved.contains(&raw) {
            return Err(SlotAllocError::NotExclusiveReserved { slot });
        }
        if self.is_live(slot) {
            return Err(SlotAllocError::SlotLive { slot });
        }
        self.exclusive_reserved.remove(&raw);
        self.slot_owners[raw as usize] = Some(id);
        self.by_id.insert(id, raw);
        Ok(())
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

    fn allocate_legacy_dfs(allocator: &mut SlotAllocator, node: &SimThing) {
        allocator.alloc_for_oracle_or_rehearsal(node.id);
        for child in &node.children {
            allocate_legacy_dfs(allocator, child);
        }
    }

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
            let mut legacy = SlotAllocator::new();
            allocate_legacy_dfs(&mut legacy, &root);

            let mut derived = SlotAllocator::new();
            derived.populate_from_tree(&root);

            let mut ids = Vec::new();
            collect_ids(&root, &mut ids);
            assert_eq!(legacy.capacity(), derived.capacity());
            assert_eq!(legacy.live_count(), derived.live_count());
            for id in ids {
                assert_eq!(legacy.slot_of(id), derived.slot_of(id));
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
        let sidecar_slot = allocator.alloc_for_oracle_or_rehearsal(root.children[0].id);
        let child_request = root.child_residency_request(&root.children[0]);

        assert_eq!(root_residency.relation(), ObjectResidencyRelation::Root);
        assert!(allocator.residency_for(child_request).is_none());

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
}
