//! ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 — driver compile behavioral proofs.

use simthing_core::{CombineFn, ConsumeMode, SourceSpec, StructuralScalarChannel};
use simthing_driver::{compile_structural_link_neighbor_sum_plan, DriverCompileError};
use simthing_mapeditor::runtime_vertical_seed_scenario_spec;
use simthing_spec::{SimThingScenarioLink, SimThingScenarioSpec};

fn vertical_seed_plan() -> simthing_core::CompiledAccumulatorOpPlan {
    let scenario = runtime_vertical_seed_scenario_spec();
    compile_structural_link_neighbor_sum_plan(
        &scenario,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("compile vertical seed")
}

#[test]
fn driver_compiles_vertical_seed_links_to_input_list_plan() {
    let plan = vertical_seed_plan();
    assert_eq!(plan.slot_count, 2);
    assert_eq!(plan.ops.len(), 2);
}

#[test]
fn driver_compiled_vertical_seed_plan_has_two_targets() {
    let plan = vertical_seed_plan();
    let mut targets: Vec<_> = plan.ops.iter().map(|op| op.targets[0].0).collect();
    targets.sort_unstable();
    assert_eq!(targets, vec![0, 1]);
}

#[test]
fn driver_compiled_vertical_seed_plan_target_0_gathers_1() {
    let plan = vertical_seed_plan();
    let op0 = plan
        .ops
        .iter()
        .find(|op| op.targets[0].0 == 0)
        .expect("target 0 op");
    let SourceSpec::ConjunctiveCrossing { inputs } = &op0.source else {
        panic!("expected input list source");
    };
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].slot, 1);
}

#[test]
fn driver_compiled_vertical_seed_plan_target_1_gathers_0() {
    let plan = vertical_seed_plan();
    let op1 = plan
        .ops
        .iter()
        .find(|op| op.targets[0].0 == 1)
        .expect("target 1 op");
    let SourceSpec::ConjunctiveCrossing { inputs } = &op1.source else {
        panic!("expected input list source");
    };
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].slot, 0);
}

#[test]
fn driver_compiled_plan_uses_accumulator_op_sum_input_list() {
    let plan = vertical_seed_plan();
    for op in &plan.ops {
        assert_eq!(op.combine, CombineFn::Sum);
        assert!(matches!(op.source, SourceSpec::ConjunctiveCrossing { .. }));
        assert_eq!(op.consume, ConsumeMode::AddToTarget);
    }
}

#[test]
fn driver_compile_rejects_invalid_scenario_links() {
    let mut scenario = runtime_vertical_seed_scenario_spec();
    scenario.links.push(SimThingScenarioLink {
        from_system_id: "99".into(),
        to_system_id: "1".into(),
    });
    let err = compile_structural_link_neighbor_sum_plan(
        &scenario,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect_err("invalid endpoint");
    assert!(matches!(
        err,
        DriverCompileError::LinkValidation(_) | DriverCompileError::InvalidLinkEndpoint { .. }
    ));
}

#[test]
fn driver_compile_does_not_use_studio_or_bevy_state() {
    let source = include_str!("../src/structural_link_accumulator_compile.rs");
    assert!(!source.contains("bevy"));
    assert!(!source.contains("mapeditor"));
    assert!(!source.contains("Studio"));
}
