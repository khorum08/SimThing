//! RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 — runtime allocation application driver proof.

mod disburse_down_fixture;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_core::SimThingKind;
use simthing_driver::{
    compile_runtime_local_allocation_application_plan, runtime_local_allocation_aggregate_slot,
    runtime_local_allocation_aggregate_tick_inputs, runtime_local_allocation_cpu_aggregate_total,
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

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_runtime_local_allocation.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "simthing-driver runtime-local-allocation lock");
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
fn runtime_local_allocation_compile_application_matches_expected_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_local_allocation_application_plan(&spec).expect("compile");
    let report = &plan.application_report;

    assert_eq!(report.allocation_count, 3);
    assert_eq!(report.allocated_total, 72);
    assert_eq!(report.unmet_total, 8);

    let owner_a: Vec<_> = report
        .states
        .iter()
        .filter(|s| s.owner_ref == "owner_a")
        .collect();
    let cohort = owner_a.iter().find(|s| s.requested == 20).expect("cohort");
    assert_eq!(cohort.allocated, 20);
    assert_eq!(cohort.unmet, 0);
    let fleet = owner_a.iter().find(|s| s.requested == 50).expect("fleet");
    assert_eq!(fleet.allocated, 42);
    assert_eq!(fleet.unmet, 8);

    let owner_b = report
        .states
        .iter()
        .find(|s| s.owner_ref == "owner_b")
        .expect("owner_b");
    assert_eq!(owner_b.allocated, 10);
    assert_eq!(owner_b.unmet, 0);
}

#[test]
fn runtime_local_allocation_compile_preserves_owner_channels() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_local_allocation_application_plan(&spec).expect("compile");
    assert_eq!(plan.application_report.owner_channel_count, 2);
}

#[test]
fn runtime_local_allocation_compile_preserves_source_simthing_ids() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_local_allocation_application_plan(&spec).expect("compile");
    assert!(plan
        .application_report
        .states
        .iter()
        .all(|s| s.source_simthing_id_raw > 0));
    let ids: std::collections::BTreeSet<_> = plan
        .application_report
        .states
        .iter()
        .map(|s| s.source_simthing_id_raw)
        .collect();
    assert_eq!(ids.len(), 3);
}

#[test]
fn runtime_local_allocation_scenario_authority_unchanged() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _plan = compile_runtime_local_allocation_application_plan(&spec).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn runtime_local_allocation_economy_execution_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_local_allocation_application_plan(&spec).expect("compile");
    assert!(plan.economy_execution_deferred);
    assert!(plan.scenario_authority_mutation_deferred);
    assert!(plan.application_report.economy_execution_deferred);
}

#[test]
fn runtime_local_allocation_gpu_allocated_aggregate_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_runtime_local_allocation_gpu_aggregate_proof());
}

fn run_runtime_local_allocation_gpu_aggregate_proof() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("RUNTIME-LOCAL-ALLOCATION-APPLICATION-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };

    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_local_allocation_application_plan(&spec).expect("compile");

    for proof_plan in &plan.gpu_allocation_aggregate_proof_plans {
        let cpu_total = runtime_local_allocation_cpu_aggregate_total(&plan, proof_plan);
        let aggregate = runtime_local_allocation_aggregate_slot(proof_plan);
        let inputs = runtime_local_allocation_aggregate_tick_inputs(&plan, proof_plan);

        let cpu_tick =
            execute_accumulator_plan_tick_cpu(&proof_plan.allocation_plan, &inputs).expect("cpu");
        let mut gpu_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.allocation_plan.clone())
                .expect("init");
        let gpu_tick = gpu_state
            .tick(&ctx, &inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu")
            .expect("readback");

        assert_eq!(cpu_tick[aggregate], gpu_tick[aggregate]);
        assert_eq!(cpu_tick[aggregate], cpu_total as f32);

        let disburse_total = plan
            .disburse_down_plan
            .cpu_results
            .iter()
            .find(|r| {
                r.owner_ref == proof_plan.owner_ref && r.resource_key == proof_plan.resource_key
            })
            .map(|r| r.allocated_total)
            .expect("disburse owner/resource");
        assert_eq!(cpu_total, disburse_total);
    }

    eprintln!("RUNTIME-LOCAL-ALLOCATION-APPLICATION-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn runtime_local_allocation_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("RUNTIME-LOCAL-ALLOCATION-APPLICATION-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
}

#[test]
fn runtime_local_allocation_uses_existing_accumulator_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_local_allocation_application_plan(&spec).expect("compile");
    for proof in &plan.gpu_allocation_aggregate_proof_plans {
        assert_eq!(proof.allocation_plan.ops.len(), 1);
    }
}
