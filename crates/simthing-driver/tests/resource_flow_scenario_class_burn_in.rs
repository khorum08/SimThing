//! RF-T5 — scenario-class burn-in / telemetry soak via `FlatStarResourceFlow` profile.

mod support;

use simthing_driver::{
    assert_profile_telemetry_contract, fixture_profile_disabled_or_default,
    fixture_profile_dynamic_fission_cadence, fixture_profile_multi_arena_no_coupling,
    fixture_profile_multi_session_replay, fixture_profile_rejection_telemetry,
    fixture_profile_repeated_resync, fixture_profile_static_128_participants,
    fixture_profile_static_256_participants, fixture_static_flat_star_10_participants,
    open_default_profile_session, open_fixture_session, open_profile_session,
    profile_telemetry_for_open_session, run_profile_multi_session_replay,
    run_profile_soak_with_telemetry, ResourceFlowFlagSource, RF_T5_PROFILE_DISABLED,
};
use simthing_sim::PipelineFlags;
use simthing_spec::{ResourceFlowExecutionProfile, ResourceFlowOptInMode};

use support::e11_burn_in_scenarios::assert_flat_star_only_no_nested_claims;
use support::e11_flat_star::{try_gpu, FlatStarSession};

#[test]
fn rf_t5_profile_static_128_participants_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_static_128_participants();
    let (burn, telemetry) = run_profile_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert!(burn.max_abs_error.is_finite());
    assert_profile_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_t5_profile_static_256_participants_1000_ticks_if_reasonable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_static_256_participants();
    let (burn, telemetry) = run_profile_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert!(burn.max_abs_error.is_finite());
    assert_profile_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_t5_profile_dynamic_fission_cadence_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_dynamic_fission_cadence();
    let (burn, telemetry) = run_profile_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert_eq!(telemetry.dynamic_admissions, 2);
    assert!(burn.replay_bit_exact);
}

#[test]
fn rf_t5_profile_multi_arena_no_coupling_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_multi_arena_no_coupling();
    let fx = open_profile_session(&fixture).expect("open");
    assert_eq!(fx.session.spec_state.arena_registry.arenas.len(), 2);
    let (burn, telemetry) = run_profile_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert_eq!(telemetry.arenas_planned, 2);
    assert!(burn.replay_bit_exact);
}

#[test]
fn rf_t5_profile_multi_session_replay_same_seed() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_multi_session_replay();
    let (report_a, report_b, telemetry) =
        run_profile_multi_session_replay(&fixture).expect("replay");
    assert_eq!(
        report_a.max_abs_error.to_bits(),
        report_b.max_abs_error.to_bits()
    );
    assert_eq!(report_a.ticks_checked, report_b.ticks_checked);
    assert!(report_a.replay_bit_exact);
    assert!(report_b.replay_bit_exact);
    assert_profile_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_t5_profile_repeated_resync_stable() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_repeated_resync();
    let (burn, telemetry) = run_profile_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(burn.sync_cycles_checked, 100);
    assert_eq!(telemetry.sync_count, burn.sync_cycles_checked);
    assert!(burn.replay_bit_exact);
    assert!(burn.total_ops > 0);
    assert!(burn.n_bands > 0);
}
#[test]
fn rf_t5_default_profile_populated_spec_stays_inactive() {
    let fixture = fixture_profile_disabled_or_default();
    let fx = open_default_profile_session(&fixture).expect("open");
    assert!(!fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.state.accumulator_resource_flow_active);
    let telemetry = profile_telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.scenario_name, RF_T5_PROFILE_DISABLED);
    assert!(!telemetry.resource_flow_enabled);
    assert_eq!(
        telemetry.flag_source,
        ResourceFlowFlagSource::DefaultDisabled
    );
    assert_eq!(
        telemetry.execution_profile_name,
        ResourceFlowExecutionProfile::DefaultDisabled.as_str()
    );
    assert_eq!(telemetry.participants_planned, fixture.participant_count);
}

#[test]
fn rf_t5_profile_records_scenario_class_flag_source() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_static_128_participants();
    let fx = open_profile_session(&fixture).expect("open");
    assert_eq!(
        fx.session.resource_flow_flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
    let telemetry = profile_telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(
        telemetry.flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
}

#[test]
fn rf_t5_profile_records_execution_profile_name() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_static_128_participants();
    let fx = open_profile_session(&fixture).expect("open");
    assert_eq!(
        fx.session.resource_flow_execution_profile,
        ResourceFlowExecutionProfile::FlatStarResourceFlow
    );
    let telemetry = profile_telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.execution_profile_name, "FlatStarResourceFlow");
}

#[test]
fn rf_t5_spec_flat_star_opt_in_precedence_still_green() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_static_flat_star_10_participants();
    let fx = open_fixture_session(&fixture).expect("open");
    assert!(fx.session.proto.flags.use_accumulator_resource_flow);
    assert_eq!(
        fx.session.resource_flow_flag_source,
        ResourceFlowFlagSource::SpecFlatStarOptIn
    );
}

#[test]
fn rf_t5_global_resource_flow_flag_default_false() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    assert_eq!(
        simthing_spec::ResourceFlowSpec::default().opt_in_mode,
        ResourceFlowOptInMode::Disabled
    );
    assert_eq!(
        ResourceFlowExecutionProfile::default(),
        ResourceFlowExecutionProfile::DefaultDisabled
    );
}

#[test]
fn rf_t5_does_not_enable_transfer_or_emission() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_static_128_participants();
    let fx = open_profile_session(&fixture).expect("open");
    assert!(fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.proto.flags.use_accumulator_transfer);
    assert!(!fx.session.proto.flags.use_accumulator_emission);
}

#[test]
fn rf_t5_no_simthing_sim_arena_imports() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(!sim_cargo.contains("simthing-driver"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("ArenaParticipant"));
    assert!(!sim_lib.contains("resource_flow_scenario_class_burn_in"));
}

#[test]
fn rf_t5_no_new_wgsl() {
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("resource_flow_scenario_class_burn_in"));
    let sync = include_str!("../../simthing-driver/src/arena_allocation_sync.rs");
    assert!(!sync.contains("wgsl"));
}

#[test]
fn rf_t5_flat_star_only_no_nested_claims() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_profile_static_128_participants();
    let fx = open_profile_session(&fixture).expect("open");
    let flat = FlatStarSession {
        session: fx.session,
        layout: fx.layout,
        cols: fx.cols,
    };
    assert_flat_star_only_no_nested_claims(&flat);
}
