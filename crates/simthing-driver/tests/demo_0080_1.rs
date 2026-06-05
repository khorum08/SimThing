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
fn demo_0080_1_explicit_opt_in_only() {
    let disabled = run_demo_0080_1(&Demo0081Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.movement_rows.is_empty());
    assert!(disabled.command_transcript.is_empty());

    let mut default_on = Demo0081Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_demo_0080_1(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"demo_0080_1_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(!admitted.disabled_no_op);
}

#[test]
fn demo_0080_1_runs_canonical_control_batch() {
    let admitted = report();
    assert!(admitted.control_admitted);
    assert!(admitted.applied_command_count > 0);
    assert!(!admitted.command_transcript.is_empty());
}

#[test]
fn demo_0080_1_uses_existing_control_schedule_observation_path() {
    let admitted = report();
    assert!(admitted.control_ran_schedule);
    assert_eq!(admitted.control_id, "CONTROL-0080-1");
    assert!(admitted.executed_step_count > 0);
}

#[test]
fn demo_0080_1_emits_nested_starmap_export() {
    let admitted = report();
    assert!(!admitted.demo_text_export.is_empty());
    assert!(admitted.demo_text_export.contains("DEMO-0080-1"));
    assert!(admitted.demo_text_export.contains("Nested Starmap"));
}

#[test]
fn demo_0080_1_includes_command_transcript() {
    let admitted = report();
    assert!(!admitted.command_transcript.is_empty());
    assert!(admitted.demo_text_export.contains("CMD|"));
}

#[test]
fn demo_0080_1_includes_terran_and_pirate_movement_rows() {
    let admitted = report();
    assert!(!admitted.movement_rows.is_empty());
    assert!(admitted.terran_move_count > 0 || admitted.pirate_move_count > 0);
    let has_terran = admitted
        .movement_rows
        .iter()
        .any(|r| r.mover_faction == "Terran");
    let has_pirate = admitted
        .movement_rows
        .iter()
        .any(|r| r.mover_faction == "Pirate");
    assert!(
        has_terran || has_pirate,
        "expected at least one Terran or Pirate row"
    );
}

#[test]
fn demo_0080_1_includes_atlas_residency_summary() {
    let admitted = report();
    assert!(admitted.atlas_residency_summary_present);
}

#[test]
fn demo_0080_1_includes_faction_index_econ_summary() {
    let admitted = report();
    assert!(admitted.faction_index_econ_summary_present);
}

#[test]
fn demo_0080_1_includes_owner_overlay_and_up_aggregation_summary() {
    let admitted = report();
    assert!(admitted.owner_overlay_summary_present);
    assert!(admitted.ownership_up_aggregation_summary_present);
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

#[test]
fn demo_0080_1_no_cli_binary_unless_authorized() {
    let admitted = report();
    assert!(admitted.no_cli_binary);

    let mut input = Demo0081Input::explicit_opt_in();
    input.surface.cli_binary_present = true;
    let rejected = run_demo_0080_1(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"cli_binary_not_authorized"));
}

#[test]
fn demo_0080_1_no_direct_movement_command() {
    let admitted = report();
    assert!(admitted.no_direct_movement_command);

    let rejected = rejected_with(|f| f.direct_movement_command = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"direct_movement_command"));
}

#[test]
fn demo_0080_1_no_external_boundary_request() {
    let admitted = report();
    assert!(admitted.no_external_boundary_request);

    let rejected = rejected_with(|f| f.external_boundary_request = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"external_boundary_request"));
}

#[test]
fn demo_0080_1_no_field_policy_bypass() {
    let admitted = report();
    assert!(admitted.no_field_policy_bypass);

    let rejected = rejected_with(|f| f.field_policy_bypass = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"field_policy_bypass"));
}

#[test]
fn demo_0080_1_no_player_command_loop() {
    let admitted = report();
    assert!(admitted.no_player_command_loop);

    let rejected = rejected_with(|f| f.player_command_loop = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"player_command_loop"));
}

#[test]
fn demo_0080_1_no_ui_framework() {
    let admitted = report();
    assert!(admitted.no_ui_framework);

    let rejected = rejected_with(|f| f.ui_framework = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"ui_framework"));
}

#[test]
fn demo_0080_1_no_realtime_loop() {
    let admitted = report();
    assert!(admitted.no_realtime_loop);

    let rejected = rejected_with(|f| f.realtime_loop = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"realtime_loop"));
}

#[test]
fn demo_0080_1_no_global_default_schedule() {
    let admitted = report();
    assert!(admitted.no_global_default_schedule);

    let rejected = rejected_with(|f| f.global_default_schedule = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_default_schedule"));
}

#[test]
fn demo_0080_1_no_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(admitted.no_semantic_or_raw_wgsl);

    let rejected = rejected_with(|f| f.semantic_or_raw_wgsl = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn demo_0080_1_no_new_shader_or_gpu_kernel() {
    let admitted = report();
    assert!(admitted.no_new_shader_or_gpu_kernel);

    let rejected = rejected_with(|f| f.new_shader_or_gpu_kernel = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"new_shader_or_gpu_kernel"));
}

#[test]
fn demo_0080_1_no_hard_currency_markets_trade_aibudget() {
    let admitted = report();
    assert!(admitted.no_hard_currency_markets_trade_aibudget);

    let rejected = rejected_with(|f| f.hard_currency_markets_trade_aibudget = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"hard_currency_markets_trade_aibudget"));
}

#[test]
fn demo_0080_1_no_nested_resource_flow() {
    let admitted = report();
    assert!(admitted.no_nested_resource_flow);

    let rejected = rejected_with(|f| f.nested_resource_flow = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"nested_resource_flow"));
}

#[test]
fn demo_0080_1_no_clausething_dependency() {
    let admitted = report();
    assert!(admitted.no_clausething_dependency);

    let rejected = rejected_with(|f| f.clausething_dependency = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));
}

#[test]
fn demo_0080_1_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.demo_id, DEMO_0080_1_ID);
    assert_eq!(admitted.status, DEMO_0080_1_STATUS_PASS);
    assert_eq!(admitted.scenario_name, DEMO_0080_1_SCENARIO);
}
