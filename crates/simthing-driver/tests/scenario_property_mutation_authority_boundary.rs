//! SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 — scenario property mutation authority driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_effect_application_plan, compile_runtime_rf_tick_plan,
    compile_runtime_tick_shell_plan, compile_scenario_property_mutation_authority_boundary_plan,
    compile_semantic_local_effects_plan,
};
use simthing_spec::{
    serialize_scenario_authority, RuntimeTickId, ScenarioPropertyMutationSourceMode,
    RUNTIME_PREVIEW_APPLIED_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

#[test]
fn scenario_property_mutation_boundary_compile_composes_runtime_property_view_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(
        plan.runtime_property_mutation_boundary_plan
            .runtime_property_view_mutation_applied
    );
    assert!(
        plan.runtime_property_mutation_boundary_plan
            .runtime_state_mutation_plan
            .runtime_state_mutation_applied
    );
}

#[test]
fn scenario_property_mutation_boundary_compile_recursive_mode_mutates_candidate_only() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("after");

    assert_eq!(before, after);
    assert!(plan.candidate_property_mutation_applied);
    assert!(plan.candidate_scenario_mutated);
    assert!(plan.original_scenario_unchanged);
    assert!(plan.scenario_property_mutation_report.mutation_record_count > 0);
}

#[test]
fn scenario_property_mutation_boundary_compile_does_not_change_default_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = compile_runtime_rf_tick_plan(&spec).expect("after");

    assert_eq!(
        before.tick_report.local_allocated_total,
        after.tick_report.local_allocated_total
    );
}

#[test]
fn scenario_property_mutation_boundary_compile_does_not_change_default_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("after");

    assert_eq!(
        before
            .runtime_rf_tick_plan
            .tick_report
            .local_allocated_total,
        after.runtime_rf_tick_plan.tick_report.local_allocated_total
    );
}

#[test]
fn scenario_property_mutation_boundary_compile_does_not_change_default_local_effect_application_plan(
) {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("after");

    assert_eq!(
        before.application_report.runtime_applied_total,
        after.application_report.runtime_applied_total
    );
}

#[test]
fn scenario_property_mutation_boundary_compile_does_not_change_default_semantic_local_effects_plan()
{
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("after");

    assert_eq!(
        before.semantic_report.runtime_applied_total,
        after.semantic_report.runtime_applied_total
    );
}

#[test]
fn scenario_property_mutation_boundary_compile_defers_savefile_and_persistent_history() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.savefile_mutation_deferred);
    assert!(plan.persistent_history_deferred);
    assert!(plan.input_scenario_property_mutation_deferred);
}

#[test]
fn scenario_property_mutation_boundary_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
    assert!(plan.gpu_residency_doctrine_preserved);
}

#[test]
fn scenario_property_mutation_boundary_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn scenario_property_mutation_boundary_compile_preserves_original_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let plan = compile_scenario_property_mutation_authority_boundary_plan(
        &spec,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("after");

    assert_eq!(before, after);
    assert!(plan.original_scenario_unchanged);
    assert!(plan
        .scenario_property_mutation_report
        .mutation_records
        .iter()
        .any(|r| r.property_id == RUNTIME_PREVIEW_APPLIED_PROPERTY_ID));
}
