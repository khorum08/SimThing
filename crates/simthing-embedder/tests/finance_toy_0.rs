//! Finance toy exemplar for the Embedder Guide.
//!
//! Domain-neutral door: a desk tree, not a game world. Every rust block in
//! `docs/embedders_guide.md` that cites this file is copied from here.

use simthing_embedder::{bind, derive, overlay, populate, run};

fn finance_owner_crossing(root: &mut populate::SimThing) {
    let owner = populate::OwnerRef::try_new_authored("alpha").expect("owner");
    populate::owner(root, &owner);
    populate::ownership(root).expect("one crossing");
}

fn finance_mark_overlay(
    root: &populate::SimThing,
    origin: &populate::SimThing,
    property_id: populate::SimPropertyId,
) -> overlay::Overlay {
    overlay::authored(
        root,
        origin,
        overlay::OverlayKind::Instruction,
        overlay::OverlaySource::System,
        vec![origin.id],
        overlay::PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(
                populate::SubFieldRole::Amount,
                populate::TransformOp::set(110.0),
            )],
        },
        vec![overlay::DissolveCondition::AtSessionEnd],
    )
    .expect("overlay")
}

#[test]
fn finance_toy_five_verbs_observe_and_serialize() {
    let seat = derive::owner_seat("alpha", "Alpha Desk", "desk").expect("owner seat");
    assert_eq!(seat.kind, populate::SimThingKind::Owner);

    let mut scenario = run::Scenario::map_light("finance-toy".into(), 1, 2, 1.0, 3);
    finance_owner_crossing(&mut scenario.root);
    let pid = scenario
        .registry
        .register(populate::SimProperty::simple("desk", "notional", 0));
    let layout = scenario.registry.property(pid).layout.clone();
    let mut pv = populate::PropertyValue::from_layout(&layout);
    pv.set_role(&populate::SubFieldRole::Amount, &layout, 100.0);
    scenario.root.add_property(pid, pv);
    let origin = scenario.root.clone();
    let directive = finance_mark_overlay(&scenario.root, &origin, pid);
    scenario.root.add_overlay(directive);

    let root_id = scenario.root.id;
    let mut session = run::initialize(scenario, &run::GameModeSpec::default()).expect("init");
    run::start(&mut session, run::ExecutionPosture::Paced).expect("start");
    run::tick(&mut session).expect("tick");
    let shadow = bind::shadow(&session);
    assert!(shadow.row(root_id).is_some());
    let replay = tempfile::NamedTempFile::new().expect("replay");
    let summary = run::serialize(&mut session, replay.path(), 1).expect("serialize");
    assert_eq!(summary.frames_written, 1);
}
