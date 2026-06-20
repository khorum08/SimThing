//! RUNTIME-TICK-EXECUTION-SHELL-0 — runtime tick shell spec proofs.

mod disburse_down_fixture;

use simthing_core::SimThingKind;
use simthing_spec::{
    evaluate_runtime_tick_shell, runtime_tick_shell_stage_order, serialize_scenario_authority,
    RuntimeTickId, RuntimeTickShellErrorKind, RuntimeTickStage, OWNER_SILO_CURRENT_PROPERTY_ID,
    PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn runtime_tick_shell_composes_runtime_rf_tick_report() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_tick_shell(&spec, TICK_ONE).expect("shell");

    assert!(report.runtime_rf_tick_ready);
    assert_eq!(report.participant_count, 4);
    assert_eq!(report.local_allocation_count, 3);
}

#[test]
fn runtime_tick_shell_records_deterministic_stage_order() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_tick_shell(&spec, TICK_ONE).expect("shell");
    let expected = runtime_tick_shell_stage_order();

    assert_eq!(report.stage_order, expected);
    assert_eq!(report.stage_count, 6);
    assert_eq!(
        report.stage_order[0],
        RuntimeTickStage::RuntimeRfTickComposition
    );
    assert_eq!(
        report.stage_order[5],
        RuntimeTickStage::RuntimeLocalAllocation
    );
}

#[test]
fn runtime_tick_shell_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_tick_shell(&spec, TICK_ONE).expect("shell");

    assert_eq!(report.local_allocated_total, 72);
    assert_eq!(report.local_unmet_total, 8);
    assert_eq!(report.reduce_up_bucket_count, 2);
    assert_eq!(report.owner_silo_writeback_count, 2);
    assert_eq!(report.disburse_down_result_count, 2);
}

#[test]
fn runtime_tick_shell_defers_economy_scenario_mutation_and_local_effects() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_tick_shell(&spec, TICK_ONE).expect("shell");

    assert!(report.economy_execution_deferred);
    assert!(report.scenario_authority_mutation_deferred);
    assert!(report.local_effect_application_deferred);
    assert!(report.deferrals.len() >= 3);
}

#[test]
fn runtime_tick_shell_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _report = evaluate_runtime_tick_shell(&spec, TICK_ONE).expect("shell");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn runtime_tick_shell_rejects_invalid_runtime_rf_tick() {
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
        .find(|c| simthing_spec::gridcell_role(c).as_deref() == Some("star_system"))
        .unwrap();
    let planet = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap();
    planet.properties.remove(&PLANET_ID_PROPERTY_ID);

    let err = evaluate_runtime_tick_shell(&spec, TICK_ONE).unwrap_err();
    assert_eq!(err.kind, RuntimeTickShellErrorKind::RuntimeRfTickRejected);
}

#[test]
fn runtime_tick_shell_rejects_invalid_tick_id_if_zero_is_disallowed() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let err = evaluate_runtime_tick_shell(&spec, RuntimeTickId(0)).unwrap_err();
    assert_eq!(err.kind, RuntimeTickShellErrorKind::InvalidTickId);
}

#[test]
fn runtime_tick_shell_no_participant_property_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let owner_before = spec
        .root
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter()
        .find(|c| simthing_spec::owner_entity_id(c).as_deref() == Some("owner_a"))
        .unwrap()
        .properties
        .get(&OWNER_SILO_CURRENT_PROPERTY_ID)
        .cloned();

    let _report = evaluate_runtime_tick_shell(&spec, TICK_ONE).expect("shell");

    let owner_after = spec
        .root
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter()
        .find(|c| simthing_spec::owner_entity_id(c).as_deref() == Some("owner_a"))
        .unwrap()
        .properties
        .get(&OWNER_SILO_CURRENT_PROPERTY_ID)
        .cloned();

    assert_eq!(owner_before, owner_after);
}

#[test]
#[ignore = "manual corpus regeneration only"]
fn runtime_tick_shell_no_new_fixture_writer_in_normal_tests() {
    panic!("fixture writer is ignored in normal test runs");
}
