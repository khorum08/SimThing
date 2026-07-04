use simthing_driver::{
    replay_production_path_0080_0, run_production_path_0080_0, ProductionPath0080ForbiddenRequests,
    ProductionPath0080Input, PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES,
    PRODUCTION_PATH_0080_0_ID, PRODUCTION_PATH_0080_0_SCENARIO, PRODUCTION_PATH_0080_0_STATUS_PASS,
    SCENARIO_0080_0_GATE_ID,
};

fn report() -> simthing_driver::ProductionPath0080Report {
    run_production_path_0080_0(&ProductionPath0080Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut ProductionPath0080ForbiddenRequests),
) -> simthing_driver::ProductionPath0080Report {
    let mut input = ProductionPath0080Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_production_path_0080_0(&input)
}

#[test]
fn production_path_0080_0_replay_deterministic() {
    let (first, second) = replay_production_path_0080_0();
    assert!(first.admitted, "{:?}", first.diagnostics);
    assert!(second.admitted, "{:?}", second.diagnostics);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
    assert_eq!(
        first
            .mobility_report
            .as_ref()
            .unwrap()
            .deterministic_replay_checksum,
        second
            .mobility_report
            .as_ref()
            .unwrap()
            .deterministic_replay_checksum
    );
}
