//! SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 — recursive RF semantic local effects spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_recursive_local_rf, evaluate_semantic_local_effects_with_rf_source,
    prove_semantic_local_effects_recursive_source_preserves_authority,
    recursive_local_rf_aggregate_source_rows, serialize_scenario_authority, RuntimeTickId,
    SemanticLocalEffectKind, SemanticLocalEffectRfSourceMode, OWNER_FLOW_DEMAND_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;
#[test]
fn semantic_recursive_source_consumes_recursive_local_effect_application_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    let local_effect = report
        .recursive_local_effect_report
        .as_ref()
        .expect("local effect report");
    assert!(
        local_effect
            .recursive_application_report
            .as_ref()
            .expect("recursive application")
            .runtime_applied_total
            > 0
    );
}

#[test]
fn semantic_recursive_source_produces_recursive_semantic_local_effects_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    let semantic = report.recursive_semantic_report.as_ref().expect("semantic");
    assert!(semantic.output_count > 0);
    assert!(semantic.runtime_applied_total > 0);
    assert!(report.semantic_local_effects_projected_for_selected_source);
    assert!(report.recursive_source_projection_only);
}

#[test]
fn semantic_recursive_source_selected_report_matches_selected_mode() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert_eq!(
        report.selected_source_mode,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable
    );
    assert_eq!(
        report.selected_semantic_report.runtime_applied_total,
        report
            .recursive_semantic_report
            .as_ref()
            .expect("recursive semantic")
            .runtime_applied_total
    );
}

#[test]
fn semantic_recursive_source_preserves_semantic_kinds() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    let semantic = report.recursive_semantic_report.as_ref().expect("semantic");
    assert!(semantic
        .outputs
        .iter()
        .any(|o| o.effect_kind == SemanticLocalEffectKind::RuntimeAppliedAmount));
    assert!(semantic
        .outputs
        .iter()
        .any(|o| o.effect_kind == SemanticLocalEffectKind::ResourceSatisfied));
    assert!(semantic.outputs.iter().all(|o| {
        matches!(
            o.effect_kind,
            SemanticLocalEffectKind::ResourceSatisfied
                | SemanticLocalEffectKind::ResourceShortfall
                | SemanticLocalEffectKind::RuntimeAppliedAmount
        )
    }));
    if semantic.shortfall_output_count > 0 {
        assert!(semantic
            .outputs
            .iter()
            .any(|o| o.effect_kind == SemanticLocalEffectKind::ResourceShortfall));
    }
}

#[test]
fn semantic_recursive_source_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    let semantic = report.recursive_semantic_report.as_ref().expect("semantic");
    assert!(semantic
        .outputs
        .iter()
        .any(|o| o.owner_ref.as_str() == "owner_a"));
    assert!(semantic
        .outputs
        .iter()
        .any(|o| o.owner_ref.as_str() == "owner_b"));
}

#[test]
fn semantic_recursive_source_preserves_recursive_resource_metadata_but_uses_current_generic_writeback_channel(
) {
    let spec = build_sibling_redistribution_spec();
    let recursive = evaluate_recursive_local_rf(&spec).expect("recursive");
    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive);
    assert!(aggregate_rows
        .iter()
        .any(|row| row.resource_key.as_str() == "food"));

    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive semantic");
    let semantic = report.recursive_semantic_report.as_ref().expect("semantic");
    assert!(semantic
        .outputs
        .iter()
        .all(|output| output.resource_key.as_str() == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn semantic_recursive_source_requires_recursive_local_effect_ready() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.source_selection.local_effect_recursive_source_ready);
    assert!(
        report
            .source_selection
            .recursive_application_report_available
    );
    assert!(report.source_selection.selection_allowed);
}

#[test]
fn semantic_recursive_source_documents_redistribution_delta_for_sibling_fixture() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("sibling");

    assert!(report.source_selection.selection_allowed);
    let local_effect = report
        .recursive_local_effect_report
        .as_ref()
        .expect("local effect");
    let allocation = local_effect
        .recursive_local_allocation_report
        .as_ref()
        .expect("allocation");
    let owner_silo = allocation
        .recursive_owner_silo_disburse_report
        .as_ref()
        .expect("owner silo");
    assert!(owner_silo.source_selection.redistribution_deltas_documented);
    assert!(report.recursive_semantic_report.is_some());
}

#[test]
fn semantic_recursive_source_defers_semantic_execution() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.semantic_execution_deferred);
    assert!(report
        .recursive_semantic_report
        .as_ref()
        .expect("semantic")
        .outputs
        .iter()
        .all(|output| output.semantic_execution_deferred));
}

#[test]
fn semantic_recursive_source_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(
        prove_semantic_local_effects_recursive_source_preserves_authority(
            &spec,
            TICK_ONE,
            SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
            REPLAY_ONE,
        )
        .expect("proof")
    );
}

#[test]
fn semantic_recursive_source_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _report = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}

#[test]
fn normal_tests_do_not_write_semantic_recursive_source_fixture() {
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
        .find(|c| simthing_spec::is_surface_gridcell(c))
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Cohort)
        .unwrap();
    cohort.properties.insert(
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        PropertyValue::from_raw_lanes(vec![1.5]),
    );

    let err = evaluate_semantic_local_effects_with_rf_source(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_ONE,
    )
    .unwrap_err();
    assert!(!err.message.is_empty());
}
