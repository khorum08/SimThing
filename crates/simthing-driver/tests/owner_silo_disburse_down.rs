//! OWNER-SILO-DISBURSE-DOWN-0 — disburse-down driver compile and GPU demand aggregate proof.

mod disburse_down_fixture;
mod reduce_up_fixture;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_core::SimThingKind;
use simthing_driver::{
    compile_owner_silo_disburse_down_plan, owner_silo_disburse_down_cpu_demand_aggregate_total,
    owner_silo_disburse_down_demand_aggregate_slot,
    owner_silo_disburse_down_demand_aggregate_tick_inputs,
};
use simthing_gpu::set_debug_readback_allowed;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy,
};
use simthing_spec::{
    serialize_scenario_authority, SpecError, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec;

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_owner_silo_disburse_down.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "simthing-driver owner-silo-disburse-down lock");
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
#[test]
fn owner_silo_disburse_down_compile_preserves_owner_channels() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_disburse_down_plan(&spec).expect("compile");
    assert_eq!(plan.demand_buckets.len(), 3);
    assert_eq!(plan.cpu_results.len(), 2);
    assert!(plan
        .demand_buckets
        .iter()
        .all(|b| b.resource_key == simthing_spec::PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn owner_silo_disburse_down_cpu_allocation_matches_expected_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_disburse_down_plan(&spec).expect("compile");

    let owner_a = plan
        .cpu_results
        .iter()
        .find(|r| r.owner_ref == "owner_a")
        .expect("owner_a");
    assert_eq!(owner_a.available_before, 62);
    assert_eq!(owner_a.remaining_after, 0);
    assert_eq!(owner_a.allocated_total, 62);
    assert_eq!(owner_a.unmet_total, 8);

    let cohort = owner_a
        .allocations
        .iter()
        .find(|a| a.requested == 20)
        .expect("cohort");
    assert_eq!(cohort.allocated, 20);
    assert_eq!(cohort.unmet, 0);

    let fleet = owner_a
        .allocations
        .iter()
        .find(|a| a.requested == 50)
        .expect("fleet");
    assert_eq!(fleet.allocated, 42);
    assert_eq!(fleet.unmet, 8);

    let owner_b = plan
        .cpu_results
        .iter()
        .find(|r| r.owner_ref == "owner_b")
        .expect("owner_b");
    assert_eq!(owner_b.available_before, 45);
    assert_eq!(owner_b.remaining_after, 35);
    assert_eq!(owner_b.allocated_total, 10);
    assert_eq!(owner_b.unmet_total, 0);
}

#[test]
fn owner_silo_disburse_down_cpu_records_remaining_and_unmet() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_disburse_down_plan(&spec).expect("compile");
    for result in &plan.cpu_results {
        assert_eq!(
            result.remaining_after,
            result
                .available_before
                .saturating_sub(result.allocated_total)
        );
        let sum_unmet: u32 = result.allocations.iter().map(|a| a.unmet).sum();
        assert_eq!(result.unmet_total, sum_unmet);
    }
}

#[test]
fn owner_silo_disburse_down_scenario_authority_unchanged() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _plan = compile_owner_silo_disburse_down_plan(&spec).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn owner_silo_disburse_down_allocation_application_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_disburse_down_plan(&spec).expect("compile");
    assert!(plan.allocation_application_deferred);
    assert!(plan.scenario_authority_mutation_deferred);
}

#[test]
fn owner_silo_disburse_down_empty_demands_zero_plan() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_owner_silo_disburse_down_plan(&spec).expect("compile");
    assert!(plan.demand_buckets.is_empty());
    assert!(plan.cpu_results.is_empty());
    assert!(plan.gpu_demand_aggregate_proof_plans.is_empty());
}

#[test]
fn owner_silo_disburse_down_gpu_demand_aggregate_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_owner_silo_disburse_down_gpu_demand_aggregate_proof());
}

fn run_owner_silo_disburse_down_gpu_demand_aggregate_proof() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("OWNER-SILO-DISBURSE-DOWN-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };

    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_disburse_down_plan(&spec).expect("compile");

    for proof_plan in &plan.gpu_demand_aggregate_proof_plans {
        let cpu_total = owner_silo_disburse_down_cpu_demand_aggregate_total(&plan, proof_plan);
        let aggregate = owner_silo_disburse_down_demand_aggregate_slot(proof_plan);
        let inputs = owner_silo_disburse_down_demand_aggregate_tick_inputs(&plan, proof_plan);

        let cpu_tick =
            execute_accumulator_plan_tick_cpu(&proof_plan.demand_plan, &inputs).expect("cpu");
        let mut gpu_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.demand_plan.clone()).expect("init");
        let gpu_tick = gpu_state
            .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu")
            .expect("readback");

        assert_eq!(cpu_tick[aggregate], gpu_tick[aggregate]);
        assert_eq!(cpu_tick[aggregate], cpu_total as f32);
    }

    eprintln!("OWNER-SILO-DISBURSE-DOWN-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn owner_silo_disburse_down_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("OWNER-SILO-DISBURSE-DOWN-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
}

#[test]
fn owner_silo_disburse_down_uses_existing_accumulator_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_disburse_down_plan(&spec).expect("compile");
    for proof in &plan.gpu_demand_aggregate_proof_plans {
        assert_eq!(proof.demand_plan.ops.len(), 1);
    }
}
