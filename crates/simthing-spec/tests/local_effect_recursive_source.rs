//! LOCAL-EFFECT-APPLICATION-RECURSIVE-RF-SOURCE-0 — recursive RF local effect spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_local_effect_application_with_rf_source, evaluate_recursive_local_rf,
    prove_local_effect_recursive_source_preserves_authority,
    recursive_local_rf_aggregate_source_rows, serialize_scenario_authority,
    LocalEffectRfSourceMode, RuntimeTickId, OWNER_FLOW_DEMAND_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn local_effect_recursive_source_preserves_legacy_default() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo,
    )
    .expect("legacy");

    assert!(report.legacy_default_preserved);
    assert_eq!(
        report.selected_source_mode,
        LocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
    );
    assert_eq!(
        report.selected_application_report.runtime_applied_total,
        report.legacy_application_report.runtime_applied_total
    );
}

#[test]
fn local_effect_recursive_source_consumes_recursive_local_allocation_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive");

    let allocation = report
        .recursive_local_allocation_report
        .as_ref()
        .expect("allocation report");
    assert!(
        allocation
            .recursive_allocation_report
            .as_ref()
            .expect("recursive alloc")
            .allocated_total
            > 0
    );
}

#[test]
fn local_effect_recursive_source_produces_recursive_participant_effects_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive");

    let effects = report
        .recursive_local_participant_effects_report
        .as_ref()
        .expect("effects");
    assert!(effects.effect_count > 0);
    assert!(effects.allocated_total > 0);
}

#[test]
fn local_effect_recursive_source_produces_recursive_effect_application_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive");

    let application = report
        .recursive_application_report
        .as_ref()
        .expect("application");
    assert!(application.application_count > 0);
    assert!(application.runtime_applied_total > 0);
    assert!(report.local_effect_application_executed_for_selected_source);
    assert!(report.recursive_source_report_only_beyond_local_effects);
    assert!(report.semantic_effect_integration_deferred);
}

#[test]
fn local_effect_recursive_source_selected_report_matches_selected_mode() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive");

    assert_eq!(
        report.selected_source_mode,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable
    );
    assert_eq!(
        report.selected_application_report.runtime_applied_total,
        report
            .recursive_application_report
            .as_ref()
            .expect("recursive application")
            .runtime_applied_total
    );
}

#[test]
fn local_effect_recursive_source_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive");

    let effects = report
        .recursive_local_participant_effects_report
        .as_ref()
        .expect("effects");
    assert!(effects.effects.iter().any(|e| e.owner_ref == "owner_a"));
    assert!(effects.effects.iter().any(|e| e.owner_ref == "owner_b"));
}

#[test]
fn local_effect_recursive_source_preserves_recursive_resource_metadata_but_uses_current_generic_writeback_channel(
) {
    let spec = build_sibling_redistribution_spec();
    let recursive = evaluate_recursive_local_rf(&spec).expect("recursive");
    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive);
    assert!(aggregate_rows.iter().any(|row| row.resource_key == "food"));

    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive effects");
    let effects = report
        .recursive_local_participant_effects_report
        .as_ref()
        .expect("effects");
    assert!(effects
        .effects
        .iter()
        .all(|effect| effect.resource_key == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn local_effect_recursive_source_requires_recursive_allocation_ready() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive");

    assert!(
        report
            .source_selection
            .local_allocation_recursive_source_ready
    );
    assert!(
        report
            .source_selection
            .recursive_allocation_report_available
    );
    assert!(report.source_selection.selection_allowed);
}

#[test]
fn local_effect_recursive_source_documents_redistribution_delta_for_sibling_fixture() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("sibling");

    assert!(report.source_selection.selection_allowed);
    let allocation = report
        .recursive_local_allocation_report
        .as_ref()
        .expect("allocation");
    let owner_silo = allocation
        .recursive_owner_silo_disburse_report
        .as_ref()
        .expect("owner silo");
    assert!(owner_silo.source_selection.redistribution_deltas_documented);
    assert!(report.recursive_application_report.is_some());
}

#[test]
fn local_effect_recursive_source_defers_semantic_effect_integration() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive");

    assert!(report.semantic_effect_integration_deferred);
    assert!(report
        .recursive_application_report
        .as_ref()
        .expect("application")
        .records
        .iter()
        .all(|record| record.semantic_effect_deferred));
}

#[test]
fn local_effect_recursive_source_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(prove_local_effect_recursive_source_preserves_authority(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("proof"));
}

#[test]
fn local_effect_recursive_source_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _report = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}

#[test]
fn normal_tests_do_not_write_local_effect_recursive_source_fixture() {
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

    let err = evaluate_local_effect_application_with_rf_source(
        &spec,
        TICK_ONE,
        LocalEffectRfSourceMode::RecursiveLocalAllocationSelectable,
    )
    .unwrap_err();
    assert!(!err.message.is_empty());
}
