//! Capability unlock threshold registrations produced by `simthing-spec` and
//! consumed by `simthing-sim` when building the Pass 7 threshold registry.

use simthing_core::{SimPropertyId, SimThingId, SubFieldRole};

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityUnlockRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id:  SimPropertyId,
    pub sub_field:    SubFieldRole,
    pub threshold:    f32,
}
