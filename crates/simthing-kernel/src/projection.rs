//! Sparse SimThing tree → dense GPU values buffer.
//!
//! Each SimThing's sparse `HashMap<SimPropertyId, PropertyValue>` is written
//! into the row at `slot_idx * n_dims`, with each property's data placed at
//! the registry's column range for that property. Untouched columns retain
//! their previous content (caller's responsibility to zero if needed).
//!
//! This is the data-shaping half of what will eventually be the
//! `EvaluationBatch` builder. The transform-matrix half waits on the
//! affine-encoding decision before it can be written.

use simthing_core::{DimensionRegistry, SimThing};

use crate::slot::SlotAllocator;

/// Walk a SimThing tree and write every node's property data into the flat
/// values buffer. Skips nodes whose ids are not in the allocator.
///
/// `values.len()` must equal `allocator.capacity() * n_dims`.
pub fn project_tree_to_values(
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    n_dims: usize,
    values: &mut [f32],
) {
    debug_assert_eq!(
        values.len(),
        allocator.capacity() * n_dims,
        "values buffer must be sized to allocator.capacity() * n_dims",
    );
    project_node(root, registry, allocator, n_dims, values);
}

fn project_node(
    node: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    n_dims: usize,
    values: &mut [f32],
) {
    if let Some(slot) = allocator.slot_of(node.id) {
        let slot_base = slot.as_usize() * n_dims;
        for (&prop_id, pv) in &node.properties {
            let range = registry.column_range(prop_id);
            let start = slot_base + range.start;
            let end = start + pv.lane_count();
            values[start..end].copy_from_slice(pv.raw_lanes_for_serialization());
        }
    }
    for child in &node.children {
        project_node(child, registry, allocator, n_dims, values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DimensionRegistry, IntensityBehavior, PropertyValue, SimProperty, SimThing, SimThingKind,
        SubFieldRole,
    };

    fn loyalty_property() -> SimProperty {
        let mut p = SimProperty::simple("core", "loyalty", 0);
        p.intensity_behavior = Some(IntensityBehavior::default());
        p
    }

}
