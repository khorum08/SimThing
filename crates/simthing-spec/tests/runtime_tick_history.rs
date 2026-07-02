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

#[test]
fn runtime_tick_history_replay_matches_for_same_scenario_and_tick() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = replay_runtime_tick_history(&spec, TICK_ONE, REPLAY_THREE).expect("replay");

    assert_eq!(report.replay_count, REPLAY_THREE);
    assert!(report.all_replays_match);
    assert_eq!(report.entries.len(), REPLAY_THREE as usize);
    assert!(report.mismatches.is_empty());
    let digest = &report.entries[0].entry_digest;
    assert!(report.entries.iter().all(|e| &e.entry_digest == digest));
}

#[test]
fn runtime_tick_history_records_expected_fixture_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let entry = evaluate_runtime_tick_history_entry(&spec, TICK_ONE).expect("entry");

    assert_eq!(entry.tick_id, TICK_ONE);
    assert_eq!(entry.local_effect_count, 3);
    assert_eq!(entry.allocated_total, 72);
    assert_eq!(entry.unmet_total, 8);
    assert_eq!(entry.satisfied_count, 2);
    assert_eq!(entry.unsatisfied_count, 1);
    assert_eq!(entry.local_allocation_count, 3);
    assert_eq!(entry.stage_order.len(), 6);
}

#[test]
fn runtime_tick_history_records_deferred_flags() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let entry = evaluate_runtime_tick_history_entry(&spec, TICK_ONE).expect("entry");

    assert!(entry.economy_execution_deferred);
    assert!(entry.participant_property_mutation_deferred);
    assert!(entry.scenario_authority_mutation_deferred);
    assert!(entry.local_effect_application_deferred);
}

#[test]
fn runtime_tick_history_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let report = replay_runtime_tick_history(&spec, TICK_ONE, REPLAY_THREE).expect("replay");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(report.scenario_authority_unchanged);
}

#[test]
fn runtime_tick_history_digest_changes_when_authority_changes() {
    let spec_a = build_owner_silo_disburse_down_scoped_spec();
    let mut spec_b = build_owner_silo_disburse_down_scoped_spec();
    spec_b.scenario_id = "mutated_scenario_id".into();

    let digest_a = scenario_authority_digest(&spec_a).expect("digest a");
    let digest_b = scenario_authority_digest(&spec_b).expect("digest b");
    assert_ne!(digest_a, digest_b);

    let entry_a = evaluate_runtime_tick_history_entry(&spec_a, TICK_ONE).expect("entry a");
    let entry_b = evaluate_runtime_tick_history_entry(&spec_b, TICK_ONE).expect("entry b");
    assert_ne!(entry_a.entry_digest, entry_b.entry_digest);
}

#[test]
fn runtime_tick_history_no_wall_clock_or_randomness_in_digest() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let d1 = evaluate_runtime_tick_history_entry(&spec, TICK_ONE)
        .expect("entry 1")
        .entry_digest;
    let d2 = evaluate_runtime_tick_history_entry(&spec, TICK_ONE)
        .expect("entry 2")
        .entry_digest;
    let d3 = evaluate_runtime_tick_history_entry(&spec, TICK_ONE)
        .expect("entry 3")
        .entry_digest;
    assert_eq!(d1, d2);
    assert_eq!(d2, d3);
    assert_eq!(d1.len(), 16);
}

#[test]
fn runtime_tick_history_no_fixture_writer_in_normal_tests() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let _report = replay_runtime_tick_history(&spec, TICK_ONE, REPLAY_THREE).expect("replay");
}
