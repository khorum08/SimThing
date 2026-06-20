//! RUNTIME-TICK-RECURSIVE-RF-SELECTABLE-SOURCE-0 — selectable RF source driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_effect_application_plan, compile_runtime_rf_tick_plan,
    compile_runtime_rf_tick_source_comparison_plan, compile_runtime_rf_tick_source_selection_plan,
    compile_runtime_tick_shell_plan, compile_runtime_tick_shell_with_selectable_rf_source_plan,
    compile_semantic_local_effects_plan,
};
use simthing_spec::{
    serialize_scenario_authority, RuntimeRfTickSourceKind, RuntimeRfTickSourceMode, RuntimeTickId,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn runtime_rf_tick_source_selection_compile_composes_comparison_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("compile");

    assert!(plan.comparison_plan.comparison_report.reconciliation_ready);
    assert!(
        plan.selected_source_report
            .selection_gate
            .reconciliation_ready
    );
}

#[test]
fn runtime_rf_tick_source_selection_compile_legacy_default_preserved() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::LegacyDefault,
    )
    .expect("compile");

    assert!(plan.legacy_default_preserved);
    assert_eq!(
        plan.selected_source_kind,
        RuntimeRfTickSourceKind::LegacyPlanetChildOwnerSilo
    );
}

#[test]
fn runtime_rf_tick_source_selection_compile_recursive_selectable_report_only() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("compile");

    assert!(plan.recursive_selected_for_rf_report_only);
    assert_eq!(
        plan.selected_source_kind,
        RuntimeRfTickSourceKind::RecursiveLocalRf
    );
    assert!(plan.owner_silo_integration_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_compile_default_runtime_rf_tick_plan_unchanged() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");
    let after = compile_runtime_rf_tick_plan(&spec).expect("after");

    assert_eq!(
        before.tick_report.local_allocated_total,
        after.tick_report.local_allocated_total
    );
}

#[test]
fn runtime_rf_tick_source_selection_compile_default_tick_shell_plan_unchanged() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");
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
fn runtime_rf_tick_source_selection_compile_preserves_local_effect_application_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("effect before");
    let _plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");
    let after = compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("effect after");

    assert_eq!(
        before.application_report.runtime_applied_total,
        after.application_report.runtime_applied_total
    );
}

#[test]
fn runtime_rf_tick_source_selection_compile_preserves_semantic_local_effect_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic before");
    let _plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");
    let after = compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic after");

    assert_eq!(
        before.semantic_report.runtime_applied_total,
        after.semantic_report.runtime_applied_total
    );
}

#[test]
fn runtime_rf_tick_source_selection_compile_defers_owner_silo_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("compile");

    assert!(plan.owner_silo_integration_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_compile_defers_local_allocation_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("compile");

    assert!(plan.local_allocation_integration_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_compile_defers_semantic_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("compile");

    assert!(plan.semantic_effect_integration_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
}

#[test]
fn runtime_rf_tick_source_selection_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn runtime_rf_tick_source_selection_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(
        plan.comparison_plan
            .comparison_report
            .scenario_authority_mutation_deferred
    );
}

#[test]
fn runtime_rf_tick_source_selection_tick_shell_wrapper_preserves_default_shell() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let shell = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("shell");
    let wrapped = compile_runtime_tick_shell_with_selectable_rf_source_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("wrap");

    assert!(wrapped.default_tick_shell_preserved);
    assert!(wrapped.selected_source_report_only);
    assert_eq!(
        shell.runtime_rf_tick_plan.tick_report.local_allocated_total,
        wrapped
            .default_tick_shell_plan
            .runtime_rf_tick_plan
            .tick_report
            .local_allocated_total
    );
}

#[test]
fn runtime_rf_tick_source_selection_reuses_recursive_aggregate_source_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let compare = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compare");
    let plan = compile_runtime_rf_tick_source_selection_plan(
        &spec,
        TICK_ONE,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert_eq!(
        plan.comparison_plan
            .recursive_local_rf_plan
            .aggregate_source_rows
            .len(),
        compare.recursive_local_rf_plan.aggregate_source_rows.len()
    );
}
