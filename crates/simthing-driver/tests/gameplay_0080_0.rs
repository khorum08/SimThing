use simthing_driver::{
    export_gameplay_0080_text, observe_gameplay_0080_0, replay_observe_gameplay_0080_0,
    run_default_schedule_0080_0, DefaultSchedule0080Input, Gameplay0080ForbiddenRequests,
    Gameplay0080ObservationInput, GAMEPLAY_0080_0_ID, GAMEPLAY_0080_0_SCENARIO,
    GAMEPLAY_0080_0_STATUS_PASS,
};

fn report() -> simthing_driver::Gameplay0080ObservationReport {
    observe_gameplay_0080_0(&Gameplay0080ObservationInput::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut Gameplay0080ForbiddenRequests),
) -> simthing_driver::Gameplay0080ObservationReport {
    let mut input = Gameplay0080ObservationInput::explicit_opt_in();
    mutate(&mut input.forbidden);
    observe_gameplay_0080_0(&input)
}

#[test]
fn gameplay_0080_0_readonly_observation_explicit_opt_in_only() {
    let disabled = observe_gameplay_0080_0(&Gameplay0080ObservationInput::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.read_only);
    assert_eq!(disabled.executed_step_count, 0);
    assert!(disabled.transcript.steps.is_empty());

    let mut default_on = Gameplay0080ObservationInput::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = observe_gameplay_0080_0(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"gameplay_0080_0_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(admitted.read_only);
}

#[test]
fn gameplay_0080_0_consumes_default_schedule_report() {
    let schedule = run_default_schedule_0080_0(&DefaultSchedule0080Input::explicit_opt_in());
    let observed = observe_gameplay_0080_0(&Gameplay0080ObservationInput::explicit_opt_in_from_report(
        schedule.clone(),
    ));
    assert!(observed.admitted);
    assert_eq!(observed.executed_step_count, schedule.executed_step_count);
    assert_eq!(
        observed.deterministic_replay_checksum,
        schedule.deterministic_replay_checksum
    );
    assert_eq!(observed.transcript.steps.len(), schedule.step_reports.len());
}

#[test]
fn gameplay_0080_0_exports_tick_transcript() {
    let admitted = report();
    assert_eq!(admitted.transcript.steps.len(), admitted.executed_step_count as usize);
    assert_eq!(admitted.transcript.steps.len(), 3);
    assert!(!admitted.text_export.is_empty());
    assert!(admitted.text_export.contains("GAMEPLAY-0080-0"));
    assert!(admitted.text_export.contains("STEP|index=0"));
    assert!(admitted.text_export.contains("STEP|index=1"));
    assert!(admitted.text_export.contains("STEP|index=2"));
    assert_eq!(export_gameplay_0080_text(&admitted), admitted.text_export);
}

#[test]
fn gameplay_0080_0_includes_patrol_pirate_economy_state() {
    let admitted = report();
    let step = &admitted.transcript.steps[0];
    assert_eq!(step.source.location_label, "source");
    assert_eq!(step.destination.location_label, "destination");
    assert!(step.source.supply > 0);
    assert!(step.source.maintenance > 0);
    assert!(step.source.local_output > 0);
    assert!(step.source.local_security >= 0);
    assert!(step.source.disruption >= 0);
    assert!(step.patrol_entity_id > 0);
    assert!(step.patrol_owner_id > 0);
    assert_eq!(step.pirate_entity_id, 8_001);
    assert!(step.pirate_location_before.is_some());
    assert!(step.pirate_location_after.is_some());
}

#[test]
fn gameplay_0080_0_includes_threshold_event_boundary_trace() {
    let admitted = report();
    let traced = admitted
        .transcript
        .steps
        .iter()
        .find(|step| step.threshold_accepted)
        .expect("threshold step");
    assert!(traced.event_emitted);
    assert!(traced.boundary_request_materialized);
    assert!(traced.production_path_invoked);
    assert!(admitted.boundary_request_count >= 1);
    assert!(admitted.production_path_invocation_count >= 1);
}

#[test]
fn gameplay_0080_0_includes_cat_and_mouse_summary() {
    let admitted = report();
    assert!(admitted.cat_and_mouse_pattern_observed);
    assert!(admitted.pirate_relocation_count > 0);
    assert!(admitted.pirate_supply_drained_total > 0);
    assert!(admitted.pirate_disruption_added_total > 0);
    assert!(admitted
        .transcript
        .steps
        .iter()
        .any(|step| step.cat_and_mouse_step_observed));
    assert!(admitted.text_export.contains("cat_and_mouse=true"));
}

#[test]
fn gameplay_0080_0_replay_transcript_deterministic() {
    let (first, second) = replay_observe_gameplay_0080_0();
    assert_eq!(first.text_export, second.text_export);
    assert_eq!(first.transcript, second.transcript);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
}

#[test]
fn gameplay_0080_0_no_player_commands() {
    let admitted = report();
    assert!(!admitted.player_commands_present);
    assert!(!admitted.command_input_present);

    let mut input = Gameplay0080ObservationInput::explicit_opt_in();
    input.surface.player_commands_registered = true;
    let rejected = observe_gameplay_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"player_commands"));

    let rejected = rejected_with(|forbidden| forbidden.player_commands = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"player_commands"));
}

#[test]
fn gameplay_0080_0_no_realtime_loop() {
    let admitted = report();
    assert!(!admitted.realtime_loop_present);
    assert!(!admitted.gameplay_scheduler_present);

    let mut input = Gameplay0080ObservationInput::explicit_opt_in();
    input.surface.realtime_loop_present = true;
    let rejected = observe_gameplay_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"realtime_loop"));
}

#[test]
fn gameplay_0080_0_no_global_default_schedule() {
    let admitted = report();
    assert!(!admitted.global_default_schedule_registered);

    let rejected = rejected_with(|forbidden| forbidden.global_default_schedule = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_default_schedule"));
}

#[test]
fn gameplay_0080_0_no_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(!admitted.semantic_or_raw_wgsl_present);

    let rejected = rejected_with(|forbidden| forbidden.semantic_or_raw_wgsl = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn gameplay_0080_0_no_cpu_planner_or_external_move_script() {
    let admitted = report();
    assert!(!admitted.cpu_planner_used);
    assert!(!admitted.external_move_script_used);

    let rejected = rejected_with(|forbidden| {
        forbidden.cpu_planner_or_external_move_script = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"cpu_planner_or_external_move_script"));
}

#[test]
fn gameplay_0080_0_no_hard_currency_markets_trade_aibudget() {
    let admitted = report();
    assert!(!admitted.hard_currency_markets_trade_aibudget);

    let rejected = rejected_with(|forbidden| {
        forbidden.hard_currency_markets_trade_aibudget = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"hard_currency_markets_trade_aibudget"));
}

#[test]
fn gameplay_0080_0_no_clausething_dependency() {
    let admitted = report();
    assert!(!admitted.clausething_dependency_present);

    let rejected = rejected_with(|forbidden| forbidden.clausething_dependency = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));
}

#[test]
fn gameplay_0080_0_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.observation_id, GAMEPLAY_0080_0_ID);
    assert_eq!(admitted.status, GAMEPLAY_0080_0_STATUS_PASS);
    assert_eq!(admitted.scenario_name, GAMEPLAY_0080_0_SCENARIO);
    assert!(admitted.schedule_admitted);
    assert!(!admitted.closed_ladders_reopened);
    assert!(!admitted.passive_proof_wrapper_present);
}
