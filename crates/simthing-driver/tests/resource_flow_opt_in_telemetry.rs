//! RF-T3 — Resource Flow opt-in telemetry and flag-source attribution.

mod support;

use simthing_driver::{
    fixture_product_disabled_spec_diagnostics, RF_T3_PRODUCT_DISABLED,
    fixture_product_dynamic_fission_cadence, fixture_product_rejection_telemetry,
    fixture_static_flat_star_10_participants, open_fixture_session, open_product_session,
    run_opt_in_burn_in, run_product_soak_with_telemetry, telemetry_for_open_session,
    ResourceFlowFlagSource,
};
use simthing_spec::ResourceFlowOptInMode;

use support::e11_flat_star::try_gpu;

#[test]
fn rf_t3_telemetry_disabled_spec_reports_default_disabled() {
    let fixture = fixture_product_disabled_spec_diagnostics();
    let fx = open_product_session(&fixture).expect("open");
    let telemetry = telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.scenario_name, RF_T3_PRODUCT_DISABLED);
    assert_eq!(telemetry.opt_in_mode, ResourceFlowOptInMode::Disabled);
    assert_eq!(telemetry.flag_source, ResourceFlowFlagSource::DefaultDisabled);
    assert!(!telemetry.resource_flow_enabled);
    assert_eq!(telemetry.participants_planned, fixture.participant_count);
    assert_eq!(telemetry.total_ops, 0);
}

#[test]
fn rf_t3_telemetry_flat_star_opt_in_reports_spec_source() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_static_flat_star_10_participants();
    let fx = open_fixture_session(&fixture).expect("open");
    let telemetry = telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.opt_in_mode, ResourceFlowOptInMode::FlatStarOptIn);
    assert_eq!(telemetry.flag_source, ResourceFlowFlagSource::SpecFlatStarOptIn);
    assert!(telemetry.resource_flow_enabled);
}

#[test]
fn rf_t3_telemetry_flat_star_reports_ops_bands_and_participants() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_static_flat_star_10_participants();
    let (_, telemetry) = run_product_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(telemetry.participants_planned, 10);
    assert!(telemetry.total_ops > 0);
    assert!(telemetry.n_bands > 0);
    assert_eq!(telemetry.sync_count, 1);
}

#[test]
fn rf_t3_telemetry_dynamic_enrollment_reports_admissions_generation_and_sync() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let fixture = fixture_product_dynamic_fission_cadence();
    let (burn, telemetry) = run_product_soak_with_telemetry(&fixture).expect("soak");
    assert_eq!(telemetry.dynamic_admissions, 2);
    assert_eq!(telemetry.dynamic_rejections, 0);
    assert!(telemetry.generation_end > telemetry.generation_start);
    assert!(telemetry.sync_count >= 1);
    assert_eq!(burn.ticks_checked, 1000);
}

#[test]
fn rf_t3_telemetry_rejections_visible() {
    let fixture = fixture_product_rejection_telemetry();
    let fx = open_fixture_session(&fixture).expect("open");
    let telemetry = telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.dynamic_rejections, 1);
    assert_eq!(telemetry.dynamic_admissions, 0);
    assert_eq!(telemetry.generation_start, telemetry.generation_end);
}

#[test]
fn rf_t3_telemetry_test_override_distinguishable_from_spec() {
    let fixture = fixture_product_disabled_spec_diagnostics();
    let mut fx = open_fixture_session(&fixture).expect("open");
    fx.session.override_resource_flow_flag_for_tests(true);
    let telemetry = telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.flag_source, ResourceFlowFlagSource::TestOverride);
    assert!(telemetry.resource_flow_enabled);
    assert_eq!(telemetry.opt_in_mode, ResourceFlowOptInMode::Disabled);
}
