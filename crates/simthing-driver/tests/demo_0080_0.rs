use simthing_driver::{
    canonical_control_input, replay_demo_0080_0, run_demo_0080_0, Control0080Command,
    Control0080CommandBatch, Demo0080ForbiddenRequests, Demo0080Input, DEMO_0080_0_ID,
    DEMO_0080_0_SCENARIO, DEMO_0080_0_STATUS_PASS,
};

fn report() -> simthing_driver::Demo0080Report {
    run_demo_0080_0(&Demo0080Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut Demo0080ForbiddenRequests),
) -> simthing_driver::Demo0080Report {
    let mut input = Demo0080Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_demo_0080_0(&input)
}

#[test]
fn demo_0080_0_explicit_opt_in_only() {
    let disabled = run_demo_0080_0(&Demo0080Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.observation_export.is_empty());

    let mut default_on = Demo0080Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_demo_0080_0(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"demo_0080_0_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.headless_library_helper_only);
}

#[test]
fn demo_0080_0_runs_canonical_control_batch() {
    let admitted = report();
    assert!(admitted.uses_canonical_control_batch);
    assert_eq!(
        admitted
            .control_report
            .as_ref()
            .map(|control| control.applied_command_count),
        Some(3)
    );
    assert_eq!(canonical_control_input().commands, Control0080CommandBatch::canonical_run());
}

#[test]
fn demo_0080_0_emits_observation_export() {
    let admitted = report();
    assert!(!admitted.observation_export.is_empty());
    assert!(admitted.observation_export.contains("GAMEPLAY-0080-0"));
    assert!(!admitted.demo_export.is_empty());
    assert!(admitted.demo_export.contains("DEMO-0080-0"));
    assert!(admitted.demo_export.contains("MOVEMENT|begin"));
}

#[test]
fn demo_0080_0_export_replay_deterministic() {
    let (first, second) = replay_demo_0080_0();
    assert_eq!(first.demo_export, second.demo_export);
    assert_eq!(first.observation_export, second.observation_export);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
    assert_eq!(first.movement_days, second.movement_days);
}

#[test]
fn demo_0080_0_uses_existing_control_schedule_observation_path() {
    let admitted = report();
    let control = admitted.control_report.as_ref().expect("control report");
    let observation = admitted
        .observation_report
        .as_ref()
        .expect("observation report");
    assert!(control.admitted);
    assert!(observation.admitted);
    assert_eq!(observation.executed_step_count, 3);
    assert_eq!(control.control_id, "CONTROL-0080-0");
    assert_eq!(observation.observation_id, "GAMEPLAY-0080-0");
    assert!(!admitted.external_boundary_request_emitted);
}

#[test]
fn demo_0080_0_includes_day_to_day_patrol_and_pirate_movement_record() {
    let admitted = report();
    assert_eq!(admitted.movement_days.len(), 3);
    let step0 = &admitted.movement_days[0].record;
    assert_eq!(step0.patrol_start, "source");
    assert_eq!(step0.pirate_start, Some("destination"));
    assert!(step0.source_supply > 0);
    assert!(admitted.demo_export.contains("MOVEMENT|step=0"));
    assert!(admitted.demo_export.contains("MOVEMENT|step=1"));
    assert!(admitted.demo_export.contains("MOVEMENT|step=2"));
}

#[test]
fn demo_0080_0_movement_record_matches_observation_steps() {
    let admitted = report();
    let observation = admitted.observation_report.as_ref().expect("observation");
    assert_eq!(admitted.movement_days.len(), observation.transcript.steps.len());
    for (day, step) in admitted
        .movement_days
        .iter()
        .zip(observation.transcript.steps.iter())
    {
        assert_eq!(day.step_index, step.step_index);
        assert_eq!(day.record.threshold_accepted, step.threshold_accepted);
        assert_eq!(day.record.pirate_relocated, step.pirate_relocated);
        assert_eq!(day.record.patrol_relocated, step.patrol_relocated);
        assert_eq!(day.record.source_supply, step.source.supply);
        assert_eq!(day.record.destination_supply, step.destination.supply);
    }
}

#[test]
fn demo_0080_0_no_direct_movement_command() {
    let admitted = report();
    assert!(!admitted.direct_movement_control);

    let mut input = Demo0080Input::explicit_opt_in();
    input.surface.direct_movement_control = true;
    let rejected = run_demo_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"direct_patrol_move_rejected"));

    let mut input = Demo0080Input::explicit_opt_in();
    input.control_input.commands.commands.push(Control0080Command::DirectPatrolMove);
    let rejected = run_demo_0080_0(&input);
    assert!(!rejected.admitted);
}

#[test]
fn demo_0080_0_no_external_boundary_request() {
    let admitted = report();
    assert!(!admitted.external_boundary_request_emitted);

    let mut input = Demo0080Input::explicit_opt_in();
    input.surface.external_boundary_request = true;
    let rejected = run_demo_0080_0(&input);
    assert!(!rejected.admitted);
}

#[test]
fn demo_0080_0_no_player_command_loop() {
    let mut input = Demo0080Input::explicit_opt_in();
    input.surface.player_command_loop = true;
    let rejected = run_demo_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"player_command_loop"));
}

#[test]
fn demo_0080_0_no_ui_framework() {
    let mut input = Demo0080Input::explicit_opt_in();
    input.surface.ui_framework_present = true;
    let rejected = run_demo_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"ui_framework"));
}

#[test]
fn demo_0080_0_no_realtime_loop() {
    let mut input = Demo0080Input::explicit_opt_in();
    input.surface.realtime_loop_present = true;
    let rejected = run_demo_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"realtime_loop"));
}

#[test]
fn demo_0080_0_no_global_default_schedule() {
    let mut input = Demo0080Input::explicit_opt_in();
    input.surface.global_default_schedule_registered = true;
    let rejected = run_demo_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_default_schedule"));
}

#[test]
fn demo_0080_0_no_cli_binary() {
    let admitted = report();
    assert!(!admitted.cli_binary_present);
    assert!(admitted.headless_library_helper_only);

    let mut input = Demo0080Input::explicit_opt_in();
    input.surface.cli_binary_requested = true;
    let rejected = run_demo_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"cli_binary"));

    let cargo_toml = include_str!("../Cargo.toml");
    assert!(!cargo_toml.contains("demo_0080"));
    assert!(!cargo_toml.contains("[[bin]]\nname = \"demo"));
}

#[test]
fn demo_0080_0_no_semantic_or_raw_wgsl() {
    let rejected = rejected_with(|forbidden| forbidden.semantic_or_raw_wgsl = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn demo_0080_0_no_hard_currency_markets_trade_aibudget() {
    let rejected = rejected_with(|forbidden| {
        forbidden.hard_currency_markets_trade_aibudget = true;
    });
    assert!(!rejected.admitted);
}

#[test]
fn demo_0080_0_no_clausething_dependency() {
    let rejected = rejected_with(|forbidden| forbidden.clausething_dependency = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));
}

#[test]
fn demo_0080_0_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.demo_id, DEMO_0080_0_ID);
    assert_eq!(admitted.status, DEMO_0080_0_STATUS_PASS);
    assert_eq!(admitted.scenario_name, DEMO_0080_0_SCENARIO);
    assert!(admitted.uses_canonical_control_batch);
}
