//! SlotAllocator — stable mapping between `SimThingId` and a dense slot index.
//!
//! Every SimThing that lives in the GPU buffer occupies a row at some
//! `slot_idx`. Slot assignments are append-only within a session; when a
//! SimThing dissolves, its slot is tombstoned and made available for the
//! next alloc, mirroring the column tombstone strategy in `DimensionRegistry`.
//!
//! Slot identity law (Tier-2 amendment; StemThing §3.1 shape (a), 6.4
//! SLOT-LOGICAL-IDENTITY-0): `SlotIndex` is the stable LOGICAL identity of a
//! SimThing's row — stable within an epoch; rebindable only at a recorded
//! boundary remap ([`SlotAllocator::epoch_rebind`], the one binding table,
//! the one `AnchorLocusRemap` history). Between epochs there is zero
//! per-access indirection: bindings are baked into uploaded artifacts, which
//! is what lets transform-matrix patches be delta uploads rather than full
//! rewrites. Physical row allocation/recycling is allocator-private state
//! behind this table; no production ordering or semantics may depend on it.
//!
//! Public slot parameters use [`SlotIndex`] — bare `u32` slot identity is
//! uncompilable at this boundary:
//!
//! ```compile_fail,E0308
//! use simthing_core::SimThingId;
//! use simthing_kernel::SlotAllocator;
//!
//! fn slot_allocator_rejects_raw_integer_slot_compile_fail(
//!     alloc: &SlotAllocator,
//!     slot: u32,
//! ) {
//!     let _ = alloc.owner_of(slot);
//! }
//! ```

use simthing_core::{
    derive_epoch_rebind_section, AnchorRemapSection, AnchoredLocusMap, BindingTableSnapshot,
    ObjectResidencyRelation, ObjectResidencyRelease, ObjectResidencyRequest, RemapSubject,
    SimThing, SimThingId, SlotIndex,
};
use std::collections::{HashMap, HashSet};

/// Bake one `EpochRebind` section into a slot-major values plane: every
/// `ObjectRow` record moves its whole row from its pre-rebind physical
/// position to its post-rebind one; columns are untouched by construction
/// (the subject carries none). Rows vacated and not re-landed-on are zeroed,
/// so the result is an exact permutation of live rows with no residue. This
/// is boundary-upload baking — the reason per-access indirection between
/// epochs stays at zero.
pub fn apply_epoch_rebind_to_values(
    values: &[f32],
    n_cols: usize,
    section: &AnchorRemapSection,
) -> Vec<f32> {
    let mut out = values.to_vec();
    if n_cols == 0 {
        return out;
    }
    let mut from_rows: HashSet<u32> = HashSet::new();
    let mut to_rows: HashSet<u32> = HashSet::new();
    for remap in &section.remaps {
        if remap.subject != RemapSubject::ObjectRow {
            continue;
        }
        let (Some(from), Some(to)) = (remap.from_slot, remap.to_slot) else {
            continue;
        };
        from_rows.insert(from.raw());
        to_rows.insert(to.raw());
        let from = from.raw() as usize * n_cols;
        let to = to.raw() as usize * n_cols;
        if from + n_cols <= values.len() && to + n_cols <= out.len() {
            out[to..to + n_cols].copy_from_slice(&values[from..from + n_cols]);
        }
    }
    for vacated in from_rows.difference(&to_rows) {
        let start = *vacated as usize * n_cols;
        if start + n_cols <= out.len() {
            out[start..start + n_cols].fill(0.0);
        }
    }
    out
}

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
    #[error("epoch rebind assignment names non-resident object {object:?}")]
    RebindUnknownObject { object: SimThingId },
    #[error("epoch rebind assignment omits live object {object:?}")]
    RebindOmitsLiveObject { object: SimThingId },
    #[error("epoch rebind assigns two objects to slot {slot:?}")]
    RebindSlotCollision { slot: SlotIndex },
    #[error(
        "epoch rebind targets slot {slot:?} beyond current capacity (growth is SlotCapacityGrow business)"
    )]
    RebindBeyondCapacity { slot: SlotIndex },
    #[error("epoch rebind targets exclusive-reserved gap slot {slot:?}")]
    RebindOntoReservedGap { slot: SlotIndex },
    #[error("epoch rebind section refused: {detail}")]
    RebindSectionRefused { detail: &'static str },
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

    /// Snapshot THE binding table (`id → slot`) — the demand source for
    /// `EpochRebind` sections. Zero-anchor objects are present here even
    /// though no anchored-locus snapshot ever names them.
    pub fn binding_table_snapshot(&self) -> BindingTableSnapshot {
        self.by_id
            .iter()
            .map(|(&id, &slot)| (id, SlotIndex::new(slot)))
            .collect()
    }

    /// SLOT-LOGICAL-IDENTITY-0 barrier-only epoch rebind on the ONE binding
    /// table. `assignment` names every live object's post-rebind slot; the
    /// rebind is capacity-preserving (growth is `SlotCapacityGrow` business),
    /// may not touch exclusive-reserved gap rows, and may not create or
    /// destroy live rows. Returns the canonical `EpochRebind` section —
    /// exactly one `ObjectRow` record per moved live row, derived from the
    /// pre/post binding-table snapshots, never from anchored loci.
    ///
    /// Callers own the generation barrier: every uploaded slot-bearing
    /// artifact must be rebuilt from the post-rebind table before the next
    /// dispatch (zero per-access indirection between epochs).
    pub fn epoch_rebind(
        &mut self,
        assignment: &BindingTableSnapshot,
        pre_loci: &AnchoredLocusMap,
        post_loci: &AnchoredLocusMap,
    ) -> Result<AnchorRemapSection, SlotAllocError> {
        let capacity = self.slot_owners.len() as u32;
        let mut targets: HashSet<u32> = HashSet::with_capacity(assignment.len());
        for (&id, &slot) in assignment {
            if !self.by_id.contains_key(&id) {
                return Err(SlotAllocError::RebindUnknownObject { object: id });
            }
            if slot.raw() >= capacity {
                return Err(SlotAllocError::RebindBeyondCapacity { slot });
            }
            if self.exclusive_reserved.contains(&slot.raw()) {
                return Err(SlotAllocError::RebindOntoReservedGap { slot });
            }
            if !targets.insert(slot.raw()) {
                return Err(SlotAllocError::RebindSlotCollision { slot });
            }
        }
        for &id in self.by_id.keys() {
            if !assignment.contains_key(&id) {
                return Err(SlotAllocError::RebindOmitsLiveObject { object: id });
            }
        }

        let pre = self.binding_table_snapshot();
        let mut owners: Vec<Option<SimThingId>> = vec![None; capacity as usize];
        for (&id, &slot) in assignment {
            owners[slot.raw() as usize] = Some(id);
        }
        self.slot_owners = owners;
        self.by_id = assignment
            .iter()
            .map(|(&id, &slot)| (id, slot.raw()))
            .collect();
        // Deterministic free-list rebuild: pop() hands out the LOWEST
        // tombstoned non-reserved row first.
        self.free = (0..capacity)
            .rev()
            .filter(|raw| {
                self.slot_owners[*raw as usize].is_none() && !self.exclusive_reserved.contains(raw)
            })
            .collect();
        let post = self.binding_table_snapshot();
        derive_epoch_rebind_section(&pre, &post, pre_loci, post_loci).map_err(|refused| {
            SlotAllocError::RebindSectionRefused {
                detail: refused.detail,
            }
        })
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

    #[test]
    fn epoch_rebind_moves_rows_exact_once_and_rebuilds_the_one_table() {
        use simthing_core::AnchoredLocusMap;
        let root = canonical_fixture();
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let pre = alloc.binding_table_snapshot();
        assert!(pre.len() >= 3, "fixture yields several live rows");

        // Reverse the live rows onto the same physical capacity — a pure
        // permutation; every row moves except any fixed point.
        let mut slots: Vec<SlotIndex> = pre.values().copied().collect();
        slots.sort();
        let mut assignment = BindingTableSnapshot::new();
        let mut ordered: Vec<_> = pre.iter().map(|(&id, &s)| (id, s)).collect();
        ordered.sort_by_key(|&(_, s)| s);
        for (i, &(id, _)) in ordered.iter().enumerate() {
            assignment.insert(id, slots[slots.len() - 1 - i]);
        }
        let loci = AnchoredLocusMap::new();
        let section = alloc.epoch_rebind(&assignment, &loci, &loci).unwrap();

        let moved = pre.iter().filter(|(id, s)| assignment[id] != **s).count();
        assert_eq!(
            section.remaps.len(),
            moved,
            "exactly one record per moved row"
        );
        assert!(section
            .remaps
            .iter()
            .all(|r| r.subject == RemapSubject::ObjectRow));
        // The ONE binding table serves the post-rebind truth.
        for (&id, &slot) in &assignment {
            assert_eq!(alloc.slot_of(id), Some(slot));
            assert_eq!(alloc.owner_of(slot), Some(id));
        }
    }

    #[test]
    fn epoch_rebind_rejects_collision_growth_reserved_and_row_churn() {
        use simthing_core::AnchoredLocusMap;
        let root = canonical_fixture();
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let reserved = alloc.reserve_exclusive_gap_block(1)[0];
        let pre = alloc.binding_table_snapshot();
        let loci = AnchoredLocusMap::new();
        let first = *pre.keys().next().unwrap();

        // Collision: two ids onto one slot.
        let mut collide = pre.clone();
        let clash = *collide.values().next().unwrap();
        for slot in collide.values_mut() {
            *slot = clash;
        }
        assert!(matches!(
            alloc.epoch_rebind(&collide, &loci, &loci),
            Err(SlotAllocError::RebindSlotCollision { .. })
        ));

        // Growth: beyond current capacity.
        let mut grow = pre.clone();
        grow.insert(first, SlotIndex::new(alloc.capacity() as u32));
        assert!(matches!(
            alloc.epoch_rebind(&grow, &loci, &loci),
            Err(SlotAllocError::RebindBeyondCapacity { .. })
        ));

        // Reserved gap row is untouchable.
        let mut onto_gap = pre.clone();
        onto_gap.insert(first, reserved);
        assert!(matches!(
            alloc.epoch_rebind(&onto_gap, &loci, &loci),
            Err(SlotAllocError::RebindOntoReservedGap { .. })
        ));

        // Row churn: omitting a live object is refused.
        let mut omit = pre.clone();
        omit.remove(&first);
        assert!(matches!(
            alloc.epoch_rebind(&omit, &loci, &loci),
            Err(SlotAllocError::RebindOmitsLiveObject { .. })
        ));
    }

    #[test]
    fn epoch_rebind_values_baking_is_an_exact_row_permutation() {
        let a = SimThingId::from_session_raw(70);
        let b = SimThingId::from_session_raw(71);
        let section = AnchorRemapSection::with_remaps(
            simthing_core::AnchorRemapOperation::EpochRebind,
            vec![
                simthing_core::AnchorLocusRemap::object_row(
                    a,
                    SlotIndex::new(0),
                    SlotIndex::new(2),
                ),
                simthing_core::AnchorLocusRemap::object_row(
                    b,
                    SlotIndex::new(2),
                    SlotIndex::new(0),
                ),
            ],
        );
        let values = vec![1.0, 2.0, 0.0, 0.0, 5.0, 6.0];
        let baked = apply_epoch_rebind_to_values(&values, 2, &section);
        assert_eq!(baked, vec![5.0, 6.0, 0.0, 0.0, 1.0, 2.0]);

        // One-way move vacates and zeroes the source row.
        let one_way = AnchorRemapSection::with_remaps(
            simthing_core::AnchorRemapOperation::EpochRebind,
            vec![simthing_core::AnchorLocusRemap::object_row(
                a,
                SlotIndex::new(0),
                SlotIndex::new(1),
            )],
        );
        let baked = apply_epoch_rebind_to_values(&[1.0, 2.0, 0.0, 0.0], 2, &one_way);
        assert_eq!(baked, vec![0.0, 0.0, 1.0, 2.0]);
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
