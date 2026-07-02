//! Phase AO-WGSL-0 — generic AccumulatorOp WGSL performance path.

#[path = "support/resource_economy_materialize.rs"]
mod materialize_support;

#[path = "support/e11_nested.rs"]
mod nested_support;

#[path = "support/e11_flat_star.rs"]
mod flat_star_support;

use flat_star_support::{
    flat_star_cell_inputs, leaf_slots, open_flat_star_session, root_slot, try_gpu as try_gpu_flat,
};
use materialize_support::{empty_registry, register_amount_property};
use nested_support::{
    a0_d3_participants, a0_d4_participants, layout_for, leaves, materialize_nested, try_gpu,
};
use simthing_core::EmlExpressionRegistry;
use simthing_driver::{
    child_share_cpu, plan_arena_allocation, register_child_share_formula,
    run_arena_allocation_oracle, total_bands_for_depth,
};
use simthing_gpu::{
    ao_wgsl0_fast_path_compatible, classify_ao_wgsl0_plan, encode_transfer_plan, plan_transfer_ops,
    set_debug_readback_allowed, threshold_registrations_to_ops, AoWgsl0Compatibility, GpuContext,
    ThresholdRegistration, TransferRegistration, WorldGpuState, DIR_UPWARD,
};
use simthing_sim::PipelineFlags;
use simthing_spec::designer_admission::{
    evaluate_designer_admission_request, DesignerAdmissionRequest,
};
use std::collections::HashMap;
use std::time::Instant;

const ROOT_INTRINSIC: f32 = 100.0;

fn wgsl_contains_forbidden_semantic_token(source: &str, token: &str) -> bool {
    source
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .any(|word| word == token)
}

fn rf_parity_run(fast_path: bool) -> nested_support::ParityResult {
    let gpu = try_gpu().expect("gpu required");
    set_debug_readback_allowed(true);
    let f = materialize_nested(7, a0_d3_participants, 16, 0);
    let layout = layout_for(&f);
    let c = f.cols;
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaf_nodes = leaves(&layout);
    let n_dims = f.reg.total_columns as u32;
    let n_slots = f.alloc.capacity() as u32;
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    let mut oracle = HashMap::new();

    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;
    flat[idx(root_slot, c.intrinsic_flow_col)] = ROOT_INTRINSIC;
    oracle.insert((root_slot, c.intrinsic_flow_col), ROOT_INTRINSIC);
    for node in layout.iter_all().into_iter().filter(|node| node.depth > 0) {
        flat[idx(node.participant_slot, c.weight_col)] = 1.0;
        oracle.insert((node.participant_slot, c.weight_col), 1.0);
    }
    if let Some(first) = leaf_nodes.first() {
        flat[idx(first.participant_slot, c.weight_col)] = 1.0;
        oracle.insert((first.participant_slot, c.weight_col), 1.0);
    }
    if let Some(second) = leaf_nodes.get(1) {
        flat[idx(second.participant_slot, c.weight_col)] = 3.0;
        oracle.insert((second.participant_slot, c.weight_col), 3.0);
    }

    run_arena_allocation_oracle(&layout, &mut oracle, 1.0);

    let mut state = WorldGpuState::new(gpu, &f.reg, n_slots);
    state.install_resolved_values_at_boundary(&flat);
    let plan = plan_arena_allocation(&layout, &[], n_slots).unwrap();
    let mut eml = EmlExpressionRegistry::new();
    register_child_share_formula(&mut eml, c).unwrap();
    state
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    state.run_resource_flow_bands_with_fast_path(plan.n_bands, 1.0, fast_path);
    let gpu_vals = state.read_values();

    let mut max_abs_error = 0.0_f32;
    let mut l_inf = 0.0_f32;
    for leaf in &leaf_nodes {
        let cpu = oracle
            .get(&(leaf.participant_slot, c.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu_val = gpu_vals[idx(leaf.participant_slot, c.allocated_flow_col)];
        let err = (cpu - gpu_val).abs();
        max_abs_error = max_abs_error.max(err);
        l_inf = l_inf.max(err);
        assert_eq!(cpu.to_bits(), gpu_val.to_bits(), "leaf parity");
    }
    nested_support::ParityResult {
        max_abs_error,
        l_inf,
        leaf_count: leaf_nodes.len(),
    }
}

fn benchmark_rf_path(fast_path: bool, warmup: u32, samples: u32) -> (u64, u64, u64, u64) {
    let gpu = try_gpu().expect("gpu");
    let f = materialize_nested(7, a0_d3_participants, 16, 0);
    let layout = layout_for(&f);
    let n_slots = f.alloc.capacity() as u32;
    let n_dims = f.reg.total_columns as u32;
    let plan = plan_arena_allocation(&layout, &[], n_slots).unwrap();
    let mut eml = EmlExpressionRegistry::new();
    register_child_share_formula(&mut eml, f.cols).unwrap();
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    flat[0] = ROOT_INTRINSIC;

    let mut state = WorldGpuState::new(gpu, &f.reg, n_slots);
    state.install_resolved_values_at_boundary(&flat);
    state
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();

    for _ in 0..warmup {
        state.run_resource_flow_bands_with_fast_path(plan.n_bands, 1.0, fast_path);
    }

    let cold = {
        let t0 = Instant::now();
        state.run_resource_flow_bands_with_fast_path(plan.n_bands, 1.0, fast_path);
        t0.elapsed().as_micros() as u64
    };

    let mut times = Vec::with_capacity(samples as usize);
    for _ in 0..samples {
        let t0 = Instant::now();
        state.run_resource_flow_bands_with_fast_path(plan.n_bands, 1.0, fast_path);
        times.push(t0.elapsed().as_micros() as u64);
    }
    let warm_mean = times.iter().sum::<u64>() / samples as u64;
    let warm_min = *times.iter().min().unwrap();
    let warm_max = *times.iter().max().unwrap();
    (cold, warm_mean, warm_min, warm_max)
}

fn flat_buffer_from_inputs(
    n_slots: u32,
    n_dims: u32,
    inputs: &HashMap<(u32, u32), f32>,
) -> Vec<f32> {
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    for ((slot, col), value) in inputs {
        flat[(slot * n_dims + col) as usize] = *value;
    }
    flat
}

#[test]
fn ao_wgsl0_unsupported_plan_falls_back_or_rejects_without_semantics_change() {
    let gpu = try_gpu().expect("gpu");
    let f = materialize_nested(7, a0_d3_participants, 16, 0);
    let layout = layout_for(&f);
    let n_slots = f.alloc.capacity() as u32;
    let plan = plan_arena_allocation(&layout, &[], n_slots).unwrap();
    let mut eml = EmlExpressionRegistry::new();
    register_child_share_formula(&mut eml, f.cols).unwrap();
    let n_dims = f.reg.total_columns as u32;
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    flat[0] = ROOT_INTRINSIC;

    let mut legacy = WorldGpuState::new(GpuContext::new_blocking().unwrap(), &f.reg, n_slots);
    legacy.install_resolved_values_at_boundary(&flat);
    legacy
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    legacy.run_resource_flow_bands(plan.n_bands, 1.0);
    let legacy_vals = legacy.read_values();

    let mut fast_ok = WorldGpuState::new(gpu, &f.reg, n_slots);
    fast_ok.install_resolved_values_at_boundary(&flat);
    fast_ok
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    fast_ok.run_resource_flow_bands_with_fast_path(plan.n_bands, 1.0, true);
    assert_eq!(legacy_vals, fast_ok.read_values());

    let (threshold_ops, _) = threshold_registrations_to_ops(&[ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: DIR_UPWARD,
        event_kind: 1,
        buffer: 0,
    }])
    .unwrap();
    let gpu_threshold: Vec<_> = threshold_ops
        .iter()
        .map(|op| simthing_gpu::AccumulatorOpGpu::from_op(op).unwrap())
        .collect();
    assert!(!ao_wgsl0_fast_path_compatible(&gpu_threshold));
}

#[test]
fn ao_wgsl0_semantic_wgsl_still_rejected_at_designer_layer() {
    let report = evaluate_designer_admission_request(DesignerAdmissionRequest::SemanticWgsl);
    assert!(!report.accepted);
}

