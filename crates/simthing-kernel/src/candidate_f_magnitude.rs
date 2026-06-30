use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;

use crate::context::GpuContext;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GradientPairGpu {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Error)]
pub enum CandidateFMagnitudeError {
    #[error("no gradient rows supplied")]
    EmptyInput,
    #[error("GPU output map failed")]
    MapFailed,
}

pub fn max_candidate_f_magnitude_bits(
    ctx: &GpuContext,
    gradients: &[GradientPairGpu],
) -> Result<u32, CandidateFMagnitudeError> {
    if gradients.is_empty() {
        return Err(CandidateFMagnitudeError::EmptyInput);
    }

    let device = &ctx.device;
    let input = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("candidate_f_gradient_input"),
        contents: bytemuck::cast_slice(gradients),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let output = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("candidate_f_max_output"),
        contents: bytemuck::bytes_of(&0u32),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("candidate_f_max_readback"),
        size: std::mem::size_of::<u32>() as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("candidate_f_params"),
        contents: bytemuck::bytes_of(&(gradients.len() as u32)),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("candidate_f_magnitude_shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/candidate_f_magnitude.wgsl").into()),
    });
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("candidate_f_magnitude_bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("candidate_f_magnitude_pl"),
        bind_group_layouts: &[&layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("candidate_f_magnitude_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("candidate_f_magnitude_bg"),
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: params.as_entire_binding(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("candidate_f_magnitude_encoder"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("candidate_f_magnitude_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(((gradients.len() as u32) + 63) / 64, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &staging, 0, std::mem::size_of::<u32>() as u64);
    ctx.queue.submit(Some(encoder.finish()));

    let slice = staging.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    device.poll(wgpu::Maintain::Wait);
    rx.recv()
        .ok()
        .and_then(Result::ok)
        .ok_or(CandidateFMagnitudeError::MapFailed)?;
    let bytes = slice.get_mapped_range();
    let value = *bytemuck::from_bytes::<u32>(&bytes);
    drop(bytes);
    staging.unmap();
    Ok(value)
}

pub fn write_max_candidate_f_magnitude_bits(
    ctx: &GpuContext,
    gradients: &[GradientPairGpu],
    target_values: &wgpu::Buffer,
    target_slot: u32,
    target_col: u32,
    n_dims: u32,
) -> Result<(), CandidateFMagnitudeError> {
    if gradients.is_empty() {
        return Err(CandidateFMagnitudeError::EmptyInput);
    }

    let device = &ctx.device;
    let input = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("candidate_f_gradient_input"),
        contents: bytemuck::cast_slice(gradients),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let output = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("candidate_f_max_output"),
        contents: bytemuck::bytes_of(&0u32),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("candidate_f_params"),
        contents: bytemuck::bytes_of(&(gradients.len() as u32)),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("candidate_f_magnitude_shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/candidate_f_magnitude.wgsl").into()),
    });
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("candidate_f_magnitude_bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("candidate_f_magnitude_pl"),
        bind_group_layouts: &[&layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("candidate_f_magnitude_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("candidate_f_magnitude_bg"),
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: params.as_entire_binding(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("candidate_f_magnitude_encoder"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("candidate_f_magnitude_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(((gradients.len() as u32) + 63) / 64, 1, 1);
    }
    let target_offset = u64::from(target_slot * n_dims + target_col) * 4;
    encoder.copy_buffer_to_buffer(&output, 0, target_values, target_offset, 4);
    ctx.queue.submit(Some(encoder.finish()));
    Ok(())
}
