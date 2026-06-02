use simthing_driver::{
    replay_default_schedule_0080_0, run_default_schedule_0080_0,
    DefaultSchedule0080ForbiddenRequests, DefaultSchedule0080Input, DEFAULT_SCHEDULE_0080_0_ID,
    DEFAULT_SCHEDULE_0080_0_SCENARIO, DEFAULT_SCHEDULE_0080_0_STATUS_1A_PASS,
    PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES,
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
fn default_schedule_0080_0_explicit_opt_in_only() {
    let disabled = run_default_schedule_0080_0(&DefaultSchedule0080Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert_eq!(disabled.executed_step_count, 0);
    assert_eq!(disabled.production_path_invocation_count, 0);

    let mut default_on = DefaultSchedule0080Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_default_schedule_0080_0(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"default_schedule_0080_0_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
}

#[test]
fn default_schedule_0080_0_default_path_has_no_schedule() {
    let disabled = run_default_schedule_0080_0(&DefaultSchedule0080Input::default_simsession());
    assert!(disabled.disabled_no_op);
    assert!(!disabled.scenario_schedule_registered);
    assert!(!disabled.global_default_schedule_registered);
    assert_eq!(disabled.executed_step_count, 0);
    assert_eq!(disabled.boundary_request_count, 0);
}

#[test]
fn default_schedule_0080_0_no_global_default_schedule() {
    let admitted = report();
    assert!(!admitted.global_default_schedule_registered);

    let mut input = DefaultSchedule0080Input::explicit_opt_in();
    input.surface.global_default_schedule_registered = true;
    let rejected = run_default_schedule_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_default_schedule"));
}

#[test]
fn default_schedule_0080_0_runs_local_patrol_economy_steps() {
    let admitted = report();
    assert_eq!(admitted.schedule_id, DEFAULT_SCHEDULE_0080_0_ID);
    assert_eq!(admitted.scenario, DEFAULT_SCHEDULE_0080_0_SCENARIO);
    assert!(admitted.scenario_scoped_only);
    assert_eq!(admitted.executed_step_count, 3);
    assert!(admitted
        .step_reports
        .iter()
        .all(|step| { step.step.source_disruption_after <= step.step.source_disruption_before }));
}

#[test]
fn default_schedule_0080_0_threshold_false_emits_no_boundary_request() {
    let input = DefaultSchedule0080Input::explicit_opt_in_threshold_false();
    let admitted = run_default_schedule_0080_0(&input);
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert_eq!(admitted.boundary_request_count, 0);
    assert_eq!(admitted.production_path_invocation_count, 0);
    assert!(admitted
        .step_reports
        .iter()
        .all(|step| !step.boundary_request_materialized));
}

#[test]
fn default_schedule_0080_0_threshold_true_emits_boundary_request() {
    let admitted = report();
    assert_eq!(admitted.boundary_request_count, 1);
    assert!(admitted.step_reports[0].sead_threshold_accepted);
    assert!(admitted.step_reports[0].sead_emit_event_emitted);
    assert!(admitted.step_reports[0].boundary_request_materialized);
}

#[test]
fn default_schedule_0080_0_routes_boundary_request_to_production_path() {
    let admitted = report();
    let routed = admitted
        .step_reports
        .iter()
        .find(|step| step.production_path_invoked)
        .expect("routed production path step");
    let production = routed
        .production_path_report
        .as_ref()
        .expect("production path report");
    assert!(production.admitted, "{:?}", production.diagnostics);
    assert!(production.mobility_substrate_consumed_boundary_request);
    assert!(production.boundary_request_materialized);
}

#[test]
fn default_schedule_0080_0_no_cpu_planner_or_external_move_script() {
    let admitted = report();
    assert!(!admitted.cpu_planner_used);
    assert!(!admitted.external_move_script_used);

    let rejected = rejected_with(|forbidden| {
        forbidden.cpu_planner_or_external_move_script = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"cpu_planner_or_external_move_script"));
}

#[test]
fn default_schedule_0080_0_preserves_identity_owner_overlay_economy() {
    let admitted = report();
    let production = admitted.step_reports[0]
        .production_path_report
        .as_ref()
        .expect("production path report");
    assert!(production.identity_preserved_after_relocation);
    assert!(production.owner_overlay_persists_after_move);
    assert!(!production.source_membership_after);
    assert!(production.destination_membership_after);
    assert_eq!(production.source_patrol_count_after, 0);
    assert_eq!(production.destination_patrol_count_after, 1);
}

#[test]
fn default_schedule_0080_0_bounded_local_economy_only() {
    let admitted = report();
    assert!(admitted.bounded_local_economy_only);
    assert_eq!(
        admitted.bounded_local_economy_values,
        PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES
    );
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
fn default_schedule_0080_0_rejects_gameplay_surface() {
    let rejected = rejected_with(|forbidden| {
        forbidden.gameplay_surface = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"gameplay_surface"));
}

#[test]
fn default_schedule_0080_0_rejects_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(!admitted.semantic_or_raw_wgsl_present);

    let rejected = rejected_with(|forbidden| {
        forbidden.semantic_or_raw_wgsl = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn default_schedule_0080_0_rejects_hard_currency_markets_trade_aibudget() {
    let rejected = rejected_with(|forbidden| {
        forbidden.hard_currency_markets_trade_aibudget = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"hard_currency_markets_trade_aibudget"));
}

#[test]
fn default_schedule_0080_0_rejects_nested_resource_flow() {
    let rejected = rejected_with(|forbidden| {
        forbidden.nested_resource_flow = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"nested_resource_flow"));
}

#[test]
fn default_schedule_0080_0_rejects_clausething_dependency() {
    let admitted = report();
    assert!(!admitted.clausething_dependency_present);

    let rejected = rejected_with(|forbidden| {
        forbidden.clausething_dependency = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));
}

#[test]
fn default_schedule_0080_0_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.schedule_id, DEFAULT_SCHEDULE_0080_0_ID);
    assert_eq!(admitted.status, DEFAULT_SCHEDULE_0080_0_STATUS_1A_PASS);
    assert!(!admitted.pirate_behavior_implemented);

    let rejected = rejected_with(|forbidden| {
        forbidden.pirate_behavior = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"pirate_behavior_not_implemented_in_1a"));
}
