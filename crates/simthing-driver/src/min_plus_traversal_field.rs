//! Generic GPU min-plus traversal field utility (PALMA algebraic provenance; not a runtime subsystem).
//!
//! Production callers use explicit GPU-resident dispatch — no public `tick()` scaffold.
//! CPU shadow gather/readback is diagnostic/compatibility only via explicitly named dispatch methods.

use simthing_core::SimThingId;
use simthing_gpu::wgpu::Buffer;
use simthing_gpu::{
    extract_d_flat, GpuContext, MinPlusStencilConfig, MinPlusStencilError,
    MinPlusTraversalDispatchReport, MinPlusTraversalExecutionMode,
    MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp, MinPlusTraversalInput,
    MinPlusTraversalWInputKind, SlotAllocator,
};
use thiserror::Error;

use crate::field_scheduler::{
    FieldCadence, FieldDispatchSchedule, FieldId, FieldRegionId, FieldRegionRegistration,
    FieldScheduleState, FieldScheduler, FieldSchedulerError, FieldSchedulerReport,
};

pub const TRAVERSAL_FIELD_UTILITY_ID: &str = "min_plus_traversal_field_v1";
pub const TRAVERSAL_FIELD_BAND_DEFAULT_ENABLED: bool = false;

pub const TRAVERSAL_FIELD_ID: FieldId = FieldId(0xF1EE_0001);
pub const TRAVERSAL_FIELD_REGION_ID: FieldRegionId = FieldRegionId(0);

pub use simthing_gpu::MinPlusTraversalExecutionMode as TraversalFieldExecutionMode;
pub use simthing_gpu::MinPlusTraversalExecutionOptions as TraversalFieldExecutionOptions;
pub use simthing_gpu::MinPlusTraversalGpuOutputHandle as TraversalFieldGpuOutputHandle;
pub use simthing_gpu::MinPlusTraversalWInputKind as TraversalFieldWInputKind;

/// GPU-native W source for production traversal dispatch.
#[derive(Debug)]
pub enum TraversalFieldGpuInput<'a> {
    /// Flat `cells` f32 buffer produced by an upstream field pass.
    FlatW { buffer: &'a Buffer },
    /// Interleaved values buffer with W in `binding.w_col`.
    InterleavedW { buffer: &'a Buffer },
}

impl<'a> TraversalFieldGpuInput<'a> {
    pub fn w_input_kind(&self) -> TraversalFieldWInputKind {
        match self {
            Self::FlatW { .. } => MinPlusTraversalWInputKind::GpuFlatW,
            Self::InterleavedW { .. } => MinPlusTraversalWInputKind::GpuInterleavedW,
        }
    }

    fn to_gpu_input(self) -> MinPlusTraversalInput<'a> {
        match self {
            Self::FlatW { buffer } => MinPlusTraversalInput::GpuFlatW(buffer),
            Self::InterleavedW { buffer } => MinPlusTraversalInput::GpuInterleavedW(buffer),
        }
    }
}

/// Explicit diagnostic/compatibility input: CPU shadow W gather (PATH-5/6 bridge only).
#[derive(Debug)]
pub struct TraversalFieldShadowColumnCompatInput<'a> {
    pub shadow: &'a mut [f32],
    pub n_dims: usize,
    pub alloc: &'a SlotAllocator,
}

/// Grid binding: row-major gridcell ids map to session shadow W/D columns.
#[derive(Clone, Debug, PartialEq)]
pub struct TraversalFieldGridBinding {
    pub width: u32,
    pub height: u32,
    pub dest_x: u32,
    pub dest_y: u32,
    pub iterations: u32,
    pub w_global_col: usize,
    pub d_global_col: usize,
    pub gridcell_ids: Vec<SimThingId>,
}

impl TraversalFieldGridBinding {
    pub fn cells(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn validate(&self) -> Result<(), TraversalFieldBandError> {
        if self.width == 0 || self.height == 0 {
            return Err(MinPlusStencilError::InvalidDimensions {
                width: self.width,
                height: self.height,
            }
            .into());
        }
        if self.gridcell_ids.len() != self.cells() {
            return Err(TraversalFieldBandError::GridcellIdCount {
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
pub enum TraversalFieldBandError {
    #[error(transparent)]
    Scheduler(#[from] FieldSchedulerError),
    #[error(transparent)]
    Stencil(#[from] MinPlusStencilError),
    #[error("gridcell id count mismatch: expected {expected}, got {actual}")]
    GridcellIdCount { expected: usize, actual: usize },
    #[error("shadow buffer too short: need {required}, got {actual}")]
    ShadowTooShort { required: usize, actual: usize },
    #[error("traversal field band is disabled")]
    Disabled,
}

/// Opt-in traversal field band: [`FieldScheduler`] cadence + generic GPU utility.
pub struct TraversalFieldBandSession {
    enabled: bool,
    cadence_tick: u32,
    scheduler: FieldScheduler,
    binding: TraversalFieldGridBinding,
    op: Option<MinPlusTraversalFieldOp>,
    last_dispatch: Option<MinPlusTraversalDispatchReport>,
}

/// Result of one explicit traversal dispatch through the band scheduler.
#[derive(Clone, Debug, PartialEq)]
pub struct TraversalFieldDispatchReport {
    pub utility_id: &'static str,
    pub cadence_tick: u32,
    pub enabled: bool,
    pub scheduled: bool,
    pub execution_mode: MinPlusTraversalExecutionMode,
    pub w_input_kind: TraversalFieldWInputKind,
    pub dispatch: Option<MinPlusTraversalDispatchReport>,
    pub shadow_writeback: bool,
    pub scheduler_report: FieldSchedulerReport,
}

impl TraversalFieldBandSession {
    pub fn new(
        binding: TraversalFieldGridBinding,
        cadence: FieldCadence,
    ) -> Result<Self, TraversalFieldBandError> {
        binding.validate()?;
        cadence.validate()?;
        let mut scheduler = FieldScheduler::new();
        scheduler.register_field(FieldScheduleState {
            field_id: TRAVERSAL_FIELD_ID,
            cadence,
            event_pending: false,
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: TRAVERSAL_FIELD_REGION_ID,
            field_id: TRAVERSAL_FIELD_ID,
            dirty: crate::field_scheduler::DirtyRegionState::default(),
        });
        Ok(Self {
            enabled: TRAVERSAL_FIELD_BAND_DEFAULT_ENABLED,
            cadence_tick: 0,
            scheduler,
            binding,
            op: None,
            last_dispatch: None,
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

    pub fn binding(&self) -> &TraversalFieldGridBinding {
        &self.binding
    }

    pub fn scheduler(&self) -> &FieldScheduler {
        &self.scheduler
    }

    pub fn last_dispatch(&self) -> Option<&MinPlusTraversalDispatchReport> {
        self.last_dispatch.as_ref()
    }

    /// GPU-resident D output handle from the last scheduled dispatch, when present.
    pub fn resident_d_output(&self) -> Option<TraversalFieldGpuOutputHandle<'_>> {
        let report = self.last_dispatch.as_ref()?;
        let op = self.op.as_ref()?;
        Some(op.output_handle(report.iterations))
    }

    /// Production dispatch: GPU W input, GPU-resident D output, no CPU readback.
    pub fn dispatch_gpu_resident(
        &mut self,
        ctx: &GpuContext,
        input: TraversalFieldGpuInput<'_>,
    ) -> Result<TraversalFieldDispatchReport, TraversalFieldBandError> {
        self.dispatch_scheduled(
            ctx,
            ScheduledTraversalDispatch {
                gpu_input: Some(input.to_gpu_input()),
                shadow_compat: None,
                mode: MinPlusTraversalExecutionMode::GpuResident,
                w_oracle: None,
                shadow_writeback: false,
            },
        )
    }

    /// Diagnostic dispatch: GPU W input with explicit CPU D readback (no shadow writeback).
    pub fn dispatch_diagnostic_readback(
        &mut self,
        ctx: &GpuContext,
        input: TraversalFieldGpuInput<'_>,
    ) -> Result<TraversalFieldDispatchReport, TraversalFieldBandError> {
        self.dispatch_scheduled(
            ctx,
            ScheduledTraversalDispatch {
                gpu_input: Some(input.to_gpu_input()),
                shadow_compat: None,
                mode: MinPlusTraversalExecutionMode::DiagnosticReadback,
                w_oracle: None,
                shadow_writeback: false,
            },
        )
    }

    /// Diagnostic/compatibility dispatch: CPU shadow W gather + optional D shadow writeback.
    pub fn dispatch_shadow_column_compatibility(
        &mut self,
        ctx: &GpuContext,
        compat: TraversalFieldShadowColumnCompatInput<'_>,
        mode: MinPlusTraversalExecutionMode,
        shadow_writeback: bool,
    ) -> Result<TraversalFieldDispatchReport, TraversalFieldBandError> {
        self.dispatch_scheduled(
            ctx,
            ScheduledTraversalDispatch {
                gpu_input: None,
                shadow_compat: Some(compat),
                mode,
                w_oracle: None,
                shadow_writeback,
            },
        )
    }

    /// Oracle verification with GPU W input and explicit CPU W oracle slice.
    pub fn dispatch_oracle_verification_gpu(
        &mut self,
        ctx: &GpuContext,
        input: TraversalFieldGpuInput<'_>,
        w_oracle: &[f32],
    ) -> Result<TraversalFieldDispatchReport, TraversalFieldBandError> {
        self.dispatch_scheduled(
            ctx,
            ScheduledTraversalDispatch {
                gpu_input: Some(input.to_gpu_input()),
                shadow_compat: None,
                mode: MinPlusTraversalExecutionMode::OracleVerification,
                w_oracle: Some(w_oracle),
                shadow_writeback: false,
            },
        )
    }

    /// Oracle verification via explicit shadow-column compatibility gather.
    pub fn dispatch_oracle_verification_shadow_compat(
        &mut self,
        ctx: &GpuContext,
        compat: TraversalFieldShadowColumnCompatInput<'_>,
    ) -> Result<TraversalFieldDispatchReport, TraversalFieldBandError> {
        self.dispatch_scheduled(
            ctx,
            ScheduledTraversalDispatch {
                gpu_input: None,
                shadow_compat: Some(compat),
                mode: MinPlusTraversalExecutionMode::OracleVerification,
                w_oracle: None,
                shadow_writeback: false,
            },
        )
    }

    fn ensure_op(
        &mut self,
        ctx: &GpuContext,
    ) -> Result<&MinPlusTraversalFieldOp, TraversalFieldBandError> {
        if self.op.is_none() {
            self.op = Some(MinPlusTraversalFieldOp::new(
                ctx,
                self.binding.stencil_config(),
            )?);
        }
        Ok(self.op.as_ref().expect("op initialized"))
    }

    pub fn gather_w_from_shadow(
        shadow: &[f32],
        n_dims: usize,
        alloc: &SlotAllocator,
        binding: &TraversalFieldGridBinding,
    ) -> Result<Vec<f32>, TraversalFieldBandError> {
        binding.validate()?;
        let cells = binding.cells();
        let required = alloc.capacity() * n_dims;
        if shadow.len() < required {
            return Err(TraversalFieldBandError::ShadowTooShort {
                required,
                actual: shadow.len(),
            });
        }
        let mut w = Vec::with_capacity(cells);
        for cell_id in &binding.gridcell_ids {
            let slot = alloc
                .slot_of(*cell_id)
                .ok_or(TraversalFieldBandError::ShadowTooShort {
                    required,
                    actual: shadow.len(),
                })?
                .as_usize();
            w.push(shadow[slot * n_dims + binding.w_global_col]);
        }
        Ok(w)
    }

    pub fn scatter_d_to_shadow(
        shadow: &mut [f32],
        n_dims: usize,
        alloc: &SlotAllocator,
        binding: &TraversalFieldGridBinding,
        d: &[f32],
    ) -> Result<(), TraversalFieldBandError> {
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
            return Err(TraversalFieldBandError::ShadowTooShort {
                required,
                actual: shadow.len(),
            });
        }
        for (idx, cell_id) in binding.gridcell_ids.iter().enumerate() {
            let slot = alloc
                .slot_of(*cell_id)
                .ok_or(TraversalFieldBandError::ShadowTooShort {
                    required,
                    actual: shadow.len(),
                })?
                .as_usize();
            shadow[slot * n_dims + binding.d_global_col] = d[idx];
        }
        Ok(())
    }

    fn dispatch_scheduled(
        &mut self,
        ctx: &GpuContext,
        request: ScheduledTraversalDispatch<'_>,
    ) -> Result<TraversalFieldDispatchReport, TraversalFieldBandError> {
        let w_input_kind = match &request.gpu_input {
            Some(input) => match input {
                MinPlusTraversalInput::GpuFlatW { .. } => MinPlusTraversalWInputKind::GpuFlatW,
                MinPlusTraversalInput::GpuInterleavedW { .. } => {
                    MinPlusTraversalWInputKind::GpuInterleavedW
                }
                MinPlusTraversalInput::PackedCpuValues(_) => {
                    MinPlusTraversalWInputKind::PackedCpuValues
                }
            },
            None => MinPlusTraversalWInputKind::PackedCpuValues,
        };
        let mode = request.mode;
        let cadence_tick = self.cadence_tick;

        if !self.enabled {
            self.cadence_tick += 1;
            return Ok(empty_dispatch_report(
                cadence_tick,
                false,
                mode,
                w_input_kind,
            ));
        }

        let (decisions, scheduler_report) = self.scheduler.decide_tick(cadence_tick)?;
        let scheduled = decisions.iter().any(|d| {
            d.field_id == TRAVERSAL_FIELD_ID
                && d.region_id == TRAVERSAL_FIELD_REGION_ID
                && matches!(d.schedule, FieldDispatchSchedule::Dispatch)
        });

        let mut dispatch_report = None;
        let mut did_shadow_writeback = false;

        if scheduled {
            let config = self.binding.stencil_config();
            let iterations = self.binding.iterations;

            let mut packed_cpu: Option<Vec<f32>> = None;
            let mut w_oracle: Option<Vec<f32>> = request.w_oracle.map(|w| w.to_vec());
            let mut shadow_for_writeback: Option<(&mut [f32], usize, &SlotAllocator)> = None;

            let gpu_input = if let Some(input) = request.gpu_input {
                input
            } else if let Some(compat) = request.shadow_compat {
                let TraversalFieldShadowColumnCompatInput {
                    shadow,
                    n_dims,
                    alloc,
                } = compat;
                let w = Self::gather_w_from_shadow(shadow, n_dims, alloc, &self.binding)?;
                if mode == MinPlusTraversalExecutionMode::OracleVerification && w_oracle.is_none() {
                    w_oracle = Some(w.clone());
                }
                packed_cpu = Some(
                    simthing_gpu::pack_w_and_initial_d(&w, &config)
                        .map_err(TraversalFieldBandError::from)?,
                );
                shadow_for_writeback = Some((shadow, n_dims, alloc));
                MinPlusTraversalInput::PackedCpuValues(
                    packed_cpu.as_ref().expect("packed").as_slice(),
                )
            } else {
                return Err(TraversalFieldBandError::Stencil(
                    MinPlusStencilError::BufferTooShort {
                        actual: 0,
                        required: config.cells() as usize,
                    },
                ));
            };

            let op = self.ensure_op(ctx)?;
            let report = op.dispatch_traversal_from_input(
                ctx,
                gpu_input,
                w_oracle.as_deref(),
                MinPlusTraversalExecutionOptions { mode, iterations },
            )?;
            dispatch_report = Some(report.clone());
            self.last_dispatch = Some(report.clone());

            if request.shadow_writeback && report.diagnostic_readback {
                if let (Some(values), Some((shadow, n_dims, alloc))) =
                    (&report.values, shadow_for_writeback)
                {
                    let d = extract_d_flat(values, &config)?;
                    Self::scatter_d_to_shadow(shadow, n_dims, alloc, &self.binding, &d)?;
                    did_shadow_writeback = true;
                }
            }
        }

        self.cadence_tick += 1;
        Ok(TraversalFieldDispatchReport {
            utility_id: TRAVERSAL_FIELD_UTILITY_ID,
            cadence_tick,
            enabled: true,
            scheduled,
            execution_mode: mode,
            w_input_kind,
            dispatch: dispatch_report,
            shadow_writeback: did_shadow_writeback,
            scheduler_report,
        })
    }
}

struct ScheduledTraversalDispatch<'a> {
    gpu_input: Option<MinPlusTraversalInput<'a>>,
    shadow_compat: Option<TraversalFieldShadowColumnCompatInput<'a>>,
    mode: MinPlusTraversalExecutionMode,
    w_oracle: Option<&'a [f32]>,
    shadow_writeback: bool,
}

fn empty_dispatch_report(
    cadence_tick: u32,
    enabled: bool,
    mode: MinPlusTraversalExecutionMode,
    w_input_kind: TraversalFieldWInputKind,
) -> TraversalFieldDispatchReport {
    TraversalFieldDispatchReport {
        utility_id: TRAVERSAL_FIELD_UTILITY_ID,
        cadence_tick,
        enabled,
        scheduled: false,
        execution_mode: mode,
        w_input_kind,
        dispatch: None,
        shadow_writeback: false,
        scheduler_report: FieldSchedulerReport {
            total_regions: 0,
            scheduled_regions: 0,
            skipped_regions: 0,
            skip_ratio: 0.0,
            false_skip_count: 0,
        },
    }
}
