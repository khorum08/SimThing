//! Bounded min-plus neighbor-relaxation stencil for Location gridcell traversal-cost fields.
//!
//! Convention (PALMA-PATH-0 guide, cell-entry form):
//! ```text
//! D_next[cell] = W[cell] + min_{neighbor ∈ N4(cell)} D_current[neighbor]
//! D[dest] = 0   each iteration
//! ```
//! Numeric buffers only — no semantic interpretation, no route objects.

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor,
    ComputePassDescriptor, ComputePipelineDescriptor, PipelineLayoutDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::context::GpuContext;
use crate::indexed_scatter::{IndexedScatterError, IndexedScatterOp, ScatterEntry};

pub const MIN_PLUS_INF: f32 = f32::INFINITY;
pub const MIN_PLUS_WORKGROUP_SIZE: u32 = 8;
pub const MIN_PLUS_MAX_ITERATIONS: u32 = 64;

/// Configuration for a min-plus relaxation session over interleaved gridcell values.
#[derive(Clone, Debug, PartialEq)]
pub struct MinPlusStencilConfig {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub d_col: u32,
    pub w_col: u32,
    pub dest_x: u32,
    pub dest_y: u32,
    pub inf_sentinel: f32,
}

impl MinPlusStencilConfig {
    pub fn cells(&self) -> u32 {
        self.width * self.height
    }

    pub fn values_len(&self) -> usize {
        (self.cells() * self.n_dims) as usize
    }

    pub fn dest_idx(&self) -> usize {
        (self.dest_y * self.width + self.dest_x) as usize
    }

    pub fn validate(&self) -> Result<(), MinPlusStencilError> {
        if self.width == 0 || self.height == 0 {
            return Err(MinPlusStencilError::InvalidDimensions {
                width: self.width,
                height: self.height,
            });
        }
        if self.n_dims == 0 {
            return Err(MinPlusStencilError::InvalidDims(self.n_dims));
        }
        if self.d_col >= self.n_dims || self.w_col >= self.n_dims {
            return Err(MinPlusStencilError::InvalidColumn {
                d_col: self.d_col,
                w_col: self.w_col,
                n_dims: self.n_dims,
            });
        }
        if self.d_col == self.w_col {
            return Err(MinPlusStencilError::AliasedColumns { col: self.d_col });
        }
        if self.dest_x >= self.width || self.dest_y >= self.height {
            return Err(MinPlusStencilError::InvalidDestination {
                dest_x: self.dest_x,
                dest_y: self.dest_y,
                width: self.width,
                height: self.height,
            });
        }
        if !self.inf_sentinel.is_finite() && self.inf_sentinel != MIN_PLUS_INF {
            return Err(MinPlusStencilError::InvalidInfSentinel);
        }
        Ok(())
    }

    pub fn validate_iterations(&self, iterations: u32) -> Result<(), MinPlusStencilError> {
        if iterations == 0 {
            return Err(MinPlusStencilError::InvalidIterations(iterations));
        }
        if iterations > MIN_PLUS_MAX_ITERATIONS {
            return Err(MinPlusStencilError::IterationsCapExceeded {
                iterations,
                cap: MIN_PLUS_MAX_ITERATIONS,
            });
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum MinPlusStencilError {
    #[error("invalid grid dimensions {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    #[error("n_dims must be > 0 (got {0})")]
    InvalidDims(u32),
    #[error("column out of range: d_col={d_col} w_col={w_col} n_dims={n_dims}")]
    InvalidColumn { d_col: u32, w_col: u32, n_dims: u32 },
    #[error("d_col and w_col must differ (got {col})")]
    AliasedColumns { col: u32 },
    #[error("destination ({dest_x},{dest_y}) out of range for {width}x{height}")]
    InvalidDestination {
        dest_x: u32,
        dest_y: u32,
        width: u32,
        height: u32,
    },
    #[error("inf_sentinel must be +inf or finite")]
    InvalidInfSentinel,
    #[error("iterations must be >= 1 (got {0})")]
    InvalidIterations(u32),
    #[error("iterations {iterations} exceed cap {cap}")]
    IterationsCapExceeded { iterations: u32, cap: u32 },
    #[error("values buffer length {actual} < required {required}")]
    BufferTooShort { actual: usize, required: usize },
    #[error(transparent)]
    Scatter(#[from] IndexedScatterError),
    #[error("GPU W buffer byte size {actual} < required {required}")]
    GpuWBufferTooShort { actual: u64, required: u64 },
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct MinPlusParamsGpu {
    width: u32,
    height: u32,
    n_dims: u32,
    d_col: u32,
    w_col: u32,
    dest_x: u32,
    dest_y: u32,
    inf_sentinel: f32,
}

impl MinPlusParamsGpu {
    fn from_config(config: &MinPlusStencilConfig) -> Self {
        Self {
            width: config.width,
            height: config.height,
            n_dims: config.n_dims,
            d_col: config.d_col,
            w_col: config.w_col,
            dest_x: config.dest_x,
            dest_y: config.dest_y,
            inf_sentinel: config.inf_sentinel,
        }
    }
}

#[inline]
pub fn cell_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

/// Pack flat `W` and initial `D` (dest=0, others=inf) into interleaved slot-major values.
pub fn pack_w_and_initial_d(
    w: &[f32],
    config: &MinPlusStencilConfig,
) -> Result<Vec<f32>, MinPlusStencilError> {
    config.validate()?;
    let cells = config.cells() as usize;
    if w.len() != cells {
        return Err(MinPlusStencilError::BufferTooShort {
            actual: w.len(),
            required: cells,
        });
    }
    let n_dims = config.n_dims as usize;
    let mut values = vec![0.0f32; config.values_len()];
    let dest = config.dest_idx();
    for i in 0..cells {
        let base = i * n_dims;
        values[base + config.w_col as usize] = w[i];
        values[base + config.d_col as usize] = if i == dest { 0.0 } else { config.inf_sentinel };
    }
    Ok(values)
}

/// Extract flat `D` column from interleaved values.
pub fn extract_d_flat(
    values: &[f32],
    config: &MinPlusStencilConfig,
) -> Result<Vec<f32>, MinPlusStencilError> {
    let required = config.values_len();
    if values.len() < required {
        return Err(MinPlusStencilError::BufferTooShort {
            actual: values.len(),
            required,
        });
    }
    let n_dims = config.n_dims as usize;
    let cells = config.cells() as usize;
    Ok((0..cells)
        .map(|i| values[i * n_dims + config.d_col as usize])
        .collect())
}

/// One CPU min-plus relaxation step over interleaved values.
pub fn cpu_min_plus_step(
    values: &[f32],
    config: &MinPlusStencilConfig,
) -> Result<Vec<f32>, MinPlusStencilError> {
    config.validate()?;
    let required = config.values_len();
    if values.len() < required {
        return Err(MinPlusStencilError::BufferTooShort {
            actual: values.len(),
            required,
        });
    }

    let width = config.width as usize;
    let height = config.height as usize;
    let n_dims = config.n_dims as usize;
    let d_col = config.d_col as usize;
    let w_col = config.w_col as usize;
    let dest = config.dest_idx();
    let inf = config.inf_sentinel;

    let mut out = values[..required].to_vec();
    for y in 0..height {
        for x in 0..width {
            let i = cell_index(x, y, width);
            let base = i * n_dims;
            if i == dest {
                out[base + d_col] = 0.0;
                continue;
            }

            let mut best = inf;
            if x > 0 {
                best = best.min(values[cell_index(x - 1, y, width) * n_dims + d_col]);
            }
            if x + 1 < width {
                best = best.min(values[cell_index(x + 1, y, width) * n_dims + d_col]);
            }
            if y > 0 {
                best = best.min(values[cell_index(x, y - 1, width) * n_dims + d_col]);
            }
            if y + 1 < height {
                best = best.min(values[cell_index(x, y + 1, width) * n_dims + d_col]);
            }

            let w_cell = values[base + w_col];
            out[base + d_col] = if best < inf && w_cell < inf {
                w_cell + best
            } else {
                inf
            };
        }
    }
    Ok(out)
}

/// Run fixed min-plus iterations from interleaved values; returns final values buffer.
pub fn cpu_min_plus_relaxation(
    values: &[f32],
    config: &MinPlusStencilConfig,
    iterations: u32,
) -> Result<Vec<f32>, MinPlusStencilError> {
    config.validate_iterations(iterations)?;
    let mut cur = values.to_vec();
    for _ in 0..iterations {
        cur = cpu_min_plus_step(&cur, config)?;
    }
    Ok(cur)
}

/// Convenience: flat `W` → flat `D` after fixed iterations.
pub fn cpu_min_plus_d_from_w(
    w: &[f32],
    config: &MinPlusStencilConfig,
    iterations: u32,
) -> Result<Vec<f32>, MinPlusStencilError> {
    let values = pack_w_and_initial_d(w, config)?;
    let final_values = cpu_min_plus_relaxation(&values, config, iterations)?;
    extract_d_flat(&final_values, config)
}

fn build_flat_w_scatter_entries(config: &MinPlusStencilConfig) -> Vec<ScatterEntry> {
    let n_dims = config.n_dims;
    let w_col = config.w_col;
    (0..config.cells())
        .map(|i| ScatterEntry {
            src_index: i,
            dst_index: i * n_dims + w_col,
        })
        .collect()
}

fn build_interleaved_w_scatter_entries(config: &MinPlusStencilConfig) -> Vec<ScatterEntry> {
    let n_dims = config.n_dims;
    let w_col = config.w_col;
    (0..config.cells())
        .map(|i| ScatterEntry {
            src_index: i * n_dims + w_col,
            dst_index: i * n_dims + w_col,
        })
        .collect()
}

fn build_d_seed_scatter(config: &MinPlusStencilConfig) -> (Vec<ScatterEntry>, Vec<f32>) {
    let n_dims = config.n_dims;
    let d_col = config.d_col;
    let dest = config.dest_idx();
    let inf = config.inf_sentinel;
    let cells = config.cells() as usize;
    let template: Vec<f32> = (0..cells)
        .map(|i| if i == dest { 0.0 } else { inf })
        .collect();
    let entries = (0..cells as u32)
        .map(|i| ScatterEntry {
            src_index: i,
            dst_index: i * n_dims + d_col,
        })
        .collect();
    (entries, template)
}

/// GPU min-plus stencil session with ping-pong buffers.
pub struct MinPlusStencilOp {
    params_buffer: Buffer,
    input_buffer: Buffer,
    output_buffer: Buffer,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    config: MinPlusStencilConfig,
    scatter_op: IndexedScatterOp,
    flat_w_scatter_entries: Vec<ScatterEntry>,
    interleaved_w_scatter_entries: Vec<ScatterEntry>,
    d_scatter_entries: Vec<ScatterEntry>,
    d_seed_buffer: Buffer,
}

/// Which ping-pong buffer holds the latest values after an odd/even iteration count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinPlusPingPongSide {
    Input,
    Output,
}

/// How W impedance was supplied for a traversal dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinPlusTraversalWInputKind {
    PackedCpuValues,
    GpuFlatW,
    GpuInterleavedW,
}

/// W source for min-plus traversal dispatch.
#[derive(Clone, Copy, Debug)]
pub enum MinPlusTraversalInput<'a> {
    /// Compatibility: CPU-packed interleaved values uploaded once per dispatch.
    PackedCpuValues(&'a [f32]),
    /// GPU-native: flat `cells` f32 buffer (row-major W per cell).
    GpuFlatW(&'a Buffer),
    /// GPU-native: interleaved values buffer with W in `config.w_col`.
    GpuInterleavedW(&'a Buffer),
}

impl MinPlusTraversalInput<'_> {
    pub fn kind(&self) -> MinPlusTraversalWInputKind {
        match self {
            Self::PackedCpuValues(_) => MinPlusTraversalWInputKind::PackedCpuValues,
            Self::GpuFlatW(_) => MinPlusTraversalWInputKind::GpuFlatW,
            Self::GpuInterleavedW(_) => MinPlusTraversalWInputKind::GpuInterleavedW,
        }
    }
}

/// GPU-resident traversal-potential output handle after dispatch.
#[derive(Clone, Copy, Debug)]
pub struct MinPlusTraversalGpuOutputHandle<'a> {
    pub buffer: &'a Buffer,
    pub side: MinPlusPingPongSide,
    pub iterations: u32,
}

/// Production execution mode for min-plus traversal field dispatch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MinPlusTraversalExecutionMode {
    /// Dispatch only; D remains in GPU ping-pong buffer (default production mode).
    #[default]
    GpuResident,
    /// Read D back to CPU; optional caller-side shadow/property scatter.
    DiagnosticReadback,
    /// Diagnostic readback plus CPU oracle comparison (tests / verification gates).
    OracleVerification,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinPlusTraversalExecutionOptions {
    pub mode: MinPlusTraversalExecutionMode,
    pub iterations: u32,
}

impl MinPlusTraversalExecutionOptions {
    pub fn gpu_resident(iterations: u32) -> Self {
        Self {
            mode: MinPlusTraversalExecutionMode::GpuResident,
            iterations,
        }
    }

    pub fn diagnostic_readback(iterations: u32) -> Self {
        Self {
            mode: MinPlusTraversalExecutionMode::DiagnosticReadback,
            iterations,
        }
    }

    pub fn oracle_verification(iterations: u32) -> Self {
        Self {
            mode: MinPlusTraversalExecutionMode::OracleVerification,
            iterations,
        }
    }
}

/// Result of a min-plus traversal dispatch.
#[derive(Clone, Debug, PartialEq)]
pub struct MinPlusTraversalDispatchReport {
    pub gpu_dispatched: bool,
    pub iterations: u32,
    /// True when D output remains GPU-resident (no CPU readback).
    pub gpu_resident: bool,
    pub diagnostic_readback: bool,
    pub w_input_kind: MinPlusTraversalWInputKind,
    pub resident_side: MinPlusPingPongSide,
    /// Present only when diagnostic/oracle readback was requested.
    pub values: Option<Vec<f32>>,
    pub max_oracle_error: Option<f32>,
}

/// Production alias — generic GPU traversal field op (PALMA is algebraic provenance only).
pub type MinPlusTraversalFieldOp = MinPlusStencilOp;

impl MinPlusStencilOp {
    pub fn new(
        ctx: &GpuContext,
        config: MinPlusStencilConfig,
    ) -> Result<Self, MinPlusStencilError> {
        config.validate()?;
        let params = MinPlusParamsGpu::from_config(&config);
        let device = &ctx.device;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("min_plus_stencil"),
            source: ShaderSource::Wgsl(include_str!("shaders/min_plus_stencil.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("min_plus_stencil_bgl"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("min_plus_stencil"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("min_plus_stencil_pl"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "min_plus_step",
            compilation_options: Default::default(),
            cache: None,
        });

        let bytes = (config.values_len() * std::mem::size_of::<f32>()) as u64;
        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("min_plus_stencil_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let input_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("min_plus_stencil_input"),
            size: bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("min_plus_stencil_output"),
            size: bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let (d_scatter_entries, d_seed_template) = build_d_seed_scatter(&config);
        let d_seed_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("min_plus_stencil_d_seed"),
            contents: bytemuck::cast_slice(&d_seed_template),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });
        let flat_w_scatter_entries = build_flat_w_scatter_entries(&config);
        let interleaved_w_scatter_entries = build_interleaved_w_scatter_entries(&config);

        Ok(Self {
            params_buffer,
            input_buffer,
            output_buffer,
            pipeline,
            bind_group_layout,
            config,
            scatter_op: IndexedScatterOp::new(ctx),
            flat_w_scatter_entries,
            interleaved_w_scatter_entries,
            d_scatter_entries,
            d_seed_buffer,
        })
    }

    pub fn config(&self) -> &MinPlusStencilConfig {
        &self.config
    }

    pub fn upload_values(
        &self,
        ctx: &GpuContext,
        values: &[f32],
    ) -> Result<(), MinPlusStencilError> {
        let required = self.config.values_len();
        if values.len() < required {
            return Err(MinPlusStencilError::BufferTooShort {
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

    fn bind_group(
        &self,
        device: &wgpu::Device,
        input: &Buffer,
        output: &Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("min_plus_stencil_bg"),
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
            ],
        })
    }

    pub fn dispatch_once(&self, ctx: &GpuContext, input: &Buffer, output: &Buffer) {
        let device = &ctx.device;
        let queue = &ctx.queue;
        let bg = self.bind_group(device, input, output);
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("min_plus_stencil_dispatch"),
        });
        self.record_dispatch_once(&mut encoder, &bg);
        queue.submit(Some(encoder.finish()));
    }

    /// Record one min-plus relaxation pass into an existing command encoder (no queue submit).
    pub fn record_dispatch_once(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bind_group: &wgpu::BindGroup,
    ) {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("min_plus_stencil_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.dispatch_workgroups(
            self.config.width.div_ceil(MIN_PLUS_WORKGROUP_SIZE),
            self.config.height.div_ceil(MIN_PLUS_WORKGROUP_SIZE),
            1,
        );
    }

    /// Record all ping-pong iterations into one command encoder (caller submits once).
    pub fn record_ping_pong(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        iterations: u32,
    ) -> Result<(), MinPlusStencilError> {
        self.config.validate_iterations(iterations)?;
        let mut read_input = true;
        for _ in 0..iterations {
            if read_input {
                let bg = self.bind_group(device, &self.input_buffer, &self.output_buffer);
                self.record_dispatch_once(encoder, &bg);
            } else {
                let bg = self.bind_group(device, &self.output_buffer, &self.input_buffer);
                self.record_dispatch_once(encoder, &bg);
            }
            read_input = !read_input;
        }
        Ok(())
    }

    /// Record interleaved-W prep scatters into one command encoder (caller submits once).
    pub fn record_prepare_from_gpu_interleaved_w(
        &self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        w_buffer: &Buffer,
    ) -> Result<(), MinPlusStencilError> {
        self.validate_interleaved_w_buffer(w_buffer)?;
        if !self.interleaved_w_scatter_entries.is_empty() {
            let w_bg = self.scatter_op.bind_group(
                ctx,
                w_buffer,
                &self.input_buffer,
                &self.interleaved_w_scatter_entries,
            )?;
            self.scatter_op.record_dispatch(
                encoder,
                &w_bg,
                self.interleaved_w_scatter_entries.len(),
            );
        }
        if !self.d_scatter_entries.is_empty() {
            let d_bg = self.scatter_op.bind_group(
                ctx,
                &self.d_seed_buffer,
                &self.input_buffer,
                &self.d_scatter_entries,
            )?;
            self.scatter_op
                .record_dispatch(encoder, &d_bg, self.d_scatter_entries.len());
        }
        Ok(())
    }

    /// Count queue submits for the serial W→PALMA chain (diagnostic evidence only).
    pub fn serial_w_palma_queue_submit_count(iterations: u32) -> u32 {
        // W compose (1) + interleaved W scatter (1) + D seed scatter (1) + min-plus iterations.
        3 + iterations
    }

    pub fn dispatch_ping_pong(
        &self,
        ctx: &GpuContext,
        iterations: u32,
    ) -> Result<(), MinPlusStencilError> {
        self.config.validate_iterations(iterations)?;
        let mut read_input = true;
        for _ in 0..iterations {
            if read_input {
                self.dispatch_once(ctx, &self.input_buffer, &self.output_buffer);
            } else {
                self.dispatch_once(ctx, &self.output_buffer, &self.input_buffer);
            }
            read_input = !read_input;
        }
        Ok(())
    }

    pub fn readback_buffer(&self, ctx: &GpuContext, src: &Buffer) -> Vec<f32> {
        let device = &ctx.device;
        let queue = &ctx.queue;
        let len = self.config.values_len();
        let bytes = (len * std::mem::size_of::<f32>()) as u64;
        let staging = device.create_buffer(&BufferDescriptor {
            label: Some("min_plus_stencil_readback"),
            size: bytes,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("min_plus_stencil_readback_enc"),
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

    pub fn readback_after_ping_pong(&self, ctx: &GpuContext, iterations: u32) -> Vec<f32> {
        let src = if iterations % 2 == 1 {
            &self.output_buffer
        } else {
            &self.input_buffer
        };
        self.readback_buffer(ctx, src)
    }

    pub fn run_ping_pong(
        &self,
        ctx: &GpuContext,
        iterations: u32,
    ) -> Result<Vec<f32>, MinPlusStencilError> {
        self.dispatch_ping_pong(ctx, iterations)?;
        Ok(self.readback_after_ping_pong(ctx, iterations))
    }

    pub fn resident_side_after_ping_pong(iterations: u32) -> MinPlusPingPongSide {
        if iterations % 2 == 1 {
            MinPlusPingPongSide::Output
        } else {
            MinPlusPingPongSide::Input
        }
    }

    pub fn values_buffer(&self, side: MinPlusPingPongSide) -> &Buffer {
        match side {
            MinPlusPingPongSide::Input => &self.input_buffer,
            MinPlusPingPongSide::Output => &self.output_buffer,
        }
    }

    pub fn resident_values_buffer(&self, iterations: u32) -> &Buffer {
        self.values_buffer(Self::resident_side_after_ping_pong(iterations))
    }

    pub fn output_handle(&self, iterations: u32) -> MinPlusTraversalGpuOutputHandle<'_> {
        MinPlusTraversalGpuOutputHandle {
            buffer: self.resident_values_buffer(iterations),
            side: Self::resident_side_after_ping_pong(iterations),
            iterations,
        }
    }

    fn validate_flat_w_buffer(&self, w_buffer: &Buffer) -> Result<(), MinPlusStencilError> {
        let required = (self.config.cells() as u64) * std::mem::size_of::<f32>() as u64;
        if w_buffer.size() < required {
            return Err(MinPlusStencilError::GpuWBufferTooShort {
                actual: w_buffer.size(),
                required,
            });
        }
        Ok(())
    }

    fn validate_interleaved_w_buffer(&self, w_buffer: &Buffer) -> Result<(), MinPlusStencilError> {
        let required = (self.config.values_len() * std::mem::size_of::<f32>()) as u64;
        if w_buffer.size() < required {
            return Err(MinPlusStencilError::GpuWBufferTooShort {
                actual: w_buffer.size(),
                required,
            });
        }
        Ok(())
    }

    /// Scatter flat GPU W into the stencil input buffer and seed initial D on GPU.
    pub fn prepare_input_from_gpu_flat_w(
        &self,
        ctx: &GpuContext,
        w_buffer: &Buffer,
    ) -> Result<(), MinPlusStencilError> {
        self.validate_flat_w_buffer(w_buffer)?;
        self.scatter_op.dispatch(
            ctx,
            w_buffer,
            &self.input_buffer,
            &self.flat_w_scatter_entries,
        )?;
        self.seed_initial_d_column(ctx)
    }

    /// Scatter GPU interleaved W column into the stencil input buffer and seed initial D on GPU.
    pub fn prepare_input_from_gpu_interleaved_w(
        &self,
        ctx: &GpuContext,
        w_buffer: &Buffer,
    ) -> Result<(), MinPlusStencilError> {
        self.validate_interleaved_w_buffer(w_buffer)?;
        self.scatter_op.dispatch(
            ctx,
            w_buffer,
            &self.input_buffer,
            &self.interleaved_w_scatter_entries,
        )?;
        self.seed_initial_d_column(ctx)
    }

    fn seed_initial_d_column(&self, ctx: &GpuContext) -> Result<(), MinPlusStencilError> {
        self.scatter_op.dispatch(
            ctx,
            &self.d_seed_buffer,
            &self.input_buffer,
            &self.d_scatter_entries,
        )?;
        Ok(())
    }

    /// Dispatch min-plus relaxation with explicit W source and execution mode.
    ///
    /// Default production path (`GpuResident`) does not read D back to CPU.
    pub fn dispatch_traversal_from_input(
        &self,
        ctx: &GpuContext,
        input: MinPlusTraversalInput<'_>,
        w_for_oracle: Option<&[f32]>,
        options: MinPlusTraversalExecutionOptions,
    ) -> Result<MinPlusTraversalDispatchReport, MinPlusStencilError> {
        let w_input_kind = input.kind();
        match input {
            MinPlusTraversalInput::PackedCpuValues(values) => self.upload_values(ctx, values)?,
            MinPlusTraversalInput::GpuFlatW(w_buffer) => {
                self.prepare_input_from_gpu_flat_w(ctx, w_buffer)?
            }
            MinPlusTraversalInput::GpuInterleavedW(w_buffer) => {
                self.prepare_input_from_gpu_interleaved_w(ctx, w_buffer)?
            }
        }
        self.dispatch_ping_pong(ctx, options.iterations)?;

        let resident_side = Self::resident_side_after_ping_pong(options.iterations);
        let need_readback = matches!(
            options.mode,
            MinPlusTraversalExecutionMode::DiagnosticReadback
                | MinPlusTraversalExecutionMode::OracleVerification
        );

        let values = if need_readback {
            Some(self.readback_after_ping_pong(ctx, options.iterations))
        } else {
            None
        };

        let max_oracle_error = if options.mode == MinPlusTraversalExecutionMode::OracleVerification
        {
            let w = w_for_oracle.ok_or(MinPlusStencilError::BufferTooShort {
                actual: 0,
                required: self.config.cells() as usize,
            })?;
            let cpu_d = cpu_min_plus_d_from_w(w, &self.config, options.iterations)?;
            let gpu_values = values.as_ref().expect("oracle mode requires readback");
            let gpu_d = extract_d_flat(gpu_values, &self.config)?;
            Some(max_d_field_error(&cpu_d, &gpu_d))
        } else {
            None
        };

        Ok(MinPlusTraversalDispatchReport {
            gpu_dispatched: true,
            iterations: options.iterations,
            gpu_resident: options.mode == MinPlusTraversalExecutionMode::GpuResident,
            diagnostic_readback: need_readback,
            w_input_kind,
            resident_side,
            values,
            max_oracle_error,
        })
    }

    /// Dispatch min-plus relaxation with CPU-packed interleaved input (compatibility path).
    pub fn dispatch_traversal(
        &self,
        ctx: &GpuContext,
        input_values: &[f32],
        w_for_oracle: Option<&[f32]>,
        options: MinPlusTraversalExecutionOptions,
    ) -> Result<MinPlusTraversalDispatchReport, MinPlusStencilError> {
        self.dispatch_traversal_from_input(
            ctx,
            MinPlusTraversalInput::PackedCpuValues(input_values),
            w_for_oracle,
            options,
        )
    }
}

/// Maximum absolute error between CPU and GPU flat `D` fields (f32 arithmetic; not exact-authority).
pub fn max_d_field_error(cpu_d: &[f32], gpu_d: &[f32]) -> f32 {
    cpu_d
        .iter()
        .zip(gpu_d)
        .map(|(a, b)| {
            if a.is_infinite() && b.is_infinite() {
                0.0
            } else {
                (a - b).abs()
            }
        })
        .fold(0.0f32, f32::max)
}
