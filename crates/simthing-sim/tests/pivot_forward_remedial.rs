//! Pivot-forward remedial: authoritative feature flags and family isolation.

use simthing_core::{
    DimensionRegistry, IntensityBehavior, PropertyTransformDelta, PropertyValue, SimProperty,
    SimPropertyId, SimThing, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_feeder::{
    feeder_channel, DispatchCoordinator, FeederWork, PatchTransform, TransformPatcher,
};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, WorldGpuState};
use simthing_sim::BoundaryProtocol;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn loyalty_world(
    n_cohorts: usize,
) -> (
    DimensionRegistry,
    SlotAllocator,
    SimThing,
    SimPropertyId,
    u32,
) {
    let mut reg = DimensionRegistry::new();
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    let pid = reg.register(p);
    let mut alloc = SlotAllocator::new();
    let mut world = SimThing::new(SimThingKind::World, 0);
    for _ in 0..n_cohorts {
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
        let id = cohort.id;
        alloc.alloc(id);
        world.add_child(cohort);
    }
    let n_dims = reg.total_columns as u32;
    (reg, alloc, world, pid, n_dims)
}

#[test]
fn c2_flag_off_clears_stale_intent_accumulator() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (reg, alloc, world, pid, n_dims) = loyalty_world(2);
    let n_slots = alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
    let (tx, rx) = feeder_channel();
    coord.shadow.fill(0.5);
    coord.upload_full_shadow(&state);

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.flags.use_accumulator_intent = true;
    proto.initial_gpu_sync(&coord, &mut state);
    assert!(state
        .accumulator_runtime
        .as_ref()
        .is_some_and(|r| r.intent_active()));

    proto.flags.use_accumulator_intent = false;
    proto.initial_gpu_sync(&coord, &mut state);
    assert!(state
        .accumulator_runtime
        .as_ref()
        .is_none_or(|r| !r.intent_active()));

    let cohort_id = proto.root.children[0].id;
    tx.send(FeederWork::Patch(PatchTransform {
        target: cohort_id,
        delta: PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.1))],
        },
    }))
    .unwrap();

    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        coord.tick(
            &rx,
            &mut patcher,
            &proto.registry,
            &proto.allocator,
            &pipelines,
            &mut state,
            1.0,
        );
    }))
    .expect_err("S-1 deleted legacy intent; flag-off pending intent must reject");
    let msg = err
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| err.downcast_ref::<&str>().copied())
        .unwrap_or("");
    assert!(
        msg.contains("Legacy intent_delta.wgsl was deleted in S-1"),
        "unexpected panic: {msg}"
    );
}

#[test]
fn c1_flag_off_clears_stale_threshold_accumulator() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (reg, alloc, world, _pid, n_dims) = loyalty_world(3);
    let n_slots = alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 1);
    let (_tx, rx) = feeder_channel();
    coord.shadow.fill(0.4);
    coord.upload_full_shadow(&state);

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.flags.use_accumulator_threshold_scan = true;
    proto.initial_gpu_sync(&coord, &mut state);
    assert!(state
        .accumulator_runtime
        .as_ref()
        .is_some_and(|r| r.threshold_active()));

    proto.flags.use_accumulator_threshold_scan = false;
    proto.initial_gpu_sync(&coord, &mut state);
    assert!(state
        .accumulator_runtime
        .as_ref()
        .is_none_or(|r| !r.threshold_active()));

    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        1.0,
    );
    assert!(state
        .accumulator_runtime
        .as_ref()
        .is_none_or(|r| !r.threshold_active()));
}

#[test]
fn disabling_one_accumulator_family_does_not_clear_others() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (reg, alloc, world, _pid, n_dims) = loyalty_world(2);
    let n_slots = alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.flags.use_accumulator_intent = true;
    proto.flags.use_accumulator_threshold_scan = true;
    proto.flags.use_accumulator_overlay_add = true;
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 1);
    coord.shadow.fill(0.5);
    coord.upload_full_shadow(&state);
    proto.initial_gpu_sync(&coord, &mut state);

    assert!(state.accumulator_runtime.as_ref().unwrap().intent_active());
    assert!(state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .threshold_active());

    proto.flags.use_accumulator_threshold_scan = false;
    proto.initial_gpu_sync(&coord, &mut state);
    let runtime = state.accumulator_runtime.as_ref().unwrap();
    assert!(!runtime.threshold_active());
    assert!(runtime.intent_active());

    proto.flags.use_accumulator_intent = false;
    proto.initial_gpu_sync(&coord, &mut state);
    let runtime = state.accumulator_runtime.as_ref().unwrap();
    assert!(!runtime.intent_active());
    assert!(!runtime.threshold_active());

    proto.flags.use_accumulator_overlay_add = false;
    proto.initial_gpu_sync(&coord, &mut state);
    let runtime = state.accumulator_runtime.as_ref().unwrap();
    assert!(!runtime.overlay_add_active());
    assert!(!state.accumulator_overlay_add_active);

    proto.flags.use_accumulator_intent = true;
    proto.initial_gpu_sync(&coord, &mut state);
    let runtime = state.accumulator_runtime.as_ref().unwrap();
    assert!(runtime.intent_active());
    assert!(!runtime.threshold_active());
    assert!(!runtime.overlay_add_active());
}
