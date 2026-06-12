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

/// GPU min-plus stencil session with ping-pong buffers.
pub struct MinPlusStencilOp {
    params_buffer: Buffer,
    input_buffer: Buffer,
    output_buffer: Buffer,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    config: MinPlusStencilConfig,
}

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

        Ok(Self {
            params_buffer,
            input_buffer,
            output_buffer,
            pipeline,
            bind_group_layout,
            config,
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
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("min_plus_stencil_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(
                self.config.width.div_ceil(MIN_PLUS_WORKGROUP_SIZE),
                self.config.height.div_ceil(MIN_PLUS_WORKGROUP_SIZE),
                1,
            );
        }
        queue.submit(Some(encoder.finish()));
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
