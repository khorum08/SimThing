//! TERRAN-PIRATE-SCENARIO-SKELETON-0R — sim resident GPU tick proofs load canonical scenario authority.

use std::sync::Mutex;

use simthing_core::StructuralScalarChannel;
use simthing_driver::compile_structural_link_neighbor_sum_plan;
use simthing_gpu::{debug_readback_allowed, set_debug_readback_allowed};
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy, SimTickError,
};
use simthing_spec::{
    deserialize_scenario_authority, validate_scenario_links, validate_stead_mapping_consistency,
    SimThingScenarioSpec,
};

const TERRAN_PIRATE_SKELETON_SCENARIO_JSON: &str =
    include_str!("../../../scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json");

static READBACK_GATE_TEST_LOCK: Mutex<()> = Mutex::new(());

fn with_isolated_readback_gate_test<F: FnOnce()>(f: F) {
    let _lock = READBACK_GATE_TEST_LOCK
        .lock()
        .expect("readback gate test lock");
    set_debug_readback_allowed(false);
    f();
    set_debug_readback_allowed(false);
}

fn canonical_skeleton_scenario() -> SimThingScenarioSpec {
    let scenario = deserialize_scenario_authority(TERRAN_PIRATE_SKELETON_SCENARIO_JSON)
        .expect("deserialize canonical skeleton");
    validate_stead_mapping_consistency(&scenario).expect("STEAD valid");
    validate_scenario_links(&scenario).expect("links valid");
    scenario
}

/// Exact f32 integer inputs in dense placement order (hub, corridor, choke, branch).
fn terran_pirate_skeleton_dense_inputs() -> Vec<f32> {
    vec![10.0, 20.0, 40.0, 30.0]
}

fn skeleton_plan() -> simthing_core::CompiledAccumulatorOpPlan {
    compile_structural_link_neighbor_sum_plan(
        &canonical_skeleton_scenario(),
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("compile")
}

#[test]
fn sim_cpu_tick_executes_terran_pirate_skeleton_plan() {
    let plan = skeleton_plan();
    let inputs = terran_pirate_skeleton_dense_inputs();
    let output = execute_accumulator_plan_tick_cpu(&plan, &inputs).expect("cpu tick");
    assert_eq!(output.len(), 4);
}

#[test]
fn sim_gpu_resident_tick_executes_terran_pirate_skeleton_plan() {
    with_isolated_readback_gate_test(|| run_gpu_resident_tick_executes_skeleton());
}

fn run_gpu_resident_tick_executes_skeleton() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("TERRAN-PIRATE-SCENARIO-SKELETON-0R: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = skeleton_plan();
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let output = state
        .tick(
            &ctx,
            &terran_pirate_skeleton_dense_inputs(),
            SimGpuReadbackPolicy::ProofReadback,
        )
        .expect("tick")
        .expect("readback");
    assert_eq!(output.len(), 4);
    eprintln!("TERRAN-PIRATE-SCENARIO-SKELETON-0R: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_resident_tick_matches_cpu_tick_for_terran_pirate_skeleton() {
    with_isolated_readback_gate_test(|| run_gpu_resident_tick_matches_cpu());
}

fn run_gpu_resident_tick_matches_cpu() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("TERRAN-PIRATE-SCENARIO-SKELETON-0R: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let plan = skeleton_plan();
    let inputs = terran_pirate_skeleton_dense_inputs();
    let cpu = execute_accumulator_plan_tick_cpu(&plan, &inputs).expect("cpu");
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
    let gpu = state
        .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("gpu tick")
        .expect("readback");
    assert_eq!(cpu, gpu);
    assert_eq!(cpu, vec![20.0, 80.0, 20.0, 20.0]);
    eprintln!("TERRAN-PIRATE-SCENARIO-SKELETON-0R: REAL_ADAPTER_OBSERVED");
}

#[test]
fn sim_gpu_resident_tick_does_not_mutate_scenario_authority() {
    let scenario = canonical_skeleton_scenario();
    let links_before = scenario.links.clone();
    let placements_before = scenario.structural_grid.placements.clone();
    let plan = compile_structural_link_neighbor_sum_plan(
        &scenario,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("compile");
    let _ = execute_accumulator_plan_tick_cpu(&plan, &terran_pirate_skeleton_dense_inputs())
        .expect("cpu tick");
    assert_eq!(scenario.links, links_before);
    assert_eq!(scenario.structural_grid.placements, placements_before);
}

#[test]
fn sim_gpu_resident_tick_rejects_wrong_input_len_for_terran_pirate_skeleton() {
    let plan = skeleton_plan();
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
fn sim_gpu_resident_tick_rejects_non_exact_integer_input_for_terran_pirate_skeleton() {
    let plan = skeleton_plan();
    let err =
        execute_accumulator_plan_tick_cpu(&plan, &[10.0, 20.0, 30.5, 40.0]).expect_err("non-exact");
    assert!(matches!(err, SimTickError::NonExactIntegerInput { .. }));
}

#[test]
fn terran_pirate_skeleton_proof_readback_does_not_leak_into_none_tick() {
    with_isolated_readback_gate_test(|| {
        let Some(ctx) = gpu_context_blocking().ok() else {
            eprintln!("TERRAN-PIRATE-SCENARIO-SKELETON-0R: GPU_TESTS_SKIPPED_NO_ADAPTER");
            return;
        };
        let plan = skeleton_plan();
        let mut state = SimGpuAccumulatorTickState::new(&ctx, plan).expect("init");
        let inputs = terran_pirate_skeleton_dense_inputs();
        state
            .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("proof tick");
        assert!(!debug_readback_allowed());
        assert!(state
            .tick(&ctx, &inputs, SimGpuReadbackPolicy::None)
            .expect("none tick")
            .is_none());
        assert!(!debug_readback_allowed());
        eprintln!("TERRAN-PIRATE-SCENARIO-SKELETON-0R: REAL_ADAPTER_OBSERVED");
    });
}
