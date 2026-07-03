//! E-2B-5R controlled opt-in soak for dynamic fission enrollment (default-off flag path).

mod support;

use simthing_sim::PipelineFlags;

use support::e11_flat_star::try_gpu;
use support::e2b5_dynamic_enrollment_soak::{
    assert_sibling_contiguity_after_admission, clone_enrolled_for_replay,
    dynamic_enrollment_contiguity_blocked_no_compaction, dynamic_enrollment_flag_off_no_gpu_sync,
    dynamic_enrollment_multiple_fissions_same_arena, dynamic_enrollment_reject_when_cap_full,
    dynamic_enrollment_repeated_resync, dynamic_enrollment_single_fission_inherit,
    dynamic_enrollment_two_arenas_inherit, open_fixture_session, open_single_fission_setup,
    run_dynamic_enrollment_soak, run_enrollment_only_soak, run_replay_burn_in,
};

fn run_gpu_soak(fixture: &support::e2b5_dynamic_enrollment_soak::DynamicEnrollmentSoakFixture) {
    let mut fx = open_fixture_session(fixture);
    let report = run_dynamic_enrollment_soak(&mut fx, fixture);
    assert_eq!(report.scenario_name, fixture.name);
    assert_eq!(report.ticks_checked, fixture.ticks);
    assert_eq!(report.admissions_observed, fixture.expected_admissions);
    assert_eq!(report.rejections_observed, fixture.expected_rejections);
}

#[test]
fn e2b5_soak_single_fission_100_ticks_gpu_parity() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = dynamic_enrollment_single_fission_inherit();
    run_gpu_soak(&fixture);
}

#[test]
fn e2b5_soak_single_fission_1000_ticks_gpu_parity() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut fixture = dynamic_enrollment_single_fission_inherit();
    fixture.ticks = 1000;
    run_gpu_soak(&fixture);
}
