//! Controlled flat-star Resource Flow burn-in reporting (driver-only, test/reporting).

use std::collections::HashMap;

use simthing_gpu::WorldGpuState;

use crate::arena_allocation_oracle::run_arena_allocation_oracle;
use crate::arena_allocation_sync::ResourceFlowSyncReport;
use crate::arena_hierarchy::{ArenaTreeLayout, NodeColumnRefs};

type CellKey = (u32, u32);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFlowBurnInReport {
    pub arenas_planned: u32,
    pub total_ops: u32,
    pub n_bands: u32,
    pub ticks_checked: u32,
    pub max_abs_error: f32,
}

impl ResourceFlowBurnInReport {
    pub fn from_sync(sync: &ResourceFlowSyncReport) -> Self {
        Self {
            arenas_planned: sync.arenas_planned,
            total_ops: sync.total_ops,
            n_bands: sync.n_bands,
            ticks_checked: 0,
            max_abs_error: 0.0,
        }
    }
}

/// Per-scenario burn-in report (driver/test-reporting only; no runtime policy branching).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFlowScenarioBurnInReport {
    pub scenario_name: String,
    pub arenas_planned: u32,
    pub total_ops: u32,
    pub n_bands: u32,
    pub ticks_checked: u32,
    pub max_abs_error: f32,
    pub replay_bit_exact: bool,
}

impl ResourceFlowScenarioBurnInReport {
    pub fn from_parts(
        scenario_name: impl Into<String>,
        sync: &ResourceFlowSyncReport,
        burn: &ResourceFlowBurnInReport,
    ) -> Self {
        Self {
            scenario_name: scenario_name.into(),
            arenas_planned: sync.arenas_planned,
            total_ops: sync.total_ops,
            n_bands: burn.n_bands.max(sync.n_bands),
            ticks_checked: burn.ticks_checked,
            max_abs_error: burn.max_abs_error,
            replay_bit_exact: burn.max_abs_error.to_bits() == 0.0_f32.to_bits(),
        }
    }
}

/// CI soak summary (driver/test-reporting only; no runtime policy branching).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFlowSoakSummaryReport {
    pub scenario_name: String,
    pub ticks_checked: u32,
    pub sync_cycles_checked: u32,
    pub total_ops: u32,
    pub n_bands: u32,
    pub max_abs_error: f32,
    pub replay_bit_exact: bool,
}

impl ResourceFlowSoakSummaryReport {
    pub fn from_parts(
        scenario_name: impl Into<String>,
        sync: &ResourceFlowSyncReport,
        burn: &ResourceFlowBurnInReport,
        sync_cycles_checked: u32,
        require_bit_exact: bool,
    ) -> Self {
        let replay_bit_exact = burn.max_abs_error.to_bits() == 0.0_f32.to_bits();
        Self {
            scenario_name: scenario_name.into(),
            ticks_checked: burn.ticks_checked,
            sync_cycles_checked,
            total_ops: sync.total_ops,
            n_bands: burn.n_bands.max(sync.n_bands),
            max_abs_error: burn.max_abs_error,
            replay_bit_exact: require_bit_exact && replay_bit_exact,
        }
    }

    pub fn assert_within_contract(&self, require_bit_exact: bool, max_abs_error_allowed: f32) {
        if require_bit_exact {
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
        } else {
            assert!(
                self.max_abs_error <= max_abs_error_allowed,
                "soak {name} max_abs_error {err} exceeds allowed {allowed}",
                name = self.scenario_name,
                err = self.max_abs_error,
                allowed = max_abs_error_allowed
            );
        }
    }
}

/// Run `ticks` flat-star allocation passes on GPU and compare leaf `allocated_flow` to the CPU oracle.
pub fn run_flat_star_burn_in(
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
    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;
    let mut report = ResourceFlowBurnInReport {
        n_bands,
        ..Default::default()
    };

    for _ in 0..ticks {
        let mut flat = vec![0.0_f32; (state.n_slots * n_dims) as usize];
        for (&(slot, col), &v) in cell_inputs {
            flat[idx(slot, col)] = v;
        }
        state.write_values(&flat);

        let mut oracle = cell_inputs.clone();
        run_arena_allocation_oracle(layout, &mut oracle, dt);

        state.run_resource_flow_bands(n_bands, dt);
        let gpu_out = state.read_values();

        for &leaf in leaf_slots {
            let cpu = oracle
                .get(&(leaf, cols.allocated_flow_col))
                .copied()
                .unwrap_or(0.0);
            let gpu = gpu_out[idx(leaf, cols.allocated_flow_col)];
            let err = (cpu - gpu).abs();
            if err > report.max_abs_error {
                report.max_abs_error = err;
            }
        }
        report.ticks_checked += 1;
    }

    report
}
