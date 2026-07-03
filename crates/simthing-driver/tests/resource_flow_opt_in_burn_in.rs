//! RF-T2 — limited opt-in scenario burn-in expansion for Resource Flow.

mod support;

use simthing_driver::{
    clone_for_replay, fixture_disabled_populated_spec, fixture_dynamic_multi_fission,
    fixture_dynamic_single_fission, fixture_repeated_resync, fixture_replay_static,
    fixture_static_flat_star_10_participants, fixture_static_flat_star_64_participants,
    fixture_static_flat_star_skewed_weights, fixture_two_arena_no_coupling,
    fixture_wildcard_rejected, open_fixture_session, run_opt_in_burn_in, RfT2BurnInFixture,
};
use simthing_sim::PipelineFlags;
use simthing_spec::ResourceFlowOptInMode;

use support::e11_burn_in_scenarios::assert_flat_star_only_no_nested_claims;
use support::e11_flat_star::{try_gpu, FlatStarSession};

fn run_burn_in(fixture: &RfT2BurnInFixture) -> simthing_driver::RfT2BurnInReport {
    let mut fx = open_fixture_session(fixture).expect("open opt-in session");
    run_opt_in_burn_in(&mut fx, fixture).expect("burn-in")
}

#[test]
fn rf_t2_static_flat_star_10_participants_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_static_flat_star_10_participants();
    let report = run_burn_in(&fixture);
    assert_eq!(report.ticks_checked, 1000);
    assert!(report.replay_bit_exact);
    assert!(report.total_ops > 0);
}

#[test]
fn rf_t2_static_flat_star_64_participants_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_static_flat_star_64_participants();
    let report = run_burn_in(&fixture);
    assert_eq!(report.ticks_checked, 1000);
    assert!(report.max_abs_error.is_finite());
    assert!(report.total_ops > 0);
}

#[test]
fn rf_t2_static_flat_star_skewed_weights_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_static_flat_star_skewed_weights();
    let report = run_burn_in(&fixture);
    assert_eq!(report.ticks_checked, 1000);
    assert!(report.replay_bit_exact);
}

#[test]
fn rf_t2_dynamic_single_fission_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_dynamic_single_fission();
    let report = run_burn_in(&fixture);
    assert_eq!(report.admissions_observed, 1);
    assert_eq!(report.ticks_checked, 1000);
    assert!(report.replay_bit_exact);
}

#[test]
fn rf_t2_dynamic_multi_fission_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_dynamic_multi_fission();
    let report = run_burn_in(&fixture);
    assert_eq!(report.admissions_observed, 2);
    assert_eq!(report.ticks_checked, 1000);
    assert!(report.replay_bit_exact);
}

#[test]
fn rf_t2_two_arena_flat_star_no_coupling_100_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_two_arena_no_coupling();
    let mut fx = open_fixture_session(&fixture).expect("open");
    assert_eq!(fx.session.spec_state.arena_registry.arenas.len(), 2);
    let report = run_opt_in_burn_in(&mut fx, &fixture).expect("burn-in");
    assert_eq!(report.ticks_checked, 100);
    assert!(report.replay_bit_exact);
}

#[test]
fn rf_t2_disabled_populated_spec_stays_inactive() {
    let fixture = fixture_disabled_populated_spec();
    let fx = open_fixture_session(&fixture).expect("open");
    assert!(!fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.state.accumulator_resource_flow_active);
    assert_eq!(
        fx.session.spec_state.arena_registry.participants.len(),
        fixture.participant_count as usize
    );
}
#[test]
fn rf_t2_repeated_resync_stable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_repeated_resync();
    let report = run_burn_in(&fixture);
    assert_eq!(report.sync_cycles_checked, 100);
    assert!(report.replay_bit_exact);
}

#[test]
fn rf_t2_replay_same_seed_same_frames() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_replay_static();
    let mut fx_a = open_fixture_session(&fixture).expect("open a");
    let mut fx_b = clone_for_replay(&fx_a, &fixture);
    let report_a = run_opt_in_burn_in(&mut fx_a, &fixture).expect("burn a");
    let report_b = run_opt_in_burn_in(&mut fx_b, &fixture).expect("burn b");
    assert_eq!(
        report_a.max_abs_error.to_bits(),
        report_b.max_abs_error.to_bits()
    );
    assert_eq!(report_a.ticks_checked, report_b.ticks_checked);
    assert!(report_a.replay_bit_exact);
    assert!(report_b.replay_bit_exact);
}

#[test]
fn rf_t2_resource_flow_flag_default_false() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    assert_eq!(
        simthing_spec::ResourceFlowSpec::default().opt_in_mode,
        ResourceFlowOptInMode::Disabled
    );
}

#[test]
fn rf_t2_does_not_enable_transfer_or_emission() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_static_flat_star_10_participants();
    let fx = open_fixture_session(&fixture).expect("open");
    assert!(fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.proto.flags.use_accumulator_transfer);
    assert!(!fx.session.proto.flags.use_accumulator_emission);
}

#[test]
fn rf_t2_no_simthing_sim_arena_imports() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(!sim_cargo.contains("simthing-driver"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("ArenaParticipant"));
    assert!(!sim_lib.contains("resource_flow_opt_in_burn_in"));
}

#[test]
fn rf_t2_no_new_wgsl() {
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("resource_flow_opt_in_burn_in"));
    let sync = include_str!("../../simthing-driver/src/arena_allocation_sync.rs");
    assert!(!sync.contains("wgsl"));
}

#[test]
fn rf_t2_flat_star_only_no_nested_claims() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_static_flat_star_10_participants();
    let fx = open_fixture_session(&fixture).expect("open");
    let flat = FlatStarSession {
        session: fx.session,
        layout: fx.layout,
        cols: fx.cols,
    };
    assert_flat_star_only_no_nested_claims(&flat);
}
