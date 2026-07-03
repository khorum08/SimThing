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
fn default_schedule_0080_0_explicit_opt_in_only() {
    let disabled = run_default_schedule_0080_0(&DefaultSchedule0080Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert_eq!(disabled.executed_step_count, 0);
    assert_eq!(disabled.production_path_invocation_count, 0);
    assert!(!disabled.pirate_behavior_implemented);

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
    assert_eq!(disabled.pirate_relocation_count, 0);
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
    assert!(admitted.pirate_behavior_implemented);
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
    assert!(admitted.boundary_request_count >= 1);
    assert!(admitted.step_reports[0].field_policy_threshold_accepted);
    assert!(admitted.step_reports[0].field_policy_emit_event_emitted);
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
fn default_schedule_0080_0_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.schedule_id, DEFAULT_SCHEDULE_0080_0_ID);
    assert_eq!(admitted.status, DEFAULT_SCHEDULE_0080_0_STATUS_1B_PASS);
    assert!(admitted.pirate_behavior_implemented);

    let rejected = rejected_with(|forbidden| {
        forbidden.unbounded_pirate_behavior = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"unbounded_pirate_behavior"));
}

#[test]
fn default_schedule_0080_0_pirate_raises_disruption_and_consumes_supply() {
    let admitted = report();
    let pirate = admitted.step_reports[0]
        .pirate_report
        .as_ref()
        .expect("pirate step report");
    assert!(pirate.local_disruption_after > pirate.local_disruption_before);
    assert!(pirate.local_supply_after < pirate.local_supply_before);
    assert!(pirate.supply_drained > 0);
    assert!(admitted.pirate_disruption_added_total > 0);
    assert!(admitted.pirate_supply_drained_total > 0);
}

#[test]
fn default_schedule_0080_0_pirate_relocates_when_disruption_ge_half_supply() {
    let admitted = report();
    let relocating = admitted
        .step_reports
        .iter()
        .filter_map(|step| step.pirate_report.as_ref())
        .find(|pirate| pirate.boundary_request_materialized)
        .expect("pirate relocation");
    assert!(relocating.relocation_threshold_crossed);
    assert!(relocating.threshold_event_emitted);
    assert!(relocating.mobility_transfer_posture_preserved);
    assert_ne!(relocating.location_before, relocating.location_after);
    assert!(admitted.pirate_relocation_count > 0);
}

#[test]
fn default_schedule_0080_0_patrol_reduces_disruption_and_relocates_to_depleted_supply() {
    let admitted = report();
    assert!(
        admitted.step_reports[0].step.source_disruption_after
            < admitted.step_reports[0].step.source_disruption_before
    );
    assert!(admitted.step_reports[0].production_path_invoked);
    let production = admitted.step_reports[0]
        .production_path_report
        .as_ref()
        .expect("production path report");
    assert!(!production.source_membership_after);
    assert!(production.destination_membership_after);
}

#[test]
fn default_schedule_0080_0_pirate_is_second_identity_not_second_economy_owner() {
    let admitted = report();
    assert!(admitted.pirate_is_second_identity);
    assert_ne!(admitted.patrol_identity_lane, admitted.pirate_identity_lane);
    assert!(!admitted.pirate_is_second_economy_owner);
    assert!(admitted
        .step_reports
        .iter()
        .filter_map(|step| step.pirate_report.as_ref())
        .all(|pirate| pirate.pirate_is_second_identity && !pirate.pirate_is_second_economy_owner));
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
fn default_schedule_0080_0_pirate_prefers_low_patrol_influence_high_supply_target() {
    let admitted = report();
    assert!(admitted.local_security_evasion_term_implemented);
    let relocating = admitted
        .step_reports
        .iter()
        .filter_map(|step| step.pirate_report.as_ref())
        .find(|pirate| pirate.boundary_request_materialized)
        .expect("pirate relocation");
    assert!(relocating.used_supply_term);
    assert!(relocating.used_disruption_term);
    assert!(relocating.used_local_security_evasion_term);
    assert!(relocating.preferred_low_patrol_influence_high_supply_target);
    assert_eq!(
        relocating.location_after,
        DefaultSchedule0080Location::Source
    );
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
