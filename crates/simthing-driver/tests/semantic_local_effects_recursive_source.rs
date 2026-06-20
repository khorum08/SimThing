//! SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 — recursive RF semantic local effects driver proofs.

#[path = "../../simthing-spec/tests/disburse_down_fixture.rs"]
mod disburse_down_fixture;

use simthing_driver::{
    compile_local_effect_application_plan, compile_runtime_rf_tick_plan,
    compile_runtime_tick_shell_plan, compile_semantic_local_effects_plan,
    compile_semantic_local_effects_recursive_source_plan,
};
use simthing_spec::{
    serialize_scenario_authority, RuntimeTickId, SemanticLocalEffectKind,
    SemanticLocalEffectRfSourceMode,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

#[test]
fn semantic_recursive_source_compile_composes_local_effect_recursive_source_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(
        plan.local_effect_recursive_source_plan
            .local_effect_application_executed_for_selected_source
    );
    assert!(
        plan.local_effect_recursive_source_plan
            .local_allocation_recursive_source_plan
            .local_allocation_executed_for_selected_source
    );
}

#[test]
fn semantic_recursive_source_compile_legacy_default_preserved() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.legacy_default_preserved);
    assert_eq!(
        plan.selected_source_mode,
        SemanticLocalEffectRfSourceMode::LegacyPlanetChildOwnerSilo
    );
}

#[test]
fn semantic_recursive_source_compile_recursive_mode_projects_semantic_local_effects() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.semantic_local_effects_projected_for_selected_source);
    let semantic = plan
        .semantic_report
        .recursive_semantic_report
        .as_ref()
        .expect("semantic");
    assert!(semantic.output_count > 0);
    assert!(semantic.runtime_applied_total > 0);
}

#[test]
fn semantic_recursive_source_compile_preserves_semantic_kinds() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    let semantic = plan
        .semantic_report
        .recursive_semantic_report
        .as_ref()
        .expect("semantic");
    assert!(semantic
        .outputs
        .iter()
        .any(|o| o.effect_kind == SemanticLocalEffectKind::RuntimeAppliedAmount));
    assert!(semantic
        .outputs
        .iter()
        .any(|o| o.effect_kind == SemanticLocalEffectKind::ResourceSatisfied));
    assert!(semantic.outputs.iter().all(|o| {
        matches!(
            o.effect_kind,
            SemanticLocalEffectKind::ResourceSatisfied
                | SemanticLocalEffectKind::ResourceShortfall
                | SemanticLocalEffectKind::RuntimeAppliedAmount
        )
    }));
    if semantic.shortfall_output_count > 0 {
        assert!(semantic
            .outputs
            .iter()
            .any(|o| o.effect_kind == SemanticLocalEffectKind::ResourceShortfall));
    }
}

#[test]
fn semantic_recursive_source_compile_does_not_change_default_runtime_rf_tick_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_rf_tick_plan(&spec).expect("before");
    let _plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
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
fn semantic_recursive_source_compile_does_not_change_default_tick_shell_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = compile_runtime_tick_shell_plan(&spec, TICK_ONE).expect("before");
    let _plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
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
fn semantic_recursive_source_compile_does_not_change_default_local_effect_application_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_local_effect_application_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
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
fn semantic_recursive_source_compile_does_not_change_default_semantic_local_effects_plan() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before =
        compile_semantic_local_effects_plan(&spec, TICK_ONE, REPLAY_THREE).expect("before");
    let _plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
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
fn semantic_recursive_source_compile_defers_semantic_execution() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.semantic_execution_deferred);
    assert!(plan.participant_property_mutation_deferred);
}

#[test]
fn semantic_recursive_source_compile_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(plan.no_new_gpu_primitive_required);
    assert!(plan.gpu_residency_doctrine_preserved);
}

#[test]
fn semantic_recursive_source_compile_reports_no_fused_recursive_rf_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_THREE,
    )
    .expect("compile");

    assert!(!plan.fused_recursive_rf_kernel_present);
}

#[test]
fn semantic_recursive_source_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("before");
    let _plan = compile_semantic_local_effects_recursive_source_plan(
        &spec,
        TICK_ONE,
        SemanticLocalEffectRfSourceMode::RecursiveLocalEffectSelectable,
        REPLAY_THREE,
    )
    .expect("compile");
    let after = serialize_scenario_authority(&spec).expect("after");
    assert_eq!(before, after);
}
