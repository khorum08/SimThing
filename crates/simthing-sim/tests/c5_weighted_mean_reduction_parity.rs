//! C-5 Mean / WeightedMean AccumulatorOp reduction parity and integration tests.

use simthing_core::{
    DimensionRegistry, PropertyValue, ReductionRule, SimProperty, SimThing, SimThingKind,
    SoftAggregateGuard, SubFieldRole,
};
use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{
    build_column_rule_descriptors, build_topology, cpu_reduce_oracle_call_count,
    encode_column_rules, plan_reduction_orderband, project_tree_to_values,
    reset_cpu_reduce_oracle_call_count, set_debug_readback_allowed, summaries_from_values,
    GpuContext, Pipelines, SlotAllocator, Topology, TopologyState, WorldGpuState,
    THRESH_BUF_OUTPUT,
};
use simthing_sim::{
    assert_no_hard_trigger_on_soft_aggregate, BoundaryProtocol, SoftAggregateViolation,
    ThresholdSemantic,
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

fn upload_topology(state: &mut WorldGpuState, topo: &Topology, reg: &DimensionRegistry) {
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

fn mean_tree_fixture() -> (DimensionRegistry, SimThing, SlotAllocator) {
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
    (reg, world, alloc)
}

fn setup_mean_state() -> (WorldGpuState, DimensionRegistry, Topology, Vec<f32>) {
    let ctx = GpuContext::new_blocking().expect("gpu");
    let (reg, world, alloc) = mean_tree_fixture();
    let n_dims = reg.total_columns;
    let topo = build_topology(&world, &alloc);
    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let mut flat = vec![0.0_f32; state.values_len()];
    project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
    state.write_values(&flat);
    upload_topology(&mut state, &topo, &reg);

    state.ensure_reduction_soft_accumulator();
    let topo_state = TopologyState::build(&world, &alloc);
    let descriptors = build_column_rule_descriptors(&reg, n_dims);
    let plan = plan_reduction_orderband(&topo_state, &descriptors, state.n_dims).unwrap();
    state
        .upload_reduction_soft_ops_with_bands(&plan.ops, plan.n_bands)
        .unwrap();

    (state, reg, topo, flat)
}

fn run_c5_reduction_only(state: &mut WorldGpuState) {
    let pipelines = Pipelines::new(&state.ctx);
    let mut runtime = state.accumulator_runtime.take().unwrap();
    let mut reduction_session = runtime.take_reduction_soft_session().unwrap();
    pipelines.run_c5_soft_reduction_passes(state, &mut reduction_session);
    runtime.restore_reduction_soft_session(Some(reduction_session));
    state.accumulator_runtime = Some(runtime);
}

#[test]
fn c5_accumulator_mean_three_runs_bit_identical() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (mut state, _, _, _) = setup_mean_state();
    run_c5_reduction_only(&mut state);
    let run1 = state.read_output_vectors();
    run_c5_reduction_only(&mut state);
    let run2 = state.read_output_vectors();
    run_c5_reduction_only(&mut state);
    let run3 = state.read_output_vectors();

    for (i, (a, b)) in run1.iter().zip(run2.iter()).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "run1 vs run2 at {i}");
    }
    for (i, (a, b)) in run1.iter().zip(run3.iter()).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "run1 vs run3 at {i}");
    }
}

#[test]
fn c5_mean_legacy_vs_accumulator_within_1e_5() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (mut state, _reg, _topo, flat) = setup_mean_state();
    let n_bands = state.accumulator_reduction_soft_bands;

    let pipelines = Pipelines::new(&state.ctx);
    state.set_reduction_soft_dispatch(false, 0);
    pipelines.run_reduction_passes(&state);
    let legacy = state.read_output_vectors();

    state.write_values(&flat);
    state.set_reduction_soft_dispatch(true, n_bands);
    run_c5_reduction_only(&mut state);
    let acc = state.read_output_vectors();

    let err = max_abs_error(&legacy, &acc);
    assert!(err < TOL, "legacy vs accumulator max_abs_error={err}");
}

#[test]
fn c5_production_path_no_cpu_mediated_reduction() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    reset_cpu_reduce_oracle_call_count();
    let (mut state, _, _, _) = setup_mean_state();
    for _ in 0..50 {
        run_c5_reduction_only(&mut state);
    }
    assert_eq!(
        cpu_reduce_oracle_call_count(),
        0,
        "cpu_reduce_oracle must not run on production C-5 tick path"
    );
}

#[test]
fn c5_world_summary_matches_full_values_after_weighted_mean_reduction() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);

    let (reg, world, alloc) = mean_tree_fixture();
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
    proto.flags.use_accumulator_reduction_soft = true;
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
    assert_eq!(gpu_summary.len(), cpu_summary.len());
    assert_eq!(gpu_summary, cpu_summary);
    let _ = patcher;
}

#[test]
fn c5_assert_no_hard_trigger_blocks_weighted_mean_without_guard() {
    let mut reg = DimensionRegistry::new();
    let weight_pid = reg.register(SimProperty::simple("core", "headcount", 0));
    let mut p = SimProperty::simple("tech", "research", 0);
    p.layout.sub_fields[0].reduction_override =
        Some(ReductionRule::WeightedMean { by: weight_pid });
    let pid = reg.register(p);
    let stid = SimThing::new(SimThingKind::Cohort, 0).id;
    let sem = ThresholdSemantic::FissionTrigger {
        sim_thing_id: stid,
        property_id: pid,
        template_idx: 0,
    };

    assert!(assert_no_hard_trigger_on_soft_aggregate(
        &sem,
        pid,
        &SubFieldRole::Amount,
        THRESH_BUF_OUTPUT,
        &reg,
    )
    .is_err());
}

#[test]
fn c5_assert_no_hard_trigger_allows_weighted_mean_with_quantized() {
    let mut reg = DimensionRegistry::new();
    let weight_pid = reg.register(SimProperty::simple("core", "headcount", 0));
    let mut p = SimProperty::simple("tech", "research", 0);
    p.layout.sub_fields[0].reduction_override =
        Some(ReductionRule::WeightedMean { by: weight_pid });
    p.layout.sub_fields[0].soft_aggregate_guard =
        Some(SoftAggregateGuard::Quantized { step: 0.01 });
    let pid = reg.register(p);
    let stid = SimThing::new(SimThingKind::Cohort, 0).id;
    let sem = ThresholdSemantic::FissionTrigger {
        sim_thing_id: stid,
        property_id: pid,
        template_idx: 0,
    };

    assert!(assert_no_hard_trigger_on_soft_aggregate(
        &sem,
        pid,
        &SubFieldRole::Amount,
        THRESH_BUF_OUTPUT,
        &reg,
    )
    .is_ok());
}

#[test]
fn c5_unguarded_allowed_for_aggregate_alert_path() {
    let mut reg = DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let stid = SimThing::new(SimThingKind::Cohort, 0).id;
    let sem = ThresholdSemantic::AggregateAlert {
        sim_thing_id: stid,
        property_id: pid,
        sub_field: SubFieldRole::Amount,
    };
    assert!(assert_no_hard_trigger_on_soft_aggregate(
        &sem,
        pid,
        &SubFieldRole::Amount,
        THRESH_BUF_OUTPUT,
        &reg,
    )
    .is_ok());
}

#[test]
fn c5_threshold_registration_on_unguarded_weighted_mean_output_panics() {
    let mut reg = DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let stid = SimThing::new(SimThingKind::Cohort, 0).id;
    let err = assert_no_hard_trigger_on_soft_aggregate(
        &ThresholdSemantic::FissionTrigger {
            sim_thing_id: stid,
            property_id: pid,
            template_idx: 0,
        },
        pid,
        &SubFieldRole::Amount,
        THRESH_BUF_OUTPUT,
        &reg,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        SoftAggregateViolation::HardTriggerOnUnguardedSoftAggregate { .. }
    ));
}
