use simthing_driver::{
    replay_default_schedule_0080_0, run_default_schedule_0080_0,
    DefaultSchedule0080ForbiddenRequests, DefaultSchedule0080Input, DefaultSchedule0080Location,
    DEFAULT_SCHEDULE_0080_0_ID, DEFAULT_SCHEDULE_0080_0_SCENARIO,
    DEFAULT_SCHEDULE_0080_0_STATUS_1B_PASS, PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES,
};

fn report() -> simthing_driver::DefaultSchedule0080RunReport {
    run_default_schedule_0080_0(&DefaultSchedule0080Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut DefaultSchedule0080ForbiddenRequests),
) -> simthing_driver::DefaultSchedule0080RunReport {
    let mut input = DefaultSchedule0080Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_default_schedule_0080_0(&input)
}

#[test]
fn default_schedule_0080_0_replay_deterministic() {
    let (first, second) = replay_default_schedule_0080_0();
    assert!(first.admitted, "{:?}", first.diagnostics);
    assert!(second.admitted, "{:?}", second.diagnostics);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
    assert_eq!(first.step_reports, second.step_reports);
}
#[test]
fn default_schedule_0080_0_predator_patrol_loop_replay_deterministic() {
    let (first, second) = replay_default_schedule_0080_0();
    assert_eq!(
        first.pirate_relocation_count,
        second.pirate_relocation_count
    );
    assert_eq!(
        first.pirate_supply_drained_total,
        second.pirate_supply_drained_total
    );
    assert_eq!(
        first.pirate_disruption_added_total,
        second.pirate_disruption_added_total
    );
    assert_eq!(first.step_reports, second.step_reports);
}

#[test]
fn default_schedule_0080_0_cat_and_mouse_pattern_emerges_deterministically() {
    let admitted = report();
    assert!(admitted.cat_and_mouse_pattern_observed);
    let patrol_relocated = admitted
        .step_reports
        .iter()
        .any(|step| step.production_path_invoked);
    let pirate_relocated = admitted
        .step_reports
        .iter()
        .filter_map(|step| step.pirate_report.as_ref())
        .any(|pirate| pirate.location_before != pirate.location_after);
    assert!(patrol_relocated);
    assert!(pirate_relocated);
}
