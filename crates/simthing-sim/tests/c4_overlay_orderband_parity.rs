//! C-4 overlay OrderBand parity and dirty-cache coverage.

use simthing_core::{
    DimensionRegistry, Direction, DissolveCondition, FissionTemplate, FissionThreshold,
    IntensityBehavior, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingKind, SimThingKindTag,
    SubFieldRole, TransformOp,
};
use simthing_feeder::{
    feeder_channel, BoundaryRequest, DispatchCoordinator, FeederWork, TransformPatcher,
};
use simthing_gpu::{
    build_overlay_deltas, set_debug_readback_allowed, summaries_from_values, GpuContext, Pipelines,
    SlotAllocator, ThresholdEvent, WorldGpuState, OP_ADD, OP_MULTIPLY, OP_SET,
};
use simthing_sim::{BoundaryProtocol, VelocityAlertRegistration};

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

struct Fixture {
    reg: DimensionRegistry,
    world: SimThing,
    alloc: SlotAllocator,
    pid: simthing_core::SimPropertyId,
    n_dims: u32,
}

fn loyalty_fixture() -> Fixture {
    let mut reg = DimensionRegistry::new();
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    let pid = reg.register(p);
    let layout = reg.property(pid).layout.clone();

    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    let mut pv = PropertyValue::from_layout(&layout);
    pv.data[layout.offset_of(&SubFieldRole::Amount).unwrap()] = 1.0;
    pv.data[layout.offset_of(&SubFieldRole::Velocity).unwrap()] = 0.0;
    cohort.add_property(pid, pv);

    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_child(cohort);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let n_dims = reg.total_columns as u32;

    Fixture {
        reg,
        world,
        alloc,
        pid,
        n_dims,
    }
}

fn assert_bits_eq(label: &str, old: &[f32], new: &[f32]) {
    assert_eq!(old.len(), new.len(), "{label}: length");
    for (i, (a, b)) in old.iter().zip(new.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "{label} mismatch at index {i}: {a} vs {b}"
        );
    }
}

fn sort_events(events: &[ThresholdEvent]) -> Vec<ThresholdEvent> {
    let mut out = events.to_vec();
    out.sort_by_key(|e| (e.slot, e.col, e.event_kind));
    out
}

fn overlay_cache_stats(state: &WorldGpuState) -> (u64, u64, u32) {
    let cache = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .overlay_compile_cache
        .as_ref()
        .unwrap();
    (
        cache.compile_count,
        cache.upload_count,
        cache.cached_op_buffer_uploaded_n_ops,
    )
}

fn project_to_coord(fx: &Fixture, coord: &mut DispatchCoordinator) {
    let projected_len = fx.alloc.capacity() * fx.n_dims as usize;
    let mut projected = vec![0.0; projected_len];
    simthing_gpu::project_tree_to_values(
        &fx.world,
        &fx.reg,
        &fx.alloc,
        fx.n_dims as usize,
        &mut projected,
    );
    coord.shadow[..projected_len].copy_from_slice(&projected);
}

fn apply_overlay_golden(values: &mut [f32], fx: &Fixture) {
    let (deltas, ranges) = build_overlay_deltas(&fx.world, &fx.reg, &fx.alloc);
    for slot in 0..fx.alloc.capacity() {
        if slot >= ranges.len() {
            break;
        }
        let range = ranges[slot];
        for i in range.offset as usize..(range.offset + range.length) as usize {
            let delta = deltas[i];
            let idx = slot * fx.n_dims as usize + delta.col as usize;
            match delta.op_kind {
                OP_ADD => values[idx] += delta.value,
                OP_MULTIPLY => values[idx] *= delta.value,
                OP_SET => values[idx] = delta.value,
                other => panic!("unsupported overlay op kind {other}"),
            }
        }
    }
}

fn golden_overlay_ticks<F>(n_ticks: u32, setup: F) -> Vec<f32>
where
    F: FnOnce(&mut SimThing, simthing_core::SimPropertyId),
{
    let mut fx = loyalty_fixture();
    setup(&mut fx.world, fx.pid);
    fx.alloc = SlotAllocator::new();
    fx.alloc.populate_from_tree(&fx.world);
    let projected_len = fx.alloc.capacity() * fx.n_dims as usize;
    let mut values = vec![0.0; projected_len];
    simthing_gpu::project_tree_to_values(
        &fx.world,
        &fx.reg,
        &fx.alloc,
        fx.n_dims as usize,
        &mut values,
    );
    for _ in 0..n_ticks {
        apply_overlay_golden(&mut values, &fx);
    }
    values
}

fn run_overlay_ticks<F>(n_ticks: u32, setup: F) -> Vec<f32>
where
    F: FnOnce(&mut SimThing, simthing_core::SimPropertyId),
{
    let mut fx = loyalty_fixture();
    setup(&mut fx.world, fx.pid);
    fx.alloc = SlotAllocator::new();
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let (_tx, rx) = feeder_channel();
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);

    for _ in 0..n_ticks {
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

    state.read_values()
}

macro_rules! parity {
    ($name:ident, $setup:expr) => {
        #[test]
        fn $name() {
            let Some(_ctx) = try_gpu() else {
                eprintln!("skipping: no GPU");
                return;
            };
            let golden = golden_overlay_ticks(1, $setup);
            let accumulator = run_overlay_ticks(1, $setup);
            assert_bits_eq(stringify!($name), &golden, &accumulator);
        }
    };
}

parity!(add_only_matches_legacy, |world: &mut SimThing, pid| {
    world.children[0].add_overlay(make_overlay(
        pid,
        vec![(SubFieldRole::Amount, TransformOp::Add(0.25))],
    ));
});

parity!(mul_only_matches_legacy, |world: &mut SimThing, pid| {
    world.children[0].add_overlay(make_overlay(
        pid,
        vec![(SubFieldRole::Amount, TransformOp::Multiply(1.5))],
    ));
});

parity!(set_only_matches_legacy, |world: &mut SimThing, pid| {
    world.children[0].add_overlay(make_overlay(
        pid,
        vec![(SubFieldRole::Amount, TransformOp::Set(0.2))],
    ));
});

parity!(
    mixed_add_mul_set_matches_legacy,
    |world: &mut SimThing, pid| {
        world.children[0].add_overlay(make_overlay(
            pid,
            vec![
                (SubFieldRole::Amount, TransformOp::Add(10.0)),
                (SubFieldRole::Amount, TransformOp::Multiply(2.0)),
                (SubFieldRole::Amount, TransformOp::Set(5.0)),
                (SubFieldRole::Amount, TransformOp::Add(1.0)),
            ],
        ));
    }
);

parity!(
    ancestor_local_mixed_matches_legacy,
    |world: &mut SimThing, pid| {
        let child = world.children.remove(0);
        let mut parent = SimThing::new(SimThingKind::Location, 0);
        parent.add_overlay(make_overlay(
            pid,
            vec![(SubFieldRole::Amount, TransformOp::Multiply(0.5))],
        ));
        let mut child = child;
        child.add_overlay(make_overlay(
            pid,
            vec![(SubFieldRole::Amount, TransformOp::Add(0.25))],
        ));
        parent.add_child(child);
        *world = parent;
    }
);

#[test]
fn c4_suspended_overlay_absent_then_activated() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut fx = loyalty_fixture();
    let target = fx.world.children[0].id;
    let mut overlay = make_overlay(fx.pid, vec![(SubFieldRole::Amount, TransformOp::Add(1.0))]);
    let overlay_id = overlay.id;
    overlay.lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(OverlayLifecycle::Permanent),
    };
    fx.world.children[0].add_overlay(overlay);
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let (_tx, rx) = feeder_channel();
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);
    assert_eq!(overlay_cache_stats(&state), (1, 1, 0));
    let revision = proto.overlay_compile_revision();

    patcher.apply_collected_as_intents(
        vec![FeederWork::Boundary(BoundaryRequest::ActivateOverlay {
            target,
            overlay_id,
        })],
        Vec::new(),
        &proto.registry,
        &proto.allocator,
    );
    let _ = proto.execute(Vec::new(), &mut patcher, &mut coord, &mut state, 1);
    assert!(proto.overlay_compile_revision() > revision);

    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.0,
    );
    let accumulator = state.read_values();
    let golden = golden_overlay_ticks(1, |world: &mut SimThing, pid| {
        world.children[0].add_overlay(make_overlay(
            pid,
            vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
        ));
    });
    assert_bits_eq(
        "c4_suspended_overlay_absent_then_activated",
        &golden,
        &accumulator,
    );
}

#[test]
fn c4_overlay_dissolve_bumps_revision_and_removes_ops() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut fx = loyalty_fixture();
    let mut overlay = make_overlay(fx.pid, vec![(SubFieldRole::Amount, TransformOp::Add(1.0))]);
    overlay.lifecycle = OverlayLifecycle::Transient {
        dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 0 }],
    };
    fx.world.children[0].add_overlay(overlay);
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let (_tx, rx) = feeder_channel();
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);
    assert_eq!(overlay_cache_stats(&state), (1, 1, 1));
    let revision = proto.overlay_compile_revision();

    let out = proto.execute(Vec::new(), &mut patcher, &mut coord, &mut state, 1);
    assert_eq!(out.lifecycle.dissolved, 1);
    assert!(proto.overlay_compile_revision() > revision);
    assert_eq!(overlay_cache_stats(&state), (2, 2, 0));

    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.0,
    );
    let accumulator = state.read_values();
    let golden = golden_overlay_ticks(1, |_world: &mut SimThing, _pid| {});
    assert_bits_eq(
        "c4_overlay_dissolve_bumps_revision_and_removes_ops",
        &golden,
        &accumulator,
    );
}

#[test]
fn c4_fission_clone_inherits_overlays_correctly() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    fn run() -> (Vec<f32>, u64, u64) {
        let mut reg = DimensionRegistry::new();
        let mut prop = SimProperty::simple("core", "loyalty", 0);
        prop.intensity_behavior = Some(IntensityBehavior::default());
        prop.fission_templates = vec![FissionThreshold {
            sub_field: SubFieldRole::Amount,
            threshold: 0.3,
            direction: Direction::Falling,
            template: FissionTemplate {
                child_kind: SimThingKindTag::Cohort,
                fusion_intensity_threshold: 0.9,
                fusion_scar_coefficient: 0.02,
                resolution_label: "resolved".into(),
                clone_capability_children: false,
                capability_container_kinds: Vec::new(),
            },
            secondary: None,
        }];
        let pid = reg.register(prop);
        let layout = reg.property(pid).layout.clone();
        let amount = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let velocity = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let intensity = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[amount] = 0.31;
        pv.data[velocity] = -0.02;
        pv.data[intensity] = 0.1;
        cohort.add_property(pid, pv);
        cohort.add_overlay(make_overlay(
            pid,
            vec![(SubFieldRole::Intensity, TransformOp::Add(0.25))],
        ));

        let mut world = SimThing::new(SimThingKind::World, 0);
        world.add_child(cohort);
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let n_slots = 8;
        let n_dims = reg.total_columns as u32;
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut state = WorldGpuState::new(ctx, &reg, n_slots);
        let pipelines = Pipelines::new(&state.ctx);
        let mut patcher = TransformPatcher::new(n_slots as usize);
        let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
        let (_tx, rx) = feeder_channel();
        let mut projected = vec![0.0; alloc.capacity() * n_dims as usize];
        simthing_gpu::project_tree_to_values(&world, &reg, &alloc, n_dims as usize, &mut projected);
        coord.shadow[..projected.len()].copy_from_slice(&projected);

        let mut proto = BoundaryProtocol::new(world, reg, alloc);
        proto.flags.use_accumulator_overlay_add = true;
        proto.initial_gpu_sync(&coord, &mut state);
        let revision = proto.overlay_compile_revision();

        let tick = coord.tick(
            &rx,
            &mut patcher,
            &proto.registry,
            &proto.allocator,
            &pipelines,
            &mut state,
            1.0,
        );
        assert!(!tick.events.is_empty(), "fission threshold never fired");
        let out = proto.execute(tick.events, &mut patcher, &mut coord, &mut state, 1);
        assert_eq!(out.fission.fissions_executed, 1);
        let revision_after = proto.overlay_compile_revision();

        let _ = coord.tick(
            &rx,
            &mut patcher,
            &proto.registry,
            &proto.allocator,
            &pipelines,
            &mut state,
            0.0,
        );
        (state.read_values(), revision, revision_after)
    }

    let (accumulator, before, after) = run();
    assert!(after > before);
    assert!(
        accumulator.iter().any(|value| *value != 0.0),
        "expected populated accumulator output after fission"
    );
}

#[test]
fn c4_no_change_tick_does_not_recompile() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut fx = loyalty_fixture();
    fx.world.children[0].add_overlay(make_overlay(
        fx.pid,
        vec![(SubFieldRole::Amount, TransformOp::Multiply(1.1))],
    ));
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);
    proto.initial_gpu_sync(&coord, &mut state);

    let cache = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .overlay_compile_cache
        .as_ref()
        .unwrap();
    assert_eq!(cache.compile_count, 1);
    assert_eq!(cache.upload_count, 1);
}

#[test]
fn c4_equality_check_skips_upload_when_deltas_unchanged() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut fx = loyalty_fixture();
    fx.world.children[0].add_overlay(make_overlay(
        fx.pid,
        vec![(SubFieldRole::Amount, TransformOp::Add(0.1))],
    ));
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);
    proto.bump_overlay_compile_revision_for_test();
    proto.initial_gpu_sync(&coord, &mut state);

    let cache = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .overlay_compile_cache
        .as_ref()
        .unwrap();
    assert_eq!(cache.compile_count, 1);
    assert_eq!(cache.upload_count, 1);
    assert_eq!(cache.compiled_at_revision, proto.overlay_compile_revision());
}

#[test]
fn c4_high_density_unchanged_no_recompile_no_upload() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let layout = reg.property(pid).layout.clone();
    let mut world = SimThing::new(SimThingKind::World, 0);
    for i in 0..1000 {
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, PropertyValue::from_layout(&layout));
        for j in 0..8 {
            cohort.add_overlay(make_overlay(
                pid,
                vec![(
                    SubFieldRole::Amount,
                    TransformOp::Add((i + j) as f32 * 0.001),
                )],
            ));
        }
        world.add_child(cohort);
    }
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let n_slots = alloc.capacity() as u32;
    let n_dims = reg.total_columns as u32;

    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
    let mut projected = vec![0.0; n_slots as usize * n_dims as usize];
    simthing_gpu::project_tree_to_values(&world, &reg, &alloc, n_dims as usize, &mut projected);
    coord.shadow.copy_from_slice(&projected);

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.flags.use_accumulator_overlay_add = true;
    for _ in 0..50 {
        proto.initial_gpu_sync(&coord, &mut state);
    }

    let cache = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .overlay_compile_cache
        .as_ref()
        .unwrap();
    assert_eq!(cache.compile_count, 1);
    assert_eq!(cache.upload_count, 1);
}

#[test]
fn c4_high_density_single_attach_recompiles_once() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let layout = reg.property(pid).layout.clone();
    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut attach_target = None;
    for i in 0..1000 {
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        if attach_target.is_none() {
            attach_target = Some(cohort.id);
        }
        cohort.add_property(pid, PropertyValue::from_layout(&layout));
        for j in 0..8 {
            cohort.add_overlay(make_overlay(
                pid,
                vec![(
                    SubFieldRole::Amount,
                    TransformOp::Add((i + j) as f32 * 0.001),
                )],
            ));
        }
        world.add_child(cohort);
    }
    let target = attach_target.unwrap();
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let n_slots = alloc.capacity() as u32;
    let n_dims = reg.total_columns as u32;

    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut projected = vec![0.0; n_slots as usize * n_dims as usize];
    simthing_gpu::project_tree_to_values(&world, &reg, &alloc, n_dims as usize, &mut projected);
    coord.shadow.copy_from_slice(&projected);

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.flags.use_accumulator_overlay_add = true;
    for _ in 0..25 {
        proto.initial_gpu_sync(&coord, &mut state);
    }
    assert_eq!(overlay_cache_stats(&state), (1, 1, 8000));

    patcher.apply_collected_as_intents(
        vec![FeederWork::Boundary(BoundaryRequest::AttachOverlay {
            target,
            overlay: make_overlay(pid, vec![(SubFieldRole::Amount, TransformOp::Add(1.0))]),
        })],
        Vec::new(),
        &proto.registry,
        &proto.allocator,
    );
    let _ = proto.execute(Vec::new(), &mut patcher, &mut coord, &mut state, 25);
    assert_eq!(overlay_cache_stats(&state), (2, 2, 8001));

    for _ in 26..50 {
        proto.initial_gpu_sync(&coord, &mut state);
    }
    assert_eq!(overlay_cache_stats(&state), (2, 2, 8001));
}

#[test]
fn c4_after_ticks_decrement_alone_does_not_recompile() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut fx = loyalty_fixture();
    let mut overlay = make_overlay(fx.pid, vec![(SubFieldRole::Amount, TransformOp::Add(0.2))]);
    overlay.lifecycle = OverlayLifecycle::Transient {
        dissolution_conditions: vec![simthing_core::DissolveCondition::AfterTicks { remaining: 5 }],
    };
    fx.world.children[0].add_overlay(overlay);
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);
    let revision = proto.overlay_compile_revision();
    let out = proto.execute(Vec::new(), &mut patcher, &mut coord, &mut state, 1);
    assert_eq!(out.lifecycle.dissolved, 0);
    assert_eq!(proto.overlay_compile_revision(), revision);
    let cache = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .overlay_compile_cache
        .as_ref()
        .unwrap();
    assert_eq!(cache.compile_count, 1);
    assert_eq!(cache.upload_count, 1);
}

#[test]
fn c4_overlay_attach_bumps_revision() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fx = loyalty_fixture();
    let target = fx.world.children[0].id;
    let pid = fx.pid;
    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);
    let revision = proto.overlay_compile_revision();

    patcher.apply_collected_as_intents(
        vec![FeederWork::Boundary(BoundaryRequest::AttachOverlay {
            target,
            overlay: make_overlay(pid, vec![(SubFieldRole::Amount, TransformOp::Add(0.1))]),
        })],
        Vec::new(),
        &proto.registry,
        &proto.allocator,
    );
    let _ = proto.execute(Vec::new(), &mut patcher, &mut coord, &mut state, 1);
    assert!(proto.overlay_compile_revision() > revision);
}

#[test]
fn c4_combined_threshold_intent_overlay_path_replay_stable() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    fn run() -> (Vec<f32>, Vec<ThresholdEvent>) {
        let mut fx = loyalty_fixture();
        let target = fx.world.children[0].id;
        let pid = fx.pid;
        fx.world.children[0].add_overlay(make_overlay(
            pid,
            vec![
                (SubFieldRole::Amount, TransformOp::Add(1.0)),
                (SubFieldRole::Amount, TransformOp::Multiply(2.0)),
                (SubFieldRole::Amount, TransformOp::Set(5.0)),
                (SubFieldRole::Amount, TransformOp::Add(1.0)),
            ],
        ));
        fx.alloc.populate_from_tree(&fx.world);

        let n_slots = fx.alloc.capacity() as u32;
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
        let pipelines = Pipelines::new(&state.ctx);
        let mut patcher = TransformPatcher::new(n_slots as usize);
        let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
        let (tx, rx) = feeder_channel();
        project_to_coord(&fx, &mut coord);

        let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
        proto.flags.use_accumulator_threshold_scan = true;
        proto.flags.use_accumulator_intent = true;
        proto.flags.use_accumulator_overlay_add = true;
        proto.register_velocity_alert(VelocityAlertRegistration {
            sim_thing_id: target,
            property_id: pid,
            sub_field: SubFieldRole::Amount,
            threshold: 4.0,
            direction: Direction::Rising,
        });
        proto.initial_gpu_sync(&coord, &mut state);

        tx.submit_patch(
            target,
            PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
            },
        )
        .unwrap();
        let tick = coord.tick(
            &rx,
            &mut patcher,
            &proto.registry,
            &proto.allocator,
            &pipelines,
            &mut state,
            0.0,
        );
        (state.read_values(), sort_events(&tick.events))
    }

    let (first_values, first_events) = run();
    let (second_values, second_events) = run();
    assert_bits_eq(
        "c4_combined_threshold_intent_overlay_path_replay_stable_values",
        &first_values,
        &second_values,
    );
    assert_eq!(first_events.len(), 1);
    assert_eq!(first_events.len(), second_events.len());
    for (first, second) in first_events.iter().zip(second_events.iter()) {
        assert_eq!(first.slot, second.slot);
        assert_eq!(first.col, second.col);
        assert_eq!(first.event_kind, second.event_kind);
        assert_eq!(first.value.to_bits(), second.value.to_bits());
    }
}

#[test]
fn c4_world_summary_matches_full_values_after_mixed_overlay() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let mut fx = loyalty_fixture();
    fx.world.children[0].add_overlay(make_overlay(
        fx.pid,
        vec![
            (SubFieldRole::Amount, TransformOp::Add(10.0)),
            (SubFieldRole::Amount, TransformOp::Multiply(2.0)),
            (SubFieldRole::Amount, TransformOp::Set(5.0)),
            (SubFieldRole::Amount, TransformOp::Add(1.0)),
        ],
    ));
    fx.alloc.populate_from_tree(&fx.world);
    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let (_tx, rx) = feeder_channel();
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
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
    let cpu_summary = summaries_from_values(&values, n_slots, fx.n_dims);
    assert_eq!(gpu_summary, cpu_summary);
}
