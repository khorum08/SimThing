//! RUNTIME-TICK-HISTORY-REPLAY-0 — runtime tick history/replay driver proofs.

mod disburse_down_fixture;

use simthing_core::SimThingKind;
use simthing_driver::compile_runtime_tick_history_plan;
use simthing_spec::{
    serialize_scenario_authority, RuntimeTickId, SpecError, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_THREE: u32 = 3;

#[test]
fn runtime_tick_history_compile_composes_tick_shell_and_effects_plans() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert_eq!(plan.tick_id, TICK_ONE);
    assert_eq!(plan.tick_shell_plan.tick_id, TICK_ONE);
    assert_eq!(
        plan.local_participant_effects_plan
            .effects_report
            .effect_count,
        3
    );
    assert!(!plan.history_entry.entry_digest.is_empty());
}

#[test]
fn runtime_tick_history_compile_replay_matches() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan.replay_report.all_replays_match);
    assert_eq!(plan.replay_report.replay_count, REPLAY_THREE);
    assert_eq!(plan.replay_report.entries.len(), REPLAY_THREE as usize);
}

#[test]
fn runtime_tick_history_compile_fixture_totals_match_expected() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    let entry = &plan.history_entry;

    assert_eq!(entry.local_effect_count, 3);
    assert_eq!(entry.allocated_total, 72);
    assert_eq!(entry.unmet_total, 8);
    assert_eq!(entry.satisfied_count, 2);
    assert_eq!(entry.unsatisfied_count, 1);
    assert!(entry.economy_execution_deferred);
    assert!(entry.participant_property_mutation_deferred);
    assert!(entry.scenario_authority_mutation_deferred);
    assert!(entry.local_effect_application_deferred);
}

#[test]
fn runtime_tick_history_compile_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let _plan = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
    assert!(plan_scenario_unchanged(&before, &after));
}

fn plan_scenario_unchanged(before: &str, after: &str) -> bool {
    before == after
}

#[test]
fn runtime_tick_history_compile_reports_stage_local_gpu_summary() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");

    assert!(plan.gpu_stage_proof_summary_available);
    assert!(
        plan.tick_shell_plan
            .gpu_stage_proof_summary
            .stage_local_gpu_proofs_available
    );
}

#[test]
fn runtime_tick_history_reports_no_fused_replay_kernel() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    assert!(!plan.fused_replay_kernel_present);
}

#[test]
fn runtime_tick_history_reports_no_new_gpu_primitive_required() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    assert!(!plan.new_gpu_primitive_required);
}

#[test]
fn runtime_tick_history_persistent_history_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let plan = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).expect("compile");
    assert!(plan.persistent_history_deferred);
    assert!(plan.scenario_authority_mutation_deferred);
}

#[test]
fn runtime_tick_history_rejects_invalid_replay_count() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let err = compile_runtime_tick_history_plan(&spec, TICK_ONE, 0).unwrap_err();
    assert!(matches!(err, SpecError::ValidationFailed));
}

#[test]
fn runtime_tick_history_rejects_invalid_earlier_stage() {
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

    let err = compile_runtime_tick_history_plan(&spec, TICK_ONE, REPLAY_THREE).unwrap_err();
    assert!(matches!(err, SpecError::ValidationFailed));
}
