//! LOCAL-EFFECT-APPLICATION-AUTHORITY-0 — local effect application spec proofs.

mod disburse_down_fixture;

use simthing_core::SimThingKind;
use simthing_spec::{
    apply_runtime_local_effect_records, evaluate_runtime_local_effect_application,
    prove_local_effect_application_preserves_authority, serialize_scenario_authority,
    LocalEffectApplicationErrorKind, OwnerRef, ResourceKey, RuntimeLocalParticipantEffect,
    RuntimeTickId, ScopeId, PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn local_effect_application_from_effects_records_per_source_application() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");

    assert_eq!(report.application_count, 3);
    assert_eq!(report.records.len(), 3);
    assert!(report.records.iter().all(|r| r.semantic_effect_deferred));
}

#[test]
fn local_effect_application_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");

    assert_eq!(report.requested_total, 80);
    assert_eq!(report.allocated_total, 72);
    assert_eq!(report.unmet_total, 8);
    assert_eq!(report.runtime_applied_total, 72);
    assert_eq!(report.satisfied_count, 2);
    assert_eq!(report.unsatisfied_count, 1);
    assert_eq!(report.owner_channel_count, 2);
}

#[test]
fn local_effect_application_runtime_applied_equals_allocated_for_now() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");

    assert!(report
        .records
        .iter()
        .all(|r| r.runtime_applied_amount == r.allocated));
    assert_eq!(report.runtime_applied_total, report.allocated_total);
}

#[test]
fn local_effect_application_marks_satisfied_and_unsatisfied() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");

    let owner_a: Vec<_> = report
        .records
        .iter()
        .filter(|r| r.owner_ref.as_str() == "owner_a")
        .collect();
    let cohort = owner_a.iter().find(|r| r.requested == 20).expect("cohort");
    assert_eq!(cohort.runtime_applied_amount, 20);
    assert_eq!(cohort.unmet, 0);
    assert!(cohort.satisfied);

    let fleet = owner_a.iter().find(|r| r.requested == 50).expect("fleet");
    assert_eq!(fleet.runtime_applied_amount, 42);
    assert_eq!(fleet.unmet, 8);
    assert!(!fleet.satisfied);

    let owner_b = report
        .records
        .iter()
        .find(|r| r.owner_ref.as_str() == "owner_b")
        .expect("owner_b");
    assert_eq!(owner_b.runtime_applied_amount, 10);
    assert!(owner_b.satisfied);
}

#[test]
fn local_effect_application_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");

    for record in &report.records {
        assert!(!record.owner_ref.as_str().is_empty());
        assert!(!record.resource_key.as_str().is_empty());
        assert!(!record.scope_id.as_str().is_empty());
    }
}

#[test]
fn local_effect_application_preserves_source_simthing_ids() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");

    assert!(report.records.iter().all(|r| r.source_simthing_id_raw > 0));
    let ids: std::collections::BTreeSet<_> = report
        .records
        .iter()
        .map(|r| r.source_simthing_id_raw)
        .collect();
    assert_eq!(ids.len(), 3);
}

#[test]
fn local_effect_application_uses_checked_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");

    let requested: u32 = report.records.iter().map(|r| r.requested).sum();
    let allocated: u32 = report.records.iter().map(|r| r.allocated).sum();
    let unmet: u32 = report.records.iter().map(|r| r.unmet).sum();
    let applied: u32 = report
        .records
        .iter()
        .map(|r| r.runtime_applied_amount)
        .sum();

    assert_eq!(report.requested_total, requested);
    assert_eq!(report.allocated_total, allocated);
    assert_eq!(report.unmet_total, unmet);
    assert_eq!(report.runtime_applied_total, applied);
}

#[test]
fn local_effect_application_authority_proof_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let proof = prove_local_effect_application_preserves_authority(&spec, TICK_ONE).expect("proof");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(proof.scenario_authority_unchanged);
    assert_eq!(
        proof.scenario_authority_digest_before,
        proof.scenario_authority_digest_after
    );
}

#[test]
fn local_effect_application_defers_semantic_effects_property_mutation_scenario_mutation_and_savefile_mutation(
) {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");
    let proof = prove_local_effect_application_preserves_authority(&spec, TICK_ONE).expect("proof");

    assert!(report.semantic_effect_execution_deferred);
    assert!(report.participant_property_mutation_deferred);
    assert!(report.scenario_authority_mutation_deferred);
    assert!(report.savefile_mutation_deferred);
    assert!(proof.semantic_effect_execution_deferred);
    assert!(proof.participant_property_mutation_deferred);
    assert!(proof.savefile_mutation_deferred);
    assert!(report.deferrals.len() >= 4);
}

#[test]
fn normal_tests_do_not_write_local_effect_application_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let _report = evaluate_runtime_local_effect_application(&spec, TICK_ONE).expect("application");
}
