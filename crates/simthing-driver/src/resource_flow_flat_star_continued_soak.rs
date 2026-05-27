//! Continued flat-star Resource Flow soak checkpoint (confidence/observability only).
//!
//! Reuses RF-T5 `FlatStarResourceFlow` profile fixtures and telemetry. Does not expand
//! Resource Flow semantics.

use simthing_spec::ResourceFlowOptInMode;

use crate::resource_flow_opt_in_burn_in::{
    clone_for_replay, fixture_product_static_512_participants,
    fixture_profile_static_512_participants, fixture_static_flat_star_skewed_weights,
    run_opt_in_burn_in, RfT2BurnInFixture, RfT2BurnInReport, RfT2OptInSession,
    RF_CONTINUED_DYNAMIC_POLICY_A, RF_CONTINUED_MULTI_ARENA, RF_CONTINUED_REPLAY,
    RF_CONTINUED_STATIC_512, RF_CONTINUED_STATIC_SKEWED,
};
use crate::resource_flow_scenario_class_burn_in::{
    assert_profile_telemetry_contract, fixture_profile_dynamic_fission_cadence,
    fixture_profile_multi_arena_no_coupling, fixture_profile_multi_session_replay,
    open_profile_session, run_profile_soak_with_telemetry,
};
use crate::resource_flow_opt_in_telemetry::{
    collect_resource_flow_opt_in_telemetry, ResourceFlowFlagSource,
    ResourceFlowOptInTelemetryReport,
};
use crate::session::SessionError;

/// Compact operator-facing summary for continued flat-star soak runs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlatStarContinuedSoakSummary {
    pub scenario_name: String,
    pub participant_count: u32,
    pub ticks_checked: u32,
    pub max_abs_error: f32,
    pub replay_bit_exact: bool,
    pub total_ops: u32,
    pub n_bands: u32,
    pub flag_source: ResourceFlowFlagSource,
    pub execution_profile_name: String,
    pub dynamic_admissions: u32,
    pub dynamic_rejections: u32,
    pub generation_end: u64,
    pub resource_flow_enabled: bool,
}

impl FlatStarContinuedSoakSummary {
    pub fn from_reports(
        fixture: &RfT2BurnInFixture,
        burn: &RfT2BurnInReport,
        telemetry: &ResourceFlowOptInTelemetryReport,
    ) -> Self {
        Self {
            scenario_name: burn.scenario_name.clone(),
            participant_count: fixture.participant_count,
            ticks_checked: burn.ticks_checked,
            max_abs_error: burn.max_abs_error,
            replay_bit_exact: burn.replay_bit_exact,
            total_ops: burn.total_ops,
            n_bands: burn.n_bands,
            flag_source: telemetry.flag_source,
            execution_profile_name: telemetry.execution_profile_name.clone(),
            dynamic_admissions: telemetry.dynamic_admissions,
            dynamic_rejections: telemetry.dynamic_rejections,
            generation_end: burn.generation_end,
            resource_flow_enabled: telemetry.resource_flow_enabled,
        }
    }
}

pub fn continued_static_512_participant_count() -> u32 {
    fixture_product_static_512_participants().participant_count
}

pub fn fixture_continued_static_512_participants() -> RfT2BurnInFixture {
    fixture_profile_static_512_participants()
}

pub fn fixture_continued_static_skewed_weights() -> RfT2BurnInFixture {
    let mut fixture = fixture_static_flat_star_skewed_weights();
    fixture.name = RF_CONTINUED_STATIC_SKEWED;
    fixture.opt_in_mode = ResourceFlowOptInMode::Disabled;
    fixture.ticks = 1000;
    fixture.require_bit_exact = false;
    fixture
}

pub fn fixture_continued_dynamic_policy_a() -> RfT2BurnInFixture {
    let mut fixture = fixture_profile_dynamic_fission_cadence();
    fixture.name = RF_CONTINUED_DYNAMIC_POLICY_A;
    fixture
}

pub fn fixture_continued_multi_arena_no_coupling() -> RfT2BurnInFixture {
    let mut fixture = fixture_profile_multi_arena_no_coupling();
    fixture.name = RF_CONTINUED_MULTI_ARENA;
    fixture
}

pub fn fixture_continued_replay() -> RfT2BurnInFixture {
    let mut fixture = fixture_profile_multi_session_replay();
    fixture.name = RF_CONTINUED_REPLAY;
    fixture
}

pub fn run_continued_soak_with_summary(
    fixture: &RfT2BurnInFixture,
) -> Result<(FlatStarContinuedSoakSummary, RfT2BurnInReport, ResourceFlowOptInTelemetryReport), SessionError>
{
    let (burn, telemetry) = run_profile_soak_with_telemetry(fixture)?;
    let summary = FlatStarContinuedSoakSummary::from_reports(fixture, &burn, &telemetry);
    Ok((summary, burn, telemetry))
}

pub fn run_continued_replay_pair(
    fixture: &RfT2BurnInFixture,
) -> Result<
    (
        FlatStarContinuedSoakSummary,
        FlatStarContinuedSoakSummary,
        ResourceFlowOptInTelemetryReport,
    ),
    SessionError,
> {
    let mut fx_a = open_profile_session(fixture)?;
    let telemetry = collect_resource_flow_opt_in_telemetry(
        &fx_a.session,
        fixture.name,
        fixture.opt_in_mode,
        None,
        Some(&fx_a.boundary_metrics),
        0,
    );
    assert_profile_telemetry_contract(&telemetry, fixture);
    let mut fx_b = clone_for_replay(&fx_a, fixture);
    let burn_a = run_opt_in_burn_in(&mut fx_a, fixture)?;
    let burn_b = run_opt_in_burn_in(&mut fx_b, fixture)?;
    let summary_a = FlatStarContinuedSoakSummary::from_reports(fixture, &burn_a, &telemetry);
    let summary_b = FlatStarContinuedSoakSummary::from_reports(fixture, &burn_b, &telemetry);
    Ok((summary_a, summary_b, telemetry))
}

pub fn open_continued_profile_session(
    fixture: &RfT2BurnInFixture,
) -> Result<RfT2OptInSession, SessionError> {
    open_profile_session(fixture)
}

pub use crate::resource_flow_scenario_class_burn_in::open_default_profile_session;
