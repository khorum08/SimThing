//! RF-T3 — Resource Flow opt-in telemetry and flag-source attribution (driver/test-reporting).

use simthing_spec::ResourceFlowOptInMode;

use crate::resource_flow_dynamic_enrollment_soak::DynamicEnrollmentBoundaryMetrics;
use crate::resource_flow_opt_in_burn_in::RfT2BurnInReport;
use crate::session::SimSession;

/// Why `use_accumulator_resource_flow` is set on the session.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResourceFlowFlagSource {
    #[default]
    DefaultDisabled,
    SpecFlatStarOptIn,
    TestOverride,
}

/// Scenario/session telemetry for product-like opt-in soak (driver/test-reporting only).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFlowOptInTelemetryReport {
    pub scenario_name: String,
    pub opt_in_mode: ResourceFlowOptInMode,
    pub flag_source: ResourceFlowFlagSource,
    pub resource_flow_enabled: bool,
    pub arenas_planned: u32,
    pub participants_planned: u32,
    pub total_ops: u32,
    pub n_bands: u32,
    pub generation_start: u64,
    pub generation_end: u64,
    pub dynamic_admissions: u32,
    pub dynamic_rejections: u32,
    pub sync_count: u32,
    pub max_abs_error: f32,
    pub replay_bit_exact: bool,
}

pub fn flag_source_from_opt_in_mode(mode: ResourceFlowOptInMode) -> ResourceFlowFlagSource {
    match mode {
        ResourceFlowOptInMode::Disabled => ResourceFlowFlagSource::DefaultDisabled,
        ResourceFlowOptInMode::FlatStarOptIn => ResourceFlowFlagSource::SpecFlatStarOptIn,
    }
}

/// Collect telemetry from an open session plus optional burn-in / boundary metrics.
pub fn collect_resource_flow_opt_in_telemetry(
    session: &SimSession,
    scenario_name: impl Into<String>,
    opt_in_mode: ResourceFlowOptInMode,
    burn: Option<&RfT2BurnInReport>,
    boundary_metrics: Option<&DynamicEnrollmentBoundaryMetrics>,
    extra_sync_count: u32,
) -> ResourceFlowOptInTelemetryReport {
    let enrollment = session.last_resource_flow_dynamic_enrollment_report.as_ref();
    let metrics = boundary_metrics.cloned().unwrap_or_default();

    let dynamic_admissions = enrollment
        .map(|r| r.admissions.len() as u32)
        .unwrap_or(metrics.admissions_observed);
    let dynamic_rejections = enrollment
        .map(|r| r.rejections.len() as u32)
        .unwrap_or(metrics.rejections_observed);

    let generation_start = enrollment
        .map(|r| r.generation_before)
        .unwrap_or(metrics.generation_start);
    let generation_end = session.spec_state.arena_registry.generation;

    let sync_count = if metrics.resource_flow_syncs_observed > 0 {
        metrics.resource_flow_syncs_observed
    } else {
        extra_sync_count.max(if session.proto.flags.use_accumulator_resource_flow {
            1
        } else {
            0
        })
    };

    let total_ops = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.resource_flow_ops.count)
        .unwrap_or(0);
    let n_bands = session.state.accumulator_resource_flow_bands;
    let max_abs_error = burn.map(|b| b.max_abs_error).unwrap_or(0.0);
    let replay_bit_exact = burn.map(|b| b.replay_bit_exact).unwrap_or_else(|| {
        max_abs_error.to_bits() == 0.0_f32.to_bits()
    });

    ResourceFlowOptInTelemetryReport {
        scenario_name: scenario_name.into(),
        opt_in_mode,
        flag_source: session.resource_flow_flag_source,
        resource_flow_enabled: session.proto.flags.use_accumulator_resource_flow,
        arenas_planned: session.spec_state.arena_registry.arenas.len() as u32,
        participants_planned: session.spec_state.arena_registry.participants.len() as u32,
        total_ops,
        n_bands,
        generation_start,
        generation_end,
        dynamic_admissions,
        dynamic_rejections,
        sync_count,
        max_abs_error,
        replay_bit_exact,
    }
}
