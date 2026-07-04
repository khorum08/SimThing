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
