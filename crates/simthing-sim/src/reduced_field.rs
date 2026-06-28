//! ReducedField — CPU view of the GPU `output_vectors` buffer.
//!
//! Produced by `BoundaryProtocol::read_reduced_field(state)` at presentation
//! cadence (typically once per boundary; can be called mid-day for live UI).
//!
//! Each row is `n_dims` wide and corresponds to one allocated slot. Inner-node
//! rows carry per-column reductions over their children; leaf rows mirror the
//! post-Pass-3 `values`. Tombstoned or unallocated slots may hold stale data —
//! callers should query through `BoundaryProtocol::allocator` to know which
//! slots are live.
//!
//! ## Decomposing a row
//!
//! Use `property_value(slot, registry, prop_id)` to extract a single property's
//! sub-fields as a `PropertyValue` you can interpret with the same APIs as a
//! live SimThing.

use simthing_core::{DimensionRegistry, PropertyValue, SimPropertyId};

/// Flat readback of the GPU `output_vectors` buffer at a point in time.
#[derive(Clone, Debug)]
pub struct ReducedField {
    pub n_dims: usize,
    /// Row-major `[n_slots × n_dims]` aggregated values.
    pub values: Vec<f32>,
}

impl ReducedField {
    pub fn n_slots(&self) -> usize {
        if self.n_dims == 0 {
            0
        } else {
            self.values.len() / self.n_dims
        }
    }

    /// Borrow one slot's row. Returns `None` if `slot` is past the buffer end.
    pub fn row(&self, slot: u32) -> Option<&[f32]> {
        let base = slot as usize * self.n_dims;
        self.values.get(base..base + self.n_dims)
    }

    /// Pull a single property's sub-field values out of the reduced row as a
    /// `PropertyValue`. Returns `None` if the slot is out of range or the
    /// property's column range overflows the row.
    pub fn property_value(
        &self,
        slot: u32,
        registry: &DimensionRegistry,
        prop_id: SimPropertyId,
    ) -> Option<PropertyValue> {
        let row = self.row(slot)?;
        let range = registry.column_range(prop_id);
        let stride = registry.property(prop_id).layout.stride();
        let end = range.start + stride;
        if end > row.len() {
            return None;
        }
        Some(PropertyValue::from_raw_lanes(
            row[range.start..end].to_vec(),
        ))
    }
}
