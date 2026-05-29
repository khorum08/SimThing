//! Acceptance-test helper for FirstSliceScenarioSpec (not exported from simthing-driver).

use simthing_driver::{
    FirstSliceCommitmentReport, FirstSliceMappingError, FirstSliceMappingReport,
    FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions,
};
use simthing_gpu::GpuContext;
use simthing_spec::{
    CompiledFirstSliceCommitmentThreshold, CompiledFirstSliceScenarioPreview,
    MappingExecutionProfile,
};

/// Scenario fixture session: mapping session + commitment binding from the same admitted preview.
pub struct FirstSliceScenarioFixtureSession {
    session: FirstSliceMappingSession,
    commitment: Option<CompiledFirstSliceCommitmentThreshold>,
    mapping_execution_profile: MappingExecutionProfile,
}

impl FirstSliceScenarioFixtureSession {
    /// Open from an admitted scenario compile preview. Commitment binding is taken from the preview only.
    pub fn open(
        ctx: &GpuContext,
        preview: &CompiledFirstSliceScenarioPreview,
    ) -> Result<Self, FirstSliceMappingError> {
        let session = FirstSliceMappingSession::open_from_scenario_preview(ctx, preview)?;
        Ok(Self {
            commitment: preview.region_field.commitment.clone(),
            mapping_execution_profile: preview.mapping_execution_profile,
            session,
        })
    }

    pub fn queue_seeds(&mut self, seeds: &[FirstSliceSeed]) -> Result<(), FirstSliceMappingError> {
        self.session.queue_seeds(seeds)
    }

    pub fn tick_mapping(
        &mut self,
        ctx: &GpuContext,
        options: FirstSliceTickOptions,
        eml_weights: (f32, f32),
    ) -> Result<FirstSliceMappingReport, FirstSliceMappingError> {
        self.session.tick(ctx, options, eml_weights)
    }

    pub fn tick_with_scenario_commitment(
        &mut self,
        ctx: &GpuContext,
        options: FirstSliceTickOptions,
        eml_weights: (f32, f32),
    ) -> Result<FirstSliceCommitmentReport, FirstSliceMappingError> {
        let commitment = self
            .commitment
            .as_ref()
            .ok_or(FirstSliceMappingError::MissingCommitmentBinding)?;
        self.session
            .tick_with_commitment_spec_fixture(ctx, options, eml_weights, commitment)
    }

    pub fn diagnostic_readback_reduction_eml(
        &mut self,
        ctx: &GpuContext,
        eml_weights: (f32, f32),
    ) -> Result<(f32, f32), FirstSliceMappingError> {
        self.session
            .diagnostic_readback_reduction_eml(ctx, eml_weights)
    }
}
