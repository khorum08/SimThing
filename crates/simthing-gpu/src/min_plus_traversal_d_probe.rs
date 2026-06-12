//! Compact downstream GPU probe over GPU-resident min-plus traversal D column (PALMA-PATH-9).
//!
//! Gathers D at explicit candidate cell indices and reduces min D in one dispatch.
//! Only the compact probe buffer is read back — never the full D field.

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;

use crate::context::GpuContext;
use crate::min_plus_stencil::{MinPlusStencilConfig, MinPlusTraversalGpuOutputHandle};

pub const TRAVERSAL_D_PROBE_MAX_CANDIDATES: u32 = 64;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct TraversalDProbeParamsGpu {
    n_dims: u32,
    d_col: u32,
    n_candidates: u32,
    inf_sentinel: f32,
}

/// Probe configuration: which D column to read from interleaved resident values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinPlusTraversalDProbeConfig {
    pub n_dims: u32,
    pub d_col: u32,
    pub inf_sentinel: f32,
}

impl MinPlusTraversalDProbeConfig {
    pub fn from_stencil_config(config: &MinPlusStencilConfig) -> Self {
        Self {
            n_dims: config.n_dims,
            d_col: config.d_col,
            inf_sentinel: config.inf_sentinel,
        }
    }
}

/// Compact probe output: gathered D per candidate plus min D across the set.
#[derive(Clone, Debug, PartialEq)]
pub struct MinPlusTraversalDProbeResult {
    pub gathered: Vec<f32>,
    pub min_d: f32,
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum MinPlusTraversalDProbeError {
    #[error("probe requires at least one candidate cell index")]
    EmptyCandidates,
    #[error("probe candidate count {count} exceeds cap {cap}")]
    TooManyCandidates { count: usize, cap: u32 },
    #[error("candidate cell index {index} out of range for {cells} cells")]
    CandidateOutOfRange { index: u32, cells: u32 },
    #[error("resident values buffer too short: need {required} bytes, got {actual}")]
    ResidentBufferTooShort { required: u64, actual: u64 },
    #[error("d_col {d_col} out of range for n_dims {n_dims}")]
    InvalidColumn { d_col: u32, n_dims: u32 },
    #[error("GPU probe output map failed")]
    MapFailed,
}

/// Generic GPU probe session consuming a resident traversal D buffer handle.
pub struct MinPlusTraversalDProbeOp {
    pipeline: wgpu::ComputePipeline,
    layout: wgpu::BindGroupLayout,
}

impl MinPlusTraversalDProbeOp {
    pub fn new(ctx: &GpuContext) -> Self {
        let device = &ctx.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("min_plus_traversal_d_probe"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("shaders/min_plus_traversal_d_probe.wgsl").into(),
            ),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("min_plus_traversal_d_probe_layout"),
            entries: &[
                uniform_entry(0),
                storage_entry(1, true),
                storage_entry(2, true),
                storage_entry(3, false),
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("min_plus_traversal_d_probe_pipeline_layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("min_plus_traversal_d_probe_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "probe_d_candidates",
            compilation_options: Default::default(),
            cache: None,
        });
        Self { pipeline, layout }
    }

    /// Probe GPU-resident traversal D at candidate cell indices; read back compact result only.
    pub fn probe_resident_d(
        &self,
        ctx: &GpuContext,
        resident: MinPlusTraversalGpuOutputHandle<'_>,
        config: &MinPlusTraversalDProbeConfig,
        candidate_cell_indices: &[u32],
        cells: u32,
    ) -> Result<MinPlusTraversalDProbeResult, MinPlusTraversalDProbeError> {
        self.validate(
            config,
            candidate_cell_indices,
            cells,
            resident.buffer.size(),
        )?;

        let device = &ctx.device;
        let queue = &ctx.queue;
        let n = candidate_cell_indices.len() as u32;
        let output_len = (n + 1) as usize;

        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("min_plus_traversal_d_probe_params"),
            contents: bytemuck::bytes_of(&TraversalDProbeParamsGpu {
                n_dims: config.n_dims,
                d_col: config.d_col,
                n_candidates: n,
                inf_sentinel: config.inf_sentinel,
            }),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("min_plus_traversal_d_probe_indices"),
            contents: bytemuck::cast_slice(candidate_cell_indices),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let output = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("min_plus_traversal_d_probe_output"),
            size: (output_len * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let staging = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("min_plus_traversal_d_probe_readback"),
            size: (output_len * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("min_plus_traversal_d_probe_bg"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: resident.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: indices.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: output.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("min_plus_traversal_d_probe_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("min_plus_traversal_d_probe_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &staging, 0, staging.size());
        queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let data = slice.get_mapped_range();
        let readback: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        staging.unmap();

        if readback.len() != output_len {
            return Err(MinPlusTraversalDProbeError::MapFailed);
        }

        Ok(MinPlusTraversalDProbeResult {
            min_d: readback[n as usize],
            gathered: readback[..n as usize].to_vec(),
        })
    }

    fn validate(
        &self,
        config: &MinPlusTraversalDProbeConfig,
        candidate_cell_indices: &[u32],
        cells: u32,
        resident_bytes: u64,
    ) -> Result<(), MinPlusTraversalDProbeError> {
        if candidate_cell_indices.is_empty() {
            return Err(MinPlusTraversalDProbeError::EmptyCandidates);
        }
        if candidate_cell_indices.len() > TRAVERSAL_D_PROBE_MAX_CANDIDATES as usize {
            return Err(MinPlusTraversalDProbeError::TooManyCandidates {
                count: candidate_cell_indices.len(),
                cap: TRAVERSAL_D_PROBE_MAX_CANDIDATES,
            });
        }
        if config.d_col >= config.n_dims || config.n_dims == 0 {
            return Err(MinPlusTraversalDProbeError::InvalidColumn {
                d_col: config.d_col,
                n_dims: config.n_dims,
            });
        }
        for &idx in candidate_cell_indices {
            if idx >= cells {
                return Err(MinPlusTraversalDProbeError::CandidateOutOfRange { index: idx, cells });
            }
        }
        let required = (cells as u64) * (config.n_dims as u64) * std::mem::size_of::<f32>() as u64;
        if resident_bytes < required {
            return Err(MinPlusTraversalDProbeError::ResidentBufferTooShort {
                required,
                actual: resident_bytes,
            });
        }
        Ok(())
    }
}

/// CPU oracle for probe output — used by tests; does not require GPU full-D readback.
pub fn cpu_probe_d_at_candidates(
    d_flat: &[f32],
    candidate_cell_indices: &[u32],
    inf_sentinel: f32,
) -> MinPlusTraversalDProbeResult {
    let gathered: Vec<f32> = candidate_cell_indices
        .iter()
        .map(|&idx| d_flat[idx as usize])
        .collect();
    let min_d = gathered
        .iter()
        .copied()
        .fold(inf_sentinel, |best, d| if d < best { d } else { best });
    MinPlusTraversalDProbeResult { gathered, min_d }
}

fn uniform_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_entry(binding: u32, read_only: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}
