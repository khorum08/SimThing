//! LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 — recursive RF local allocation driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_allocation_recursive_source_plan, compile_local_effect_application_plan,
    compile_runtime_rf_tick_plan, compile_runtime_tick_shell_plan,
    compile_semantic_local_effects_plan,
};
use simthing_spec::{serialize_scenario_authority, LocalAllocationRfSourceMode, RuntimeTickId};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn local_allocation_recursive_source_compile_composes_owner_silo_recursive_source_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");

    assert!(
        plan.owner_silo_recursive_source_plan
            .disburse_report
            .owner_silo_disburse_down_executed_for_selected_source
    );
    assert!(!plan
        .owner_silo_recursive_source_plan
        .recursive_local_rf_plan
        .aggregate_source_rows
        .is_empty());
}

#[test]
fn local_allocation_recursive_source_compile_recursive_mode_runs_local_allocation_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");

    assert!(plan.local_allocation_executed_for_selected_source);
    assert!(
        plan.allocation_report
            .recursive_allocation_report
            .as_ref()
            .expect("recursive")
            .allocated_total
            > 0
    );
}

#[test]
fn local_allocation_recursive_source_compile_does_not_change_default_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");
    let after = compile_runtime_rf_tick_plan(&spec).expect("after");

    assert_eq!(
        before.tick_report.local_allocated_total,
        after.tick_report.local_allocated_total
    );
}

#[test]
fn local_allocation_recursive_source_compile_does_not_change_default_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");
    let after = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("after");

    assert_eq!(
        before
            .runtime_rf_tick_plan
            .tick_report
            .local_allocated_total,
        after.runtime_rf_tick_plan.tick_report.local_allocated_total
    );
}

#[test]
fn local_allocation_recursive_source_compile_does_not_change_local_effect_application_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("before");
    let _plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");
    let after = compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("after");

    assert_eq!(
        before.application_report.runtime_applied_total,
        after.application_report.runtime_applied_total
    );
}

#[test]
fn local_allocation_recursive_source_compile_does_not_change_semantic_local_effects_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("before");
    let _plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");
    let after = compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("after");

    assert_eq!(
        before.semantic_report.runtime_applied_total,
        after.semantic_report.runtime_applied_total
    );
}

#[test]
fn local_allocation_recursive_source_compile_defers_local_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");

    assert!(plan.local_effect_integration_deferred);
}

#[test]
fn local_allocation_recursive_source_compile_defers_semantic_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");

    assert!(plan.semantic_effect_integration_deferred);
}

#[test]
fn local_allocation_recursive_source_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
    assert!(plan.gpu_residency_doctrine_preserved);
}

#[test]
fn local_allocation_recursive_source_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn local_allocation_recursive_source_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _plan = compile_local_allocation_recursive_source_plan(
        &spec,
        TICK_ONE,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}
