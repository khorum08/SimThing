//! RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 — property mutation boundary driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_effect_application_plan,
    compile_runtime_participant_property_mutation_boundary_plan, compile_runtime_rf_tick_plan,
    compile_runtime_tick_shell_plan, compile_semantic_local_effects_plan,
};
use simthing_spec::{
    serialize_scenario_authority, RuntimeParticipantPropertyMutationSourceMode, RuntimeTickId,
    RUNTIME_PREVIEW_APPLIED_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

#[test]
fn runtime_property_mutation_boundary_compile_composes_runtime_state_mutation_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(
        plan.runtime_state_mutation_plan
            .runtime_state_mutation_applied
    );
    assert!(
        plan.runtime_state_mutation_plan
            .delta_preview_plan
            .delta_preview_boundary_proven
    );
}

#[test]
fn runtime_property_mutation_boundary_compile_recursive_mode_applies_property_view_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.property_mutation_boundary_report.mutation_record_count > 0);
    assert!(!plan
        .property_mutation_boundary_report
        .after_property_view_rows
        .is_empty());
    assert!(plan.runtime_property_view_mutation_applied);
    assert!(plan
        .property_mutation_boundary_report
        .after_property_view_rows
        .iter()
        .any(|row| row.property_id == RUNTIME_PREVIEW_APPLIED_PROPERTY_ID));
}

#[test]
fn runtime_property_mutation_boundary_compile_does_not_change_default_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
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
fn runtime_property_mutation_boundary_compile_does_not_change_default_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
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
fn runtime_property_mutation_boundary_compile_does_not_change_default_local_effect_application_plan(
) {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
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
fn runtime_property_mutation_boundary_compile_does_not_change_default_semantic_local_effects_plan()
{
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
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
fn runtime_property_mutation_boundary_compile_defers_scenario_simthing_property_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.scenario_simthing_property_mutation_deferred);
    assert!(plan.scenario_authority_mutation_deferred);
    assert!(plan.savefile_mutation_deferred);
    assert!(plan.persistent_history_deferred);
}

#[test]
fn runtime_property_mutation_boundary_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
    assert!(plan.gpu_residency_doctrine_preserved);
}

#[test]
fn runtime_property_mutation_boundary_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn runtime_property_mutation_boundary_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _plan = compile_runtime_participant_property_mutation_boundary_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}
