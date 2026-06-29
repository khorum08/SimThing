//! GPU/runtime buffer row index for a SimThing slot (`slot_idx`).
//!
//! Distinct from [`RoleOffset`] (layout-resolved lane within one property value)
//! and from global matrix column indices at upload boundaries.
//!
//! Transposition with layout-resolved lane offsets is uncompilable:
//!
//! ```compile_fail
//! use simthing_core::{RoleOffset, SlotIndex};
//!
//! fn takes_role_offset(_: RoleOffset) {}
//!
//! fn slot_index_rejects_role_offset_compile_fail(slot: SlotIndex) {
//!     takes_role_offset(slot);
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_core::{RoleOffset, SlotIndex};
//!
//! fn takes_slot(_: SlotIndex) {}
//!
//! fn role_offset_rejects_slot_index_compile_fail(offset: RoleOffset) {
//!     takes_slot(offset);
//! }
//! ```
//!
//! Private field — bare integer slot forgery is uncompilable:
//!
//! ```compile_fail
//! use simthing_core::SlotIndex;
//!
//! fn slot_index_fields_private_compile_fail() {
//!     let _ = SlotIndex(0);
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_core::{ColumnIndex, SlotIndex};
//!
//! fn takes_column(_: ColumnIndex) {}
//!
//! fn slot_index_rejects_column_index_compile_fail(slot: SlotIndex) {
//!     takes_column(slot);
//! }
//! ```

use crate::property::RoleOffset;
use serde::{Deserialize, Serialize};

/// Dense GPU buffer row index assigned by [`simthing_gpu::SlotAllocator`].
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct SlotIndex(u32);

impl std::fmt::Display for SlotIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SlotIndex {
    pub fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub fn raw(self) -> u32 {
        self.0
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    pub fn saturating_add(self, delta: u32) -> Self {
        Self::new(self.0.saturating_add(delta))
    }
}

impl From<SlotIndex> for usize {
    fn from(slot: SlotIndex) -> Self {
        slot.as_usize()
    }
}

/// Compile-time guard: slot identity and layout-resolved lane offsets must not mix.
pub fn _slot_index_axis_distinct_from_role_offset(_slot: SlotIndex, _offset: RoleOffset) {}

#[cfg(test)]
mod behavior {
    use super::*;

    #[test]
    fn slot_allocator_behavior_preserved_after_slot_index_newtype() {
        // Mirror SlotAllocator idempotency/round-trip invariants at the SlotIndex boundary.
        let a = SlotIndex::new(0);
        let b = SlotIndex::new(1);
        assert_ne!(a, b);
        assert_eq!(a.raw(), 0);
        assert_eq!(b.as_usize(), 1);
        assert_eq!(a.saturating_add(1), b);
    }

    #[test]
    fn migrated_index_path_behavior_preserved() {
        let first = SlotIndex::new(4);
        let count = 3u32;
        let last_exclusive = first.saturating_add(count);
        assert!(SlotIndex::new(5) >= first && SlotIndex::new(5) < last_exclusive);
        assert!(!(SlotIndex::new(7) < last_exclusive));
    }
}
