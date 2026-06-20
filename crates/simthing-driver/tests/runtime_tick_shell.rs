//! RUNTIME-TICK-EXECUTION-SHELL-0 — runtime tick shell driver proofs.

mod disburse_down_fixture;

use simthing_core::SimThingKind;
use simthing_driver::compile_runtime_tick_shell_plan;
use simthing_spec::{
    serialize_scenario_authority, RuntimeTickId, SpecError, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn runtime_tick_shell_compile_composes_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");

    assert!(!plan
        .runtime_rf_tick_plan
        .participant_plan
        .participants
        .is_empty());
    assert_eq!(
        plan.runtime_rf_tick_plan.tick_report.local_allocation_count,
        3
    );
}

#[test]
fn runtime_tick_shell_compile_preserves_expected_fixture_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");
    let report = &plan.execution_report;

    assert_eq!(plan.tick_id, TICK_ONE);
    assert_eq!(report.local_allocated_total, 72);
    assert_eq!(report.local_unmet_total, 8);
    assert_eq!(report.stage_count, 6);
}

#[test]
fn runtime_tick_shell_compile_records_stage_local_gpu_proof_summary() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");

    assert!(
        plan.gpu_stage_proof_summary
            .stage_local_gpu_proofs_available
    );
    assert!(plan.execution_report.gpu_stage_proof_available);
}

#[test]
fn runtime_tick_shell_compile_rejects_invalid_earlier_stage() {
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
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .unwrap();
    let planet = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap();
    planet.properties.remove(&PLANET_ID_PROPERTY_ID);

    let err = compile_runtime_tick_shell_plan(&spec, TICK_ONE).unwrap_err();
    assert!(matches!(err, SpecError::ValidationFailed));
}

#[test]
fn runtime_tick_shell_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn runtime_tick_shell_economy_execution_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");
    assert!(plan.economy_execution_deferred);
    assert!(plan.execution_report.economy_execution_deferred);
}

#[test]
fn runtime_tick_shell_local_effect_application_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");
    assert!(plan.local_effect_application_deferred);
    assert!(plan.execution_report.local_effect_application_deferred);
}

#[test]
fn runtime_tick_shell_reports_no_fused_gpu_tick_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");
    assert!(!plan.gpu_stage_proof_summary.fused_tick_kernel_present);
}

#[test]
fn runtime_tick_shell_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");
    assert!(!plan.gpu_stage_proof_summary.new_gpu_primitive_required);
}

#[test]
fn runtime_tick_shell_reuses_existing_accumulator_proof_surfaces() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("compile");
    let rf = &plan.runtime_rf_tick_plan;

    assert_eq!(rf.participant_plan.surplus_plan.ops.len(), 1);
    assert_eq!(rf.gpu_proof_summary.reduce_up_bucket_plan_count, 2);
    assert_eq!(
        rf.gpu_proof_summary.local_allocation_aggregate_plan_count,
        2
    );
}
