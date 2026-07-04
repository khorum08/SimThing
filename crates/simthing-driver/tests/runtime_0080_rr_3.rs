use std::sync::OnceLock;

use simthing_driver::{
    build_recursive_world, replay_runtime_0080_rr_3, run_runtime_0080_rr_3, Runtime0080Rr3Input,
    Runtime0080Rr3Report, Runtime0080Rr3TierTransition, RR_3_SLOTS_PER_SYSTEM,
    RUNTIME_0080_RR_3_ID, RUNTIME_0080_RR_3_STATUS_PASS, RUNTIME_RR_3_EXPECTED_REPORT_CHECKSUM,
};

static REPORT: OnceLock<Runtime0080Rr3Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr3Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_3(&Runtime0080Rr3Input::explicit_opt_in()))
}

#[test]
fn rr_3_disabled_surface_to_planet_reduce_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_surface_to_planet_fails_parity);
}

#[test]
fn rr_3_reenabled_surface_to_planet_reduce_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_surface_to_planet_restores_parity);
}

#[test]
fn rr_3_disabled_galaxy_to_faction_reduce_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_galaxy_to_stockpile_fails_parity);
}

#[test]
fn rr_3_reenabled_galaxy_to_faction_reduce_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_galaxy_to_stockpile_restores_parity);
}

#[test]
fn rr_3_disabled_disburse_down_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_disburse_down_fails_parity);
}

#[test]
fn rr_3_reenabled_disburse_down_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_disburse_down_restores_parity);
}

#[test]
fn rr_3_replay_deterministic() {
    let (left, right) = replay_runtime_0080_rr_3();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_eq!(left.id, RUNTIME_0080_RR_3_ID);
    assert!(!left.reduce_up_rows.is_empty());
    assert!(!left.disburse_down_rows.is_empty());
}
