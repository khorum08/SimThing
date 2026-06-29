//! S-2 legacy intensity sunset validation.

use simthing_core::{
    DimensionRegistry, IntensityBehavior, PropertyValue, SimProperty, SimThing, SimThingKind,
    SubFieldRole,
};
use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{
    build_intensity_eml_entries, project_tree_to_values, set_debug_readback_allowed, GpuContext,
    Pipelines, SlotAllocator, WorldGpuState,
};
use simthing_sim::{BoundaryProtocol, SimRuntimeTree};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn intensity_property() -> SimProperty {
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    p
}

#[test]
fn s2_no_legacy_intensity_shader_file() {
    let shader_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-gpu/src/shaders/intensity_update.wgsl");
    assert!(
        !shader_path.exists(),
        "legacy intensity_update.wgsl must be deleted in S-2"
    );
}

#[test]
fn s2_accumulator_intensity_is_default_path() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let mut reg = DimensionRegistry::new();
    let pid = reg.register(intensity_property());
    let layout = reg.property(pid).layout.clone();
    let icol = reg
        .column_range(pid)
        .col_for_role(&SubFieldRole::Intensity, &layout)
        .unwrap();

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    let mut pv = PropertyValue::from_layout(&layout);
    pv.data[layout.offset_of(&SubFieldRole::Velocity).unwrap()] = 0.05;
    pv.data[icol] = 0.3;
    cohort.add_property(pid, pv);
    world.add_child(cohort);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let n_dims = reg.total_columns as u32;
    let n_slots = alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
    let mut patcher = TransformPatcher::new(n_slots as usize);

    let projected_len = alloc.capacity() * n_dims as usize;
    let mut projected = vec![0.0; projected_len];
    project_tree_to_values(&world, &reg, &alloc, n_dims as usize, &mut projected);
    coord.shadow[..projected_len].copy_from_slice(&projected);
    coord.upload_full_shadow(&state);

    let proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
    assert!(proto.flags.use_accumulator_intensity);
    assert!(proto.flags.use_accumulator_eml);

    let mut proto = proto;
    proto.initial_gpu_sync(&coord, &mut state);
    assert!(state.accumulator_intensity_eml_active);

    let cohort_slot = proto
        .allocator
        .slot_of(proto.root.direct_child_id(0).expect("world has cohort"))
        .unwrap();
    let idx = cohort_slot as usize * n_dims as usize + icol;
    let before = state.read_values()[idx];

    let (tx, rx) = feeder_channel();
    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        1.0,
    );
    drop(tx);

    let after = state.read_values()[idx];
    assert_ne!(after.to_bits(), before.to_bits());
}

#[test]
fn s2_intensity_disabled_rejects_world_with_intensity_behavior() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property());
    assert!(!build_intensity_eml_entries(&reg).is_empty());

    let mut proto = BoundaryProtocol::new(
        SimRuntimeTree::admit(SimThing::new(SimThingKind::World, 0)),
        reg,
        SlotAllocator::new(),
    );
    proto.flags.use_accumulator_intensity = false;
    proto.flags.use_accumulator_eml = false;

    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &proto.registry, 1);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        proto
            .flags
            .validate_intensity_enabled_for_registry(&proto.registry);
    }));
    assert!(
        result.is_err(),
        "expected panic when intensity disabled with IntensityBehavior"
    );
    let _ = state;
}

#[test]
fn s2_no_cpu_mediated_production_intensity() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property());
    let mut state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), &reg, 1);
    state.install_resolved_values_at_boundary(&[0.0, 0.1, 0.5]);
    state.sync_intensity_eml_accumulator(&reg);
    assert!(state.accumulator_intensity_eml_active);

    let pipelines = Pipelines::new(&state.ctx);
    pipelines.run_accumulator_intensity_eml(&mut state, 0.25);
    assert_ne!(state.read_values()[2].to_bits(), 0.5f32.to_bits());
}
