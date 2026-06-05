use simthing_driver::{
    export_gameplay_0080_1_text, observe_gameplay_0080_1, replay_observe_gameplay_0080_1,
    run_default_schedule_0080_1, DefaultSchedule0081Input, DefaultSchedule0081ShipFaction,
    Gameplay0081ForbiddenRequests, Gameplay0081Input, GAMEPLAY_0080_1_ID, GAMEPLAY_0080_1_SCENARIO,
    GAMEPLAY_0080_1_STATUS_PASS,
};

fn report() -> simthing_driver::Gameplay0081ObservationReport {
    observe_gameplay_0080_1(&Gameplay0081Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut Gameplay0081ForbiddenRequests),
) -> simthing_driver::Gameplay0081ObservationReport {
    let mut input = Gameplay0081Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    observe_gameplay_0080_1(&input)
}

#[test]
fn gameplay_0080_1_readonly_observation_explicit_opt_in_only() {
    let disabled = observe_gameplay_0080_1(&Gameplay0081Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.default_off);
    assert!(disabled.read_only);
    assert_eq!(disabled.executed_step_count, 0);
    assert!(disabled.transcript.rows.is_empty());

    let mut default_on = Gameplay0081Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = observe_gameplay_0080_1(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"gameplay_0080_1_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(admitted.read_only);
}

#[test]
fn gameplay_0080_1_consumes_default_schedule_report() {
    let schedule = run_default_schedule_0080_1(&DefaultSchedule0081Input::explicit_opt_in());
    let observed = observe_gameplay_0080_1(&Gameplay0081Input::explicit_opt_in_from_report(
        schedule.clone(),
    ));
    assert!(observed.admitted);
    assert!(observed.schedule_report_consumed);
    assert!(!observed.schedule_invoked_by_observer);
    assert_eq!(observed.executed_step_count, schedule.executed_step_count);
    assert_eq!(observed.transcript.rows.len(), schedule.step_reports.len());
    assert_eq!(
        observed.replay_checksum,
        schedule.deterministic_replay_checksum
    );

    let invoked = report();
    assert!(invoked.schedule_invoked_by_observer);
}

#[test]
fn gameplay_0080_1_exports_nested_starmap_transcript() {
    let admitted = report();
    assert_eq!(admitted.scenario_id, "SCENARIO-0080-1");
    assert_eq!(admitted.scenario_name, "Nested Starmap");
    assert_eq!(admitted.transcript.rows.len(), 3);
    assert!(!admitted.text_export.is_empty());
    assert!(admitted.text_export.contains("GAMEPLAY-0080-1"));
    assert!(admitted.text_export.contains("scenario_id=SCENARIO-0080-1"));
    assert!(admitted
        .text_export
        .contains("scenario_name=Nested Starmap"));
    assert!(admitted.text_export.contains("MOVE|step=0"));
    assert!(admitted.text_export.contains("MOVE|step=1"));
    assert!(admitted.text_export.contains("MOVE|step=2"));
    assert_eq!(export_gameplay_0080_1_text(&admitted), admitted.text_export);
}

#[test]
fn gameplay_0080_1_includes_atlas_residency_summary() {
    let admitted = report();
    assert_eq!(admitted.summary.starmap_shape.starmap_side, 10);
    assert_eq!(admitted.summary.starmap_shape.starsystem_count, 10);
    assert_eq!(admitted.summary.starmap_shape.starsystem_side, 10);
    assert_eq!(admitted.summary.starmap_shape.planet_side, 10);
    assert_eq!(admitted.summary.starmap_shape.logical_location_count, 2_100);
    assert!(admitted.summary.atlas.sparse_residency_composed);
    assert!(!admitted.summary.atlas.active_theaters.is_empty());
    assert!(!admitted.summary.atlas.resident_theaters.is_empty());
    assert!(admitted.text_export.contains("active_theaters="));
    assert!(admitted.text_export.contains("resident_theaters="));
}

#[test]
fn gameplay_0080_1_includes_faction_index_econ_summary() {
    let admitted = report();
    assert!(
        admitted
            .summary
            .faction_econ
            .fixed_terran_pirate_faction_set
    );
    assert!(
        admitted
            .summary
            .faction_econ
            .pirate_full_economy_participation
    );
    assert!(admitted.summary.faction_econ.contended_econ_visible);
    assert_eq!(admitted.summary.faction_econ.faction_count, 2);
    assert!(admitted.text_export.contains("fixed_factions=true"));
    assert!(admitted.text_export.contains("pirate_full_economy=true"));
    assert!(admitted.text_export.contains("contended_econ=true"));
}

#[test]
fn gameplay_0080_1_includes_owner_overlay_and_up_aggregation_summary() {
    let admitted = report();
    assert!(admitted
        .summary
        .owner_overlay_inheritance_summary
        .contains("terran=80100"));
    assert!(admitted
        .summary
        .owner_overlay_inheritance_summary
        .contains("pirate=80200"));
    assert!(admitted
        .summary
        .ownership_up_aggregation_summary
        .contains("planet_to_starsystem=true"));
    assert!(admitted
        .summary
        .ownership_up_aggregation_summary
        .contains("capture_as_reparenting=false"));
    assert!(admitted.text_export.contains("OWNER|overlay="));
}

#[test]
fn gameplay_0080_1_includes_field_policy_movement_trace() {
    let admitted = report();
    assert!(admitted.summary.field_policy_movement_trace_included);
    assert!(admitted
        .transcript
        .rows
        .iter()
        .all(|row| row.threshold_accepted && row.event_emitted));
    assert!(admitted
        .transcript
        .rows
        .iter()
        .all(|row| row.boundary_request_materialized));
    assert!(!admitted.observer_emitted_events);
    assert!(!admitted.observer_materialized_boundary_requests);
}

#[test]
fn gameplay_0080_1_includes_terran_and_pirate_movement_rows() {
    let admitted = report();
    assert_eq!(admitted.summary.terran_movement_rows, 1);
    assert_eq!(admitted.summary.pirate_movement_rows, 1);
    assert_eq!(admitted.summary.no_mover_rows, 1);
    assert!(admitted
        .transcript
        .rows
        .iter()
        .any(|row| row.mover_faction == Some(DefaultSchedule0081ShipFaction::Terran)));
    assert!(admitted
        .transcript
        .rows
        .iter()
        .any(|row| row.mover_faction == Some(DefaultSchedule0081ShipFaction::Pirate)));
    assert!(admitted
        .transcript
        .rows
        .iter()
        .any(|row| row.mover_id.is_none()));
}

#[test]
fn gameplay_0080_1_replay_transcript_deterministic() {
    let (first, second) = replay_observe_gameplay_0080_1();
    assert_eq!(first, second);
    assert_eq!(first.text_export, second.text_export);
    assert_eq!(first.transcript, second.transcript);
    assert_eq!(first.replay_checksum, second.replay_checksum);
}

#[test]
fn gameplay_0080_1_no_control_or_command_input() {
    let admitted = report();
    assert!(!admitted.control_or_command_input_present);

    let mut input = Gameplay0081Input::explicit_opt_in();
    input.surface.command_input_present = true;
    let rejected = observe_gameplay_0080_1(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"control_or_command_input"));

    let rejected = rejected_with(|forbidden| forbidden.control_or_command_input = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"control_or_command_input"));
}

#[test]
fn gameplay_0080_1_no_demo_packaging() {
    let admitted = report();
    assert!(!admitted.demo_packaging_present);

    let rejected = rejected_with(|forbidden| forbidden.demo_packaging = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"demo_packaging"));
}

#[test]
fn gameplay_0080_1_no_ui_framework() {
    let admitted = report();
    assert!(!admitted.ui_framework_present);

    let rejected = rejected_with(|forbidden| forbidden.ui_framework = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"ui_framework"));
}

#[test]
fn gameplay_0080_1_no_realtime_loop() {
    let admitted = report();
    assert!(!admitted.realtime_loop_present);

    let rejected = rejected_with(|forbidden| forbidden.realtime_loop = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"realtime_loop"));
}

#[test]
fn gameplay_0080_1_no_global_default_schedule() {
    let admitted = report();
    assert!(!admitted.global_default_schedule_registered);

    let rejected = rejected_with(|forbidden| forbidden.global_default_schedule = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_default_schedule"));
}

#[test]
fn gameplay_0080_1_no_direct_movement_or_external_boundary_request() {
    let admitted = report();
    assert!(!admitted.direct_movement_command);
    assert!(!admitted.external_boundary_request);

    let rejected = rejected_with(|forbidden| forbidden.direct_movement_command = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"direct_movement_command"));

    let rejected = rejected_with(|forbidden| forbidden.external_boundary_request = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"external_boundary_request"));
}

#[test]
fn gameplay_0080_1_no_cpu_planner_or_commitment() {
    let admitted = report();
    assert!(!admitted.cpu_planner_urgency_commitment);

    let rejected = rejected_with(|forbidden| forbidden.cpu_planner_urgency_commitment = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn gameplay_0080_1_no_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(!admitted.semantic_or_raw_wgsl_present);

    let rejected = rejected_with(|forbidden| forbidden.semantic_or_raw_wgsl = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn gameplay_0080_1_no_new_shader_or_gpu_kernel() {
    let admitted = report();
    assert!(!admitted.new_shader_or_gpu_kernel);

    let rejected = rejected_with(|forbidden| forbidden.new_shader_or_gpu_kernel = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"new_shader_or_gpu_kernel"));
}

#[test]
fn gameplay_0080_1_no_hard_currency_markets_trade_aibudget() {
    let admitted = report();
    assert!(!admitted.hard_currency_markets_trade_aibudget);

    let rejected = rejected_with(|forbidden| {
        forbidden.hard_currency_markets_trade_aibudget = true;
    });
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"hard_currency_markets_trade_aibudget"));
}

#[test]
fn gameplay_0080_1_no_nested_resource_flow() {
    let admitted = report();
    assert!(!admitted.nested_resource_flow);

    let rejected = rejected_with(|forbidden| forbidden.nested_resource_flow = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"nested_resource_flow"));
}

#[test]
fn gameplay_0080_1_no_clausething_dependency() {
    let admitted = report();
    assert!(!admitted.clausething_dependency_present);
    assert!(!admitted.simthing_spec_altered);
    assert!(!admitted.invariant_edited);
    assert!(!admitted.passive_proof_wrapper_present);
    assert!(!admitted.general_gameplay_framework_present);

    let rejected = rejected_with(|forbidden| forbidden.clausething_dependency = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));

    let rejected = rejected_with(|forbidden| forbidden.simthing_spec_alteration = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"simthing_spec_alteration"));

    let rejected = rejected_with(|forbidden| forbidden.invariant_edit = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"invariant_edit"));

    let rejected = rejected_with(|forbidden| forbidden.passive_proof_wrapper = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"passive_proof_wrapper"));

    let rejected = rejected_with(|forbidden| forbidden.general_gameplay_framework = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"general_gameplay_framework"));
}

#[test]
fn gameplay_0080_1_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.observation_id, GAMEPLAY_0080_1_ID);
    assert_eq!(admitted.status, GAMEPLAY_0080_1_STATUS_PASS);
    assert_eq!(admitted.scenario_name, GAMEPLAY_0080_1_SCENARIO);
    assert_eq!(admitted.schedule_id, "DEFAULT-SCHEDULE-0080-1");
    assert!(admitted.schedule_admitted);
}
