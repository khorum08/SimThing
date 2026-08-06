//! OWNER-CHANNEL-INTRINSIC-0 compatibility-ingress and RF-authority proofs.
//!
//! The fixture is synthetic and crate-local. Legacy flat owner references exist only at ingress;
//! every assertion after admission reads the intrinsic owner view.

#[path = "support/reduce_up_fixture.rs"]
mod reduce_up_fixture;

use simthing_core::{SimPropertyId, SimThing, SimThingKind};
use simthing_spec::{
    admit_intrinsic_owner_channels, apply_participant_owner_flow_metadata, game_session_owners,
    make_owner_entity, owner_entity_id, planet_child_rf_participant_inputs_from_owner_view,
    validate_session_owner_entities, OwnerRef, OWNER_FLOW_OWNER_REF_PROPERTY_ID,
    PLANET_OWNER_REF_PROPERTY_ID,
};

use reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec;

fn count_property(node: &SimThing, property_id: SimPropertyId) -> u32 {
    u32::from(node.properties.contains_key(&property_id))
        + node
            .children
            .iter()
            .map(|child| count_property(child, property_id))
            .sum::<u32>()
}

fn find_mut(node: &mut SimThing, raw_id: u32) -> Option<&mut SimThing> {
    if node.id.raw() == raw_id {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| find_mut(child, raw_id))
}

#[test]
fn compatibility_owner_stamps_convert_once_then_rf_reads_only_intrinsic_view() {
    let mut source = build_planet_child_rf_reduce_up_scoped_spec();
    assert!(count_property(&source.root, OWNER_FLOW_OWNER_REF_PROPERTY_ID) > 0);

    let owner_view = admit_intrinsic_owner_channels(&source).expect("one-way owner admission");
    assert!(owner_view.stats().compatibility_property_count > 0);
    assert_eq!(owner_view.stats().legacy_owner_properties_remaining, 0);
    assert_eq!(
        count_property(&owner_view.scenario().root, OWNER_FLOW_OWNER_REF_PROPERTY_ID),
        0
    );
    assert_eq!(
        count_property(&owner_view.scenario().root, PLANET_OWNER_REF_PROPERTY_ID),
        0
    );

    let before = planet_child_rf_participant_inputs_from_owner_view(&owner_view)
        .expect("intrinsic RF participants");
    assert_eq!(before.len(), 4);
    assert!(before.iter().all(|participant| {
        !participant.owner_ref.is_unowned()
            && owner_view.admitted_owners().contains(&participant.owner_ref)
    }));

    let first = before.first().expect("participant");
    let source_node = find_mut(&mut source.root, first.simthing_id_raw).expect("source participant");
    apply_participant_owner_flow_metadata(source_node, "owner_b", 999, 999);

    let after = planet_child_rf_participant_inputs_from_owner_view(&owner_view)
        .expect("same resolved execution view");
    assert_eq!(after, before, "post-ingress flat metadata cannot affect RF authority");
}

#[test]
fn every_non_neutral_resolved_owner_is_an_admitted_owner_entity() {
    let source = build_planet_child_rf_reduce_up_scoped_spec();
    let owner_view = admit_intrinsic_owner_channels(&source).expect("owner admission");

    let authored = game_session_owners(owner_view.scenario())
        .expect("canonical owner entities")
        .into_iter()
        .filter_map(owner_entity_id)
        .map(OwnerRef::new)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(&authored, owner_view.admitted_owners());
    assert!(owner_view
        .resolved_owners()
        .values()
        .filter(|owner| !owner.is_unowned())
        .all(|owner| authored.contains(owner)));
}

#[test]
fn authored_owner_cannot_claim_reserved_neutral_identity() {
    let mut source = build_planet_child_rf_reduce_up_scoped_spec();
    let session = source.root.children.first_mut().expect("game session");
    session
        .children
        .retain(|child| child.kind != SimThingKind::Owner);
    session.add_child(make_owner_entity("unowned", "Invalid", "synthetic"));

    let error = validate_session_owner_entities(&source).expect_err("reserved id must reject");
    assert!(error.to_string().contains("reserved neutral"));
}
