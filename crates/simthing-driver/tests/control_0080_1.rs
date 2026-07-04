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
