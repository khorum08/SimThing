//! PALMA-PATH-6 — opt-in Location min-plus field band over admitted W/D property columns.
//!
//! Wires existing GPU [`MinPlusStencilOp`] behind the generic [`FieldScheduler`] cadence layer
//! (same posture as [`FirstSliceMappingSession`]). Not wired into default `SimSession` tick;
//! callers opt in explicitly via [`PalmaMinPlusFieldBandSession::enable`].

use simthing_core::SimThingId;
use simthing_gpu::{
    cpu_min_plus_d_from_w, extract_d_flat, max_d_field_error, pack_w_and_initial_d, GpuContext,
    MinPlusStencilConfig, MinPlusStencilError, MinPlusStencilOp, SlotAllocator,
};
use thiserror::Error;

use crate::field_scheduler::{
    FieldCadence, FieldDispatchSchedule, FieldId, FieldRegionId, FieldRegionRegistration,
    FieldScheduleState, FieldScheduler, FieldSchedulerError, FieldSchedulerReport,
};

pub const PALMA_MIN_PLUS_FIELD_BAND_PROFILE_ID: &str = "palma_min_plus_traversal_v1";
pub const PALMA_MIN_PLUS_FIELD_BAND_DEFAULT_ENABLED: bool = false;

/// PALMA-PATH-6 field band id in the generic scheduler namespace.
pub const PALMA_MIN_PLUS_FIELD_ID: FieldId = FieldId(0xF1EE_0001);
pub const PALMA_MIN_PLUS_REGION_ID: FieldRegionId = FieldRegionId(0);

/// Grid binding: row-major gridcell ids map to session shadow W/D columns.
#[derive(Clone, Debug, PartialEq)]
pub struct PalmaMinPlusGridBinding {
    pub width: u32,
    pub height: u32,
    pub dest_x: u32,
    pub dest_y: u32,
    pub iterations: u32,
    pub w_global_col: usize,
    pub d_global_col: usize,
    pub gridcell_ids: Vec<SimThingId>,
}

impl PalmaMinPlusGridBinding {
    pub fn cells(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn validate(&self) -> Result<(), PalmaMinPlusFieldBandError> {
        if self.width == 0 || self.height == 0 {
            return Err(MinPlusStencilError::InvalidDimensions {
                width: self.width,
                height: self.height,
            }
            .into());
        }
        if self.gridcell_ids.len() != self.cells() {
            return Err(PalmaMinPlusFieldBandError::GridcellIdCount {
                expected: self.cells(),
                actual: self.gridcell_ids.len(),
            });
        }
        self.stencil_config().validate()?;
        self.stencil_config().validate_iterations(self.iterations)?;
        Ok(())
    }

    pub fn stencil_config(&self) -> MinPlusStencilConfig {
        MinPlusStencilConfig {
            width: self.width,
            height: self.height,
            n_dims: 2,
            d_col: 0,
            w_col: 1,
            dest_x: self.dest_x,
            dest_y: self.dest_y,
            inf_sentinel: simthing_gpu::MIN_PLUS_INF,
        }
    }
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum PalmaMinPlusFieldBandError {
    #[error(transparent)]
    Scheduler(#[from] FieldSchedulerError),
    #[error(transparent)]
    Stencil(#[from] MinPlusStencilError),
    #[error("gridcell id count mismatch: expected {expected}, got {actual}")]
    GridcellIdCount { expected: usize, actual: usize },
    #[error("shadow buffer too short: need {required}, got {actual}")]
    ShadowTooShort { required: usize, actual: usize },
    #[error("min-plus field band is disabled")]
    Disabled,
    #[error("GPU MinPlusStencilOp not initialized")]
    OpNotInitialized,
}

/// Opt-in session band: scheduler decides cadence; GPU min-plus runs when scheduled.
pub struct PalmaMinPlusFieldBandSession {
    enabled: bool,
    tick: u32,
    scheduler: FieldScheduler,
    binding: PalmaMinPlusGridBinding,
    op: Option<MinPlusStencilOp>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PalmaMinPlusFieldBandTickReport {
    pub profile_id: &'static str,
    pub tick: u32,
    pub enabled: bool,
    pub scheduled: bool,
    pub gpu_dispatched: bool,
    pub iterations: u32,
    pub scheduler_report: FieldSchedulerReport,
    /// True when caller requested verification readback (not required for residency).
    pub verification_readback: bool,
    pub max_oracle_error: Option<f32>,
}

impl PalmaMinPlusFieldBandSession {
    pub fn new(
        binding: PalmaMinPlusGridBinding,
        cadence: FieldCadence,
    ) -> Result<Self, PalmaMinPlusFieldBandError> {
        binding.validate()?;
        cadence.validate()?;
        let mut scheduler = FieldScheduler::new();
        scheduler.register_field(FieldScheduleState {
            field_id: PALMA_MIN_PLUS_FIELD_ID,
            cadence,
            event_pending: false,
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: PALMA_MIN_PLUS_REGION_ID,
            field_id: PALMA_MIN_PLUS_FIELD_ID,
            dirty: crate::field_scheduler::DirtyRegionState::default(),
        });
        Ok(Self {
            enabled: PALMA_MIN_PLUS_FIELD_BAND_DEFAULT_ENABLED,
            tick: 0,
            scheduler,
            binding,
            op: None,
        })
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn binding(&self) -> &PalmaMinPlusGridBinding {
        &self.binding
    }

    pub fn scheduler(&self) -> &FieldScheduler {
        &self.scheduler
    }

    fn ensure_op(
        &mut self,
        ctx: &GpuContext,
    ) -> Result<&MinPlusStencilOp, PalmaMinPlusFieldBandError> {
        if self.op.is_none() {
            self.op = Some(MinPlusStencilOp::new(ctx, self.binding.stencil_config())?);
        }
        Ok(self.op.as_ref().expect("op initialized"))
    }

    /// Gather row-major W from admitted session shadow columns.
    pub fn gather_w_from_shadow(
        shadow: &[f32],
        n_dims: usize,
        alloc: &SlotAllocator,
        binding: &PalmaMinPlusGridBinding,
    ) -> Result<Vec<f32>, PalmaMinPlusFieldBandError> {
        binding.validate()?;
        let cells = binding.cells();
        let required = alloc.capacity() * n_dims;
        if shadow.len() < required {
            return Err(PalmaMinPlusFieldBandError::ShadowTooShort {
                required,
                actual: shadow.len(),
            });
        }
        let mut w = Vec::with_capacity(cells);
        for cell_id in &binding.gridcell_ids {
            let slot =
                alloc
                    .slot_of(*cell_id)
                    .ok_or(PalmaMinPlusFieldBandError::ShadowTooShort {
                        required,
                        actual: shadow.len(),
                    })? as usize;
            w.push(shadow[slot * n_dims + binding.w_global_col]);
        }
        Ok(w)
    }

    /// Scatter row-major D into admitted session shadow columns.
    pub fn scatter_d_to_shadow(
        shadow: &mut [f32],
        n_dims: usize,
        alloc: &SlotAllocator,
        binding: &PalmaMinPlusGridBinding,
        d: &[f32],
    ) -> Result<(), PalmaMinPlusFieldBandError> {
        binding.validate()?;
        if d.len() != binding.cells() {
            return Err(MinPlusStencilError::BufferTooShort {
                actual: d.len(),
                required: binding.cells(),
            }
            .into());
        }
        let required = alloc.capacity() * n_dims;
        if shadow.len() < required {
            return Err(PalmaMinPlusFieldBandError::ShadowTooShort {
                required,
                actual: shadow.len(),
            });
        }
        for (idx, cell_id) in binding.gridcell_ids.iter().enumerate() {
            let slot =
                alloc
                    .slot_of(*cell_id)
                    .ok_or(PalmaMinPlusFieldBandError::ShadowTooShort {
                        required,
                        actual: shadow.len(),
                    })? as usize;
            shadow[slot * n_dims + binding.d_global_col] = d[idx];
        }
        Ok(())
    }

    /// One band tick: scheduler → optional GPU min-plus → D writeback to shadow.
    ///
    /// When `verify_oracle` is true, performs verification readback and CPU oracle compare
    /// (diagnostic only — not a runtime dependency).
    pub fn tick(
        &mut self,
        ctx: &GpuContext,
        shadow: &mut [f32],
        n_dims: usize,
        alloc: &SlotAllocator,
        verify_oracle: bool,
    ) -> Result<PalmaMinPlusFieldBandTickReport, PalmaMinPlusFieldBandError> {
        let tick = self.tick;
        if !self.enabled {
            self.tick += 1;
            return Ok(PalmaMinPlusFieldBandTickReport {
                profile_id: PALMA_MIN_PLUS_FIELD_BAND_PROFILE_ID,
                tick,
                enabled: false,
                scheduled: false,
                gpu_dispatched: false,
                iterations: 0,
                scheduler_report: FieldSchedulerReport {
                    total_regions: 0,
                    scheduled_regions: 0,
                    skipped_regions: 0,
                    skip_ratio: 0.0,
                    false_skip_count: 0,
                },
                verification_readback: false,
                max_oracle_error: None,
            });
        }

        let (decisions, scheduler_report) = self.scheduler.decide_tick(tick)?;
        let scheduled = decisions.iter().any(|d| {
            d.field_id == PALMA_MIN_PLUS_FIELD_ID
                && d.region_id == PALMA_MIN_PLUS_REGION_ID
                && matches!(d.schedule, FieldDispatchSchedule::Dispatch)
        });

        let mut gpu_dispatched = false;
        let mut max_oracle_error = None;

        if scheduled {
            let w = Self::gather_w_from_shadow(shadow, n_dims, alloc, &self.binding)?;
            let config = self.binding.stencil_config();
            let iterations = self.binding.iterations;
            let op = self.ensure_op(ctx)?;
            let values = pack_w_and_initial_d(&w, &config)?;
            op.upload_values(ctx, &values)?;
            op.dispatch_ping_pong(ctx, iterations)?;
            gpu_dispatched = true;

            let gpu_values = op.readback_after_ping_pong(ctx, iterations);
            let gpu_d = extract_d_flat(&gpu_values, &config)?;
            Self::scatter_d_to_shadow(shadow, n_dims, alloc, &self.binding, &gpu_d)?;

            if verify_oracle {
                let cpu_d = cpu_min_plus_d_from_w(&w, &config, iterations)?;
                max_oracle_error = Some(max_d_field_error(&cpu_d, &gpu_d));
            }
        }

        self.tick += 1;
        Ok(PalmaMinPlusFieldBandTickReport {
            profile_id: PALMA_MIN_PLUS_FIELD_BAND_PROFILE_ID,
            tick,
            enabled: true,
            scheduled,
            gpu_dispatched,
            iterations: if gpu_dispatched {
                self.binding.iterations
            } else {
                0
            },
            scheduler_report,
            verification_readback: verify_oracle && gpu_dispatched,
            max_oracle_error,
        })
    }
}
