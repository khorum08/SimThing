//! EVENT-GENERATION-STAMP-0 — reduce-up second-carrier production-path witnesses (synthetic).

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    GenerationStamp, IntegrationSchedule, SimThing, SimThingId, SimThingKind,
};
use simthing_spec::{
    integrate_raw_reduce_up_report_forbidden, integrate_stamped_reduce_up,
    plant_wait_for_fresh_child_mutant, reduce_owner_channel_rf, reduce_up_product_key,
    replay_reduce_up_schedule, OwnerChannelRfOwnAggregate, ParentRfIntegrationState, ResourceKey,
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
    assert!(stamped.product().bucket_count > 0);

    let parent_gen = GenerationStamp::new(6);
    let mut schedule = IntegrationSchedule::new();
    let mut parent = ParentRfIntegrationState::default();
    let receipt =
        integrate_stamped_reduce_up(parent_gen, &stamped, &mut parent, &mut schedule).unwrap();
    assert_eq!(receipt.staleness, 3);
    assert_eq!(receipt.product_key, reduce_up_product_key(stamped.product()));
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
    assert_eq!(parent.product_count, 0);
}

#[test]
fn parent_n_plus_3_integrating_child_n_is_ordinary_and_wait_mutant_reds() {
    let (root, rows) = two_owner_tree();
    let stamped = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(1)).unwrap();
    let mut schedule = IntegrationSchedule::new();
    let mut parent = ParentRfIntegrationState::default();

    // Ordinary path: no wait.
    integrate_stamped_reduce_up(
        GenerationStamp::new(4),
        &stamped,
        &mut parent,
        &mut schedule,
    )
    .expect("async ordinary");
    assert_eq!(parent.product_count, 1);

    // Plant wait mutant → N+3 <- N REDs.
    plant_wait_for_fresh_child_mutant(true);
    let mut schedule2 = IntegrationSchedule::new();
    let mut parent2 = ParentRfIntegrationState::default();
    let err = integrate_stamped_reduce_up(
        GenerationStamp::new(4),
        &stamped,
        &mut parent2,
        &mut schedule2,
    )
    .expect_err("wait mutant must RED lagged integration");
    assert!(matches!(
        err,
        simthing_core::IntegrateError::WouldWaitForLaggingChild { parent: 4, child: 1 }
    ));
    plant_wait_for_fresh_child_mutant(false);

    // Restored ordinary path is green again.
    integrate_stamped_reduce_up(
        GenerationStamp::new(4),
        &stamped,
        &mut parent2,
        &mut schedule2,
    )
    .expect("restored ordinary path");
}

#[test]
fn schedule_replay_reproduces_integrated_parent_state_bit_exactly() {
    let (root, rows) = two_owner_tree();
    let p1 = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(1)).unwrap();
    // Second product: same tree, different generation identity.
    let p2 = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(2)).unwrap();

    let mut schedule = IntegrationSchedule::new();
    let mut live = ParentRfIntegrationState::default();
    // Integrate out of ambient arrival order: gen2 first, then gen1.
    integrate_stamped_reduce_up(GenerationStamp::new(5), &p2, &mut live, &mut schedule).unwrap();
    integrate_stamped_reduce_up(GenerationStamp::new(5), &p1, &mut live, &mut schedule).unwrap();

    // Present products in the opposite ambient order; schedule drives selection.
    let products = vec![p1.clone(), p2.clone()];
    let replayed = replay_reduce_up_schedule(&schedule, &products).unwrap();
    assert_eq!(replayed, live, "replay must match live integrated state bit-exactly");

    // Drop schedule mutant REDs.
    let empty = IntegrationSchedule::new();
    let err = replay_reduce_up_schedule(&empty, &products).unwrap_err();
    assert!(matches!(err, simthing_core::IntegrateError::MissingSchedule));
}
