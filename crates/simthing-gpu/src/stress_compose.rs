//! BH-2S: GPU-resident generic stress field algebra over resident choke columns.
//!
//! Scenario-track addendum under CT-4b. Single-pass per-cell field algebra only.

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;

use crate::GpuContext;

pub const STRESS_COMPOSE_WORKGROUP_SIZE: u32 = 256;
pub const STRESS_COMPOSE_MAX_PROFILES: u32 = 8;
pub const STRESS_COMPOSE_MAX_INPUT_FIELDS: u32 = 4;
pub const STRESS_OP_OVERLAP: u32 = 0;
pub const STRESS_OP_MISMATCH: u32 = 1;
pub const STRESS_OP_WEIGHTED: u32 = 2;
pub const STRESS_OP_VELOCITY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct StressComposeParamsGpu {
    width: u32,
    height: u32,
    n_dims: u32,
    choke_a_col: u32,
    choke_b_col: u32,
    n_profiles: u32,
    _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct StressComposeProfileGpu {
    operator_kind: u32,
    weight_a: f32,
    weight_b: f32,
    output_col: u32,
    choke_now_col: u32,
    choke_prev_col: u32,
}

/// One admitted stress profile (mirrors compiled admission output).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StressComposeProfile {
    pub operator_kind: u32,
    pub weight_a: f32,
    pub weight_b: f32,
    pub output_col: u32,
    pub choke_now_col: u32,
    pub choke_prev_col: u32,
}

/// GPU stress composition configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct StressComposeConfig {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub choke_a_col: u32,
    pub choke_b_col: u32,
    pub profiles: Vec<StressComposeProfile>,
}

impl StressComposeConfig {
    pub fn cells(&self) -> u32 {
        self.width * self.height
    }

    pub fn values_len(&self) -> usize {
        (self.cells() as usize) * (self.n_dims as usize)
    }

    pub fn workgroup_count(&self) -> u32 {
        self.cells().div_ceil(STRESS_COMPOSE_WORKGROUP_SIZE)
    }

    pub fn input_field_count(&self) -> usize {
        let mut cols = std::collections::BTreeSet::from([self.choke_a_col, self.choke_b_col]);
        for profile in &self.profiles {
            if profile.operator_kind == STRESS_OP_VELOCITY {
                cols.insert(profile.choke_now_col);
                cols.insert(profile.choke_prev_col);
            }
        }
        cols.len()
    }

    pub fn validate(&self) -> Result<(), StressComposeError> {
        if self.width == 0 || self.height == 0 || self.n_dims == 0 {
            return Err(StressComposeError::InvalidDimensions {
                width: self.width,
                height: self.height,
            });
        }
        if self.choke_a_col >= self.n_dims || self.choke_b_col >= self.n_dims {
            return Err(StressComposeError::InvalidColumn {
                n_dims: self.n_dims,
            });
        }
        if self.profiles.is_empty() {
            return Err(StressComposeError::NoProfiles);
        }
        if self.profiles.len() > STRESS_COMPOSE_MAX_PROFILES as usize {
            return Err(StressComposeError::TooManyProfiles {
                max: STRESS_COMPOSE_MAX_PROFILES,
            });
        }
        if self.input_field_count() > STRESS_COMPOSE_MAX_INPUT_FIELDS as usize {
            return Err(StressComposeError::InputFieldBudgetExceeded {
                max: STRESS_COMPOSE_MAX_INPUT_FIELDS,
            });
        }

        let mut output_cols = std::collections::BTreeSet::new();
        for (i, profile) in self.profiles.iter().enumerate() {
            if profile.output_col >= self.n_dims {
                return Err(StressComposeError::InvalidColumn {
                    n_dims: self.n_dims,
                });
            }
            if !output_cols.insert(profile.output_col) {
                return Err(StressComposeError::DuplicateOutputCol {
                    col: profile.output_col,
                });
            }
            if profile.operator_kind == STRESS_OP_WEIGHTED {
                if !profile.weight_a.is_finite() || !profile.weight_b.is_finite() {
                    return Err(StressComposeError::InvalidWeight { profile: i });
                }
            }
            if profile.operator_kind == STRESS_OP_VELOCITY {
                if profile.choke_now_col >= self.n_dims
                    || profile.choke_prev_col >= self.n_dims
                    || profile.choke_now_col == profile.choke_prev_col
                {
                    return Err(StressComposeError::InvalidColumn {
                        n_dims: self.n_dims,
                    });
                }
            }
        }

        let mut input_cols = std::collections::BTreeSet::from([self.choke_a_col, self.choke_b_col]);
        for profile in &self.profiles {
            if profile.operator_kind == STRESS_OP_VELOCITY {
                input_cols.insert(profile.choke_now_col);
                input_cols.insert(profile.choke_prev_col);
            }
        }
        let mut all_cols: Vec<u32> = input_cols.iter().copied().collect();
        all_cols.extend(output_cols.iter().copied());
        let unique: std::collections::BTreeSet<_> = all_cols.iter().copied().collect();
        if unique.len() != all_cols.len() {
            return Err(StressComposeError::ColumnAliasing);
        }

        let cells = self.width as u64 * self.height as u64;
        if cells > u32::MAX as u64 {
            return Err(StressComposeError::ShapeOverflow);
        }
        let values_len = cells * self.n_dims as u64;
        if values_len > u32::MAX as u64 {
            return Err(StressComposeError::ShapeOverflow);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum StressComposeError {
    #[error("width/height must be > 0 (got {width}x{height})")]
    InvalidDimensions { width: u32, height: u32 },
    #[error("column index out of range for n_dims {n_dims}")]
    InvalidColumn { n_dims: u32 },
    #[error("profiles must contain at least one entry")]
    NoProfiles,
    #[error("profiles exceeds max {max} profiles")]
    TooManyProfiles { max: u32 },
    #[error("input field fan-in exceeds max {max} distinct columns")]
    InputFieldBudgetExceeded { max: u32 },
    #[error("profile {profile} weights must be finite")]
    InvalidWeight { profile: usize },
    #[error("duplicate output_col {col} across profiles")]
    DuplicateOutputCol { col: u32 },
    #[error("input choke columns and output_col must be distinct")]
    ColumnAliasing,
    #[error("width * height * n_dims overflows representable flat buffer length")]
    ShapeOverflow,
    #[error("resident values buffer too short: need {required} bytes, got {actual}")]
    ResidentBufferTooShort { required: u64, actual: u64 },
}

pub struct StressComposeOp {
    pipeline: wgpu::ComputePipeline,
    layout: wgpu::BindGroupLayout,
}

impl StressComposeOp {
    pub fn new(ctx: &GpuContext) -> Self {
        let device = &ctx.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stress_compose"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/stress_compose.wgsl").into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("stress_compose_layout"),
            entries: &[
                uniform_entry(0),
                storage_entry(1, false),
                storage_entry(2, true),
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("stress_compose_pipeline_layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("stress_compose_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "compose_stress_fields",
            compilation_options: Default::default(),
            cache: None,
        });
        Self { pipeline, layout }
    }

    pub fn compose_resident_field(
        &self,
        ctx: &GpuContext,
        resident_values: &wgpu::Buffer,
        config: &StressComposeConfig,
    ) -> Result<(), StressComposeError> {
        config.validate()?;
        let required = (config.values_len() as u64) * std::mem::size_of::<f32>() as u64;
        if resident_values.size() < required {
            return Err(StressComposeError::ResidentBufferTooShort {
                required,
                actual: resident_values.size(),
            });
        }

        let profiles_gpu: Vec<StressComposeProfileGpu> = config
            .profiles
            .iter()
            .map(|p| StressComposeProfileGpu {
                operator_kind: p.operator_kind,
                weight_a: p.weight_a,
                weight_b: p.weight_b,
                output_col: p.output_col,
                choke_now_col: p.choke_now_col,
                choke_prev_col: p.choke_prev_col,
            })
            .collect();

        let device = &ctx.device;
        let queue = &ctx.queue;

        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("stress_compose_params"),
            contents: bytemuck::bytes_of(&StressComposeParamsGpu {
                width: config.width,
                height: config.height,
                n_dims: config.n_dims,
                choke_a_col: config.choke_a_col,
                choke_b_col: config.choke_b_col,
                n_profiles: config.profiles.len() as u32,
                _pad: 0,
            }),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let profiles_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("stress_compose_profiles"),
            contents: bytemuck::cast_slice(&profiles_gpu),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("stress_compose_bg"),
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
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("stress_compose_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("stress_compose_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(config.workgroup_count(), 1, 1);
        }
        queue.submit(Some(encoder.finish()));
        Ok(())
    }
}

/// CPU oracle for stress composition — test comparison only.
pub fn cpu_stress_compose_oracle(values: &[f32], config: &StressComposeConfig) -> Vec<f32> {
    config.validate().expect("oracle config");
    let mut out = values.to_vec();
    let nd = config.n_dims as usize;
    let cells = config.cells() as usize;
    let choke_a_col = config.choke_a_col as usize;
    let choke_b_col = config.choke_b_col as usize;
    for slot in 0..cells {
        let base = slot * nd;
        let choke_a = out[base + choke_a_col];
        let choke_b = out[base + choke_b_col];
        for profile in &config.profiles {
            let value = match profile.operator_kind {
                STRESS_OP_OVERLAP => choke_a * choke_b,
                STRESS_OP_MISMATCH => (choke_a - choke_b).abs(),
                STRESS_OP_WEIGHTED => profile.weight_a * choke_a + profile.weight_b * choke_b,
                STRESS_OP_VELOCITY => {
                    let now = out[base + profile.choke_now_col as usize];
                    let prev = out[base + profile.choke_prev_col as usize];
                    (now - prev).abs()
                }
                _ => 0.0,
            };
            out[base + profile.output_col as usize] = value;
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
