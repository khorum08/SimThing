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
fn default_schedule_0080_1_explicit_opt_in_only() {
    let disabled = run_default_schedule_0080_1(&DefaultSchedule0081Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert_eq!(disabled.executed_step_count, 0);
    assert!(disabled.production_path_report.is_none());

    let mut default_on = DefaultSchedule0081Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_default_schedule_0080_1(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"default_schedule_0080_1_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
}

#[test]
fn default_schedule_0080_1_requires_production_path_admitted() {
    let admitted = report();
    assert!(admitted.production_path_admitted_pass);
    assert!(admitted.consumed_production_path);

    let mut input = DefaultSchedule0081Input::explicit_opt_in();
    input.production_path_input.forbidden.semantic_or_raw_wgsl = true;
    let rejected = run_default_schedule_0080_1(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"production_path_0080_1_disabled_rejected_or_not_admitted"));
    assert!(rejected.production_path_report.is_some());
}
#[test]
fn default_schedule_0080_1_runs_bounded_nested_starmap_steps() {
    let admitted = report();
    assert!(admitted.bounded_step_loop);
    assert!(!admitted.wall_clock_loop);
    assert!(!admitted.async_background_loop);
    assert_eq!(admitted.requested_step_count, 3);
    assert_eq!(admitted.executed_step_count, 3);
    assert_eq!(admitted.step_reports.len(), 3);
}

#[test]
fn default_schedule_0080_1_threshold_false_emits_no_boundary_request() {
    let input = DefaultSchedule0081Input::explicit_opt_in_threshold_false();
    let admitted = run_default_schedule_0080_1(&input);
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert_eq!(admitted.threshold_false_count, admitted.executed_step_count);
    assert_eq!(admitted.event_emitted_count, 0);
    assert_eq!(admitted.boundary_request_count, 0);
    assert!(admitted
        .step_reports
        .iter()
        .all(|step| step.movement.is_none()));
}

#[test]
fn default_schedule_0080_1_threshold_true_emits_boundary_request() {
    let admitted = report();
    assert_eq!(admitted.threshold_true_count, admitted.executed_step_count);
    assert_eq!(admitted.event_emitted_count, admitted.executed_step_count);
    assert_eq!(
        admitted.boundary_request_count,
        admitted.executed_step_count
    );
    assert!(admitted
        .step_reports
        .iter()
        .all(|step| step.decision.boundary_request_materialized));
}

#[test]
fn default_schedule_0080_1_routes_boundary_request_through_mobility_substrate() {
    let admitted = report();
    assert_eq!(admitted.mobility_substrate_routed_count, 2);
    assert!(admitted
        .step_reports
        .iter()
        .filter_map(|step| step.movement.as_ref())
        .all(|movement| movement.routed_through_mobility_substrate
            && movement.existing_mobility_transfer_posture));
}

#[test]
fn default_schedule_0080_1_moves_terran_ship_by_field_policy_gap() {
    let admitted = report();
    assert_eq!(admitted.terran_move_count, 1);
    let movement = admitted
        .step_reports
        .iter()
        .find_map(|step| {
            step.movement
                .as_ref()
                .filter(|m| m.mover_faction == DefaultSchedule0081ShipFaction::Terran)
        })
        .unwrap();
    assert_eq!(movement.mover_id, 80_301);
    assert_eq!(movement.start_starsystem, 0);
    assert_eq!(movement.end_starsystem, 1);
}

#[test]
fn default_schedule_0080_1_moves_pirate_ship_by_field_policy_gap() {
    let admitted = report();
    assert_eq!(admitted.pirate_move_count, 1);
    let movement = admitted
        .step_reports
        .iter()
        .find_map(|step| {
            step.movement
                .as_ref()
                .filter(|m| m.mover_faction == DefaultSchedule0081ShipFaction::Pirate)
        })
        .unwrap();
    assert_eq!(movement.mover_id, 80_401);
    assert_eq!(movement.start_starsystem, 6);
    assert_eq!(movement.end_starsystem, 2);
    assert!(admitted.pirate_full_economy_faction_preserved);
}

#[test]
fn default_schedule_0080_1_preserves_identity_and_owner_overlay() {
    let admitted = report();
    assert!(admitted.identity_preserved);
    assert!(admitted.owner_overlay_preserved);
    assert!(admitted
        .step_reports
        .iter()
        .filter_map(|step| step.movement.as_ref())
        .all(
            |movement| movement.owner_id_before == movement.owner_id_after
                && movement.identity_preserved
                && movement.owner_overlay_preserved
        ));
}

#[test]
fn default_schedule_0080_1_updates_membership_without_reparenting() {
    let admitted = report();
    assert!(admitted.membership_updated_without_reparenting);
    assert!(!admitted.owner_entity_as_spatial_parent);
    assert!(!admitted.capture_as_reparenting);
    assert!(admitted
        .step_reports
        .iter()
        .filter_map(|step| step.movement.as_ref())
        .all(|movement| movement.membership_updated
            && movement.membership_updated_without_reparenting
            && !movement.owner_entity_as_spatial_parent
            && !movement.capture_as_reparenting));
}

#[test]
fn default_schedule_0080_1_consumes_atlas_residency_report() {
    let admitted = report();
    assert!(admitted.consumed_atlas_residency_report);
    let production = admitted.production_path_report.as_ref().unwrap();
    assert!(production.atlas_report_admitted_pass);
    assert!(!production.active_theaters.is_empty());
}

#[test]
fn default_schedule_0080_1_consumes_faction_index_econ_report() {
    let admitted = report();
    assert!(admitted.consumed_faction_index_econ_report);
    let production = admitted.production_path_report.as_ref().unwrap();
    assert!(production.econ_scale_report_admitted_pass);
    assert!(production.fixed_terran_pirate_faction_set);
    assert!(production.contended_clearing_reports_visible);
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

#[test]
fn default_schedule_0080_1_no_observation_control_demo() {
    let admitted = report();
    assert!(!admitted.observation_control_demo_0080_1);

    let rejected = rejected_with(|f| f.observation_control_demo_0080_1 = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"observation_control_demo_0080_1"));
}

#[test]
fn default_schedule_0080_1_no_direct_movement_command() {
    let admitted = report();
    assert!(!admitted.direct_movement_command);

    let rejected = rejected_with(|f| f.direct_movement_command = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"direct_movement_command"));
}

#[test]
fn default_schedule_0080_1_no_external_boundary_request() {
    let admitted = report();
    assert!(!admitted.external_boundary_request);

    let rejected = rejected_with(|f| f.external_boundary_request = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"external_boundary_request"));
}

#[test]
fn default_schedule_0080_1_no_cpu_planner_or_commitment() {
    let admitted = report();
    assert!(!admitted.cpu_planner_urgency_commitment);

    let rejected = rejected_with(|f| f.cpu_planner_urgency_commitment = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn default_schedule_0080_1_no_default_session_pass_graph_wiring() {
    let admitted = report();
    assert!(!admitted.default_session_pass_graph_wiring);

    let rejected = rejected_with(|f| f.default_session_pass_graph_wiring = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"default_session_pass_graph_wiring"));
}

#[test]
fn default_schedule_0080_1_no_global_default_schedule() {
    let admitted = report();
    assert!(!admitted.global_default_schedule);

    let rejected = rejected_with(|f| f.global_default_schedule = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_default_schedule"));
}

#[test]
fn default_schedule_0080_1_no_realtime_loop_or_ui() {
    let admitted = report();
    assert!(!admitted.realtime_loop_or_ui);

    let rejected = rejected_with(|f| f.realtime_loop_or_ui = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"realtime_loop_or_ui"));
}

#[test]
fn default_schedule_0080_1_no_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(!admitted.semantic_or_raw_wgsl_present);

    let rejected = rejected_with(|f| f.semantic_or_raw_wgsl = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn default_schedule_0080_1_no_new_shader_or_gpu_kernel() {
    let admitted = report();
    assert!(!admitted.new_shader_or_gpu_kernel);

    let rejected = rejected_with(|f| f.new_shader_or_gpu_kernel = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"new_shader_or_gpu_kernel"));
}

#[test]
fn default_schedule_0080_1_no_hard_currency_markets_trade_aibudget() {
    let admitted = report();
    assert!(!admitted.hard_currency_markets_trade_aibudget);

    let rejected = rejected_with(|f| f.hard_currency_markets_trade_aibudget = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"hard_currency_markets_trade_aibudget"));
}

#[test]
fn default_schedule_0080_1_no_nested_resource_flow() {
    let admitted = report();
    assert!(!admitted.nested_resource_flow);

    let rejected = rejected_with(|f| f.nested_resource_flow = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"nested_resource_flow"));
}

#[test]
fn default_schedule_0080_1_no_clausething_dependency() {
    let admitted = report();
    assert!(!admitted.clausething_dependency_present);
    assert!(!admitted.simthing_spec_altered);
    assert!(!admitted.invariant_edited);
    assert!(!admitted.passive_proof_wrapper);
    assert!(!admitted.general_scheduler);

    let rejected = rejected_with(|f| f.clausething_dependency = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));

    let rejected = rejected_with(|f| f.simthing_spec_alteration = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"simthing_spec_alteration"));

    let rejected = rejected_with(|f| f.invariant_edit = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"invariant_edit"));

    let rejected = rejected_with(|f| f.passive_proof_wrapper = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"passive_proof_wrapper"));

    let rejected = rejected_with(|f| f.general_scheduler = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"general_scheduler"));
}

#[test]
fn default_schedule_0080_1_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.schedule_id, DEFAULT_SCHEDULE_0080_1_ID);
    assert_eq!(admitted.status, DEFAULT_SCHEDULE_0080_1_STATUS_PASS);
    assert_eq!(admitted.scenario, DEFAULT_SCHEDULE_0080_1_SCENARIO);
    assert!(admitted.scenario_scoped_only);
}

#[test]
fn default_schedule_0080_1_terran_and_pirate_moves_are_distinct_identities() {
    let admitted = report();
    let terran = admitted
        .step_reports
        .iter()
        .find_map(|step| {
            step.movement
                .as_ref()
                .filter(|m| m.mover_faction == DefaultSchedule0081ShipFaction::Terran)
        })
        .unwrap();
    let pirate = admitted
        .step_reports
        .iter()
        .find_map(|step| {
            step.movement
                .as_ref()
                .filter(|m| m.mover_faction == DefaultSchedule0081ShipFaction::Pirate)
        })
        .unwrap();
    assert_ne!(terran.mover_id, pirate.mover_id);
    assert_ne!(terran.owner_id_after, pirate.owner_id_after);
}

#[test]
fn default_schedule_0080_1_owner_entity_not_spatial_parent() {
    let admitted = report();
    assert!(!admitted.owner_entity_as_spatial_parent);

    let rejected = rejected_with(|f| f.owner_entity_as_spatial_parent = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"owner_entity_as_spatial_parent"));
}
