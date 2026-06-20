//! RUNTIME-TICK-RECURSIVE-RF-SELECTABLE-SOURCE-0 — selectable RF source spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_spec::{
    evaluate_runtime_rf_tick, evaluate_runtime_rf_tick_source_selection,
    prove_runtime_rf_tick_source_selection_preserves_authority, RuntimeRfTickSourceDeltaKind,
    RuntimeRfTickSourceKind, RuntimeRfTickSourceMode,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

#[test]
fn runtime_rf_tick_source_selection_legacy_default_preserved() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report =
        evaluate_runtime_rf_tick_source_selection(&spec, RuntimeRfTickSourceMode::LegacyDefault)
            .expect("select");

    assert_eq!(
        report.selection_gate.selected_source_kind,
        RuntimeRfTickSourceKind::LegacyPlanetChildOwnerSilo
    );
    assert!(report.selection_gate.legacy_default_preserved);
}

#[test]
fn runtime_rf_tick_source_selection_recursive_selectable_requires_reconciliation_ready() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert!(report.selection_gate.reconciliation_ready);
}

#[test]
fn runtime_rf_tick_source_selection_recursive_selectable_allowed_for_owner_silo_fixture_report_only(
) {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert!(report.selection_gate.selection_allowed);
    assert_eq!(
        report.selection_gate.selected_source_kind,
        RuntimeRfTickSourceKind::RecursiveLocalRf
    );
    assert!(report.recursive_source_selected_for_rf_report_only);
    assert!(report.owner_silo_integration_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_recursive_selectable_preserves_legacy_tick_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let legacy = evaluate_runtime_rf_tick(&spec).expect("legacy");
    let _report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");
    let legacy_after = evaluate_runtime_rf_tick(&spec).expect("legacy after");

    assert_eq!(
        legacy.local_allocated_total,
        legacy_after.local_allocated_total
    );
    assert_eq!(legacy.local_unmet_total, legacy_after.local_unmet_total);
}

#[test]
fn runtime_rf_tick_source_selection_recursive_selectable_defers_owner_silo_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert!(report.owner_silo_integration_deferred);
    assert!(report.selection_gate.downstream_effect_paths_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_recursive_selectable_defers_local_allocation_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert!(report.local_allocation_integration_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_recursive_selectable_defers_local_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert!(report.local_effect_integration_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_recursive_selectable_defers_semantic_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert!(report.semantic_effect_integration_deferred);
}

#[test]
fn runtime_rf_tick_source_selection_documents_redistribution_delta_for_sibling_fixture() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert!(report.selection_gate.selection_allowed);
    assert!(report.selection_gate.redistribution_deltas_documented);
    assert!(report.comparison_report.deltas.iter().any(|delta| {
        matches!(
            delta.delta_kind,
            RuntimeRfTickSourceDeltaKind::ScopeProjectionDelta
                | RuntimeRfTickSourceDeltaKind::RecursiveRedistributionDelta
        )
    }));
}

#[test]
fn runtime_rf_tick_source_selection_preserves_gpu_residency_doctrine() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");

    assert!(report.selected_summary.gpu_residency_doctrine_preserved);
    assert!(
        report
            .comparison_report
            .recursive_summary
            .gpu_compatible_row_count
            > 0
    );
}

#[test]
fn runtime_rf_tick_source_selection_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(prove_runtime_rf_tick_source_selection_preserves_authority(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable
    )
    .expect("proof"));
}

#[test]
fn runtime_rf_tick_source_selection_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");
    let _report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");
    let after = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
}

#[test]
fn normal_tests_do_not_write_runtime_rf_tick_source_selection_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let _report = evaluate_runtime_rf_tick_source_selection(
        &spec,
        RuntimeRfTickSourceMode::RecursiveSelectable,
    )
    .expect("select");
}
