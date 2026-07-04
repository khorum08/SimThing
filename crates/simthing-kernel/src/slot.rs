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

use simthing_core::{SimThingId, SlotIndex};
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
    pub fn alloc(&mut self, id: SimThingId) -> SlotIndex {
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

    /// Tombstone the slot held by `id`. Returns the freed slot index, or
    /// `None` if the id was not allocated. The slot remains indexed in the
    /// GPU buffer but is marked available; its row's float values are not
    /// auto-cleared — callers that care about residue should zero it.
    pub fn tombstone(&mut self, id: SimThingId) -> Option<SlotIndex> {
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
    /// preserved due to `alloc`'s idempotency.
    pub fn populate_from_tree(&mut self, root: &simthing_core::SimThing) {
        self.alloc(root.id);
        for child in &root.children {
            self.populate_from_tree(child);
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
    /// or otherwise unavailable — never falls back to non-contiguous `alloc()`.
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
    use simthing_core::{SimThing, SimThingKind};

}
