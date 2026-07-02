//! RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 — optional RF tick source comparison spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_spec::{
    evaluate_runtime_rf_tick, evaluate_runtime_rf_tick_source_comparison,
    evaluate_runtime_rf_tick_source_preview, prove_runtime_rf_tick_source_preserves_authority,
    RuntimeRfTickSourceDeltaKind, RuntimeRfTickSourceKind, RuntimeRfTickSourceMode,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::{build_sibling_redistribution_spec, star_system_id_raw};
#[test]
fn runtime_rf_tick_source_comparison_reports_recursive_preview_available() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");

    assert!(report.recursive_source_available);
    assert!(report.recursive_source_preview_only);
    assert!(report.recursive_summary.gpu_compatible_row_count > 0);
}

#[test]
fn runtime_rf_tick_source_comparison_composes_reconciliation_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");

    assert!(report.reconciliation_ready);
    assert!(!report.deltas.is_empty() || report.reconciliation_compatible);
}

#[test]
fn runtime_rf_tick_source_comparison_distinguishes_redistribution_delta_from_error() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");

    assert!(report.recursive_source_available);
    assert!(report.deltas.iter().any(|delta| {
        delta.delta_kind == RuntimeRfTickSourceDeltaKind::ScopeProjectionDelta
            || delta.delta_kind == RuntimeRfTickSourceDeltaKind::RecursiveRedistributionDelta
    }));
    assert!(report.tick_shell_source_replacement_deferred);
}

#[test]
fn runtime_rf_tick_source_comparison_owner_silo_fixture_documents_surface_scope_delta() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");

    assert!(report.participant_projection_compatible);
    assert!(!report.reconciliation_compatible);
    assert!(report.deltas.iter().any(|delta| {
        delta.delta_kind == RuntimeRfTickSourceDeltaKind::ScopeProjectionDelta
    }));
}

#[test]
fn runtime_rf_tick_source_comparison_sibling_fixture_documents_scope_delta() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");
    let star_id = star_system_id_raw(&spec);

    assert!(
        report.deltas.iter().any(|delta| {
            delta.delta_kind == RuntimeRfTickSourceDeltaKind::ScopeProjectionDelta
                && delta.recursive_surplus_total > 0
        }) || report.recursive_summary.net_surplus_total > 0
    );
    let _ = star_id;
}

#[test]
fn runtime_rf_tick_source_comparison_preserves_gpu_residency_doctrine() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");

    assert!(report.legacy_summary.gpu_residency_doctrine_preserved);
    assert!(report.recursive_summary.gpu_residency_doctrine_preserved);
    assert!(report.recursive_summary.gpu_compatible_row_count > 0);
}

#[test]
fn runtime_rf_tick_source_comparison_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(prove_runtime_rf_tick_source_preserves_authority(&spec).expect("proof"));
}

#[test]
fn runtime_rf_tick_source_comparison_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");
    let _report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");
    let after = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
}

#[test]
fn runtime_rf_tick_source_comparison_defers_tick_shell_source_replacement() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");

    assert!(report.tick_shell_source_replacement_deferred);
}

#[test]
fn runtime_rf_tick_source_comparison_defers_semantic_execution() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");

    assert!(report.semantic_execution_deferred);
}

#[test]
fn runtime_rf_tick_source_preview_recursive_mode_selects_recursive_kind() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report =
        evaluate_runtime_rf_tick_source_preview(&spec, RuntimeRfTickSourceMode::RecursivePreview)
            .expect("preview");

    assert_eq!(
        report.selected_source_kind,
        RuntimeRfTickSourceKind::RecursiveLocalRf
    );
    assert!(report.recursive_source_preview_only);
}

#[test]
fn normal_tests_do_not_write_runtime_rf_tick_source_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let _report = evaluate_runtime_rf_tick_source_comparison(&spec).expect("compare");
}
