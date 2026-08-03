//! EVENT-GENERATION-STAMP-0 synthetic production-path witnesses.
//!
//! Inline trees only — no shipped scenario, no domain vocabulary, no cross-crate fixtures.

use simthing_core::{
    admit_dispatch_minted_overlay, deliver_routed_overlay, dispatch_until_dissolved,
    BackpressurePolicy, DissolveCondition, GenerationStamp, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, RingPushOutcome, SimPropertyId,
    SimThing, SimThingId, SimThingKind, StampedEgressEntry, StampedEventRing, SubFieldRole,
    TransformOp,
};

fn stamp(g: u32) -> GenerationStamp {
    GenerationStamp::new(g)
}

// ── Stamped ring (production-shaped backpressure surface) ────────────────────

#[test]
fn forced_observer_lag_honors_overwrite_oldest_without_blocking() {
    let mut ring = StampedEventRing::admit(2, BackpressurePolicy::OverwriteOldest);
    assert_eq!(
        ring.push(StampedEgressEntry {
            generation: stamp(1),
            key: 1,
            payload_bits: 11,
        }),
        RingPushOutcome::Accepted
    );
    assert_eq!(
        ring.push(StampedEgressEntry {
            generation: stamp(2),
            key: 2,
            payload_bits: 22,
        }),
        RingPushOutcome::Accepted
    );
    assert_eq!(
        ring.push(StampedEgressEntry {
            generation: stamp(3),
            key: 3,
            payload_bits: 33,
        }),
        RingPushOutcome::OverwroteOldest
    );
    assert_eq!(ring.len(), 2);
    assert_eq!(ring.entries()[0].payload_bits, 22);
    assert_eq!(ring.backpressure_actions, 1);
    let drained = ring.observer_drain(1);
    assert_eq!(drained[0].payload_bits, 22);
}

#[test]
fn forced_observer_lag_throttle_and_coalesce_without_perturbing_sim() {
    let mut throttle = StampedEventRing::admit(1, BackpressurePolicy::Throttle);
    assert_eq!(
        throttle.push(StampedEgressEntry {
            generation: stamp(1),
            key: 1,
            payload_bits: 1,
        }),
        RingPushOutcome::Accepted
    );
    assert_eq!(
        throttle.push(StampedEgressEntry {
            generation: stamp(2),
            key: 2,
            payload_bits: 2,
        }),
        RingPushOutcome::Throttled
    );
    assert_eq!(throttle.entries()[0].payload_bits, 1);

    let mut coalesce = StampedEventRing::admit(1, BackpressurePolicy::CoalescePerKey);
    assert_eq!(
        coalesce.push(StampedEgressEntry {
            generation: stamp(1),
            key: 7,
            payload_bits: 100,
        }),
        RingPushOutcome::Accepted
    );
    assert_eq!(
        coalesce.push(StampedEgressEntry {
            generation: stamp(2),
            key: 7,
            payload_bits: 200,
        }),
        RingPushOutcome::Coalesced
    );
    assert_eq!(coalesce.entries()[0].payload_bits, 200);
}

// ── Dispatch dissolve at the production delivery door ────────────────────────

fn dispatch_instruction(origin: SimThingId, lifecycle: OverlayLifecycle) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Event,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(1.0))],
        },
        lifecycle,
    }
}

#[test]
fn production_delivery_rejects_dispatch_mint_without_authored_dissolve() {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    let origin = child.id;
    let target = child.id;
    root.add_child(child);

    let bare = dispatch_instruction(origin, OverlayLifecycle::UntilDissolved);
    let err = deliver_routed_overlay(&mut root, target, bare)
        .expect_err("unit UntilDissolved dispatch mint must RED at production delivery");
    assert!(matches!(
        err,
        simthing_core::OverlayDeliveryError::DispatchDissolveRequired { .. }
    ));

    let ok = dispatch_instruction(
        origin,
        dispatch_until_dissolved(vec![DissolveCondition::AtSessionEnd]).unwrap(),
    );
    deliver_routed_overlay(&mut root, target, ok).expect("authored dissolve admits");
}

#[test]
fn admit_dispatch_helpers_match_production_door() {
    let ok = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Event,
        origin: SimThingId::new(),
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(1.0))],
        },
        lifecycle: dispatch_until_dissolved(vec![DissolveCondition::AtSessionEnd]).unwrap(),
    };
    admit_dispatch_minted_overlay(&ok).unwrap();

    let bare = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Event,
        origin: SimThingId::new(),
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(1.0))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    };
    assert!(admit_dispatch_minted_overlay(&bare).is_err());
    assert!(dispatch_until_dissolved(Vec::new()).is_err());
}
