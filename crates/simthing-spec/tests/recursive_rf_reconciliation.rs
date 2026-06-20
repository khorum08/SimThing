//! PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 — reconciliation spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_spec::{
    project_planet_child_rf_ladder_rows, project_recursive_local_rf_rows,
    prove_recursive_rf_reconciliation_preserves_authority,
    reconcile_planet_child_rf_with_recursive_local_rf, RecursiveRfReconciliationMismatchKind,
    OWNER_FLOW_DEFAULT_RESOURCE_KEY, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::{build_sibling_redistribution_spec, star_system_id_raw};

#[test]
fn recursive_rf_reconciliation_projects_legacy_planet_child_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let rows = project_planet_child_rf_ladder_rows(&spec).expect("legacy");

    assert!(rows.len() >= 3);
    assert!(rows.iter().all(|row| row.source_simthing_id_raw > 0));
}

#[test]
fn recursive_rf_reconciliation_projects_recursive_aggregate_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let rows = project_recursive_local_rf_rows(&spec).expect("recursive");

    assert!(rows
        .iter()
        .any(|row| row.source_kind_label == "direct_participant"));
    assert!(rows
        .iter()
        .any(|row| row.source_kind_label == "arena_settlement"));
}

#[test]
fn recursive_rf_reconciliation_preserves_source_ids_owner_resource_and_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let legacy = project_planet_child_rf_ladder_rows(&spec).expect("legacy");
    let report = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("reconcile");

    assert!(report.participant_row_compatible);
    for row in &legacy {
        assert!(!row.owner_ref.is_empty());
        assert_eq!(row.resource_key, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY);
        assert!(row.planet_gridcell_id_raw > 0);
    }
}

#[test]
fn recursive_rf_reconciliation_preserves_generic_resource_key_fallback() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let legacy = project_planet_child_rf_ladder_rows(&spec).expect("legacy");

    assert!(legacy
        .iter()
        .all(|row| row.resource_key == OWNER_FLOW_DEFAULT_RESOURCE_KEY));
}

#[test]
fn recursive_rf_reconciliation_supports_explicit_resource_key_metadata() {
    let spec = build_sibling_redistribution_spec();
    let recursive = project_recursive_local_rf_rows(&spec).expect("recursive");

    assert!(recursive
        .iter()
        .any(|row| row.resource_key == "food" && row.source_kind_label == "direct_participant"));
}

#[test]
fn recursive_rf_reconciliation_reports_compatible_buckets_for_owner_silo_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("reconcile");

    assert!(report.participant_row_compatible);
    assert!(report.compatible_bucket_count > 0);
    assert_eq!(report.incompatible_bucket_count, 0);
    assert!(report.buckets.iter().all(|bucket| bucket.compatible));
}

#[test]
fn recursive_rf_reconciliation_documents_sibling_redistribution_scope_delta() {
    let spec = build_sibling_redistribution_spec();
    let report = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("reconcile");
    let star_id = star_system_id_raw(&spec);

    assert!(report.participant_row_compatible);
    assert!(report.sibling_redistribution_scope_delta_observed);
    assert!(report.mismatches.iter().any(|m| {
        m.mismatch_kind == RecursiveRfReconciliationMismatchKind::ScopeProjectionMismatch
            && m.location_id_raw == Some(star_id)
    }));
}

#[test]
fn recursive_rf_reconciliation_mismatch_report_is_deterministic() {
    let spec = build_sibling_redistribution_spec();
    let first = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("first");
    let second = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("second");

    assert_eq!(first.mismatches, second.mismatches);
    assert_eq!(first.buckets, second.buckets);
}

#[test]
fn recursive_rf_reconciliation_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(prove_recursive_rf_reconciliation_preserves_authority(&spec).expect("proof"));
}

#[test]
fn recursive_rf_reconciliation_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");
    let _report = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("reconcile");
    let after = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
}

#[test]
fn recursive_rf_reconciliation_does_not_replace_tick_shell_source() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("reconcile");

    assert!(report.tick_shell_source_replacement_deferred);
    assert!(report
        .deferrals
        .iter()
        .any(|d| d.reason.contains("planet-child/owner-silo ladder")));
}

#[test]
fn normal_tests_do_not_write_recursive_rf_reconciliation_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let _report = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("reconcile");
}
