//! RUNTIME-PARTICIPANT-STATE-MUTATION-0 — runtime participant state mutation driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_effect_application_plan, compile_runtime_participant_state_mutation_plan,
    compile_runtime_rf_tick_plan, compile_runtime_tick_shell_plan,
    compile_semantic_local_effects_plan,
};
use simthing_spec::{
    serialize_scenario_authority, RuntimeParticipantStateMutationKind,
    RuntimeParticipantStateMutationSourceMode, RuntimeTickId,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

#[test]
fn runtime_participant_state_mutation_compile_composes_delta_preview_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.delta_preview_plan.delta_preview_boundary_proven);
    assert!(
        plan.delta_preview_plan
            .execution_boundary_plan
            .semantic_execution_boundary_proven
    );
}

#[test]
fn runtime_participant_state_mutation_compile_recursive_mode_applies_runtime_state_rows() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.mutation_report.mutation_record_count > 0);
    assert!(!plan.mutation_report.after_rows.is_empty());
    assert!(plan.runtime_state_mutation_applied);
    assert!(plan
        .mutation_report
        .mutation_records
        .iter()
        .any(|r| r.mutation_kind
            == RuntimeParticipantStateMutationKind::ApplyRuntimeAppliedAmountDelta));
}

#[test]
fn runtime_participant_state_mutation_compile_does_not_change_default_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
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
fn runtime_participant_state_mutation_compile_does_not_change_default_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
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
fn runtime_participant_state_mutation_compile_does_not_change_default_local_effect_application_plan(
) {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
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
fn runtime_participant_state_mutation_compile_does_not_change_default_semantic_local_effects_plan()
{
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
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
fn runtime_participant_state_mutation_compile_defers_participant_property_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.participant_property_mutation_deferred);
    assert!(plan.scenario_authority_mutation_deferred);
    assert!(plan.savefile_mutation_deferred);
    assert!(plan.persistent_history_deferred);
}

#[test]
fn runtime_participant_state_mutation_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
    assert!(plan.gpu_residency_doctrine_preserved);
}

#[test]
fn runtime_participant_state_mutation_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn runtime_participant_state_mutation_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _plan = compile_runtime_participant_state_mutation_plan(
        &spec,
        TICK_ONE,
        RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}
