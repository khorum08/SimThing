//! BH-2B: GPU-resident numeric W impedance composition from base W and choke columns.
//!
//! Opened by named consumer `CT-4b_Local_Automata_W_Feedstock`. Linear weighted composition only;
//! no movement policy or semantic branches.

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;

use crate::GpuContext;

pub const W_IMPEDANCE_COMPOSE_WORKGROUP_SIZE: u32 = 256;
pub const W_IMPEDANCE_COMPOSE_MAX_PROFILES: u32 = 8;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct WImpedanceComposeParamsGpu {
    width: u32,
    height: u32,
    n_dims: u32,
    base_w_col: u32,
    choke_a_col: u32,
    choke_b_col: u32,
    n_profiles: u32,
    _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct WImpedanceComposeProfileGpu {
    weight_a: f32,
    weight_b: f32,
    output_w_col: u32,
    _pad: u32,
}

/// One admitted impedance profile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WImpedanceComposeProfile {
    pub weight_a: f32,
    pub weight_b: f32,
    pub output_w_col: u32,
}

/// GPU W composition configuration (numeric columns only).
#[derive(Clone, Debug, PartialEq)]
pub struct WImpedanceComposeConfig {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub base_w_col: u32,
    pub choke_a_col: u32,
    pub choke_b_col: u32,
    pub profiles: Vec<WImpedanceComposeProfile>,
}

impl WImpedanceComposeConfig {
    pub fn cells(&self) -> u32 {
        self.width * self.height
    }

    pub fn values_len(&self) -> usize {
        (self.cells() as usize) * (self.n_dims as usize)
    }

    pub fn workgroup_count(&self) -> u32 {
        self.cells().div_ceil(W_IMPEDANCE_COMPOSE_WORKGROUP_SIZE)
    }

    pub fn validate(&self) -> Result<(), WImpedanceComposeError> {
        if self.width == 0 || self.height == 0 {
            return Err(WImpedanceComposeError::InvalidDimensions {
                width: self.width,
                height: self.height,
            });
        }
        if self.n_dims == 0 {
            return Err(WImpedanceComposeError::InvalidDimensions {
                width: self.width,
                height: self.height,
            });
        }
        if self.base_w_col >= self.n_dims
            || self.choke_a_col >= self.n_dims
            || self.choke_b_col >= self.n_dims
        {
            return Err(WImpedanceComposeError::InvalidColumn {
                n_dims: self.n_dims,
            });
        }
        if self.profiles.is_empty() {
            return Err(WImpedanceComposeError::NoProfiles);
        }
        if self.profiles.len() > W_IMPEDANCE_COMPOSE_MAX_PROFILES as usize {
            return Err(WImpedanceComposeError::TooManyProfiles {
                max: W_IMPEDANCE_COMPOSE_MAX_PROFILES,
            });
        }
        let mut cols = vec![self.base_w_col, self.choke_a_col, self.choke_b_col];
        let mut seen_outputs = std::collections::BTreeSet::new();
        for (i, profile) in self.profiles.iter().enumerate() {
            if !profile.weight_a.is_finite() || !profile.weight_b.is_finite() {
                return Err(WImpedanceComposeError::InvalidWeight { profile: i });
            }
            if profile.output_w_col >= self.n_dims {
                return Err(WImpedanceComposeError::InvalidColumn {
                    n_dims: self.n_dims,
                });
            }
            if !seen_outputs.insert(profile.output_w_col) {
                return Err(WImpedanceComposeError::DuplicateOutputCol {
                    col: profile.output_w_col,
                });
            }
            cols.push(profile.output_w_col);
        }
        let unique: std::collections::BTreeSet<_> = cols.iter().copied().collect();
        if unique.len() != cols.len() {
            return Err(WImpedanceComposeError::ColumnAliasing);
        }
        let cells = self.width as u64 * self.height as u64;
        if cells > u32::MAX as u64 {
            return Err(WImpedanceComposeError::ShapeOverflow);
        }
        let values_len = cells * self.n_dims as u64;
        if values_len > u32::MAX as u64 {
            return Err(WImpedanceComposeError::ShapeOverflow);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum WImpedanceComposeError {
    #[error("width/height must be > 0 (got {width}x{height})")]
    InvalidDimensions { width: u32, height: u32 },
    #[error("column index out of range for n_dims {n_dims}")]
    InvalidColumn { n_dims: u32 },
    #[error("profiles must contain at least one entry")]
    NoProfiles,
    #[error("profiles exceeds max {max} profiles")]
    TooManyProfiles { max: u32 },
    #[error("profile {profile} weights must be finite")]
    InvalidWeight { profile: usize },
    #[error("duplicate output_w_col {col} across profiles")]
    DuplicateOutputCol { col: u32 },
    #[error("base_w_col, choke_a_col, choke_b_col, and output_w_col must be distinct")]
    ColumnAliasing,
    #[error("width * height * n_dims overflows representable flat buffer length")]
    ShapeOverflow,
    #[error("resident values buffer too short: need {required} bytes, got {actual}")]
    ResidentBufferTooShort { required: u64, actual: u64 },
}

/// Generic GPU session: compose one or more W profiles from base W + choke columns.
pub struct WImpedanceComposeOp {
    pipeline: wgpu::ComputePipeline,
    layout: wgpu::BindGroupLayout,
}

impl WImpedanceComposeOp {
    pub fn new(ctx: &GpuContext) -> Self {
        let device = &ctx.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("w_impedance_compose"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("shaders/w_impedance_compose.wgsl").into(),
            ),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("w_impedance_compose_layout"),
            entries: &[
                uniform_entry(0),
                storage_entry(1, false),
                storage_entry(2, true),
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("w_impedance_compose_pipeline_layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("w_impedance_compose_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "compose_w_impedance",
            compilation_options: Default::default(),
            cache: None,
        });
        Self { pipeline, layout }
    }

    /// Compose W profile columns in-place on a GPU-resident interleaved field buffer.
    pub fn compose_resident_field(
        &self,
        ctx: &GpuContext,
        resident_values: &wgpu::Buffer,
        config: &WImpedanceComposeConfig,
    ) -> Result<(), WImpedanceComposeError> {
        let bind_group = self.compose_bind_group(ctx, resident_values, config)?;
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("w_impedance_compose_enc"),
            });
        self.record_compose_pass(&mut encoder, &bind_group, config);
        ctx.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// Build bind group for W compose (batched encoder scheduling).
    pub fn compose_bind_group(
        &self,
        ctx: &GpuContext,
        resident_values: &wgpu::Buffer,
        config: &WImpedanceComposeConfig,
    ) -> Result<wgpu::BindGroup, WImpedanceComposeError> {
        config.validate()?;
        let required = (config.values_len() as u64) * std::mem::size_of::<f32>() as u64;
        if resident_values.size() < required {
            return Err(WImpedanceComposeError::ResidentBufferTooShort {
                required,
                actual: resident_values.size(),
            });
        }

        let profiles_gpu: Vec<WImpedanceComposeProfileGpu> = config
            .profiles
            .iter()
            .map(|p| WImpedanceComposeProfileGpu {
                weight_a: p.weight_a,
                weight_b: p.weight_b,
                output_w_col: p.output_w_col,
                _pad: 0,
            })
            .collect();

        let device = &ctx.device;
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("w_impedance_compose_params"),
            contents: bytemuck::bytes_of(&WImpedanceComposeParamsGpu {
                width: config.width,
                height: config.height,
                n_dims: config.n_dims,
                base_w_col: config.base_w_col,
                choke_a_col: config.choke_a_col,
                choke_b_col: config.choke_b_col,
                n_profiles: config.profiles.len() as u32,
                _pad: 0,
            }),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let profiles_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("w_impedance_compose_profiles"),
            contents: bytemuck::cast_slice(&profiles_gpu),
            usage: wgpu::BufferUsages::STORAGE,
        });

        Ok(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("w_impedance_compose_bg"),
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
                    resource: profiles_buffer.as_entire_binding(),
                },
            ],
        }))
    }

    /// Record W compose compute pass into an existing command encoder (no queue submit).
    pub fn record_compose_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bind_group: &wgpu::BindGroup,
        config: &WImpedanceComposeConfig,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("w_impedance_compose_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.dispatch_workgroups(config.workgroup_count(), 1, 1);
    }
}

/// CPU oracle for W composition — test comparison only.
pub fn cpu_w_impedance_compose_oracle(
    values: &[f32],
    config: &WImpedanceComposeConfig,
) -> Vec<f32> {
    config.validate().expect("oracle config");
    let mut out = values.to_vec();
    let nd = config.n_dims as usize;
    let cells = config.cells() as usize;
    let base_col = config.base_w_col as usize;
    let choke_a_col = config.choke_a_col as usize;
    let choke_b_col = config.choke_b_col as usize;
    for slot in 0..cells {
        let base = slot * nd;
        let base_w = out[base + base_col];
        let choke_a = out[base + choke_a_col];
        let choke_b = out[base + choke_b_col];
        for profile in &config.profiles {
            let composed = base_w + profile.weight_a * choke_a + profile.weight_b * choke_b;
            out[base + profile.output_w_col as usize] = composed;
        }
    }
    out
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
