use simthing_driver::{
    export_gameplay_0080_1_text, observe_gameplay_0080_1, replay_observe_gameplay_0080_1,
    run_default_schedule_0080_1, DefaultSchedule0081Input, DefaultSchedule0081ShipFaction,
    Gameplay0081ForbiddenRequests, Gameplay0081Input, GAMEPLAY_0080_1_ID, GAMEPLAY_0080_1_SCENARIO,
    GAMEPLAY_0080_1_STATUS_PASS,
};

fn report() -> simthing_driver::Gameplay0081ObservationReport {
    observe_gameplay_0080_1(&Gameplay0081Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut Gameplay0081ForbiddenRequests),
) -> simthing_driver::Gameplay0081ObservationReport {
    let mut input = Gameplay0081Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    observe_gameplay_0080_1(&input)
}

#[test]
fn gameplay_0080_1_replay_transcript_deterministic() {
    let (first, second) = replay_observe_gameplay_0080_1();
    assert_eq!(first, second);
    assert_eq!(first.text_export, second.text_export);
    assert_eq!(first.transcript, second.transcript);
    assert_eq!(first.replay_checksum, second.replay_checksum);
}
