use simthing_driver::{
    replay_dress_rehearsal_r5_movement_reenroll, run_dress_rehearsal_r4_field_policy_consumption,
    run_dress_rehearsal_r5_movement_reenroll, DressRehearsalR4Decision, DressRehearsalR4Input,
    DressRehearsalR4Report, DressRehearsalR5Input, DressRehearsalR5Report,
    DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_ID, DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_STATUS_PASS,
    GALACTIC_STRUCTURAL_PARENT,
};
fn report() -> DressRehearsalR5Report {
    run_dress_rehearsal_r5_movement_reenroll(&DressRehearsalR5Input::explicit_opt_in())
}

fn r4_report() -> DressRehearsalR4Report {
    run_dress_rehearsal_r4_field_policy_consumption(&DressRehearsalR4Input::explicit_opt_in())
}

#[test]
fn r5_deterministic_replay_and_cpu_oracle_parity() {
    let admitted = report();
    assert!(admitted.cpu_oracle_parity);
    let (a, b) = replay_dress_rehearsal_r5_movement_reenroll();
    assert_eq!(a.summary.stable_checksum, b.summary.stable_checksum);
    assert_eq!(a.summary.stable_checksum, admitted.summary.stable_checksum);
}
