//! Global flat column index in the runtime values matrix (`values[slot * n_dims + col]`).
//!
//! Distinct from [`RoleOffset`] (layout-resolved lane within one property value)
//! and from [`SlotIndex`] (buffer row / SimThing slot).
//!
//! Transposition with slot identity is uncompilable:
//!
//! ```compile_fail,E0308
//! use simthing_core::{ColumnIndex, SlotIndex};
//!
//! fn takes_slot(_: SlotIndex) {}
//!
//! fn column_index_rejects_slot_index_compile_fail(col: ColumnIndex) {
//!     takes_slot(col);
//! }
//! ```
//!
//! ```compile_fail,E0308
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
//! ```compile_fail,E0423
//! use simthing_core::ColumnIndex;
//!
//! fn column_index_fields_private_compile_fail() {
//!     let _ = ColumnIndex(0);
//! }
//! ```
//!
//! Layout-resolved lane offsets must not substitute for global columns:
//!
//! ```compile_fail,E0308
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
    /// Production callers must enter through `simthing_kernel::wgsl_encode::column_from_wire`
    /// (the sole WGSL/authored-wire rematerialize helper). Direct use outside that
    /// helper is a census failure.
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

    /// STRUCTURAL-PLAN door: seal a plan-local structural grid channel into a
    /// [`ColumnIndex`] for AccumulatorOp plans that own their own `n_dims` grid.
    ///
    /// Only [`crate::StructuralScalarChannel::into_plan_column`] may call this.
    pub(crate) fn from_structural_plan_channel(raw: u32) -> Self {
        Self(raw as usize)
    }

    /// AUTHORED-ADMIT door: convert an authored-wire column into a typed plan
    /// column after proving `raw < bound` (typically `n_dims`).
    ///
    /// This is **not** a bare `u32 → ColumnIndex` constructor — admission must
    /// supply the bound that makes the mint lawful. Authored/serde surfaces stay
    /// `u32`; compiled/intermediate plan records carry [`ColumnIndex`].
    pub fn try_from_admitted_authored(
        raw: u32,
        bound: u32,
    ) -> Result<Self, AuthoredColumnAdmitError> {
        if bound == 0 || raw >= bound {
            return Err(AuthoredColumnAdmitError { raw, bound });
        }
        Ok(Self(raw as usize))
    }

    pub fn raw(self) -> usize {
        self.0
    }

    pub fn raw_u32(self) -> u32 {
        self.0 as u32
    }
}

/// Failure from [`ColumnIndex::try_from_admitted_authored`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthoredColumnAdmitError {
    pub raw: u32,
    pub bound: u32,
}

impl std::fmt::Display for AuthoredColumnAdmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "authored column {} out of range for bound {}",
            self.raw, self.bound
        )
    }
}

impl std::error::Error for AuthoredColumnAdmitError {}

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
    fn authored_admit_door_rejects_out_of_range_and_preserves_in_range() {
        assert_eq!(
            ColumnIndex::try_from_admitted_authored(3, 4).unwrap().raw(),
            3
        );
        assert!(ColumnIndex::try_from_admitted_authored(4, 4).is_err());
        assert!(ColumnIndex::try_from_admitted_authored(0, 0).is_err());
    }

    #[test]
    fn raw_oracle_rehearsal_door_preserves_column_bits() {
        assert_eq!(ColumnIndex::from_raw_for_oracle_or_rehearsal(23).raw(), 23);
    }
}
