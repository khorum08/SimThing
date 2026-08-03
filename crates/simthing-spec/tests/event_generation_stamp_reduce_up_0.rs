//! EVENT-GENERATION-STAMP-0 — reduce-up second-carrier witnesses (synthetic).

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    integrate_unstamped_product_forbidden, GenerationStamp, IntegrationSchedule, SimThing,
    SimThingId, SimThingKind,
};
use simthing_spec::{
    integrate_stamped_reduce_up, reduce_owner_channel_rf, reduce_up_product_key,
    stamp_reduce_up_product, OwnerChannelRfOwnAggregate, ResourceKey,
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

#[test]
fn reduce_up_product_carries_generation_stamp_and_integrates_async() {
    let (root, rows) = two_owner_tree();
    let report = reduce_owner_channel_rf(&root, &rows).expect("reduce-up");
    let child_gen = GenerationStamp::new(3);
    let stamped = stamp_reduce_up_product(child_gen, report.clone());
    assert_eq!(stamped.generation(), child_gen);

    let parent_gen = GenerationStamp::new(6);
    let mut schedule = IntegrationSchedule::new();
    let receipt = integrate_stamped_reduce_up(parent_gen, &stamped, &mut schedule);
    assert_eq!(receipt.staleness, 3);
    assert_eq!(receipt.child_generation, child_gen);
    assert_eq!(receipt.product_key, reduce_up_product_key(&report));
    assert_eq!(schedule.entries().len(), 1);
}

#[test]
fn unstamped_reduce_up_integration_hard_errors() {
    let mut schedule = IntegrationSchedule::new();
    let err = integrate_unstamped_product_forbidden(0, &mut schedule).unwrap_err();
    assert!(matches!(
        err,
        simthing_core::IntegrateError::UnstampedProduct
    ));
}

#[test]
fn parent_n_plus_3_integrating_child_n_is_ordinary_with_no_wait() {
    let (root, rows) = two_owner_tree();
    let report = reduce_owner_channel_rf(&root, &rows).expect("reduce-up");
    let stamped = stamp_reduce_up_product(GenerationStamp::new(1), report);
    let mut schedule = IntegrationSchedule::new();
    // Single-call completion is the no-wait proof.
    let receipt =
        integrate_stamped_reduce_up(GenerationStamp::new(4), &stamped, &mut schedule);
    assert_eq!(receipt.staleness, 3);
    assert_eq!(schedule.entries().len(), 1);
}
