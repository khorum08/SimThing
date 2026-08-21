//! GPU/runtime buffer row index for a SimThing slot (`slot_idx`).
//!
//! Distinct from [`RoleOffset`] (layout-resolved lane within one property value)
//! and from global matrix column indices at upload boundaries.
//!
//! Transposition with layout-resolved lane offsets is uncompilable:
//!
//! ```compile_fail,E0308
//! use simthing_core::{RoleOffset, SlotIndex};
//!
//! fn takes_role_offset(_: RoleOffset) {}
//!
//! fn slot_index_rejects_role_offset_compile_fail(slot: SlotIndex) {
//!     takes_role_offset(slot);
//! }
//! ```
//!
//! ```compile_fail,E0308
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
//! ```compile_fail,E0423
//! use simthing_core::SlotIndex;
//!
//! fn slot_index_fields_private_compile_fail() {
//!     let _ = SlotIndex(0);
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

/// Dense CELL-SPACE index: the authored-coordinate identity `y*width + x` of
/// one cell inside a field's dense grid (K3 census row; 6.4
/// SLOT-LOGICAL-IDENTITY-0).
///
/// This names the OTHER index space that used to hide inside `SlotIndex`:
/// a cell index derives from authored grid coordinates and stays meaningful
/// under any physical row rebinding — it is never a matrix-row identity, and
/// baking one into an EML literal bakes an authored coordinate, not a
/// physical row. Transposition with `SlotIndex` is uncompilable:
///
/// ```compile_fail,E0308
/// use simthing_core::{CellSpaceIndex, SlotIndex};
///
/// fn takes_slot(_: SlotIndex) {}
///
/// fn cell_space_rejects_slot_position_compile_fail(cell: CellSpaceIndex) {
///     takes_slot(cell);
/// }
/// ```
///
/// ```compile_fail,E0308
/// use simthing_core::{CellSpaceIndex, SlotIndex};
///
/// fn takes_cell(_: CellSpaceIndex) {}
///
/// fn slot_rejects_cell_space_compile_fail(slot: SlotIndex) {
///     takes_cell(slot);
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CellSpaceIndex(u32);

impl CellSpaceIndex {
    /// Mint from authored grid coordinates — the only production door.
    pub fn from_authored_grid(x: u32, y: u32, width: u32) -> Self {
        Self(y * width + x)
    }

    pub fn raw(self) -> u32 {
        self.0
    }

    /// Bake as an EML literal: an authored-coordinate identity by
    /// construction, never a physical matrix row.
    pub fn as_eml_literal(self) -> f32 {
        self.0 as f32
    }
}

#[cfg(test)]
mod behavior {
    use super::*;
}
