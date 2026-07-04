//! SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0 — owner-silo accumulator GPU tick integration proof.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_driver::{
    compile_owner_silo_gpu_tick_plan, owner_silo_aggregate_slot, owner_silo_deficit_tick_inputs,
    owner_silo_participant_deficit_total, owner_silo_participant_surplus_total,
    owner_silo_surplus_tick_inputs,
};
use simthing_gpu::{debug_readback_allowed, set_debug_readback_allowed};
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy,
};
use simthing_spec::{
    deserialize_scenario_authority, evaluate_owner_silo_flow, owner_silo_flow_participant_inputs,
    OwnerSiloAdmissionClassification, OwnerSiloAdmissionErrorKind, SimThingScenarioSpec, SpecError,
};

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_owner_silo_gpu_readback_test.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "simthing-driver owner-silo readback integration lock");
                    return Self { path };
                }
                Err(_) => thread::sleep(Duration::from_millis(25)),
            }
        }
    }
}

impl Drop for ProcessReadbackTestLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
        set_debug_readback_allowed(false);
    }
}

fn with_isolated_readback_gate_test<F: FnOnce()>(f: F) {
    let _lock = ProcessReadbackTestLock::acquire();
    set_debug_readback_allowed(false);
    f();
    set_debug_readback_allowed(false);
}

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_corpus(name: &str) -> SimThingScenarioSpec {
    let json = fs::read_to_string(corpus_path(name)).expect("corpus");
    deserialize_scenario_authority(&json).expect("parse")
}

#[test]
fn owner_silo_cpu_oracle_matches_accumulator_inputs() {
    let scenario = load_corpus("owner_silo_balanced_flow.simthing-scenario.json");
    let plan = compile_owner_silo_gpu_tick_plan(&scenario).expect("compile");
    let admission = evaluate_owner_silo_flow(&scenario);

    assert_eq!(owner_silo_participant_surplus_total(&plan), 30);
    assert_eq!(owner_silo_participant_deficit_total(&plan), 20);
    assert_eq!(admission.reducible_surplus_total, 30.0);
    assert_eq!(admission.resolvable_deficit_total, 20.0);
    assert_eq!(admission.unresolved_deficit_total, 0.0);

    let surplus_inputs = owner_silo_surplus_tick_inputs(&plan);
    let deficit_inputs = owner_silo_deficit_tick_inputs(&plan);
    let surplus_cpu = execute_accumulator_plan_tick_cpu(&plan.surplus_plan, &surplus_inputs)
        .expect("surplus cpu");
    let deficit_cpu = execute_accumulator_plan_tick_cpu(&plan.deficit_plan, &deficit_inputs)
        .expect("deficit cpu");
    let aggregate = owner_silo_aggregate_slot(&plan);
    assert_eq!(surplus_cpu[aggregate], 30.0);
    assert_eq!(deficit_cpu[aggregate], 20.0);
}

#[test]
fn owner_silo_gpu_tick_matches_cpu_oracle_on_real_adapter_or_records_skip() {
    with_isolated_readback_gate_test(|| run_owner_silo_gpu_tick_matches_cpu_oracle());
}

fn run_owner_silo_gpu_tick_matches_cpu_oracle() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let scenario = load_corpus("owner_silo_balanced_flow.simthing-scenario.json");
    let plan = compile_owner_silo_gpu_tick_plan(&scenario).expect("compile");
    let aggregate = owner_silo_aggregate_slot(&plan);

    let surplus_inputs = owner_silo_surplus_tick_inputs(&plan);
    let surplus_cpu = execute_accumulator_plan_tick_cpu(&plan.surplus_plan, &surplus_inputs)
        .expect("surplus cpu");
    let mut surplus_state =
        SimGpuAccumulatorTickState::new(&ctx, plan.surplus_plan.clone()).expect("surplus init");
    let surplus_gpu = surplus_state
        .tick(&ctx, &surplus_inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("surplus gpu")
        .expect("surplus readback");
    assert_eq!(surplus_cpu[aggregate], surplus_gpu[aggregate]);

    let deficit_inputs = owner_silo_deficit_tick_inputs(&plan);
    let deficit_cpu = execute_accumulator_plan_tick_cpu(&plan.deficit_plan, &deficit_inputs)
        .expect("deficit cpu");
    let mut deficit_state =
        SimGpuAccumulatorTickState::new(&ctx, plan.deficit_plan.clone()).expect("deficit init");
    let deficit_gpu = deficit_state
        .tick(&ctx, &deficit_inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("deficit gpu")
        .expect("deficit readback");
    assert_eq!(deficit_cpu[aggregate], deficit_gpu[aggregate]);
    eprintln!("SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0: REAL_ADAPTER_OBSERVED");
}

fn run_owner_silo_gpu_tick_none_policy_no_readback() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let scenario = load_corpus("owner_silo_balanced_flow.simthing-scenario.json");
    let plan = compile_owner_silo_gpu_tick_plan(&scenario).expect("compile");
    let surplus_inputs = owner_silo_surplus_tick_inputs(&plan);
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan.surplus_plan.clone()).expect("init");
    assert!(state
        .tick(&ctx, &surplus_inputs, SimGpuReadbackPolicy::None)
        .expect("none tick")
        .is_none());
    assert!(!debug_readback_allowed());
    eprintln!("SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0: REAL_ADAPTER_OBSERVED");
}

fn run_owner_silo_gpu_tick_proof_readback_scoped() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let scenario = load_corpus("owner_silo_balanced_flow.simthing-scenario.json");
    let plan = compile_owner_silo_gpu_tick_plan(&scenario).expect("compile");
    let inputs = owner_silo_surplus_tick_inputs(&plan);
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan.surplus_plan.clone()).expect("init");
    assert!(!debug_readback_allowed());
    state
        .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("proof tick");
    assert!(!debug_readback_allowed());
    eprintln!("SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0: REAL_ADAPTER_OBSERVED");
}

fn run_owner_silo_gpu_tick_readback_gate_restored() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let scenario = load_corpus("owner_silo_balanced_flow.simthing-scenario.json");
    let plan = compile_owner_silo_gpu_tick_plan(&scenario).expect("compile");
    let inputs = owner_silo_surplus_tick_inputs(&plan);
    let mut state = SimGpuAccumulatorTickState::new(&ctx, plan.surplus_plan.clone()).expect("init");
    state
        .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("proof tick");
    assert!(!debug_readback_allowed());
    assert!(state
        .tick(&ctx, &inputs, SimGpuReadbackPolicy::None)
        .expect("none tick")
        .is_none());
    assert!(!debug_readback_allowed());
    eprintln!("SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0: REAL_ADAPTER_OBSERVED");
}
