//! AccumulatorOp reduction vs CPU oracle tolerance confirmation.

use simthing_core::{
    DimensionRegistry, PropertyValue, ReductionRule, SimProperty, SimThing, SimThingKind,
    SubFieldRole,
};
use simthing_gpu::{
    build_column_rule_descriptors, build_topology, cpu_reduce_oracle, encode_column_rules,
    plan_reduction_orderband, project_tree_to_values, GpuContext, Pipelines, SlotAllocator,
    TopologyState, WorldGpuState,
};

const TOL: f32 = 1e-5;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn max_abs_error(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0_f32, f32::max)
}

fn weighted_mean_fixture() -> (DimensionRegistry, SimThing, SlotAllocator, usize) {
    let mut reg = DimensionRegistry::new();
    let pop_id = reg.register(SimProperty::simple("demo", "population", 0));
    let pop_layout = reg.property(pop_id).layout.clone();
    let pop_a_off = pop_layout.offset_of(&SubFieldRole::Amount).unwrap();

    let mut loyalty = SimProperty::simple("core", "loyalty", 0);
    let loyalty_layout = loyalty.layout.clone();
    let loyalty_a_off = loyalty_layout.offset_of(&SubFieldRole::Amount).unwrap();
    loyalty.layout.sub_fields[0].reduction_override =
        Some(ReductionRule::WeightedMean { by: pop_id });
    let lid = reg.register(loyalty);

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut loc = SimThing::new(SimThingKind::Location, 0);
    for &(loyalty_amt, pop_amt) in &[(0.40, 100.0), (0.85, 200.0), (0.10, 50.0)] {
        let mut c = SimThing::new(SimThingKind::Cohort, 0);
        let mut lpv = PropertyValue::from_layout(&loyalty_layout);
        lpv.data[loyalty_a_off] = loyalty_amt;
        c.add_property(lid, lpv);
        let mut ppv = PropertyValue::from_layout(&pop_layout);
        ppv.data[pop_a_off] = pop_amt;
        c.add_property(pop_id, ppv);
        loc.add_child(c);
    }
    world.add_child(loc);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let n_cols = reg.total_columns;
    (reg, world, alloc, n_cols)
}

fn upload_topology(state: &mut WorldGpuState, world: &SimThing, reg: &DimensionRegistry, alloc: &SlotAllocator) {
    let n_dims = state.n_dims as usize;
    let topo = build_topology(world, alloc);
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

fn run_accumulator_reduction(state: &mut WorldGpuState) {
    let pipelines = Pipelines::new(&state.ctx);
    let mut runtime = state.accumulator_runtime.take().unwrap();
    let mut session = runtime.take_reduction_soft_session().unwrap();
    pipelines.run_accumulator_reduction_passes(state, &mut session);
    runtime.restore_reduction_soft_session(Some(session));
    state.accumulator_runtime = Some(runtime);
}

#[test]
fn accumulator_weighted_mean_matches_cpu_oracle_within_1e_5() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (reg, world, alloc, n_dims) = weighted_mean_fixture();
    let topo = build_topology(&world, &alloc);
    let descriptors = build_column_rule_descriptors(&reg, n_dims);

    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let mut flat = vec![0.0_f32; state.values_len()];
    project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
    state.write_values(&flat);
    upload_topology(&mut state, &world, &reg, &alloc);

    state.ensure_reduction_soft_accumulator();
    let topo_state = TopologyState::build(&world, &alloc);
    let plan = plan_reduction_orderband(&topo_state, &descriptors, state.n_dims).unwrap();
    state
        .upload_reduction_soft_ops_with_bands(&plan.ops, plan.n_bands)
        .unwrap();

    let mut cpu_output = vec![0.0_f32; flat.len()];
    cpu_reduce_oracle(&topo, &descriptors, n_dims, &flat, &mut cpu_output);

    run_accumulator_reduction(&mut state);
    let gpu_output = state.read_output_vectors();

    let err = max_abs_error(&cpu_output, &gpu_output);
    eprintln!("accumulator weighted mean max_abs_error={err}");
    assert!(err < TOL, "max_abs_error={err}");
}

#[test]
fn accumulator_mean_matches_cpu_oracle_within_1e_5() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    let lid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let layout = reg.property(lid).layout.clone();
    let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();

    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut loc = SimThing::new(SimThingKind::Location, 0);
    for &a in &[0.40, 0.85, 0.10] {
        let mut c = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[a_off] = a;
        c.add_property(lid, pv);
        loc.add_child(c);
    }
    world.add_child(loc);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let n_dims = reg.total_columns;
    let topo = build_topology(&world, &alloc);
    let descriptors = build_column_rule_descriptors(&reg, n_dims);

    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let mut flat = vec![0.0_f32; state.values_len()];
    project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
    state.write_values(&flat);
    upload_topology(&mut state, &world, &reg, &alloc);

    state.ensure_reduction_soft_accumulator();
    let topo_state = TopologyState::build(&world, &alloc);
    let plan = plan_reduction_orderband(&topo_state, &descriptors, state.n_dims).unwrap();
    state
        .upload_reduction_soft_ops_with_bands(&plan.ops, plan.n_bands)
        .unwrap();

    let mut cpu_output = vec![0.0_f32; flat.len()];
    cpu_reduce_oracle(&topo, &descriptors, n_dims, &flat, &mut cpu_output);

    run_accumulator_reduction(&mut state);
    let gpu_output = state.read_output_vectors();

    let err = max_abs_error(&cpu_output, &gpu_output);
    assert!(err < TOL, "max_abs_error={err}");
}
