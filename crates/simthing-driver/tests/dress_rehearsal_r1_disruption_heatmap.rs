use simthing_driver::{
    bounded_feedback_next, cpu_oracle_dress_rehearsal_r1_disruption_heatmap,
    dress_rehearsal_r1_cell_index, replay_dress_rehearsal_r1_disruption_heatmap,
    run_dress_rehearsal_r1_disruption_heatmap, DressRehearsalR1Channel,
    DressRehearsalR1ForbiddenRequests, DressRehearsalR1Input, DressRehearsalR1OccupantContribution,
    DressRehearsalR1OccupantKind, DressRehearsalR1Owner, DressRehearsalR1Report,
    DressRehearsalR1Scenario, CEILING, DECAY, DISRUPTION_COL,
    DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_ID, DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_STATUS_PASS,
    FLOOR, GALAXY_CELL_COUNT, GALAXY_SIDE, HOTSPOT_COUNT, LOCATION_STATUS_COL, PATROL_SUPPRESS,
    PIRATE_EMIT, SYSTEM_COUNT,
};
use simthing_spec::EmlGadgetInstanceSpec;

fn report() -> DressRehearsalR1Report {
    run_dress_rehearsal_r1_disruption_heatmap(&DressRehearsalR1Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut DressRehearsalR1ForbiddenRequests),
) -> DressRehearsalR1Report {
    let mut input = DressRehearsalR1Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_dress_rehearsal_r1_disruption_heatmap(&input)
}

fn contribution(
    id: &str,
    kind: DressRehearsalR1OccupantKind,
    owner: DressRehearsalR1Owner,
    x: u32,
    y: u32,
    channel: DressRehearsalR1Channel,
    value: f32,
) -> DressRehearsalR1OccupantContribution {
    DressRehearsalR1OccupantContribution {
        source_id: id.to_string(),
        kind,
        owner,
        x,
        y,
        cell_index: dress_rehearsal_r1_cell_index(x, y),
        channel,
        value,
    }
}

fn pirate(id: &str, x: u32, y: u32) -> DressRehearsalR1OccupantContribution {
    contribution(
        id,
        DressRehearsalR1OccupantKind::PirateFleet,
        DressRehearsalR1Owner::Pirate,
        x,
        y,
        DressRehearsalR1Channel::PirateDisruption,
        PIRATE_EMIT,
    )
}

fn patrol(id: &str, x: u32, y: u32) -> DressRehearsalR1OccupantContribution {
    contribution(
        id,
        DressRehearsalR1OccupantKind::PatrolFleet,
        DressRehearsalR1Owner::Terran,
        x,
        y,
        DressRehearsalR1Channel::PatrolSuppression,
        -PATROL_SUPPRESS,
    )
}

fn inert_system(
    id: &str,
    owner: DressRehearsalR1Owner,
    x: u32,
    y: u32,
) -> DressRehearsalR1OccupantContribution {
    contribution(
        id,
        DressRehearsalR1OccupantKind::System,
        owner,
        x,
        y,
        DressRehearsalR1Channel::InertSystem,
        0.0,
    )
}

fn run_custom(
    occupants: Vec<DressRehearsalR1OccupantContribution>,
    tick_count: u32,
    initial: Vec<f32>,
) -> DressRehearsalR1Report {
    let mut scenario = DressRehearsalR1Scenario::empty();
    scenario.occupants = occupants;
    let mut input = DressRehearsalR1Input::with_scenario(scenario, tick_count);
    input.initial_disruption = initial;
    run_dress_rehearsal_r1_disruption_heatmap(&input)
}

fn run_custom_zero(
    occupants: Vec<DressRehearsalR1OccupantContribution>,
    tick_count: u32,
) -> DressRehearsalR1Report {
    run_custom(occupants, tick_count, vec![0.0; GALAXY_CELL_COUNT])
}

fn cell_input(
    report: &DressRehearsalR1Report,
    x: u32,
    y: u32,
) -> &simthing_driver::DressRehearsalR1CellInput {
    let idx = dress_rehearsal_r1_cell_index(x, y);
    report
        .cell_inputs
        .iter()
        .find(|cell| cell.cell_index == idx)
        .expect("cell input")
}

fn disruption_at(report: &DressRehearsalR1Report, x: u32, y: u32) -> f32 {
    report.final_disruption[dress_rehearsal_r1_cell_index(x, y) as usize]
}

fn status_at(report: &DressRehearsalR1Report, x: u32, y: u32) -> f32 {
    report.location_status[dress_rehearsal_r1_cell_index(x, y) as usize]
}

#[test]
fn r1_explicit_opt_in_default_off() {
    assert_eq!(
        DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_ID,
        "SCENARIO-0080-2-R1-DISRUPTION-HEATMAP"
    );
    assert!(DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_STATUS_PASS.contains("IMPLEMENTED / PASS"));

    let disabled =
        run_dress_rehearsal_r1_disruption_heatmap(&DressRehearsalR1Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.final_disruption.is_empty());

    let mut default_on = DressRehearsalR1Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_dress_rehearsal_r1_disruption_heatmap(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"r1_default_on_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(!admitted.disabled_no_op);
}

#[test]
fn r1_heatmap_shape_20x20_preserves_13_systems() {
    let admitted = report();
    assert_eq!(admitted.galaxy_side, GALAXY_SIDE);
    assert_eq!(admitted.grid_cell_count, GALAXY_CELL_COUNT);
    assert_eq!(admitted.scenario.grid_cells.len(), 400);
    assert_eq!(admitted.system_count, SYSTEM_COUNT);
    assert_eq!(admitted.scenario.system_cells.len(), 13);
    assert_eq!(
        admitted
            .scenario
            .occupants
            .iter()
            .filter(|occupant| occupant.kind == DressRehearsalR1OccupantKind::System)
            .count(),
        13
    );
}

#[test]
fn r1_heatmap_cell_index_is_row_major_y_times_20_plus_x() {
    let admitted = report();
    for cell in &admitted.scenario.grid_cells {
        assert_eq!(cell.cell_index, cell.y * 20 + cell.x);
        assert_eq!(
            cell.cell_index,
            dress_rehearsal_r1_cell_index(cell.x, cell.y)
        );
    }
}

#[test]
fn r1_heatmap_allocates_disruption_and_location_status_columns() {
    let admitted = report();
    assert_eq!(admitted.disruption_col, DISRUPTION_COL);
    assert_eq!(admitted.location_status_col, LOCATION_STATUS_COL);
    assert!(admitted.source_target_columns_distinct);
    for cell in &admitted.scenario.grid_cells {
        assert_eq!(cell.disruption_col, DISRUPTION_COL);
        assert_eq!(cell.location_status_col, LOCATION_STATUS_COL);
        assert_ne!(cell.disruption_col, cell.location_status_col);
    }
    match admitted.bounded_feedback_gadget {
        EmlGadgetInstanceSpec::BoundedFeedback {
            previous_col,
            output_col,
            decay,
            gain,
            min,
            max,
            ..
        } => {
            assert_eq!(previous_col, DISRUPTION_COL);
            assert_eq!(output_col, Some(DISRUPTION_COL));
            assert_eq!(decay, DECAY);
            assert_eq!(gain, 1.0);
            assert_eq!(min, FLOOR);
            assert_eq!(max, CEILING);
        }
        other => panic!("unexpected gadget: {other:?}"),
    }
}

#[test]
fn r1_heatmap_occupants_are_contributors_not_merged_cells() {
    let admitted = report();
    let pirate_cell = admitted
        .cell_inputs
        .iter()
        .find(|cell| cell.pirate_count == 10)
        .expect("canonical shared pirate cell");
    assert_eq!(
        pirate_cell.separated_entries.len(),
        11,
        "10 fleets plus inert pirate system"
    );
    let pirate_sources: std::collections::HashSet<_> = pirate_cell
        .separated_entries
        .iter()
        .filter(|entry| entry.channel == DressRehearsalR1Channel::PirateDisruption)
        .map(|entry| entry.source_id.as_str())
        .collect();
    assert_eq!(pirate_sources.len(), 10);
    assert_eq!(
        admitted.scenario.grid_cells.len(),
        400,
        "cells remain cells, not fleets"
    );
}

#[test]
fn r1_heatmap_no_blind_sum_by_position_across_channels_or_owners() {
    let admitted = run_custom_zero(
        vec![
            inert_system("system-a", DressRehearsalR1Owner::Terran, 3, 3),
            pirate("pirate-a", 3, 3),
            patrol("patrol-a", 3, 3),
        ],
        1,
    );
    let cell = cell_input(&admitted, 3, 3);
    let keys: std::collections::HashSet<_> = cell
        .separated_entries
        .iter()
        .map(|entry| (entry.channel, entry.owner))
        .collect();
    assert!(keys.contains(&(
        DressRehearsalR1Channel::InertSystem,
        DressRehearsalR1Owner::Terran
    )));
    assert!(keys.contains(&(
        DressRehearsalR1Channel::PirateDisruption,
        DressRehearsalR1Owner::Pirate
    )));
    assert!(keys.contains(&(
        DressRehearsalR1Channel::PatrolSuppression,
        DressRehearsalR1Owner::Terran
    )));
    assert_eq!(cell.input_cell, PIRATE_EMIT - PATROL_SUPPRESS);
}

#[test]
fn r1_source_pirate_fleet_contributes_positive_disruption() {
    let admitted = report();
    let pirate_cell = admitted
        .cell_inputs
        .iter()
        .find(|cell| cell.pirate_count == 10)
        .expect("canonical pirate cell");
    assert_eq!(pirate_cell.pirate_contribution, 10.0 * PIRATE_EMIT);
    assert!(pirate_cell.input_cell > 0.0);
    assert_eq!(
        admitted.final_disruption[pirate_cell.cell_index as usize],
        CEILING
    );
}

#[test]
fn r1_source_patrol_fleet_contributes_suppression() {
    let admitted = report();
    let patrol_cell = admitted
        .cell_inputs
        .iter()
        .find(|cell| cell.patrol_count == 1)
        .expect("canonical patrol cell");
    assert_eq!(patrol_cell.patrol_suppression, PATROL_SUPPRESS);
    assert_eq!(patrol_cell.input_cell, -PATROL_SUPPRESS);
    assert_eq!(
        admitted.final_disruption[patrol_cell.cell_index as usize],
        FLOOR
    );
}

#[test]
fn r1_source_non_fleet_occupants_are_inert() {
    let admitted = report();
    let inert_only = admitted
        .cell_inputs
        .iter()
        .find(|cell| cell.inert_count > 0 && cell.pirate_count == 0 && cell.patrol_count == 0)
        .expect("inert system-only cell");
    assert_eq!(inert_only.input_cell, 0.0);
    assert_eq!(
        admitted.final_disruption[inert_only.cell_index as usize],
        0.0
    );
}

#[test]
fn r1_source_colocated_pirate_and_patrol_remain_channel_owner_separated_before_net() {
    let admitted = run_custom_zero(vec![pirate("pirate-a", 9, 9), patrol("patrol-a", 9, 9)], 1);
    let cell = cell_input(&admitted, 9, 9);
    assert_eq!(cell.pirate_count, 1);
    assert_eq!(cell.patrol_count, 1);
    assert_eq!(cell.separated_entries.len(), 2);
    assert_ne!(
        cell.separated_entries[0].owner,
        cell.separated_entries[1].owner
    );
    assert_ne!(
        cell.separated_entries[0].channel,
        cell.separated_entries[1].channel
    );
    assert_eq!(cell.input_cell, 5.0);
}

#[test]
fn r1_recurrence_same_inputs_same_outputs() {
    let left = report();
    let right = report();
    assert_eq!(left.final_disruption, right.final_disruption);
    assert_eq!(left.location_status, right.location_status);
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
}

#[test]
fn r1_recurrence_no_source_decays_by_0_8() {
    let mut initial = vec![0.0; GALAXY_CELL_COUNT];
    initial[0] = 50.0;
    let admitted = run_custom(Vec::new(), 1, initial);
    assert_eq!(disruption_at(&admitted, 0, 0), 50.0 * DECAY);
}

#[test]
fn r1_recurrence_floor_holds_under_patrol_suppression() {
    let mut initial = vec![0.0; GALAXY_CELL_COUNT];
    initial[dress_rehearsal_r1_cell_index(4, 4) as usize] = 5.0;
    let admitted = run_custom(vec![patrol("patrol-a", 4, 4)], 1, initial);
    assert_eq!(disruption_at(&admitted, 4, 4), FLOOR);
}

#[test]
fn r1_recurrence_ceiling_holds_under_pirate_emission() {
    let occupants: Vec<_> = (0..10)
        .map(|idx| pirate(&format!("pirate-{idx}"), 4, 4))
        .collect();
    let admitted = run_custom_zero(occupants, 1);
    assert_eq!(disruption_at(&admitted, 4, 4), CEILING);
}

#[test]
fn r1_recurrence_lone_pirate_converges_to_100() {
    let admitted = run_custom_zero(vec![pirate("pirate-a", 4, 4)], 32);
    let cell_index = dress_rehearsal_r1_cell_index(4, 4);
    let series: Vec<_> = admitted
        .recurrence_rows
        .iter()
        .filter(|row| row.cell_index == cell_index)
        .map(|row| row.disruption_after)
        .collect();
    assert!(series.windows(2).all(|pair| pair[1] >= pair[0]));
    let final_value = *series.last().expect("series");
    assert!(final_value > 99.0, "{final_value}");
    assert!(final_value <= CEILING);
}

#[test]
fn r1_recurrence_two_patrols_vs_one_pirate_floors_at_zero() {
    let admitted = run_custom_zero(
        vec![
            pirate("pirate-a", 4, 4),
            patrol("patrol-a", 4, 4),
            patrol("patrol-b", 4, 4),
        ],
        4,
    );
    assert_eq!(cell_input(&admitted, 4, 4).input_cell, -10.0);
    assert_eq!(disruption_at(&admitted, 4, 4), FLOOR);
}

#[test]
fn r1_recurrence_implementation_matches_cpu_oracle() {
    let input = DressRehearsalR1Input::explicit_opt_in();
    let admitted = run_dress_rehearsal_r1_disruption_heatmap(&input);
    let oracle = cpu_oracle_dress_rehearsal_r1_disruption_heatmap(&input);
    assert!(admitted.cpu_oracle_parity);
    assert_eq!(admitted.final_disruption, oracle.final_disruption);
    assert_eq!(admitted.location_status, oracle.location_status);
    assert_eq!(
        admitted.starmap_summary.stable_checksum,
        oracle.summary.stable_checksum
    );
    assert_eq!(bounded_feedback_next(10.0, 5.0), 13.0);
}

#[test]
fn r1_diffusion_writes_location_status_not_disruption() {
    let admitted = run_custom_zero(vec![pirate("pirate-a", 10, 10)], 1);
    assert_eq!(disruption_at(&admitted, 10, 10), PIRATE_EMIT);
    let row = admitted
        .diffusion_rows
        .iter()
        .find(|row| row.cell_index == dress_rehearsal_r1_cell_index(10, 10))
        .expect("diffusion row");
    assert_eq!(row.source_col, DISRUPTION_COL);
    assert_eq!(row.target_col, LOCATION_STATUS_COL);
    assert_eq!(row.disruption_source, PIRATE_EMIT);
    assert_eq!(disruption_at(&admitted, 10, 10), PIRATE_EMIT);
}

#[test]
fn r1_diffusion_falloff_reaches_von_neumann_neighbors() {
    let occupants: Vec<_> = (0..10)
        .map(|idx| pirate(&format!("pirate-{idx}"), 10, 10))
        .collect();
    let admitted = run_custom_zero(occupants, 1);
    assert!(status_at(&admitted, 11, 10) > 0.0);
    assert!(status_at(&admitted, 9, 10) > 0.0);
    assert!(status_at(&admitted, 10, 11) > 0.0);
    assert!(status_at(&admitted, 10, 9) > 0.0);
}

#[test]
fn r1_diffusion_falloff_decays_with_distance() {
    let occupants: Vec<_> = (0..10)
        .map(|idx| pirate(&format!("pirate-{idx}"), 10, 10))
        .collect();
    let admitted = run_custom_zero(occupants, 1);
    let center = status_at(&admitted, 10, 10);
    let adjacent = status_at(&admitted, 11, 10);
    let distance_two = status_at(&admitted, 12, 10);
    assert!(center > adjacent);
    assert!(adjacent > distance_two);
}

#[test]
fn r1_diffusion_no_inter_tile_bleed() {
    let occupants: Vec<_> = (0..10)
        .map(|idx| pirate(&format!("pirate-{idx}"), 0, 0))
        .collect();
    let admitted = run_custom_zero(occupants, 1);
    assert!(status_at(&admitted, 1, 0) > 0.0);
    assert!(status_at(&admitted, 0, 1) > 0.0);
    assert_eq!(status_at(&admitted, 19, 0), 0.0, "no x-wrap bleed");
    assert_eq!(status_at(&admitted, 0, 19), 0.0, "no y-wrap bleed");
}

#[test]
fn r1_heatmap_artifact_emitted() {
    let admitted = report();
    assert_eq!(admitted.artifact.rows.len(), GALAXY_CELL_COUNT);
    assert_eq!(admitted.artifact.hotspots.len(), HOTSPOT_COUNT);
    assert!(admitted
        .artifact
        .markdown
        .contains("R1 Disruption Heatmap Artifact"));
    assert!(admitted.artifact.markdown.contains("20x20 Cell Table"));
}

#[test]
fn r1_heatmap_artifact_deterministic_checksum_stable() {
    let (left, right) = replay_dress_rehearsal_r1_disruption_heatmap();
    assert_eq!(left, right);
    assert_eq!(
        left.artifact.summary.stable_checksum,
        right.artifact.summary.stable_checksum
    );
    assert_eq!(left.artifact.markdown, right.artifact.markdown);
}

#[test]
fn r1_heatmap_artifact_nonzero_field() {
    let admitted = report();
    assert!(admitted.starmap_summary.total_disruption > 0.0);
    assert!(admitted
        .artifact
        .rows
        .iter()
        .any(|row| row.disruption > 0.0));
}

#[test]
fn r1_heatmap_artifact_hotspot_near_pirate() {
    let admitted = report();
    let pirate_cell = admitted
        .cell_inputs
        .iter()
        .find(|cell| cell.pirate_count == 10)
        .expect("canonical pirate cell");
    assert_eq!(admitted.hotspots[0].cell_index, pirate_cell.cell_index);
    assert_eq!(admitted.hotspots[0].disruption, CEILING);
}

#[test]
fn r1_heatmap_artifact_suppressed_near_patrol() {
    let admitted = report();
    let patrol_cell = admitted
        .cell_inputs
        .iter()
        .find(|cell| cell.patrol_count == 1)
        .expect("canonical patrol cell");
    assert_eq!(
        admitted.final_disruption[patrol_cell.cell_index as usize],
        FLOOR
    );
    assert!(
        admitted.location_status[patrol_cell.cell_index as usize]
            < admitted.hotspots[0].location_status
    );
}

#[test]
fn r1_no_sead_movement() {
    let admitted = report();
    assert!(admitted.no_sead_movement);
    let rejected = rejected_with(|forbidden| forbidden.sead_movement = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"sead_movement"));
}

#[test]
fn r1_no_gradientxy_consumption() {
    let admitted = report();
    assert!(admitted.no_gradientxy_consumption);
    let rejected = rejected_with(|forbidden| forbidden.gradientxy_consumption = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"gradientxy_consumption"));
}

#[test]
fn r1_no_recursive_r2_reduce_up() {
    let admitted = report();
    assert!(admitted.no_recursive_r2_reduce_up);
    let rejected = rejected_with(|forbidden| forbidden.recursive_r2_reduce_up = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"recursive_r2_reduce_up"));
}

#[test]
fn r1_no_r3_mask_down() {
    let admitted = report();
    assert!(admitted.no_r3_mask_down);
    let rejected = rejected_with(|forbidden| forbidden.r3_mask_down = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"r3_mask_down"));
}

#[test]
fn r1_no_reenroll() {
    let admitted = report();
    assert!(admitted.no_reenroll);
    let rejected = rejected_with(|forbidden| forbidden.reenroll = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"reenroll"));
}

#[test]
fn r1_no_default_simsession_pass_graph_change() {
    let admitted = report();
    assert!(admitted.no_default_simsession_pass_graph_change);
    let rejected = rejected_with(|forbidden| forbidden.default_simsession_pass_graph_wiring = true);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"default_simsession_pass_graph_wiring"));
}

#[test]
fn r1_no_new_shader_or_wgsl() {
    let admitted = report();
    assert!(admitted.no_new_shader_or_wgsl);
    let rejected = rejected_with(|forbidden| forbidden.new_shader_or_gpu_kernel = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"new_shader_or_gpu_kernel"));

    let source = include_str!("../src/dress_rehearsal_r1_disruption_heatmap.rs");
    for forbidden in ["create_shader_module", ".wgsl", "simthing_gpu"] {
        assert!(
            !source.contains(forbidden),
            "R1 CPU fixture must not introduce {forbidden}"
        );
    }
}

#[test]
fn r1_no_f32_bit_exact_claim() {
    let admitted = report();
    assert!(admitted.no_f32_bit_exact_claim);
    assert_eq!(admitted.artifact.gpu_cross_check, "NotRunCpuOraclePrimary");
    assert!(!admitted.artifact.markdown.contains("bit-exact"));
    let rejected = rejected_with(|forbidden| forbidden.f32_bit_exact_claim = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"f32_bit_exact_claim"));
}
