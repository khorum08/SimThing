//! RECURSIVE-LOCAL-RF-EVALUATOR-0 — recursive local RF spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_participant_owner_flow_metadata, apply_participant_owner_flow_resource_key_metadata,
    evaluate_planet_child_rf_admission, evaluate_recursive_local_rf, owner_flow_resource_key,
    prove_recursive_local_rf_preserves_authority, recursive_local_rf_aggregate_source_rows,
    recursive_local_rf_report_matches_planet_child_compatibility_slice,
    scenario_metadata_u32_value, serialize_scenario_authority, RecursiveLocalRfAggregateSourceKind,
    RecursiveLocalRfErrorKind, OWNER_FLOW_DEFAULT_RESOURCE_KEY, OWNER_FLOW_OWNER_REF_PROPERTY_ID,
    OWNER_FLOW_SURPLUS_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::build_sibling_redistribution_spec;

fn arena_for_location<'a>(
    report: &'a simthing_spec::RecursiveLocalRfEvaluationReport,
    location_id_raw: u32,
) -> &'a simthing_spec::LocationRfArenaReport {
    report
        .arena_reports
        .iter()
        .find(|arena| arena.location_id_raw == location_id_raw)
        .expect("arena")
}

#[test]
fn recursive_local_rf_treats_location_gridcells_as_evaluator_arenas() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");

    assert!(report.location_count >= 4);
    assert!(report
        .arena_reports
        .iter()
        .all(|arena| arena.location_id_raw > 0));
}

#[test]
fn recursive_local_rf_collects_direct_non_location_participants() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");

    let has_cohort = report.arena_reports.iter().any(|arena| {
        arena
            .participant_rows
            .iter()
            .any(|row| row.participant_kind_label.contains("Cohort"))
    });
    assert!(has_cohort);
}

#[test]
fn recursive_local_rf_collects_location_node_participant_metadata() {
    let mut spec = build_sibling_redistribution_spec();
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
    apply_participant_owner_flow_metadata(planet, "owner_a", 5, 0);
    apply_participant_owner_flow_resource_key_metadata(planet, "food");
    let planet_id_raw = planet.id.raw();

    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let arena = arena_for_location(&report, planet_id_raw);
    assert!(arena
        .participant_rows
        .iter()
        .any(|row| { row.source_simthing_id_raw == planet_id_raw && row.surplus == 5 }));
}

#[test]
fn recursive_local_rf_collects_child_location_outputs() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let star_id = sibling_redistribution_fixture::star_system_id_raw(&spec);

    let star_arena = arena_for_location(&report, star_id);
    assert!(!star_arena.child_outputs.is_empty());
}

#[test]
fn recursive_local_rf_matches_sibling_surplus_to_sibling_deficit_before_bubbling_up() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let star_id = sibling_redistribution_fixture::star_system_id_raw(&spec);
    let settlement = arena_for_location(&report, star_id)
        .settlements
        .iter()
        .find(|s| s.owner_ref == "owner_a" && s.resource_key == "food")
        .expect("food settlement");

    assert_eq!(settlement.locally_matched_total, 20);
    assert!(settlement.child_surplus_total >= 30);
    assert!(settlement.child_deficit_total >= 20);
}

#[test]
fn recursive_local_rf_bubbles_only_net_surplus_or_deficit_to_parent() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let star_id = sibling_redistribution_fixture::star_system_id_raw(&spec);
    let settlement = arena_for_location(&report, star_id)
        .settlements
        .iter()
        .find(|s| s.owner_ref == "owner_a" && s.resource_key == "food")
        .expect("food settlement");

    assert_eq!(settlement.net_surplus_to_parent, 10);
    assert_eq!(settlement.net_deficit_to_parent, 0);
}

#[test]
fn recursive_local_rf_supports_explicit_resource_key_metadata() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");

    assert!(report.arena_reports.iter().any(|arena| {
        arena
            .participant_rows
            .iter()
            .any(|row| row.resource_key == "food")
    }));
}

#[test]
fn recursive_local_rf_preserves_generic_resource_key_fallback() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");

    assert!(report.arena_reports.iter().any(|arena| {
        arena
            .participant_rows
            .iter()
            .any(|row| row.resource_key == OWNER_FLOW_DEFAULT_RESOURCE_KEY)
    }));
}

#[test]
fn recursive_local_rf_preserves_owner_channel_metadata_not_spatial_parentage() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");

    for arena in &report.arena_reports {
        for row in &arena.participant_rows {
            assert!(!row.owner_ref.is_empty());
            assert_ne!(row.owner_ref, row.parent_location_id_raw.to_string());
        }
    }
}

#[test]
fn recursive_local_rf_preserves_previous_planet_child_rf_ladder_outputs() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let admission = evaluate_planet_child_rf_admission(&spec);
    assert_ne!(
        admission.classification,
        simthing_spec::PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(admission.total_participant_count >= 3);

    let compatibility =
        recursive_local_rf_report_matches_planet_child_compatibility_slice(&spec).expect("compat");
    assert!(compatibility.previous_rf_ladder_preserved);
    assert_eq!(
        compatibility.planet_child_participants_found_in_recursive,
        compatibility.planet_child_participant_count
    );
}

#[test]
fn recursive_local_rf_preserves_owner_silo_disburse_down_fixture_behavior() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let compatibility =
        recursive_local_rf_report_matches_planet_child_compatibility_slice(&spec).expect("compat");
    assert!(compatibility.owner_silo_fixture_compatible);
}

#[test]
fn recursive_local_rf_uses_checked_totals() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");

    for arena in &report.arena_reports {
        for settlement in &arena.settlements {
            assert_eq!(
                settlement.total_surplus,
                settlement
                    .direct_surplus_total
                    .saturating_add(settlement.child_surplus_total)
            );
            assert_eq!(
                settlement.total_demand,
                settlement
                    .direct_demand_total
                    .saturating_add(settlement.child_deficit_total)
            );
            assert_eq!(
                settlement.locally_matched_total,
                settlement.total_surplus.min(settlement.total_demand)
            );
        }
    }
}

#[test]
fn recursive_local_rf_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let proof = prove_recursive_local_rf_preserves_authority(&spec).expect("proof");
    let after = serialize_scenario_authority(&spec).expect("serialize");

    assert_eq!(before, after);
    assert!(proof.scenario_authority_unchanged);
}

#[test]
fn recursive_local_rf_rejects_missing_owner_for_active_participant() {
    let mut participant = SimThing::new(SimThingKind::Cohort, 42);
    participant.add_property(
        OWNER_FLOW_SURPLUS_PROPERTY_ID,
        scenario_metadata_u32_value(10),
    );
    let mut spec = build_owner_silo_disburse_down_scoped_spec();
    let planet = spec
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
        .unwrap()
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap();
    planet.add_child(participant);

    let err = evaluate_recursive_local_rf(&spec).unwrap_err();
    assert!(matches!(
        err.kind,
        RecursiveLocalRfErrorKind::MissingOwnerChannelForActiveParticipant
    ));
}

#[test]
fn recursive_local_rf_sibling_fixture_matches_expected_food_totals() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let star_id = sibling_redistribution_fixture::star_system_id_raw(&spec);
    let settlement = arena_for_location(&report, star_id)
        .settlements
        .iter()
        .find(|s| s.owner_ref == "owner_a" && s.resource_key == "food")
        .expect("food settlement");

    assert_eq!(settlement.locally_matched_total, 20);
    assert_eq!(settlement.net_surplus_to_parent, 10);
    assert_eq!(settlement.net_deficit_to_parent, 0);
}

#[test]
fn recursive_local_rf_explicit_resource_key_helper_falls_back_to_generic() {
    let node = SimThing::new(SimThingKind::Cohort, 0);
    assert_eq!(
        owner_flow_resource_key(&node),
        OWNER_FLOW_DEFAULT_RESOURCE_KEY
    );
}

#[test]
fn normal_tests_do_not_write_recursive_local_rf_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let _report = evaluate_recursive_local_rf(&spec).expect("recursive");
}

#[test]
fn recursive_local_rf_aggregate_sources_include_direct_participants() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let sources = recursive_local_rf_aggregate_source_rows(&report);

    assert!(sources.iter().any(|row| {
        row.source_kind == RecursiveLocalRfAggregateSourceKind::DirectParticipant && row.surplus > 0
    }));
}

#[test]
fn recursive_local_rf_aggregate_sources_include_child_location_outputs() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let sources = recursive_local_rf_aggregate_source_rows(&report);

    assert!(sources.iter().any(|row| {
        row.source_kind == RecursiveLocalRfAggregateSourceKind::ChildLocationOutput
    }));
}

#[test]
fn recursive_local_rf_aggregate_source_totals_match_settlement_totals() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let sources = recursive_local_rf_aggregate_source_rows(&report);

    for arena in &report.arena_reports {
        for settlement in &arena.settlements {
            let surplus_sum = sources
                .iter()
                .filter(|row| {
                    row.arena_location_id_raw == settlement.arena_location_id_raw
                        && row.owner_ref == settlement.owner_ref
                        && row.resource_key == settlement.resource_key
                })
                .try_fold(0u32, |acc, row| acc.checked_add(row.surplus))
                .expect("surplus overflow");
            let demand_sum = sources
                .iter()
                .filter(|row| {
                    row.arena_location_id_raw == settlement.arena_location_id_raw
                        && row.owner_ref == settlement.owner_ref
                        && row.resource_key == settlement.resource_key
                })
                .try_fold(0u32, |acc, row| acc.checked_add(row.demand))
                .expect("demand overflow");

            assert_eq!(surplus_sum, settlement.total_surplus);
            assert_eq!(demand_sum, settlement.total_demand);
        }
    }
}

#[test]
fn recursive_local_rf_aggregate_sources_are_gpu_table_compatible() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let sources = recursive_local_rf_aggregate_source_rows(&report);

    assert!(!sources.is_empty());
    for row in &sources {
        assert!(row.arena_location_id_raw > 0);
        assert!(!row.owner_ref.is_empty());
        assert!(!row.resource_key.is_empty());
    }
    for window in sources.windows(2) {
        let a = &window[0];
        let b = &window[1];
        assert!(
            (
                a.arena_location_id_raw,
                &a.owner_ref,
                &a.resource_key,
                a.source_kind,
                a.source_simthing_or_location_id_raw,
            ) <= (
                b.arena_location_id_raw,
                &b.owner_ref,
                &b.resource_key,
                b.source_kind,
                b.source_simthing_or_location_id_raw,
            )
        );
    }
}

#[test]
fn recursive_local_rf_aggregate_sources_preserve_resource_key() {
    let spec = build_sibling_redistribution_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let sources = recursive_local_rf_aggregate_source_rows(&report);

    assert!(sources.iter().any(|row| row.resource_key == "food"));
}

#[test]
fn recursive_local_rf_aggregate_sources_preserve_generic_fallback() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_recursive_local_rf(&spec).expect("recursive");
    let sources = recursive_local_rf_aggregate_source_rows(&report);

    assert!(sources
        .iter()
        .any(|row| row.resource_key == OWNER_FLOW_DEFAULT_RESOURCE_KEY));
}
