//! ASYNC-COMMAND-QUEUE-0 — synthetic seam, conservation, and replay proofs.

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    capture_ancestor_standing_policy, AncestorStandingPolicyView, AuthoredSeamStaleness,
    GenerationStamp, GenerationStamped, IntegrateError, IntegrationSchedule,
    IntegrationScheduleRowKind, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_spec::{
    reduce_owner_channel_rf, replay_async_owner_channel_rf_seam, AsyncOwnerChannelRfSeam,
    OwnerChannelRfOwnAggregate, ParentRfIntegrationState, ResourceKey,
};

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("synthetic".into()), 0)
}

fn source_tree() -> (SimThing, Vec<OwnerChannelRfOwnAggregate>) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("alpha"));
    let inherited = node();
    let inherited_id = inherited.id;
    let mut crossing = node();
    bind_owner(&mut crossing, &OwnerRef::new("beta"));
    let crossing_id = crossing.id;
    root.add_child(inherited);
    root.add_child(crossing);
    let rows = vec![
        OwnerChannelRfOwnAggregate {
            simthing_id: inherited_id,
            resource_key: ResourceKey::new("resource-a"),
            surplus: 7,
            deficit: 2,
        },
        OwnerChannelRfOwnAggregate {
            simthing_id: crossing_id,
            resource_key: ResourceKey::new("resource-a"),
            surplus: 3,
            deficit: 5,
        },
    ];
    (root, rows)
}

fn product_at(generation: u32) -> GenerationStamped<simthing_spec::OwnerChannelRfReduceUpReport> {
    let (root, rows) = source_tree();
    reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(generation)).expect("reduce-up")
}

fn products_at(
    generations: impl IntoIterator<Item = u32>,
) -> Vec<GenerationStamped<simthing_spec::OwnerChannelRfReduceUpReport>> {
    let (root, rows) = source_tree();
    generations
        .into_iter()
        .map(|generation| {
            reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(generation))
                .expect("reduce-up")
        })
        .collect()
}

fn policy(origin: SimThingId, amount: f32) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(77),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(amount))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

#[test]
fn same_key_burst_coalesces_exactly_conserves_and_replay_mutants_red() {
    let products = products_at(1..=5);
    let first = products[0].product();
    let mut seam = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(3));
    for product in &products {
        seam.enqueue_reduce_up(product)
            .expect("nonblocking enqueue");
    }

    assert_eq!(seam.pending_len(), first.buckets.len());
    let carriers = seam.pending_carriers();
    assert_eq!(carriers.len(), first.buckets.len());
    for (carrier, original) in carriers.iter().zip(&first.buckets) {
        assert_eq!(carrier.generation(), GenerationStamp::new(5));
        assert_eq!(carrier.product().scope, original.scope);
        assert_eq!(
            carrier.product().value.participant_count,
            u64::from(original.participant_count) * 5
        );
        assert_eq!(
            carrier.product().value.surplus_total,
            u64::from(original.surplus_total) * 5
        );
        assert_eq!(
            carrier.product().value.deficit_total,
            u64::from(original.deficit_total) * 5
        );
        assert_eq!(
            carrier.product().value.net_surplus,
            u64::from(original.net_surplus) * 5
        );
        assert_eq!(
            carrier.product().value.net_deficit,
            u64::from(original.net_deficit) * 5
        );
        let held = seam.balance(&original.scope).expect("accounted scope");
        assert!(held.is_exact());
        assert_eq!(held.child(), Default::default());
        assert_eq!(held.seam(), carrier.product().value);
        assert_eq!(held.parent(), Default::default());
    }

    let mut parent = ParentRfIntegrationState::default();
    let mut schedule = IntegrationSchedule::new();
    let receipt = seam
        .apply_parent_generation_barrier(GenerationStamp::new(8), &mut parent, &mut schedule)
        .expect("lagged child never waits");
    assert_eq!(receipt.distinct_bucket_count, first.buckets.len());
    assert_eq!(receipt.contributing_product_count, products.len());
    assert!(seam.is_empty());
    assert_eq!(
        schedule
            .entries_of_kind(IntegrationScheduleRowKind::QueueInjection)
            .count(),
        products.len()
    );
    for original in &first.buckets {
        let landed = parent.buckets.get(&original.scope).expect("parent bucket");
        assert_eq!(
            landed.participant_count,
            u64::from(original.participant_count) * 5
        );
        let balance = seam.balance(&original.scope).expect("accounted scope");
        assert!(balance.is_exact());
        assert_eq!(balance.child(), Default::default());
        assert_eq!(balance.seam(), Default::default());
        assert_eq!(balance.parent(), balance.admitted());
    }

    let queue_rows: Vec<_> = schedule
        .entries_of_kind(IntegrationScheduleRowKind::QueueInjection)
        .collect();
    let generations: Vec<_> = queue_rows
        .iter()
        .map(|entry| entry.child_generation.get())
        .collect();
    assert_eq!(generations, vec![1, 2, 3, 4, 5]);

    let mut rejected = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(2));
    for product in &products {
        rejected.enqueue_reduce_up(product).unwrap();
    }
    let rejected_pending_before = rejected.pending_carriers();
    let mut rejected_parent = ParentRfIntegrationState::default();
    let mut rejected_schedule = IntegrationSchedule::new();
    let error = rejected
        .apply_parent_generation_barrier(
            GenerationStamp::new(8),
            &mut rejected_parent,
            &mut rejected_schedule,
        )
        .expect_err("newest carrier stamp outside authored tolerance must hard-error");
    assert!(matches!(
        error,
        IntegrateError::StalenessToleranceExceeded {
            integration: 8,
            source_generation: 5,
            observed: 3,
            allowed: 2,
        }
    ));
    assert_eq!(rejected.pending_carriers(), rejected_pending_before);
    assert_eq!(rejected_parent, ParentRfIntegrationState::default());
    assert!(rejected_schedule.entries().is_empty());

    let mut ambient_reversed = products.clone();
    ambient_reversed.reverse();
    let replay = replay_async_owner_channel_rf_seam(&schedule, &ambient_reversed, &[])
        .expect("one-log replay");
    assert_eq!(replay.parent_state, parent);

    let mut latest_only = schedule.clone();
    let newest_index = latest_only
        .entries
        .iter()
        .rposition(|entry| entry.row_kind() == IntegrationScheduleRowKind::QueueInjection)
        .unwrap();
    latest_only.entries = vec![latest_only.entries[newest_index].clone()];
    let collapsed = replay_async_owner_channel_rf_seam(&latest_only, &products, &[]).unwrap();
    assert_ne!(
        collapsed.parent_state, parent,
        "bucket-latest collapse must RED"
    );
    assert!(matches!(
        replay_async_owner_channel_rf_seam(&IntegrationSchedule::new(), &products, &[])
            .unwrap_err(),
        IntegrateError::MissingSchedule
    ));
}

#[test]
fn slow_child_never_blocks_and_authored_tolerance_breach_is_atomic_hard_error() {
    let product = product_at(1);
    let mut admitted = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(3));
    admitted.enqueue_reduce_up(&product).unwrap();
    admitted
        .apply_parent_generation_barrier(
            GenerationStamp::new(4),
            &mut ParentRfIntegrationState::default(),
            &mut IntegrationSchedule::new(),
        )
        .expect("N+3 <- N is admitted without waiting");

    let mut rejected = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(2));
    rejected.enqueue_reduce_up(&product).unwrap();
    let pending_before = rejected.pending_carriers();
    let mut parent = ParentRfIntegrationState::default();
    let mut schedule = IntegrationSchedule::new();
    let error = rejected
        .apply_parent_generation_barrier(GenerationStamp::new(4), &mut parent, &mut schedule)
        .expect_err("authored tolerance breach must hard-error");
    assert!(matches!(
        error,
        IntegrateError::StalenessToleranceExceeded {
            integration: 4,
            source_generation: 1,
            observed: 3,
            allowed: 2,
        }
    ));
    assert_eq!(rejected.pending_carriers(), pending_before);
    assert_eq!(parent, ParentRfIntegrationState::default());
    assert!(schedule.entries().is_empty());
}

#[test]
fn one_schedule_replays_upward_products_and_torn_free_downward_standing_reads_bit_exactly() {
    let product = product_at(3);
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("canonical-owner"));
    root.add_overlay(policy(root.id, 1.25));
    let mut child = node();
    let child_id = child.id;
    child.add_overlay(policy(child_id, 99.0));
    root.add_child(child);
    let captured = capture_ancestor_standing_policy(&root, child_id).expect("local capture");
    assert_eq!(
        captured.len(),
        1,
        "child-local policy stays in its own site"
    );
    let view_one = GenerationStamped::stamp(
        GenerationStamp::new(4),
        AncestorStandingPolicyView::new(OwnerRef::new("canonical-owner"), captured),
    );

    let mut seam = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(8));
    let mut schedule = IntegrationSchedule::new();
    let mut parent = ParentRfIntegrationState::default();
    seam.enqueue_reduce_up(&product).unwrap();
    seam.apply_parent_generation_barrier(GenerationStamp::new(6), &mut parent, &mut schedule)
        .unwrap();
    seam.stage_ancestor_standing_view(view_one.clone());
    seam.apply_child_generation_barrier(GenerationStamp::new(6), &mut schedule)
        .unwrap();
    assert_eq!(
        seam.standing_view(GenerationStamp::new(6)).unwrap(),
        &view_one
    );

    let view_two = GenerationStamped::stamp(
        GenerationStamp::new(5),
        AncestorStandingPolicyView::new(
            OwnerRef::new("canonical-owner"),
            vec![policy(root.id, 9.5)],
        ),
    );
    seam.stage_ancestor_standing_view(view_two.clone());
    // Staging cannot tear the published generation/value pair.
    assert_eq!(
        seam.standing_view(GenerationStamp::new(6)).unwrap(),
        &view_one
    );
    seam.apply_child_generation_barrier(GenerationStamp::new(7), &mut schedule)
        .unwrap();
    assert_eq!(
        seam.standing_view(GenerationStamp::new(7)).unwrap(),
        &view_two
    );
    assert!(seam
        .apply_child_generation_barrier(GenerationStamp::new(7), &mut schedule)
        .unwrap()
        .is_none());

    assert_eq!(
        schedule
            .entries_of_kind(IntegrationScheduleRowKind::QueueInjection)
            .count(),
        1
    );
    assert_eq!(
        schedule
            .entries_of_kind(IntegrationScheduleRowKind::StandingView)
            .count(),
        2
    );
    let replay = replay_async_owner_channel_rf_seam(
        &schedule,
        &[product],
        &[view_two.clone(), view_one.clone()],
    )
    .expect("mixed-direction replay from one log");
    assert_eq!(replay.parent_state, parent);
    assert_eq!(replay.standing_reads, vec![view_one.clone(), view_two]);

    let encoded = serde_json::to_string(view_one.product()).expect("standing view encodes");
    assert!(encoded.contains("canonical-owner"));
}
