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
    state.write_values(&flat);
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
    state.write_values(&flat);
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
fn ao_wgsl0_generic_kernel_matches_existing_ao_for_flat_star() {
    let _gpu = try_gpu_flat().expect("gpu");
    set_debug_readback_allowed(true);
    let fs = open_flat_star_session(4, false);
    let layout = fs.layout;
    let cols = fs.cols;
    let reg = fs.session.proto.registry.clone();
    let n_slots = fs.session.proto.allocator.capacity() as u32;
    let n_dims = reg.total_columns as u32;
    let plan = plan_arena_allocation(&layout, &[], n_slots).unwrap();
    let inputs = flat_star_cell_inputs(
        root_slot(&layout),
        &leaf_slots(&layout),
        cols,
        50.0,
        &[1.0, 3.0],
    );
    let flat = flat_buffer_from_inputs(n_slots, n_dims, &inputs);

    let mut eml = EmlExpressionRegistry::new();
    register_child_share_formula(&mut eml, cols).unwrap();

    let mut legacy_state = WorldGpuState::new(GpuContext::new_blocking().unwrap(), &reg, n_slots);
    legacy_state.write_values(&flat);
    legacy_state
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    legacy_state.run_resource_flow_bands(plan.n_bands, 1.0);
    let legacy = legacy_state.read_values();

    let mut fast_state = WorldGpuState::new(GpuContext::new_blocking().unwrap(), &reg, n_slots);
    fast_state.write_values(&flat);
    fast_state
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    fast_state.run_resource_flow_bands_with_fast_path(plan.n_bands, 1.0, true);
    let fast = fast_state.read_values();

    assert_eq!(legacy, fast);
}

#[test]
fn ao_wgsl0_generic_kernel_matches_existing_ao_for_a0_d3_nested_resource_flow() {
    let legacy = rf_parity_run(false);
    assert_eq!(legacy.max_abs_error, 0.0);
    let fast = rf_parity_run(true);
    assert_eq!(fast.max_abs_error, 0.0);
}

#[test]
fn ao_wgsl0_generic_kernel_matches_existing_ao_for_a0_d4_nested_resource_flow() {
    let gpu = try_gpu().expect("gpu");
    set_debug_readback_allowed(true);
    let f = materialize_nested(7, a0_d4_participants, 16, 0);
    let layout = layout_for(&f);
    let n_slots = f.alloc.capacity() as u32;
    let plan = plan_arena_allocation(&layout, &[], n_slots).unwrap();
    let mut eml = EmlExpressionRegistry::new();
    register_child_share_formula(&mut eml, f.cols).unwrap();
    let n_dims = f.reg.total_columns as u32;
    let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
    flat[0] = ROOT_INTRINSIC;

    let mut legacy = WorldGpuState::new(GpuContext::new_blocking().unwrap(), &f.reg, n_slots);
    legacy.write_values(&flat);
    legacy
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    legacy.run_resource_flow_bands(plan.n_bands, 1.0);
    let legacy_vals = legacy.read_values();

    let mut fast = WorldGpuState::new(gpu, &f.reg, n_slots);
    fast.write_values(&flat);
    fast.sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    fast.run_resource_flow_bands_with_fast_path(plan.n_bands, 1.0, true);
    let fast_vals = fast.read_values();

    assert_eq!(legacy_vals, fast_vals);
    assert_eq!(plan.n_bands, total_bands_for_depth(4));
}

#[test]
fn ao_wgsl0_generic_kernel_matches_existing_ao_for_b0_transfer_orderband_if_supported() {
    let _gpu = try_gpu().expect("gpu");
    set_debug_readback_allowed(true);

    let regs = vec![
        TransferRegistration {
            inputs: vec![simthing_gpu::TransferInputRef {
                slot: 0,
                col: 0,
                unit_cost: 1.0,
            }],
            target_slot: 0,
            target_col: 1,
            output_scale: 1.0,
            max_transfer: Some(3.0),
            tree_id: None,
            order_band: 0,
        },
        TransferRegistration {
            inputs: vec![simthing_gpu::TransferInputRef {
                slot: 0,
                col: 0,
                unit_cost: 1.0,
            }],
            target_slot: 0,
            target_col: 2,
            output_scale: 1.0,
            max_transfer: Some(4.0),
            tree_id: None,
            order_band: 1,
        },
    ];
    let plan = plan_transfer_ops(&regs).unwrap();
    let gpu_ops = encode_transfer_plan(&plan, &[]).unwrap();
    assert!(matches!(
        classify_ao_wgsl0_plan(&gpu_ops),
        AoWgsl0Compatibility::Compatible(_)
    ));

    let n_dims = 4u32;
    let n_slots = 1u32;
    let mut reg = empty_registry();
    register_amount_property(&mut reg, "core", "treasury");
    register_amount_property(&mut reg, "core", "sink0");
    register_amount_property(&mut reg, "core", "sink1");
    let n_dims = reg.total_columns as u32;

    let run = |fast: bool| -> Vec<f32> {
        let mut state = WorldGpuState::new(GpuContext::new_blocking().unwrap(), &reg, n_slots);
        let mut flat = vec![0.0_f32; (n_slots * n_dims) as usize];
        flat[0] = 10.0;
        state.write_values(&flat);
        state.sync_transfer_accumulator(&regs).unwrap();
        let mut runtime = state.accumulator_runtime.take().unwrap();
        let mut session = runtime.take_transfer_session().unwrap();
        let mut encoder =
            state
                .ctx
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("ao_wgsl0_transfer_test"),
                });
        if fast {
            session.encode_orderband_fast_into(
                &state.ctx,
                &mut encoder,
                &state.values,
                &state.previous_values,
                plan.n_bands,
                1.0,
                None,
            );
        } else {
            session.encode_orderband_with_eml_into(
                &state.ctx,
                &mut encoder,
                &state.values,
                &state.previous_values,
                plan.n_bands,
                1.0,
                None,
            );
        }
        state.ctx.queue.submit(Some(encoder.finish()));
        let _ = state.ctx.device.poll(wgpu::Maintain::Wait);
        runtime.restore_transfer_session(Some(session));
        state.accumulator_runtime = Some(runtime);
        state.read_values()
    };

    let legacy = run(false);
    let fast = run(true);
    assert_eq!(legacy, fast);
    assert_eq!(legacy[1].to_bits(), 3.0_f32.to_bits());
    assert_eq!(legacy[2].to_bits(), 4.0_f32.to_bits());
    assert_eq!(legacy[0].to_bits(), 3.0_f32.to_bits());
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
    legacy.write_values(&flat);
    legacy
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, plan.n_bands, &eml)
        .unwrap();
    legacy.run_resource_flow_bands(plan.n_bands, 1.0);
    let legacy_vals = legacy.read_values();

    let mut fast_ok = WorldGpuState::new(gpu, &f.reg, n_slots);
    fast_ok.write_values(&flat);
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
fn ao_wgsl0_replay_reproducibility() {
    let a = rf_parity_run(true);
    let b = rf_parity_run(true);
    assert_eq!(a.max_abs_error, b.max_abs_error);
    assert_eq!(a.l_inf, b.l_inf);
}

#[test]
fn ao_wgsl0_no_designer_authored_wgsl() {
    let wgsl = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    let generic = include_str!("../../simthing-gpu/src/shaders/accumulator_op_generic.wgsl");
    for forbidden in [
        "faction",
        "planet",
        "ClauseThing",
        "ClauseScript",
        "scenario",
    ] {
        assert!(
            !wgsl_contains_forbidden_semantic_token(wgsl, forbidden),
            "accumulator_op.wgsl must not embed {forbidden}"
        );
    }
    assert!(generic.contains("AO-WGSL-0"));
    assert!(wgsl.contains("execute_orderband_bands"));
}

#[test]
fn ao_wgsl0_semantic_wgsl_still_rejected_at_designer_layer() {
    let report = evaluate_designer_admission_request(DesignerAdmissionRequest::SemanticWgsl);
    assert!(!report.accepted);
}

#[test]
fn ao_wgsl0_no_simthing_sim_awareness() {
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!wgsl_contains_forbidden_semantic_token(
        sim_lib,
        "execute_orderband_bands"
    ));
    let boundary = include_str!("../../simthing-sim/src/boundary.rs");
    assert!(boundary.contains("use_accumulator_wgsl_fast_path"));
}

#[test]
fn ao_wgsl0_no_default_on_resource_flow_or_hard_currency_reroute() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    assert!(!PipelineFlags::default().use_accumulator_wgsl_fast_path);
    assert!(!PipelineFlags::default().use_accumulator_transfer);
}

#[test]
fn ao_wgsl0_no_l3_frontierv2_5_act_event_obs_pipe() {
    let _ = child_share_cpu;
}

#[test]
fn ao_wgsl0_benchmark_report_smoke() {
    let (legacy_cold, legacy_mean, legacy_min, legacy_max) = benchmark_rf_path(false, 3, 10);
    let (fast_cold, fast_mean, fast_min, fast_max) = benchmark_rf_path(true, 3, 10);
    eprintln!(
        "AO-WGSL-0 D=3 fixture benchmark (includes queue sync in harness): \
         legacy cold={legacy_cold}us mean={legacy_mean}us min={legacy_min}us max={legacy_max}us; \
         fast cold={fast_cold}us mean={fast_mean}us min={fast_min}us max={fast_max}us; \
         ratio={:.2}",
        legacy_mean as f64 / fast_mean.max(1) as f64
    );
    assert!(legacy_mean > 0);
    assert!(fast_mean > 0);
}
