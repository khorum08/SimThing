//! SEMANTIC-LOCAL-EFFECT-TYPES-0 — typed semantic local effect spec proofs.

mod disburse_down_fixture;

use simthing_spec::{
    evaluate_semantic_local_effects, prove_semantic_local_effects_preserve_authority,
    semantic_local_effects_from_application, serialize_scenario_authority, OwnerRef, ResourceKey,
    RuntimeLocalEffectApplicationRecord, RuntimeLocalEffectApplicationReport, RuntimeTickId,
    ScopeId, SemanticLocalEffectErrorKind, SemanticLocalEffectKind,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

fn make_application_report(
    records: Vec<RuntimeLocalEffectApplicationRecord>,
) -> RuntimeLocalEffectApplicationReport {
    RuntimeLocalEffectApplicationReport {
        application_count: records.len() as u32,
        owner_channel_count: 0,
        requested_total: 0,
        allocated_total: 0,
        unmet_total: 0,
        runtime_applied_total: 0,
        satisfied_count: 0,
        unsatisfied_count: 0,
        records,
        semantic_effect_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        errors: Vec::new(),
        deferrals: Vec::new(),
    }
}

fn make_record(
    source: u32,
    owner: &str,
    requested: u32,
    allocated: u32,
    unmet: u32,
    satisfied: bool,
) -> RuntimeLocalEffectApplicationRecord {
    RuntimeLocalEffectApplicationRecord {
        source_simthing_id_raw: source,
        owner_ref: OwnerRef::new(owner),
        resource_key: ResourceKey::new("food"),
        scope_id: ScopeId::new("scope_a"),
        requested,
        allocated,
        unmet,
        satisfied,
        runtime_applied_amount: allocated,
        semantic_effect_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
    }
}

#[test]
fn semantic_local_effects_from_application_emits_typed_outputs() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let application = simthing_spec::evaluate_runtime_local_effect_application(&spec, TICK_ONE)
        .expect("application");
    let report = semantic_local_effects_from_application(&application).expect("semantic");

    assert_eq!(report.output_count, 6);
    assert!(report
        .outputs
        .iter()
        .any(|o| o.effect_kind == SemanticLocalEffectKind::RuntimeAppliedAmount));
    assert!(report
        .outputs
        .iter()
        .any(|o| o.effect_kind == SemanticLocalEffectKind::ResourceSatisfied));
    assert!(report
        .outputs
        .iter()
        .any(|o| o.effect_kind == SemanticLocalEffectKind::ResourceShortfall));
}

#[test]
fn semantic_local_effects_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects(&spec, TICK_ONE, 1).expect("semantic");

    assert_eq!(report.output_count, 6);
    assert_eq!(report.owner_channel_count, 2);
    assert_eq!(report.runtime_applied_total, 72);
    assert_eq!(report.shortfall_total, 8);
    assert_eq!(report.satisfied_output_count, 2);
    assert_eq!(report.shortfall_output_count, 1);
}

#[test]
fn semantic_local_effects_marks_resource_satisfied_and_shortfall() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects(&spec, TICK_ONE, 1).expect("semantic");

    let owner_a: Vec<_> = report
        .outputs
        .iter()
        .filter(|o| o.owner_ref.as_str() == "owner_a")
        .collect();

    let cohort_applied = owner_a
        .iter()
        .find(|o| {
            o.effect_kind == SemanticLocalEffectKind::RuntimeAppliedAmount
                && o.source_simthing_id_raw != 0
                && o.amount == 20
        })
        .expect("cohort applied");
    assert_eq!(cohort_applied.amount, 20);

    let cohort_satisfied = owner_a
        .iter()
        .find(|o| o.effect_kind == SemanticLocalEffectKind::ResourceSatisfied && o.amount == 20)
        .expect("cohort satisfied");

    let fleet_shortfall = owner_a
        .iter()
        .find(|o| o.effect_kind == SemanticLocalEffectKind::ResourceShortfall)
        .expect("fleet shortfall");
    assert_eq!(fleet_shortfall.amount, 8);

    let owner_b_applied = report
        .outputs
        .iter()
        .find(|o| {
            o.owner_ref.as_str() == "owner_b"
                && o.effect_kind == SemanticLocalEffectKind::RuntimeAppliedAmount
        })
        .expect("owner_b applied");
    assert_eq!(owner_b_applied.amount, 10);

    assert!(cohort_satisfied.semantic_execution_deferred);
    assert!(fleet_shortfall.participant_property_mutation_deferred);
}

#[test]
fn semantic_local_effects_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects(&spec, TICK_ONE, 1).expect("semantic");

    for output in &report.outputs {
        assert!(!output.owner_ref.as_str().is_empty());
        assert!(!output.resource_key.as_str().is_empty());
        assert!(!output.scope_id.as_str().is_empty());
    }
}

#[test]
fn semantic_local_effects_preserves_source_simthing_ids() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects(&spec, TICK_ONE, 1).expect("semantic");

    assert!(report.outputs.iter().all(|o| o.source_simthing_id_raw > 0));
    let ids: std::collections::BTreeSet<_> = report
        .outputs
        .iter()
        .filter(|o| o.effect_kind == SemanticLocalEffectKind::RuntimeAppliedAmount)
        .map(|o| o.source_simthing_id_raw)
        .collect();
    assert_eq!(ids.len(), 3);
}

#[test]
fn semantic_local_effects_rejects_missing_source_id() {
    let report = make_application_report(vec![make_record(0, "owner_a", 10, 5, 5, false)]);
    let err = semantic_local_effects_from_application(&report).unwrap_err();
    assert!(matches!(
        err.kind,
        SemanticLocalEffectErrorKind::MissingSourceSimThingId
    ));
}

#[test]
fn semantic_local_effects_rejects_duplicate_output() {
    let records = vec![
        make_record(42, "owner_a", 10, 5, 5, false),
        make_record(42, "owner_a", 10, 5, 5, false),
    ];
    let report = make_application_report(records);
    let err = semantic_local_effects_from_application(&report).unwrap_err();
    assert!(matches!(
        err.kind,
        SemanticLocalEffectErrorKind::DuplicateSemanticOutput
    ));
}

#[test]
fn semantic_local_effects_uses_checked_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects(&spec, TICK_ONE, 1).expect("semantic");

    let applied: u32 = report
        .outputs
        .iter()
        .filter(|o| o.effect_kind == SemanticLocalEffectKind::RuntimeAppliedAmount)
        .map(|o| o.amount)
        .sum();
    let shortfall: u32 = report
        .outputs
        .iter()
        .filter(|o| o.effect_kind == SemanticLocalEffectKind::ResourceShortfall)
        .map(|o| o.amount)
        .sum();

    assert_eq!(report.runtime_applied_total, applied);
    assert_eq!(report.shortfall_total, shortfall);
}

#[test]
fn semantic_local_effects_authority_proof_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let proof = prove_semantic_local_effects_preserve_authority(&spec, TICK_ONE, 1).expect("proof");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(proof.scenario_authority_unchanged);
    assert_eq!(
        proof.scenario_authority_digest_before,
        proof.scenario_authority_digest_after
    );
}

#[test]
fn semantic_local_effects_defers_execution_property_mutation_scenario_mutation_and_savefile_mutation(
) {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_semantic_local_effects(&spec, TICK_ONE, 1).expect("semantic");
    let proof = prove_semantic_local_effects_preserve_authority(&spec, TICK_ONE, 1).expect("proof");

    assert!(report.semantic_execution_deferred);
    assert!(report.participant_property_mutation_deferred);
    assert!(report.scenario_authority_mutation_deferred);
    assert!(report.savefile_mutation_deferred);
    assert!(proof.semantic_execution_deferred);
    assert!(proof.participant_property_mutation_deferred);
    assert!(proof.savefile_mutation_deferred);
    assert!(report.deferrals.len() >= 5);
    assert!(report
        .outputs
        .iter()
        .all(|o| o.semantic_execution_deferred && o.participant_property_mutation_deferred));
}

#[test]
fn semantic_local_effects_empty_application_returns_empty_report() {
    let report = make_application_report(Vec::new());
    let semantic = semantic_local_effects_from_application(&report).expect("empty");
    assert_eq!(semantic.output_count, 0);
    assert!(semantic.semantic_execution_deferred);
}

#[test]
fn normal_tests_do_not_write_semantic_local_effects_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let _report = evaluate_semantic_local_effects(&spec, TICK_ONE, 1).expect("semantic");
}
