//! SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 — runtime participant property delta previews without mutation.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_recursive_local_rf, evaluate_semantic_effect_execution_boundary,
    evaluate_semantic_participant_delta_preview,
    prove_semantic_participant_delta_preview_preserves_authority,
    recursive_local_rf_aggregate_source_rows, serialize_scenario_authority,
    ParticipantDeltaPreviewKind, ParticipantDeltaPreviewSourceMode, RuntimeTickId,
    SemanticEffectExecutionSourceMode, OWNER_FLOW_DEMAND_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY, RUNTIME_PREVIEW_APPLIED_PROPERTY_ID,
    RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID, RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;
#[test]
fn semantic_delta_preview_consumes_recursive_execution_records() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let execution = evaluate_semantic_effect_execution_boundary(
        &spec,
        TICK_ONE,
        SemanticEffectExecutionSourceMode::RecursiveSemanticLocalEffectsSelectable,
        REPLAY_ONE,
    )
    .expect("execution");
    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.execution_boundary_ready);
    assert_eq!(report.delta_preview_count, execution.execution_record_count);
    assert_eq!(
        report.selected_source_mode,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable
    );
}

#[test]
fn semantic_delta_preview_produces_delta_preview_records() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.delta_preview_count > 0);
    assert!(report.preview_amount_total > 0);
    assert!(!report.delta_records.is_empty());
    assert!(report.runtime_delta_preview_only);
}

#[test]
fn semantic_delta_preview_preserves_execution_kinds() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report
        .delta_records
        .iter()
        .any(|r| r.delta_kind == ParticipantDeltaPreviewKind::RuntimeAppliedAmountDelta));
    assert!(report
        .delta_records
        .iter()
        .any(|r| r.delta_kind == ParticipantDeltaPreviewKind::ResourceSatisfiedDelta));
    assert!(report.delta_records.iter().all(|r| {
        matches!(
            r.delta_kind,
            ParticipantDeltaPreviewKind::ResourceSatisfiedDelta
                | ParticipantDeltaPreviewKind::ResourceShortfallDelta
                | ParticipantDeltaPreviewKind::RuntimeAppliedAmountDelta
        )
    }));
    if report.resource_shortfall_delta_count > 0 {
        assert!(report
            .delta_records
            .iter()
            .any(|r| r.delta_kind == ParticipantDeltaPreviewKind::ResourceShortfallDelta));
    }
}

#[test]
fn semantic_delta_preview_maps_target_property_ids_without_writing_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);

    assert!(report
        .delta_records
        .iter()
        .any(|r| r.target_property_id == RUNTIME_PREVIEW_APPLIED_PROPERTY_ID));
    assert!(report
        .delta_records
        .iter()
        .any(|r| r.target_property_id == RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID));
    assert!(report.delta_records.iter().all(|r| {
        matches!(
            r.target_property_id.as_str(),
            RUNTIME_PREVIEW_APPLIED_PROPERTY_ID
                | RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID
                | RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID
        )
    }));
    assert!(report
        .delta_records
        .iter()
        .all(|r| r.preview_delta_value == f64::from(r.amount)));
}

#[test]
fn semantic_delta_preview_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report
        .delta_records
        .iter()
        .any(|r| r.owner_ref.as_str() == "owner_a"));
    assert!(report
        .delta_records
        .iter()
        .any(|r| r.owner_ref.as_str() == "owner_b"));
}

#[test]
fn semantic_delta_preview_preserves_execution_resource_key_without_authoritative_typed_mutation() {
    let spec = build_sibling_redistribution_spec();
    let recursive = evaluate_recursive_local_rf(&spec).expect("recursive");
    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive);
    assert!(aggregate_rows
        .iter()
        .any(|row| row.resource_key.as_str() == "food"));

    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive delta");
    assert!(report
        .delta_records
        .iter()
        .all(|record| record.resource_key.as_str() == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn semantic_delta_preview_defers_participant_property_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.participant_property_mutation_deferred);
    assert!(report
        .delta_records
        .iter()
        .all(|record| record.participant_property_mutation_deferred));
}

#[test]
fn semantic_delta_preview_defers_scenario_authority_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.scenario_authority_mutation_deferred);
    assert!(report
        .delta_records
        .iter()
        .all(|record| record.scenario_authority_mutation_deferred));
}

#[test]
fn semantic_delta_preview_defers_savefile_and_persistent_history() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.savefile_mutation_deferred);
    assert!(report.persistent_history_deferred);
    assert!(report
        .delta_records
        .iter()
        .all(|record| record.savefile_mutation_deferred));
}

#[test]
fn semantic_delta_preview_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(
        prove_semantic_participant_delta_preview_preserves_authority(
            &spec,
            TICK_ONE,
            ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
            REPLAY_ONE,
        )
        .expect("proof")
    );
}

#[test]
fn semantic_delta_preview_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _report = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}

#[test]
fn normal_tests_do_not_write_semantic_delta_preview_fixture() {
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

    let err = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .unwrap_err();
    assert!(!err.message.is_empty());
}
