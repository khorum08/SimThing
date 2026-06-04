//! RF-T3 — product-like FlatStarOptIn soak scenarios.

mod support;

use simthing_driver::{
    assert_telemetry_contract, fixture_product_disabled_spec_diagnostics,
    fixture_product_dynamic_fission_cadence, fixture_product_multi_arena_no_coupling,
    fixture_product_multi_session_replay, fixture_product_rejection_telemetry,
    fixture_product_repeated_resync, fixture_product_static_128_participants,
    fixture_product_static_256_participants, open_product_session, run_multi_session_replay,
    run_product_soak_with_telemetry, RF_T3_PRODUCT_DISABLED,
};
use simthing_sim::PipelineFlags;
use simthing_spec::ResourceFlowOptInMode;

use support::e11_burn_in_scenarios::assert_flat_star_only_no_nested_claims;
use support::e11_flat_star::{try_gpu, FlatStarSession};

#[test]
fn rf_t3_product_static_128_participants_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_static_128_participants();
    let (burn, telemetry) = run_product_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert!(burn.max_abs_error.is_finite());
    assert_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_t3_product_static_256_participants_if_runtime_reasonable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_static_256_participants();
    let (burn, telemetry) = run_product_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert!(burn.max_abs_error.is_finite());
    assert_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_t3_product_dynamic_fission_cadence_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_dynamic_fission_cadence();
    let (burn, telemetry) = run_product_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert_eq!(telemetry.dynamic_admissions, 2);
    assert!(burn.replay_bit_exact);
}

#[test]
fn rf_t3_product_multi_arena_no_coupling_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_multi_arena_no_coupling();
    let fx = open_product_session(&fixture).expect("open");
    assert_eq!(fx.session.spec_state.arena_registry.arenas.len(), 2);
    let (burn, telemetry) = run_product_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert_eq!(telemetry.arenas_planned, 2);
    assert!(burn.replay_bit_exact);
}

#[test]
fn rf_t3_product_multi_session_replay_same_seed() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_multi_session_replay();
    let (report_a, report_b) = run_multi_session_replay(&fixture).expect("replay");
    assert_eq!(
        report_a.max_abs_error.to_bits(),
        report_b.max_abs_error.to_bits()
    );
    assert_eq!(report_a.ticks_checked, report_b.ticks_checked);
    assert!(report_a.replay_bit_exact);
    assert!(report_b.replay_bit_exact);
}

#[test]
fn rf_t3_product_repeated_resync_stable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_repeated_resync();
    let (burn, telemetry) = run_product_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.sync_cycles_checked, 100);
    assert_eq!(telemetry.sync_count, burn.sync_cycles_checked);
    assert!(burn.replay_bit_exact);
    let ops_after = burn.total_ops;
    let bands_after = burn.n_bands;
    assert!(ops_after > 0);
    assert!(bands_after > 0);
}

#[test]
fn rf_t3_disabled_populated_spec_inactive_but_reported() {
    let fixture = fixture_product_disabled_spec_diagnostics();
    let fx = open_product_session(&fixture).expect("open");
    assert!(!fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.state.accumulator_resource_flow_active);
    let telemetry = simthing_driver::telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.scenario_name, RF_T3_PRODUCT_DISABLED);
    assert!(!telemetry.resource_flow_enabled);
    assert_eq!(telemetry.participants_planned, fixture.participant_count);
}

#[test]
fn rf_t3_global_resource_flow_flag_default_false() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    assert_eq!(
        simthing_spec::ResourceFlowSpec::default().opt_in_mode,
        ResourceFlowOptInMode::Disabled
    );
}

#[test]
fn rf_t3_does_not_enable_transfer_or_emission() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_static_128_participants();
    let fx = open_product_session(&fixture).expect("open");
    assert!(fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.proto.flags.use_accumulator_transfer);
    assert!(!fx.session.proto.flags.use_accumulator_emission);
}

#[test]
fn rf_t3_product_rejection_telemetry_fixture() {
    let fixture = fixture_product_rejection_telemetry();
    let fx = open_product_session(&fixture).expect("open");
    let telemetry = simthing_driver::telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.dynamic_rejections, 1);
    assert_eq!(telemetry.dynamic_admissions, 0);
}

#[test]
fn rf_t3_no_simthing_sim_arena_imports() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(!sim_cargo.contains("simthing-driver"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("ArenaParticipant"));
    assert!(!sim_lib.contains("resource_flow_opt_in_product_soak"));
}

#[test]
fn rf_t3_no_new_wgsl() {
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("resource_flow_opt_in_product_soak"));
    let sync = include_str!("../../simthing-driver/src/arena_allocation_sync.rs");
    assert!(!sync.contains("wgsl"));
}

#[test]
fn rf_t3_flat_star_only_no_nested_claims() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_static_128_participants();
    let fx = open_product_session(&fixture).expect("open");
    let flat = FlatStarSession {
        session: fx.session,
        layout: fx.layout,
        cols: fx.cols,
    };
    assert_flat_star_only_no_nested_claims(&flat);
}
