//! ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 — sim tick/boundary ownership proofs.

use simthing_core::StructuralScalarChannel;
use simthing_driver::compile_structural_link_neighbor_sum_plan;
use simthing_mapeditor::runtime_vertical_seed_scenario_spec;
use simthing_sim::execute_accumulator_plan_tick_cpu;

#[test]
fn sim_tick_executes_driver_compiled_vertical_seed_accumulator_plan() {
    let scenario = runtime_vertical_seed_scenario_spec();
    let plan = compile_structural_link_neighbor_sum_plan(
        &scenario,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("compile");
    let output = execute_accumulator_plan_tick_cpu(&plan, &[10.0, 20.0]).expect("tick");
    assert_eq!(output.len(), 2);
}

#[test]
fn sim_tick_vertical_seed_outputs_20_10() {
    let scenario = runtime_vertical_seed_scenario_spec();
    let plan = compile_structural_link_neighbor_sum_plan(
        &scenario,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("compile");
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
