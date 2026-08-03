//! Release-only generation-level evidence for BAND-QUANTIZED-DRAW-0.
//!
//! The file intentionally compiles unchanged at pre-join SHA
//! `7d9766299be96e4b35da02e678c88b985307b176` and the specialized head: the
//! historical `Set`/`Add`/`Multiply` wire shape constructs the operation on
//! both sides, while the ordinary evaluator supplies the generation workload.

use std::hint::black_box;
use std::time::Instant;

use simthing_core::evaluate::Evaluator;
use simthing_core::{
    DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimProperty, SimThing, SimThingKind, SubFieldRole, TransformOp,
};

const PARTICIPANTS: usize = 40_000;
const OVERLAY_APPLICATIONS_PER_PARTICIPANT: usize = 3;
const OVERLAY_APPLICATIONS_PER_GENERATION: usize =
    PARTICIPANTS * OVERLAY_APPLICATIONS_PER_PARTICIPANT;
const WARMUP_GENERATIONS: u32 = 3;
const MEASURED_GENERATIONS: u32 = 15;

fn wire_op(name: &str, value: f32) -> TransformOp {
    serde_json::from_str(&format!(r#"{{"{name}":{value}}}"#))
        .expect("historical TransformOp wire shape admits")
}

fn representative_tree() -> (DimensionRegistry, SimThing, simthing_core::SimPropertyId) {
    let mut registry = DimensionRegistry::new();
    let property_id = registry.register(SimProperty::simple("bench", "amount", 0));
    let property = registry.property(property_id);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let root_id = root.id;
    root.add_overlay(Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Governance,
        source: OverlaySource::System,
        origin: root_id,
        affects: vec![root_id],
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![
                (SubFieldRole::Amount, wire_op("Set", 0.25)),
                (SubFieldRole::Amount, wire_op("Add", 0.5)),
                (SubFieldRole::Amount, wire_op("Multiply", 1.25)),
            ],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    });

    for _ in 0..PARTICIPANTS {
        let mut participant = SimThing::new(SimThingKind::Cohort, 0);
        let mut value = property.default_value();
        value.set_role(&SubFieldRole::Amount, &property.layout, 2.0);
        participant.add_property(property_id, value);
        root.add_child(participant);
    }
    (registry, root, property_id)
}

fn assert_semantic_result(
    snapshot: &simthing_core::evaluate::FieldSnapshot,
    root: &SimThing,
    registry: &DimensionRegistry,
    property_id: simthing_core::SimPropertyId,
) {
    let participant = root.children.last().expect("representative participants");
    let amount = snapshot
        .get(participant.id)
        .and_then(|entity| entity.properties.get(&property_id))
        .expect("participant property")
        .get_role(
            &SubFieldRole::Amount,
            &registry.property(property_id).layout,
        );
    assert_eq!(amount.to_bits(), 0.9375f32.to_bits());
}

#[test]
#[ignore = "release-only exact-head generation wall-clock evidence"]
fn representative_generation_wall_clock() {
    let (registry, root, property_id) = representative_tree();
    let evaluator = Evaluator::new(&registry, 0.0);

    for generation in 0..WARMUP_GENERATIONS {
        let snapshot = evaluator.evaluate(&root, generation);
        assert_semantic_result(&snapshot, &root, &registry, property_id);
        black_box(snapshot);
    }

    let mut samples_ns = Vec::with_capacity(MEASURED_GENERATIONS as usize);
    for generation in 0..MEASURED_GENERATIONS {
        let start = Instant::now();
        let snapshot = evaluator.evaluate(&root, generation);
        let elapsed = start.elapsed().as_nanos() as u64;
        assert_semantic_result(&snapshot, &root, &registry, property_id);
        black_box(snapshot);
        samples_ns.push(elapsed);
    }

    let mut sorted = samples_ns.clone();
    sorted.sort_unstable();
    let median_ns = sorted[sorted.len() / 2];
    let p10_ns = sorted[sorted.len() / 10];
    let p90_ns = sorted[(sorted.len() * 9) / 10];
    let variant = std::env::var("BAND_BENCH_VARIANT").unwrap_or_else(|_| "unspecified".into());
    eprintln!(
        "BAND-QUANTIZED-DRAW-0 generation benchmark: variant={variant} participants={PARTICIPANTS} \
         overlay_applications_per_generation={OVERLAY_APPLICATIONS_PER_GENERATION} \
         warmup_generations={WARMUP_GENERATIONS} measured_generations={MEASURED_GENERATIONS} \
         samples_ns={samples_ns:?} median_ns={median_ns} p10_ns={p10_ns} p90_ns={p90_ns}"
    );
}
