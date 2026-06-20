//! OWNER-SILO-RECURSIVE-RF-SOURCE-0 — recursive RF owner-silo source spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_owner_silo_disburse_down_with_rf_source,
    owner_silo_demand_buckets_from_recursive_local_rf, prove_owner_silo_recursive_source_preserves_authority,
    serialize_scenario_authority, OwnerSiloRfSourceMode, OWNER_FLOW_DEMAND_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

#[test]
fn owner_silo_recursive_source_preserves_legacy_default() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo,
    )
    .expect("legacy");

    assert!(report.legacy_default_preserved);
    assert_eq!(
        report.selected_source_mode,
        OwnerSiloRfSourceMode::LegacyPlanetChildOwnerSilo
    );
    assert_eq!(
        report.selected_disburse_report.allocated_total,
        report.legacy_disburse_report.allocated_total
    );
}

#[test]
fn owner_silo_recursive_source_projects_recursive_demand_buckets() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let buckets = owner_silo_demand_buckets_from_recursive_local_rf(&spec).expect("buckets");

    assert!(!buckets.is_empty());
    assert!(buckets.iter().all(|b| !b.owner_ref.is_empty()));
    assert!(buckets
        .iter()
        .all(|b| !b.resource_key.is_empty() || b.resource_key == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn owner_silo_recursive_source_runs_disburse_down_for_recursive_source() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("recursive");

    assert!(report.source_selection.selection_allowed);
    assert_eq!(
        report.selected_source_mode,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable
    );
    assert!(report.owner_silo_disburse_down_executed_for_selected_source);
    let recursive = report.recursive_disburse_report.as_ref().expect("recursive report");
    assert!(recursive.owner_silo_disburse_down_executed);
    assert!(!recursive.disburse_down_results.is_empty());
    assert!(recursive.allocated_total > 0);
}

#[test]
fn owner_silo_recursive_source_is_not_comparison_only_hygiene_layer() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("recursive");

    assert!(report.owner_silo_disburse_down_executed_for_selected_source);
    assert!(
        report
            .selected_disburse_report
            .disburse_down_results
            .iter()
            .any(|r| r.allocated_total > 0)
    );
    assert!(report.local_allocation_integration_deferred);
}

#[test]
fn owner_silo_recursive_source_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let buckets = owner_silo_demand_buckets_from_recursive_local_rf(&spec).expect("buckets");

    assert!(buckets.iter().any(|b| b.owner_ref == "owner_a"));
    assert!(buckets.iter().any(|b| b.owner_ref == "owner_b"));
    assert!(buckets
        .iter()
        .all(|b| b.scope_id.starts_with("location/")));
}

#[test]
fn owner_silo_recursive_source_preserves_generic_resource_fallback() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let buckets = owner_silo_demand_buckets_from_recursive_local_rf(&spec).expect("buckets");

    assert!(buckets
        .iter()
        .all(|b| !b.resource_key.trim().is_empty()));
}

#[test]
fn owner_silo_recursive_source_requires_reconciliation_ready() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("recursive");

    assert!(report.source_selection.reconciliation_ready);
    assert!(report.source_selection.selection_allowed);
}

#[test]
fn owner_silo_recursive_source_documents_redistribution_delta_for_sibling_fixture() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("sibling");

    assert!(report.source_selection.redistribution_deltas_documented);
    assert!(report.source_selection.selection_allowed);
    let recursive = report.recursive_disburse_report.as_ref().expect("recursive");
    assert!(!recursive.demand_buckets.is_empty());
}

#[test]
fn owner_silo_recursive_source_defers_local_allocation_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("recursive");

    assert!(report.local_allocation_integration_deferred);
    assert!(report.recursive_source_report_only_beyond_owner_silo);
}

#[test]
fn owner_silo_recursive_source_defers_local_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("recursive");

    assert!(report.local_effect_integration_deferred);
}

#[test]
fn owner_silo_recursive_source_defers_semantic_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("recursive");

    assert!(report.semantic_effect_integration_deferred);
}

#[test]
fn owner_silo_recursive_source_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(prove_owner_silo_recursive_source_preserves_authority(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("proof"));
}

#[test]
fn owner_silo_recursive_source_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _report = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}

#[test]
fn normal_tests_do_not_write_owner_silo_recursive_source_fixture() {
    let mut spec = build_owner_silo_disburse_down_scoped_spec();
    let gs = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap();
    let star = gs
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| simthing_spec::gridcell_role(c).as_deref() == Some("star_system"))
        .unwrap();
    let planet = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap();
    let cohort = planet
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Cohort)
        .unwrap();
    cohort.properties.insert(
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        PropertyValue { data: vec![1.5] },
    );

    let err = evaluate_owner_silo_disburse_down_with_rf_source(
        &spec,
        OwnerSiloRfSourceMode::RecursiveLocalRfSelectable,
    )
    .unwrap_err();
    assert!(!err.message.is_empty());
}