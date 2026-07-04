//! RUNTIME-TICK-HISTORY-REPLAY-0 — runtime tick history/replay spec proofs.

mod disburse_down_fixture;

use simthing_core::SimThingKind;
use simthing_spec::{
    evaluate_runtime_tick_history_entry, replay_runtime_tick_history, scenario_authority_digest,
    serialize_scenario_authority, RuntimeTickHistoryErrorKind, RuntimeTickId,
    PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

#[test]
fn runtime_tick_history_entry_is_deterministic() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let a = evaluate_runtime_tick_history_entry(&spec, TICK_ONE).expect("entry a");
    let b = evaluate_runtime_tick_history_entry(&spec, TICK_ONE).expect("entry b");

    assert_eq!(a.entry_digest, b.entry_digest);
    assert_eq!(a.scenario_authority_digest, b.scenario_authority_digest);
    assert!(!a.entry_digest.is_empty());
}
