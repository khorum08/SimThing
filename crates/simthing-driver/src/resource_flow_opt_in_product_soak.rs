//! RF-T3 — product-like FlatStarOptIn soak scenarios and telemetry surfacing.

use simthing_spec::ResourceFlowOptInMode;

use crate::resource_flow_opt_in_burn_in::{
    clone_for_replay, open_fixture_session, run_opt_in_burn_in, RfT2BurnInFixture,
    RfT2BurnInReport, RfT2OptInSession,
};
use crate::resource_flow_opt_in_telemetry::{
    collect_resource_flow_opt_in_telemetry, ResourceFlowOptInTelemetryReport,
};
use crate::session::SessionError;

pub use crate::resource_flow_opt_in_burn_in::{
    fixture_product_disabled_spec_diagnostics, fixture_product_dynamic_fission_cadence,
    fixture_product_multi_arena_no_coupling, fixture_product_multi_session_replay,
    fixture_product_rejection_telemetry, fixture_product_repeated_resync,
    fixture_product_static_128_participants, fixture_product_static_256_participants,
    RF_T3_PRODUCT_DISABLED, RF_T3_PRODUCT_DYNAMIC_FISSION, RF_T3_PRODUCT_MULTI_ARENA,
    RF_T3_PRODUCT_MULTI_SESSION, RF_T3_PRODUCT_REJECTION, RF_T3_PRODUCT_RESYNC,
    RF_T3_PRODUCT_STATIC_128, RF_T3_PRODUCT_STATIC_256,
};

pub fn run_product_soak_with_telemetry(
    fixture: &RfT2BurnInFixture,
) -> Result<(RfT2BurnInReport, ResourceFlowOptInTelemetryReport), SessionError> {
    let mut fx = open_fixture_session(fixture)?;
    let burn = run_opt_in_burn_in(&mut fx, fixture)?;
    let telemetry = collect_resource_flow_opt_in_telemetry(
        &fx.session,
        fixture.name,
        fixture.opt_in_mode,
        Some(&burn),
        Some(&fx.boundary_metrics),
        0,
    );
    Ok((burn, telemetry))
}

pub fn open_product_session(fixture: &RfT2BurnInFixture) -> Result<RfT2OptInSession, SessionError> {
    open_fixture_session(fixture)
}

pub fn telemetry_for_open_session(
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

pub fn run_multi_session_replay(
    fixture: &RfT2BurnInFixture,
) -> Result<(RfT2BurnInReport, RfT2BurnInReport), SessionError> {
    let mut fx_a = open_fixture_session(fixture)?;
    let mut fx_b = clone_for_replay(&fx_a, fixture);
    let report_a = run_opt_in_burn_in(&mut fx_a, fixture)?;
    let report_b = run_opt_in_burn_in(&mut fx_b, fixture)?;
    Ok((report_a, report_b))
}

pub fn assert_telemetry_contract(
    telemetry: &ResourceFlowOptInTelemetryReport,
    fixture: &RfT2BurnInFixture,
) {
    assert_eq!(telemetry.scenario_name, fixture.name);
    assert_eq!(telemetry.opt_in_mode, fixture.opt_in_mode);
    assert_eq!(
        telemetry.resource_flow_enabled,
        fixture.opt_in_mode == ResourceFlowOptInMode::FlatStarOptIn
    );
    if fixture.opt_in_mode == ResourceFlowOptInMode::Disabled {
        assert_eq!(
            telemetry.flag_source,
            crate::resource_flow_opt_in_telemetry::ResourceFlowFlagSource::DefaultDisabled
        );
        assert!(!telemetry.resource_flow_enabled);
    } else {
        assert_eq!(
            telemetry.flag_source,
            crate::resource_flow_opt_in_telemetry::ResourceFlowFlagSource::SpecFlatStarOptIn
        );
        assert!(telemetry.resource_flow_enabled);
    }
    if fixture.expected_admissions > 0 || fixture.expected_rejections > 0 {
        assert_eq!(telemetry.dynamic_admissions, fixture.expected_admissions);
        assert_eq!(telemetry.dynamic_rejections, fixture.expected_rejections);
    }
    if fixture.expect_gpu_active {
        assert!(telemetry.total_ops > 0 || fixture.ticks == 0);
        assert!(telemetry.n_bands > 0 || fixture.ticks == 0);
    }
}
