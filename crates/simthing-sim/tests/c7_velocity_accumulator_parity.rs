//! C-7 velocity integration AccumulatorOp parity vs legacy Pass 1.

use simthing_core::{ClampBehavior, DimensionRegistry, SimProperty, SubFieldRole};
use simthing_gpu::{
    build_governed_pairs, plan_velocity_integration, GpuContext, Pipelines, WorldGpuState,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn assert_bits_eq(label: &str, legacy: &[f32], acc: &[f32]) {
    assert_eq!(legacy.len(), acc.len(), "{label}: length mismatch");
    for (i, (a, b)) in legacy.iter().zip(acc.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "{label}: index {i} diverges — legacy={a} ({:08x}), acc={b} ({:08x})",
            a.to_bits(),
            b.to_bits(),
        );
    }
}

fn governed_amount_velocity_property(vel_max: Option<f32>, clamp: ClampBehavior) -> SimProperty {
    let mut p = SimProperty::simple("core", "governed", 0);
    for sf in &mut p.layout.sub_fields {
        if matches!(sf.role, SubFieldRole::Amount) {
            sf.velocity_max = vel_max;
            sf.clamp = clamp.clone();
        }
    }
    p
}

fn setup_velocity_state(reg: &DimensionRegistry, n_slots: u32, initial: &[f32]) -> WorldGpuState {
    let mut state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), reg, n_slots);
    let n_dims = state.n_dims as usize;
    let mut flat = vec![0.0_f32; state.values_len()];
    for (slot, row) in initial.chunks(n_dims).enumerate() {
        flat[slot * n_dims..slot * n_dims + n_dims].copy_from_slice(row);
    }
    state.write_values(&flat);

    state.ensure_velocity_accumulator();
    let pairs = build_governed_pairs(reg);
    let plan = plan_velocity_integration(&pairs, n_slots);
    state
        .upload_velocity_ops_with_bands(&plan.ops, plan.n_bands)
        .expect("velocity upload");
    state
}

fn run_legacy_velocity(state: &WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    pipelines.run_snapshot(state);
    pipelines.run_velocity_integration(state, dt);
    state.read_values()
}

fn run_accumulator_velocity(state: &mut WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    let mut runtime = state.accumulator_runtime.take().unwrap();
    let mut velocity_session = runtime.take_velocity_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        simthing_gpu::AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: velocity_session.as_mut(),
            intensity_eml: None,
            transfer: None,
            emission: None,
            encode_world_summary: false,
        },
    );
    runtime.restore_velocity_session(velocity_session);
    state.accumulator_runtime = Some(runtime);
    state.read_values()
}

#[test]
fn c7_velocity_basic_legacy_vs_accumulator_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(governed_amount_velocity_property(
        Some(10.0),
        ClampBehavior::Bounded {
            min: 0.0,
            max: 100.0,
        },
    ));
    let n_dims = reg.total_columns;
    let mut row = vec![0.0_f32; n_dims];
    row[0] = 10.0; // amount
    row[1] = 2.0; // velocity

    let legacy_state = setup_velocity_state(&reg, 1, &row);
    let mut acc_state = setup_velocity_state(&reg, 1, &row);
    let dt = 0.5;

    let legacy = run_legacy_velocity(&legacy_state, dt);
    let acc = run_accumulator_velocity(&mut acc_state, dt);
    assert_bits_eq("basic", &legacy, &acc);
    assert_eq!(legacy[0].to_bits(), (11.0_f32).to_bits());
    assert_eq!(legacy[1].to_bits(), (2.0_f32).to_bits());
}

#[test]
fn c7_velocity_vel_max_boundary_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let vel_max = 0.5_f32;
    let mut reg = DimensionRegistry::new();
    reg.register(governed_amount_velocity_property(
        Some(vel_max),
        ClampBehavior::Bounded {
            min: 0.0,
            max: 100.0,
        },
    ));
    let n_dims = reg.total_columns;

    let cases: &[(f32, &str)] = &[
        (vel_max, "exactly +vel_max"),
        (vel_max + 0.01, "above +vel_max"),
        (-vel_max, "exactly -vel_max"),
        (-vel_max - 0.01, "below -vel_max"),
    ];

    for (velocity, label) in cases {
        let mut row = vec![0.0_f32; n_dims];
        row[0] = 0.4;
        row[1] = *velocity;

        let legacy_state = setup_velocity_state(&reg, 1, &row);
        let mut acc_state = setup_velocity_state(&reg, 1, &row);
        let legacy = run_legacy_velocity(&legacy_state, 1.0);
        let acc = run_accumulator_velocity(&mut acc_state, 1.0);
        assert_bits_eq(label, &legacy, &acc);
    }
}

#[test]
fn c7_velocity_amount_clamp_min_max_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(governed_amount_velocity_property(
        Some(10.0),
        ClampBehavior::Bounded { min: 0.0, max: 1.0 },
    ));
    let n_dims = reg.total_columns;

    let cases: &[(f32, f32, f32, &str)] = &[
        (0.9, 0.5, 1.0, "crossing max"),
        (0.1, -0.5, 0.0, "crossing min"),
        (1.0, 0.0, 1.0, "exactly max"),
        (0.0, 0.0, 0.0, "exactly min"),
    ];

    for (amount, velocity, dt, label) in cases {
        let mut row = vec![0.0_f32; n_dims];
        row[0] = *amount;
        row[1] = *velocity;

        let legacy_state = setup_velocity_state(&reg, 1, &row);
        let mut acc_state = setup_velocity_state(&reg, 1, &row);
        let legacy = run_legacy_velocity(&legacy_state, *dt);
        let acc = run_accumulator_velocity(&mut acc_state, *dt);
        assert_bits_eq(label, &legacy, &acc);
    }
}

#[test]
fn c7_velocity_writes_amount_and_velocity_targets() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(governed_amount_velocity_property(
        Some(1.0),
        ClampBehavior::Bounded { min: 0.0, max: 1.0 },
    ));
    let n_dims = reg.total_columns;
    let mut row = vec![0.0_f32; n_dims];
    row[0] = 0.0;
    row[1] = -0.2;

    let legacy_state = setup_velocity_state(&reg, 1, &row);
    let mut acc_state = setup_velocity_state(&reg, 1, &row);
    let legacy = run_legacy_velocity(&legacy_state, 1.0);
    let acc = run_accumulator_velocity(&mut acc_state, 1.0);
    assert_bits_eq("floor pin amount+velocity", &legacy, &acc);
    assert_eq!(legacy[0].to_bits(), 0.0_f32.to_bits());
    assert!(legacy[1] >= 0.0);
}

#[test]
fn c7_velocity_many_slots_many_pairs_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(governed_amount_velocity_property(
        Some(0.5),
        ClampBehavior::Bounded { min: 0.0, max: 1.0 },
    ));
    let n_dims = reg.total_columns;
    let n_slots = 4u32;
    let mut flat = vec![0.0_f32; n_dims * n_slots as usize];
    for slot in 0..n_slots as usize {
        let base = slot * n_dims;
        flat[base] = 0.2 + slot as f32 * 0.1;
        flat[base + 1] = -0.1 + slot as f32 * 0.05;
    }

    let legacy_state = setup_velocity_state(&reg, n_slots, &flat);
    let mut acc_state = setup_velocity_state(&reg, n_slots, &flat);
    let legacy = run_legacy_velocity(&legacy_state, 0.25);
    let acc = run_accumulator_velocity(&mut acc_state, 0.25);
    assert_bits_eq("multi-slot", &legacy, &acc);
}

#[test]
fn c7_velocity_dt_zero_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(governed_amount_velocity_property(
        Some(1.0),
        ClampBehavior::Bounded { min: 0.0, max: 1.0 },
    ));
    let n_dims = reg.total_columns;
    let mut row = vec![0.0_f32; n_dims];
    row[0] = 0.42;
    row[1] = -0.33;

    let legacy_state = setup_velocity_state(&reg, 1, &row);
    let mut acc_state = setup_velocity_state(&reg, 1, &row);
    let legacy = run_legacy_velocity(&legacy_state, 0.0);
    let acc = run_accumulator_velocity(&mut acc_state, 0.0);
    assert_bits_eq("dt=0", &legacy, &acc);
}

#[test]
fn c7_velocity_path_no_cpu_mediated_integration() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(governed_amount_velocity_property(
        Some(1.0),
        ClampBehavior::Bounded { min: 0.0, max: 1.0 },
    ));
    let n_dims = reg.total_columns;
    let mut row = vec![0.0_f32; n_dims];
    row[0] = 0.5;
    row[1] = 0.1;

    let mut state = setup_velocity_state(&reg, 1, &row);
    assert!(state.accumulator_velocity_active);
    let _ = run_accumulator_velocity(&mut state, 0.25);
}

#[test]
fn c7_combined_c1_c2_c4_reduction_velocity_all_flags_on() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    use simthing_core::{PropertyValue, SimThing, SimThingKind};
    use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
    use simthing_gpu::SlotAllocator;
    use simthing_sim::BoundaryProtocol;

    let mut reg = DimensionRegistry::new();
    let mut prop = SimProperty::simple("core", "loyalty", 0);
    prop.intensity_behavior = Some(simthing_core::IntensityBehavior::default());
    let pid = reg.register(prop);
    let layout = reg.property(pid).layout.clone();
    let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut loc = SimThing::new(SimThingKind::Location, 0);
    for i in 0..2 {
        let mut c = SimThing::new(SimThingKind::Cohort, i);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[a_off] = 0.4 + i as f32 * 0.05;
        pv.data[v_off] = 0.03 - i as f32 * 0.01;
        c.add_property(pid, pv);
        loc.add_child(c);
    }
    world.add_child(loc);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let n_dims = reg.total_columns as u32;
    let n_slots = alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    coord.shadow.fill(0.5);
    coord.upload_full_shadow(&state);

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.flags.use_accumulator_threshold_scan = true;
    proto.flags.use_accumulator_intent = true;
    proto.flags.use_accumulator_overlay_add = true;
    proto.flags.use_accumulator_reduction_soft = true;
    proto.flags.use_accumulator_reduction_exact = true;
    proto.flags.use_accumulator_velocity = true;
    proto.initial_gpu_sync(&coord, &mut state);

    let (tx, rx) = feeder_channel();
    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.5,
    );
    drop(tx);

    assert!(state.accumulator_velocity_active);
    let values = state.read_values();
    assert!(values.iter().all(|v| v.is_finite()));
    let gpu_summary = state.readback_accumulator_summary().unwrap();
    let cpu_summary = simthing_gpu::summaries_from_values(&values, n_slots, n_dims);
    assert_eq!(gpu_summary, cpu_summary);
}
