//! RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 — optional RF tick source comparison driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_effect_application_plan, compile_runtime_rf_tick_plan,
    compile_runtime_rf_tick_source_comparison_plan, compile_runtime_tick_shell_plan,
    compile_runtime_tick_shell_with_rf_source_comparison_plan, compile_semantic_local_effects_plan,
};
use simthing_spec::{serialize_scenario_authority, RuntimeRfTickSourceKind, RuntimeTickId};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn runtime_rf_tick_source_compile_composes_legacy_and_recursive_plans() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compile");

    assert!(plan.legacy_tick_plan.tick_report.participant_count > 0);
    assert!(!plan
        .recursive_local_rf_plan
        .aggregate_source_rows
        .is_empty());
    assert!(plan.reconciliation_plan.previous_ladder_preserved);
    assert!(plan.comparison_report.reconciliation_ready);
}

#[test]
fn runtime_rf_tick_source_compile_preserves_default_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _compare = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compare");
    let after = compile_runtime_rf_tick_plan(&spec).expect("after");

    assert_eq!(
        before.tick_report.local_allocated_total,
        after.tick_report.local_allocated_total
    );
    assert_eq!(
        before.tick_report.local_unmet_total,
        after.tick_report.local_unmet_total
    );
}

#[test]
fn runtime_rf_tick_source_compile_preserves_default_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _compare = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compare");
    let after = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("after");

    assert_eq!(
        before.execution_report.runtime_rf_tick_ready,
        after.execution_report.runtime_rf_tick_ready
    );
    assert_eq!(
        before
            .runtime_rf_tick_plan
            .tick_report
            .local_allocated_total,
        after.runtime_rf_tick_plan.tick_report.local_allocated_total
    );
}

#[test]
fn runtime_rf_tick_source_compile_reports_recursive_preview_only() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compile");

    assert!(plan.recursive_source_preview_only);
    assert_eq!(
        plan.default_source_kind,
        RuntimeRfTickSourceKind::LegacyPlanetChildOwnerSilo
    );
    assert!(plan.comparison_report.recursive_source_available);
}

#[test]
fn runtime_rf_tick_source_compile_reports_tick_shell_source_replacement_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compile");

    assert!(plan.tick_shell_source_replacement_deferred);
    assert!(
        plan.comparison_report
            .tick_shell_source_replacement_deferred
    );
}

#[test]
fn runtime_rf_tick_source_compile_reuses_recursive_aggregate_source_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compile");

    assert!(!plan
        .recursive_local_rf_plan
        .aggregate_source_rows
        .is_empty());
    assert_eq!(
        plan.comparison_report
            .recursive_summary
            .gpu_compatible_row_count,
        plan.recursive_local_rf_plan.aggregate_source_rows.len() as u32
    );
}

#[test]
fn runtime_rf_tick_source_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
    for proof in &plan.recursive_local_rf_plan.gpu_arena_aggregate_proof_plans {
        assert_eq!(proof.surplus_plan.ops.len(), 1);
    }
}

#[test]
fn runtime_rf_tick_source_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn runtime_rf_tick_source_compile_preserves_local_effect_application_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("effect before");
    let _compare = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compare");
    let after = compile_local_effect_application_plan(&spec, TICK_ONE, 3).expect("effect after");

    assert_eq!(
        before.application_report.runtime_applied_total,
        after.application_report.runtime_applied_total
    );
}

#[test]
fn runtime_rf_tick_source_compile_preserves_semantic_local_effect_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic before");
    let _compare = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compare");
    let after = compile_semantic_local_effects_plan(&spec, TICK_ONE, 1).expect("semantic after");

    assert_eq!(
        before.semantic_report.runtime_applied_total,
        after.semantic_report.runtime_applied_total
    );
}

#[test]
fn runtime_rf_tick_source_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let plan = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(plan.comparison_report.scenario_authority_mutation_deferred);
}

#[test]
fn runtime_rf_tick_source_tick_shell_wrapper_preserves_default_shell() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let shell = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("shell");
    let wrapped =
        compile_runtime_tick_shell_with_rf_source_comparison_plan(&spec, TICK_ONE).expect("wrap");

    assert_eq!(
        shell.runtime_rf_tick_plan.tick_report.local_allocated_total,
        wrapped
            .tick_shell_plan
            .runtime_rf_tick_plan
            .tick_report
            .local_allocated_total
    );
    assert!(
        wrapped
            .rf_source_comparison_plan
            .recursive_source_preview_only
    );
}

#[test]
fn runtime_rf_tick_source_cpu_oracle_does_not_wire_recursive_into_tick_shell_default() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_source_comparison_plan(&spec).expect("compile");
    let shell = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("shell");

    assert!(plan.tick_shell_source_replacement_deferred);
    assert!(shell.execution_report.runtime_rf_tick_ready);
    assert_eq!(
        plan.default_source_kind,
        RuntimeRfTickSourceKind::LegacyPlanetChildOwnerSilo
    );
}
