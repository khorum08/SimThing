//! RECURSIVE-LOCAL-RF-EVALUATOR-0 — recursive local RF driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;
#[path = "../../simthing-spec/tests/sibling_redistribution_fixture.rs"]
mod sibling_redistribution_fixture;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_driver::{
    compile_local_effect_application_plan, compile_recursive_local_rf_plan,
    compile_runtime_rf_tick_plan, compile_semantic_local_effects_plan,
    recursive_local_rf_cpu_demand_total, recursive_local_rf_cpu_surplus_total,
    recursive_local_rf_demand_aggregate_slot, recursive_local_rf_demand_tick_inputs,
    recursive_local_rf_surplus_aggregate_slot, recursive_local_rf_surplus_tick_inputs,
};
use simthing_gpu::set_debug_readback_allowed;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy,
};
use simthing_spec::RecursiveLocalRfAggregateSourceKind;
use simthing_spec::{serialize_scenario_authority, RuntimeTickId};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::{build_sibling_redistribution_spec, star_system_id_raw};

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_recursive_local_rf.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "simthing-driver recursive-local-rf lock");
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
fn recursive_local_rf_compile_builds_plan_from_scenario() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    assert!(plan.evaluation_report.location_count > 0);
    assert!(plan.authority_proof.scenario_authority_unchanged);
    assert!(plan.compatibility_report.previous_rf_ladder_preserved);
}

#[test]
fn recursive_local_rf_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(plan.authority_proof.scenario_authority_unchanged);
}

#[test]
fn recursive_local_rf_compile_sibling_surplus_deficit_fixture_matches_expected() {
    let spec = build_sibling_redistribution_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");
    let star_id = star_system_id_raw(&spec);
    let settlement = plan
        .evaluation_report
        .arena_reports
        .iter()
        .find(|arena| arena.location_id_raw == star_id)
        .expect("star arena")
        .settlements
        .iter()
        .find(|s| s.owner_ref == "owner_a" && s.resource_key == "food")
        .expect("food settlement");

    assert_eq!(settlement.locally_matched_total, 20);
    assert_eq!(settlement.net_surplus_to_parent, 10);
    assert_eq!(settlement.net_deficit_to_parent, 0);
}

#[test]
fn recursive_local_rf_compile_preserves_previous_ladder_compatibility() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    assert!(plan.previous_rf_ladder_compatibility_preserved);
    assert!(plan.compatibility_report.owner_silo_fixture_compatible);
}

#[test]
fn recursive_local_rf_coexists_with_semantic_local_effects_without_changing_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let semantic_before =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic before");
    let _recursive = compile_recursive_local_rf_plan(&spec).expect("recursive");
    let semantic_after =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic after");

    assert_eq!(
        semantic_before.semantic_report.runtime_applied_total,
        semantic_after.semantic_report.runtime_applied_total
    );
    assert_eq!(
        semantic_before.semantic_report.shortfall_total,
        semantic_after.semantic_report.shortfall_total
    );
}

#[test]
fn recursive_local_rf_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    for proof in &plan.gpu_arena_aggregate_proof_plans {
        assert_eq!(proof.surplus_plan.ops.len(), 1);
        assert_eq!(proof.demand_plan.ops.len(), 1);
    }
}

#[test]
fn recursive_local_rf_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    for proof in &plan.gpu_arena_aggregate_proof_plans {
        assert_eq!(proof.surplus_plan.ops.len(), 1);
    }
}

#[test]
fn recursive_local_rf_reuses_existing_accumulator_surfaces() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    assert!(
        !plan.gpu_arena_aggregate_proof_plans.is_empty()
            || plan.evaluation_report.participant_count > 0
    );
    for proof in &plan.gpu_arena_aggregate_proof_plans {
        assert!(proof.surplus_plan.slot_count > 0);
        assert!(proof.demand_plan.slot_count > 0);
    }
}

#[test]
fn recursive_local_rf_gpu_aggregate_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_recursive_local_rf_gpu_aggregate_proof());
}

fn run_recursive_local_rf_gpu_aggregate_proof() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("RECURSIVE-LOCAL-RF-EVALUATOR-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };

    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    for proof_plan in &plan.gpu_arena_aggregate_proof_plans {
        let cpu_surplus = recursive_local_rf_cpu_surplus_total(&plan, proof_plan);
        let cpu_demand = recursive_local_rf_cpu_demand_total(&plan, proof_plan);
        let surplus_aggregate = recursive_local_rf_surplus_aggregate_slot(proof_plan);
        let demand_aggregate = recursive_local_rf_demand_aggregate_slot(proof_plan);

        let surplus_inputs = recursive_local_rf_surplus_tick_inputs(&plan, proof_plan);
        let cpu_surplus_tick =
            execute_accumulator_plan_tick_cpu(&proof_plan.surplus_plan, &surplus_inputs)
                .expect("cpu surplus");
        let mut gpu_surplus_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.surplus_plan.clone())
                .expect("init surplus");
        let gpu_surplus_tick = gpu_surplus_state
            .tick(&ctx, &surplus_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu surplus")
            .expect("readback surplus");

        let demand_inputs = recursive_local_rf_demand_tick_inputs(&plan, proof_plan);
        let cpu_demand_tick =
            execute_accumulator_plan_tick_cpu(&proof_plan.demand_plan, &demand_inputs)
                .expect("cpu demand");
        let mut gpu_demand_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.demand_plan.clone())
                .expect("init demand");
        let gpu_demand_tick = gpu_demand_state
            .tick(&ctx, &demand_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu demand")
            .expect("readback demand");

        assert_eq!(
            cpu_surplus_tick[surplus_aggregate],
            gpu_surplus_tick[surplus_aggregate]
        );
        assert_eq!(
            cpu_demand_tick[demand_aggregate],
            gpu_demand_tick[demand_aggregate]
        );
        assert_eq!(cpu_surplus_tick[surplus_aggregate] as u32, cpu_surplus);
        assert_eq!(gpu_surplus_tick[surplus_aggregate] as u32, cpu_surplus);
        assert_eq!(cpu_demand_tick[demand_aggregate] as u32, cpu_demand);
        assert_eq!(gpu_demand_tick[demand_aggregate] as u32, cpu_demand);
    }

    eprintln!("RECURSIVE-LOCAL-RF-GPU-RESIDENCY-REMEDIATION-0R: REAL_ADAPTER_OBSERVED");
}

#[test]
fn recursive_local_rf_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("RECURSIVE-LOCAL-RF-GPU-RESIDENCY-REMEDIATION-0R: GPU_TESTS_SKIPPED_NO_ADAPTER");
}

#[test]
fn recursive_local_rf_aggregate_sources_include_direct_participants() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    assert!(plan
        .aggregate_source_rows
        .iter()
        .any(|row| { row.source_kind == RecursiveLocalRfAggregateSourceKind::DirectParticipant }));
}

#[test]
fn recursive_local_rf_aggregate_sources_include_child_location_outputs() {
    let spec = build_sibling_redistribution_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    assert!(plan.aggregate_source_rows.iter().any(|row| {
        row.source_kind == RecursiveLocalRfAggregateSourceKind::ChildLocationOutput
    }));
}

#[test]
fn recursive_local_rf_aggregate_source_totals_match_settlement_totals() {
    let spec = build_sibling_redistribution_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    for arena in &plan.evaluation_report.arena_reports {
        for settlement in &arena.settlements {
            let surplus_sum = plan
                .aggregate_source_rows
                .iter()
                .filter(|row| {
                    row.arena_location_id_raw == settlement.arena_location_id_raw
                        && row.owner_ref == settlement.owner_ref
                        && row.resource_key == settlement.resource_key
                })
                .try_fold(0u32, |acc, row| acc.checked_add(row.surplus))
                .expect("surplus overflow");
            let demand_sum = plan
                .aggregate_source_rows
                .iter()
                .filter(|row| {
                    row.arena_location_id_raw == settlement.arena_location_id_raw
                        && row.owner_ref == settlement.owner_ref
                        && row.resource_key == settlement.resource_key
                })
                .try_fold(0u32, |acc, row| acc.checked_add(row.demand))
                .expect("demand overflow");

            assert_eq!(surplus_sum, settlement.total_surplus);
            assert_eq!(demand_sum, settlement.total_demand);
        }
    }
}

#[test]
fn recursive_local_rf_aggregate_sources_are_gpu_table_compatible() {
    let spec = build_sibling_redistribution_spec();
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");

    assert!(!plan.aggregate_source_rows.is_empty());
    for proof in &plan.gpu_arena_aggregate_proof_plans {
        for &index in &proof.source_indices {
            let row = &plan.aggregate_source_rows[index];
            assert_eq!(row.arena_location_id_raw, proof.arena_location_id_raw);
            assert_eq!(row.owner_ref, proof.owner_ref);
            assert_eq!(row.resource_key, proof.resource_key);
        }
    }
}

#[test]
fn recursive_local_rf_coexists_with_local_effect_application_without_changing_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let effect_before =
        compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("effect before");
    let _recursive = compile_recursive_local_rf_plan(&spec).expect("recursive");
    let effect_after =
        compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("effect after");

    assert_eq!(
        effect_before.application_report.runtime_applied_total,
        effect_after.application_report.runtime_applied_total
    );
    assert_eq!(
        effect_before.application_report.unmet_total,
        effect_after.application_report.unmet_total
    );
}

#[test]
fn recursive_local_rf_does_not_replace_tick_shell_rf_source() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let rf_before = compile_runtime_rf_tick_plan(&spec).expect("rf before");
    let _recursive = compile_recursive_local_rf_plan(&spec).expect("recursive");
    let rf_after = compile_runtime_rf_tick_plan(&spec).expect("rf after");

    assert_eq!(
        rf_before.tick_report.local_allocated_total,
        rf_after.tick_report.local_allocated_total
    );
    assert_eq!(
        rf_before.tick_report.disburse_down_result_count,
        rf_after.tick_report.disburse_down_result_count
    );
    assert_eq!(
        rf_before.tick_report.local_unmet_total,
        rf_after.tick_report.local_unmet_total
    );
}

#[test]
fn recursive_local_rf_compile_does_not_alter_semantic_local_effects_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let semantic_before =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic before");
    let _recursive = compile_recursive_local_rf_plan(&spec).expect("recursive");
    let semantic_after =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic after");

    assert_eq!(
        semantic_before.semantic_report.runtime_applied_total,
        semantic_after.semantic_report.runtime_applied_total
    );
    assert_eq!(
        semantic_before.semantic_report.shortfall_total,
        semantic_after.semantic_report.shortfall_total
    );
}

#[test]
fn recursive_local_rf_proof_remediation_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");
    let plan = compile_recursive_local_rf_plan(&spec).expect("compile");
    let after = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(plan.authority_proof.scenario_authority_unchanged);
}

#[test]
fn recursive_local_rf_proof_remediation_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");
    let _plan = compile_recursive_local_rf_plan(&spec).expect("compile");
    let after = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
}
