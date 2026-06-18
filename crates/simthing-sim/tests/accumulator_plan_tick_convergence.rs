//! ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 + SIM-GPU-ACCUMULATOR-BACKEND-0 — sim tick ownership proofs.

use simthing_core::StructuralScalarChannel;
use simthing_driver::compile_structural_link_neighbor_sum_plan;
use simthing_mapeditor::runtime_vertical_seed_scenario_spec;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, execute_accumulator_plan_tick_gpu, gpu_context_blocking,
    SimTickError,
};

fn vertical_seed_plan() -> simthing_core::CompiledAccumulatorOpPlan {
    let scenario = runtime_vertical_seed_scenario_spec();
    compile_structural_link_neighbor_sum_plan(
        &scenario,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("compile")
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
        eprintln!("SIM-GPU-ACCUMULATOR-BACKEND-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let output = execute_accumulator_plan_tick_gpu(&ctx, &plan, &[10.0, 20.0]).expect("gpu tick");
    assert_eq!(output.len(), 2);
    eprintln!("SIM-GPU-ACCUMULATOR-BACKEND-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_tick_vertical_seed_outputs_20_10() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-ACCUMULATOR-BACKEND-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let output = execute_accumulator_plan_tick_gpu(&ctx, &plan, &[10.0, 20.0]).expect("gpu tick");
    assert_eq!(output, vec![20.0, 10.0]);
    eprintln!("SIM-GPU-ACCUMULATOR-BACKEND-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_tick_matches_cpu_tick_for_vertical_seed() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-ACCUMULATOR-BACKEND-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = vertical_seed_plan();
    let inputs = [10.0, 20.0];
    let cpu = execute_accumulator_plan_tick_cpu(&plan, &inputs).expect("cpu tick");
    let gpu = execute_accumulator_plan_tick_gpu(&ctx, &plan, &inputs).expect("gpu tick");
    assert_eq!(cpu, gpu);
    assert_eq!(gpu, vec![20.0, 10.0]);
    eprintln!("SIM-GPU-ACCUMULATOR-BACKEND-0: REAL_ADAPTER_OBSERVED");
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
            eprintln!("SIM-GPU-ACCUMULATOR-BACKEND-0: REAL_ADAPTER_OBSERVED");
        }
        Err(SimTickError::GpuUnavailable(_)) => {
            eprintln!("SIM-GPU-ACCUMULATOR-BACKEND-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
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
