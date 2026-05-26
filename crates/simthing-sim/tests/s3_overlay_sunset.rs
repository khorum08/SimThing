//! S-3 overlay sunset: legacy Pass 3 shader/runtime is gone.

use simthing_core::{
    DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_feeder::{
    feeder_channel, BoundaryRequest, DispatchCoordinator, FeederWork, TransformPatcher,
};
use simthing_gpu::{
    build_overlay_deltas, GpuContext, Pipelines, SlotAllocator, WorldGpuState, OP_ADD, OP_MULTIPLY,
    OP_SET,
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

struct Fixture {
    reg: DimensionRegistry,
    world: SimThing,
    alloc: SlotAllocator,
    pid: simthing_core::SimPropertyId,
    n_dims: u32,
}

fn loyalty_fixture() -> Fixture {
    let mut reg = DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
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

    Fixture {
        n_dims: reg.total_columns as u32,
        reg,
        world,
        alloc,
        pid,
    }
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

fn assert_bits_eq(label: &str, expected: &[f32], actual: &[f32]) {
    assert_eq!(expected.len(), actual.len(), "{label}: length");
    for (i, (a, b)) in expected.iter().zip(actual.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "{label} mismatch at index {i}: {a} vs {b}"
        );
    }
}

#[test]
fn s3_no_legacy_overlay_shader_file() {
    assert!(
        !std::path::Path::new("crates/simthing-gpu/src/shaders/transform_application.wgsl")
            .exists()
    );
}

#[test]
fn s3_accumulator_overlay_is_default_path() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut fx = loyalty_fixture();
    fx.world.children[0].add_overlay(make_overlay(
        fx.pid,
        vec![(SubFieldRole::Amount, TransformOp::Add(0.25))],
    ));
    fx.alloc = SlotAllocator::new();
    fx.alloc.populate_from_tree(&fx.world);
    let target_slot = fx.alloc.slot_of(fx.world.children[0].id).unwrap() as usize;

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let (_tx, rx) = feeder_channel();
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.initial_gpu_sync(&coord, &mut state);
    assert!(state.accumulator_overlay_add_active);

    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.0,
    );
    assert_eq!(
        state.read_values()[target_slot * fx.n_dims as usize].to_bits(),
        1.25f32.to_bits()
    );
}

#[test]
fn s3_overlay_disabled_rejects_overlay_workload() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut fx = loyalty_fixture();
    fx.world.children[0].add_overlay(make_overlay(
        fx.pid,
        vec![(SubFieldRole::Amount, TransformOp::Add(0.25))],
    ));
    fx.alloc = SlotAllocator::new();
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = false;
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        proto.initial_gpu_sync(&coord, &mut state);
    }))
    .expect_err("disabled overlay flag should reject active overlay workload");
    let message = panic
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| panic.downcast_ref::<&'static str>().copied())
        .unwrap_or("");
    assert!(message.contains(
        "Legacy overlay path was deleted in S-3; AccumulatorOp overlay must remain enabled."
    ));
}

#[test]
fn s3_overlay_accumulator_matches_cpu_golden_add_multiply_set() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let layout = reg.property(pid).layout.clone();
    let mut world = SimThing::new(SimThingKind::Cohort, 0);
    let mut pv = PropertyValue::from_layout(&layout);
    pv.data[layout.offset_of(&SubFieldRole::Amount).unwrap()] = 1.0;
    pv.data[layout.offset_of(&SubFieldRole::Velocity).unwrap()] = 0.0;
    world.add_property(pid, pv);
    world.add_overlay(make_overlay(
        pid,
        vec![
            (SubFieldRole::Amount, TransformOp::Add(10.0)),
            (SubFieldRole::Amount, TransformOp::Multiply(2.0)),
            (SubFieldRole::Amount, TransformOp::Set(5.0)),
            (SubFieldRole::Amount, TransformOp::Add(1.0)),
        ],
    ));
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let fx = Fixture {
        n_dims: reg.total_columns as u32,
        reg,
        world,
        alloc,
        pid,
    };

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let (_tx, rx) = feeder_channel();
    project_to_coord(&fx, &mut coord);

    let mut expected = coord.shadow.clone();
    apply_overlay_golden(&mut expected, &fx);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
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
    assert_bits_eq("s3 mixed overlay golden", &expected, &state.read_values());
}

#[test]
fn s3_overlay_cache_rebuilds_after_fission_or_lifecycle_change() {
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
    fx.alloc = SlotAllocator::new();
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    project_to_coord(&fx, &mut coord);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.initial_gpu_sync(&coord, &mut state);
    let revision = proto.overlay_compile_revision();
    let cache = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .overlay_compile_cache
        .as_ref()
        .unwrap();
    assert_eq!(cache.cached_op_buffer_uploaded_n_ops, 0);

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
    let cache = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .overlay_compile_cache
        .as_ref()
        .unwrap();
    assert_eq!(cache.cached_op_buffer_uploaded_n_ops, 1);
    assert_eq!(cache.compile_count, 2);
}
