//! SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 — cloned candidate ScenarioSpec property mutation proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_recursive_local_rf, evaluate_runtime_participant_property_mutation_boundary,
    evaluate_scenario_property_mutation_authority_boundary,
    prove_scenario_property_mutation_boundary_preserves_original_authority,
    recursive_local_rf_aggregate_source_rows, replay_scenario_property_mutation_authority_boundary,
    serialize_scenario_authority, RuntimeParticipantPropertyMutationSourceMode, RuntimeTickId,
    ScenarioPropertyMutationSourceMode, OWNER_FLOW_DEMAND_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY, RUNTIME_PREVIEW_APPLIED_PROPERTY_ID,
    RUNTIME_PREVIEW_APPLIED_SIM_PROPERTY_ID, RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID,
    RUNTIME_PREVIEW_SATISFIED_SIM_PROPERTY_ID, RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;
const REPLAY_THREE: u32 = 3;

#[test]
fn scenario_property_mutation_boundary_preserves_legacy_default() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo,
        REPLAY_ONE,
    )
    .expect("legacy");

    assert_eq!(
        report.selected_source_mode,
        ScenarioPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo
    );
    assert!(report.selection_allowed);
    assert!(!report.candidate_property_mutation_applied);
}

#[test]
fn scenario_property_mutation_boundary_consumes_runtime_property_view_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let property_view = evaluate_runtime_participant_property_mutation_boundary(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_ONE,
    )
    .expect("property view");
    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.runtime_property_view_ready);
    assert_eq!(
        report.mutation_record_count,
        property_view.mutation_record_count
    );
    assert_eq!(
        report.selected_source_mode,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable
    );
}

#[test]
fn scenario_property_mutation_boundary_mutates_candidate_only() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");
    let after = serialize_scenario_authority(&spec).expect("after");

    assert_eq!(before, after);
    assert!(report.candidate_property_mutation_applied);
    assert!(report.candidate_scenario_mutated);
    assert!(report.input_scenario_property_mutation_deferred);
}

#[test]
fn scenario_property_mutation_boundary_preserves_original_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    assert!(
        prove_scenario_property_mutation_boundary_preserves_original_authority(
            &spec,
            TICK_ONE,
            ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
            REPLAY_ONE,
        )
        .expect("proof")
    );
}

#[test]
fn scenario_property_mutation_boundary_candidate_digest_changes_when_records_exist() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.mutation_record_count > 0);
    assert_ne!(
        report.original_before_authority_digest,
        report.candidate_after_authority_digest
    );
    assert_eq!(
        report.original_before_authority_digest,
        report.original_after_authority_digest
    );
}

#[test]
fn scenario_property_mutation_boundary_records_before_runtime_and_after_values() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(!report.mutation_records.is_empty());
    for record in &report.mutation_records {
        assert!(record.before_value.is_none() || record.before_value.unwrap().is_finite());
        assert!(record.runtime_property_view_value.is_finite());
        assert!(record.candidate_after_value.is_finite());
        assert_eq!(
            record.candidate_after_value,
            record.runtime_property_view_value
        );
        assert!(record.candidate_property_mutation_applied);
    }
}

#[test]
fn scenario_property_mutation_boundary_uses_preview_property_ids_in_candidate_only() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
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
    assert!(report.mutation_records.iter().all(|r| matches!(
        map_record_to_sim_property_id(&r.property_id),
        RUNTIME_PREVIEW_APPLIED_SIM_PROPERTY_ID
            | RUNTIME_PREVIEW_SATISFIED_SIM_PROPERTY_ID
            | simthing_spec::RUNTIME_PREVIEW_SHORTFALL_SIM_PROPERTY_ID
    )));
}

fn map_record_to_sim_property_id(property_id: &str) -> simthing_core::SimPropertyId {
    match property_id {
        RUNTIME_PREVIEW_APPLIED_PROPERTY_ID => RUNTIME_PREVIEW_APPLIED_SIM_PROPERTY_ID,
        RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID => RUNTIME_PREVIEW_SATISFIED_SIM_PROPERTY_ID,
        RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID => {
            simthing_spec::RUNTIME_PREVIEW_SHORTFALL_SIM_PROPERTY_ID
        }
        _ => panic!("unexpected property id"),
    }
}

#[test]
fn scenario_property_mutation_boundary_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report
        .mutation_records
        .iter()
        .any(|r| r.owner_ref == "owner_a"));
    assert!(report
        .mutation_records
        .iter()
        .any(|r| r.owner_ref == "owner_b"));
    assert!(report
        .mutation_records
        .iter()
        .any(|r| r.property_id == RUNTIME_PREVIEW_APPLIED_PROPERTY_ID));
}

#[test]
fn scenario_property_mutation_boundary_preserves_resource_key_without_authoritative_typed_channels()
{
    let spec = build_sibling_redistribution_spec();
    let recursive = evaluate_recursive_local_rf(&spec).expect("recursive");
    let aggregate_rows = recursive_local_rf_aggregate_source_rows(&recursive);
    assert!(aggregate_rows.iter().any(|row| row.resource_key == "food"));

    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive scenario property mutation");
    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.resource_key == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn scenario_property_mutation_boundary_defers_savefile_and_persistent_history() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .expect("recursive");

    assert!(report.savefile_mutation_deferred);
    assert!(report.persistent_history_deferred);
    assert!(report.input_scenario_property_mutation_deferred);
    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.savefile_mutation_deferred));
    assert!(report
        .mutation_records
        .iter()
        .all(|record| record.persistent_history_deferred));
}

#[test]
fn scenario_property_mutation_boundary_is_deterministic_under_replay() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let replay = replay_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("replay");

    assert!(replay.replay_deterministic);
    assert_eq!(replay.replay_count, REPLAY_THREE);
    assert!(replay.reference_report.candidate_property_mutation_applied);
}

#[test]
fn normal_tests_do_not_write_scenario_property_mutation_boundary_fixture() {
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

    let err = evaluate_scenario_property_mutation_authority_boundary(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .unwrap_err();
    assert!(!err.message.is_empty());
}
