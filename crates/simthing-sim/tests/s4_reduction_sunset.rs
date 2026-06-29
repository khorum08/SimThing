//! S-4 reduction sunset: AccumulatorOp is the sole production reduction path.

use std::path::Path;
use std::sync::Mutex;

use simthing_core::{
    DimensionRegistry, PropertyValue, ReductionRule, SimProperty, SimThing, SimThingKind,
    SubFieldRole,
};
use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{
    build_column_rule_descriptors, build_topology, cpu_reduce_oracle, cpu_reduce_oracle_call_count,
    encode_column_rules, plan_reduction_orderband, project_tree_to_values,
    reset_cpu_reduce_oracle_call_count, set_debug_readback_allowed, summaries_from_values,
    GpuContext, Pipelines, SlotAllocator, Topology, TopologyState, WorldGpuState,
    THRESH_BUF_OUTPUT,
};
use simthing_sim::{BoundaryProtocol, SimRuntimeTree};

const TOL: f32 = 1e-5;

static CPU_ORACLE_COUNTER_GUARD: Mutex<()> = Mutex::new(());

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn upload_topology(
    state: &mut WorldGpuState,
    topo: &simthing_gpu::Topology,
    reg: &DimensionRegistry,
) {
    let n_dims = state.n_dims as usize;
    let descriptors = build_column_rule_descriptors(reg, n_dims);
    let rules_u32 = encode_column_rules(&descriptors);
    let mut depth_slots = Vec::new();
    let mut depth_ranges = Vec::new();
    for bucket in &topo.depth_buckets {
        let offset = depth_slots.len() as u32;
        depth_slots.extend_from_slice(bucket);
        depth_ranges.push((offset, bucket.len() as u32));
    }
    state.upload_reduction_topology(
        &topo.child_starts,
        &topo.child_indices,
        &rules_u32,
        &depth_slots,
        depth_ranges,
    );
}

fn mixed_all_rules_fixture() -> (DimensionRegistry, SimThing, SlotAllocator) {
    let mut reg = DimensionRegistry::new();
    let pop_id = reg.register({
        let mut p = SimProperty::simple("demo", "population", 0);
        p.layout.sub_fields[0].reduction_override = Some(ReductionRule::Sum);
        p
    });
    let pop_layout = reg.property(pop_id).layout.clone();
    let pop_off = pop_layout.offset_of(&SubFieldRole::Amount).unwrap();

    let mut loyalty = SimProperty::simple("core", "loyalty", 0);
    loyalty.layout.sub_fields[0].reduction_override =
        Some(ReductionRule::WeightedMean { by: pop_id });
    let loyalty_id = reg.register(loyalty);
    let loyalty_layout = reg.property(loyalty_id).layout.clone();
    let loyalty_off = loyalty_layout.offset_of(&SubFieldRole::Amount).unwrap();

    let mut threat = SimProperty::simple("core", "threat", 0);
    threat.layout.sub_fields[0].reduction_override = Some(ReductionRule::Max);
    let threat_id = reg.register(threat);
    let threat_layout = reg.property(threat_id).layout.clone();
    let threat_off = threat_layout.offset_of(&SubFieldRole::Amount).unwrap();

    let mut scarcity = SimProperty::simple("core", "scarcity", 0);
    scarcity.layout.sub_fields[0].reduction_override = Some(ReductionRule::Min);
    let scarcity_id = reg.register(scarcity);
    let scarcity_layout = reg.property(scarcity_id).layout.clone();
    let scarcity_off = scarcity_layout.offset_of(&SubFieldRole::Amount).unwrap();

    let mut founder = SimProperty::simple("core", "founder_trait", 0);
    founder.layout.sub_fields[0].reduction_override = Some(ReductionRule::First);
    let founder_id = reg.register(founder);
    let founder_layout = reg.property(founder_id).layout.clone();
    let founder_off = founder_layout.offset_of(&SubFieldRole::Amount).unwrap();

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut loc = SimThing::new(SimThingKind::Location, 0);
    for &(loyalty, pop, threat, scarcity, founder) in
        &[(0.2, 10.0, -1.0, 5.0, 11.0), (0.8, 30.0, 3.0, 2.0, 99.0)]
    {
        let mut c = SimThing::new(SimThingKind::Cohort, 0);
        let mut lpv = PropertyValue::from_layout(&loyalty_layout);
        lpv.data[loyalty_off] = loyalty;
        c.add_property(loyalty_id, lpv);
        let mut ppv = PropertyValue::from_layout(&pop_layout);
        ppv.data[pop_off] = pop;
        c.add_property(pop_id, ppv);
        let mut tpv = PropertyValue::from_layout(&threat_layout);
        tpv.data[threat_off] = threat;
        c.add_property(threat_id, tpv);
        let mut spv = PropertyValue::from_layout(&scarcity_layout);
        spv.data[scarcity_off] = scarcity;
        c.add_property(scarcity_id, spv);
        let mut fpv = PropertyValue::from_layout(&founder_layout);
        fpv.data[founder_off] = founder;
        c.add_property(founder_id, fpv);
        loc.add_child(c);
    }
    world.add_child(loc);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    (reg, world, alloc)
}

fn setup_reduction_state(
    reg: &DimensionRegistry,
    world: &SimThing,
    alloc: &SlotAllocator,
) -> (WorldGpuState, Topology, Vec<f32>) {
    let ctx = GpuContext::new_blocking().expect("gpu");
    let n_dims = reg.total_columns;
    let topo = build_topology(world, alloc);
    let mut state = WorldGpuState::new(ctx, reg, alloc.capacity() as u32);
    let mut flat = vec![0.0_f32; state.values_len()];
    project_tree_to_values(world, reg, alloc, n_dims, &mut flat);
    state.install_resolved_values_at_boundary(&flat);
    upload_topology(&mut state, &topo, reg);

    state.ensure_reduction_soft_accumulator();
    let topo_state = TopologyState::build(world, alloc);
    let descriptors = build_column_rule_descriptors(reg, n_dims);
    let plan = plan_reduction_orderband(&topo_state, &descriptors, state.n_dims).unwrap();
    state
        .upload_reduction_soft_ops_with_bands(&plan.ops, plan.n_bands)
        .unwrap();

    (state, topo, flat)
}

fn run_accumulator_reduction(state: &mut WorldGpuState) {
    let pipelines = Pipelines::new(&state.ctx);
    let mut runtime = state.accumulator_runtime.take().unwrap();
    let mut session = runtime.take_reduction_soft_session().unwrap();
    pipelines.run_accumulator_reduction_passes(state, &mut session);
    runtime.restore_reduction_soft_session(Some(session));
    state.accumulator_runtime = Some(runtime);
}

#[test]
fn s4_legacy_reduction_shader_not_present() {
    assert!(
        !Path::new("crates/simthing-gpu/src/shaders/reduction.wgsl").exists(),
        "legacy reduction.wgsl must be deleted in S-4"
    );
}

#[test]
fn s4_all_reduction_rules_accumulator_matches_legacy_golden() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (reg, world, alloc) = mixed_all_rules_fixture();
    let (mut state, topo, flat) = setup_reduction_state(&reg, &world, &alloc);
    let n_dims = reg.total_columns;
    let descriptors = build_column_rule_descriptors(&reg, n_dims);
    let mut golden = vec![0.0_f32; flat.len()];
    {
        let _guard = CPU_ORACLE_COUNTER_GUARD.lock().unwrap();
        cpu_reduce_oracle(&topo, &descriptors, n_dims, &flat, &mut golden);
    }

    run_accumulator_reduction(&mut state);
    let acc = state.read_output_vectors();

    let pop_id = reg.id_of("demo", "population").expect("population");
    let loyalty_id = reg.id_of("core", "loyalty").expect("loyalty");
    let threat_id = reg.id_of("core", "threat").expect("threat");
    let scarcity_id = reg.id_of("core", "scarcity").expect("scarcity");
    let founder_id = reg.id_of("core", "founder_trait").expect("founder_trait");
    let pop_off = reg
        .property(pop_id)
        .layout
        .offset_of(&SubFieldRole::Amount)
        .unwrap();
    let loyalty_off = reg
        .property(loyalty_id)
        .layout
        .offset_of(&SubFieldRole::Amount)
        .unwrap();
    let threat_off = reg
        .property(threat_id)
        .layout
        .offset_of(&SubFieldRole::Amount)
        .unwrap();
    let scarcity_off = reg
        .property(scarcity_id)
        .layout
        .offset_of(&SubFieldRole::Amount)
        .unwrap();
    let founder_off = reg
        .property(founder_id)
        .layout
        .offset_of(&SubFieldRole::Amount)
        .unwrap();

    let exact_cols = [
        reg.column_range(pop_id).start + pop_off,
        reg.column_range(threat_id).start + threat_off,
        reg.column_range(scarcity_id).start + scarcity_off,
        reg.column_range(founder_id).start + founder_off,
    ];
    let soft_col = reg.column_range(loyalty_id).start + loyalty_off;

    for col in exact_cols {
        for slot in 0..state.n_slots as usize {
            let i = slot * reg.total_columns + col;
            assert_eq!(
                golden[i].to_bits(),
                acc[i].to_bits(),
                "exact col {col} slot {slot}"
            );
        }
    }
    for slot in 0..state.n_slots as usize {
        let i = slot * reg.total_columns + soft_col;
        assert!(
            (golden[i] - acc[i]).abs() < TOL,
            "soft col slot {slot}: golden={} acc={}",
            golden[i],
            acc[i]
        );
    }
}

#[test]
fn s4_reduction_path_no_cpu_mediated_reduction() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let _guard = CPU_ORACLE_COUNTER_GUARD.lock().unwrap();
    reset_cpu_reduce_oracle_call_count();
    let (reg, world, alloc) = mixed_all_rules_fixture();
    let (mut state, _, _) = setup_reduction_state(&reg, &world, &alloc);
    for _ in 0..50 {
        run_accumulator_reduction(&mut state);
    }
    assert_eq!(
        cpu_reduce_oracle_call_count(),
        0,
        "cpu_reduce_oracle must not run on production reduction path"
    );
}

#[test]
fn s4_combined_c1_c2_c4_reduction_path_green() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let (reg, world, alloc) = mixed_all_rules_fixture();
    let n_dims = reg.total_columns as u32;
    let n_slots = alloc.capacity() as u32;
    let ctx = GpuContext::new_blocking().expect("gpu");
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    let pipelines = Pipelines::new(&state.ctx);
    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    coord.shadow.fill(0.5);
    coord.upload_full_shadow(&state);

    let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
    proto.flags.use_accumulator_threshold_scan = true;
    proto.flags.use_accumulator_intent = true;
    proto.flags.use_accumulator_overlay_add = true;
    proto.initial_gpu_sync(&coord, &mut state);

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

    let values = state.read_values();
    let gpu_summary = state.readback_accumulator_summary().unwrap();
    let cpu_summary = summaries_from_values(&values, n_slots, n_dims);
    assert_eq!(gpu_summary, cpu_summary);

    let out = state.read_output_vectors();
    assert!(
        out.iter().any(|v| v.is_finite()),
        "output_vectors must be populated"
    );
    let _ = THRESH_BUF_OUTPUT;
    let _ = patcher;
}
