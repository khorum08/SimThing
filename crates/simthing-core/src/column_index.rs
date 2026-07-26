//! Global flat column index in the runtime values matrix (`values[slot * n_dims + col]`).
//!
//! Distinct from [`RoleOffset`] (layout-resolved lane within one property value)
//! and from [`SlotIndex`] (buffer row / SimThing slot).
//!
//! Transposition with slot identity is uncompilable:
//!
//! ```compile_fail
//! use simthing_core::{ColumnIndex, SlotIndex};
//!
//! fn takes_slot(_: SlotIndex) {}
//!
//! fn column_index_rejects_slot_index_compile_fail(col: ColumnIndex) {
//!     takes_slot(col);
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
//!
//! Private field — bare integer column forgery is uncompilable:
//!
//! ```compile_fail
//! use simthing_core::ColumnIndex;
//!
//! fn column_index_fields_private_compile_fail() {
//!     let _ = ColumnIndex(0);
//! }
//! ```
//!
//! Layout-resolved lane offsets must not substitute for global columns:
//!
//! ```compile_fail
//! use simthing_core::{ColumnIndex, PropertyLayout, SubFieldRole};
//!
//! fn column_index_rejects_role_offset_compile_fail() {
//!     let layout = PropertyLayout::standard(0);
//!     let _: ColumnIndex = layout.offset_of(&SubFieldRole::Amount).unwrap();
//! }
//! ```

use crate::property::RoleOffset;
use serde::{Deserialize, Serialize};

/// Flat runtime matrix column index (not layout-relative).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct ColumnIndex(usize);

impl std::fmt::Display for ColumnIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ColumnIndex {
    /// Compatibility alias for legacy column mints.
    ///
    /// New production code must use the layout-derived role pathway or the
    /// explicitly fenced GPU round-trip and oracle/rehearsal doors.
    /// Promotion blocker: rung 9.2 migrates legacy callers and removes this
    /// compatibility surface.
    #[deprecated(
        note = "new code must use PropertyColumnRange::col_for_role, from_gpu_round_trip, or from_raw_for_oracle_or_rehearsal"
    )]
    pub fn new(raw: usize) -> Self {
        Self::from_raw_for_oracle_or_rehearsal(raw)
    }

    /// LAYOUT-DERIVED door: combines a registry-owned global range with a
    /// [`RoleOffset`] resolved by [`crate::property::PropertyLayout::offset_of`].
    ///
    /// This constructor is crate-private so external callers must enter through
    /// [`crate::registry::PropertyColumnRange::col_for_role`] (or its range
    /// counterpart). Promotion blocker: none; this is the permanent P0/P4 role
    /// pathway and must remain registry-owned.
    pub(crate) fn from_layout_role(range_start: usize, local: RoleOffset) -> Self {
        Self(range_start + local.lane())
    }

    /// GPU-ROUND-TRIP door: re-materializes a column from a `gpu.*_col`
    /// adapter/plan field after a GPU representation round trip.
    ///
    /// Promotion blocker: rung 4.2 carries [`ColumnIndex`] through plan structs
    /// end-to-end and confines raw `u32` columns to the single WGSL
    /// encode/decode boundary.
    pub fn from_gpu_round_trip(raw: u32) -> Self {
        Self(raw as usize)
    }

    /// RAW-ORACLE-REHEARSAL door: mints an independent raw column only for CPU
    /// oracles and bounded rehearsal code whose judging independence requires
    /// construction without the production layout path.
    ///
    /// Promotion blocker: retire uses as each oracle/rehearsal gains a typed
    /// input without depending on the production derivation it judges.
    pub fn from_raw_for_oracle_or_rehearsal(raw: usize) -> Self {
        Self(raw)
    }

    pub fn raw(self) -> usize {
        self.0
    }

    pub fn raw_u32(self) -> u32 {
        self.0 as u32
    }
}

impl From<ColumnIndex> for usize {
    fn from(col: ColumnIndex) -> Self {
        col.raw()
    }
}

/// Compile-time guard: global column and layout lane must not mix at typed boundaries.
pub fn _column_index_axis_distinct_from_role_offset(_col: ColumnIndex, _offset: RoleOffset) {}

#[cfg(test)]
mod tests {
    use super::ColumnIndex;

    #[test]
    fn gpu_round_trip_door_preserves_column_bits() {
        assert_eq!(ColumnIndex::from_gpu_round_trip(17).raw(), 17);
    }

    #[test]
    fn raw_oracle_rehearsal_door_preserves_column_bits() {
        assert_eq!(ColumnIndex::from_raw_for_oracle_or_rehearsal(23).raw(), 23);
    }

    #[test]
    #[allow(deprecated)]
    fn legacy_new_delegates_to_the_fenced_raw_door() {
        assert_eq!(
            ColumnIndex::new(31),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(31)
        );
    }
}
