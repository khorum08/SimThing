//! TERRAN-PIRATE-SCENARIO-SKELETON-0 — driver compile proofs for horizon skeleton.

use simthing_core::{CombineFn, ConsumeMode, SourceSpec, StructuralScalarChannel};
use simthing_driver::compile_structural_link_neighbor_sum_plan;
use simthing_gpu::{classify_ao_wgsl0_plan, AccumulatorOpGpu, AoWgsl0Compatibility};
use simthing_mapeditor::terran_pirate_skeleton_scenario_spec;

fn skeleton_plan() -> simthing_core::CompiledAccumulatorOpPlan {
    compile_structural_link_neighbor_sum_plan(
        &terran_pirate_skeleton_scenario_spec(),
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("compile skeleton")
}

#[test]
fn driver_compiles_terran_pirate_skeleton_links_to_input_list_plan() {
    let plan = skeleton_plan();
    assert_eq!(plan.slot_count, 4);
    assert_eq!(plan.ops.len(), 4);
}

#[test]
fn driver_compiled_terran_pirate_plan_target_adjacency_matches_links() {
    let plan = skeleton_plan();
    let corridor = plan
        .ops
        .iter()
        .find(|op| op.targets[0].0 == 1)
        .expect("corridor dense slot 1");
    let SourceSpec::ConjunctiveCrossing { inputs } = &corridor.source else {
        panic!("expected input list");
    };
    let mut neighbor_slots: Vec<_> = inputs.iter().map(|input| input.slot).collect();
    neighbor_slots.sort_unstable();
    assert_eq!(neighbor_slots, vec![0, 2, 3]);

    let hub = plan
        .ops
        .iter()
        .find(|op| op.targets[0].0 == 0)
        .expect("hub");
    let SourceSpec::ConjunctiveCrossing { inputs } = &hub.source else {
        panic!("expected input list");
    };
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].slot, 1);
}

#[test]
fn driver_compiled_terran_pirate_plan_uses_sum_over_input_list() {
    let plan = skeleton_plan();
    for op in &plan.ops {
        assert_eq!(op.combine, CombineFn::Sum);
        assert!(matches!(op.source, SourceSpec::ConjunctiveCrossing { .. }));
        assert_eq!(op.consume, ConsumeMode::AddToTarget);
    }
}

#[test]
fn driver_compiled_terran_pirate_plan_is_ao_wgsl0_compatible_or_reports_exact_reason() {
    let plan = skeleton_plan();
    let gpu_ops: Vec<AccumulatorOpGpu> = plan
        .ops
        .iter()
        .map(|op| AccumulatorOpGpu::from_op(op).expect("encode"))
        .collect();
    match classify_ao_wgsl0_plan(&gpu_ops) {
        AoWgsl0Compatibility::Compatible(_) => {}
        AoWgsl0Compatibility::Fallback(reason) => {
            panic!("skeleton plan must be AO-WGSL-0 compatible, got fallback: {reason:?}");
        }
    }
}

#[test]
fn driver_compile_terran_pirate_skeleton_does_not_use_studio_or_bevy_state() {
    let source = include_str!("../src/structural_link_accumulator_compile.rs");
    assert!(!source.contains("bevy"));
    assert!(!source.contains("mapeditor"));
    assert!(!source.contains("Studio"));
}
