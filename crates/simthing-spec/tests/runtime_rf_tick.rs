//! RUNTIME-RF-TICK-INTEGRATION-0 — composed runtime RF tick boundary proofs.

mod disburse_down_fixture;
mod reduce_up_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_runtime_rf_tick, serialize_scenario_authority, RuntimeRfTickErrorKind,
    OWNER_SILO_CURRENT_PROPERTY_ID, PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec;

#[test]
fn runtime_rf_tick_composes_all_prior_reports() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick(&spec).expect("tick");

    assert!(report.participant_report.total_participant_count > 0);
    assert!(report.reduce_up_report.bucket_count > 0);
    assert_eq!(report.writeback_results.len(), 2);
    assert_eq!(report.disburse_down_results.len(), 2);
    assert_eq!(report.local_allocation_report.allocation_count, 3);
}

#[test]
fn runtime_rf_tick_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick(&spec).expect("tick");

    assert_eq!(report.participant_count, 4);
    assert_eq!(report.reduce_up_bucket_count, 2);
    assert_eq!(report.local_allocated_total, 72);
    assert_eq!(report.local_unmet_total, 8);

    let owner_a = report
        .writeback_results
        .iter()
        .find(|r| r.owner_ref.as_str() == "owner_a")
        .expect("owner_a");
    assert_eq!(owner_a.previous_current, 50);
    assert_eq!(owner_a.next_current, 62);
}

#[test]
fn runtime_rf_tick_stage_ready_flags_are_true_for_valid_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick(&spec).expect("tick");

    assert!(report.participant_admission_ready);
    assert!(report.reduce_up_ready);
    assert!(report.owner_silo_writeback_ready);
    assert!(report.owner_silo_disburse_down_ready);
    assert!(report.runtime_local_allocation_ready);
}

#[test]
fn runtime_rf_tick_defers_economy_scenario_mutation_and_local_effects() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_rf_tick(&spec).expect("tick");

    assert!(report.economy_execution_deferred);
    assert!(report.scenario_authority_mutation_deferred);
    assert!(report.local_effect_application_deferred);
    assert!(report.deferrals.len() >= 3);
}

#[test]
fn runtime_rf_tick_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _report = evaluate_runtime_rf_tick(&spec).expect("tick");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn runtime_rf_tick_handles_empty_demands_deterministically() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let report = evaluate_runtime_rf_tick(&spec).expect("tick");

    assert!(report.owner_silo_disburse_down_ready);
    assert!(report.runtime_local_allocation_ready);
    assert_eq!(report.disburse_down_result_count, 0);
    assert_eq!(report.local_allocation_count, 0);
    assert_eq!(report.local_allocated_total, 0);
}

#[test]
#[ignore = "manual corpus regeneration only"]
fn runtime_rf_tick_no_new_fixture_writer_in_normal_tests() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let json = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/runtime_rf_tick_scoped.simthing-scenario.json");
    std::fs::write(path, json).expect("write");
}
