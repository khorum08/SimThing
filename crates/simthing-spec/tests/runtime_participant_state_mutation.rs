//! RUNTIME-PARTICIPANT-STATE-MUTATION-0 — runtime-only participant state mutation proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_recursive_local_rf, evaluate_runtime_participant_state_mutation,
    evaluate_semantic_participant_delta_preview,
    prove_runtime_participant_state_mutation_preserves_authority,
    recursive_local_rf_aggregate_source_rows, replay_runtime_participant_state_mutation,
    serialize_scenario_authority, ParticipantDeltaPreviewSourceMode,
    RuntimeParticipantStateMutationKind, RuntimeParticipantStateMutationSourceMode, RuntimeTickId,
    OWNER_FLOW_DEMAND_PROPERTY_ID, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
    RUNTIME_PREVIEW_APPLIED_PROPERTY_ID, RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID,
    RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;
const REPLAY_THREE: u32 = 3;
#[test]
fn runtime_participant_state_mutation_consumes_recursive_delta_preview_records() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let delta_preview = evaluate_semantic_participant_delta_preview(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_ONE,
    )
    .expect("delta preview");
    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.delta_preview_ready);
    assert_eq!(
        report.mutation_record_count,
        delta_preview.delta_preview_count
    );
    assert_eq!(
        report.selected_source_mode,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable
    );
}

#[test]
fn runtime_participant_state_mutation_produces_before_mutation_and_after_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(!report.before_rows.is_empty());
    assert!(!report.mutation_records.is_empty());
    assert!(!report.after_rows.is_empty());
    assert!(report.runtime_state_mutation_applied);
    assert!(report.before_rows.iter().all(|row| row.value == 0.0));
}

#[test]
fn runtime_participant_state_mutation_applies_deltas_to_runtime_rows_only() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.runtime_state_mutation_applied));
    assert!(report.after_rows.iter().any(|row| row.value > 0.0));
    for record in &report.mutation_records {
        assert_eq!(record.after_value, record.before_value + record.delta_value);
    }
}

#[test]
fn runtime_participant_state_mutation_preserves_kind_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report
        .mutation_records
        .iter()
        .any(|r| r.owner_ref.as_str() == "owner_a"));
    assert!(report
        .mutation_records
        .iter()
        .any(|r| r.owner_ref.as_str() == "owner_b"));
    assert!(report
        .mutation_records
        .iter()
        .any(|r| r.mutation_kind
            == RuntimeParticipantStateMutationKind::ApplyRuntimeAppliedAmountDelta));
}

#[test]
fn runtime_participant_state_mutation_uses_preview_target_property_ids_without_writing_simthing_properties(
) {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);

    assert!(report
        .mutation_records
        .iter()
        .any(|r| r.property_id == RUNTIME_PREVIEW_APPLIED_PROPERTY_ID));
    assert!(report
        .mutation_records
        .iter()
        .any(|r| r.property_id == RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID));
    assert!(report.mutation_records.iter().all(|r| {
        matches!(
            r.property_id.as_str(),
            RUNTIME_PREVIEW_APPLIED_PROPERTY_ID
                | RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID
                | RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID
        )
    }));
}

#[test]
fn runtime_participant_state_mutation_preserves_delta_resource_key_without_authoritative_typed_mutation(
) {
    let spec = build_sibling_redistribution_spec();
    let recursive = evaluate_recursive_local_rf(&spec).expect("recursive");
    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive);
    assert!(aggregate_rows
        .iter()
        .any(|row| row.resource_key.as_str() == "food"));

    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive mutation");
    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.resource_key.as_str() == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn runtime_participant_state_mutation_defers_participant_property_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.participant_property_mutation_deferred);
    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.participant_property_mutation_deferred));
}

#[test]
fn runtime_participant_state_mutation_defers_scenario_authority_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.scenario_authority_mutation_deferred);
    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.scenario_authority_mutation_deferred));
}

#[test]
fn runtime_participant_state_mutation_defers_savefile_and_persistent_history() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.savefile_mutation_deferred);
    assert!(report.persistent_history_deferred);
    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.savefile_mutation_deferred));
}

#[test]
fn runtime_participant_state_mutation_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(
        prove_runtime_participant_state_mutation_preserves_authority(
            &spec,
            TICK_ONE,
            RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
            REPLAY_ONE,
        )
        .expect("proof")
    );
}

#[test]
fn runtime_participant_state_mutation_does_not_mutate_participant_properties() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _report = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}

#[test]
fn runtime_participant_state_mutation_is_deterministic_under_replay() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let replay = replay_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_THREE,
    )
    .expect("replay");

    assert!(replay.replay_deterministic);
    assert_eq!(replay.replay_count, REPLAY_THREE);
    assert!(replay.reference_report.runtime_state_mutation_applied);
}

#[test]
fn normal_tests_do_not_write_runtime_participant_state_mutation_fixture() {
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

    let err = evaluate_runtime_participant_state_mutation(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_ONE,
    )
    .unwrap_err();
    assert!(!err.message.is_empty());
}
