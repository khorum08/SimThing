//! SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 — participant delta preview driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_effect_application_plan, compile_runtime_rf_tick_plan,
    compile_runtime_tick_shell_plan, compile_semantic_local_effects_plan,
    compile_semantic_participant_delta_preview_plan,
};
use simthing_spec::{
    serialize_scenario_authority, ParticipantDeltaPreviewKind, ParticipantDeltaPreviewSourceMode,
    RuntimeTickId,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

#[test]
fn semantic_delta_preview_compile_composes_execution_boundary_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(
        plan.execution_boundary_plan
            .semantic_execution_boundary_proven
    );
    assert!(
        plan.execution_boundary_plan
            .semantic_recursive_source_plan
            .semantic_local_effects_projected_for_selected_source
    );
}

#[test]
fn semantic_delta_preview_compile_recursive_mode_produces_delta_records() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.delta_preview_report.delta_preview_count > 0);
    assert!(plan.delta_preview_report.preview_amount_total > 0);
    assert!(!plan.delta_preview_report.delta_records.is_empty());
    assert!(plan.delta_preview_boundary_proven);
    assert!(plan
        .delta_preview_report
        .delta_records
        .iter()
        .any(|r| r.delta_kind == ParticipantDeltaPreviewKind::RuntimeAppliedAmountDelta));
}

#[test]
fn semantic_delta_preview_compile_does_not_change_default_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
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
fn semantic_delta_preview_compile_does_not_change_default_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
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
fn semantic_delta_preview_compile_does_not_change_default_local_effect_application_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
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
fn semantic_delta_preview_compile_does_not_change_default_semantic_local_effects_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
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
fn semantic_delta_preview_compile_defers_participant_property_mutation() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.participant_property_mutation_deferred);
    assert!(plan.scenario_authority_mutation_deferred);
    assert!(plan.savefile_mutation_deferred);
    assert!(plan.persistent_history_deferred);
}

#[test]
fn semantic_delta_preview_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
    assert!(plan.gpu_residency_doctrine_preserved);
}

#[test]
fn semantic_delta_preview_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn semantic_delta_preview_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _plan = compile_semantic_participant_delta_preview_plan(
        &spec,
        TICK_ONE,
        ParticipantDeltaPreviewSourceMode::RecursiveSemanticExecutionSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}
