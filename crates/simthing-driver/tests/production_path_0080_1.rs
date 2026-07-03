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
fn production_path_0080_1_explicit_opt_in_only() {
    let disabled = run_production_path_0080_1(&ProductionPath0081Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.atlas_report.is_none());
    assert!(disabled.econ_scale_report.is_none());
    assert!(!disabled.nested_starmap_instantiated);

    let mut default_on = ProductionPath0081Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_production_path_0080_1(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"production_path_0080_1_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
}

#[test]
fn production_path_0080_1_requires_atlas_and_econ_scale_admitted() {
    let admitted = report();
    assert!(admitted.atlas_report_admitted_pass);
    assert!(admitted.econ_scale_report_admitted_pass);
    assert!(admitted.atlas_report.as_ref().unwrap().admitted);
    assert!(admitted.econ_scale_report.as_ref().unwrap().admitted);

    let mut atlas_rejected = ProductionPath0081Input::explicit_opt_in();
    atlas_rejected.atlas_input.forbidden.semantic_or_raw_wgsl = true;
    let rejected = run_production_path_0080_1(&atlas_rejected);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"atlas_0080_0_disabled_rejected_or_not_admitted"));
    assert!(rejected.atlas_report.is_some());

    let mut econ_rejected = ProductionPath0081Input::explicit_opt_in();
    econ_rejected
        .econ_scale_input
        .forbidden
        .nested_resource_flow = true;
    let rejected = run_production_path_0080_1(&econ_rejected);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"econ_scale_0080_0_disabled_rejected_or_not_admitted"));
    assert!(rejected.econ_scale_report.is_some());
}
#[test]
fn production_path_0080_1_instantiates_nested_starmap_shape() {
    let admitted = report();
    assert!(admitted.nested_starmap_instantiated);
    assert_eq!(admitted.starmap_side, 10);
    assert_eq!(admitted.starsystem_count, 10);
    assert_eq!(admitted.starsystem_side, 10);
    assert_eq!(admitted.planets_per_starsystem, 1);
    assert_eq!(admitted.planet_side, 10);
    assert_eq!(admitted.logical_location_count, 2_100);
}

#[test]
fn production_path_0080_1_composes_sparse_residency_report() {
    let admitted = report();
    assert!(admitted.sparse_residency_composed);
    let atlas = admitted.atlas_report.as_ref().unwrap();
    assert!(atlas.sparse_residency);
    assert!(!atlas.residency_reports.is_empty());
    assert_eq!(admitted.active_theaters, vec![Atlas0080TheaterId::Starmap]);
    assert_eq!(admitted.resident_theaters, admitted.active_theaters);
    assert!(atlas
        .residency_reports
        .iter()
        .any(|step| step
            .resident_theaters
            .contains(&Atlas0080TheaterId::Planet {
                starsystem_index: 0
            })));
}

#[test]
fn production_path_0080_1_composes_faction_index_econ_report() {
    let admitted = report();
    assert!(admitted.fixed_terran_pirate_faction_set);
    assert!(admitted.factions.contains(&EconScale0080Faction::Terran));
    assert!(admitted.factions.contains(&EconScale0080Faction::Pirate));
    assert!(admitted.contended_clearing_reports_visible);
    assert!(
        admitted
            .econ_scale_report
            .as_ref()
            .unwrap()
            .adversarial_contended_clearing
    );
    assert!(!admitted.hard_currency_markets_trade_aibudget);
    assert!(!admitted.nested_resource_flow);
    assert!(!admitted.unbounded_factions);
}

#[test]
fn production_path_0080_1_reports_owner_overlay_inheritance() {
    let admitted = report();
    let summary = admitted.owner_overlay_summary;
    assert!(summary.faction_owner_simthings_are_session_siblings);
    assert!(summary.location_owner_overlays_inherit_numeric_weights);
    assert!(summary.ship_owner_overlays_inherit_faction_weights);
    assert!(summary.no_new_owner_substrate_opened);
    assert_ne!(summary.terran_policy_weight, summary.pirate_policy_weight);
    assert!(!summary.owner_entity_as_spatial_parent);
}

#[test]
fn production_path_0080_1_reports_ownership_up_aggregation() {
    let admitted = report();
    let summary = admitted.ownership_aggregation_summary;
    assert!(summary.derived_owner_overlay_summary);
    assert!(summary.planet_to_starsystem_up_aggregation);
    assert_eq!(summary.terran_owned_planets, 6);
    assert_eq!(summary.terran_owned_starsystems_derived, 6);
    assert_eq!(summary.neutral_starsystems, 4);
    assert!(!summary.capture_as_reparenting);
    assert!(!summary.spatial_reparenting_used);
}

#[test]
fn production_path_0080_1_reports_field_policy_composite_gap_terms_readonly() {
    let admitted = report();
    let terms = admitted.field_policy_composite_gap_terms;
    assert_eq!(
        terms.composite_gap_sum,
        terms.current_space_minus_inherited_setpoint
            + terms.supply_security_gap
            + terms.bilateral_relational_gap
    );
    assert!(terms.read_only_terms_only);
    assert!(!terms.movement_execution);
    assert!(!terms.schedule_execution);
    assert!(!terms.direct_move_request);
    assert!(!terms.external_boundary_request);
    assert!(!terms.cpu_planner_urgency_or_commitment);
    assert!(!terms.new_field_policy_substrate);
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

#[test]
fn production_path_0080_1_no_schedule_observation_control_demo() {
    let admitted = report();
    assert!(!admitted.schedule_observation_control_demo_0080_1);

    let rejected = rejected_with(|f| f.schedule_observation_control_demo_0080_1 = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"schedule_observation_control_demo_0080_1"));
}

#[test]
fn production_path_0080_1_no_default_session_pass_graph_wiring() {
    let admitted = report();
    assert!(!admitted.default_session_pass_graph_wiring);

    let rejected = rejected_with(|f| f.default_session_pass_graph_wiring = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"default_session_pass_graph_wiring"));
}

#[test]
fn production_path_0080_1_no_global_default_schedule() {
    let admitted = report();
    assert!(!admitted.global_default_schedule);

    let rejected = rejected_with(|f| f.global_default_schedule = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_default_schedule"));
}

#[test]
fn production_path_0080_1_no_realtime_loop_or_ui() {
    let admitted = report();
    assert!(!admitted.realtime_loop_or_ui);

    let rejected = rejected_with(|f| f.realtime_loop_or_ui = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"realtime_loop_or_ui"));
}

#[test]
fn production_path_0080_1_no_direct_movement_or_external_boundary_request() {
    let admitted = report();
    assert!(!admitted.direct_movement_command);
    assert!(!admitted.external_boundary_request);

    let rejected = rejected_with(|f| f.direct_movement_command = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"direct_movement_command"));

    let rejected = rejected_with(|f| f.external_boundary_request = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"external_boundary_request"));
}

#[test]
fn production_path_0080_1_no_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(!admitted.semantic_or_raw_wgsl_present);
    assert!(!admitted.new_shader_or_gpu_kernel);

    let rejected = rejected_with(|f| f.semantic_or_raw_wgsl = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));

    let rejected = rejected_with(|f| f.new_shader_or_gpu_kernel = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"new_shader_or_gpu_kernel"));
}

#[test]
fn production_path_0080_1_no_hard_currency_markets_trade_aibudget() {
    let admitted = report();
    assert!(!admitted.hard_currency_markets_trade_aibudget);

    let rejected = rejected_with(|f| f.hard_currency_markets_trade_aibudget = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"hard_currency_markets_trade_aibudget"));
}

#[test]
fn production_path_0080_1_no_nested_resource_flow() {
    let admitted = report();
    assert!(!admitted.nested_resource_flow);

    let rejected = rejected_with(|f| f.nested_resource_flow = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"nested_resource_flow"));
}

#[test]
fn production_path_0080_1_no_clausething_dependency() {
    let admitted = report();
    assert!(!admitted.clausething_dependency_present);
    assert!(!admitted.simthing_spec_altered);
    assert!(!admitted.invariant_edited);
    assert!(!admitted.passive_proof_wrapper);
    assert!(!admitted.general_production_path);

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

    let rejected = rejected_with(|f| f.general_production_path = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"general_production_path"));
}

#[test]
fn production_path_0080_1_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.path_id, PRODUCTION_PATH_0080_1_ID);
    assert_eq!(admitted.scenario_gate_id, SCENARIO_0080_1_GATE_ID);
    assert_eq!(admitted.status, PRODUCTION_PATH_0080_1_STATUS_PASS);
    assert!(admitted.scenario_scoped_only);
}

#[test]
fn production_path_0080_1_pirate_full_economy_faction_visible_in_report() {
    let admitted = report();
    assert!(admitted.pirate_full_economy_faction_visible);
    assert!(
        admitted
            .econ_scale_report
            .as_ref()
            .unwrap()
            .pirate_is_full_economy_faction
    );
}

#[test]
fn production_path_0080_1_owner_entity_not_spatial_parent() {
    let admitted = report();
    assert!(
        !admitted
            .owner_overlay_summary
            .owner_entity_as_spatial_parent
    );

    let rejected = rejected_with(|f| f.owner_entity_as_spatial_parent = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"owner_entity_as_spatial_parent"));
}
