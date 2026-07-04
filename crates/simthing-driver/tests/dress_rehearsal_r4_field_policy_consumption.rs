use simthing_driver::{
    cpu_mag2_sum, cpu_oracle_dress_rehearsal_r4_field_policy_consumption,
    exact_mag2_bits_from_fixed, render_dress_rehearsal_r4_artifact,
    replay_dress_rehearsal_r4_field_policy_consumption,
    run_dress_rehearsal_r4_field_policy_consumption, sqrt_cr_f_bits, DressRehearsalR4Decision,
    DressRehearsalR4Input, DressRehearsalR4Owner, DressRehearsalR4Report,
    DISRUPTION_DECAY_MODIFIER, DRESS_REHEARSAL_R4_FIELD_POLICY_CONSUMPTION_ID,
    DRESS_REHEARSAL_R4_FIELD_POLICY_CONSUMPTION_STATUS_PASS, PIRATE_EMISSION_MODIFIER,
    RAIDING_LOGISTICS_MODIFIER,
};
use simthing_spec::SQRT_F_ARTIFACT_HASH;

fn report() -> DressRehearsalR4Report {
    run_dress_rehearsal_r4_field_policy_consumption(&DressRehearsalR4Input::explicit_opt_in())
}

#[test]
fn r4_deterministic_replay_and_cpu_oracle_parity() {
    let (left, right) = replay_dress_rehearsal_r4_field_policy_consumption();
    assert!(left.admitted && right.admitted);
    assert_eq!(left.summary.stable_checksum, right.summary.stable_checksum);
    assert_eq!(left.mover_rows, right.mover_rows);
    let input = DressRehearsalR4Input::explicit_opt_in();
    let admitted = run_dress_rehearsal_r4_field_policy_consumption(&input);
    let oracle = cpu_oracle_dress_rehearsal_r4_field_policy_consumption(&input);
    assert!(admitted.cpu_oracle_parity);
    assert_eq!(admitted.mover_rows, oracle.mover_rows);
    assert_eq!(admitted.summary, oracle.summary);
}
