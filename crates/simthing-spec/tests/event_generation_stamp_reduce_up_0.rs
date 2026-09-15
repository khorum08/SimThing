//! EVENT-GENERATION-STAMP-0 — reduce-up second-carrier production-path witnesses (synthetic).
//!
//! Remand 2 + DA addendum (HD-RECEIPT 9df0629526ec): no production wait path;
//! schedule is per-product full generation set.

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{GenerationStamp, IntegrationSchedule, SimThing, SimThingId, SimThingKind};
use simthing_spec::{
    integrate_raw_reduce_up_report_forbidden, integrate_stamped_reduce_up, reduce_owner_channel_rf,
    reduce_up_product_key, replay_reduce_up_schedule, OwnerChannelRfOwnAggregate,
    ParentRfIntegrationState, ResourceKey,
};

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("synthetic".into()), 0)
}

fn own(
    simthing_id: SimThingId,
    resource: &str,
    surplus: u32,
    deficit: u32,
) -> OwnerChannelRfOwnAggregate {
    OwnerChannelRfOwnAggregate {
        simthing_id,
        resource_key: ResourceKey::new(resource),
        surplus,
        deficit,
    }
}

fn two_owner_tree() -> (SimThing, Vec<OwnerChannelRfOwnAggregate>) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("alpha"));
    let mut inherited = node();
    let leaf = node();
    let leaf_id = leaf.id;
    inherited.add_child(leaf);
    let mut crossing = node();
    bind_owner(&mut crossing, &OwnerRef::new("beta"));
    let crossing_id = crossing.id;
    let root_id = root.id;
    root.add_child(inherited);
    root.add_child(crossing);
    let rows = vec![
        own(root_id, "ore", 3, 0),
        own(leaf_id, "ore", 4, 0),
        own(crossing_id, "ore", 5, 1),
    ];
    (root, rows)
}

