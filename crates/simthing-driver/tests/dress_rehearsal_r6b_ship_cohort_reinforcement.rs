use simthing_driver::{
    construction_threshold_emission, damage_output_for_cohort, hp_to_retire_for_cohort,
    replay_dress_rehearsal_r6b_ship_cohort_reinforcement,
    run_dress_rehearsal_r6b_ship_cohort_reinforcement, run_r6_combat_with_r6b_cohorts,
    DressRehearsalR6bInput, DressRehearsalR6bOwner, DressRehearsalR6bReport,
    DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_ID,
    DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_STATUS_PASS, FLEET_COHORT_NUM_SHIPS,
    FLEET_DAMAGE_PER_SHIP_PER_TICK, FLEET_HP_PER_SHIP, R6B_FUSION_FIXTURE_CELL, R6B_FUSION_LEFT_ID,
    R6B_FUSION_RIGHT_ID, SHIP_COST,
};

fn report() -> DressRehearsalR6bReport {
    run_dress_rehearsal_r6b_ship_cohort_reinforcement(&DressRehearsalR6bInput::explicit_opt_in())
}

fn entity_id_for_fusion_survivor(report: &DressRehearsalR6bReport, fleet_id: &str) -> u64 {
    report
        .cohort_rows
        .iter()
        .find(|c| c.fleet_id == fleet_id)
        .map(|c| c.entity_id)
        .expect("survivor cohort")
}

#[test]
fn r6b_deterministic_replay_and_cpu_oracle_parity() {
    let admitted = report();
    assert!(admitted.cpu_oracle_parity);
    let (a, b) = replay_dress_rehearsal_r6b_ship_cohort_reinforcement();
    assert_eq!(a.summary.stable_checksum, b.summary.stable_checksum);
    assert_ne!(admitted.summary.stable_checksum, 0);
}
