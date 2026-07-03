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
fn production_path_0080_0_explicit_opt_in_only() {
    let disabled = run_production_path_0080_0(&ProductionPath0080Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(!disabled.local_patrol_economy_instantiated);
    assert!(!disabled.mobility_substrate_consumed_boundary_request);

    let mut default_on = ProductionPath0080Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_production_path_0080_0(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"production_path_0080_0_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
}

#[test]
fn production_path_0080_0_no_global_default_schedule() {
    let admitted = report();
    assert!(!admitted.global_default_schedule_registered);

    let mut input = ProductionPath0080Input::explicit_opt_in();
    input.surface.global_default_schedule_registered = true;
    let rejected = run_production_path_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_default_schedule"));
}

#[test]
fn production_path_0080_0_instantiates_local_patrol_economy() {
    let admitted = report();
    assert_eq!(admitted.path_id, PRODUCTION_PATH_0080_0_ID);
    assert_eq!(PRODUCTION_PATH_0080_0_SCENARIO, "Local Patrol Economy");
    assert!(admitted.local_patrol_economy_instantiated);
}

#[test]
fn production_path_0080_0_field_policy_threshold_emits_boundary_request() {
    let admitted = report();
    assert!(admitted.field_policy_threshold_accepted);
    assert!(admitted.field_policy_emit_event_emitted);
    assert!(admitted.boundary_request_materialized);
    assert!(admitted.mobility_substrate_consumed_boundary_request);
}

#[test]
fn production_path_0080_0_no_cpu_planner_or_external_move_script() {
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
fn production_path_0080_0_identity_preserved_after_relocation() {
    let admitted = report();
    assert_eq!(
        admitted.patrol_entity_id_before,
        admitted.patrol_entity_id_after
    );
    assert!(admitted.identity_preserved_after_relocation);
}

#[test]
fn production_path_0080_0_source_membership_updates() {
    let admitted = report();
    assert!(admitted.source_membership_before);
    assert!(!admitted.source_membership_after);
}

#[test]
fn production_path_0080_0_destination_membership_updates() {
    let admitted = report();
    assert!(!admitted.destination_membership_before);
    assert!(admitted.destination_membership_after);
}

#[test]
fn production_path_0080_0_owner_overlay_persists_after_move() {
    let admitted = report();
    assert_eq!(admitted.owner_id_before, admitted.owner_id_after);
    assert_eq!(
        admitted.owner_overlay_modifier_before,
        admitted.owner_overlay_modifier_after
    );
    assert!(admitted.owner_overlay_persists_after_move);
}

#[test]
fn production_path_0080_0_source_economy_stops_counting_patrol() {
    let admitted = report();
    assert_eq!(admitted.source_patrol_count_before, 1);
    assert_eq!(admitted.source_patrol_count_after, 0);
}

#[test]
fn production_path_0080_0_destination_economy_starts_counting_patrol() {
    let admitted = report();
    assert_eq!(admitted.destination_patrol_count_before, 0);
    assert_eq!(admitted.destination_patrol_count_after, 1);
}

#[test]
fn production_path_0080_0_bounded_local_economy_only() {
    let admitted = report();
    assert!(admitted.bounded_local_economy_only);
    assert_eq!(
        admitted.bounded_local_economy_values,
        PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES
    );
}
#[test]
fn production_path_0080_0_no_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(!admitted.semantic_or_raw_wgsl_present);

    let rejected = rejected_with(|forbidden| {
        forbidden.semantic_or_raw_wgsl = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn production_path_0080_0_no_gameplay_surface() {
    let admitted = report();
    assert!(!admitted.gameplay_surface_present);

    let rejected = rejected_with(|forbidden| {
        forbidden.gameplay_surface = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"gameplay_surface"));
}

#[test]
fn production_path_0080_0_no_clausething_dependency() {
    let admitted = report();
    assert!(!admitted.clausething_dependency_present);

    let rejected = rejected_with(|forbidden| {
        forbidden.clausething_dependency = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));
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

#[test]
fn production_path_0080_0_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.path_id, PRODUCTION_PATH_0080_0_ID);
    assert_eq!(admitted.scenario_gate_id, SCENARIO_0080_0_GATE_ID);
    assert_eq!(admitted.status, PRODUCTION_PATH_0080_0_STATUS_PASS);
    assert!(!admitted.closed_ladders_reopened);
}
