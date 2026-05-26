//! B-4 world summary after integrated C-2/C-3 Accumulator tick paths.

use simthing_core::{
    DimensionRegistry, IntensityBehavior, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingKind,
    SubFieldRole, TransformOp,
};
use simthing_feeder::{
    feeder_channel, DispatchCoordinator, FeederWork, PatchTransform, TransformPatcher,
};
use simthing_gpu::{
    set_debug_readback_allowed, summaries_from_values, GpuContext, Pipelines, SlotAllocator,
    WorldGpuState,
};
use simthing_sim::BoundaryProtocol;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn make_overlay(
    pid: simthing_core::SimPropertyId,
    ops: Vec<(SubFieldRole, TransformOp)>,
) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: ops,
        },
        lifecycle: OverlayLifecycle::Permanent,
    }
}

#[test]
fn b4_world_summary_matches_full_values_after_integrated_intent() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let mut reg = DimensionRegistry::new();
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    let pid = reg.register(p);
    let mut alloc = SlotAllocator::new();
    let id = SimThing::new(SimThingKind::Cohort, 0).id;
    alloc.alloc(id);
    let n_dims = reg.total_columns as u32;
    let n_slots = alloc.capacity() as u32;

    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
    let (tx, rx) = feeder_channel();
    coord.shadow.fill(0.5);
    coord.upload_full_shadow(&state);

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
    world.add_child(cohort);

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.flags.use_accumulator_intent = true;
    proto.initial_gpu_sync(&coord, &mut state);

    tx.send(FeederWork::Patch(PatchTransform {
        target: id,
        delta: PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.1))],
        },
    }))
    .unwrap();

    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        1.0,
    );

    let values = state.read_values();
    let gpu_summary = state.readback_accumulator_summary().unwrap();
    let cpu_summary = summaries_from_values(&values, n_slots, n_dims);
    assert_eq!(gpu_summary, cpu_summary);
}

#[test]
fn b4_world_summary_matches_after_overlay_add_orderbands() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let mut reg = DimensionRegistry::new();
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    let pid = reg.register(p);
    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
    cohort.add_overlay(make_overlay(
        pid,
        vec![
            (SubFieldRole::Amount, TransformOp::Add(1e20)),
            (SubFieldRole::Amount, TransformOp::Add(-1e20)),
        ],
    ));
    world.add_child(cohort);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let n_dims = reg.total_columns as u32;
    let n_slots = alloc.capacity() as u32;

    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
    let (_tx, rx) = feeder_channel();

    let projected_len = n_slots as usize * n_dims as usize;
    let mut projected = vec![0.0; projected_len];
    simthing_gpu::project_tree_to_values(&world, &reg, &alloc, n_dims as usize, &mut projected);
    projected[0] = 1.0;
    coord.shadow[..projected_len].copy_from_slice(&projected);
    coord.upload_full_shadow(&state);

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);

    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.0,
    );

    let values = state.read_values();
    let gpu_summary = state.readback_accumulator_summary().unwrap();
    let cpu_summary = summaries_from_values(&values, n_slots, n_dims);
    assert_eq!(gpu_summary, cpu_summary);
}
