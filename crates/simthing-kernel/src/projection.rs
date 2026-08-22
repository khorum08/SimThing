//! Sparse SimThing tree → dense GPU values buffer.
//!
//! Each SimThing's sparse **registered dimension** properties are written into
//! the row at `slot_idx * n_dims`, with each property's data placed at the
//! registry's column range. Intrinsic structural metadata (for example the
//! owner-channel binding) has no registry range and never enters GPU values.
//! Untouched columns retain their previous content (caller's responsibility to
//! zero if needed).
//!
//! This is the data-shaping half of what will eventually be the
//! `EvaluationBatch` builder. The transform-matrix half waits on the
//! affine-encoding decision before it can be written.

use simthing_core::{DimensionRegistry, ObjectResidencyRequest, SimThing};

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
    project_node(
        root,
        root.root_residency_request(),
        registry,
        allocator,
        n_dims,
        values,
    );
}

fn project_node(
    node: &SimThing,
    request: ObjectResidencyRequest,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    n_dims: usize,
    values: &mut [f32],
) {
    if let Some(residency) = allocator.residency_for(&request) {
        let slot = residency.slot();
        let slot_base = slot.as_usize() * n_dims;
        for (&prop_id, pv) in &node.properties {
            let Some(range) = registry.try_column_range(prop_id) else {
                // Sparse SimThing properties also carry intrinsic structural
                // metadata. The DimensionRegistry remains the sole authority
                // for which properties own dense GPU columns.
                continue;
            };
            let start = slot_base + range.start;
            let end = start + pv.lane_count();
            values[start..end].copy_from_slice(pv.raw_lanes_for_serialization());
        }
    }
    for child in &node.children {
        let request = node
            .attached_child_residency_request(child)
            .expect("tree traversal holds the attached direct child");
        project_node(child, request, registry, allocator, n_dims, values);
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
