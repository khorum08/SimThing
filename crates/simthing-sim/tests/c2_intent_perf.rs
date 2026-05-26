//! C-2 performance gate: AccumulatorOp intent path vs legacy intent pass.
//!
//! Conservative gate per `docs/workshop/c1_perf_reframe_memo.md`: no regression
//! on the full tick pipeline path, with a 1.5× warning threshold.

use simthing_core::{
    DimensionRegistry, PropertyTransformDelta, SimProperty, SimThing, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_feeder::{
    feeder_channel, DispatchCoordinator, FeederWork, PatchTransform, TransformPatcher,
};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, WorldGpuState};
use simthing_sim::BoundaryProtocol;
use std::time::Instant;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

#[test]
#[ignore = "C-2 perf gate: manual/noisy; run with --ignored for perf report"]
fn c2_intent_perf_no_regression() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    const N_SLOTS: u32 = 256;
    const N_INTENTS: u32 = 2_048;
    const N_TICKS: u32 = 50;
    const N_WARMUP: u32 = 5;

    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "loyalty", 0));
    let pid = reg.id_of("core", "loyalty").unwrap();
    let n_dims = reg.total_columns as u32;

    let mut ids = Vec::with_capacity(N_SLOTS as usize);
    let mut alloc = SlotAllocator::new();
    for _ in 0..N_SLOTS {
        let id = SimThing::new(SimThingKind::Cohort, 0).id;
        alloc.alloc(id);
        ids.push(id);
    }

    let measure = |use_accumulator_intent: bool| -> f64 {
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
        let pipelines = Pipelines::new(&state.ctx);
        let mut patcher = TransformPatcher::new(N_SLOTS as usize);
        let mut coord = DispatchCoordinator::new(N_SLOTS, n_dims, N_TICKS);
        let (tx, rx) = feeder_channel();

        let world = SimThing::new(SimThingKind::World, 0);
        let mut proto = BoundaryProtocol::new(world, reg.clone(), alloc.clone());
        proto.flags.use_accumulator_intent = use_accumulator_intent;
        proto.initial_gpu_sync(&coord, &mut state);

        let mk = |slot_idx: u32| PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: vec![(
                SubFieldRole::Amount,
                TransformOp::Add(0.001 * slot_idx as f32),
            )],
        };

        for _ in 0..N_WARMUP {
            for i in 0..N_INTENTS {
                tx.send(FeederWork::Patch(PatchTransform {
                    target: ids[(i % N_SLOTS) as usize],
                    delta: mk(i),
                }))
                .unwrap();
            }
            let _ = coord.tick(
                &rx,
                &mut patcher,
                &proto.registry,
                &proto.allocator,
                &pipelines,
                &mut state,
                0.0,
            );
        }

        let started = Instant::now();
        for _ in 0..N_TICKS {
            for i in 0..N_INTENTS {
                tx.send(FeederWork::Patch(PatchTransform {
                    target: ids[(i % N_SLOTS) as usize],
                    delta: mk(i),
                }))
                .unwrap();
            }
            let _ = coord.tick(
                &rx,
                &mut patcher,
                &proto.registry,
                &proto.allocator,
                &pipelines,
                &mut state,
                0.0,
            );
        }
        started.elapsed().as_secs_f64() * 1000.0 / N_TICKS as f64
    };

    let old_ms = measure(false);
    let new_ms = measure(true);
    let ratio = old_ms / new_ms.max(f64::MIN_POSITIVE);

    eprintln!(
        "c2 perf (tick pipeline): n_slots={N_SLOTS} n_intent_deltas={N_INTENTS} \
         old_ms={old_ms:.4} new_ms={new_ms:.4} ratio={ratio:.2}x"
    );

    const NO_REGRESSION_RATIO: f64 = 1.0;
    const WARNING_RATIO: f64 = 1.5;
    if ratio < WARNING_RATIO {
        eprintln!("WARN: c2 tick pipeline ratio {ratio:.2}x below {WARNING_RATIO:.1}x threshold");
    }
    assert!(
        ratio >= NO_REGRESSION_RATIO,
        "C-2 tick pipeline regressed: ratio={ratio:.2}x (old={old_ms:.4}ms new={new_ms:.4}ms)"
    );
}
