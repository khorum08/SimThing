use simthing_driver::{
    replay_atlas_0080_0, run_atlas_0080_0, Atlas0080ForbiddenRequests, Atlas0080Input,
    Atlas0080ResidencyRequest, Atlas0080Scenario, Atlas0080TheaterId, ATLAS_0080_0_ID,
    ATLAS_0080_0_LOGICAL_LOCATION_COUNT, ATLAS_0080_0_PLANET_SIDE, ATLAS_0080_0_SCENARIO,
    ATLAS_0080_0_STARMAP_SIDE, ATLAS_0080_0_STARSYSTEM_COUNT, ATLAS_0080_0_STARSYSTEM_SIDE,
    ATLAS_0080_0_STATUS_PASS,
};

fn report() -> simthing_driver::Atlas0080Report {
    run_atlas_0080_0(&Atlas0080Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut Atlas0080ForbiddenRequests),
) -> simthing_driver::Atlas0080Report {
    let mut input = Atlas0080Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_atlas_0080_0(&input)
}

#[test]
fn atlas_0080_0_explicit_opt_in_only() {
    let disabled = run_atlas_0080_0(&Atlas0080Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.default_session_has_no_residency_runtime);
    assert!(disabled.residency_reports.is_empty());

    let mut default_on = Atlas0080Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_atlas_0080_0(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"atlas_0080_0_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
}

#[test]
fn atlas_0080_0_default_session_has_no_residency_runtime() {
    let disabled = run_atlas_0080_0(&Atlas0080Input::default_simsession());
    assert!(disabled.disabled_no_op);
    assert!(disabled.default_session_has_no_residency_runtime);
    assert!(!disabled.default_session_pass_graph_wiring);
    assert_eq!(disabled.max_resident_cell_count, 0);
}

#[test]
fn atlas_0080_0_nested_starmap_shape_bounded() {
    let admitted = report();
    assert_eq!(admitted.starmap_side, ATLAS_0080_0_STARMAP_SIDE);
    assert_eq!(admitted.starsystem_count, ATLAS_0080_0_STARSYSTEM_COUNT);
    assert_eq!(admitted.starsystem_side, ATLAS_0080_0_STARSYSTEM_SIDE);
    assert_eq!(admitted.planet_count, ATLAS_0080_0_STARSYSTEM_COUNT);
    assert_eq!(admitted.planet_side, ATLAS_0080_0_PLANET_SIDE);
    assert_eq!(
        admitted.logical_location_count,
        ATLAS_0080_0_LOGICAL_LOCATION_COUNT
    );
    assert_eq!(admitted.total_logical_cell_count, 2_100);
}

#[test]
fn atlas_0080_0_deterministic_starsystem_seed_selection() {
    let first = Atlas0080Scenario::from_seed(0xAA55);
    let second = Atlas0080Scenario::from_seed(0xAA55);
    let different = Atlas0080Scenario::from_seed(0xAA56);
    assert_eq!(first.starsystem_cells, second.starsystem_cells);
    assert_eq!(first.planet_cells, second.planet_cells);
    assert_ne!(first.starsystem_cells, different.starsystem_cells);

    let mut unique = first.starsystem_cells.clone();
    unique.sort_by_key(|cell| (cell.y, cell.x));
    unique.dedup();
    assert_eq!(unique.len(), ATLAS_0080_0_STARSYSTEM_COUNT);
}

#[test]
fn atlas_0080_0_sparse_residency_only_active_theaters() {
    let admitted = report();
    assert!(admitted.sparse_residency);
    assert!(admitted.max_resident_cell_count < admitted.total_logical_cell_count);
    assert!(admitted
        .residency_reports
        .iter()
        .all(|step| step.sparse_residency_only_active_theaters));
    assert!(admitted
        .residency_reports
        .iter()
        .all(|step| step.resident_theaters == step.active_theaters_after));
    assert!(admitted
        .residency_reports
        .iter()
        .any(|step| step
            .resident_theaters
            .contains(&Atlas0080TheaterId::Planet {
                starsystem_index: 0,
            })));
}

#[test]
fn atlas_0080_0_nested_descent_ascent_deterministic() {
    let admitted = report();
    let requests: Vec<_> = admitted
        .descent_ascent_reports
        .iter()
        .map(|step| step.request)
        .collect();
    assert_eq!(requests, Atlas0080Input::canonical_access_pattern());
    assert!(admitted
        .descent_ascent_reports
        .iter()
        .all(|step| step.deterministic));
    assert_eq!(
        admitted
            .residency_reports
            .last()
            .unwrap()
            .active_theaters_after,
        vec![Atlas0080TheaterId::Starmap]
    );
}

#[test]
fn atlas_0080_0_residency_is_value_noop_parity_bit_exact() {
    let admitted = report();
    assert!(admitted.value_noop_parity_bit_exact);
    assert!(admitted
        .residency_reports
        .iter()
        .all(|step| !step.residency_changes_values && step.value_noop_parity_bit_exact));
    assert!(admitted
        .residency_reports
        .iter()
        .all(|step| step.materialized_i8_values_after.len() == step.resident_cell_count as usize));

    let rejected = rejected_with(|forbidden| forbidden.residency_alters_field_values = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"residency_alters_field_values"));
}

#[test]
fn atlas_0080_0_residency_within_vram_budget() {
    let admitted = report();
    assert!(admitted.within_vram_budget);
    assert!(admitted.max_estimated_resident_bytes <= admitted.vram_budget_bytes);
    assert!(admitted
        .residency_reports
        .iter()
        .all(|step| step.within_vram_budget));
}

#[test]
fn atlas_0080_0_replay_deterministic() {
    let (first, second) = replay_atlas_0080_0();
    assert!(first.admitted, "{:?}", first.diagnostics);
    assert!(second.admitted, "{:?}", second.diagnostics);
    assert_eq!(
        first.deterministic_replay_checksum,
        second.deterministic_replay_checksum
    );
    assert_eq!(first.residency_reports, second.residency_reports);
    assert_eq!(first.descent_ascent_reports, second.descent_ascent_reports);
}

#[test]
fn atlas_0080_0_no_default_session_pass_graph_wiring() {
    let admitted = report();
    assert!(!admitted.default_session_pass_graph_wiring);

    let mut input = Atlas0080Input::explicit_opt_in();
    input.surface.default_session_pass_graph_wiring = true;
    let rejected = run_atlas_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"default_session_pass_graph_wiring"));
}

#[test]
fn atlas_0080_0_no_realtime_loop() {
    let mut input = Atlas0080Input::explicit_opt_in();
    input.surface.realtime_loop = true;
    let rejected = run_atlas_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"realtime_loop"));
}

#[test]
fn atlas_0080_0_no_global_mapping_scheduler() {
    let mut input = Atlas0080Input::explicit_opt_in();
    input.surface.global_mapping_scheduler = true;
    let rejected = run_atlas_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"global_mapping_scheduler"));
}

#[test]
fn atlas_0080_0_no_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(!admitted.semantic_or_raw_wgsl_present);

    let rejected = rejected_with(|forbidden| forbidden.semantic_or_raw_wgsl = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));

    let rejected = rejected_with(|forbidden| forbidden.semantically_named_shader = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantically_named_shader"));
}

#[test]
fn atlas_0080_0_no_clausething_dependency() {
    let admitted = report();
    assert!(!admitted.clausething_dependency_present);

    let rejected = rejected_with(|forbidden| forbidden.clausething_dependency = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));
}

#[test]
fn atlas_0080_0_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.atlas_id, ATLAS_0080_0_ID);
    assert_eq!(admitted.scenario_name, ATLAS_0080_0_SCENARIO);
    assert_eq!(admitted.status, ATLAS_0080_0_STATUS_PASS);
    assert!(admitted.scenario_scoped_only);
    assert!(!admitted.econ_scale_0080_0_implemented);
    assert!(!admitted.production_path_0080_1_implemented);

    let rejected = rejected_with(|forbidden| forbidden.econ_scale_0080_0 = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"econ_scale_0080_0_not_implemented"));

    let rejected = rejected_with(|forbidden| forbidden.production_path_0080_1 = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"production_path_0080_1_not_implemented"));
}
#[test]
fn atlas_0080_0_access_pattern_bounds_starsystem_indices() {
    let mut input = Atlas0080Input::explicit_opt_in();
    input.access_pattern = vec![Atlas0080ResidencyRequest::DescendToStarsystem {
        starsystem_index: ATLAS_0080_0_STARSYSTEM_COUNT as u8,
    }];
    let rejected = run_atlas_0080_0(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"starsystem_index_out_of_bounds"));
}
