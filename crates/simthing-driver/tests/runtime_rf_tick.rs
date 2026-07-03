//! RUNTIME-RF-TICK-INTEGRATION-0 — composed runtime RF tick driver proof.

mod disburse_down_fixture;
mod reduce_up_fixture;

use simthing_core::SimThingKind;
use simthing_driver::compile_runtime_rf_tick_plan;
use simthing_spec::{
    serialize_scenario_authority, SpecError, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec;

#[test]
fn runtime_rf_tick_compile_composes_all_stage_plans() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_plan(&spec).expect("compile");

    assert!(!plan.participant_plan.participants.is_empty());
    assert!(!plan.reduce_up_plan.bucket_plans.is_empty());
    assert!(!plan.owner_silo_writeback_plan.cpu_results.is_empty());
    assert!(!plan.owner_silo_disburse_down_plan.cpu_results.is_empty());
    assert_eq!(
        plan.runtime_local_allocation_plan
            .application_report
            .allocation_count,
        3
    );
}

#[test]
fn runtime_rf_tick_compile_preserves_expected_fixture_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_plan(&spec).expect("compile");
    let report = &plan.tick_report;

    assert_eq!(report.local_allocated_total, 72);
    assert_eq!(report.local_unmet_total, 8);
    assert_eq!(report.participant_count, 4);
    assert_eq!(report.reduce_up_bucket_count, 2);
}
#[test]
fn runtime_rf_tick_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _plan = compile_runtime_rf_tick_plan(&spec).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn runtime_rf_tick_compile_has_gpu_proof_stage_plan_summary() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_plan(&spec).expect("compile");
    let summary = &plan.gpu_proof_summary;

    assert!(summary.participant_surplus_plan_ready);
    assert!(summary.participant_deficit_plan_ready);
    assert_eq!(summary.reduce_up_bucket_plan_count, 2);
    assert_eq!(summary.writeback_aggregate_plan_count, 2);
    assert_eq!(summary.disburse_demand_aggregate_plan_count, 2);
    assert_eq!(summary.local_allocation_aggregate_plan_count, 2);
}

#[test]
fn runtime_rf_tick_compile_reuses_existing_accumulator_plans() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_plan(&spec).expect("compile");

    assert_eq!(plan.participant_plan.surplus_plan.ops.len(), 1);
    assert_eq!(plan.participant_plan.deficit_plan.ops.len(), 1);
    for bucket in &plan.reduce_up_plan.bucket_plans {
        assert_eq!(bucket.surplus_plan.ops.len(), 1);
        assert_eq!(bucket.deficit_plan.ops.len(), 1);
    }
}

#[test]
fn runtime_rf_tick_economy_execution_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_plan(&spec).expect("compile");
    assert!(plan.economy_execution_deferred);
    assert!(plan.tick_report.economy_execution_deferred);
}

#[test]
fn runtime_rf_tick_local_effect_application_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_plan(&spec).expect("compile");
    assert!(plan.local_effect_application_deferred);
    assert!(plan.tick_report.local_effect_application_deferred);
}

#[test]
fn runtime_rf_tick_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_rf_tick_plan(&spec).expect("compile");
    assert_eq!(plan.participant_plan.surplus_plan.ops.len(), 1);
    assert_eq!(
        plan.owner_silo_writeback_plan.gpu_aggregate_proof_plans[0]
            .surplus_plan
            .ops
            .len(),
        1
    );
}

#[test]
fn runtime_rf_tick_empty_demands_compiles() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_runtime_rf_tick_plan(&spec).expect("compile empty demands path");
    assert_eq!(plan.tick_report.local_allocation_count, 0);
    assert_eq!(plan.tick_report.disburse_down_result_count, 0);
}
