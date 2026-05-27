//! Controlled opt-in soak reporting for E-2B-5R dynamic fission enrollment (driver/test-only).

use std::collections::HashMap;

use simthing_gpu::WorldGpuState;

use crate::arena_allocation_sync::ResourceFlowSyncReport;
use crate::arena_hierarchy::{ArenaTreeLayout, NodeColumnRefs};
use crate::resource_flow_burn_in::{run_flat_star_burn_in, ResourceFlowBurnInReport};
use crate::session::SimSession;

type CellKey = (u32, u32);

/// Boundary-time enrollment metrics collected during soak setup (driver/test-reporting only).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DynamicEnrollmentBoundaryMetrics {
    pub boundaries_checked: u32,
    pub fissions_observed: u32,
    pub admissions_observed: u32,
    pub rejections_observed: u32,
    pub generation_start: u64,
    pub generation_end: u64,
    pub resource_flow_syncs_observed: u32,
}

impl DynamicEnrollmentBoundaryMetrics {
    pub fn from_enrollment_report(
        report: &crate::resource_flow_fission_enrollment::DynamicFissionEnrollmentReport,
        boundaries_checked: u32,
        fissions_observed: u32,
        resource_flow_syncs_observed: u32,
    ) -> Self {
        Self {
            boundaries_checked,
            fissions_observed,
            admissions_observed: report.admissions.len() as u32,
            rejections_observed: report.rejections.len() as u32,
            generation_start: report.generation_before,
            generation_end: report.generation_after,
            resource_flow_syncs_observed,
        }
    }
}

/// CI soak summary for dynamic enrollment scenarios (driver/test-reporting only).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DynamicEnrollmentSoakReport {
    pub scenario_name: String,
    pub ticks_checked: u32,
    pub boundaries_checked: u32,
    pub fissions_observed: u32,
    pub admissions_observed: u32,
    pub rejections_observed: u32,
    pub generation_start: u64,
    pub generation_end: u64,
    pub resource_flow_syncs_observed: u32,
    pub max_abs_error: f32,
    pub replay_bit_exact: bool,
}

impl DynamicEnrollmentSoakReport {
    pub fn from_parts(
        scenario_name: impl Into<String>,
        metrics: &DynamicEnrollmentBoundaryMetrics,
        burn: &ResourceFlowBurnInReport,
        require_bit_exact: bool,
    ) -> Self {
        let replay_bit_exact = burn.max_abs_error.to_bits() == 0.0_f32.to_bits();
        Self {
            scenario_name: scenario_name.into(),
            ticks_checked: burn.ticks_checked,
            boundaries_checked: metrics.boundaries_checked,
            fissions_observed: metrics.fissions_observed,
            admissions_observed: metrics.admissions_observed,
            rejections_observed: metrics.rejections_observed,
            generation_start: metrics.generation_start,
            generation_end: metrics.generation_end,
            resource_flow_syncs_observed: metrics.resource_flow_syncs_observed,
            max_abs_error: burn.max_abs_error,
            replay_bit_exact: require_bit_exact && replay_bit_exact,
        }
    }

    pub fn enrollment_only(
        scenario_name: impl Into<String>,
        metrics: &DynamicEnrollmentBoundaryMetrics,
    ) -> Self {
        Self {
            scenario_name: scenario_name.into(),
            boundaries_checked: metrics.boundaries_checked,
            fissions_observed: metrics.fissions_observed,
            admissions_observed: metrics.admissions_observed,
            rejections_observed: metrics.rejections_observed,
            generation_start: metrics.generation_start,
            generation_end: metrics.generation_end,
            resource_flow_syncs_observed: metrics.resource_flow_syncs_observed,
            replay_bit_exact: true,
            ..Default::default()
        }
    }

    pub fn assert_within_contract(&self, require_bit_exact: bool) {
        if require_bit_exact && self.ticks_checked > 0 {
            assert_eq!(
                self.max_abs_error.to_bits(),
                0.0_f32.to_bits(),
                "soak {name} must be bit-exact",
                name = self.scenario_name
            );
            assert!(
                self.replay_bit_exact,
                "soak {name} must report replay_bit_exact",
                name = self.scenario_name
            );
        }
    }
}

/// Run GPU flat-star burn-in ticks for a post-enrollment session.
pub fn run_dynamic_enrollment_gpu_burn_in(
    state: &mut WorldGpuState,
    layout: &ArenaTreeLayout,
    cols: NodeColumnRefs,
    n_dims: u32,
    cell_inputs: &HashMap<CellKey, f32>,
    leaf_slots: &[u32],
    n_bands: u32,
    ticks: u32,
    dt: f32,
) -> ResourceFlowBurnInReport {
    run_flat_star_burn_in(
        state,
        layout,
        cols,
        n_dims,
        cell_inputs,
        leaf_slots,
        n_bands,
        ticks,
        dt,
    )
}

/// Repeated Resource Flow sync cycles; returns (syncs_run, final_op_count, final_n_bands).
pub fn run_dynamic_enrollment_resync_cycles(
    session: &mut SimSession,
    sync_cycles: u32,
) -> Result<(u32, u32, u32), crate::session::SessionError> {
    let mut syncs_run = 0u32;
    let initial_ops = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.resource_flow_ops.count)
        .unwrap_or(0);
    let initial_bands = session.state.accumulator_resource_flow_bands;

    for _ in 0..sync_cycles {
        session.sync_resource_flow_if_enabled()?;
        syncs_run += 1;
        if sync_cycles > 1 {
            let ops = session
                .state
                .accumulator_runtime
                .as_ref()
                .map(|r| r.resource_flow_ops.count)
                .unwrap_or(0);
            assert_eq!(ops, initial_ops, "dynamic enrollment soak op count unstable");
            assert_eq!(
                session.state.accumulator_resource_flow_bands, initial_bands,
                "dynamic enrollment soak n_bands unstable"
            );
        }
    }

    let final_ops = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.resource_flow_ops.count)
        .unwrap_or(0);
    Ok((syncs_run, final_ops, session.state.accumulator_resource_flow_bands))
}

/// Initial sync via session wrapper; returns sync report when flag enabled.
pub fn initial_dynamic_enrollment_sync(
    session: &mut SimSession,
) -> Result<ResourceFlowSyncReport, crate::session::SessionError> {
    session.sync_resource_flow_if_enabled()?;
    Ok(ResourceFlowSyncReport {
        arenas_planned: session
            .spec_state
            .arena_registry
            .arenas
            .len() as u32,
        total_ops: session
            .state
            .accumulator_runtime
            .as_ref()
            .map(|r| r.resource_flow_ops.count)
            .unwrap_or(0),
        n_bands: session.state.accumulator_resource_flow_bands,
        enabled: session.proto.flags.use_accumulator_resource_flow,
    })
}
