//! RF-T5: scenario-class burn-in / telemetry soak via the ordinary recursive Arena profile.

use simthing_spec::{ResourceFlowExecutionProfile, ResourceFlowOptInMode};

use crate::resource_flow_opt_in_burn_in::{
    clone_for_replay, open_fixture_session_with_default_profile,
    open_fixture_session_with_execution_profile, run_opt_in_burn_in, RfT2BurnInFixture,
    RfT2BurnInReport, RfT2OptInSession,
};
use crate::resource_flow_opt_in_telemetry::{
    collect_resource_flow_opt_in_telemetry, ResourceFlowFlagSource,
    ResourceFlowOptInTelemetryReport,
};
use crate::session::SessionError;

pub use crate::resource_flow_opt_in_burn_in::{
    fixture_profile_disabled_or_default, fixture_profile_dynamic_fission_cadence,
    fixture_profile_multi_arena_no_coupling, fixture_profile_multi_session_replay,
    fixture_profile_rejection_telemetry, fixture_profile_repeated_resync,
    fixture_profile_static_128_participants, fixture_profile_static_256_participants,
    RF_T5_PROFILE_DISABLED, RF_T5_PROFILE_DYNAMIC_FISSION, RF_T5_PROFILE_MULTI_ARENA,
    RF_T5_PROFILE_MULTI_SESSION, RF_T5_PROFILE_REJECTION, RF_T5_PROFILE_RESYNC,
    RF_T5_PROFILE_STATIC_128, RF_T5_PROFILE_STATIC_256,
};

pub fn open_profile_session(fixture: &RfT2BurnInFixture) -> Result<RfT2OptInSession, SessionError> {
    open_fixture_session_with_execution_profile(
        fixture,
        ResourceFlowExecutionProfile::RecursiveArenaResourceFlow,
    )
}

pub fn profile_telemetry_for_open_session(
    fx: &RfT2OptInSession,
    fixture: &RfT2BurnInFixture,
    burn: Option<&RfT2BurnInReport>,
) -> ResourceFlowOptInTelemetryReport {
    collect_resource_flow_opt_in_telemetry(
        &fx.session,
        fixture.name,
        fixture.opt_in_mode,
        burn,
        Some(&fx.boundary_metrics),
        0,
    )
}

pub fn assert_profile_telemetry_contract(
    telemetry: &ResourceFlowOptInTelemetryReport,
    fixture: &RfT2BurnInFixture,
) {
    assert_eq!(telemetry.scenario_name, fixture.name);
    assert_eq!(telemetry.opt_in_mode, ResourceFlowOptInMode::Disabled);
    assert_eq!(
        telemetry.flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
    assert_eq!(
        telemetry.execution_profile_name,
        "RecursiveArenaResourceFlow"
    );
    assert!(telemetry.resource_flow_enabled);
    if fixture.expected_admissions > 0 || fixture.expected_rejections > 0 {
        assert_eq!(telemetry.dynamic_admissions, fixture.expected_admissions);
        assert_eq!(telemetry.dynamic_rejections, fixture.expected_rejections);
    }
    if fixture.expect_gpu_active {
        assert!(telemetry.total_ops > 0 || fixture.ticks == 0);
        assert!(telemetry.n_bands > 0 || fixture.ticks == 0);
    }
}

pub fn run_profile_soak_with_telemetry(
    fixture: &RfT2BurnInFixture,
) -> Result<(RfT2BurnInReport, ResourceFlowOptInTelemetryReport), SessionError> {
    let mut fx = open_profile_session(fixture)?;
    let burn = run_opt_in_burn_in(&mut fx, fixture)?;
    let telemetry = collect_resource_flow_opt_in_telemetry(
        &fx.session,
        fixture.name,
        fixture.opt_in_mode,
        Some(&burn),
        Some(&fx.boundary_metrics),
        0,
    );
    assert_profile_telemetry_contract(&telemetry, fixture);
    Ok((burn, telemetry))
}

pub fn run_profile_multi_session_replay(
    fixture: &RfT2BurnInFixture,
) -> Result<
    (
        RfT2BurnInReport,
        RfT2BurnInReport,
        ResourceFlowOptInTelemetryReport,
    ),
    SessionError,
> {
    let mut fx_a = open_profile_session(fixture)?;
    let telemetry = profile_telemetry_for_open_session(&fx_a, fixture, None);
    assert_profile_telemetry_contract(&telemetry, fixture);
    let mut fx_b = clone_for_replay(&fx_a, fixture);
    let report_a = run_opt_in_burn_in(&mut fx_a, fixture)?;
    let report_b = run_opt_in_burn_in(&mut fx_b, fixture)?;
    Ok((report_a, report_b, telemetry))
}

pub fn open_default_profile_session(
    fixture: &RfT2BurnInFixture,
) -> Result<RfT2OptInSession, SessionError> {
    open_fixture_session_with_default_profile(fixture)
}
