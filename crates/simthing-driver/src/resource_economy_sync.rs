//! Phase T-4 — Session/boundary sync for materialized resource economy registrations.

use simthing_gpu::{
    conjunctive_recipe_registrations_to_transfer, discrete_transfer_registrations_to_transfer,
    WorldGpuState,
};
use thiserror::Error;

use crate::resource_economy_compile::ResourceEconomyRegistry;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResourceEconomySyncReport {
    pub transfer_enabled: bool,
    pub emission_enabled: bool,
    pub transfer_upload_skipped: bool,
    pub emission_upload_skipped: bool,
    pub transfer_registrations: u32,
    pub recipe_registrations: u32,
    pub emission_registrations: u32,
}

#[derive(Debug, Error)]
pub enum ResourceEconomySyncError {
    #[error(
        "resource economy has transfer/recipe registrations but use_accumulator_transfer is false"
    )]
    TransferFlagOffPopulatedSpec,

    #[error("resource economy has emission registrations but use_accumulator_emission is false")]
    EmissionFlagOffPopulatedSpec,

    #[error(transparent)]
    TransferUpload(#[from] simthing_gpu::TransferSyncError),

    #[error(transparent)]
    EmissionUpload(#[from] simthing_gpu::EmissionSyncError),
}

impl ResourceEconomyRegistry {
    pub fn has_transfer_content(&self) -> bool {
        !self.registrations.transfers.is_empty() || !self.registrations.recipes.is_empty()
    }

    pub fn has_emission_content(&self) -> bool {
        !self.registrations.emissions.is_empty()
    }
}

/// Upload transfer/recipe and emission registrations when pipeline flags allow.
///
/// Rejects populated specs when the corresponding flag is off. Skips GPU re-upload
/// when `registry.generation` matches `uploaded_generation` (generation-keyed skip).
pub fn sync_resource_economy_accumulator(
    state: &mut WorldGpuState,
    registry: &ResourceEconomyRegistry,
    uploaded_generation: &mut u64,
    transfer_enabled: bool,
    emission_enabled: bool,
    reject_flag_off_populated: bool,
) -> Result<ResourceEconomySyncReport, ResourceEconomySyncError> {
    let has_transfer = registry.has_transfer_content();
    let has_emission = registry.has_emission_content();

    if reject_flag_off_populated && has_transfer && !transfer_enabled {
        return Err(ResourceEconomySyncError::TransferFlagOffPopulatedSpec);
    }
    if reject_flag_off_populated && has_emission && !emission_enabled {
        return Err(ResourceEconomySyncError::EmissionFlagOffPopulatedSpec);
    }

    let skip_upload = registry.generation == *uploaded_generation && *uploaded_generation > 0;
    let mut uploaded_this_sync = false;
    let report = ResourceEconomySyncReport {
        transfer_enabled,
        emission_enabled,
        transfer_registrations: registry.registrations.transfers.len() as u32,
        recipe_registrations: registry.registrations.recipes.len() as u32,
        emission_registrations: registry.registrations.emissions.len() as u32,
        transfer_upload_skipped: skip_upload && has_transfer && transfer_enabled,
        emission_upload_skipped: skip_upload && has_emission && emission_enabled,
    };

    if !transfer_enabled {
        if let Some(runtime) = state.accumulator_runtime.as_mut() {
            runtime.clear_transfer();
        }
        state.set_transfer_dispatch(false, 0);
    } else if has_transfer && !skip_upload {
        let mut gpu_regs =
            discrete_transfer_registrations_to_transfer(&registry.registrations.transfers);
        gpu_regs.extend(conjunctive_recipe_registrations_to_transfer(
            &registry.registrations.recipes,
        ));
        state.sync_transfer_accumulator(&gpu_regs)?;
        uploaded_this_sync = true;
    }

    if !emission_enabled {
        if let Some(runtime) = state.accumulator_runtime.as_mut() {
            runtime.clear_emission();
        }
        state.set_emission_dispatch(false, 0);
    } else if has_emission && !skip_upload {
        state.sync_emission_accumulator(&registry.registrations.emissions)?;
        uploaded_this_sync = true;
    }

    if uploaded_this_sync {
        *uploaded_generation = registry.generation;
    }

    Ok(report)
}

/// Convenience for session integration: sync when registry is present.
pub fn sync_resource_economy_if_present(
    state: &mut WorldGpuState,
    registry: Option<&ResourceEconomyRegistry>,
    uploaded_generation: &mut u64,
    transfer_enabled: bool,
    emission_enabled: bool,
    reject_flag_off_populated: bool,
) -> Result<Option<ResourceEconomySyncReport>, ResourceEconomySyncError> {
    let Some(registry) = registry else {
        return Ok(None);
    };
    Ok(Some(sync_resource_economy_accumulator(
        state,
        registry,
        uploaded_generation,
        transfer_enabled,
        emission_enabled,
        reject_flag_off_populated,
    )?))
}
