//! Resident multi-theater atlas scheduler over already-compiled generic mapping plans.
//!
//! Schedules multiple `SimGpuMappingTickState` instances in stable slot order. Driver/spec
//! compile meaning stays outside sim; scheduler owns resident tick lifecycle only.

use crate::accumulator_plan_tick::SimTickError;
use crate::mapping_plan_tick::{
    CompiledMappingPlan, MappingTickInputs, SimGpuMappingReadbackPolicy, SimGpuMappingTickOutput,
    SimGpuMappingTickState,
};

/// Stable slot index for a scheduled theater — numeric only, no semantic names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MappingTheaterSlot(pub u32);

/// Batch of generic compiled mapping plans ready for resident sim scheduling.
#[derive(Clone, Debug)]
pub struct CompiledMappingAtlas {
    pub plans: Vec<CompiledMappingPlan>,
}

impl CompiledMappingAtlas {
    pub fn theater_count(&self) -> usize {
        self.plans.len()
    }
}

/// Per-tick inputs for all scheduled theaters (generic buffers only).
pub struct MappingAtlasTickInputs<'a> {
    pub theater_inputs: Vec<MappingTickInputs<'a>>,
}

/// Per-theater tick outputs in stable slot order — proof values are projection/cache only.
#[derive(Debug, Clone, PartialEq)]
pub struct MappingAtlasTickOutput {
    pub theater_outputs: Vec<SimGpuMappingTickOutput>,
}

/// Resident sim-owned scheduler over multiple compiled mapping plans.
pub struct SimGpuMappingAtlasScheduler {
    states: Vec<SimGpuMappingTickState>,
    tick_count: u32,
}

impl SimGpuMappingAtlasScheduler {
    /// Build one resident mapping tick state per compiled plan.
    pub fn new(
        ctx: &simthing_gpu::GpuContext,
        atlas: CompiledMappingAtlas,
    ) -> Result<Self, SimTickError> {
        if atlas.plans.is_empty() {
            return Err(SimTickError::Readback("empty mapping atlas".into()));
        }
        let mut states = Vec::with_capacity(atlas.plans.len());
        for plan in atlas.plans {
            states.push(SimGpuMappingTickState::new(ctx, plan)?);
        }
        Ok(Self {
            states,
            tick_count: 0,
        })
    }

    pub fn theater_count(&self) -> usize {
        self.states.len()
    }

    pub fn resident_tick_count(&self) -> u32 {
        self.tick_count
    }

    pub fn theater_resident_tick_count(
        &self,
        slot: MappingTheaterSlot,
    ) -> Result<u32, SimTickError> {
        let index = slot.0 as usize;
        self.states
            .get(index)
            .map(|state| state.resident_tick_count())
            .ok_or_else(|| SimTickError::InvalidInputLength {
                expected: self.states.len(),
                actual: index,
            })
    }

    /// Execute one scheduler tick: each resident state runs once with the shared readback policy.
    pub fn tick(
        &mut self,
        ctx: &simthing_gpu::GpuContext,
        inputs: MappingAtlasTickInputs<'_>,
        readback: SimGpuMappingReadbackPolicy,
    ) -> Result<MappingAtlasTickOutput, SimTickError> {
        if inputs.theater_inputs.len() != self.states.len() {
            return Err(SimTickError::InvalidInputLength {
                expected: self.states.len(),
                actual: inputs.theater_inputs.len(),
            });
        }

        let mut theater_outputs = Vec::with_capacity(self.states.len());
        for (state, theater_input) in self
            .states
            .iter_mut()
            .zip(inputs.theater_inputs.into_iter())
        {
            let output = state.tick(ctx, theater_input, readback)?;
            theater_outputs.push(output);
        }

        self.tick_count += 1;
        Ok(MappingAtlasTickOutput { theater_outputs })
    }
}
