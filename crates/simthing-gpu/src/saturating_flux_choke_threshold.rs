//! BH-1R: compact GPU-resident sum/threshold consumer over a choke readout column.
//!
//! Reads a GPU-resident structured field buffer and writes a compact 4-float result
//! (sum, max, count-above-threshold, crossed flag). Only the compact buffer is read
//! back — never the full field.

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;

use crate::context::GpuContext;

pub const CHOKE_THRESHOLD_COMPACT_FLOATS: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ChokeThresholdParamsGpu {
    width: u32,
    height: u32,
    n_dims: u32,
    choke_col: u32,
    threshold: f32,
}

/// Configuration for GPU choke-column reduce/threshold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingFluxChokeThresholdConfig {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub choke_col: u32,
    pub threshold: f32,
}

impl SaturatingFluxChokeThresholdConfig {
    pub fn cells(&self) -> u32 {
        self.width * self.height
    }

    pub fn validate(&self) -> Result<(), SaturatingFluxChokeThresholdError> {
        if self.width == 0 || self.height == 0 {
            return Err(SaturatingFluxChokeThresholdError::InvalidDimensions {
                width: self.width,
                height: self.height,
            });
        }
        if self.n_dims == 0 || self.choke_col >= self.n_dims {
            return Err(SaturatingFluxChokeThresholdError::InvalidColumn {
                choke_col: self.choke_col,
                n_dims: self.n_dims,
            });
        }
        if !self.threshold.is_finite() {
            return Err(SaturatingFluxChokeThresholdError::InvalidThreshold(
                self.threshold,
            ));
        }
        Ok(())
    }
}

/// Compact GPU reduce/threshold result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingFluxChokeThresholdResult {
    pub sum_choke: f32,
    pub max_choke: f32,
    pub count_above_threshold: u32,
    pub crossed_threshold: bool,
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum SaturatingFluxChokeThresholdError {
    #[error("width/height must be > 0 (got {width}x{height})")]
    InvalidDimensions { width: u32, height: u32 },
    #[error("choke_col {choke_col} out of range for n_dims {n_dims}")]
    InvalidColumn { choke_col: u32, n_dims: u32 },
    #[error("threshold must be finite (got {0})")]
    InvalidThreshold(f32),
    #[error("resident values buffer too short: need {required} bytes, got {actual}")]
    ResidentBufferTooShort { required: u64, actual: u64 },
    #[error("GPU choke threshold output map failed")]
    MapFailed,
}

/// Generic GPU session: sum + threshold over a resident choke column.
pub struct SaturatingFluxChokeThresholdOp {
    pipeline: wgpu::ComputePipeline,
    layout: wgpu::BindGroupLayout,
}

impl SaturatingFluxChokeThresholdOp {
    pub fn new(ctx: &GpuContext) -> Self {
        let device = &ctx.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("saturating_flux_choke_threshold"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("shaders/saturating_flux_choke_threshold.wgsl").into(),
            ),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("saturating_flux_choke_threshold_layout"),
            entries: &[
                uniform_entry(0),
                storage_entry(1, true),
                storage_entry(2, false),
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("saturating_flux_choke_threshold_pipeline_layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("saturating_flux_choke_threshold_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "reduce_choke_threshold",
            compilation_options: Default::default(),
            cache: None,
        });
        Self { pipeline, layout }
    }

    /// Reduce/threshold over GPU-resident field values; read back compact result only.
    pub fn reduce_resident_field(
        &self,
        ctx: &GpuContext,
        resident_values: &wgpu::Buffer,
        config: &SaturatingFluxChokeThresholdConfig,
    ) -> Result<SaturatingFluxChokeThresholdResult, SaturatingFluxChokeThresholdError> {
        config.validate()?;
        let cells = config.cells();
        let required = (cells as u64) * (config.n_dims as u64) * std::mem::size_of::<f32>() as u64;
        if resident_values.size() < required {
            return Err(SaturatingFluxChokeThresholdError::ResidentBufferTooShort {
                required,
                actual: resident_values.size(),
            });
        }

        let device = &ctx.device;
        let queue = &ctx.queue;
        let output_bytes =
            (CHOKE_THRESHOLD_COMPACT_FLOATS as u64) * std::mem::size_of::<f32>() as u64;

        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("saturating_flux_choke_threshold_params"),
            contents: bytemuck::bytes_of(&ChokeThresholdParamsGpu {
                width: config.width,
                height: config.height,
                n_dims: config.n_dims,
                choke_col: config.choke_col,
                threshold: config.threshold,
            }),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let output = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("saturating_flux_choke_threshold_output"),
            size: output_bytes,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let staging = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("saturating_flux_choke_threshold_readback"),
            size: output_bytes,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("saturating_flux_choke_threshold_bg"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: resident_values.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: output.as_entire_binding(),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("saturating_flux_choke_threshold_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("saturating_flux_choke_threshold_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &staging, 0, output_bytes);
        queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let data = slice.get_mapped_range();
        let readback: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        staging.unmap();

        if readback.len() != CHOKE_THRESHOLD_COMPACT_FLOATS as usize {
            return Err(SaturatingFluxChokeThresholdError::MapFailed);
        }

        Ok(SaturatingFluxChokeThresholdResult {
            sum_choke: readback[0],
            max_choke: readback[1],
            count_above_threshold: readback[2] as u32,
            crossed_threshold: readback[3] != 0.0,
        })
    }
}

/// CPU oracle for compact reduce/threshold — test comparison only.
pub fn cpu_choke_threshold_oracle(
    values: &[f32],
    config: &SaturatingFluxChokeThresholdConfig,
) -> SaturatingFluxChokeThresholdResult {
    config.validate().expect("oracle config");
    let cells = config.cells() as usize;
    let nd = config.n_dims as usize;
    let col = config.choke_col as usize;
    let mut sum_choke = 0.0f32;
    let mut max_choke = 0.0f32;
    let mut count_above_threshold = 0u32;
    for slot in 0..cells {
        let v = values[slot * nd + col];
        sum_choke += v;
        if v > max_choke {
            max_choke = v;
        }
        if v > config.threshold {
            count_above_threshold += 1;
        }
    }
    SaturatingFluxChokeThresholdResult {
        sum_choke,
        max_choke,
        count_above_threshold,
        crossed_threshold: sum_choke > config.threshold,
    }
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
