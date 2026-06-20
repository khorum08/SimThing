//! LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 — recursive RF local allocation spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_recursive_local_rf, evaluate_runtime_local_allocation_with_rf_source,
    owner_silo_demand_buckets_from_recursive_local_rf,
    prove_local_allocation_recursive_source_preserves_authority,
    recursive_local_rf_aggregate_source_rows, serialize_scenario_authority,
    LocalAllocationRfSourceMode, OWNER_FLOW_DEMAND_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

#[test]
fn local_allocation_recursive_source_preserves_legacy_default() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo,
    )
    .expect("legacy");

    assert!(report.legacy_default_preserved);
    assert_eq!(
        report.selected_source_mode,
        LocalAllocationRfSourceMode::LegacyPlanetChildOwnerSilo
    );
    assert_eq!(
        report.selected_allocation_report.allocated_total,
        report.legacy_allocation_report.allocated_total
    );
}

#[test]
fn local_allocation_recursive_source_consumes_recursive_owner_silo_disburse_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");

    let owner_silo = report
        .recursive_owner_silo_disburse_report
        .as_ref()
        .expect("owner silo report");
    assert!(
        owner_silo
            .recursive_disburse_report
            .as_ref()
            .expect("recursive disburse")
            .owner_silo_disburse_down_executed
    );
}

#[test]
fn local_allocation_recursive_source_produces_recursive_allocation_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");

    assert!(report.source_selection.selection_allowed);
    let recursive = report
        .recursive_allocation_report
        .as_ref()
        .expect("recursive alloc");
    assert!(recursive.allocation_count > 0);
    assert!(recursive.allocated_total > 0);
    assert!(report.local_allocation_executed_for_selected_source);
}

#[test]
fn local_allocation_recursive_source_is_not_comparison_only_hygiene_layer() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");

    assert!(report.local_allocation_executed_for_selected_source);
    assert_eq!(
        report.selected_source_mode,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable
    );
    assert!(report.local_effect_integration_deferred);
}

#[test]
fn local_allocation_recursive_source_selected_report_matches_selected_mode() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");

    assert_eq!(
        report.selected_source_mode,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable
    );
    assert_eq!(
        report.selected_allocation_report.allocated_total,
        report
            .recursive_allocation_report
            .as_ref()
            .expect("recursive")
            .allocated_total
    );
}

#[test]
fn local_allocation_recursive_source_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");

    let recursive = report
        .recursive_allocation_report
        .as_ref()
        .expect("recursive");
    assert!(recursive.states.iter().any(|s| s.owner_ref == "owner_a"));
    assert!(recursive.states.iter().any(|s| s.owner_ref == "owner_b"));
}

#[test]
fn local_allocation_recursive_source_preserves_recursive_resource_metadata_in_source_rows_but_uses_generic_writeback_channel(
) {
    let spec = build_sibling_redistribution_spec();
    let recursive = evaluate_recursive_local_rf(&spec).expect("recursive");
    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive);
    assert!(aggregate_rows.iter().any(|row| row.resource_key == "food"));

    let buckets = owner_silo_demand_buckets_from_recursive_local_rf(&spec).expect("buckets");
    assert!(buckets
        .iter()
        .all(|bucket| bucket.resource_key == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn local_allocation_recursive_source_requires_owner_silo_recursive_source_ready() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");

    assert!(report.source_selection.owner_silo_recursive_source_ready);
    assert!(report.source_selection.owner_silo_disburse_report_available);
}

#[test]
fn local_allocation_recursive_source_documents_redistribution_delta_for_sibling_fixture() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("sibling");

    assert!(report.source_selection.selection_allowed);
    let owner_silo = report
        .recursive_owner_silo_disburse_report
        .as_ref()
        .expect("owner silo");
    assert!(owner_silo.source_selection.redistribution_deltas_documented);
    assert!(report.recursive_allocation_report.is_some());
}

#[test]
fn local_allocation_recursive_source_defers_local_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");

    assert!(report.local_effect_integration_deferred);
    assert!(report.recursive_source_report_only_beyond_local_allocation);
}

#[test]
fn local_allocation_recursive_source_defers_semantic_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");

    assert!(report.semantic_effect_integration_deferred);
}

#[test]
fn local_allocation_recursive_source_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(prove_local_allocation_recursive_source_preserves_authority(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("proof"));
}

#[test]
fn local_allocation_recursive_source_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _report = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}

#[test]
fn normal_tests_do_not_write_local_allocation_recursive_source_fixture() {
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

    let err = evaluate_runtime_local_allocation_with_rf_source(
        &spec,
        LocalAllocationRfSourceMode::RecursiveOwnerSiloSelectable,
    )
    .unwrap_err();
    assert!(!err.message.is_empty());
}
