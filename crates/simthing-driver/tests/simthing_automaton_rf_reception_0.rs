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

#[test]
fn command_deficit_rides_disbursement_and_arrives_with_live_route_policy() {
    let (registry, property_id) = registry();
    let mut root = node(SimThingKind::World);
    let mut policy_host = node(SimThingKind::Location);
    let origin = node(SimThingKind::Cohort);
    let origin_id = origin.id;
    let policy_host_id = policy_host.id;
    policy_host.add_overlay(overlay(
        policy_host_id,
        OverlayKind::Policy,
        property_id,
        TransformOp::multiply(0.5),
    ));
    policy_host.add_child(origin);
    let receiver = with_amount(&registry, property_id, 0.2);
    let receiver_id = receiver.id;
    let root_id = root.id;
    root.add_child(policy_host);
    root.add_child(receiver);

    let owner_ref = OwnerRef::new("owner");
    let resource_key = ResourceKey::new("command");
    let scope_id = ScopeId::from_boundary(root_id);
    let writeback = vec![RuntimeOwnerSiloWritebackResult {
        owner_ref: owner_ref.clone(),
        resource_key: resource_key.clone(),
        previous_current: 1,
        next_current: 1,
        capacity: None,
        applied_surplus: 0,
        applied_deficit: 0,
        clamped_surplus: 0,
        unmet_deficit: 0,
    }];
    let directive = overlay(
        origin_id,
        OverlayKind::Instruction,
        property_id,
        TransformOp::add(0.4),
    );
    let directive_id = directive.id;
    let deficit = CommandDeficit {
        receiver: receiver_id,
        owner_ref: owner_ref.clone(),
        resource_key: resource_key.clone(),
        scope_id: scope_id.clone(),
        priority: 0,
        directive,
    };

    // Planted pre-fix production seam: disbursement and local allocation both
    // succeed, but bypassing the reception hook leaves the inbox empty.
    let mut bypass = root.clone();
    let demand = RuntimeOwnerSiloDemandBucket {
        owner_ref,
        resource_key,
        scope_id,
        requested: 1,
        priority: 0,
        source_simthing_id_raw: Some(receiver_id.raw()),
    };
    let disbursed = apply_owner_silo_runtime_disburse_down_cpu(&writeback, &[demand])
        .expect("existing disbursement supplies command unit");
    let allocated = apply_runtime_local_allocations_from_disburse_down(&disbursed)
        .expect("existing local allocation admits command unit");
    assert_eq!(allocated.allocated_total, 1);
    assert!(find_mut(&mut bypass, receiver_id).overlays.is_empty());

    let mut replay = root.clone();
    let report =
        receive_command_deficits_from_disbursement(&mut root, &writeback, &[deficit.clone()])
            .expect("production RF seam delivers supplied command");
    let replay_report =
        receive_command_deficits_from_disbursement(&mut replay, &writeback, &[deficit])
            .expect("identical production input replays deterministically");
    assert_eq!(report, replay_report);
    let delivered_ids = |tree: &SimThing| {
        find(tree, receiver_id)
            .overlays
            .iter()
            .map(|overlay| overlay.id)
            .collect::<Vec<_>>()
    };
    assert_eq!(delivered_ids(&root), delivered_ids(&replay));
    assert_eq!(report.local_allocation.allocated_total, 1);
    assert_eq!(report.local_allocation.unmet_total, 0);
    assert_eq!(
        report.disburse_down_results[0].available_before,
        report.disburse_down_results[0].allocated_total
            + report.disburse_down_results[0].remaining_after,
        "the delivered command unit must conserve owner-silo availability"
    );
    assert_eq!(report.deliveries.len(), 1);
    assert_eq!(report.deliveries[0].overlay_id, directive_id);
    assert_eq!(
        report.deliveries[0].route,
        vec![origin_id, policy_host_id, root_id, receiver_id]
    );
    assert_eq!(find(&root, receiver_id).overlays.len(), 1);
    assert_eq!(
        amount(&registry, property_id, &root, receiver_id).to_bits(),
        0.3_f32.to_bits()
    );
    assert_eq!(
        target_gpu_ops(&root, &registry, receiver_id),
        vec![(OP_ADD, 0.4), (OP_MULTIPLY, 0.5)],
        "GPU preparation must use the same live route order as the CPU evaluator"
    );

    find_mut(&mut root, policy_host_id).overlays[0]
        .transform
        .sub_field_deltas[0]
        .1 = TransformOp::multiply(0.25);
    assert_eq!(find(&root, receiver_id).overlays[0].id, directive_id);
    assert_eq!(
        amount(&registry, property_id, &root, receiver_id).to_bits(),
        0.15_f32.to_bits(),
        "delivery-time policy snapshot mutant must RED"
    );
    assert_eq!(
        target_gpu_ops(&root, &registry, receiver_id),
        vec![(OP_ADD, 0.4), (OP_MULTIPLY, 0.25)]
    );

    find_mut(&mut root, policy_host_id).overlays[0].lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(OverlayLifecycle::UntilDissolved),
    };
    assert_eq!(find(&root, receiver_id).overlays.len(), 1);
    assert_eq!(
        amount(&registry, property_id, &root, receiver_id).to_bits(),
        0.6_f32.to_bits()
    );
    assert_eq!(
        target_gpu_ops(&root, &registry, receiver_id),
        vec![(OP_ADD, 0.4)]
    );

    find_mut(&mut root, policy_host_id).overlays.clear();
    assert_eq!(find(&root, receiver_id).overlays.len(), 1);
    assert_eq!(
        amount(&registry, property_id, &root, receiver_id).to_bits(),
        0.6_f32.to_bits()
    );
    assert_eq!(
        target_gpu_ops(&root, &registry, receiver_id),
        vec![(OP_ADD, 0.4)]
    );
}

#[test]
fn standing_directive_uses_shared_inheritance_walk_without_descendant_state() {
    let (registry, property_id) = registry();
    let mut root = node(SimThingKind::World);
    let root_id = root.id;
    let mut middle = node(SimThingKind::Location);
    let receiver = with_amount(&registry, property_id, 0.1);
    let receiver_id = receiver.id;
    middle.add_child(receiver);
    root.add_child(middle);

    deliver_standing_directive(
        &mut root,
        root_id,
        overlay(
            root_id,
            OverlayKind::Governance,
            property_id,
            TransformOp::add(0.25),
        ),
    )
    .expect("standing directive installs at the resolution root");

    assert_eq!(root.overlays.len(), 1);
    assert_eq!(root.children[0].overlays.capacity(), 0);
    assert_eq!(root.children[0].children[0].overlays.capacity(), 0);
    assert_eq!(
        amount(&registry, property_id, &root, receiver_id).to_bits(),
        0.35_f32.to_bits(),
        "absence at descendants must inherit through the production evaluator walk"
    );
    assert_eq!(
        target_gpu_ops(&root, &registry, receiver_id),
        vec![(OP_ADD, 0.25)],
        "GPU preparation must share the same inherited standing transform"
    );
}
