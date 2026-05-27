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

use simthing_core::SimThingId;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SlotAllocError {
    #[error("slot {slot} is not exclusively reserved for gap consumption")]
    NotExclusiveReserved { slot: u32 },
    #[error("slot {slot} is live")]
    SlotLive { slot: u32 },
    #[error("cannot reserve adjacent gap at slot {slot}: occupied by live SimThing")]
    AdjacentOccupied { slot: u32 },
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
}

impl SlotAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate (or return existing) slot for the given SimThing.
    /// Idempotent — repeated calls with the same id return the same slot.
    pub fn alloc(&mut self, id: SimThingId) -> u32 {
        if let Some(&existing) = self.by_id.get(&id) {
            return existing;
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
        slot
    }

    /// Tombstone the slot held by `id`. Returns the freed slot index, or
    /// `None` if the id was not allocated. The slot remains indexed in the
    /// GPU buffer but is marked available; its row's float values are not
    /// auto-cleared — callers that care about residue should zero it.
    pub fn tombstone(&mut self, id: SimThingId) -> Option<u32> {
        let slot = self.by_id.remove(&id)?;
        self.slot_owners[slot as usize] = None;
        self.free.push(slot);
        Some(slot)
    }

    pub fn slot_of(&self, id: SimThingId) -> Option<u32> {
        self.by_id.get(&id).copied()
    }

    pub fn owner_of(&self, slot: u32) -> Option<SimThingId> {
        self.slot_owners.get(slot as usize).copied().flatten()
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

    pub fn is_live(&self, slot: u32) -> bool {
        self.slot_owners
            .get(slot as usize)
            .map(|o| o.is_some())
            .unwrap_or(false)
    }

    /// Recursively allocate slots for every node in a SimThing tree
    /// (depth-first, root before children). Existing allocations are
    /// preserved due to `alloc`'s idempotency.
    pub fn populate_from_tree(&mut self, root: &simthing_core::SimThing) {
        self.alloc(root.id);
        for child in &root.children {
            self.populate_from_tree(child);
        }
    }

    /// True when `slot` is tombstoned and held for a parent's reserved gap pool.
    pub fn is_exclusive_reserved(&self, slot: u32) -> bool {
        self.exclusive_reserved.contains(&slot)
    }

    /// Extend the buffer with exclusively reserved tombstoned slots immediately
    /// after `parent_slot`. Returns ascending slot ids. These slots are not placed
    /// on the global LIFO `free` stack until claimed via [`Self::claim_exclusive_slot`].
    pub fn reserve_adjacent_gaps_after(
        &mut self,
        parent_slot: u32,
        count: u32,
    ) -> Result<Vec<u32>, SlotAllocError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut slots = Vec::with_capacity(count as usize);
        for i in 1..=count {
            let slot = parent_slot.saturating_add(i);
            while self.capacity() as u32 <= slot {
                self.slot_owners.push(None);
            }
            if self.is_live(slot) {
                return Err(SlotAllocError::AdjacentOccupied { slot });
            }
            if let Some(pos) = self.free.iter().position(|&s| s == slot) {
                self.free.remove(pos);
            }
            self.slot_owners[slot as usize] = None;
            self.exclusive_reserved.insert(slot);
            slots.push(slot);
        }
        Ok(slots)
    }

    /// Assign `id` to an exclusively reserved tombstoned slot.
    pub fn claim_exclusive_slot(&mut self, slot: u32, id: SimThingId) -> Result<(), SlotAllocError> {
        if self.by_id.contains_key(&id) {
            return Ok(());
        }
        if !self.exclusive_reserved.contains(&slot) {
            return Err(SlotAllocError::NotExclusiveReserved { slot });
        }
        if self.is_live(slot) {
            return Err(SlotAllocError::SlotLive { slot });
        }
        self.exclusive_reserved.remove(&slot);
        self.slot_owners[slot as usize] = Some(id);
        self.by_id.insert(id, slot);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{SimThing, SimThingKind};

    #[test]
    fn alloc_returns_distinct_slots_for_distinct_ids() {
        let mut alloc = SlotAllocator::new();
        let a = SimThing::new(SimThingKind::Cohort, 0).id;
        let b = SimThing::new(SimThingKind::Cohort, 0).id;
        let sa = alloc.alloc(a);
        let sb = alloc.alloc(b);
        assert_ne!(sa, sb);
        assert_eq!(alloc.capacity(), 2);
        assert_eq!(alloc.live_count(), 2);
    }

    #[test]
    fn alloc_is_idempotent() {
        let mut alloc = SlotAllocator::new();
        let id = SimThing::new(SimThingKind::Cohort, 0).id;
        let s1 = alloc.alloc(id);
        let s2 = alloc.alloc(id);
        assert_eq!(s1, s2);
        assert_eq!(alloc.capacity(), 1);
    }

    #[test]
    fn tombstone_makes_slot_reusable_lifo() {
        let mut alloc = SlotAllocator::new();
        let a = SimThing::new(SimThingKind::Cohort, 0).id;
        let b = SimThing::new(SimThingKind::Cohort, 0).id;
        let c = SimThing::new(SimThingKind::Cohort, 0).id;
        let d = SimThing::new(SimThingKind::Cohort, 0).id;

        let sa = alloc.alloc(a); // 0
        let sb = alloc.alloc(b); // 1
        let sc = alloc.alloc(c); // 2
        assert_eq!((sa, sb, sc), (0, 1, 2));

        assert_eq!(alloc.tombstone(b), Some(1));
        assert!(!alloc.is_live(1));
        assert_eq!(alloc.live_count(), 2);
        assert_eq!(alloc.capacity(), 3); // tombstoning doesn't shrink capacity

        let sd = alloc.alloc(d);
        assert_eq!(sd, 1); // reused freed slot
        assert_eq!(alloc.capacity(), 3);
    }

    #[test]
    fn tombstone_returns_none_for_unknown_id() {
        let mut alloc = SlotAllocator::new();
        let ghost = SimThing::new(SimThingKind::Cohort, 0).id;
        assert_eq!(alloc.tombstone(ghost), None);
    }

    #[test]
    fn populate_from_tree_allocates_every_node() {
        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut loc1 = SimThing::new(SimThingKind::Location, 0);
        loc1.add_child(SimThing::new(SimThingKind::Cohort, 0));
        loc1.add_child(SimThing::new(SimThingKind::Cohort, 0));
        let mut loc2 = SimThing::new(SimThingKind::Location, 0);
        loc2.add_child(SimThing::new(SimThingKind::Cohort, 0));
        world.add_child(loc1);
        world.add_child(loc2);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);
        assert_eq!(alloc.capacity(), 6); // world + 2 loc + 3 cohort
        assert_eq!(alloc.live_count(), 6);
    }

    #[test]
    fn slot_owner_round_trips() {
        let mut alloc = SlotAllocator::new();
        let id = SimThing::new(SimThingKind::Cohort, 0).id;
        let slot = alloc.alloc(id);
        assert_eq!(alloc.slot_of(id), Some(slot));
        assert_eq!(alloc.owner_of(slot), Some(id));
    }

    #[test]
    fn owner_of_returns_none_for_tombstoned_slot() {
        let mut alloc = SlotAllocator::new();
        let id = SimThing::new(SimThingKind::Cohort, 0).id;
        let slot = alloc.alloc(id);
        alloc.tombstone(id);
        assert_eq!(alloc.owner_of(slot), None);
    }
}
