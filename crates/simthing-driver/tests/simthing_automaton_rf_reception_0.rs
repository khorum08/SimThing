//! SIMTHING-AUTOMATON-INTRINSIC-0 production-seam referees.
//!
//! Synthetic structural trees only: no authored corpus or domain vocabulary.

use simthing_core::evaluate::Evaluator;
use simthing_core::{
    deliver_standing_directive, DimensionRegistry, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty, SimPropertyId, SimThing,
    SimThingId, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_driver::{receive_command_deficits_from_disbursement, CommandDeficit};
use simthing_gpu::{build_overlay_deltas, SlotAllocator, OP_ADD, OP_MULTIPLY};
use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, apply_runtime_local_allocations_from_disburse_down,
    OwnerRef, ResourceKey, RuntimeOwnerSiloDemandBucket, RuntimeOwnerSiloWritebackResult, ScopeId,
};

fn registry() -> (DimensionRegistry, SimPropertyId) {
    let mut registry = DimensionRegistry::new();
    let property_id = registry.register(SimProperty::simple("test", "signal", 0));
    (registry, property_id)
}

fn node(kind: SimThingKind) -> SimThing {
    SimThing::new(kind, 0)
}

fn with_amount(registry: &DimensionRegistry, property_id: SimPropertyId, amount: f32) -> SimThing {
    let mut node = node(SimThingKind::Cohort);
    let property = registry.property(property_id);
    let mut value = property.default_value();
    value.set_role(&SubFieldRole::Amount, &property.layout, amount);
    node.add_property(property_id, value);
    node
}

fn overlay(
    origin: SimThingId,
    kind: OverlayKind,
    property_id: SimPropertyId,
    op: TransformOp,
) -> Overlay {
    let lifecycle = match kind {
        OverlayKind::Instruction | OverlayKind::Custom(_) => OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![simthing_core::DissolveCondition::AtSessionEnd],
        },
        _ => OverlayLifecycle::UntilDissolved,
    };
    Overlay {
        id: OverlayId::new(),
        kind,
        source: OverlaySource::System,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(SubFieldRole::Amount, op)],
        },
        lifecycle,
    }
}

fn find(root: &SimThing, target: SimThingId) -> &SimThing {
    if root.id == target {
        return root;
    }
    root.children
        .iter()
        .find_map(|child| find_optional(child, target))
        .expect("target is in the synthetic tree")
}

fn find_optional(root: &SimThing, target: SimThingId) -> Option<&SimThing> {
    (root.id == target).then_some(root).or_else(|| {
        root.children
            .iter()
            .find_map(|child| find_optional(child, target))
    })
}

fn find_mut(root: &mut SimThing, target: SimThingId) -> &mut SimThing {
    if root.id == target {
        return root;
    }
    root.children
        .iter_mut()
        .find_map(|child| find_mut_optional(child, target))
        .expect("target is in the synthetic tree")
}

fn find_mut_optional(root: &mut SimThing, target: SimThingId) -> Option<&mut SimThing> {
    if root.id == target {
        return Some(root);
    }
    root.children
        .iter_mut()
        .find_map(|child| find_mut_optional(child, target))
}

fn amount(
    registry: &DimensionRegistry,
    property_id: SimPropertyId,
    root: &SimThing,
    target: SimThingId,
) -> f32 {
    let snapshot = Evaluator::new(registry, 0.0).evaluate(root, 0);
    snapshot
        .get(target)
        .and_then(|entity| entity.properties.get(&property_id))
        .expect("target carries property")
        .get_role(
            &SubFieldRole::Amount,
            &registry.property(property_id).layout,
        )
}

fn target_gpu_ops(
    root: &SimThing,
    registry: &DimensionRegistry,
    target: SimThingId,
) -> Vec<(u32, f32)> {
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(root);
    let (deltas, ranges) = build_overlay_deltas(root, registry, &allocator);
    let slot = allocator
        .slot_of(target)
        .expect("target resident")
        .as_usize();
    let range = ranges[slot];
    deltas[range.offset as usize..(range.offset + range.length) as usize]
        .iter()
        .map(|delta| (delta.op_kind, delta.value))
        .collect()
}
