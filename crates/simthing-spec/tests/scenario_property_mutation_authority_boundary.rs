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
