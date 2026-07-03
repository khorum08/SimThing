//! Generic structured 2D field stencil primitive (`StructuredFieldStencilOp`).
//!
//! Semantic-free tensor propagation over flat slot buffers. Not wired into the
//! production pass graph by default; callers opt in explicitly.

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor,
    PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::GpuContext;

pub const WORKGROUP_SIZE: u32 = 8;

/// Default tactical horizon cap (validated unless extended horizon is explicitly allowed).
pub const DEFAULT_HORIZON_CAP: u32 = 8;

/// Extended horizon cap when stability policy (source cap / decay) is authored.
pub const EXTENDED_HORIZON_CAP: u32 = 16;

const VARIANT_NORMALIZED: u32 = 1;
const VARIANT_DIRECTED: u32 = 2;
const VARIANT_SOURCE_CAPPED: u32 = 5;
const VARIANT_GRADIENT_XY: u32 = 6;
const VARIANT_SATURATING_FLUX: u32 = 7;

/// BH-0 CFL bound with dt fixed at 1.0.
pub const SATURATING_FLUX_CHI_CFL_MAX: f32 = 0.25;

const BOUNDARY_ZERO: u32 = 0;
const BOUNDARY_CLAMP: u32 = 1;

const DIRECTED_SE: u32 = 0;
const DIRECTED_NW: u32 = 1;

/// Stencil operator mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StructuredFieldStencilOperator {
    Normalized,
    SourceCappedNormalized,
    /// Optional; requires explicit orientation metadata at call sites.
    Directed {
        northwest: bool,
    },
    /// Single-target axis-X extraction: `(east − west) / 2` via per-direction weights.
    GradientX,
    /// Single-target axis-Y extraction: `(south − north) / 2` via per-direction weights.
    GradientY,
    /// Dual-output extraction in one dispatch: axis-X gradient → `target_col` (E/W weights),
    /// axis-Y gradient → `target_col_y` (N/S weights). The two output columns must differ
    /// (no-aliasing admission). Optimization of running `GradientX` then `GradientY`.
    GradientXY {
        target_col_y: u32,
    },
    /// BH-0: conservative saturating-flux operator with transient register-local C.
    /// BH-1: optional `choke_output_col` writes `1 − C(i)/χ` in the same dispatch.
    SaturatingFlux {
        u_sat: f32,
        chi: f32,
        choke_output_col: Option<u32>,
    },
}

/// Source injection policy between stencil hops.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructuredFieldStencilSourcePolicy {
    /// Validated safe default contract: caller seeds source cells once, clears `source_col`
    /// after the initial hop, then runs configured-horizon propagation. The primitive does
    /// not identify or zero source slots automatically.
    CallerManagedOneShotSeedThenZero,
}

/// Boundary sampling mode for out-of-grid neighbors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructuredFieldStencilBoundaryMode {
    Zero,
    Clamp,
}

/// Active-cell mask behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructuredFieldStencilMaskMode {
    /// Process all cells (default).
    All,
    /// Experimental: skip inactive cells. Halo/frontier semantics are **not**
    /// production-authorized; do not depend on this mode in production paths.
    ActiveOnlyExperimentalNoHalo,
}

/// Configuration for a structured field stencil run.
#[derive(Clone, Debug)]
pub struct StructuredFieldStencilConfig {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub source_col: u32,
    pub target_col: u32,
    pub horizon: u32,
    pub alpha_self: f32,
    /// Legacy scalar neighbor coefficient; isotropic operators derive equal per-direction weights as `gamma_neighbor / 4`.
    pub gamma_neighbor: f32,
    pub weight_north: f32,
    pub weight_south: f32,
    pub weight_east: f32,
    pub weight_west: f32,
    pub source_cap: Option<f32>,
    pub operator: StructuredFieldStencilOperator,
    pub source_policy: StructuredFieldStencilSourcePolicy,
    pub boundary_mode: StructuredFieldStencilBoundaryMode,
    pub mask_mode: StructuredFieldStencilMaskMode,
    /// Allow horizon in `(DEFAULT_HORIZON_CAP, EXTENDED_HORIZON_CAP]`.
    pub allow_extended_horizon: bool,
}

impl StructuredFieldStencilConfig {
    /// Per-direction weights default to zero; isotropic operators derive from `gamma_neighbor` at resolve time.
    pub fn zero_directional_weights() -> (f32, f32, f32, f32) {
        (0.0, 0.0, 0.0, 0.0)
    }

    pub fn cells(&self) -> u32 {
        self.width * self.height
    }

    pub fn values_len(&self) -> usize {
        (self.cells() * self.n_dims) as usize
    }

    /// Resolve per-direction weights for GPU/CPU oracle (operator + explicit weights + legacy gamma).
    pub fn resolved_directional_weights(&self) -> (f32, f32, f32, f32) {
        if matches!(
            self.operator,
            StructuredFieldStencilOperator::GradientX
                | StructuredFieldStencilOperator::GradientY
                | StructuredFieldStencilOperator::GradientXY { .. }
        ) || (self.weight_north != 0.0
            || self.weight_south != 0.0
            || self.weight_east != 0.0
            || self.weight_west != 0.0)
        {
            return (
                self.weight_north,
                self.weight_south,
                self.weight_east,
                self.weight_west,
            );
        }
        match self.operator {
            StructuredFieldStencilOperator::Directed { northwest } => {
                if northwest {
                    (
                        self.gamma_neighbor / 2.0,
                        0.0,
                        0.0,
                        self.gamma_neighbor / 2.0,
                    )
                } else {
                    (
                        0.0,
                        self.gamma_neighbor / 2.0,
                        self.gamma_neighbor / 2.0,
                        0.0,
                    )
                }
            }
            StructuredFieldStencilOperator::GradientX => (0.0, 0.0, 0.5, -0.5),
            StructuredFieldStencilOperator::GradientY => (-0.5, 0.5, 0.0, 0.0),
            StructuredFieldStencilOperator::GradientXY { .. } => (-0.5, 0.5, 0.5, -0.5),
            StructuredFieldStencilOperator::Normalized
            | StructuredFieldStencilOperator::SourceCappedNormalized
            | StructuredFieldStencilOperator::SaturatingFlux { .. } => {
                let w = self.gamma_neighbor / 4.0;
                (w, w, w, w)
            }
        }
    }

    pub fn validate(&self) -> Result<(), StructuredFieldStencilError> {
        if self.width == 0 || self.height == 0 {
            return Err(StructuredFieldStencilError::InvalidDimensions {
                width: self.width,
                height: self.height,
            });
        }
        if self.n_dims == 0 {
            return Err(StructuredFieldStencilError::InvalidDims(self.n_dims));
        }
        if self.source_col >= self.n_dims || self.target_col >= self.n_dims {
            return Err(StructuredFieldStencilError::InvalidColumn {
                source_col: self.source_col,
                target_col: self.target_col,
                n_dims: self.n_dims,
            });
        }
        if self.horizon < 1 {
            return Err(StructuredFieldStencilError::InvalidHorizon(self.horizon));
        }
        if self.horizon > DEFAULT_HORIZON_CAP && !self.allow_extended_horizon {
            return Err(StructuredFieldStencilError::HorizonCapExceeded {
                horizon: self.horizon,
                cap: DEFAULT_HORIZON_CAP,
            });
        }
        if self.horizon > EXTENDED_HORIZON_CAP {
            return Err(StructuredFieldStencilError::HorizonCapExceeded {
                horizon: self.horizon,
                cap: EXTENDED_HORIZON_CAP,
            });
        }
        if !self.alpha_self.is_finite() {
            return Err(StructuredFieldStencilError::NonFiniteCoefficients);
        }
        if !matches!(
            self.operator,
            StructuredFieldStencilOperator::GradientX
                | StructuredFieldStencilOperator::GradientY
                | StructuredFieldStencilOperator::GradientXY { .. }
                | StructuredFieldStencilOperator::SaturatingFlux { .. }
        ) && !self.gamma_neighbor.is_finite()
        {
            return Err(StructuredFieldStencilError::NonFiniteCoefficients);
        }
        if let StructuredFieldStencilOperator::SaturatingFlux {
            u_sat,
            chi,
            choke_output_col,
        } = self.operator
        {
            if !u_sat.is_finite() || u_sat <= 0.0 {
                return Err(StructuredFieldStencilError::InvalidUSat(u_sat));
            }
            if !chi.is_finite() || chi <= 0.0 {
                return Err(StructuredFieldStencilError::InvalidChi(chi));
            }
            if chi > SATURATING_FLUX_CHI_CFL_MAX {
                return Err(StructuredFieldStencilError::ChiExceedsCflBound {
                    chi,
                    max: SATURATING_FLUX_CHI_CFL_MAX,
                });
            }
            if self.source_col != self.target_col {
                return Err(StructuredFieldStencilError::SaturatingFluxColumnMismatch {
                    source_col: self.source_col,
                    target_col: self.target_col,
                });
            }
            if self.source_cap.is_some() {
                return Err(StructuredFieldStencilError::SourceCapRequiresOperator);
            }
            if let Some(choke_col) = choke_output_col {
                if choke_col >= self.n_dims {
                    return Err(StructuredFieldStencilError::ChokeOutputColOutOfRange {
                        choke_output_col: choke_col,
                        n_dims: self.n_dims,
                    });
                }
                if choke_col == self.source_col {
                    return Err(StructuredFieldStencilError::ChokeOutputColAliasedSource {
                        choke_output_col: choke_col,
                        source_col: self.source_col,
                    });
                }
            }
        }
        if let StructuredFieldStencilOperator::GradientXY { target_col_y } = self.operator {
            if target_col_y >= self.n_dims {
                return Err(StructuredFieldStencilError::GradientXyTargetYOutOfRange {
                    target_col_y,
                    n_dims: self.n_dims,
                });
            }
            if target_col_y == self.target_col {
                return Err(StructuredFieldStencilError::GradientXyAliasedOutputs {
                    target_col: self.target_col,
                    target_col_y,
                });
            }
        }
        for w in [
            self.weight_north,
            self.weight_south,
            self.weight_east,
            self.weight_west,
        ] {
            if !w.is_finite() {
                return Err(StructuredFieldStencilError::NonFiniteCoefficients);
            }
        }
        if let Some(cap) = self.source_cap {
            if !cap.is_finite() || cap <= 0.0 {
                return Err(StructuredFieldStencilError::InvalidSourceCap(cap));
            }
            if !matches!(
                self.operator,
                StructuredFieldStencilOperator::SourceCappedNormalized
            ) {
                return Err(StructuredFieldStencilError::SourceCapRequiresOperator);
            }
        }
        if matches!(
            self.operator,
            StructuredFieldStencilOperator::SourceCappedNormalized
        ) && self.source_cap.is_none()
        {
            return Err(StructuredFieldStencilError::MissingSourceCap);
        }
        Ok(())
    }

    /// Validate execution step count against this config's horizon and global caps.
    pub fn validate_execution_steps(&self, steps: u32) -> Result<(), StructuredFieldStencilError> {
        if steps < 1 {
            return Err(StructuredFieldStencilError::InvalidHorizon(steps));
        }
        if steps > self.horizon {
            return Err(StructuredFieldStencilError::ExecutionHorizonExceedsConfig {
                steps,
                horizon: self.horizon,
            });
        }
        if steps > DEFAULT_HORIZON_CAP && !self.allow_extended_horizon {
            return Err(StructuredFieldStencilError::HorizonCapExceeded {
                horizon: steps,
                cap: DEFAULT_HORIZON_CAP,
            });
        }
        if steps > EXTENDED_HORIZON_CAP {
            return Err(StructuredFieldStencilError::HorizonCapExceeded {
                horizon: steps,
                cap: EXTENDED_HORIZON_CAP,
            });
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum StructuredFieldStencilError {
    #[error("invalid grid dimensions {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    #[error("n_dims must be > 0 (got {0})")]
    InvalidDims(u32),
    #[error("column out of range: source={source_col} target={target_col} n_dims={n_dims}")]
    InvalidColumn {
        source_col: u32,
        target_col: u32,
        n_dims: u32,
    },
    #[error("horizon must be >= 1 (got {0})")]
    InvalidHorizon(u32),
    #[error("horizon {horizon} exceeds cap {cap}")]
    HorizonCapExceeded { horizon: u32, cap: u32 },
    #[error("alpha/gamma must be finite")]
    NonFiniteCoefficients,
    #[error("invalid source_cap {0}")]
    InvalidSourceCap(f32),
    #[error("source_cap requires SourceCappedNormalized operator")]
    SourceCapRequiresOperator,
    #[error("SourceCappedNormalized requires source_cap")]
    MissingSourceCap,
    #[error("values buffer length {actual} < required {required}")]
    BufferTooShort { actual: usize, required: usize },
    #[error("execution steps {steps} exceed configured horizon {horizon}")]
    ExecutionHorizonExceedsConfig { steps: u32, horizon: u32 },
    #[error("GradientXY target_col_y {target_col_y} out of range (n_dims={n_dims})")]
    GradientXyTargetYOutOfRange { target_col_y: u32, n_dims: u32 },
    #[error("GradientXY output columns must differ (no aliasing): target_col={target_col} target_col_y={target_col_y}")]
    GradientXyAliasedOutputs { target_col: u32, target_col_y: u32 },
    #[error("SaturatingFlux u_sat must be finite and > 0 (got {0})")]
    InvalidUSat(f32),
    #[error("SaturatingFlux chi must be finite and > 0 (got {0})")]
    InvalidChi(f32),
    #[error("SaturatingFlux chi {chi} exceeds CFL bound {max} (dt=1.0)")]
    ChiExceedsCflBound { chi: f32, max: f32 },
    #[error("SaturatingFlux requires source_col == target_col (source={source_col} target={target_col})")]
    SaturatingFluxColumnMismatch { source_col: u32, target_col: u32 },
    #[error("SaturatingFlux choke_output_col {choke_output_col} out of range for n_dims {n_dims}")]
    ChokeOutputColOutOfRange { choke_output_col: u32, n_dims: u32 },
    #[error(
        "SaturatingFlux choke_output_col {choke_output_col} must differ from source_col {source_col}"
    )]
    ChokeOutputColAliasedSource {
        choke_output_col: u32,
        source_col: u32,
    },
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct FieldStencilParamsGpu {
    width: u32,
    height: u32,
    n_dims: u32,
    source_col: u32,
    target_col: u32,
    alpha_self_decay: f32,
    weight_north: f32,
    weight_south: f32,
    weight_east: f32,
    weight_west: f32,
    cap: f32,
    source_cap: f32,
    boundary_mode: u32,
    variant: u32,
    directed_mode: u32,
    use_active_mask: u32,
    target_col_y: u32,
    u_sat: f32,
    chi: f32,
    choke_output_enabled: u32,
}

impl FieldStencilParamsGpu {
    pub fn from_config(config: &StructuredFieldStencilConfig) -> Self {
        let (weight_north, weight_south, weight_east, weight_west) =
            config.resolved_directional_weights();
        let variant = match config.operator {
            StructuredFieldStencilOperator::Normalized => VARIANT_NORMALIZED,
            StructuredFieldStencilOperator::SourceCappedNormalized => VARIANT_SOURCE_CAPPED,
            StructuredFieldStencilOperator::Directed { .. } => VARIANT_DIRECTED,
            StructuredFieldStencilOperator::GradientX
            | StructuredFieldStencilOperator::GradientY => VARIANT_NORMALIZED,
            StructuredFieldStencilOperator::GradientXY { .. } => VARIANT_GRADIENT_XY,
            StructuredFieldStencilOperator::SaturatingFlux { .. } => VARIANT_SATURATING_FLUX,
        };
        let directed_mode = match config.operator {
            StructuredFieldStencilOperator::Directed { northwest } => {
                if northwest {
                    DIRECTED_NW
                } else {
                    DIRECTED_SE
                }
            }
            _ => DIRECTED_SE,
        };
        Self {
            width: config.width,
            height: config.height,
            n_dims: config.n_dims,
            source_col: config.source_col,
            target_col: config.target_col,
            alpha_self_decay: config.alpha_self,
            weight_north,
            weight_south,
            weight_east,
            weight_west,
            cap: 0.0,
            source_cap: if matches!(
                config.operator,
                StructuredFieldStencilOperator::SourceCappedNormalized
            ) {
                config.source_cap.unwrap_or(0.0)
            } else {
                0.0
            },
            boundary_mode: match config.boundary_mode {
                StructuredFieldStencilBoundaryMode::Zero => BOUNDARY_ZERO,
                StructuredFieldStencilBoundaryMode::Clamp => BOUNDARY_CLAMP,
            },
            variant,
            directed_mode,
            use_active_mask: u32::from(matches!(
                config.mask_mode,
                StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo
            )),
            target_col_y: match config.operator {
                StructuredFieldStencilOperator::GradientXY { target_col_y } => target_col_y,
                StructuredFieldStencilOperator::SaturatingFlux {
                    choke_output_col: Some(col),
                    ..
                } => col,
                _ => 0,
            },
            u_sat: match config.operator {
                StructuredFieldStencilOperator::SaturatingFlux { u_sat, .. } => u_sat,
                _ => 0.0,
            },
            chi: match config.operator {
                StructuredFieldStencilOperator::SaturatingFlux { chi, .. } => chi,
                _ => 0.0,
            },
            choke_output_enabled: match config.operator {
                StructuredFieldStencilOperator::SaturatingFlux {
                    choke_output_col: Some(_),
                    ..
                } => 1,
                _ => 0,
            },
        }
    }
}

/// GPU structured field stencil session with ping-pong buffers.
pub struct StructuredFieldStencilOp {
    params_buffer: Buffer,
    pub input_buffer: Buffer,
    pub output_buffer: Buffer,
    mask_buffer: Buffer,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    config: StructuredFieldStencilConfig,
    params: FieldStencilParamsGpu,
}

impl StructuredFieldStencilOp {
    pub fn new(
        ctx: &GpuContext,
        config: StructuredFieldStencilConfig,
    ) -> Result<Self, StructuredFieldStencilError> {
        config.validate()?;
        let params = FieldStencilParamsGpu::from_config(&config);
        let device = &ctx.device;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("structured_field_stencil"),
            source: ShaderSource::Wgsl(
                include_str!("shaders/structured_field_stencil.wgsl").into(),
            ),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("structured_field_stencil_bgl"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("structured_field_stencil"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("structured_field_stencil_pl"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "stencil_step",
            compilation_options: Default::default(),
            cache: None,
        });

        let len = config.values_len();
        let cells = config.cells() as usize;
        let bytes = (len * std::mem::size_of::<f32>()) as u64;

        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("structured_field_stencil_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let input_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("structured_field_stencil_input"),
            size: bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("structured_field_stencil_output"),
            size: bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let mask_init = vec![1u32; cells];
        let mask_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("structured_field_stencil_mask"),
            contents: bytemuck::cast_slice(&mask_init),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        Ok(Self {
            params_buffer,
            input_buffer,
            output_buffer,
            mask_buffer,
            pipeline,
            bind_group_layout,
            config,
            params,
        })
    }

    pub fn config(&self) -> &StructuredFieldStencilConfig {
        &self.config
    }

    /// Validate execution step count against configured horizon and global caps.
    pub fn validate_execution_steps(&self, steps: u32) -> Result<(), StructuredFieldStencilError> {
        self.config.validate_execution_steps(steps)
    }

    pub fn upload_values(
        &self,
        ctx: &GpuContext,
        values: &[f32],
    ) -> Result<(), StructuredFieldStencilError> {
        let required = self.config.values_len();
        if values.len() < required {
            return Err(StructuredFieldStencilError::BufferTooShort {
                actual: values.len(),
                required,
            });
        }
        ctx.queue.write_buffer(
            &self.input_buffer,
            0,
            bytemuck::cast_slice(&values[..required]),
        );
        Ok(())
    }

    pub fn upload_mask(
        &self,
        ctx: &GpuContext,
        mask: &[u32],
    ) -> Result<(), StructuredFieldStencilError> {
        if mask.len() != self.config.cells() as usize {
            return Err(StructuredFieldStencilError::BufferTooShort {
                actual: mask.len(),
                required: self.config.cells() as usize,
            });
        }
        ctx.queue
            .write_buffer(&self.mask_buffer, 0, bytemuck::cast_slice(mask));
        Ok(())
    }

    fn values_byte_len(&self) -> u64 {
        (self.config.values_len() * std::mem::size_of::<f32>()) as u64
    }

    fn cell_byte_offset(&self, slot: u32, col: u32) -> u64 {
        ((slot * self.config.n_dims + col) * std::mem::size_of::<f32>() as u32) as u64
    }

    /// Copy an entire values buffer (`src` → `dst`).
    pub fn copy_values_buffer(&self, ctx: &GpuContext, src: &Buffer, dst: &Buffer) {
        let bytes = self.values_byte_len();
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("structured_field_stencil_copy"),
            });
        encoder.copy_buffer_to_buffer(src, 0, dst, 0, bytes);
        ctx.queue.submit(Some(encoder.finish()));
    }

    /// Copy output → input (after a single input→output dispatch).
    pub fn copy_output_to_input(&self, ctx: &GpuContext) {
        self.copy_values_buffer(ctx, &self.output_buffer, &self.input_buffer);
    }

    /// After `steps` ping-pong dispatches starting from input, ensure canonical state lives in input.
    pub fn canonicalize_input_after_ping_pong(&self, ctx: &GpuContext, steps: u32) {
        if steps % 2 == 1 {
            self.copy_output_to_input(ctx);
        }
    }

    /// Write specific `(slot, col)` values into a values buffer via queue writes.
    pub fn write_cell_values(
        &self,
        ctx: &GpuContext,
        buffer: &Buffer,
        writes: &[(u32, u32, f32)],
    ) -> Result<(), StructuredFieldStencilError> {
        for &(slot, col, value) in writes {
            if slot >= self.config.cells() || col >= self.config.n_dims {
                return Err(StructuredFieldStencilError::BufferTooShort {
                    actual: slot as usize,
                    required: self.config.cells() as usize,
                });
            }
            let offset = self.cell_byte_offset(slot, col);
            ctx.queue
                .write_buffer(buffer, offset, bytemuck::bytes_of(&value));
        }
        Ok(())
    }

    /// Zero specific `(slot, col)` entries in a values buffer via queue writes.
    pub fn zero_cell_values(
        &self,
        ctx: &GpuContext,
        buffer: &Buffer,
        cells: &[(u32, u32)],
    ) -> Result<(), StructuredFieldStencilError> {
        let zero = 0.0f32;
        for &(slot, col) in cells {
            if slot >= self.config.cells() || col >= self.config.n_dims {
                return Err(StructuredFieldStencilError::BufferTooShort {
                    actual: slot as usize,
                    required: self.config.cells() as usize,
                });
            }
            let offset = self.cell_byte_offset(slot, col);
            ctx.queue
                .write_buffer(buffer, offset, bytemuck::bytes_of(&zero));
        }
        Ok(())
    }

    /// Read back the canonical input buffer (current field state when canonicalized).
    pub fn readback_input_buffer(&self, ctx: &GpuContext) -> Vec<f32> {
        self.readback_buffer(ctx, &self.input_buffer)
    }

    pub fn set_mask_mode(
        &mut self,
        ctx: &GpuContext,
        mode: StructuredFieldStencilMaskMode,
    ) -> Result<(), StructuredFieldStencilError> {
        self.config.mask_mode = mode;
        self.params.use_active_mask = u32::from(matches!(
            mode,
            StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo
        ));
        ctx.queue
            .write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(&self.params));
        Ok(())
    }

    fn bind_group(
        &self,
        device: &wgpu::Device,
        input: &Buffer,
        output: &Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("structured_field_stencil_bg"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: input.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: output.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: self.mask_buffer.as_entire_binding(),
                },
            ],
        })
    }

    pub fn dispatch_once(&self, ctx: &GpuContext, input: &Buffer, output: &Buffer) -> u32 {
        let device = &ctx.device;
        let queue = &ctx.queue;
        let bg = self.bind_group(device, input, output);
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("structured_field_stencil_dispatch"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("structured_field_stencil_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(
                self.config.width.div_ceil(WORKGROUP_SIZE),
                self.config.height.div_ceil(WORKGROUP_SIZE),
                1,
            );
        }
        queue.submit(Some(encoder.finish()));
        1
    }

    /// Ping-pong dispatch for `steps` stencil hops (required for H > 1).
    pub fn dispatch_ping_pong(
        &self,
        ctx: &GpuContext,
        steps: u32,
    ) -> Result<u32, StructuredFieldStencilError> {
        self.validate_execution_steps(steps)?;
        let mut dispatches = 0u32;
        let mut read_input = true;
        for _ in 0..steps {
            if read_input {
                dispatches += self.dispatch_once(ctx, &self.input_buffer, &self.output_buffer);
            } else {
                dispatches += self.dispatch_once(ctx, &self.output_buffer, &self.input_buffer);
            }
            read_input = !read_input;
        }
        Ok(dispatches)
    }

    pub fn readback_buffer(&self, ctx: &GpuContext, src: &Buffer) -> Vec<f32> {
        let device = &ctx.device;
        let queue = &ctx.queue;
        let len = self.config.values_len();
        let bytes = (len * std::mem::size_of::<f32>()) as u64;
        let staging = device.create_buffer(&BufferDescriptor {
            label: Some("structured_field_stencil_readback"),
            size: bytes,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("structured_field_stencil_readback_enc"),
        });
        encoder.copy_buffer_to_buffer(src, 0, &staging, 0, bytes);
        queue.submit(Some(encoder.finish()));
        let slice = staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let data = slice.get_mapped_range();
        let out = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        staging.unmap();
        out
    }

    pub fn readback_after_ping_pong(&self, ctx: &GpuContext, steps: u32) -> Vec<f32> {
        let src = if steps % 2 == 1 {
            &self.output_buffer
        } else {
            &self.input_buffer
        };
        self.readback_buffer(ctx, src)
    }

    pub fn run_ping_pong(
        &self,
        ctx: &GpuContext,
        steps: u32,
    ) -> Result<(Vec<f32>, u32), StructuredFieldStencilError> {
        let dispatches = self.dispatch_ping_pong(ctx, steps)?;
        Ok((self.readback_after_ping_pong(ctx, steps), dispatches))
    }

    /// Run exactly `config.horizon` ping-pong hops (cannot bypass configured horizon).
    pub fn run_configured_horizon(
        &self,
        ctx: &GpuContext,
    ) -> Result<(Vec<f32>, u32), StructuredFieldStencilError> {
        self.run_ping_pong(ctx, self.config.horizon)
    }

    /// Execute the configured structured field stencil and return a generic report.
    ///
    /// Uses [`Self::run_configured_horizon`] unless `options.steps` is set; optional
    /// step overrides must not exceed `config.horizon`. By default dispatches without
    /// CPU readback; set `readback_values` or `collect_field_stats` for explicit readback.
    pub fn execute_configured(
        &self,
        ctx: &GpuContext,
        options: StructuredFieldExecutionOptions,
    ) -> Result<StructuredFieldExecutionReport, StructuredFieldStencilError> {
        let steps = options.steps.unwrap_or(self.config.horizon);
        if let Some(requested) = options.steps {
            if requested > self.config.horizon {
                return Err(StructuredFieldStencilError::ExecutionHorizonExceedsConfig {
                    steps: requested,
                    horizon: self.config.horizon,
                });
            }
        }
        self.validate_execution_steps(steps)?;
        let dispatches = self.dispatch_ping_pong(ctx, steps)?;
        let mut debug =
            StructuredFieldStencilDebugReport::from_run(&self.config, steps, dispatches);

        let want_readback = options.readback_values || options.collect_field_stats;
        let values = if want_readback {
            Some(self.readback_after_ping_pong(ctx, steps))
        } else {
            None
        };

        if options.collect_field_stats {
            let buf = values
                .as_ref()
                .expect("collect_field_stats requires values readback");
            debug.apply_field_stats(buf, &self.config);
            debug.active_mask_ratio = Some(match self.config.mask_mode {
                StructuredFieldStencilMaskMode::All => 1.0,
                StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo => {
                    self.readback_active_mask_ratio(ctx)?
                }
            });
        }

        Ok(StructuredFieldExecutionReport { values, debug })
    }

    fn readback_active_mask_ratio(
        &self,
        ctx: &GpuContext,
    ) -> Result<f32, StructuredFieldStencilError> {
        let device = &ctx.device;
        let queue = &ctx.queue;
        let cells = self.config.cells() as usize;
        let bytes = (cells * std::mem::size_of::<u32>()) as u64;
        let staging = device.create_buffer(&BufferDescriptor {
            label: Some("structured_field_stencil_mask_readback"),
            size: bytes,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("structured_field_stencil_mask_readback_enc"),
        });
        encoder.copy_buffer_to_buffer(&self.mask_buffer, 0, &staging, 0, bytes);
        queue.submit(Some(encoder.finish()));
        let slice = staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let data = slice.get_mapped_range();
        let mask: &[u32] = bytemuck::cast_slice(&data);
        let active = mask.iter().filter(|&&v| v != 0).count();
        drop(data);
        staging.unmap();
        Ok(active as f32 / cells as f32)
    }
}

/// Options for [`StructuredFieldStencilOp::execute_configured`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructuredFieldExecutionOptions {
    /// When true, read back target-column max/L1 and active-mask ratio (requires values readback).
    pub collect_field_stats: bool,
    /// When true, read back the full values buffer after dispatch. Default is GPU-resident dispatch only.
    pub readback_values: bool,
    /// Optional step override. Must be `<= config.horizon` when set.
    pub steps: Option<u32>,
}

impl Default for StructuredFieldExecutionOptions {
    fn default() -> Self {
        Self {
            collect_field_stats: false,
            readback_values: false,
            steps: None,
        }
    }
}

/// Result of a configured structured-field execution.
#[derive(Clone, Debug, PartialEq)]
pub struct StructuredFieldExecutionReport {
    /// Present only when readback was requested (`readback_values` or `collect_field_stats`).
    pub values: Option<Vec<f32>>,
    pub debug: StructuredFieldStencilDebugReport,
}

/// Generic debug/observability surface for structured field stencil runs.
#[derive(Clone, Debug, PartialEq)]
pub struct StructuredFieldStencilDebugReport {
    pub dispatch_count: u32,
    pub configured_horizon: u32,
    pub executed_horizon: u32,
    pub operator: StructuredFieldStencilOperator,
    pub source_policy: StructuredFieldStencilSourcePolicy,
    pub boundary_mode: StructuredFieldStencilBoundaryMode,
    pub mask_mode: StructuredFieldStencilMaskMode,
    pub cell_count: u32,
    pub values_len: usize,
    pub field_max: Option<f32>,
    pub field_l1_norm: Option<f32>,
    pub active_mask_ratio: Option<f32>,
}

impl StructuredFieldStencilDebugReport {
    fn from_run(
        config: &StructuredFieldStencilConfig,
        executed_horizon: u32,
        dispatch_count: u32,
    ) -> Self {
        Self {
            dispatch_count,
            configured_horizon: config.horizon,
            executed_horizon,
            operator: config.operator,
            source_policy: config.source_policy,
            boundary_mode: config.boundary_mode,
            mask_mode: config.mask_mode,
            cell_count: config.cells(),
            values_len: config.values_len(),
            field_max: None,
            field_l1_norm: None,
            active_mask_ratio: None,
        }
    }

    fn apply_field_stats(&mut self, values: &[f32], config: &StructuredFieldStencilConfig) {
        let col = config.target_col;
        let nd = config.n_dims;
        let mut max_v = 0.0f32;
        let mut l1 = 0.0f32;
        for slot in 0..config.cells() {
            let v = values[(slot * nd + col) as usize];
            if v.is_finite() {
                max_v = max_v.max(v);
                l1 += v.abs();
            }
        }
        self.field_max = Some(max_v);
        self.field_l1_norm = Some(l1);
    }
}

/// CPU oracle for parity tests (normalized / source-capped / directed / saturating-flux modes).
pub fn cpu_stencil_step(values: &[f32], params: &FieldStencilParamsGpu) -> Vec<f32> {
    if params.variant == VARIANT_SATURATING_FLUX {
        return cpu_saturating_flux_step(values, params);
    }
    let mut out = values.to_vec();
    let w = params.width;
    let h = params.height;
    let nd = params.n_dims;
    let sc = params.source_col;
    let tc = params.target_col;

    let sample = |buf: &[f32], x: i32, y: i32| -> f32 {
        let (sx, sy) = if params.boundary_mode == BOUNDARY_CLAMP {
            (x.clamp(0, w as i32 - 1), y.clamp(0, h as i32 - 1))
        } else if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
            return 0.0;
        } else {
            (x, y)
        };
        let idx = sy as u32 * w + sx as u32;
        buf[(idx * nd + sc) as usize]
    };

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let ix = x as i32;
            let iy = y as i32;
            let center = sample(values, ix, iy);
            let north = sample(values, ix, iy - 1);
            let south = sample(values, ix, iy + 1);
            let west = sample(values, ix - 1, iy);
            let east = sample(values, ix + 1, iy);

            if params.variant == VARIANT_GRADIENT_XY {
                // Dual-output: axis-X gradient (E/W weights) → tc, axis-Y gradient (N/S) → tc_y.
                let gx = params.weight_east * east + params.weight_west * west;
                let gy = params.weight_north * north + params.weight_south * south;
                out[(idx * nd + tc) as usize] = gx;
                out[(idx * nd + params.target_col_y) as usize] = gy;
                continue;
            }

            let mut next = params.alpha_self_decay * center
                + params.weight_north * north
                + params.weight_south * south
                + params.weight_east * east
                + params.weight_west * west;

            if params.variant == VARIANT_SOURCE_CAPPED && params.source_cap > 0.0 {
                next = next.clamp(0.0, params.source_cap);
            }

            out[(idx * nd + tc) as usize] = next;
        }
    }
    out
}

fn sigma_u(u: f32, u_sat: f32) -> f32 {
    let x = u / u_sat;
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

fn read_u_source(values: &[f32], params: &FieldStencilParamsGpu, x: i32, y: i32) -> f32 {
    let idx = y as u32 * params.width + x as u32;
    values[(idx * params.n_dims + params.source_col) as usize]
}

/// Compute transient C at `(x, y)` for oracle introspection (BH-0 tests).
pub fn cpu_compute_c_at(values: &[f32], params: &FieldStencilParamsGpu, x: i32, y: i32) -> f32 {
    let w = params.width as i32;
    let h = params.height as i32;
    let u_sat = params.u_sat;
    let mut c = params.chi;

    if y - 1 >= 0 && y - 1 < h && x >= 0 && x < w {
        let u = read_u_source(values, params, x, y - 1);
        c *= 1.0 - sigma_u(u, u_sat);
    }
    if y + 1 >= 0 && y + 1 < h && x >= 0 && x < w {
        let u = read_u_source(values, params, x, y + 1);
        c *= 1.0 - sigma_u(u, u_sat);
    }
    if x + 1 >= 0 && x + 1 < w && y >= 0 && y < h {
        let u = read_u_source(values, params, x + 1, y);
        c *= 1.0 - sigma_u(u, u_sat);
    }
    if x - 1 >= 0 && x - 1 < w && y >= 0 && y < h {
        let u = read_u_source(values, params, x - 1, y);
        c *= 1.0 - sigma_u(u, u_sat);
    }
    c
}

/// BH-1: choke readout `1 − C/χ`, clamped to `[0, 1]`.
pub fn cpu_compute_choke_at(c: f32, chi: f32) -> f32 {
    let mut choke = 1.0 - c / chi;
    if choke < 0.0 {
        choke = 0.0;
    } else if choke > 1.0 {
        choke = 1.0;
    }
    choke
}

/// BH-1: choke readout at `(x, y)` for oracle introspection (tests only).
pub fn cpu_compute_choke_readout_at(
    values: &[f32],
    params: &FieldStencilParamsGpu,
    x: i32,
    y: i32,
) -> f32 {
    cpu_compute_choke_at(cpu_compute_c_at(values, params, x, y), params.chi)
}

fn cpu_saturating_flux_step(values: &[f32], params: &FieldStencilParamsGpu) -> Vec<f32> {
    let mut out = values.to_vec();
    let w = params.width;
    let h = params.height;
    let nd = params.n_dims;
    let tc = params.target_col;

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let ix = x as i32;
            let iy = y as i32;
            let u_i = read_u_source(values, params, ix, iy);
            let c_i = cpu_compute_c_at(values, params, ix, iy);
            let mut next = u_i;

            if iy - 1 >= 0 {
                let c_n = cpu_compute_c_at(values, params, ix, iy - 1);
                let u_n = read_u_source(values, params, ix, iy - 1);
                next += ((c_i + c_n) * 0.5) * (u_n - u_i);
            }
            if iy + 1 < h as i32 {
                let c_s = cpu_compute_c_at(values, params, ix, iy + 1);
                let u_s = read_u_source(values, params, ix, iy + 1);
                next += ((c_i + c_s) * 0.5) * (u_s - u_i);
            }
            if ix + 1 < w as i32 {
                let c_e = cpu_compute_c_at(values, params, ix + 1, iy);
                let u_e = read_u_source(values, params, ix + 1, iy);
                next += ((c_i + c_e) * 0.5) * (u_e - u_i);
            }
            if ix - 1 >= 0 {
                let c_w = cpu_compute_c_at(values, params, ix - 1, iy);
                let u_w = read_u_source(values, params, ix - 1, iy);
                next += ((c_i + c_w) * 0.5) * (u_w - u_i);
            }

            out[(idx * nd + tc) as usize] = next;
            if params.choke_output_enabled != 0 {
                let choke = cpu_compute_choke_at(c_i, params.chi);
                out[(idx * nd + params.target_col_y) as usize] = choke;
            }
        }
    }
    out
}

pub fn cpu_horizon(values: &[f32], params: &FieldStencilParamsGpu, hops: u32) -> Vec<f32> {
    let mut cur = values.to_vec();
    for _ in 0..hops {
        cur = cpu_stencil_step(&cur, params);
    }
    cur
}

pub fn params_from_config(config: &StructuredFieldStencilConfig) -> FieldStencilParamsGpu {
    FieldStencilParamsGpu::from_config(config)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn base_config() -> StructuredFieldStencilConfig {
        StructuredFieldStencilConfig {
            width: 3,
            height: 3,
            n_dims: 4,
            source_col: 0,
            target_col: 0,
            horizon: 1,
            alpha_self: 0.8,
            gamma_neighbor: 0.16,
            weight_north: 0.0,
            weight_south: 0.0,
            weight_east: 0.0,
            weight_west: 0.0,
            source_cap: None,
            operator: StructuredFieldStencilOperator::Normalized,
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        }
    }

    #[test]
    fn extended_horizon_allows_h16_with_flag() {
        let mut c = base_config();
        c.horizon = 16;
        c.allow_extended_horizon = true;
        assert!(c.validate().is_ok());
    }

    #[test]
    fn source_capped_requires_cap() {
        let mut c = base_config();
        c.operator = StructuredFieldStencilOperator::SourceCappedNormalized;
        assert_eq!(
            c.validate().unwrap_err(),
            StructuredFieldStencilError::MissingSourceCap
        );
    }

    #[test]
    fn debug_report_skips_stats_by_default() {
        let config = base_config();
        let report = StructuredFieldStencilDebugReport::from_run(&config, 1, 1);
        assert_eq!(report.configured_horizon, 1);
        assert_eq!(report.executed_horizon, 1);
        assert_eq!(report.dispatch_count, 1);
        assert!(report.field_max.is_none());
        assert!(report.field_l1_norm.is_none());
        assert!(report.active_mask_ratio.is_none());
    }

    #[test]
    fn debug_report_field_stats_from_values() {
        let config = base_config();
        let mut report = StructuredFieldStencilDebugReport::from_run(&config, 1, 1);
        let mut values = vec![0.0f32; config.values_len()];
        values[4] = 1.0;
        values[8] = 2.0;
        values[12] = -3.0;
        report.apply_field_stats(&values, &config);
        assert_eq!(report.field_max, Some(2.0));
        assert_eq!(report.field_l1_norm, Some(6.0));
    }
}
