//! LOCAL-PARTICIPANT-EFFECTS-0 — local participant effects driver proofs.

mod disburse_down_fixture;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_core::SimThingKind;
use simthing_driver::{
    compile_local_participant_effects_plan, local_participant_effects_aggregate_slot,
    local_participant_effects_allocated_tick_inputs, local_participant_effects_cpu_allocated_total,
    local_participant_effects_cpu_unmet_total, local_participant_effects_unmet_tick_inputs,
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

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_local_participant_effects.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "simthing-driver local-participant-effects lock");
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
fn local_participant_effects_compile_composes_runtime_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");

    assert_eq!(plan.tick_shell_plan.tick_id, TICK_ONE);
    assert_eq!(plan.effects_report.effect_count, 3);
    assert!(plan.tick_shell_plan.execution_report.runtime_rf_tick_ready);
}

#[test]
fn local_participant_effects_compile_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");
    let report = &plan.effects_report;

    assert_eq!(report.effect_count, 3);
    assert_eq!(report.allocated_total, 72);
    assert_eq!(report.unmet_total, 8);
    assert_eq!(report.satisfied_count, 2);
    assert_eq!(report.unsatisfied_count, 1);
}

#[test]
fn local_participant_effects_compile_preserves_source_simthing_ids() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");

    assert!(plan
        .effects_report
        .effects
        .iter()
        .all(|e| e.source_simthing_id_raw > 0));
    let ids: std::collections::BTreeSet<_> = plan
        .effects_report
        .effects
        .iter()
        .map(|e| e.source_simthing_id_raw)
        .collect();
    assert_eq!(ids.len(), 3);
}

#[test]
fn local_participant_effects_compile_records_satisfied_unsatisfied_counts() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");

    let satisfied = plan
        .effects_report
        .effects
        .iter()
        .filter(|e| e.satisfied)
        .count();
    let unsatisfied = plan
        .effects_report
        .effects
        .iter()
        .filter(|e| !e.satisfied)
        .count();
    assert_eq!(satisfied as u32, plan.effects_report.satisfied_count);
    assert_eq!(unsatisfied as u32, plan.effects_report.unsatisfied_count);
}

#[test]
fn local_participant_effects_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn local_participant_effects_economy_execution_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");

    assert!(plan.economy_execution_deferred);
    assert!(plan.effects_report.economy_execution_deferred);
}

#[test]
fn local_participant_effects_participant_property_mutation_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");

    assert!(plan.participant_property_mutation_deferred);
    assert!(plan.effects_report.participant_property_mutation_deferred);
    assert!(plan
        .effects_report
        .effects
        .iter()
        .all(|e| e.effect_application_deferred));
}

#[test]
fn local_participant_effects_reuses_existing_accumulator_surfaces() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");

    for proof in &plan.gpu_effect_aggregate_proof_plans {
        assert_eq!(proof.allocated_plan.ops.len(), 1);
        assert_eq!(proof.unmet_plan.ops.len(), 1);
    }
}

#[test]
fn local_participant_effects_gpu_aggregate_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_local_participant_effects_gpu_aggregate_proof());
}

fn run_local_participant_effects_gpu_aggregate_proof() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("LOCAL-PARTICIPANT-EFFECTS-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };

    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_participant_effects_plan(&spec, TICK_ONE).expect("compile");

    for proof_plan in &plan.gpu_effect_aggregate_proof_plans {
        let cpu_allocated = local_participant_effects_cpu_allocated_total(&plan, proof_plan);
        let cpu_unmet = local_participant_effects_cpu_unmet_total(&plan, proof_plan);
        let aggregate = local_participant_effects_aggregate_slot(proof_plan);

        let alloc_inputs = local_participant_effects_allocated_tick_inputs(&plan, proof_plan);
        let cpu_alloc_tick =
            execute_accumulator_plan_tick_cpu(&proof_plan.allocated_plan, &alloc_inputs)
                .expect("cpu alloc");
        let mut gpu_alloc_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.allocated_plan.clone())
                .expect("init alloc");
        let gpu_alloc_tick = gpu_alloc_state
            .tick(&ctx, &alloc_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu alloc")
            .expect("readback alloc");

        assert_eq!(cpu_alloc_tick[aggregate], gpu_alloc_tick[aggregate]);
        assert_eq!(cpu_alloc_tick[aggregate], cpu_allocated as f32);

        let unmet_inputs = local_participant_effects_unmet_tick_inputs(&plan, proof_plan);
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

    eprintln!("LOCAL-PARTICIPANT-EFFECTS-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn local_participant_effects_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("LOCAL-PARTICIPANT-EFFECTS-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
}

#[test]
fn local_participant_effects_compile_rejects_invalid_earlier_stage() {
    let mut spec = build_owner_silo_disburse_down_scoped_spec();
    let star = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .unwrap();
    let planet = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap();
    planet.properties.remove(&PLANET_ID_PROPERTY_ID);

    let err = compile_local_participant_effects_plan(&spec, TICK_ONE).unwrap_err();
    assert!(matches!(err, SpecError::ValidationFailed));
}
