use simthing_driver::{
    replay_default_schedule_0080_1, run_default_schedule_0080_1,
    DefaultSchedule0081ForbiddenRequests, DefaultSchedule0081Input, DefaultSchedule0081ShipFaction,
    ProductionPath0081Input, DEFAULT_SCHEDULE_0080_1_ID, DEFAULT_SCHEDULE_0080_1_SCENARIO,
    DEFAULT_SCHEDULE_0080_1_STATUS_PASS,
};

fn report() -> simthing_driver::DefaultSchedule0081RunReport {
    run_default_schedule_0080_1(&DefaultSchedule0081Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut DefaultSchedule0081ForbiddenRequests),
) -> simthing_driver::DefaultSchedule0081RunReport {
    let mut input = DefaultSchedule0081Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_default_schedule_0080_1(&input)
}

#[test]
fn default_schedule_0080_1_replay_deterministic() {
    let (first, second) = replay_default_schedule_0080_1();
    assert!(first.admitted, "{:?}", first.diagnostics);
    assert!(second.admitted, "{:?}", second.diagnostics);
    assert_eq!(first, second);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
}
