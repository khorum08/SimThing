//! C-2 AccumulatorOp affine intent coverage after S-1 legacy deletion.

use simthing_core::{
    DimensionRegistry, IntensityBehavior, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingId,
    SimThingKind, SimThingKindTag, SubFieldRole, TransformOp,
};
use simthing_feeder::{
    feeder_channel, DispatchCoordinator, FeederSender, FeederWork, PatchTransform, TransformPatcher,
};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, WorldGpuState};
use simthing_sim::{BoundaryProtocol, SimRuntimeTree};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

struct Fixture {
    reg: DimensionRegistry,
    alloc: SlotAllocator,
    pid: simthing_core::SimPropertyId,
    ids: Vec<SimThingId>,
    n_dims: u32,
}

fn loyalty_fixture(n_cohorts: usize) -> Fixture {
    let mut reg = DimensionRegistry::new();
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    let pid = reg.register(p);
    let mut alloc = SlotAllocator::new();
    let mut ids = Vec::with_capacity(n_cohorts);
    for _ in 0..n_cohorts {
        let id = SimThing::new(SimThingKind::Cohort, 0).id;
        alloc.alloc(id);
        ids.push(id);
    }
    Fixture {
        n_dims: reg.total_columns as u32,
        reg,
        alloc,
        pid,
        ids,
    }
}

#[derive(Debug, PartialEq)]
struct TickSnapshot {
    values: Vec<f32>,
    previous: Vec<f32>,
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

fn run_ticks<F>(_use_accumulator_intent: bool, n_ticks: u32, dt: f32, setup: F) -> TickSnapshot
where
    F: FnOnce(
        &mut DispatchCoordinator,
        &FeederSender,
        &DimensionRegistry,
        &SlotAllocator,
        simthing_core::SimPropertyId,
        &[SimThingId],
    ),
{
    let fx = loyalty_fixture(4);
    let n_slots = fx.alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &fx.reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, fx.n_dims, 8);
    let (tx, rx) = feeder_channel();

    coord.shadow.fill(0.0);
    coord.shadow[0] = 0.5;
    coord.shadow[fx.n_dims as usize] = 0.25;
    coord.upload_full_shadow(&state);

    let mut world = SimThing::new(SimThingKind::World, 0);
    for _id in &fx.ids {
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(
            fx.pid,
            PropertyValue::from_layout(&fx.reg.property(fx.pid).layout),
        );
        world.add_child(cohort);
    }
    let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), fx.reg, fx.alloc);
    proto.flags.use_accumulator_intent = true;
    proto.initial_gpu_sync(&coord, &mut state);

    setup(
        &mut coord,
        &tx,
        &proto.registry,
        &proto.allocator,
        fx.pid,
        &fx.ids,
    );

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
        previous: state.read_previous_values(),
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
            let old = run_ticks(true, $ticks, $dt, $setup);
            let new = run_ticks(true, $ticks, $dt, $setup);
            assert_bits_eq(stringify!($name), &old.values, &new.values);
            assert_bits_eq(
                &format!("{} previous", stringify!($name)),
                &old.previous,
                &new.previous,
            );
        }
    };
}

parity_scenario!(
    c2_no_intents,
    3,
    0.0,
    |_coord, _tx, _reg, _alloc, _pid, _ids| {}
);

parity_scenario!(c2_single_add, 1, 0.0, |_coord, tx, reg, alloc, pid, ids| {
    let mk = |op: TransformOp| PropertyTransformDelta {
        property_id: pid,
        sub_field_deltas: vec![(SubFieldRole::Amount, op)],
    };
    tx.send(FeederWork::Patch(PatchTransform {
        target: ids[0],
        delta: mk(TransformOp::Add(0.25)),
    }))
    .unwrap();
    let _ = (reg, alloc);
});

parity_scenario!(
    c2_single_multiply,
    1,
    0.0,
    |_coord, tx, _reg, _alloc, pid, ids| {
        tx.send(FeederWork::Patch(PatchTransform {
            target: ids[0],
            delta: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(0.5))],
            },
        }))
        .unwrap();
    }
);

parity_scenario!(
    c2_multiply_then_add_folded,
    1,
    0.0,
    |_coord, tx, _reg, _alloc, pid, ids| {
        let mk = |op: TransformOp| PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, op)],
        };
        tx.send(FeederWork::Patch(PatchTransform {
            target: ids[0],
            delta: mk(TransformOp::Add(0.25)),
        }))
        .unwrap();
        tx.send(FeederWork::Patch(PatchTransform {
            target: ids[0],
            delta: mk(TransformOp::Multiply(0.5)),
        }))
        .unwrap();
    }
);

parity_scenario!(
    c2_set_like_fold,
    1,
    0.0,
    |_coord, tx, _reg, _alloc, pid, ids| {
        tx.send(FeederWork::Patch(PatchTransform {
            target: ids[0],
            delta: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.75))],
            },
        }))
        .unwrap();
    }
);

parity_scenario!(
    c2_many_slots_unique_columns,
    1,
    0.0,
    |_coord, tx, reg, alloc, pid, ids| {
        let layout = reg.property(pid).layout.clone();
        let amount = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let velocity = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        for (slot_idx, id) in ids.iter().enumerate().take(3) {
            let col = if slot_idx == 0 { amount } else { velocity };
            let role = if slot_idx == 0 {
                SubFieldRole::Amount
            } else {
                SubFieldRole::Velocity
            };
            tx.send(FeederWork::Patch(PatchTransform {
                target: *id,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![(role, TransformOp::Add(0.01 * slot_idx as f32))],
                },
            }))
            .unwrap();
            let _ = col;
        }
        let _ = alloc;
    }
);

parity_scenario!(
    c2_same_cell_fold_sequence,
    1,
    0.0,
    |_coord, tx, _reg, _alloc, pid, ids| {
        let mk = |op: TransformOp| PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, op)],
        };
        for i in 1..=10 {
            tx.send(FeederWork::Patch(PatchTransform {
                target: ids[0],
                delta: mk(TransformOp::Set(i as f32 * 0.01)),
            }))
            .unwrap();
        }
    }
);

parity_scenario!(
    c2_player_intent_patch,
    1,
    0.0,
    |_coord, tx, _reg, _alloc, pid, ids| {
        let overlay = Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            affects: vec![ids[0]],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(-0.05))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        };
        tx.submit_player_intent(ids[0], overlay).unwrap();
    }
);

parity_scenario!(
    c2_negative_add_decay,
    1,
    0.0,
    |_coord, tx, _reg, _alloc, pid, ids| {
        tx.send(FeederWork::Patch(PatchTransform {
            target: ids[0],
            delta: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(-0.4))],
            },
        }))
        .unwrap();
    }
);

parity_scenario!(
    c2_sparse_many_intents,
    1,
    0.0,
    |_coord, tx, _reg, alloc, pid, ids| {
        let n = alloc.capacity().min(64);
        let mk = |_target, v| PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(v))],
        };
        for i in 0..n {
            if i % 7 == 0 {
                let target = ids[i % ids.len()];
                tx.send(FeederWork::Patch(PatchTransform {
                    target,
                    delta: mk(target, 0.001 * i as f32),
                }))
                .unwrap();
            }
        }
    }
);

// ── Combined C-1 + C-2 ordering ─────────────────────────────────────────────

use simthing_core::{Direction, FissionTemplate, FissionThreshold};
use simthing_gpu::ThresholdEvent;

fn sort_events(events: &[ThresholdEvent]) -> Vec<ThresholdEvent> {
    let mut out = events.to_vec();
    out.sort_by_key(|e| (e.slot, e.col, e.event_kind));
    out
}

#[test]
fn c1_c2_combined_accumulator_intent_then_threshold_parity() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let run = || -> Vec<Vec<ThresholdEvent>> {
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

        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let cohort_id = cohort.id;
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[amount] = 0.45;
        cohort.add_property(pid, pv);
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

        let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
        proto.flags.use_accumulator_intent = true;
        proto.flags.use_accumulator_threshold_scan = true;
        proto.initial_gpu_sync(&coord, &mut state);

        let mut per_tick = Vec::new();
        for tick in 0..3 {
            if tick == 1 {
                tx.send(FeederWork::Patch(PatchTransform {
                    target: cohort_id,
                    delta: PropertyTransformDelta {
                        property_id: pid,
                        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(-0.1))],
                    },
                }))
                .unwrap();
            }
            let out = coord.tick(
                &rx,
                &mut patcher,
                &proto.registry,
                &proto.allocator,
                &pipelines,
                &mut state,
                1.0,
            );
            per_tick.push(sort_events(&out.events));
        }
        per_tick
    };

    let old_both = run();
    let new_both = run();

    assert_eq!(old_both.len(), new_both.len());
    for (tick, (old, new)) in old_both.iter().zip(new_both.iter()).enumerate() {
        assert_eq!(old.len(), new.len(), "tick {tick} event count");
        for (a, b) in old.iter().zip(new.iter()) {
            assert_eq!(a.slot(), b.slot());
            assert_eq!(a.col(), b.col());
            assert_eq!(a.event_kind(), b.event_kind());
            assert_eq!(a.value().to_bits(), b.value().to_bits());
        }
    }
}
