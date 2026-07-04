//! RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 — runtime property view mutation boundary proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_recursive_local_rf, evaluate_runtime_participant_property_mutation_boundary,
    evaluate_runtime_participant_state_mutation,
    prove_runtime_participant_property_mutation_boundary_preserves_authority,
    recursive_local_rf_aggregate_source_rows,
    replay_runtime_participant_property_mutation_boundary, serialize_scenario_authority,
    RuntimeParticipantPropertyMutationSourceMode, RuntimeParticipantStateMutationSourceMode,
    RuntimeTickId, OWNER_FLOW_DEMAND_PROPERTY_ID, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
    RUNTIME_PREVIEW_APPLIED_PROPERTY_ID, RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID,
    RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;
const REPLAY_THREE: u32 = 3;
#[test]
fn runtime_property_mutation_boundary_is_deterministic_under_replay() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let replay = replay_runtime_participant_property_mutation_boundary(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_THREE,
    )
    .expect("replay");

    assert!(replay.replay_deterministic);
    assert_eq!(replay.replay_count, REPLAY_THREE);
    assert!(
        replay
            .reference_report
            .runtime_property_view_mutation_applied
    );
}
