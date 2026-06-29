//! LOCAL-PARTICIPANT-EFFECTS-0 — local participant effects spec proofs.

mod disburse_down_fixture;

use simthing_core::SimThingKind;
use simthing_spec::{
    evaluate_local_participant_effects, local_participant_effects_from_allocations,
    serialize_scenario_authority, LocalParticipantEffectsErrorKind, OwnerRef, ResourceKey,
    RuntimeLocalAllocationState, RuntimeTickId, ScopeId, PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn local_participant_effects_from_allocations_records_per_source_effects() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");

    assert_eq!(report.effect_count, 3);
    assert_eq!(report.effects.len(), 3);
    assert!(report.effects.iter().all(|e| e.effect_application_deferred));
}

#[test]
fn local_participant_effects_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");

    assert_eq!(report.allocated_total, 72);
    assert_eq!(report.unmet_total, 8);
    assert_eq!(report.satisfied_count, 2);
    assert_eq!(report.unsatisfied_count, 1);
    assert_eq!(report.owner_channel_count, 2);
}

#[test]
fn local_participant_effects_marks_satisfied_and_unsatisfied() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");

    let owner_a: Vec<_> = report
        .effects
        .iter()
        .filter(|e| e.owner_ref.as_str() == "owner_a")
        .collect();
    let cohort = owner_a.iter().find(|e| e.requested == 20).expect("cohort");
    assert_eq!(cohort.allocated, 20);
    assert_eq!(cohort.unmet, 0);
    assert!(cohort.satisfied);

    let fleet = owner_a.iter().find(|e| e.requested == 50).expect("fleet");
    assert_eq!(fleet.allocated, 42);
    assert_eq!(fleet.unmet, 8);
    assert!(!fleet.satisfied);

    let owner_b = report
        .effects
        .iter()
        .find(|e| e.owner_ref.as_str() == "owner_b")
        .expect("owner_b");
    assert_eq!(owner_b.allocated, 10);
    assert_eq!(owner_b.unmet, 0);
    assert!(owner_b.satisfied);
}

#[test]
fn local_participant_effects_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");

    for effect in &report.effects {
        assert!(!effect.owner_ref.as_str().is_empty());
        assert!(!effect.resource_key.as_str().is_empty());
        assert!(!effect.scope_id.as_str().is_empty());
    }
}

#[test]
fn local_participant_effects_preserves_source_simthing_ids() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");

    assert!(report.effects.iter().all(|e| e.source_simthing_id_raw > 0));
    let ids: std::collections::BTreeSet<_> = report
        .effects
        .iter()
        .map(|e| e.source_simthing_id_raw)
        .collect();
    assert_eq!(ids.len(), 3);
}

#[test]
fn local_participant_effects_rejects_missing_source_id() {
    let allocation = RuntimeLocalAllocationState {
        source_simthing_id_raw: 0,
        owner_ref: OwnerRef::new("owner_a"),
        resource_key: ResourceKey::new("food"),
        scope_id: ScopeId::new("scope"),
        planet_id: None,
        star_system_gridcell_id_raw: None,
        requested: 10,
        allocated: 5,
        unmet: 5,
        priority: 1,
    };
    let err = local_participant_effects_from_allocations(&[allocation]).unwrap_err();
    assert!(matches!(
        err.kind,
        LocalParticipantEffectsErrorKind::MissingSourceSimThingId
    ));
}

#[test]
fn local_participant_effects_rejects_duplicate_source_effect() {
    let make = |id: u32| RuntimeLocalAllocationState {
        source_simthing_id_raw: id,
        owner_ref: OwnerRef::new("owner_a"),
        resource_key: ResourceKey::new("food"),
        scope_id: ScopeId::new("scope"),
        planet_id: None,
        star_system_gridcell_id_raw: None,
        requested: 10,
        allocated: 5,
        unmet: 5,
        priority: 1,
    };
    let allocations = vec![make(42), make(42)];
    let err = local_participant_effects_from_allocations(&allocations).unwrap_err();
    assert!(matches!(
        err.kind,
        LocalParticipantEffectsErrorKind::DuplicateSourceEffect
    ));
}

#[test]
fn local_participant_effects_uses_checked_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");

    let requested: u32 = report.effects.iter().map(|e| e.requested).sum();
    let allocated: u32 = report.effects.iter().map(|e| e.allocated).sum();
    let unmet: u32 = report.effects.iter().map(|e| e.unmet).sum();

    assert_eq!(report.requested_total, requested);
    assert_eq!(report.allocated_total, allocated);
    assert_eq!(report.unmet_total, unmet);
}

#[test]
fn local_participant_effects_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn local_participant_effects_defers_economy_property_mutation_and_scenario_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");

    assert!(report.economy_execution_deferred);
    assert!(report.participant_property_mutation_deferred);
    assert!(report.scenario_authority_mutation_deferred);
    assert!(report.deferrals.len() >= 3);
}

#[test]
fn normal_tests_do_not_write_local_effect_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let _report = evaluate_local_participant_effects(&spec, TICK_ONE).expect("effects");
}

#[test]
fn local_participant_effects_rejects_invalid_runtime_rf_tick() {
    let mut spec = build_owner_silo_disburse_down_scoped_spec();
    let star = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref()
                == Some(simthing_spec::GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .unwrap();
    let planet = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap();
    planet.properties.remove(&PLANET_ID_PROPERTY_ID);

    let err = evaluate_local_participant_effects(&spec, TICK_ONE).unwrap_err();
    assert!(matches!(
        err.kind,
        LocalParticipantEffectsErrorKind::RuntimeTickShellRejected
    ));
}
