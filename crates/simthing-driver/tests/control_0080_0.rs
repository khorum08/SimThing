use simthing_driver::{
    admit_control_0080_0, replay_admit_control_0080_0, Control0080AdmissionInput,
    Control0080Command, Control0080CommandBatch, Control0080ForbiddenRequests, CONTROL_0080_0_ID,
    CONTROL_0080_0_SCENARIO, CONTROL_0080_0_STATUS_PASS,
};

fn report() -> simthing_driver::Control0080AdmissionReport {
    admit_control_0080_0(&Control0080AdmissionInput::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut Control0080ForbiddenRequests),
) -> simthing_driver::Control0080AdmissionReport {
    let mut input = Control0080AdmissionInput::explicit_opt_in();
    mutate(&mut input.forbidden);
    admit_control_0080_0(&input)
}

fn with_commands(commands: Vec<Control0080Command>) -> simthing_driver::Control0080AdmissionReport {
    let mut input = Control0080AdmissionInput::explicit_opt_in();
    input.commands = Control0080CommandBatch { commands };
    admit_control_0080_0(&input)
}

#[test]
fn control_0080_0_explicit_opt_in_only() {
    let disabled = admit_control_0080_0(&Control0080AdmissionInput::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.observation_report.is_none());

    let mut default_on = Control0080AdmissionInput::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = admit_control_0080_0(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"control_0080_0_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(admitted.bounded_command_admission_only);
}

#[test]
fn control_0080_0_accepts_bounded_scenario_value_commands() {
    let admitted = with_commands(vec![
        Control0080Command::SetSourceDisruption(9),
        Control0080Command::SetDestinationDisruption(1),
        Control0080Command::SetSourceSupply(12),
        Control0080Command::SetDestinationSupply(9),
        Control0080Command::SetSourceLocalSecurity(2),
        Control0080Command::SetDestinationLocalSecurity(7),
        Control0080Command::SetStepCount(3),
        Control0080Command::SetPatrolDisruptionReduction(1),
        Control0080Command::RunObservedScenario,
    ]);
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert_eq!(admitted.schedule_input.scenario.source.disruption, 9);
    assert_eq!(admitted.schedule_input.scenario.destination.supply, 9);
    assert_eq!(admitted.schedule_input.step_count, 3);
    assert_eq!(admitted.applied_command_count, 9);
}

#[test]
fn control_0080_0_runs_observed_scenario_after_admitted_command() {
    let admitted = report();
    let observation = admitted
        .observation_report
        .as_ref()
        .expect("observation report");
    assert!(observation.admitted);
    assert_eq!(observation.executed_step_count, 3);
    assert!(!admitted.external_boundary_request_emitted);
    assert!(!admitted.cpu_planner_used);
}

#[test]
fn control_0080_0_exports_transcript_after_command() {
    let admitted = report();
    assert!(!admitted.text_export.is_empty());
    assert!(admitted.text_export.contains("GAMEPLAY-0080-0"));
    assert!(admitted.text_export.contains("STEP|index=0"));
    assert_eq!(
        admitted.text_export,
        admitted
            .observation_report
            .as_ref()
            .map(simthing_driver::export_gameplay_0080_text)
            .unwrap_or_default()
    );
}

#[test]
fn control_0080_0_replay_after_command_deterministic() {
    let (first, second) = replay_admit_control_0080_0();
    assert_eq!(first.text_export, second.text_export);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
    assert_eq!(
        first
            .observation_report
            .as_ref()
            .map(|report| report.deterministic_replay_checksum),
        second
            .observation_report
            .as_ref()
            .map(|report| report.deterministic_replay_checksum)
    );
}
#[test]
fn control_0080_0_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.control_id, CONTROL_0080_0_ID);
    assert_eq!(admitted.status, CONTROL_0080_0_STATUS_PASS);
    assert_eq!(admitted.scenario_name, CONTROL_0080_0_SCENARIO);
    assert!(!admitted.direct_movement_control);
    assert!(!admitted.player_command_loop);
    assert!(!admitted.external_boundary_request_emitted);
}
