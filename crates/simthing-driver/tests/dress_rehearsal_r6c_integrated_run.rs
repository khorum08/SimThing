use simthing_driver::{
    cpu_oracle_dress_rehearsal_r6c_integrated_run, replay_dress_rehearsal_r6c_integrated_run,
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cDetectorStatus,
    DressRehearsalR6cInput, DressRehearsalR6cOwner, DressRehearsalR6cReport,
    DRESS_REHEARSAL_R6C_INTEGRATED_RUN_ID, DRESS_REHEARSAL_R6C_INTEGRATED_RUN_STATUS_PASS,
    R6C_CANONICAL_TICK_COUNT, R6C_GPU_POSTURE, R6C_TIE_BREAKER_POLICY,
};
use std::sync::OnceLock;

static REPORT: OnceLock<DressRehearsalR6cReport> = OnceLock::new();

fn report() -> &'static DressRehearsalR6cReport {
    REPORT.get_or_init(|| {
        run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in())
    })
}

#[test]
fn r6c_deterministic_replay_checksum_stable() {
    let admitted = report();
    assert_ne!(admitted.deterministic_replay_checksum, 0);
    let (a, b) = replay_dress_rehearsal_r6c_integrated_run();
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
    assert_eq!(
        a.deterministic_replay_checksum,
        admitted.deterministic_replay_checksum
    );
}

#[test]
fn r6c_cpu_oracle_parity() {
    let admitted = report();
    assert!(admitted.cpu_oracle_parity);
    let oracle =
        cpu_oracle_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    assert_eq!(
        oracle.summary.stable_checksum,
        admitted.summary.stable_checksum
    );
    assert_eq!(
        oracle.final_world,
        admitted.final_world.as_ref().expect("final world").clone()
    );
}
