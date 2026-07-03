//! LOCAL-EFFECT-APPLICATION-AUTHORITY-0 — local effect application driver proofs.

mod disburse_down_fixture;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_core::SimThingKind;
use simthing_driver::{
    compile_local_effect_application_plan, local_effect_application_aggregate_slot,
    local_effect_application_cpu_runtime_applied_total, local_effect_application_cpu_unmet_total,
    local_effect_application_runtime_applied_tick_inputs,
    local_effect_application_unmet_tick_inputs,
};
use simthing_gpu::set_debug_readback_allowed;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy,
};
use simthing_spec::{
    serialize_scenario_authority, RuntimeTickId, SpecError, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_local_effect_application.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "simthing-driver local-effect-application lock");
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
fn local_effect_application_compile_composes_history_and_effects_plans() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert_eq!(plan.tick_history_plan.tick_id, TICK_ONE);
    assert_eq!(
        plan.local_participant_effects_plan
            .effects_report
            .effect_count,
        3
    );
    assert_eq!(plan.application_report.application_count, 3);
    assert!(plan.authority_proof.scenario_authority_unchanged);
}

#[test]
fn local_effect_application_compile_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    let report = &plan.application_report;

    assert_eq!(report.application_count, 3);
    assert_eq!(report.requested_total, 80);
    assert_eq!(report.allocated_total, 72);
    assert_eq!(report.unmet_total, 8);
    assert_eq!(report.runtime_applied_total, 72);
    assert_eq!(report.satisfied_count, 2);
    assert_eq!(report.unsatisfied_count, 1);
}

#[test]
fn local_effect_application_compile_preserves_source_simthing_ids() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan
        .application_report
        .records
        .iter()
        .all(|r| r.source_simthing_id_raw > 0));
    let ids: std::collections::BTreeSet<_> = plan
        .application_report
        .records
        .iter()
        .map(|r| r.source_simthing_id_raw)
        .collect();
    assert_eq!(ids.len(), 3);
}

#[test]
fn local_effect_application_compile_records_satisfied_unsatisfied_counts() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    let satisfied = plan
        .application_report
        .records
        .iter()
        .filter(|r| r.satisfied)
        .count();
    let unsatisfied = plan
        .application_report
        .records
        .iter()
        .filter(|r| !r.satisfied)
        .count();
    assert_eq!(satisfied as u32, plan.application_report.satisfied_count);
    assert_eq!(
        unsatisfied as u32,
        plan.application_report.unsatisfied_count
    );
}

#[test]
fn local_effect_application_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(plan.authority_proof.scenario_authority_unchanged);
}

#[test]
fn local_effect_application_semantic_effect_execution_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan.semantic_effect_execution_deferred);
    assert!(plan.application_report.semantic_effect_execution_deferred);
    assert!(plan.authority_proof.semantic_effect_execution_deferred);
}

#[test]
fn local_effect_application_participant_property_mutation_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan.participant_property_mutation_deferred);
    assert!(plan
        .application_report
        .records
        .iter()
        .all(|r| r.participant_property_mutation_deferred));
}

#[test]
fn local_effect_application_savefile_mutation_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan.savefile_mutation_deferred);
    assert!(plan.application_report.savefile_mutation_deferred);
    assert!(plan.authority_proof.savefile_mutation_deferred);
}

#[test]
fn local_effect_application_reuses_existing_accumulator_surfaces() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    for proof in &plan.gpu_application_aggregate_proof_plans {
        assert_eq!(proof.runtime_applied_plan.ops.len(), 1);
        assert_eq!(proof.unmet_plan.ops.len(), 1);
    }
}

#[test]
fn local_effect_application_gpu_aggregate_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_local_effect_application_gpu_aggregate_proof());
}

fn run_local_effect_application_gpu_aggregate_proof() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("LOCAL-EFFECT-APPLICATION-AUTHORITY-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };

    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    for proof_plan in &plan.gpu_application_aggregate_proof_plans {
        let cpu_applied = local_effect_application_cpu_runtime_applied_total(&plan, proof_plan);
        let cpu_unmet = local_effect_application_cpu_unmet_total(&plan, proof_plan);
        let aggregate = local_effect_application_aggregate_slot(proof_plan);

        let applied_inputs =
            local_effect_application_runtime_applied_tick_inputs(&plan, proof_plan);
        let cpu_applied_tick =
            execute_accumulator_plan_tick_cpu(&proof_plan.runtime_applied_plan, &applied_inputs)
                .expect("cpu applied");
        let mut gpu_applied_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.runtime_applied_plan.clone())
                .expect("init applied");
        let gpu_applied_tick = gpu_applied_state
            .tick(&ctx, &applied_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu applied")
            .expect("readback applied");

        assert_eq!(cpu_applied_tick[aggregate], gpu_applied_tick[aggregate]);
        assert_eq!(cpu_applied_tick[aggregate], cpu_applied as f32);

        let unmet_inputs = local_effect_application_unmet_tick_inputs(&plan, proof_plan);
        let cpu_unmet_tick =
            execute_accumulator_plan_tick_cpu(&proof_plan.unmet_plan, &unmet_inputs)
                .expect("cpu unmet");
        let mut gpu_unmet_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.unmet_plan.clone())
                .expect("init unmet");
        let gpu_unmet_tick = gpu_unmet_state
            .tick(&ctx, &unmet_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu unmet")
            .expect("readback unmet");

        assert_eq!(cpu_unmet_tick[aggregate], gpu_unmet_tick[aggregate]);
        assert_eq!(cpu_unmet_tick[aggregate], cpu_unmet as f32);
    }

    eprintln!("LOCAL-EFFECT-APPLICATION-AUTHORITY-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn local_effect_application_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("LOCAL-EFFECT-APPLICATION-AUTHORITY-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
}
