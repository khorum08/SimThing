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
