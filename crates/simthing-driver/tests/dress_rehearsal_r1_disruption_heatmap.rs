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
fn r1_no_f32_bit_exact_claim() {
    let admitted = report();
    assert!(admitted.no_f32_bit_exact_claim);
    assert_eq!(admitted.artifact.gpu_cross_check, "NotRunCpuOraclePrimary");
    assert!(!admitted.artifact.markdown.contains("bit-exact"));
    let rejected = rejected_with(|forbidden| forbidden.f32_bit_exact_claim = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"f32_bit_exact_claim"));
}
