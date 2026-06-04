//! RF-T4 — limited scenario-class default-on for Resource Flow.

mod support;

use simthing_driver::{
    clone_for_replay, collect_resource_flow_opt_in_telemetry, fixture_disabled_populated_spec,
    fixture_dynamic_single_fission, fixture_product_rejection_telemetry,
    fixture_static_flat_star_10_participants, fixture_wildcard_rejected, open_fixture_session,
    open_fixture_session_with_execution_profile, run_opt_in_burn_in, ResourceFlowFlagSource,
};
use simthing_sim::PipelineFlags;
use simthing_spec::{ResourceFlowExecutionProfile, ResourceFlowOptInMode};

use support::e11_burn_in_scenarios::assert_flat_star_only_no_nested_claims;
use support::e11_flat_star::{
    fill_explicit_participants, flat_star_game_mode, flat_star_scenario, try_gpu, FlatStarSession,
};

fn open_scenario_class(
    fixture: &simthing_driver::RfT2BurnInFixture,
) -> simthing_driver::RfT2OptInSession {
    open_fixture_session_with_execution_profile(
        fixture,
        ResourceFlowExecutionProfile::FlatStarResourceFlow,
    )
    .expect("open scenario class session")
}

fn scenario_class_static_fixture() -> simthing_driver::RfT2BurnInFixture {
    let mut f = fixture_static_flat_star_10_participants();
    f.name = "rf_t4_scenario_class_static_flat_star";
    f.opt_in_mode = ResourceFlowOptInMode::Disabled;
    f.ticks = 100;
    f
}

#[test]
fn rf_t4_global_pipeline_flag_default_false() {
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
fn rf_t4_scenario_class_enables_resource_flow_flag() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fx = open_scenario_class(&scenario_class_static_fixture());
    assert!(fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(fx.session.state.accumulator_resource_flow_active);
}

#[test]
fn rf_t4_scenario_class_records_flag_source() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fx = open_scenario_class(&scenario_class_static_fixture());
    assert_eq!(
        fx.session.resource_flow_flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
}

#[test]
fn rf_t4_scenario_class_records_profile_name() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fx = open_scenario_class(&scenario_class_static_fixture());
    assert_eq!(
        fx.session.resource_flow_execution_profile,
        ResourceFlowExecutionProfile::FlatStarResourceFlow
    );
    let telemetry = collect_resource_flow_opt_in_telemetry(
        &fx.session,
        fx.session.scenario.name.clone(),
        ResourceFlowOptInMode::Disabled,
        None,
        Some(&fx.boundary_metrics),
        0,
    );
    assert_eq!(telemetry.execution_profile_name, "FlatStarResourceFlow");
    assert_eq!(
        telemetry.flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
}

#[test]
fn rf_t4_populated_spec_without_class_or_opt_in_stays_inactive() {
    let fixture = fixture_disabled_populated_spec();
    let fx = open_fixture_session(&fixture).expect("open");
    assert!(!fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.state.accumulator_resource_flow_active);
    assert_eq!(
        fx.session.resource_flow_flag_source,
        ResourceFlowFlagSource::DefaultDisabled
    );
}

#[test]
fn rf_t4_spec_flat_star_opt_in_still_works() {
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
fn rf_t4_scenario_class_with_spec_disabled_enables_when_profile_declares_execution() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = scenario_class_static_fixture();
    assert_eq!(fixture.opt_in_mode, ResourceFlowOptInMode::Disabled);
    let fx = open_scenario_class(&fixture);
    assert_eq!(
        fx.session.resource_flow_flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
    assert!(fx.session.proto.flags.use_accumulator_resource_flow);
    assert_eq!(
        fx.session.spec_state.arena_registry.participants.len(),
        fixture.participant_count as usize
    );
}

#[test]
fn rf_t4_scenario_class_rejects_wildcard_or_nested_claim() {
    let mut fixture = fixture_wildcard_rejected();
    fixture.opt_in_mode = ResourceFlowOptInMode::Disabled;
    let err = match open_fixture_session_with_execution_profile(
        &fixture,
        ResourceFlowExecutionProfile::FlatStarResourceFlow,
    ) {
        Err(e) => e,
        Ok(_) => panic!("wildcard scenario class must be rejected at session open"),
    };
    assert!(
        err.to_string().contains("wildcard"),
        "expected wildcard rejection, got {err}"
    );
}

#[test]
fn rf_t4_scenario_class_dynamic_enrollment_resyncs_after_fission() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut fixture = fixture_dynamic_single_fission();
    fixture.opt_in_mode = ResourceFlowOptInMode::Disabled;
    fixture.ticks = 100;
    let mut fx = open_scenario_class(&fixture);
    let report = run_opt_in_burn_in(&mut fx, &fixture).expect("burn-in");
    assert_eq!(report.admissions_observed, 1);
    assert!(report.generation_end > report.generation_start);
    assert!(report.total_ops > 0);
}

#[test]
fn rf_t4_scenario_class_replay_same_seed_same_telemetry() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = scenario_class_static_fixture();
    let mut fx_a = open_scenario_class(&fixture);
    let mut fx_b = clone_for_replay(&fx_a, &fixture);
    let report_a = run_opt_in_burn_in(&mut fx_a, &fixture).expect("burn a");
    let report_b = run_opt_in_burn_in(&mut fx_b, &fixture).expect("burn b");
    let tel_a = collect_resource_flow_opt_in_telemetry(
        &fx_a.session,
        fixture.name,
        fixture.opt_in_mode,
        Some(&report_a),
        Some(&fx_a.boundary_metrics),
        0,
    );
    let tel_b = collect_resource_flow_opt_in_telemetry(
        &fx_b.session,
        fixture.name,
        fixture.opt_in_mode,
        Some(&report_b),
        Some(&fx_b.boundary_metrics),
        0,
    );
    assert_eq!(
        tel_a.flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
    assert_eq!(
        tel_b.flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
    assert_eq!(tel_a.max_abs_error.to_bits(), tel_b.max_abs_error.to_bits());
    assert_eq!(report_a.ticks_checked, report_b.ticks_checked);
}

#[test]
fn rf_t4_scenario_class_rejection_telemetry_visible() {
    let mut fixture = fixture_product_rejection_telemetry();
    fixture.opt_in_mode = ResourceFlowOptInMode::Disabled;
    let fx = open_scenario_class(&fixture);
    let telemetry = collect_resource_flow_opt_in_telemetry(
        &fx.session,
        fixture.name,
        fixture.opt_in_mode,
        None,
        Some(&fx.boundary_metrics),
        0,
    );
    assert_eq!(telemetry.dynamic_rejections, 1);
    assert_eq!(telemetry.dynamic_admissions, 0);
    assert_eq!(
        telemetry.flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
}

#[test]
fn rf_t4_does_not_enable_transfer_or_emission() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fx = open_scenario_class(&scenario_class_static_fixture());
    assert!(fx.session.proto.flags.use_accumulator_resource_flow);
    assert!(!fx.session.proto.flags.use_accumulator_transfer);
    assert!(!fx.session.proto.flags.use_accumulator_emission);
}

#[test]
fn rf_t4_no_simthing_sim_arena_imports() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(!sim_cargo.contains("simthing-driver"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("ArenaParticipant"));
    assert!(!sim_lib.contains("ResourceFlowExecutionProfile"));
}

#[test]
fn rf_t4_no_new_wgsl() {
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("ResourceFlowExecutionProfile"));
    let sync = include_str!("../../simthing-driver/src/arena_allocation_sync.rs");
    assert!(!sync.contains("wgsl"));
}

#[test]
fn rf_t4_scenario_class_flat_star_guard() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fx = open_scenario_class(&scenario_class_static_fixture());
    let flat = FlatStarSession {
        session: fx.session,
        layout: fx.layout,
        cols: fx.cols,
    };
    assert_flat_star_only_no_nested_claims(&flat);
}

#[test]
fn rf_t4_spec_opt_in_precedence_over_scenario_class() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = flat_star_game_mode(16);
    game_mode.resource_flow.as_mut().unwrap().opt_in_mode = ResourceFlowOptInMode::FlatStarOptIn;
    game_mode.resource_flow_execution_profile = ResourceFlowExecutionProfile::FlatStarResourceFlow;
    fill_explicit_participants(&mut game_mode, &scenario);
    let session = simthing_driver::SimSession::open_from_spec(scenario, &game_mode).expect("open");
    assert_eq!(
        session.resource_flow_flag_source,
        ResourceFlowFlagSource::SpecFlatStarOptIn
    );
}
