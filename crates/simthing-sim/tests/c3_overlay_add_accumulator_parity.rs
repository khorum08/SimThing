//! C-3/C-4 bit-exact parity: AccumulatorOp OrderBand overlays vs CPU golden order.

use simthing_core::{
    DimensionRegistry, IntensityBehavior, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingKind,
    SubFieldRole, TransformOp,
};
use simthing_feeder::{
    feeder_channel, DispatchCoordinator, FeederWork, PatchTransform, TransformPatcher,
};
use simthing_gpu::{
    build_overlay_deltas, GpuContext, Pipelines, SlotAllocator, WorldGpuState, OP_ADD, OP_MULTIPLY,
    OP_SET,
};
use simthing_sim::BoundaryProtocol;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

struct Fixture {
    reg: DimensionRegistry,
    alloc: SlotAllocator,
    pid: simthing_core::SimPropertyId,
    world: SimThing,
    n_dims: u32,
}

fn loyalty_fixture() -> Fixture {
    let mut reg = DimensionRegistry::new();
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    let pid = reg.register(p);
    let layout = reg.property(pid).layout.clone();

    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    let mut pv = PropertyValue::from_layout(&layout);
    pv.data[layout.offset_of(&SubFieldRole::Amount).unwrap()] = 1.0;
    child.add_property(pid, pv);

    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_child(child);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);

    Fixture {
        n_dims: reg.total_columns as u32,
        reg,
        alloc,
        pid,
        world,
    }
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

#[derive(Debug, PartialEq)]
struct TickSnapshot {
    values: Vec<f32>,
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

fn golden_overlay_ticks<F>(n_ticks: u32, dt: f32, setup: F) -> TickSnapshot
where
    F: FnOnce(&mut SimThing, simthing_core::SimPropertyId),
{
    assert_eq!(
        dt.to_bits(),
        0.0f32.to_bits(),
        "CPU overlay golden covers overlay-only scenarios"
    );
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
    TickSnapshot { values }
}

fn run_overlay_ticks<F>(n_ticks: u32, dt: f32, setup: F) -> TickSnapshot
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

    let projected_len = n_slots as usize * fx.n_dims as usize;
    let mut projected = vec![0.0; projected_len];
    simthing_gpu::project_tree_to_values(
        &fx.world,
        &fx.reg,
        &fx.alloc,
        fx.n_dims as usize,
        &mut projected,
    );
    coord.shadow[..projected_len].copy_from_slice(&projected);

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
            dt,
        );
    }

    TickSnapshot {
        values: state.read_values(),
    }
}

macro_rules! parity_scenario {
    ($name:ident, $ticks:expr, $dt:expr, $setup:expr) => {
        #[test]
        fn $name() {
            let Some(_ctx) = try_gpu() else {
                eprintln!("skipping: no GPU");
                return;
            };
            let golden = golden_overlay_ticks($ticks, $dt, $setup);
            let gpu = run_overlay_ticks($ticks, $dt, $setup);
            assert_bits_eq(stringify!($name), &golden.values, &gpu.values);
        }
    };
}

parity_scenario!(c3_no_overlays, 2, 0.0, |_world, _pid| {});

parity_scenario!(c3_single_add_overlay, 1, 0.0, |world, pid| {
    let cohort = &mut world.children[0];
    cohort.add_overlay(make_overlay(
        pid,
        vec![(SubFieldRole::Amount, TransformOp::Add(0.25))],
    ));
});

parity_scenario!(c3_parent_add_child_add_ordering, 1, 0.0, |world, pid| {
    let child = world.children.remove(0);
    let mut parent = SimThing::new(SimThingKind::Location, 0);
    parent.add_overlay(make_overlay(
        pid,
        vec![(SubFieldRole::Amount, TransformOp::Add(0.1))],
    ));
    let mut child = child;
    child.add_overlay(make_overlay(
        pid,
        vec![(SubFieldRole::Amount, TransformOp::Add(0.05))],
    ));
    parent.add_child(child);
    *world = parent;
});

parity_scenario!(c3_multiple_columns, 1, 0.0, |world, pid| {
    let cohort = &mut world.children[0];
    cohort.add_overlay(make_overlay(
        pid,
        vec![
            (SubFieldRole::Amount, TransformOp::Add(0.1)),
            (SubFieldRole::Velocity, TransformOp::Add(0.02)),
        ],
    ));
});

parity_scenario!(c3_lifecycle_suspended_filtered, 1, 0.0, |world, pid| {
    let cohort = &mut world.children[0];
    let mut overlay = make_overlay(pid, vec![(SubFieldRole::Amount, TransformOp::Add(0.5))]);
    overlay.lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(OverlayLifecycle::Permanent),
    };
    cohort.add_overlay(overlay);
});

parity_scenario!(c3_mixed_add_multiply_split, 1, 0.0, |world, pid| {
    let cohort = &mut world.children[0];
    cohort.add_overlay(make_overlay(
        pid,
        vec![
            (SubFieldRole::Amount, TransformOp::Add(0.2)),
            (SubFieldRole::Amount, TransformOp::Multiply(1.5)),
        ],
    ));
});

parity_scenario!(c3_mixed_add_set_split, 1, 0.0, |world, pid| {
    let cohort = &mut world.children[0];
    cohort.add_overlay(make_overlay(
        pid,
        vec![
            (SubFieldRole::Amount, TransformOp::Add(0.15)),
            (SubFieldRole::Amount, TransformOp::Set(0.9)),
        ],
    ));
});

parity_scenario!(
    c3_mixed_add_multiply_set_orderband_parity,
    1,
    0.0,
    |world, pid| {
        let cohort = &mut world.children[0];
        cohort.add_overlay(make_overlay(
            pid,
            vec![
                (SubFieldRole::Amount, TransformOp::Add(0.1)),
                (SubFieldRole::Amount, TransformOp::Multiply(1.5)),
                (SubFieldRole::Amount, TransformOp::Set(0.9)),
            ],
        ));
    }
);

parity_scenario!(c3_add_only_old_pass3_noop, 1, 0.0, |world, pid| {
    let cohort = &mut world.children[0];
    cohort.add_overlay(make_overlay(
        pid,
        vec![(SubFieldRole::Amount, TransformOp::Add(0.33))],
    ));
});

#[test]
fn c3_repeated_add_same_cell_preserves_legacy_f32_order() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let setup = |world: &mut SimThing, pid: simthing_core::SimPropertyId| {
        let cohort = &mut world.children[0];
        cohort.add_overlay(make_overlay(
            pid,
            vec![
                (SubFieldRole::Amount, TransformOp::Add(1e20)),
                (SubFieldRole::Amount, TransformOp::Add(-1e20)),
            ],
        ));
    };

    let new = run_overlay_ticks(1, 0.0, setup);

    let expected = (1.0f32 + 1e20f32) + (-1e20f32);
    let amount_idx = 0usize; // loyalty Amount is first column in simple property
    assert_eq!(new.values[amount_idx].to_bits(), expected.to_bits());
    let golden = golden_overlay_ticks(1, 0.0, setup);
    assert_bits_eq("c3_repeated_add_same_cell", &golden.values, &new.values);
}

#[test]
fn c3_same_cell_many_overlays_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let n_overlays = 8usize;
    let setup = |world: &mut SimThing, pid: simthing_core::SimPropertyId| {
        let cohort = &mut world.children[0];
        for i in 0..n_overlays {
            cohort.add_overlay(make_overlay(
                pid,
                vec![(
                    SubFieldRole::Amount,
                    TransformOp::Add(0.01 * (i as f32 + 1.0)),
                )],
            ));
        }
    };
    let golden = golden_overlay_ticks(1, 0.0, setup);
    let new = run_overlay_ticks(1, 0.0, setup);
    assert_bits_eq("c3_same_cell_many_overlays", &golden.values, &new.values);
}

#[test]
fn c3_mixed_overlay_routes_through_c4_orderband_path() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut fx = loyalty_fixture();
    let cohort = &mut fx.world.children[0];
    cohort.add_overlay(make_overlay(
        fx.pid,
        vec![
            (SubFieldRole::Amount, TransformOp::Add(0.2)),
            (SubFieldRole::Amount, TransformOp::Multiply(1.5)),
        ],
    ));
    fx.alloc = SlotAllocator::new();
    fx.alloc.populate_from_tree(&fx.world);

    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);

    let projected_len = n_slots as usize * fx.n_dims as usize;
    let mut projected = vec![0.0; projected_len];
    simthing_gpu::project_tree_to_values(
        &fx.world,
        &fx.reg,
        &fx.alloc,
        fx.n_dims as usize,
        &mut projected,
    );
    coord.shadow[..projected_len].copy_from_slice(&projected);

    let mut proto = BoundaryProtocol::new(fx.world, fx.reg, fx.alloc);
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);

    assert!(
        state.accumulator_overlay_add_active,
        "mixed overlay batch should use the C-4 OrderBand Accumulator path"
    );
}

// ── Combined C-1 + C-2 + C-3 ──────────────────────────────────────────────────

use simthing_core::{Direction, FissionTemplate, FissionThreshold, SimThingKindTag};
use simthing_gpu::ThresholdEvent;

fn sort_events(events: &[ThresholdEvent]) -> Vec<ThresholdEvent> {
    let mut out = events.to_vec();
    out.sort_by_key(|e| (e.slot, e.col, e.event_kind));
    out
}

#[test]
fn c1_c2_c3_combined_accumulator_paths_parity() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let run = |use_intent: bool,
               use_threshold: bool,
               use_overlay: bool|
     -> (TickSnapshot, Vec<ThresholdEvent>) {
        let mut reg = DimensionRegistry::new();
        let mut pressure = SimProperty::simple("stress", "pressure", 0);
        pressure.intensity_behavior = Some(IntensityBehavior::default());
        pressure.fission_templates = vec![FissionThreshold {
            sub_field: SubFieldRole::Amount,
            threshold: 0.4,
            direction: Direction::Falling,
            template: FissionTemplate {
                child_kind: SimThingKindTag::Cohort,
                fusion_intensity_threshold: 0.9,
                fusion_scar_coefficient: 0.02,
                resolution_label: "stress_resolved".into(),
                clone_capability_children: false,
                capability_container_kinds: Vec::new(),
            },
            secondary: None,
        }];
        let pid = reg.register(pressure);
        let layout = reg.property(pid).layout.clone();
        let amount = layout.offset_of(&SubFieldRole::Amount).unwrap();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[amount] = 0.45;
        cohort.add_property(pid, pv);
        cohort.add_overlay(make_overlay(
            pid,
            vec![(SubFieldRole::Amount, TransformOp::Add(-0.05))],
        ));

        let mut world = SimThing::new(SimThingKind::World, 0);
        world.add_child(cohort);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);
        let n_slots = alloc.capacity() as u32;
        let n_dims = reg.total_columns as u32;

        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut state = WorldGpuState::new(ctx, &reg, n_slots);
        let pipelines = Pipelines::new(&state.ctx);
        let mut patcher = TransformPatcher::new(n_slots as usize);
        let mut coord = DispatchCoordinator::new(n_slots, n_dims, 4);
        let (tx, rx) = feeder_channel();

        let projected_len = n_slots as usize * n_dims as usize;
        let mut projected = vec![0.0; projected_len];
        simthing_gpu::project_tree_to_values(&world, &reg, &alloc, n_dims as usize, &mut projected);
        coord.shadow[..projected_len].copy_from_slice(&projected);

        let mut proto = BoundaryProtocol::new(world, reg, alloc);
        proto.flags.use_accumulator_intent = use_intent;
        proto.flags.use_accumulator_threshold_scan = use_threshold;
        proto.flags.use_accumulator_overlay_add = use_overlay;
        proto.initial_gpu_sync(&coord, &mut state);

        let cohort_id = proto.root.children[0].id;
        tx.send(FeederWork::Patch(PatchTransform {
            target: cohort_id,
            delta: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(-0.02))],
            },
        }))
        .unwrap();

        let mut last_events = Vec::new();
        for _ in 0..2 {
            let out = coord.tick(
                &rx,
                &mut patcher,
                &proto.registry,
                &proto.allocator,
                &pipelines,
                &mut state,
                1.0,
            );
            last_events = sort_events(&out.events);
        }

        (
            TickSnapshot {
                values: state.read_values(),
            },
            last_events,
        )
    };

    let (old_vals, old_events) = run(false, false, true);
    let (new_vals, new_events) = run(true, true, true);
    assert_bits_eq("combined values", &old_vals.values, &new_vals.values);
    assert_eq!(old_events, new_events);
}
