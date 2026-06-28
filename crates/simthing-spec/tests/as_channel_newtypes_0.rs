//! AS-CHANNEL-NEWTYPES-0 — behavior-preservation and admission proofs for RF channel identity.

mod reduce_up_fixture;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_participant_owner_flow_metadata, evaluate_planet_child_rf_admission,
    evaluate_planet_child_rf_reduce_up, gridcell_role, is_planet_gridcell,
    PlanetChildRfAdmissionClassification, PlanetChildRfAdmissionErrorKind,
    SimThingScenarioSpec, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

fn terra_prime_cohort(spec: &mut SimThingScenarioSpec) -> &mut SimThing {
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
        .find(|c| gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM))
        .unwrap();
    let terra = star
        .children
        .iter_mut()
        .find(|c| is_planet_gridcell(c))
        .unwrap();
    terra
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_surface_gridcell(c))
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Cohort)
        .unwrap()
}

#[test]
fn planet_child_rf_scope_key_groups_equivalently_after_newtypes() {
    let spec = reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec();
    let admission = evaluate_planet_child_rf_admission(&spec);
    assert_eq!(
        admission.classification,
        PlanetChildRfAdmissionClassification::PartiallyAdmitted
    );
    assert_eq!(admission.total_participant_count, 4);
    assert_eq!(admission.owner_channel_count, 2);

    let reduce_up = evaluate_planet_child_rf_reduce_up(&spec);
    assert_eq!(reduce_up.bucket_count, 2);
    assert_eq!(reduce_up.participant_count, 4);
    assert_eq!(reduce_up.surplus_total, 27);
    assert_eq!(reduce_up.deficit_total, 10);

    let owners: Vec<_> = reduce_up
        .buckets
        .iter()
        .map(|b| b.scope.owner_ref.as_str())
        .collect();
    assert!(owners.contains(&"owner_a"));
    assert!(owners.contains(&"owner_b"));
    assert!(reduce_up
        .buckets
        .iter()
        .all(|b| b.scope.resource_key.as_str() == PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY));
}

#[test]
fn planet_child_rf_empty_owner_ref_still_rejects() {
    let mut spec = reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec();
    let cohort = terra_prime_cohort(&mut spec);
    apply_participant_owner_flow_metadata(cohort, "   ", 1, 0);

    let report = evaluate_planet_child_rf_admission(&spec);
    assert_eq!(
        report.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(report.errors.iter().any(|e| {
        e.kind == PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant
            && e.message.contains("empty")
    }));
}

#[test]
fn planet_child_rf_unknown_owner_ref_still_rejects() {
    let mut spec = reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec();
    let cohort = terra_prime_cohort(&mut spec);
    apply_participant_owner_flow_metadata(cohort, "unknown_owner", 1, 0);

    let report = evaluate_planet_child_rf_admission(&spec);
    assert_eq!(
        report.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(report.errors.iter().any(|e| {
        e.kind == PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant
            && e.message.contains("unknown owner/channel reference")
    }));
}
