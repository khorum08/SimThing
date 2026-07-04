use simthing_driver::{
    replay_demo_0080_1, run_demo_0080_1, Demo0081ForbiddenRequests, Demo0081Input, DEMO_0080_1_ID,
    DEMO_0080_1_SCENARIO, DEMO_0080_1_STATUS_PASS,
};

fn report() -> simthing_driver::Demo0081Report {
    run_demo_0080_1(&Demo0081Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut Demo0081ForbiddenRequests),
) -> simthing_driver::Demo0081Report {
    let mut input = Demo0081Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_demo_0080_1(&input)
}

#[test]
fn demo_0080_1_replay_deterministic() {
    let (a, b) = replay_demo_0080_1();
    assert_eq!(a, b);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}
