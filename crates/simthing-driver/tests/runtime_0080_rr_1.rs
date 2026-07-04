use std::sync::OnceLock;

use simthing_driver::{
    build_recursive_world, canonical_access_pattern, replay_runtime_0080_rr_1,
    run_runtime_0080_rr_1, try_access_surface_for_system, try_access_system_at_galaxy_cell,
    Runtime0080Rr0Owner, Runtime0080Rr1Input, Runtime0080Rr1Report, Runtime0080Rr1ResidencyRequest,
    RR_1_GALAXY_CELL_COUNT, RR_1_SURFACE_CELL_COUNT, RR_1_SYSTEM_CELL_COUNT, RR_1_SYSTEM_COUNT,
    RUNTIME_0080_RR_1_ID, RUNTIME_0080_RR_1_STATUS_PASS, RUNTIME_RR_1_EXPECTED_REPORT_CHECKSUM,
};

static REPORT: OnceLock<Runtime0080Rr1Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr1Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_1(&Runtime0080Rr1Input::explicit_opt_in()))
}

#[test]
fn rr_1_mapping_parity_matches_rr_0() {
    let admitted = report();
    assert!(admitted.mapping_parity_ok);
    assert_eq!(admitted.mapping_parity_rows.len(), RR_1_SYSTEM_COUNT);
    for row in &admitted.mapping_parity_rows {
        assert!(row.owner_matches_rr_0);
        assert!(row.parent_galaxy_matches_rr_0);
        assert!(row.system_dims_match_rr_0);
        assert!(row.surface_dims_match_rr_0);
        assert!(row.pop_placement_matches_rr_0);
        assert!(row.factory_placement_matches_rr_0);
    }
}

#[test]
fn rr_1_replay_deterministic() {
    let (left, right) = replay_runtime_0080_rr_1();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_ne!(left.deterministic_replay_checksum, 0);
}
