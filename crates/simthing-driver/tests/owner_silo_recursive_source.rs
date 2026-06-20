//! OWNER-SILO-RECURSIVE-RF-SOURCE-0 — recursive RF owner-silo source driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_effect_application_plan, compile_owner_silo_recursive_source_plan,
    compile_recursive_local_rf_plan, compile_runtime_rf_tick_plan, compile_runtime_tick_shell_plan,
    compile_semantic_local_effects_plan,
};
use simthing_spec::{serialize_scenario_authority, OwnerSiloRfSourceMode, RuntimeTickId};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn owner_silo_recursive_source_compile_composes_selection_reconciliation_and_recursive_rf_plans() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");

    assert!(plan.reconciliation_plan.previous_ladder_preserved);
    assert!(!plan
        .recursive_local_rf_plan
        .aggregate_source_rows
        .is_empty());
    assert!(
        plan.source_selection_plan
            .selected_source_report
            .selection_gate
            .reconciliation_ready
    );
}

#[test]
fn owner_silo_recursive_source_compile_legacy_default_preserved() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo,
    )
    .expect("compile");

    assert!(plan.legacy_default_preserved);
    assert_eq!(
        plan.selected_source_mode,
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo
    );
}

#[test]
fn owner_silo_recursive_source_compile_recursive_mode_runs_owner_silo_disburse_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");

    assert!(plan.owner_silo_disburse_down_executed_for_selected_source);
    assert!(
        plan.disburse_report
            .recursive_disburse_report
            .as_ref()
            .expect("recursive")
            .owner_silo_disburse_down_executed
    );
    assert!(
        plan.disburse_report
            .selected_disburse_report
            .allocated_total
            > 0
    );
}

#[test]
fn owner_silo_recursive_source_compile_does_not_change_default_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");
    let after = compile_runtime_rf_tick_plan(&spec).expect("after");

    assert_eq!(
        before.tick_report.disburse_allocated_total,
        after.tick_report.disburse_allocated_total
    );
}

#[test]
fn owner_silo_recursive_source_compile_does_not_change_default_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");
    let after = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("after");

    assert_eq!(
        before
            .runtime_rf_tick_plan
            .tick_report
            .disburse_allocated_total,
        after
            .runtime_rf_tick_plan
            .tick_report
            .disburse_allocated_total
    );
}

#[test]
fn owner_silo_recursive_source_compile_preserves_local_effect_application_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("before");
    let _plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");
    let after = compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("after");

    assert_eq!(
        before.application_report.runtime_applied_total,
        after.application_report.runtime_applied_total
    );
}

#[test]
fn owner_silo_recursive_source_compile_preserves_semantic_local_effect_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("before");
    let _plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");
    let after = compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("after");

    assert_eq!(
        before.semantic_report.runtime_applied_total,
        after.semantic_report.runtime_applied_total
    );
}

#[test]
fn owner_silo_recursive_source_compile_defers_local_allocation_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");

    assert!(plan.local_allocation_integration_deferred);
}

#[test]
fn owner_silo_recursive_source_compile_defers_local_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");

    assert!(plan.local_effect_integration_deferred);
}

#[test]
fn owner_silo_recursive_source_compile_defers_semantic_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");

    assert!(plan.semantic_effect_integration_deferred);
}

#[test]
fn owner_silo_recursive_source_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
    assert!(plan.gpu_residency_doctrine_preserved);
}

#[test]
fn owner_silo_recursive_source_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn owner_silo_recursive_source_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}

#[test]
fn owner_silo_recursive_source_compile_reuses_recursive_aggregate_source_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let rf_plan = compile_recursive_local_rf_plan(&spec).expect("rf");
    let plan = compile_owner_silo_recursive_source_plan(
        &spec,
        TICK_ONE,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("compile");

    assert_eq!(
        plan.recursive_local_rf_plan.aggregate_source_rows.len(),
        rf_plan.aggregate_source_rows.len()
    );
}
