//! SEMANTIC-LOCAL-EFFECT-TYPES-0 — typed semantic local effect driver proofs.

mod disburse_down_fixture;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_core::SimThingKind;
use simthing_driver::{
    compile_semantic_local_effects_plan, semantic_local_effects_cpu_runtime_applied_total,
    semantic_local_effects_cpu_shortfall_total,
    semantic_local_effects_runtime_applied_aggregate_slot,
    semantic_local_effects_runtime_applied_tick_inputs,
    semantic_local_effects_shortfall_aggregate_slot, semantic_local_effects_shortfall_tick_inputs,
};
use simthing_gpu::set_debug_readback_allowed;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy,
};
use simthing_spec::{
    serialize_scenario_authority, RuntimeTickId, SemanticLocalEffectKind, SpecError,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_semantic_local_effects.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "simthing-driver semantic-local-effects lock");
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
fn semantic_local_effects_compile_composes_application_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert_eq!(
        plan.local_effect_application_plan
            .application_report
            .application_count,
        3
    );
    assert_eq!(plan.semantic_report.output_count, 6);
    assert!(plan.authority_proof.scenario_authority_unchanged);
}

#[test]
fn semantic_local_effects_compile_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    let report = &plan.semantic_report;

    assert_eq!(report.runtime_applied_total, 72);
    assert_eq!(report.shortfall_total, 8);
    assert_eq!(report.satisfied_output_count, 2);
    assert_eq!(report.shortfall_output_count, 1);
}

#[test]
fn semantic_local_effects_compile_preserves_source_simthing_ids() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan
        .semantic_report
        .outputs
        .iter()
        .all(|o| o.source_simthing_id_raw > 0));
}

#[test]
fn semantic_local_effects_compile_records_satisfied_shortfall_counts() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    let satisfied = plan
        .semantic_report
        .outputs
        .iter()
        .filter(|o| o.effect_kind == SemanticLocalEffectKind::ResourceSatisfied)
        .count();
    let shortfall = plan
        .semantic_report
        .outputs
        .iter()
        .filter(|o| o.effect_kind == SemanticLocalEffectKind::ResourceShortfall)
        .count();
    assert_eq!(
        satisfied as u32,
        plan.semantic_report.satisfied_output_count
    );
    assert_eq!(
        shortfall as u32,
        plan.semantic_report.shortfall_output_count
    );
}

#[test]
fn semantic_local_effects_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(plan.authority_proof.scenario_authority_unchanged);
}

#[test]
fn semantic_local_effects_semantic_execution_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan.semantic_execution_deferred);
    assert!(plan.semantic_report.semantic_execution_deferred);
    assert!(plan.authority_proof.semantic_execution_deferred);
}

#[test]
fn semantic_local_effects_participant_property_mutation_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan.participant_property_mutation_deferred);
    assert!(plan
        .semantic_report
        .outputs
        .iter()
        .all(|o| o.participant_property_mutation_deferred));
}

#[test]
fn semantic_local_effects_savefile_mutation_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan.savefile_mutation_deferred);
    assert!(plan.semantic_report.savefile_mutation_deferred);
    assert!(plan.authority_proof.savefile_mutation_deferred);
}

#[test]
fn semantic_local_effects_reuses_existing_accumulator_surfaces() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    for proof in &plan.gpu_semantic_aggregate_proof_plans {
        assert_eq!(proof.runtime_applied_plan.ops.len(), 1);
        assert_eq!(proof.shortfall_plan.ops.len(), 1);
    }
}

#[test]
fn semantic_local_effects_gpu_aggregate_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_semantic_local_effects_gpu_aggregate_proof());
}

fn run_semantic_local_effects_gpu_aggregate_proof() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("SEMANTIC-LOCAL-EFFECT-TYPES-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };

    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    for proof_plan in &plan.gpu_semantic_aggregate_proof_plans {
        let cpu_applied = semantic_local_effects_cpu_runtime_applied_total(&plan, proof_plan);
        let cpu_shortfall = semantic_local_effects_cpu_shortfall_total(&plan, proof_plan);
        let applied_aggregate =
            semantic_local_effects_runtime_applied_aggregate_slot(&plan, proof_plan);
        let shortfall_aggregate =
            semantic_local_effects_shortfall_aggregate_slot(&plan, proof_plan);

        let applied_inputs = semantic_local_effects_runtime_applied_tick_inputs(&plan, proof_plan);
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

        assert_eq!(
            cpu_applied_tick[applied_aggregate],
            gpu_applied_tick[applied_aggregate]
        );
        assert_eq!(cpu_applied_tick[applied_aggregate], cpu_applied as f32);

        let shortfall_inputs = semantic_local_effects_shortfall_tick_inputs(&plan, proof_plan);
        let cpu_shortfall_tick =
            execute_accumulator_plan_tick_cpu(&proof_plan.shortfall_plan, &shortfall_inputs)
                .expect("cpu shortfall");
        let mut gpu_shortfall_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.shortfall_plan.clone())
                .expect("init shortfall");
        let gpu_shortfall_tick = gpu_shortfall_state
            .tick(&ctx, &shortfall_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu shortfall")
            .expect("readback shortfall");

        assert_eq!(
            cpu_shortfall_tick[shortfall_aggregate],
            gpu_shortfall_tick[shortfall_aggregate]
        );
        assert_eq!(
            cpu_shortfall_tick[shortfall_aggregate],
            cpu_shortfall as f32
        );
    }

    eprintln!("SEMANTIC-LOCAL-EFFECT-TYPES-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn semantic_local_effects_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("SEMANTIC-LOCAL-EFFECT-TYPES-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
}
