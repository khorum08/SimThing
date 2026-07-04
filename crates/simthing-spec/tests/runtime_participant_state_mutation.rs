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
