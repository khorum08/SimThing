//! Continued flat-star Resource Flow soak checkpoint tests.

mod support;

use simthing_driver::{
    assert_profile_telemetry_contract, continued_static_512_participant_count,
    fixture_continued_dynamic_policy_a, fixture_continued_multi_arena_no_coupling,
    fixture_continued_replay, fixture_continued_static_512_participants,
    fixture_continued_static_skewed_weights, fixture_profile_disabled_or_default,
    open_continued_profile_session, open_default_profile_session,
    profile_telemetry_for_open_session, run_continued_replay_pair, run_continued_soak_with_summary,
    ResourceFlowFlagSource, RF_CONTINUED_STATIC_512,
};
use simthing_sim::PipelineFlags;
use simthing_spec::{ResourceFlowExecutionProfile, ResourceFlowOptInMode};

use support::e11_burn_in_scenarios::assert_flat_star_only_no_nested_claims;
use support::e11_flat_star::{try_gpu, FlatStarSession};

#[test]
fn rf_flat_star_continued_static_512_participants_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_continued_static_512_participants();
    assert_eq!(continued_static_512_participant_count(), 512);
    let (summary, burn, telemetry) =
        run_continued_soak_with_summary(&fixture).expect("continued static 512 soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert_eq!(summary.participant_count, 512);
    assert_eq!(summary.scenario_name, RF_CONTINUED_STATIC_512);
    assert!(summary.max_abs_error.is_finite());
    assert!(summary.total_ops > 0);
    assert!(summary.n_bands > 0);
    assert_profile_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_flat_star_continued_static_skewed_weights_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_continued_static_skewed_weights();
    let (summary, burn, telemetry) =
        run_continued_soak_with_summary(&fixture).expect("continued skewed soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert!(summary.max_abs_error.is_finite());
    assert_profile_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_flat_star_continued_dynamic_policy_a_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_continued_dynamic_policy_a();
    let (summary, burn, telemetry) =
        run_continued_soak_with_summary(&fixture).expect("continued dynamic soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert_eq!(summary.dynamic_admissions, 2);
    assert!(burn.replay_bit_exact);
    assert_profile_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_flat_star_continued_multi_arena_no_coupling_1000_ticks() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_continued_multi_arena_no_coupling();
    let fx = open_continued_profile_session(&fixture).expect("open");
    assert_eq!(fx.session.spec_state.arena_registry.arenas.len(), 2);
    let (summary, burn, telemetry) =
        run_continued_soak_with_summary(&fixture).expect("continued multi-arena soak");
    assert_eq!(burn.ticks_checked, 1000);
    assert_eq!(telemetry.arenas_planned, 2);
    assert!(summary.replay_bit_exact);
    assert_profile_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_flat_star_continued_replay_same_seed_same_summary() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_continued_replay();
    let (summary_a, summary_b, telemetry) =
        run_continued_replay_pair(&fixture).expect("continued replay");
    assert_eq!(summary_a.ticks_checked, summary_b.ticks_checked);
    assert_eq!(
        summary_a.max_abs_error.to_bits(),
        summary_b.max_abs_error.to_bits()
    );
    assert!(summary_a.replay_bit_exact);
    assert!(summary_b.replay_bit_exact);
    assert_profile_telemetry_contract(&telemetry, &fixture);
}

#[test]
fn rf_flat_star_continued_telemetry_has_flag_source_and_profile() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_continued_static_512_participants();
    let fx = open_continued_profile_session(&fixture).expect("open");
    assert_eq!(
        fx.session.resource_flow_flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
    assert_eq!(
        fx.session.resource_flow_execution_profile,
        ResourceFlowExecutionProfile::FlatStarResourceFlow
    );
    let (_, _, telemetry) = run_continued_soak_with_summary(&fixture).expect("telemetry soak");
    assert_eq!(
        telemetry.flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
    assert_eq!(telemetry.execution_profile_name, "FlatStarResourceFlow");
}

#[test]
fn rf_flat_star_continued_global_flag_default_false() {
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
fn rf_flat_star_continued_populated_spec_without_opt_in_inactive() {
    let fixture = fixture_profile_disabled_or_default();
    let fx = open_default_profile_session(&fixture).expect("open");
    assert!(!fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.state.accumulator_resource_flow_active);
    let telemetry = profile_telemetry_for_open_session(&fx, &fixture, None);
    assert!(!telemetry.resource_flow_enabled);
    assert_eq!(
        telemetry.flag_source,
        ResourceFlowFlagSource::DefaultDisabled
    );
}

#[test]
fn rf_flat_star_continued_does_not_enable_transfer_or_emission() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_continued_static_512_participants();
    let fx = open_continued_profile_session(&fixture).expect("open");
    assert!(fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.proto.flags.use_accumulator_transfer);
    assert!(!fx.session.proto.flags.use_accumulator_emission);
}

#[test]
fn rf_flat_star_continued_no_simthing_sim_arena_imports() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(!sim_cargo.contains("simthing-driver"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("ArenaParticipant"));
    assert!(!sim_lib.contains("resource_flow_flat_star_continued_soak"));
}

#[test]
fn rf_flat_star_continued_no_new_wgsl() {
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("resource_flow_flat_star_continued_soak"));
    let sync = include_str!("../../simthing-driver/src/arena_allocation_sync.rs");
    assert!(!sync.contains("wgsl"));
}

#[test]
fn rf_flat_star_continued_flat_star_only_no_nested_claims() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_continued_static_512_participants();
    let fx = open_continued_profile_session(&fixture).expect("open");
    let flat = FlatStarSession {
        session: fx.session,
        layout: fx.layout,
        cols: fx.cols,
    };
    assert_flat_star_only_no_nested_claims(&flat);
}
