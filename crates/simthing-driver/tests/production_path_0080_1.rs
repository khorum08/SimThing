use simthing_driver::{
    replay_production_path_0080_1, run_production_path_0080_1, Atlas0080Input, Atlas0080TheaterId,
    EconScale0080Faction, EconScale0080Input, ProductionPath0081ForbiddenRequests,
    ProductionPath0081Input, PRODUCTION_PATH_0080_1_ID, PRODUCTION_PATH_0080_1_STATUS_PASS,
    SCENARIO_0080_1_GATE_ID,
};

fn report() -> simthing_driver::ProductionPath0081Report {
    run_production_path_0080_1(&ProductionPath0081Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut ProductionPath0081ForbiddenRequests),
) -> simthing_driver::ProductionPath0081Report {
    let mut input = ProductionPath0081Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_production_path_0080_1(&input)
}

#[test]
fn production_path_0080_1_replay_deterministic() {
    let (first, second) = replay_production_path_0080_1();
    assert!(first.admitted, "{:?}", first.diagnostics);
    assert!(second.admitted, "{:?}", second.diagnostics);
    assert_eq!(first, second);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
    assert_eq!(
        first
            .atlas_report
            .as_ref()
            .unwrap()
            .deterministic_replay_checksum,
        second
            .atlas_report
            .as_ref()
            .unwrap()
            .deterministic_replay_checksum
    );
    assert_eq!(
        first
            .econ_scale_report
            .as_ref()
            .unwrap()
            .deterministic_replay_checksum,
        second
            .econ_scale_report
            .as_ref()
            .unwrap()
            .deterministic_replay_checksum
    );
}
