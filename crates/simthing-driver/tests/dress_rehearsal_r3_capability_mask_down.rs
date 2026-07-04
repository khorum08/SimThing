use simthing_driver::{
    cpu_oracle_dress_rehearsal_r3_capability_mask_down,
    replay_dress_rehearsal_r3_capability_mask_down, run_dress_rehearsal_r3_capability_mask_down,
    DressRehearsalR3Input, DressRehearsalR3Owner, DressRehearsalR3Report, BLOCKADE_DIVERT_MODIFIER,
    COMBAT_BONUS_PLACEHOLDER_MODIFIER, DEFENSIVE_LOGISTICS_MODIFIER, DISRUPTION_DECAY_MODIFIER,
    DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_ID,
    DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_STATUS_PASS, MIN_MODIFIER_BPS,
    PATROL_SUPPRESSION_MODIFIER, PIRATE_EMISSION_MODIFIER, RAIDING_LOGISTICS_MODIFIER,
};

fn report() -> DressRehearsalR3Report {
    run_dress_rehearsal_r3_capability_mask_down(&DressRehearsalR3Input::explicit_opt_in())
}

#[test]
fn r3_deterministic_replay_and_cpu_oracle_parity() {
    let (left, right) = replay_dress_rehearsal_r3_capability_mask_down();
    assert_eq!(left, right);
    assert!(left.cpu_oracle_parity);
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );

    let input = DressRehearsalR3Input::explicit_opt_in();
    let admitted = run_dress_rehearsal_r3_capability_mask_down(&input);
    let oracle = cpu_oracle_dress_rehearsal_r3_capability_mask_down(&input);
    assert_eq!(admitted.modifier_overlay_rows, oracle.modifier_rows);
    assert_eq!(admitted.owner_mask_application_rows, oracle.owner_mask_rows);
    assert_eq!(admitted.modified_r1_signal_rows, oracle.modified_r1_rows);
    assert_eq!(
        admitted.modified_economy_signal_rows,
        oracle.modified_economy_rows
    );
    assert_eq!(admitted.summary, oracle.summary);
}
