//! rehearsal_lifecycle_interior_policy_overlay_0 — coverage witness for the
//! Interior-Policy Composition Law's INSTALLED-OVERLAY authority (relay
//! `5626653653`, completing DA ruling `5626045761`; the law completed by the DA
//! ruling on the live Board `5879126789`). An active installed overlay that
//! REPLACES the flow property's `Named("weight")` participation — a Set, a
//! non-literal program, or a routed instruction — classifies its host AND every
//! affected SimThing. A standing Multiply or Add policy classifies nothing: it
//! reaches its host's whole subtree by tree-position inheritance, so it deforms
//! the rolled-up total instead. All through the sealed tree's narrow
//! observation-only query, never a raw walk.

use simthing_core::{
    Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
    SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_driver::{
    arena_allocation_sync::collect_weight_overlay_targets, ArenaRegistry, GpuArenaDescriptor,
};
use simthing_sim::SimRuntimeTree;

fn weight_overlay(
    kind: OverlayKind,
    origin: SimThingId,
    property: SimPropertyId,
    affects: Vec<SimThingId>,
    op: TransformOp,
) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind,
        source: OverlaySource::System,
        origin,
        affects,
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Named("weight".into()), op)],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

fn node(name: &str) -> SimThing {
    SimThing::new(SimThingKind::Custom(name.into()), 0)
}

#[test]
fn installed_weight_overlay_classifies_host_and_affected() {
    let flow = SimPropertyId(1);
    let other = SimPropertyId(2);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut set_host = node("set_host");
    let set_child = node("set_child");
    let mut scale_host = node("scale_host");
    let scale_child = node("scale_child");
    let mut route_origin = node("route_origin");
    let route_target = node("route_target");
    let bystander = node("bystander");
    let ids = [
        set_host.id,
        set_child.id,
        scale_host.id,
        scale_child.id,
        route_origin.id,
        route_target.id,
        bystander.id,
    ];
    let [set_host_id, set_child_id, scale_host_id, scale_child_id, origin_id, target_id, bystander_id] =
        ids;

    // Replacing: a standing Set on the flow property's weight.
    set_host.overlays.push(weight_overlay(
        OverlayKind::Policy,
        set_host_id,
        flow,
        vec![set_child_id],
        TransformOp::set(2.0),
    ));
    // Deforming: standing Multiply and Add reach scale_host's subtree by inheritance.
    for op in [TransformOp::multiply(7.0), TransformOp::add(1.0)] {
        scale_host.overlays.push(weight_overlay(
            OverlayKind::Policy,
            scale_host_id,
            flow,
            vec![scale_child_id],
            op,
        ));
    }
    // Replacing: a routed instruction is not inherited below its target.
    route_origin.overlays.push(weight_overlay(
        OverlayKind::Instruction,
        origin_id,
        flow,
        vec![target_id],
        TransformOp::multiply(3.0),
    ));
    // Never classifying: a Set on another property, and a Set on another role.
    set_host.overlays.push(weight_overlay(
        OverlayKind::Policy,
        set_host_id,
        other,
        vec![bystander_id],
        TransformOp::set(5.0),
    ));
    let mut amount_only = weight_overlay(
        OverlayKind::Policy,
        set_host_id,
        flow,
        vec![bystander_id],
        TransformOp::set(5.0),
    );
    amount_only.transform.sub_field_deltas = vec![(SubFieldRole::Amount, TransformOp::set(5.0))];
    set_host.overlays.push(amount_only);

    set_host.add_child(set_child);
    scale_host.add_child(scale_child);
    root.add_child(set_host);
    root.add_child(scale_host);
    root.add_child(route_origin);
    root.add_child(route_target);
    root.add_child(bystander);
    let tree = SimRuntimeTree::admit(root);

    let mut registry = ArenaRegistry::default();
    registry.arenas.push(GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: flow,
        balance_property_id: None,
        max_participants: 8,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: Default::default(),
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    });
    let map = collect_weight_overlay_targets(&tree, &registry);
    let replaced = map.get(&flow).expect("flow property classified");
    assert!(
        replaced.contains(&set_host_id) && replaced.contains(&set_child_id),
        "a Set replaces the host's and the affected participation"
    );
    assert!(
        replaced.contains(&origin_id) && replaced.contains(&target_id),
        "a routed instruction replaces at its origin and target"
    );
    assert!(
        !replaced.contains(&scale_host_id) && !replaced.contains(&scale_child_id),
        "standing Multiply/Add policies deform the rolled-up total; they never pin a participant"
    );
    assert!(
        !replaced.contains(&bystander_id),
        "wrong-property and wrong-role overlays never classify"
    );
    assert!(
        !map.contains_key(&other),
        "non-arena property never enters the map"
    );

    // The query itself carries no law: an admitting predicate enumerates every
    // active weight overlay, the scaling policies included.
    let every =
        tree.overlay_transform_targets(flow, &SubFieldRole::Named("weight".into()), &|_, _| true);
    assert!(
        [
            set_host_id,
            scale_host_id,
            scale_child_id,
            origin_id,
            target_id
        ]
        .iter()
        .all(|id| every.contains(id)),
        "the observation-only query enumerates every active weight overlay"
    );
}
