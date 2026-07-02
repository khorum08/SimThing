//! SEMANTIC-EFFECT-EXECUTION-BOUNDARY-0 — runtime semantic execution records without mutation.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_recursive_local_rf, evaluate_semantic_effect_execution_boundary,
    prove_semantic_effect_execution_boundary_preserves_authority,
    recursive_local_rf_aggregate_source_rows, serialize_scenario_authority, RuntimeTickId,
    SemanticEffectExecutionKind, SemanticEffectExecutionSourceMode, OWNER_FLOW_DEMAND_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;
#[test]
fn semantic_execution_boundary_consumes_recursive_semantic_local_effects_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.recursive_semantic_report_available);
    assert!(report.semantic_projection_ready);
    assert_eq!(
        report.selected_source_mode,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable
    );
}

#[test]
fn semantic_execution_boundary_produces_execution_records() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.execution_record_count > 0);
    assert!(report.executed_amount_total > 0);
    assert!(!report.execution_records.is_empty());
    assert!(report.semantic_execution_boundary_proven);
    assert!(report.recursive_execution_report_only);
}

#[test]
fn semantic_execution_boundary_preserves_semantic_kinds() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report
        .execution_records
        .iter()
        .any(|r| r.execution_kind == SemanticEffectExecutionKind::RuntimeAppliedAmountExecution));
    assert!(report
        .execution_records
        .iter()
        .any(|r| r.execution_kind == SemanticEffectExecutionKind::ResourceSatisfiedExecution));
    assert!(report.execution_records.iter().all(|r| {
        matches!(
            r.execution_kind,
            SemanticEffectExecutionKind::ResourceSatisfiedExecution
                | SemanticEffectExecutionKind::ResourceShortfallExecution
                | SemanticEffectExecutionKind::RuntimeAppliedAmountExecution
        )
    }));
    if report.resource_shortfall_execution_count > 0 {
        assert!(report
            .execution_records
            .iter()
            .any(|r| r.execution_kind == SemanticEffectExecutionKind::ResourceShortfallExecution));
    }
}

#[test]
fn semantic_execution_boundary_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report
        .execution_records
        .iter()
        .any(|r| r.owner_ref.as_str() == "owner_a"));
    assert!(report
        .execution_records
        .iter()
        .any(|r| r.owner_ref.as_str() == "owner_b"));
}

#[test]
fn semantic_execution_boundary_preserves_recursive_resource_metadata_but_uses_current_generic_writeback_channel(
) {
    let spec = build_sibling_redistribution_spec();
    let recursive = evaluate_recursive_local_rf(&spec).expect("recursive");
    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive);
    assert!(aggregate_rows
        .iter()
        .any(|row| row.resource_key.as_str() == "food"));

    let report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive execution");
    assert!(report
        .execution_records
        .iter()
        .all(|record| record.resource_key.as_str() == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn semantic_execution_boundary_defers_participant_property_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.participant_property_mutation_deferred);
    assert!(report
        .execution_records
        .iter()
        .all(|record| record.participant_property_mutation_deferred));
}

#[test]
fn semantic_execution_boundary_defers_scenario_authority_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.scenario_authority_mutation_deferred);
    assert!(report
        .execution_records
        .iter()
        .all(|record| record.scenario_authority_mutation_deferred));
}

#[test]
fn semantic_execution_boundary_defers_savefile_and_persistent_history() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.savefile_mutation_deferred);
    assert!(report.persistent_history_deferred);
    assert!(report
        .execution_records
        .iter()
        .all(|record| record.savefile_mutation_deferred));
}

#[test]
fn semantic_execution_boundary_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(
        prove_semantic_effect_execution_boundary_preserves_authority(
            &spec,
            TICK_ONE,
            SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
            REPLAY_ONE,
        )
        .expect("proof")
    );
}

#[test]
fn semantic_execution_boundary_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _report = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}

#[test]
fn normal_tests_do_not_write_semantic_execution_boundary_fixture() {
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

    let err = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .unwrap_err();
    assert!(!err.message.is_empty());
}
