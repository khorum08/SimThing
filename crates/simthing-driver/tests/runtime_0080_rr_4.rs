use std::sync::OnceLock;

use simthing_driver::{
    dress_rehearsal_r6c_integrated_run::R6C_CANONICAL_TICK_COUNT, replay_runtime_0080_rr_4,
    run_runtime_0080_rr_4, Runtime0080Rr4Input, Runtime0080Rr4Report, RUNTIME_0080_RR_4_ID,
    RUNTIME_0080_RR_4_STATUS_PASS, RUNTIME_RR_4_EXPECTED_REPORT_CHECKSUM,
};

static REPORT: OnceLock<Runtime0080Rr4Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr4Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_4(&Runtime0080Rr4Input::explicit_opt_in()))
}

#[test]
fn rr_4_replay_deterministic() {
    let (left, right) = replay_runtime_0080_rr_4();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_eq!(left.id, RUNTIME_0080_RR_4_ID);
    assert_eq!(left.tick_parity_rows.len(), 200);
}
