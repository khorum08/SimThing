//! PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 — reconciliation driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;
#[path = "../../simthing-spec/tests/sibling_redistribution_fixture.rs"]
mod sibling_redistribution_fixture;

use simthing_driver::{
    compile_local_effect_application_plan, compile_recursive_rf_reconciliation_plan,
    compile_runtime_rf_tick_plan, compile_semantic_local_effects_plan,
};
use simthing_spec::{serialize_scenario_authority, RuntimeTickId};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn recursive_rf_reconciliation_compile_composes_recursive_local_rf_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_rf_reconciliation_plan(&spec).expect("compile");

    assert!(
        plan.recursive_local_rf_plan
            .evaluation_report
            .location_count
            > 0
    );
    assert!(!plan
        .recursive_local_rf_plan
        .aggregate_source_rows
        .is_empty());
    assert!(plan.reconciliation_report.legacy_projection_count > 0);
}

#[test]
fn recursive_rf_reconciliation_compile_preserves_prior_runtime_rf_tick_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let rf_before = compile_runtime_rf_tick_plan(&spec).expect("rf before");
    let _reconcile = compile_recursive_rf_reconciliation_plan(&spec).expect("reconcile");
    let rf_after = compile_runtime_rf_tick_plan(&spec).expect("rf after");

    assert_eq!(
        rf_before.tick_report.local_allocated_total,
        rf_after.tick_report.local_allocated_total
    );
    assert_eq!(
        rf_before.tick_report.local_unmet_total,
        rf_after.tick_report.local_unmet_total
    );
}

#[test]
fn recursive_rf_reconciliation_compile_preserves_local_effect_application_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let effect_before =
        compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("effect before");
    let _reconcile = compile_recursive_rf_reconciliation_plan(&spec).expect("reconcile");
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
fn recursive_rf_reconciliation_compile_preserves_semantic_local_effect_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let semantic_before =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic before");
    let _reconcile = compile_recursive_rf_reconciliation_plan(&spec).expect("reconcile");
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
fn recursive_rf_reconciliation_compile_reports_gpu_residency_doctrine_preserved() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_rf_reconciliation_plan(&spec).expect("compile");

    assert!(plan.gpu_residency_doctrine_preserved);
    assert!(!plan
        .recursive_local_rf_plan
        .aggregate_source_rows
        .is_empty());
}

#[test]
fn recursive_rf_reconciliation_compile_reports_tick_shell_source_replacement_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_rf_reconciliation_plan(&spec).expect("compile");

    assert!(plan.tick_shell_source_replacement_deferred);
    assert!(
        plan.reconciliation_report
            .tick_shell_source_replacement_deferred
    );
}

#[test]
fn recursive_rf_reconciliation_compile_reuses_recursive_aggregate_source_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_rf_reconciliation_plan(&spec).expect("compile");

    assert!(!plan
        .recursive_local_rf_plan
        .aggregate_source_rows
        .is_empty());
    assert!(plan.reconciliation_report.recursive_projection_count > 0);
}

#[test]
fn recursive_rf_reconciliation_compile_does_not_require_new_gpu_primitive() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_rf_reconciliation_plan(&spec).expect("compile");

    for proof in &plan.recursive_local_rf_plan.gpu_arena_aggregate_proof_plans {
        assert_eq!(proof.surplus_plan.ops.len(), 1);
        assert_eq!(proof.demand_plan.ops.len(), 1);
    }
}

#[test]
fn recursive_rf_reconciliation_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let plan = compile_recursive_rf_reconciliation_plan(&spec).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(
        plan.recursive_local_rf_plan
            .authority_proof
            .scenario_authority_unchanged
    );
    assert!(
        plan.reconciliation_report
            .scenario_authority_mutation_deferred
    );
}

#[test]
fn recursive_rf_reconciliation_cpu_oracle_does_not_wire_recursive_rf_into_tick_shell() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_recursive_rf_reconciliation_plan(&spec).expect("compile");
    let rf = compile_runtime_rf_tick_plan(&spec).expect("rf");

    assert!(plan.tick_shell_source_replacement_deferred);
    assert!(rf.tick_report.participant_admission_ready);
    assert!(plan.previous_ladder_preserved);
}
