//! EVENT-GENERATION-STAMP-0 — reduce-up second-carrier production-path witnesses (synthetic).
//!
//! Remand 2 + DA addendum (HD-RECEIPT 9df0629526ec): no production wait path;
//! schedule is per-product full generation set.

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    GenerationStamp, IntegrationSchedule, SimThing, SimThingId, SimThingKind,
};
use simthing_spec::{
    integrate_raw_reduce_up_report_forbidden, integrate_stamped_reduce_up,
    reduce_owner_channel_rf, reduce_up_product_key, replay_reduce_up_schedule,
    OwnerChannelRfOwnAggregate, ParentRfIntegrationState, ResourceKey,
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
fn production_reduce_up_returns_stamped_product_and_integrates_into_parent_state() {
    let (root, rows) = two_owner_tree();
    let child_gen = GenerationStamp::new(3);
    let stamped = reduce_owner_channel_rf(&root, &rows, child_gen).expect("reduce-up");
    assert_eq!(stamped.generation(), child_gen);

    let parent_gen = GenerationStamp::new(6);
    let mut schedule = IntegrationSchedule::new();
    let mut parent = ParentRfIntegrationState::default();
    let receipt =
        integrate_stamped_reduce_up(parent_gen, &stamped, &mut parent, &mut schedule).unwrap();
    assert_eq!(receipt.staleness, 3);
    assert_eq!(parent.product_count, 1);
    assert_eq!(parent.surplus_total, stamped.product().surplus_total);
}

#[test]
fn raw_unstamped_report_is_rejected_at_production_integration_door() {
    let (root, rows) = two_owner_tree();
    let stamped = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(1)).unwrap();
    let raw = stamped.product().clone();
    let mut schedule = IntegrationSchedule::new();
    let mut parent = ParentRfIntegrationState::default();
    let err = integrate_raw_reduce_up_report_forbidden(&raw, &mut parent, &mut schedule).unwrap_err();
    assert!(matches!(err, simthing_core::IntegrateError::UnstampedProduct));
}

#[test]
fn production_integrate_n_plus_3_from_n_never_waits() {
    let (root, rows) = two_owner_tree();
    let stamped = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(1)).unwrap();
    let mut schedule = IntegrationSchedule::new();
    let mut parent = ParentRfIntegrationState::default();

    // Production path: N+3 <- N is ordinary, no wait, no freshness error.
    integrate_stamped_reduce_up(
        GenerationStamp::new(4),
        &stamped,
        &mut parent,
        &mut schedule,
    )
    .expect("production path has no wait branch");
    assert_eq!(parent.product_count, 1);
    assert_eq!(schedule.entries().len(), 1);
    assert_eq!(schedule.entries()[0].child_generation, GenerationStamp::new(1));
}

#[test]
fn schedule_is_per_product_full_generation_set_and_replays_bit_exactly() {
    let (root, rows) = two_owner_tree();
    let p1 = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(1)).unwrap();
    let p2 = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(2)).unwrap();
    // Same product_key family (identical conserved totals) at two generations.
    assert_eq!(
        reduce_up_product_key(p1.product()),
        reduce_up_product_key(p2.product())
    );
    let key = reduce_up_product_key(p1.product());

    let mut schedule = IntegrationSchedule::new();
    let mut live = ParentRfIntegrationState::default();
    integrate_stamped_reduce_up(GenerationStamp::new(5), &p1, &mut live, &mut schedule).unwrap();
    integrate_stamped_reduce_up(GenerationStamp::new(5), &p2, &mut live, &mut schedule).unwrap();

    // Full generation set preserved — never per-bucket-latest collapse.
    let gens = schedule.child_generations_for_key(key);
    assert_eq!(gens.len(), 2, "schedule must keep both product generations");
    assert!(gens.contains(&GenerationStamp::new(1)));
    assert!(gens.contains(&GenerationStamp::new(2)));
    assert_eq!(schedule.entries().len(), 2);

    // Values sum under integration; stamps remain distinct in the schedule.
    assert_eq!(live.surplus_total, p1.product().surplus_total * 2);

    let products = vec![p2.clone(), p1.clone()]; // ambient order reversed
    let replayed = replay_reduce_up_schedule(&schedule, &products).unwrap();
    assert_eq!(replayed, live);

    let empty = IntegrationSchedule::new();
    assert!(matches!(
        replay_reduce_up_schedule(&empty, &products).unwrap_err(),
        simthing_core::IntegrateError::MissingSchedule
    ));
}
