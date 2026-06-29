//! C-1 threshold scan coverage after S-6: AccumulatorOp vs golden/replay stability.

use simthing_core::{
    DimensionRegistry, Direction, FissionTemplate, FissionThreshold, IntensityBehavior,
    PropertyValue, SimProperty, SimThing, SimThingKind, SimThingKindTag, SubFieldRole,
};
use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
use simthing_sim::{BoundaryProtocol, SimRuntimeTree};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn fission_stress_world(n_slots: u32) -> (SimThing, DimensionRegistry, SlotAllocator) {
    let mut reg = DimensionRegistry::new();
    let mut pressure = SimProperty::simple("stress", "pressure", 0);
    pressure.intensity_behavior = Some(IntensityBehavior::default());
    pressure.fission_templates = vec![FissionThreshold {
        sub_field: SubFieldRole::Amount,
        threshold: 0.3,
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
    let velocity = layout.offset_of(&SubFieldRole::Velocity).unwrap();
    let intensity = layout.offset_of(&SubFieldRole::Intensity).unwrap();

    let mut world = SimThing::new(SimThingKind::World, 0);
    for i in 0..n_slots.saturating_sub(1) {
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[amount] = 0.31 + ((i % 5) as f32) * 0.001;
        pv.data[velocity] = -0.02;
        pv.data[intensity] = 0.1;
        cohort.add_property(pid, pv);
        world.add_child(cohort);
    }

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    (world, reg, alloc)
}

fn sort_events(events: &[ThresholdEvent]) -> Vec<ThresholdEvent> {
    let mut out = events.to_vec();
    out.sort_by_key(|e| (e.slot, e.col, e.event_kind));
    out
}

fn run_ticks(n_slots: u32, n_ticks: u32) -> Vec<Vec<ThresholdEvent>> {
    let (world, reg, alloc) = fission_stress_world(n_slots);
    let n_dims = reg.total_columns as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 1);
    let (_tx, rx) = feeder_channel();

    let projected_len = alloc.capacity() * n_dims as usize;
    let mut projected = vec![0.0; projected_len];
    simthing_gpu::project_tree_to_values(&world, &reg, &alloc, n_dims as usize, &mut projected);
    coord.shadow[..projected_len].copy_from_slice(&projected);

    let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
    proto.flags.use_accumulator_threshold_scan = true;
    proto.initial_gpu_sync(&coord, &mut state);

    let mut per_tick = Vec::with_capacity(n_ticks as usize);
    for _ in 0..n_ticks {
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
}

#[test]
fn fission_stress_100_ticks_accumulator_replay_stable_events() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    const N_SLOTS: u32 = 20_000;
    const N_TICKS: u32 = 100;

    let old_path = run_ticks(N_SLOTS, N_TICKS);
    let new_path = run_ticks(N_SLOTS, N_TICKS);

    assert_eq!(old_path.len(), new_path.len());
    for (tick, (old, new)) in old_path.iter().zip(new_path.iter()).enumerate() {
        assert_eq!(old.len(), new.len(), "tick {tick} event count");
        for (a, b) in old.iter().zip(new.iter()) {
            assert_eq!(a.slot, b.slot, "tick {tick} slot");
            assert_eq!(a.col, b.col, "tick {tick} col");
            assert_eq!(a.event_kind, b.event_kind, "tick {tick} event_kind");
            assert_eq!(a.value.to_bits(), b.value.to_bits(), "tick {tick} value");
        }
    }
}

#[test]
fn c1_threshold_accumulator_readback_succeeds_in_tick_outcome() {
    use simthing_gpu::{
        ThresholdRegistration, WorldAccumulatorRuntime, DIR_UPWARD, THRESH_BUF_VALUES,
    };

    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (world, reg, alloc) = fission_stress_world(3);
    let n_slots = alloc.capacity() as u32;
    let n_dims = reg.total_columns as u32;
    let amount_col = {
        let pid = world.children[0].properties.keys().next().copied().unwrap();
        reg.column_range(pid)
            .col_for_role(&SubFieldRole::Amount, &reg.property(pid).layout)
            .unwrap() as u32
    };
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 1);
    let (_tx, rx) = feeder_channel();

    let projected_len = n_slots as usize * n_dims as usize;
    let mut projected = vec![0.0; projected_len];
    simthing_gpu::project_tree_to_values(&world, &reg, &alloc, n_dims as usize, &mut projected);
    coord.shadow[..projected_len].copy_from_slice(&projected);

    let regs = vec![
        ThresholdRegistration {
            slot: 1,
            col: amount_col,
            threshold: 0.5,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        },
        ThresholdRegistration {
            slot: 2,
            col: amount_col,
            threshold: 0.5,
            direction: DIR_UPWARD,
            event_kind: 2,
            buffer: THRESH_BUF_VALUES,
        },
    ];
    state.upload_thresholds(&regs);

    let mut flat = projected.clone();
    let mut prev = projected.clone();
    let layout = reg
        .property(world.children[0].properties.keys().next().copied().unwrap())
        .layout
        .clone();
    let velocity_col = layout.offset_of(&SubFieldRole::Velocity).unwrap() as u32;
    for slot in [1u32, 2] {
        let base = slot as usize * n_dims as usize;
        flat[base + amount_col as usize] = 0.4;
        flat[base + velocity_col as usize] = 0.2;
        prev[base + amount_col as usize] = 0.4;
    }
    state.install_resolved_values_at_boundary(&flat);
    state.install_resolved_previous_values_at_boundary(&prev);

    let mut runtime = WorldAccumulatorRuntime::new();
    runtime.ensure_threshold_session(&state.ctx, n_slots, n_dims, 1);
    runtime.upload_threshold_ops(&state.ctx, &regs).unwrap();
    state.accumulator_runtime = Some(runtime);

    let out = coord.tick(&rx, &mut patcher, &reg, &alloc, &pipelines, &mut state, 1.0);

    assert!(
        out.gpu_error.is_none(),
        "unexpected GPU error: {:?}",
        out.gpu_error
    );
}
