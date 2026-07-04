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
fn gameplay_0080_0_replay_transcript_deterministic() {
    let (first, second) = replay_observe_gameplay_0080_0();
    assert_eq!(first.text_export, second.text_export);
    assert_eq!(first.transcript, second.transcript);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
}
