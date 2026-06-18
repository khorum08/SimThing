//! SIMTHING-SIM-DEVDEP-SEAM-0 — resident tick proofs over hand-built generic plans.

mod support;

use simthing_gpu::debug_readback_allowed;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy, SimTickError,
};

use support::accumulator_plan_fixtures::{
    forked_four_slot_dense_inputs, forked_four_slot_input_list_plan,
};
use support::readback_gate::with_isolated_readback_gate_test;

#[test]
fn sim_cpu_tick_executes_forked_four_slot_input_list_plan() {
    let plan = forked_four_slot_input_list_plan();
    let output = execute_accumulator_plan_tick_cpu(&plan, &forked_four_slot_dense_inputs())
        .expect("cpu tick");
    assert_eq!(output.len(), 4);
}

#[test]
fn sim_gpu_resident_tick_executes_forked_four_slot_input_list_plan() {
    with_isolated_readback_gate_test(|| run_gpu_resident_tick_executes_forked_plan());
}

fn run_gpu_resident_tick_executes_forked_plan() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIMTHING-SIM-DEVDEP-SEAM-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = forked_four_slot_input_list_plan();
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let output = state
        .tick(
            &ctx,
            &forked_four_slot_dense_inputs(),
            SimGpuReadbackPolicy::ProofReadback,
        )
        .expect("tick")
        .expect("readback");
    assert_eq!(output.len(), 4);
    eprintln!("SIMTHING-SIM-DEVDEP-SEAM-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_resident_tick_matches_cpu_tick_for_forked_four_slot_plan() {
    with_isolated_readback_gate_test(|| run_gpu_resident_tick_matches_cpu_forked());
}

fn run_gpu_resident_tick_matches_cpu_forked() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIMTHING-SIM-DEVDEP-SEAM-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = forked_four_slot_input_list_plan();
    let inputs = forked_four_slot_dense_inputs();
    let cpu = execute_accumulator_plan_tick_cpu(&plan, &inputs).expect("cpu");
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let gpu = state
        .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("gpu tick")
        .expect("readback");
    assert_eq!(cpu, gpu);
    assert_eq!(cpu, vec![20.0, 80.0, 20.0, 20.0]);
    eprintln!("SIMTHING-SIM-DEVDEP-SEAM-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_resident_tick_rejects_wrong_input_len_for_forked_four_slot_plan() {
    let plan = forked_four_slot_input_list_plan();
    let err = execute_accumulator_plan_tick_cpu(&plan, &[10.0, 20.0]).expect_err("wrong len");
    assert!(matches!(
        err,
        SimTickError::InvalidInputLength {
            expected: 4,
            actual: 2
        }
    ));
}

#[test]
fn sim_gpu_resident_tick_rejects_non_exact_integer_input_for_forked_four_slot_plan() {
    let plan = forked_four_slot_input_list_plan();
    let err =
        execute_accumulator_plan_tick_cpu(&plan, &[10.0, 20.0, 30.5, 40.0]).expect_err("non-exact");
    assert!(matches!(err, SimTickError::NonExactIntegerInput { .. }));
}

#[test]
fn forked_four_slot_proof_readback_does_not_leak_into_none_tick() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIMTHING-SIM-DEVDEP-SEAM-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let plan = forked_four_slot_input_list_plan();
        let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
        let inputs = forked_four_slot_dense_inputs();
        state
            .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("proof tick");
        assert!(!debug_readback_allowed());
        assert!(state
            .tick(&ctx, &inputs, SimGpuReadbackPolicy::None)
            .expect("none tick")
            .is_none());
        assert!(!debug_readback_allowed());
        eprintln!("SIMTHING-SIM-DEVDEP-SEAM-0: REAL_ADAPTER_OBSERVED");
    });
}
