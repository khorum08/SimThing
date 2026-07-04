use std::sync::OnceLock;

use simthing_driver::{
    build_recursive_world, replay_runtime_0080_rr_2, run_runtime_0080_rr_2, Runtime0080Rr0Owner,
    Runtime0080Rr2Input, Runtime0080Rr2Report, RR_2_ACTIVE_SURFACE_COUNT, RUNTIME_0080_RR_2_ID,
    RUNTIME_0080_RR_2_STATUS_PASS, RUNTIME_RR_2_EXPECTED_REPORT_CHECKSUM,
};

static REPORT: OnceLock<Runtime0080Rr2Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr2Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_2(&Runtime0080Rr2Input::explicit_opt_in()))
}

#[test]
fn rr_2_production_bits_match_rr_0_oracle() {
    let admitted = report();
    assert!(admitted.production_parity_ok);
    assert!(admitted
        .parity_rows
        .iter()
        .all(|row| row.cpu_production_bits == row.gpu_production_bits));
}

#[test]
fn rr_2_disabled_labor_emitter_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_emitter_fails_parity);
}

#[test]
fn rr_2_reenabled_labor_emitter_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_emitter_restores_parity);
}

#[test]
fn rr_2_disabled_factory_consumer_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_consumer_fails_parity);
}

#[test]
fn rr_2_reenabled_factory_consumer_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_consumer_restores_parity);
}

#[test]
fn rr_2_replay_deterministic() {
    let (left, right) = replay_runtime_0080_rr_2();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_eq!(left.id, RUNTIME_0080_RR_2_ID);
    assert!(left.parity_rows.len() >= 2);
    assert_eq!(left.parity_rows.len(), RR_2_ACTIVE_SURFACE_COUNT);
}
