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
