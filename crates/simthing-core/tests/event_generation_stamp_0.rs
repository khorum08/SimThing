//! EVENT-GENERATION-STAMP-0 synthetic contract witnesses.
//!
//! Inline trees only — no shipped scenario, no domain vocabulary, no cross-crate fixtures.

use simthing_core::{
    admit_dispatch_minted_overlay, dispatch_until_dissolved, integrate_stamped_product,
    integrate_unstamped_product_forbidden, replay_integration_schedule, BackpressurePolicy,
    DissolveCondition, GenerationStamp, GenerationStamped, IntegrateError, IntegrationSchedule,
    Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
    RingPushOutcome, SimPropertyId, SimThingId, StampedEgressEntry, StampedEventRing, SubFieldRole,
    TransformOp,
};

fn stamp(g: u32) -> GenerationStamp {
    GenerationStamp::new(g)
}

fn product(n: u32) -> u32 {
    n
}

fn product_key(n: u32) -> u64 {
    n as u64
}

// ── Second carrier: reduce-up / product stamps ───────────────────────────────

#[test]
fn stamped_product_integrates_at_parent_without_wait_including_async_lag() {
    // Parent at N+3 integrating child gen-N is the ORDINARY case.
    let parent = stamp(10);
    let child_product = GenerationStamped::stamp(stamp(7), product(42));
    let mut schedule = IntegrationSchedule::new();

    let receipt = integrate_stamped_product(parent, &child_product, product_key(42), &mut schedule);

    assert_eq!(receipt.parent_generation, parent);
    assert_eq!(receipt.child_generation, stamp(7));
    assert_eq!(receipt.staleness, 3, "staleness must be visible and attributable");
    assert_eq!(schedule.entries().len(), 1);
    assert_eq!(schedule.entries()[0].parent_generation, parent);
    assert_eq!(schedule.entries()[0].child_generation, stamp(7));
}

#[test]
fn unstamped_product_integration_hard_errors() {
    let mut schedule = IntegrationSchedule::new();
    let err = integrate_unstamped_product_forbidden(product_key(1), &mut schedule)
        .expect_err("unstamped integrate must hard-error");
    assert_eq!(err, IntegrateError::UnstampedProduct);
    assert!(
        schedule.entries().is_empty(),
        "failed unstamped integrate must not pollute the schedule"
    );
}

#[test]
fn make_the_parent_wait_mutant_is_absent_from_the_api() {
    // The integrate path has no wait/block parameter and always returns immediately.
    // A mutant that introduced waiting would change this function's contract; the
    // proof is that N+3 <- N completes in one call with no retry surface.
    let parent = stamp(100);
    let lagging = GenerationStamped::stamp(stamp(1), product(9));
    let mut schedule = IntegrationSchedule::new();
    let receipt = integrate_stamped_product(parent, &lagging, product_key(9), &mut schedule);
    assert_eq!(receipt.staleness, 99);
    // No second call, no pending flag, no wait token — async is ordinary.
    assert_eq!(schedule.entries().len(), 1);
}

#[test]
fn recorded_schedule_replay_is_bit_exact_and_ambient_timing_is_not_authority() {
    let products = vec![
        GenerationStamped::stamp(stamp(1), product(10)),
        GenerationStamped::stamp(stamp(2), product(20)),
        GenerationStamped::stamp(stamp(3), product(30)),
    ];
    let keys = vec![product_key(10), product_key(20), product_key(30)];

    // Author a schedule: parent 5 integrates child-gen-1 then child-gen-3 (skipping ambient order).
    let mut schedule = IntegrationSchedule::new();
    schedule.record(stamp(5), stamp(1), product_key(10));
    schedule.record(stamp(5), stamp(3), product_key(30));

    let receipts = replay_integration_schedule(&schedule, &products, &keys).expect("schedule present");
    assert_eq!(receipts.len(), 2);
    assert_eq!(receipts[0].child_generation, stamp(1));
    assert_eq!(receipts[1].child_generation, stamp(3));
    assert_eq!(receipts[0].staleness, 4);
    assert_eq!(receipts[1].staleness, 2);

    // Drop-the-schedule mutant: non-empty products, empty schedule → RED.
    let empty = IntegrationSchedule::new();
    let err = replay_integration_schedule(&empty, &products, &keys)
        .expect_err("ambient timing without schedule must hard-error");
    assert_eq!(err, IntegrateError::MissingSchedule);
}

#[test]
fn staleness_is_observable_from_stamp_alone() {
    let parent = stamp(12);
    let child = stamp(9);
    assert_eq!(parent.staleness_from_child(child), 3);
    assert!(child.is_stale_relative_to_parent(parent));
    assert!(!stamp(12).is_stale_relative_to_parent(parent));
    // Child ahead of parent is not "stale" for the parent (staleness is parent-relative lag).
    assert_eq!(stamp(5).staleness_from_child(stamp(8)), 0);
}

// ── Stamped ring egress / backpressure ───────────────────────────────────────

#[test]
fn forced_observer_lag_honors_overwrite_oldest_without_blocking() {
    let mut ring = StampedEventRing::admit(2, BackpressurePolicy::OverwriteOldest);
    let a = StampedEgressEntry {
        generation: stamp(1),
        key: 1,
        payload_bits: 11,
    };
    let b = StampedEgressEntry {
        generation: stamp(2),
        key: 2,
        payload_bits: 22,
    };
    let c = StampedEgressEntry {
        generation: stamp(3),
        key: 3,
        payload_bits: 33,
    };
    assert_eq!(ring.push(a), RingPushOutcome::Accepted);
    assert_eq!(ring.push(b), RingPushOutcome::Accepted);
    assert_eq!(ring.push(c), RingPushOutcome::OverwroteOldest);
    assert_eq!(ring.len(), 2);
    assert_eq!(ring.entries()[0].payload_bits, 22);
    assert_eq!(ring.entries()[1].payload_bits, 33);
    assert_eq!(ring.backpressure_actions, 1);
    // Observer lag drain does not require sim cooperation.
    let drained = ring.observer_drain(1);
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0].payload_bits, 22);
    assert_eq!(ring.len(), 1);
}

#[test]
fn forced_observer_lag_throttle_drops_without_perturbing_capacity() {
    let mut ring = StampedEventRing::admit(1, BackpressurePolicy::Throttle);
    assert_eq!(
        ring.push(StampedEgressEntry {
            generation: stamp(1),
            key: 1,
            payload_bits: 1,
        }),
        RingPushOutcome::Accepted
    );
    assert_eq!(
        ring.push(StampedEgressEntry {
            generation: stamp(2),
            key: 2,
            payload_bits: 2,
        }),
        RingPushOutcome::Throttled
    );
    assert_eq!(ring.len(), 1);
    assert_eq!(ring.entries()[0].payload_bits, 1);
    assert_eq!(ring.backpressure_actions, 1);
}

#[test]
fn coalesce_per_key_merges_same_key_under_lag() {
    let mut ring = StampedEventRing::admit(1, BackpressurePolicy::CoalescePerKey);
    assert_eq!(
        ring.push(StampedEgressEntry {
            generation: stamp(1),
            key: 7,
            payload_bits: 100,
        }),
        RingPushOutcome::Accepted
    );
    assert_eq!(
        ring.push(StampedEgressEntry {
            generation: stamp(2),
            key: 7,
            payload_bits: 200,
        }),
        RingPushOutcome::Coalesced
    );
    assert_eq!(ring.len(), 1);
    assert_eq!(ring.entries()[0].payload_bits, 200);
    assert_eq!(ring.entries()[0].generation, stamp(2));
}

// ── Dispatch dissolve discipline ─────────────────────────────────────────────

fn dispatch_overlay(conditions: Vec<DissolveCondition>) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Event,
        origin: SimThingId::new(),
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
        },
        lifecycle: dispatch_until_dissolved(conditions).expect("conditions non-empty"),
    }
}

#[test]
fn dispatch_minted_overlay_requires_until_dissolved_with_authored_condition() {
    let ok = dispatch_overlay(vec![DissolveCondition::AtSessionEnd]);
    admit_dispatch_minted_overlay(&ok).expect("authored condition admits");

    let bare = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Event,
        origin: SimThingId::new(),
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    };
    let err = admit_dispatch_minted_overlay(&bare).expect_err("unit UntilDissolved is not enough");
    assert!(matches!(
        err,
        simthing_core::DispatchOverlayError::MissingDissolveCondition
    ));

    let empty = dispatch_until_dissolved(Vec::new());
    assert!(matches!(
        empty,
        Err(simthing_core::DispatchOverlayError::MissingDissolveCondition)
    ));
}

#[test]
fn transient_lifecycle_is_rejected_for_dispatch_mint_path() {
    // Dispatch path requires UntilDissolvedWith, not Transient (order-weight uses Transient
    // as a player price injection, which is a different surface).
    let overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Event,
        origin: SimThingId::new(),
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AtSessionEnd],
        },
    };
    let err = admit_dispatch_minted_overlay(&overlay).expect_err("Transient is not UntilDissolved");
    assert!(matches!(
        err,
        simthing_core::DispatchOverlayError::NotUntilDissolved
    ));
}
