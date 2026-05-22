//! CPU preparation pass for Pass 3: iterative overlay transform application.
//!
//! Walks the SimThing tree depth-first in the same order as `Evaluator::evaluate_node`,
//! building a flat `Vec<OverlayDelta>` (ancestor stack first, then local, in evaluation
//! order) and a `Vec<SlotDeltaRange>` (one per slot, indexed by slot index).
//!
//! The GPU shader (Pass 3) walks each slot's delta range and applies ops in order —
//! same order the CPU evaluator applies them in step 5. Bit-exact parity is therefore
//! trivially preserved: no composition step, no rounding-order divergence.

use simthing_core::overlay::PropertyTransformDelta;
use simthing_core::property::TransformOp;
use simthing_core::{DimensionRegistry, SimThing};

use crate::slot::SlotAllocator;
use crate::world_state::{OverlayDelta, SlotDeltaRange, OP_ADD, OP_MULTIPLY, OP_SET};

/// Build the per-tick overlay delta batch for upload to `WorldGpuState`.
///
/// Mirrors `Evaluator::evaluate_node` exactly:
///   - Ancestor transforms accumulate depth-first in push order.
///   - Local overlays are appended after ancestors (same as `TransformStack::push`).
///   - Only deltas for properties the node actually has are emitted (mirrors the
///     evaluator iterating `resolved` which contains only the node's own properties).
///   - Column resolution via `col_for_role` only (Invariant I1).
///
/// `ranges` is indexed by slot index and initialized to zero-length for all slots,
/// so slots with no overlays naturally get `length = 0` and Pass 3 skips them.
pub fn build_overlay_deltas(
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> (Vec<OverlayDelta>, Vec<SlotDeltaRange>) {
    let n_slots = allocator.capacity();
    let mut deltas: Vec<OverlayDelta> = Vec::new();
    let mut ranges: Vec<SlotDeltaRange> = vec![SlotDeltaRange::default(); n_slots];

    build_node(root, &[], registry, allocator, &mut deltas, &mut ranges);

    (deltas, ranges)
}

/// Recursive helper. `ancestor_transforms` carries the ordered list of
/// `PropertyTransformDelta`s accumulated from the root down to the current node's
/// parent — matching `TransformStack::deltas` at the point of recursion in the evaluator.
fn build_node(
    node: &SimThing,
    ancestor_transforms: &[PropertyTransformDelta],
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    deltas: &mut Vec<OverlayDelta>,
    ranges: &mut Vec<SlotDeltaRange>,
) {
    // Compose: ancestor transforms + this node's overlay transforms, in order.
    // Mirrors: local_stack = node.overlays.iter().fold(ancestors.clone(), |s, o| s.push(...))
    let mut local_transforms: Vec<PropertyTransformDelta> = ancestor_transforms.to_vec();
    for overlay in &node.overlays {
        if !overlay.is_active() {
            continue;
        }
        local_transforms.push(overlay.transform.clone());
    }

    // Emit deltas for this node's slot (if it has one).
    if let Some(slot) = allocator.slot_of(node.id) {
        let offset = deltas.len() as u32;

        // Mirrors evaluator step 5: apply local_stack to each property the node HAS.
        // Only emit a delta if node.properties contains the transform's target property.
        for transform in &local_transforms {
            if !node.properties.contains_key(&transform.property_id) {
                continue;
            }
            let range = registry.column_range(transform.property_id);
            let layout = &registry.property(transform.property_id).layout;
            for (role, op) in &transform.sub_field_deltas {
                // I1: resolve role → global column via col_for_role only.
                let Some(col) = range.col_for_role(role, layout) else {
                    continue;
                };
                let (op_kind, value) = match op {
                    TransformOp::Multiply(v) => (OP_MULTIPLY, *v),
                    TransformOp::Add(v) => (OP_ADD, *v),
                    TransformOp::Set(v) => (OP_SET, *v),
                };
                deltas.push(OverlayDelta {
                    col: col as u32,
                    op_kind,
                    value,
                    _pad: 0,
                });
            }
        }

        let length = deltas.len() as u32 - offset;
        ranges[slot as usize] = SlotDeltaRange { offset, length };
    }

    // Recurse children with the full local_transforms (this node's overlays included).
    // Mirrors: evaluate_node(child, &local_stack, ...)
    for child in &node.children {
        build_node(
            child,
            &local_transforms,
            registry,
            allocator,
            deltas,
            ranges,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slot::SlotAllocator;
    use crate::world_state::{OP_ADD, OP_MULTIPLY, OP_SET};
    use simthing_core::ids::OverlayId;
    use simthing_core::overlay::{Overlay, OverlayKind, OverlayLifecycle, OverlaySource};
    use simthing_core::property::{SimProperty, SubFieldRole};
    use simthing_core::{DimensionRegistry, SimThing, SimThingKind};

    fn reg_with_loyalty() -> (DimensionRegistry, simthing_core::SimPropertyId) {
        let mut reg = DimensionRegistry::new();
        let id = reg.register(SimProperty::simple("core", "loyalty", 0));
        (reg, id)
    }

    fn make_overlay(
        prop_id: simthing_core::SimPropertyId,
        deltas: Vec<(SubFieldRole, TransformOp)>,
    ) -> Overlay {
        Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: prop_id,
                sub_field_deltas: deltas,
            },
            lifecycle: OverlayLifecycle::Permanent,
        }
    }

    /// A node with no overlays produces a zero-length range.
    #[test]
    fn no_overlays_produces_empty_range() {
        let (reg, id) = reg_with_loyalty();
        let mut node = SimThing::new(SimThingKind::Cohort, 0);
        node.add_property(
            id,
            simthing_core::PropertyValue::from_layout(&reg.property(id).layout),
        );

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&node);

        let (deltas, ranges) = build_overlay_deltas(&node, &reg, &alloc);
        assert!(deltas.is_empty());
        let slot = alloc.slot_of(node.id).unwrap();
        assert_eq!(ranges[slot as usize].length, 0);
    }

    /// A single local overlay emits the expected OverlayDelta.
    #[test]
    fn local_overlay_emits_correct_delta() {
        let (reg, lid) = reg_with_loyalty();
        let layout = reg.property(lid).layout.clone();

        let mut node = SimThing::new(SimThingKind::Cohort, 0);
        node.add_property(lid, simthing_core::PropertyValue::from_layout(&layout));
        node.add_overlay(make_overlay(
            lid,
            vec![(SubFieldRole::Amount, TransformOp::Multiply(0.8))],
        ));

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&node);

        let (deltas, ranges) = build_overlay_deltas(&node, &reg, &alloc);
        let slot = alloc.slot_of(node.id).unwrap();
        let r = ranges[slot as usize];
        assert_eq!(r.length, 1);

        let d = deltas[r.offset as usize];
        let expected_col = reg
            .column_range(lid)
            .col_for_role(&SubFieldRole::Amount, &layout)
            .unwrap();
        assert_eq!(d.col, expected_col as u32);
        assert_eq!(d.op_kind, OP_MULTIPLY);
        assert_eq!(d.value.to_bits(), 0.8_f32.to_bits());
    }

    /// Ancestor overlays appear before local overlays in the delta list.
    #[test]
    fn ancestor_deltas_precede_local_deltas() {
        let (reg, lid) = reg_with_loyalty();
        let layout = reg.property(lid).layout.clone();
        let amount_col = reg
            .column_range(lid)
            .col_for_role(&SubFieldRole::Amount, &layout)
            .unwrap() as u32;

        let mut parent = SimThing::new(SimThingKind::Location, 0);
        parent.add_overlay(make_overlay(
            lid,
            vec![(SubFieldRole::Amount, TransformOp::Multiply(0.9))],
        ));

        let mut child = SimThing::new(SimThingKind::Cohort, 0);
        child.add_property(lid, simthing_core::PropertyValue::from_layout(&layout));
        child.add_overlay(make_overlay(
            lid,
            vec![(SubFieldRole::Amount, TransformOp::Add(-0.05))],
        ));
        let child_id = child.id;
        parent.add_child(child);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&parent);

        let (deltas, ranges) = build_overlay_deltas(&parent, &reg, &alloc);

        let slot = alloc.slot_of(child_id).unwrap();
        let r = ranges[slot as usize];
        assert_eq!(r.length, 2, "ancestor + local = 2 deltas");

        let first = deltas[r.offset as usize];
        let second = deltas[r.offset as usize + 1];

        // First delta: ancestor Multiply(0.9)
        assert_eq!(first.col, amount_col);
        assert_eq!(first.op_kind, OP_MULTIPLY);
        assert_eq!(first.value.to_bits(), 0.9_f32.to_bits());

        // Second delta: local Add(-0.05)
        assert_eq!(second.col, amount_col);
        assert_eq!(second.op_kind, OP_ADD);
        assert_eq!(second.value.to_bits(), (-0.05_f32).to_bits());
    }

    /// A node that does not have the target property is skipped.
    #[test]
    fn overlay_for_absent_property_is_skipped() {
        let (reg, lid) = reg_with_loyalty();
        let layout = reg.property(lid).layout.clone();

        // Parent has loyalty overlay. Child does NOT have loyalty property.
        let mut parent = SimThing::new(SimThingKind::Location, 0);
        parent.add_overlay(make_overlay(
            lid,
            vec![(SubFieldRole::Amount, TransformOp::Set(0.5))],
        ));

        let child = SimThing::new(SimThingKind::Cohort, 0);
        // Deliberately add NO properties to child.
        let _ = layout; // suppress unused warning
        let child_id = child.id;
        parent.add_child(child);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&parent);

        let (deltas, ranges) = build_overlay_deltas(&parent, &reg, &alloc);
        let slot = alloc.slot_of(child_id).unwrap();
        let r = ranges[slot as usize];
        assert_eq!(r.length, 0, "child without property gets zero deltas");
        let _ = deltas;
    }

    /// All three op kinds round-trip correctly.
    #[test]
    fn all_op_kinds_are_encoded() {
        let (reg, lid) = reg_with_loyalty();
        let layout = reg.property(lid).layout.clone();

        let mut node = SimThing::new(SimThingKind::Cohort, 0);
        node.add_property(lid, simthing_core::PropertyValue::from_layout(&layout));
        node.add_overlay(make_overlay(
            lid,
            vec![
                (SubFieldRole::Amount, TransformOp::Multiply(0.7)),
                (SubFieldRole::Velocity, TransformOp::Add(0.02)),
                (SubFieldRole::Amount, TransformOp::Set(0.3)),
            ],
        ));

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&node);

        let (deltas, ranges) = build_overlay_deltas(&node, &reg, &alloc);
        let slot = alloc.slot_of(node.id).unwrap();
        let r = ranges[slot as usize];
        assert_eq!(r.length, 3);

        assert_eq!(deltas[r.offset as usize].op_kind, OP_MULTIPLY);
        assert_eq!(deltas[r.offset as usize + 1].op_kind, OP_ADD);
        assert_eq!(deltas[r.offset as usize + 2].op_kind, OP_SET);
    }

    #[test]
    fn suspended_overlay_is_not_encoded() {
        let (reg, lid) = reg_with_loyalty();
        let layout = reg.property(lid).layout.clone();

        let mut node = SimThing::new(SimThingKind::Cohort, 0);
        node.add_property(lid, simthing_core::PropertyValue::from_layout(&layout));
        let mut overlay = make_overlay(lid, vec![(SubFieldRole::Amount, TransformOp::Set(0.9))]);
        overlay.lifecycle = OverlayLifecycle::Suspended {
            when_activated: Box::new(OverlayLifecycle::Permanent),
        };
        node.add_overlay(overlay);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&node);

        let (deltas, ranges) = build_overlay_deltas(&node, &reg, &alloc);
        let slot = alloc.slot_of(node.id).unwrap();
        assert!(deltas.is_empty());
        assert_eq!(ranges[slot as usize].length, 0);
    }
}
