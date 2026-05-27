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

#[test]
fn e2b5_soak_multiple_fissions_100_ticks_stable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = dynamic_enrollment_multiple_fissions_same_arena();
    run_gpu_soak(&fixture);
}

#[test]
fn e2b5_soak_two_arenas_dynamic_enrollment_100_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = dynamic_enrollment_two_arenas_inherit();
    run_gpu_soak(&fixture);
}

#[test]
fn e2b5_soak_reject_when_cap_full_no_partial_mutation() {
    let fixture = dynamic_enrollment_reject_when_cap_full();
    let mut setup = open_single_fission_setup(1, 1, 0);
    let report = run_enrollment_only_soak(&mut setup, &fixture);

    assert_eq!(report.admissions_observed, 0);
    assert_eq!(report.rejections_observed, 1);
    assert_eq!(report.generation_start, report.generation_end);
    assert!(
        setup
            .spec_state
            .arena_participant_scaffold
            .index
            .participant_slot(setup.child_ids[0], 0)
            .is_none()
    );
}

#[test]
fn e2b5_soak_contiguity_blocked_no_compaction() {
    let fixture = dynamic_enrollment_contiguity_blocked_no_compaction();
    let mut setup = open_single_fission_setup(1, 16, 2);
    let report = run_enrollment_only_soak(&mut setup, &fixture);

    assert_eq!(report.admissions_observed, 0);
    assert_eq!(report.rejections_observed, 1);
    assert!(
        setup
            .spec_state
            .arena_participant_scaffold
            .index
            .participant_slot(setup.child_ids[0], 0)
            .is_none()
    );
}

#[test]
fn e2b5_soak_flag_off_updates_registry_but_no_gpu_sync() {
    let fixture = dynamic_enrollment_flag_off_no_gpu_sync();
    let mut fx = open_fixture_session(&fixture);
    let report = run_dynamic_enrollment_soak(&mut fx, &fixture);

    assert_eq!(report.admissions_observed, 1);
    assert_eq!(report.resource_flow_syncs_observed, 0);
    assert!(!fx.session.state.accumulator_resource_flow_active);
    assert!(
        fx.session
            .spec_state
            .arena_participant_scaffold
            .index
            .participant_slot(fx.enrollment_report.admissions[0].child_id, 0)
            .is_some()
    );
    assert_sibling_contiguity_after_admission(&fx);
}

#[test]
fn e2b5_soak_replay_same_seed_same_dynamic_enrollment_frames() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = dynamic_enrollment_single_fission_inherit();
    let mut fx_a = open_fixture_session(&fixture);
    let mut fx_b = clone_enrolled_for_replay(&fx_a);

    let report_a = run_replay_burn_in(&mut fx_a, 10);
    let report_b = run_replay_burn_in(&mut fx_b, 10);

    assert_eq!(report_a.max_abs_error.to_bits(), report_b.max_abs_error.to_bits());
    assert_eq!(report_a.ticks_checked, report_b.ticks_checked);
    assert!(report_a.replay_bit_exact);
    assert!(report_b.replay_bit_exact);
}

#[test]
fn e2b5_soak_repeated_resync_after_dynamic_admissions_stable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let fixture = dynamic_enrollment_repeated_resync();
    let mut fx = open_fixture_session(&fixture);
    let report = run_dynamic_enrollment_soak(&mut fx, &fixture);

    assert!(report.resource_flow_syncs_observed >= 100);
    assert_eq!(report.ticks_checked, 10);
    assert!(report.replay_bit_exact);
}

#[test]
fn e2b5_soak_resource_flow_flag_default_false() {
    assert!(
        !PipelineFlags::default().use_accumulator_resource_flow,
        "controlled soak must not flip default-on"
    );
}

#[test]
fn e2b5_soak_no_simthing_sim_arena_imports() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(!sim_cargo.contains("simthing-driver"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("ArenaParticipant"));
    assert!(!sim_lib.contains("resource_flow_fission"));
}

#[test]
fn e2b5_soak_no_new_wgsl() {
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("resource_flow_fission"));
    let soak = include_str!("../../simthing-driver/src/resource_flow_dynamic_enrollment_soak.rs");
    assert!(!soak.contains("wgsl"));
}
