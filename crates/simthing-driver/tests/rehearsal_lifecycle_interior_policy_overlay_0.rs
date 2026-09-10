//! rehearsal_lifecycle_interior_policy_overlay_0 — coverage witness for the
//! Interior-Policy Composition Law's INSTALLED-OVERLAY authority (relay
//! `5626653653`, completing DA ruling `5626045761`). The policy-bearing set
//! must derive from EVERY canonical authored AllocatorWeight authority: an
//! active installed overlay whose transform targets the flow property's
//! `Named("weight")` sub-field classifies its host AND every affected
//! SimThing — through the sealed tree's narrow observation-only query, never
//! a raw walk.

use simthing_core::{
    ColumnIndex, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimProperty, SimPropertyId, SimThing, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_driver::{
    arena_allocation_sync::collect_weight_overlay_targets, ArenaRegistry, GpuArenaDescriptor,
};
use simthing_sim::SimRuntimeTree;

fn weight_overlay(_id: u64, property: SimPropertyId, affects: Vec<simthing_core::SimThingId>) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin: affects.first().copied().unwrap_or_else(|| SimThing::new(SimThingKind::World, 0).id),
        affects,
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Named("weight".into()), TransformOp::multiply(7.0))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

#[test]
fn installed_weight_overlay_classifies_host_and_affected() {
    let flow = SimPropertyId(1);
    let other = SimPropertyId(2);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut owner = SimThing::new(SimThingKind::Custom("owner".into()), 0);
    let child = SimThing::new(SimThingKind::Custom("child".into()), 0);
    let bystander = SimThing::new(SimThingKind::Custom("bystander".into()), 0);
    let (owner_id, child_id, bystander_id) = (owner.id, child.id, bystander.id);

    // Qualifying: active, flow property, Named("weight") role — on the owner,
    // affecting the child too.
    owner.overlays.push(weight_overlay(1, flow, vec![child_id]));
    // Non-qualifying: wrong property.
    owner.overlays.push(weight_overlay(2, other, vec![bystander_id]));
    // Non-qualifying: flow property but a different sub-field role.
    let mut amount_only = weight_overlay(3, flow, vec![bystander_id]);
    amount_only.transform.sub_field_deltas = vec![(SubFieldRole::Amount, TransformOp::multiply(2.0))];
    owner.overlays.push(amount_only);

    owner.add_child(child);
    root.add_child(owner);
    root.add_child(bystander);
    let tree = SimRuntimeTree::admit(root);

    let targets = tree.overlay_transform_targets(flow, &SubFieldRole::Named("weight".into()));
    assert!(targets.contains(&owner_id), "host of the qualifying overlay classifies");
    assert!(targets.contains(&child_id), "affected SimThing classifies");
    assert!(
        !targets.contains(&bystander_id),
        "wrong-property and wrong-role overlays never classify"
    );

    // The sync-boundary map keys by the arena's flow property only.
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
    let ids = map.get(&flow).expect("flow property classified");
    assert!(ids.contains(&owner_id) && ids.contains(&child_id));
    assert!(!map.contains_key(&other), "non-arena property never enters the map");

    // Registry sanity so `flow`/`ColumnIndex` stay honest admitted vocabulary.
    let mut dims = simthing_core::DimensionRegistry::new();
    let registered = dims.register(SimProperty::simple("rf", "stock", 0));
    assert_eq!(registered, SimPropertyId(0));
    let _ = ColumnIndex::from_raw_for_oracle_or_rehearsal(0);
}
