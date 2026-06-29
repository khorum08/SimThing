//! C-8 completion gate: full GPU-resident EML/intensity/transfer/emission block.

use simthing_core::{
    ClampBehavior, DimensionRegistry, IntensityBehavior, PropertyValue, SimProperty, SimPropertyId,
    SimThing, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{
    project_tree_to_values, set_debug_readback_allowed, EmissionFormula, EmissionRegistration,
    GpuContext, Pipelines, SlotAllocator, TransferInputRef, TransferRegistration, WorldGpuState,
};
use simthing_sim::{BoundaryProtocol, SimRuntimeTree};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn c8_full_registry() -> (
    DimensionRegistry,
    SimPropertyId,
    SimPropertyId,
    u32,
    u32,
    u32,
) {
    let mut reg = DimensionRegistry::new();
    let mut loyalty = SimProperty::simple("core", "loyalty", 0);
    loyalty.intensity_behavior = Some(IntensityBehavior::default());
    let loyalty_pid = reg.register(loyalty);

    let resource_cols = vec![
        SubFieldSpec {
            role: SubFieldRole::Named("stock".into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "stock".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        },
        SubFieldSpec {
            role: SubFieldRole::Named("pool".into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "pool".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        },
        SubFieldSpec {
            role: SubFieldRole::Named("sink".into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "sink".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        },
    ];
    let resources_pid = reg.register(SimProperty {
        namespace: "econ".into(),
        name: "resources".into(),
        layout: simthing_core::PropertyLayout {
            sub_fields: resource_cols,
        },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    });

    let loyalty_layout = reg.property(loyalty_pid).layout.clone();
    let amount_col = reg
        .column_range(loyalty_pid)
        .col_for_role(&SubFieldRole::Amount, &loyalty_layout)
        .unwrap();
    let velocity_col = reg
        .column_range(loyalty_pid)
        .col_for_role(&SubFieldRole::Velocity, &loyalty_layout)
        .unwrap();
    let intensity_col = reg
        .column_range(loyalty_pid)
        .col_for_role(&SubFieldRole::Intensity, &loyalty_layout)
        .unwrap();

    (
        reg,
        loyalty_pid,
        resources_pid,
        amount_col as u32,
        velocity_col as u32,
        intensity_col as u32,
    )
}

fn c8_full_world(
    reg: &DimensionRegistry,
    loyalty_pid: SimPropertyId,
    resources_pid: SimPropertyId,
) -> (SimThing, SlotAllocator) {
    let loyalty_layout = reg.property(loyalty_pid).layout.clone();
    let resources_layout = reg.property(resources_pid).layout.clone();
    let amount = loyalty_layout.offset_of(&SubFieldRole::Amount).unwrap();
    let velocity = loyalty_layout.offset_of(&SubFieldRole::Velocity).unwrap();
    let intensity = loyalty_layout.offset_of(&SubFieldRole::Intensity).unwrap();
    let stock = resources_layout
        .offset_of(&SubFieldRole::Named("stock".into()))
        .unwrap();
    let pool = resources_layout
        .offset_of(&SubFieldRole::Named("pool".into()))
        .unwrap();
    let sink = resources_layout
        .offset_of(&SubFieldRole::Named("sink".into()))
        .unwrap();

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut loc = SimThing::new(SimThingKind::Location, 0);
    for i in 0..2 {
        let mut cohort = SimThing::new(SimThingKind::Cohort, i);
        let mut loyalty_pv = PropertyValue::from_layout(&loyalty_layout);
        loyalty_pv.data[amount] = 0.45 + i as f32 * 0.05;
        loyalty_pv.data[velocity] = 0.025 - i as f32 * 0.005;
        loyalty_pv.data[intensity] = 0.35;
        cohort.add_property(loyalty_pid, loyalty_pv);

        let mut resources_pv = PropertyValue::from_layout(&resources_layout);
        resources_pv.data[stock] = 10.0;
        resources_pv.data[pool] = 2.0 + i as f32;
        resources_pv.data[sink] = 0.0;
        cohort.add_property(resources_pid, resources_pv);
        loc.add_child(cohort);
    }
    world.add_child(loc);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    (world, alloc)
}

fn all_c8_flags(proto: &mut BoundaryProtocol) {
    proto.flags.use_accumulator_threshold_scan = true;
    proto.flags.use_accumulator_intent = true;
    proto.flags.use_accumulator_overlay_add = true;
    proto.flags.use_accumulator_reduction_soft = true;
    proto.flags.use_accumulator_reduction_exact = true;
    proto.flags.use_accumulator_velocity = true;
    proto.flags.use_accumulator_eml = true;
    proto.flags.use_accumulator_intensity = true;
    proto.flags.use_accumulator_transfer = true;
    proto.flags.use_accumulator_emission = true;
}

fn transfer_regs(cohort_slot: u32, stock_col: u32, pool_col: u32) -> Vec<TransferRegistration> {
    vec![TransferRegistration {
        inputs: vec![TransferInputRef {
            slot: cohort_slot,
            col: stock_col,
            unit_cost: 1.0,
        }],
        target_slot: cohort_slot,
        target_col: pool_col,
        output_scale: 1.0,
        max_transfer: Some(1.0),
        tree_id: None,
        order_band: 0,
    }]
}

fn emission_regs(cohort_slot: u32, pool_col: u32) -> Vec<EmissionRegistration> {
    vec![EmissionRegistration {
        source_slot: cohort_slot,
        source_col: pool_col,
        tree_id: None,
        formula: EmissionFormula::IdentityFloor,
        max_emit: None,
        reg_idx: 99,
    }]
}

fn sync_c8_substrates(
    state: &mut WorldGpuState,
    reg: &DimensionRegistry,
    cohort_slot: u32,
    stock_col: u32,
    pool_col: u32,
) {
    state.sync_intensity_eml_accumulator(reg);
    state
        .sync_transfer_accumulator(&transfer_regs(cohort_slot, stock_col, pool_col))
        .expect("transfer sync");
    state
        .sync_emission_accumulator(&emission_regs(cohort_slot, pool_col))
        .expect("emission sync");
}

fn col_global(reg: &DimensionRegistry, pid: SimPropertyId, role: SubFieldRole) -> u32 {
    let layout = reg.property(pid).layout.clone();
    reg.column_range(pid).col_for_role(&role, &layout).unwrap() as u32
}

fn flat_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn first_cohort_slot(world: &SimThing, alloc: &SlotAllocator) -> u32 {
    let cohort_id = world.children[0].children[0].id;
    alloc.slot_of(cohort_id).expect("cohort slot")
}

struct UploadSnapshot {
    eml_upload_count: u64,
    input_list_upload_count: u64,
    intensity_op_upload_count: u64,
    transfer_op_upload_count: u64,
    emission_op_upload_count: u64,
}

fn snapshot_upload_counts(state: &WorldGpuState) -> UploadSnapshot {
    let runtime = state.accumulator_runtime.as_ref().unwrap();
    UploadSnapshot {
        eml_upload_count: runtime.eml.as_ref().map(|t| t.upload_count()).unwrap_or(0),
        input_list_upload_count: runtime
            .input_lists
            .as_ref()
            .map(|t| t.upload_count)
            .unwrap_or(0),
        intensity_op_upload_count: runtime.intensity_op_upload_count(),
        transfer_op_upload_count: runtime.transfer_op_upload_count(),
        emission_op_upload_count: runtime.emission_op_upload_count(),
    }
}

fn legacy_intensity_shader_deleted() {
    let shader_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-gpu/src/shaders/intensity_update.wgsl");
    assert!(!shader_path.exists());
}

#[test]
fn c8_full_gpu_resident_pipeline_all_flags_on() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    legacy_intensity_shader_deleted();

    let (reg, loyalty_pid, resources_pid, _, _, intensity_col) = c8_full_registry();
    let stock_col = col_global(&reg, resources_pid, SubFieldRole::Named("stock".into()));
    let pool_col = col_global(&reg, resources_pid, SubFieldRole::Named("pool".into()));

    let (world, alloc) = c8_full_world(&reg, loyalty_pid, resources_pid);
    let cohort_slot = first_cohort_slot(&world, &alloc);
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

    let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
    all_c8_flags(&mut proto);
    proto.initial_gpu_sync(&coord, &mut state);
    sync_c8_substrates(
        &mut state,
        &proto.registry,
        cohort_slot,
        stock_col,
        pool_col,
    );

    let intensity_idx = flat_index(cohort_slot, intensity_col, n_dims);
    let stock_idx = flat_index(cohort_slot, stock_col, n_dims);
    let pool_idx = flat_index(cohort_slot, pool_col, n_dims);
    let intensity_before = state.read_values()[intensity_idx].to_bits();
    let stock_before = state.read_values()[stock_idx].to_bits();
    let pool_before = state.read_values()[pool_idx].to_bits();

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

    assert!(state.accumulator_intensity_eml_active);
    assert!(state.accumulator_transfer_active);
    assert!(state.accumulator_emission_active);
    assert!(state.accumulator_velocity_active);

    let values = state.read_values();
    assert!(values.iter().all(|v| v.is_finite()));

    let intensity_after = values[intensity_idx];
    assert_ne!(intensity_after.to_bits(), intensity_before);

    let stock_after = values[stock_idx];
    let pool_after = values[pool_idx];
    assert_eq!(
        stock_after.to_bits(),
        (f32::from_bits(stock_before) - 1.0).to_bits()
    );
    assert_eq!(
        pool_after.to_bits(),
        (f32::from_bits(pool_before) + 1.0).to_bits()
    );

    let emissions = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .emission_session()
        .unwrap()
        .readback_emissions(&state.ctx)
        .expect("emission readback");
    assert_eq!(emissions.len(), 1);
    assert_eq!(emissions[0].reg_idx, 99);
    assert_eq!(emissions[0].emit_count, pool_after.floor() as u32);

    let gpu_summary = state.readback_accumulator_summary().unwrap();
    let cpu_summary = simthing_gpu::summaries_from_values(&values, n_slots, n_dims);
    assert_eq!(gpu_summary, cpu_summary);

    let runtime = state.accumulator_runtime.as_ref().unwrap();
    assert!(runtime.threshold_active());
}

#[test]
fn c8_full_pipeline_reuses_persistent_tables_and_ops_across_ticks() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let (reg, loyalty_pid, resources_pid, _, _, _) = c8_full_registry();
    let stock_col = col_global(&reg, resources_pid, SubFieldRole::Named("stock".into()));
    let pool_col = col_global(&reg, resources_pid, SubFieldRole::Named("pool".into()));

    let (world, alloc) = c8_full_world(&reg, loyalty_pid, resources_pid);
    let cohort_slot = first_cohort_slot(&world, &alloc);
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

    let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
    all_c8_flags(&mut proto);
    proto.initial_gpu_sync(&coord, &mut state);
    sync_c8_substrates(
        &mut state,
        &proto.registry,
        cohort_slot,
        stock_col,
        pool_col,
    );
    let baseline = snapshot_upload_counts(&state);

    let (tx, rx) = feeder_channel();
    for dt in [0.5, 1.0, 0.25] {
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
    drop(tx);

    sync_c8_substrates(
        &mut state,
        &proto.registry,
        cohort_slot,
        stock_col,
        pool_col,
    );
    let after_resync = snapshot_upload_counts(&state);
    assert_eq!(after_resync.eml_upload_count, baseline.eml_upload_count);
    assert_eq!(
        after_resync.input_list_upload_count,
        baseline.input_list_upload_count
    );
    assert_eq!(
        after_resync.intensity_op_upload_count,
        baseline.intensity_op_upload_count
    );
    assert_eq!(
        after_resync.transfer_op_upload_count,
        baseline.transfer_op_upload_count
    );
    assert_eq!(
        after_resync.emission_op_upload_count,
        baseline.emission_op_upload_count
    );
}

#[test]
fn c8_accumulator_intensity_uses_eval_eml_only() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    legacy_intensity_shader_deleted();

    let (reg, loyalty_pid, resources_pid, _, _, _) = c8_full_registry();
    let stock_col = col_global(&reg, resources_pid, SubFieldRole::Named("stock".into()));
    let pool_col = col_global(&reg, resources_pid, SubFieldRole::Named("pool".into()));

    let (world, alloc) = c8_full_world(&reg, loyalty_pid, resources_pid);
    let cohort_slot = first_cohort_slot(&world, &alloc);
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

    let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
    all_c8_flags(&mut proto);
    proto.initial_gpu_sync(&coord, &mut state);
    sync_c8_substrates(
        &mut state,
        &proto.registry,
        cohort_slot,
        stock_col,
        pool_col,
    );

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

    assert!(state.accumulator_intensity_eml_active);
    assert!(
        state
            .accumulator_runtime
            .as_ref()
            .unwrap()
            .intensity_op_upload_count()
            > 0
    );
}
