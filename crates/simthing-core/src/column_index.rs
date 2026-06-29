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
    pub fn new(raw: usize) -> Self {
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
