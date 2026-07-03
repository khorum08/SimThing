use simthing_driver::{
    admit_control_0080_1, export_gameplay_0080_1_text, replay_admit_control_0080_1,
    Control0081AdmissionInput, Control0081Command, Control0081CommandBatch,
    Control0081ForbiddenRequests, CONTROL_0080_1_ID, CONTROL_0080_1_SCENARIO,
    CONTROL_0080_1_STATUS_PASS,
};

fn report() -> simthing_driver::Control0081AdmissionReport {
    admit_control_0080_1(&Control0081AdmissionInput::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut Control0081ForbiddenRequests),
) -> simthing_driver::Control0081AdmissionReport {
    let mut input = Control0081AdmissionInput::explicit_opt_in();
    mutate(&mut input.forbidden);
    admit_control_0080_1(&input)
}

fn with_commands(commands: Vec<Control0081Command>) -> simthing_driver::Control0081AdmissionReport {
    let mut input = Control0081AdmissionInput::explicit_opt_in();
    input.commands = Control0081CommandBatch { commands };
    admit_control_0080_1(&input)
}

#[test]
fn control_0080_1_explicit_opt_in_only() {
    let disabled = admit_control_0080_1(&Control0081AdmissionInput::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.observation_report.is_none());

    let mut default_on = Control0081AdmissionInput::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = admit_control_0080_1(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"control_0080_1_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(admitted.bounded_command_admission_only);
}

#[test]
fn control_0080_1_accepts_bounded_schedule_value_commands() {
    let admitted = with_commands(vec![
        Control0081Command::SetStepCount(2),
        Control0081Command::SetTerranThreshold(-1),
        Control0081Command::SetPirateThreshold(1),
        Control0081Command::SetTerranSourceStarsystem(0),
        Control0081Command::SetTerranCandidateStarsystem(4),
        Control0081Command::SetPirateSourceStarsystem(6),
        Control0081Command::SetPirateCandidateStarsystem(8),
        Control0081Command::SetSupplySecurityGap(-7),
        Control0081Command::SetBilateralRelationalGap(4),
        Control0081Command::SetCompositeGapSum(1),
        Control0081Command::RunObservedScenario,
    ]);
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert_eq!(admitted.applied_command_count, 11);
    assert_eq!(admitted.schedule_input.step_count, 2);
    assert_eq!(admitted.schedule_input.movement_threshold, 1);
    assert_eq!(admitted.bounded_config.terran_candidate_starsystem, 4);
    assert_eq!(admitted.bounded_config.pirate_candidate_starsystem, 8);
    assert_eq!(admitted.bounded_config.supply_security_gap, -7);
    assert!(admitted.command_writes_existing_bounded_values_only);
    assert!(!admitted.command_moved_ship);
    assert!(!admitted.command_emitted_boundary_request);
    assert!(!admitted.command_bypassed_field_policy);
}

#[test]
fn control_0080_1_runs_observed_scenario_after_admitted_command() {
    let admitted = report();
    let observation = admitted
        .observation_report
        .as_ref()
        .expect("observation report");
    assert!(observation.admitted);
    assert_eq!(observation.observation_id, "GAMEPLAY-0080-1");
    assert_eq!(observation.schedule_id, "DEFAULT-SCHEDULE-0080-1");
    assert_eq!(observation.executed_step_count, 3);
    assert!(observation.schedule_invoked_by_observer);
    assert!(!observation.observer_materialized_boundary_requests);
}

#[test]
fn control_0080_1_exports_transcript_after_command() {
    let admitted = report();
    assert!(!admitted.text_export.is_empty());
    assert!(admitted.text_export.contains("GAMEPLAY-0080-1"));
    assert!(admitted.text_export.contains("MOVE|step=0"));
    assert_eq!(
        admitted.text_export,
        admitted
            .observation_report
            .as_ref()
            .map(export_gameplay_0080_1_text)
            .unwrap_or_default()
    );
}

#[test]
fn control_0080_1_replay_after_command_deterministic() {
    let (first, second) = replay_admit_control_0080_1();
    assert_eq!(first.text_export, second.text_export);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
    assert_eq!(
        first
            .observation_report
            .as_ref()
            .map(|report| report.replay_checksum),
        second
            .observation_report
            .as_ref()
            .map(|report| report.replay_checksum)
    );
}
#[test]
fn control_0080_1_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.control_id, CONTROL_0080_1_ID);
    assert_eq!(admitted.status, CONTROL_0080_1_STATUS_PASS);
    assert_eq!(admitted.scenario_name, CONTROL_0080_1_SCENARIO);
    assert!(!admitted.direct_movement_control);
    assert!(!admitted.player_command_loop);
    assert!(!admitted.command_emitted_boundary_request);
    assert!(!admitted.command_moved_ship);
    assert!(!admitted.command_bypassed_field_policy);
}
