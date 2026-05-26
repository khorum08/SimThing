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
        let slot_base = slot as usize * n_dims;
        for (&prop_id, pv) in &node.properties {
            let range = registry.column_range(prop_id);
            let start = slot_base + range.start;
            let end = start + pv.data.len();
            values[start..end].copy_from_slice(&pv.data);
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

    #[test]
    fn projection_writes_property_data_at_slot_and_column_range() {
        let mut reg = DimensionRegistry::new();
        let loyalty_id = reg.register(loyalty_property());
        let food_id = reg.register(SimProperty::simple("core", "food_security", 0));

        // Build: world → 2 children, each with both properties populated.
        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut a = SimThing::new(SimThingKind::Location, 0);
        let mut b = SimThing::new(SimThingKind::Location, 0);

        let layout_l = reg.property(loyalty_id).layout.clone();
        let a_off = layout_l.offset_of(&SubFieldRole::Amount).unwrap();

        for (node, amount_val) in [(&mut a, 0.3f32), (&mut b, 0.7f32)] {
            let mut pv_l = PropertyValue::from_layout(&layout_l);
            pv_l.data[a_off] = amount_val;
            node.add_property(loyalty_id, pv_l);

            let pv_f = PropertyValue::from_layout(&reg.property(food_id).layout);
            node.add_property(food_id, pv_f);
        }
        world.add_child(a);
        world.add_child(b);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let n_dims = reg.total_columns;
        let mut flat = vec![0.0f32; alloc.capacity() * n_dims];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);

        // World has no properties → its row is all zeros.
        let world_slot = alloc.slot_of(world.id).unwrap() as usize;
        assert!(flat[world_slot * n_dims..(world_slot + 1) * n_dims]
            .iter()
            .all(|&x| x == 0.0));

        // Check the two location rows: loyalty amount appears at its column.
        let loyalty_range = reg.column_range(loyalty_id);
        for child in &world.children {
            let slot = alloc.slot_of(child.id).unwrap() as usize;
            let cpu_amount = child.properties[&loyalty_id].data[a_off];
            let gpu_amount = flat[slot * n_dims + loyalty_range.start + a_off];
            assert_eq!(cpu_amount.to_bits(), gpu_amount.to_bits());
        }
    }

    #[test]
    fn projection_skips_unallocated_nodes() {
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());

        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut pv = PropertyValue::from_layout(&reg.property(id).layout);
        let a_off = reg
            .property(id)
            .layout
            .offset_of(&SubFieldRole::Amount)
            .unwrap();
        pv.data[a_off] = 0.42;
        world.add_property(id, pv);

        // Allocator that does NOT include `world.id`.
        let alloc = SlotAllocator::new();
        let n_dims = reg.total_columns;
        let mut flat = vec![0.0f32; alloc.capacity() * n_dims];
        // capacity is 0 → flat is empty → projection is a silent no-op.
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        assert!(flat.is_empty());
    }
}
