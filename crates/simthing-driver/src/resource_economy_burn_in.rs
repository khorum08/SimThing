//! Controlled resource economy burn-in reporting (driver-only, test/reporting).

use simthing_core::{
    ConjunctiveRecipeRegistration, DiscreteTransferRegistration,
};
use simthing_gpu::{
    set_debug_readback_allowed, AccumulatorPipelineSessions, EmissionRecord, EmissionRegistration,
    Pipelines, WorldGpuState,
};

use crate::resource_economy_oracle::{
    run_emission_cpu_oracle, run_transfer_recipe_cpu_oracle, ResourceEconomyOracleError,
};
use crate::resource_economy_sync::ResourceEconomySyncReport;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceEconomyBurnInReport {
    pub ticks_checked: u32,
    pub boundaries_checked: u32,
    pub transfer_ops_uploaded: u32,
    pub emission_ops_uploaded: u32,
    pub transfer_upload_skips: u32,
    pub emission_upload_skips: u32,
    pub max_abs_conservation_error: f32,
    pub replay_bit_exact: bool,
}

impl ResourceEconomyBurnInReport {
    pub fn from_sync(sync: &ResourceEconomySyncReport) -> Self {
        Self {
            transfer_upload_skips: u32::from(sync.transfer_upload_skipped),
            emission_upload_skips: u32::from(sync.emission_upload_skipped),
            ..Default::default()
        }
    }

    pub fn note_sync_uploads(
        &mut self,
        state: &WorldGpuState,
        sync: &ResourceEconomySyncReport,
    ) {
        if let Some(runtime) = state.accumulator_runtime.as_ref() {
            self.transfer_ops_uploaded = runtime.transfer_op_upload_count() as u32;
            self.emission_ops_uploaded = runtime.emission_op_upload_count() as u32;
        }
        self.transfer_upload_skips += u32::from(sync.transfer_upload_skipped);
        self.emission_upload_skips += u32::from(sync.emission_upload_skipped);
    }

    pub fn finalize_replay_bit_exact(&mut self) {
        self.replay_bit_exact = self.max_abs_conservation_error.to_bits() == 0.0_f32.to_bits();
    }
}

fn run_accumulator_transfer(state: &mut WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    let mut transfer_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_transfer_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: transfer_session.as_mut(),
            emission: None,
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_transfer_session(transfer_session);
    state.read_values()
}

fn run_accumulator_emission(
    state: &mut WorldGpuState,
    dt: f32,
) -> Result<Vec<EmissionRecord>, simthing_gpu::AccumulatorOpSessionError> {
    set_debug_readback_allowed(true);
    let pipelines = Pipelines::new(&state.ctx);
    let mut emission_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_emission_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: None,
            emission: emission_session.as_mut(),
            encode_world_summary: false,
        },
    );
    let gpu_records = emission_session.as_ref().unwrap().readback_emissions(&state.ctx)?;
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_emission_session(emission_session);
    Ok(gpu_records)
}

/// Run transfer/recipe burn-in: compare GPU transfer bands to CPU oracle on watched cells.
pub fn run_transfer_recipe_burn_in(
    state: &mut WorldGpuState,
    n_dims: u32,
    initial_flat: &[f32],
    transfers: &[DiscreteTransferRegistration],
    recipes: &[ConjunctiveRecipeRegistration],
    watched_cells: &[(u32, u32)],
    ticks: u32,
    dt: f32,
) -> Result<ResourceEconomyBurnInReport, ResourceEconomyOracleError> {
    let idx = |slot: u32, col: u32| crate::resource_economy_oracle::cell_index(slot, col, n_dims);
    let mut report = ResourceEconomyBurnInReport::default();

    for _ in 0..ticks {
        let mut flat = initial_flat.to_vec();
        state.write_values(&flat);

        run_transfer_recipe_cpu_oracle(&mut flat, n_dims, transfers, recipes)?;

        let gpu_out = run_accumulator_transfer(state, dt);

        if recipes.is_empty() && !transfers.is_empty() {
            let before_sum: f32 = watched_cells
                .iter()
                .map(|&(slot, col)| initial_flat[idx(slot, col)])
                .sum();
            let after_sum: f32 = watched_cells
                .iter()
                .map(|&(slot, col)| gpu_out[idx(slot, col)])
                .sum();
            let err = (before_sum - after_sum).abs();
            if err > report.max_abs_conservation_error {
                report.max_abs_conservation_error = err;
            }
        }

        for &(slot, col) in watched_cells {
            let cpu = flat[idx(slot, col)];
            let gpu = gpu_out[idx(slot, col)];
            let err = (cpu - gpu).abs();
            if err > report.max_abs_conservation_error {
                report.max_abs_conservation_error = err;
            }
        }
        report.ticks_checked += 1;
    }

    report.finalize_replay_bit_exact();
    Ok(report)
}

/// Run emission burn-in: compare GPU emission readback to CPU oracle emit counts.
pub fn run_emission_burn_in(
    state: &mut WorldGpuState,
    n_dims: u32,
    initial_flat: &[f32],
    emissions: &[EmissionRegistration],
    ticks: u32,
    dt: f32,
) -> Result<ResourceEconomyBurnInReport, ResourceEconomyOracleError> {
    let mut report = ResourceEconomyBurnInReport::default();

    for _ in 0..ticks {
        let flat = initial_flat.to_vec();
        state.write_values(&flat);

        let cpu_records = run_emission_cpu_oracle(&flat, n_dims, emissions)?;
        let gpu_records = run_accumulator_emission(state, dt)
            .map_err(|e| ResourceEconomyOracleError::Cpu(e.to_string()))?;

        let cpu_total: u32 = cpu_records.iter().map(|r| r.emit_count).sum();
        let gpu_total: u32 = gpu_records.iter().map(|r| r.emit_count).sum();
        let err = (cpu_total as f32 - gpu_total as f32).abs();
        if err > report.max_abs_conservation_error {
            report.max_abs_conservation_error = err;
        }
        report.ticks_checked += 1;
    }

    report.finalize_replay_bit_exact();
    Ok(report)
}
