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
