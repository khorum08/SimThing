//! ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 + SIM-GPU-ACCUMULATOR-BACKEND-0 +
//! SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0 + SIM-GPU-READBACK-SCOPE-0 — sim tick ownership proofs.

use std::sync::Mutex;

use simthing_core::StructuralScalarChannel;
use simthing_driver::compile_structural_link_neighbor_sum_plan;
use simthing_gpu::{
    debug_readback_allowed, scoped_debug_readback_allowed, set_debug_readback_allowed,
};
use simthing_mapeditor::runtime_vertical_seed_scenario_spec;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, execute_accumulator_plan_tick_gpu, gpu_context_blocking,
    SimGpuAccumulatorTickState, SimGpuReadbackPolicy, SimTickError,
};

static READBACK_GATE_TEST_LOCK: Mutex<()> = Mutex::new(());

fn with_isolated_readback_gate_test<F: FnOnce()>(f: F) {
    let _lock = READBACK_GATE_TEST_LOCK.lock().expect("readback gate test lock");
    set_debug_readback_allowed(false);
    f();
    set_debug_readback_allowed(false);
}

fn vertical_seed_plan() -> simthing_core::CompiledAccumulatorOpPlan {
    let scenario = runtime_vertical_seed_scenario_spec();
    compile_structural_link_neighbor_sum_plan(
        &scenario,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("compile")
}

fn tick_body_source() -> &'static str {
    let source = include_str!("../src/accumulator_plan_tick.rs");
    let start = source.find("pub fn tick(").expect("tick function");
    let end = source[start..]
        .find("\n    /// Driver-compiled plan")
        .or_else(|| source[start..].find("\n    /// Read back"))
        .or_else(|| source[start..].find("\nfn validate_accumulator"))
        .expect("tick body end");
    &source[start..start + end]
}

#[test]
fn sim_tick_executes_driver_compiled_vertical_seed_accumulator_plan() {
    let plan = vertical_seed_plan();
    let output = execute_accumulator_plan_tick_cpu(&plan, &[10.0, 20.0]).expect("tick");
    assert_eq!(output.len(), 2);
}

#[test]
fn sim_tick_vertical_seed_outputs_20_10() {
    let plan = vertical_seed_plan();
    let output = execute_accumulator_plan_tick_cpu(&plan, &[10.0, 20.0]).expect("tick");
    assert_eq!(output, vec![20.0, 10.0]);
}

#[test]
fn sim_tick_owns_execution_boundary_not_studio() {
    let source = include_str!("../src/accumulator_plan_tick.rs");
    assert!(!source.contains("bevy"));
    assert!(!source.contains("mapeditor"));
    assert!(!source.contains("Studio"));
}

#[test]
fn sim_tick_does_not_use_structural_link_accumulator() {
    let source = include_str!("../src/accumulator_plan_tick.rs");
    assert!(!source.contains("structural_link_accumulator"));
}

#[test]
fn sim_gpu_tick_executes_driver_compiled_vertical_seed_plan() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let output = execute_accumulator_plan_tick_gpu(&ctx, &plan, &[10.0, 20.0]).expect("gpu tick");
    assert_eq!(output.len(), 2);
    eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_tick_vertical_seed_outputs_20_10() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let output = execute_accumulator_plan_tick_gpu(&ctx, &plan, &[10.0, 20.0]).expect("gpu tick");
    assert_eq!(output, vec![20.0, 10.0]);
    eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_tick_matches_cpu_tick_for_vertical_seed() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let inputs = [10.0, 20.0];
    let cpu = execute_accumulator_plan_tick_cpu(&plan, &inputs).expect("cpu tick");
    let gpu = execute_accumulator_plan_tick_gpu(&ctx, &plan, &inputs).expect("gpu tick");
    assert_eq!(cpu, gpu);
    assert_eq!(gpu, vec![20.0, 10.0]);
    eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_tick_rejects_wrong_input_len() {
    let plan = vertical_seed_plan();
    let err = execute_accumulator_plan_tick_cpu(&plan, &[10.0])
        .expect_err("shared validation rejects wrong length");
    assert!(matches!(
        err,
        SimTickError::InvalidInputLength {
            expected: 2,
            actual: 1
        }
    ));
}

#[test]
fn sim_gpu_tick_rejects_non_exact_integer_input() {
    let plan = vertical_seed_plan();
    let err = execute_accumulator_plan_tick_cpu(&plan, &[10.0, 0.5])
        .expect_err("shared validation rejects non-exact integer");
    assert!(matches!(err, SimTickError::NonExactIntegerInput { .. }));
}

#[test]
fn sim_gpu_tick_returns_error_or_partial_without_adapter() {
    match gpu_context_blocking() {
        Ok(ctx) => {
            let plan = vertical_seed_plan();
            let output =
                execute_accumulator_plan_tick_gpu(&ctx, &plan, &[10.0, 20.0]).expect("gpu tick");
            assert_eq!(output, vec![20.0, 10.0]);
            eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
        }
        Err(SimTickError::GpuUnavailable(_)) => {
            eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        }
        Err(other) => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn sim_gpu_tick_does_not_use_studio_or_bevy_state() {
    let source = include_str!("../src/accumulator_plan_tick.rs");
    assert!(!source.contains("bevy"));
    assert!(!source.contains("mapeditor"));
    assert!(!source.contains("Studio"));
}

#[test]
fn sim_gpu_tick_does_not_use_structural_link_accumulator() {
    let source = include_str!("../src/accumulator_plan_tick.rs");
    assert!(!source.contains("structural_link_accumulator"));
}

#[test]
fn sim_gpu_resident_state_initializes_from_driver_compiled_plan() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let state = SimGpuAccumulatorTickState::new(&ctx, plan.clone()).expect("init");
    assert_eq!(state.plan().slot_count, plan.slot_count);
    assert_eq!(state.plan().ops.len(), plan.ops.len());
    eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_resident_state_uploads_ops_once_or_only_on_plan_change() {
    let source = include_str!("../src/accumulator_plan_tick.rs");
    assert!(source.contains("upload_ops_resolving_input_lists"));
    let tick_body = tick_body_source();
    assert!(
        !tick_body.contains("upload_ops_resolving_input_lists"),
        "resident tick must not re-upload ops each tick"
    );
}

#[test]
fn sim_gpu_resident_state_ticks_vertical_seed_20_10() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let output = state
        .tick(&ctx, &[10.0, 20.0], SimGpuReadbackPolicy::ProofReadback)
        .expect("tick")
        .expect("proof readback");
    assert_eq!(output, vec![20.0, 10.0]);
    eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_resident_state_ticks_twice_with_different_inputs() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let first = state
        .tick(&ctx, &[10.0, 20.0], SimGpuReadbackPolicy::ProofReadback)
        .expect("tick 1")
        .expect("readback 1");
    assert_eq!(first, vec![20.0, 10.0]);
    let second = state
        .tick(&ctx, &[30.0, 40.0], SimGpuReadbackPolicy::ProofReadback)
        .expect("tick 2")
        .expect("readback 2");
    assert_eq!(second, vec![40.0, 30.0]);
    eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_resident_state_cpu_gpu_parity_vertical_seed() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let inputs = [10.0, 20.0];
    let cpu = execute_accumulator_plan_tick_cpu(&plan, &inputs).expect("cpu");
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let gpu = state
        .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("gpu tick")
        .expect("readback");
    assert_eq!(cpu, gpu);
    eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_resident_state_rejects_wrong_input_len() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let err = state
        .tick(&ctx, &[10.0], SimGpuReadbackPolicy::None)
        .expect_err("wrong len");
    assert!(matches!(
        err,
        SimTickError::InvalidInputLength {
            expected: 2,
            actual: 1
        }
    ));
}

#[test]
fn sim_gpu_resident_state_rejects_non_exact_integer_input() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let err = state
        .tick(&ctx, &[10.0, 0.5], SimGpuReadbackPolicy::None)
        .expect_err("non-exact");
    assert!(matches!(err, SimTickError::NonExactIntegerInput { .. }));
}

#[test]
fn sim_gpu_resident_state_proof_readback_is_explicit() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    assert!(state
        .tick(&ctx, &[10.0, 20.0], SimGpuReadbackPolicy::None)
        .expect("no readback tick")
        .is_none());
    assert_eq!(
        state
            .tick(&ctx, &[10.0, 20.0], SimGpuReadbackPolicy::ProofReadback)
            .expect("proof tick")
            .expect("values"),
        vec![20.0, 10.0]
    );
    eprintln!("SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_one_shot_helper_uses_resident_state_or_is_marked_proof_only() {
    let source = include_str!("../src/accumulator_plan_tick.rs");
    assert!(source.contains("One-shot proof/convenience helper"));
    assert!(source.contains("SimGpuAccumulatorTickState::new"));
    assert!(source.contains("ProofReadback"));
}

#[test]
fn sim_gpu_tick_does_not_silently_enable_debug_readback() {
    let tick_body = tick_body_source();
    assert!(
        !tick_body.contains("set_debug_readback_allowed"),
        "production tick must not silently enable debug readback"
    );
    let source = include_str!("../src/accumulator_plan_tick.rs");
    assert!(source.contains("run_with_proof_readback_enabled"));
    assert!(source.contains("scoped_debug_readback_allowed"));
}

#[test]
fn proof_readback_restores_debug_gate_after_success() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-GPU-READBACK-SCOPE-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let plan = vertical_seed_plan();
        let mut state = SimGpuAccumulatorTickState::new(&ctx, plan.clone()).expect("init");
        state
            .tick(&ctx, &[10.0, 20.0], SimGpuReadbackPolicy::ProofReadback)
            .expect("proof tick");
        assert!(!debug_readback_allowed());
        eprintln!("SIM-GPU-READBACK-SCOPE-0: REAL_ADAPTER_OBSERVED");
    });
}

#[test]
fn proof_readback_restores_debug_gate_after_readback_error_if_testable() {
    with_isolated_readback_gate_test(|| {
        let result: Result<(), SimTickError> = (|| {
            let _guard = scoped_debug_readback_allowed(true);
            assert!(debug_readback_allowed());
            Err(SimTickError::Readback("simulated readback failure".into()))
        })();
        assert!(result.is_err());
        assert!(!debug_readback_allowed());
    });
}

#[test]
fn proof_readback_does_not_leak_into_subsequent_none_tick() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-GPU-READBACK-SCOPE-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let plan = vertical_seed_plan();
        let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
        state
            .tick(&ctx, &[10.0, 20.0], SimGpuReadbackPolicy::ProofReadback)
            .expect("proof tick");
        assert!(!debug_readback_allowed());
        assert!(state
            .tick(&ctx, &[30.0, 40.0], SimGpuReadbackPolicy::None)
            .expect("none tick")
            .is_none());
        assert!(!debug_readback_allowed());
        eprintln!("SIM-GPU-READBACK-SCOPE-0: REAL_ADAPTER_OBSERVED");
    });
}

#[test]
fn production_none_tick_never_enables_debug_readback() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-GPU-READBACK-SCOPE-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let plan = vertical_seed_plan();
        let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
        assert!(state
            .tick(&ctx, &[10.0, 20.0], SimGpuReadbackPolicy::None)
            .expect("none tick")
            .is_none());
        assert!(!debug_readback_allowed());
    });
}

#[test]
fn resident_state_ticks_twice_after_proof_readback_without_forced_readback() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-GPU-READBACK-SCOPE-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let plan = vertical_seed_plan();
        let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
        assert_eq!(
            state
                .tick(&ctx, &[10.0, 20.0], SimGpuReadbackPolicy::ProofReadback)
                .expect("tick 1")
                .expect("readback 1"),
            vec![20.0, 10.0]
        );
        assert!(!debug_readback_allowed());
        assert!(state
            .tick(&ctx, &[30.0, 40.0], SimGpuReadbackPolicy::None)
            .expect("none tick")
            .is_none());
        assert_eq!(
            state
                .tick(&ctx, &[30.0, 40.0], SimGpuReadbackPolicy::ProofReadback)
                .expect("tick 2")
                .expect("readback 2"),
            vec![40.0, 30.0]
        );
        assert!(!debug_readback_allowed());
        eprintln!("SIM-GPU-READBACK-SCOPE-0: REAL_ADAPTER_OBSERVED");
    });
}

#[test]
fn one_shot_gpu_helper_scopes_proof_readback() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("SIM-GPU-READBACK-SCOPE-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let plan = vertical_seed_plan();
        let output =
            execute_accumulator_plan_tick_gpu(&ctx, &plan, &[10.0, 20.0]).expect("one-shot gpu");
        assert_eq!(output, vec![20.0, 10.0]);
        assert!(!debug_readback_allowed());
        let source = include_str!("../src/accumulator_plan_tick.rs");
        assert!(source.contains("scoped_debug_readback_allowed"));
    });
}
