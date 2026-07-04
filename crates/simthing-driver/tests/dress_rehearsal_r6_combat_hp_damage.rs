use simthing_driver::{
    damage_output_for_cohort, emission_band_ship_attrition, hp_to_retire_for_cohort,
    replay_dress_rehearsal_r6_combat_hp_damage, run_dress_rehearsal_r6_combat_hp_damage,
    DressRehearsalR6Input, DressRehearsalR6Owner, DressRehearsalR6Report,
    DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_ID, DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_STATUS_PASS,
    FLEET_COHORT_NUM_SHIPS, FLEET_DAMAGE_PER_SHIP_PER_TICK, FLEET_HP_PER_SHIP,
    GALACTIC_STRUCTURAL_PARENT,
};

fn report() -> DressRehearsalR6Report {
    run_dress_rehearsal_r6_combat_hp_damage(&DressRehearsalR6Input::explicit_opt_in())
}

#[test]
fn r6_deterministic_replay_and_cpu_oracle_parity() {
    let admitted = report();
    assert!(admitted.cpu_oracle_parity);
    let (a, b) = replay_dress_rehearsal_r6_combat_hp_damage();
    assert_eq!(a.summary.stable_checksum, b.summary.stable_checksum);
    assert_eq!(a.summary.stable_checksum, admitted.summary.stable_checksum);
}

fn identity_lane_for_owner(owner: DressRehearsalR6Owner) -> u32 {
    match owner {
        DressRehearsalR6Owner::Terran => 0,
        DressRehearsalR6Owner::Pirate => 1,
    }
}
