//! EVENT-GENERATION-STAMP-0 synthetic production-path witnesses.
//!
//! Inline trees only — no shipped scenario, no domain vocabulary, no cross-crate fixtures.

use simthing_core::{
    admit_dispatch_minted_overlay, deliver_routed_overlay, dispatch_until_dissolved,
    BackpressurePolicy, DissolveCondition, GenerationStamp, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, RingPushOutcome,
    RoutedGenerationDuration, SimPropertyId, SimThing, SimThingId, SimThingKind,
    StampedEgressEntry, StampedEventRing, SubFieldRole, TransformOp,
};

fn stamp(g: u32) -> GenerationStamp {
    GenerationStamp::new(g)
}

// ── Stamped ring (production-shaped backpressure surface) ────────────────────

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
