//! Prototype-only structured 2D field stencil dispatch harness (sandbox).
//! Not wired into production tick pipeline.

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor,
    PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::context::GpuContext;

pub const WORKGROUP_SIZE: u32 = 8;

pub const VARIANT_RAW: u32 = 0;
pub const VARIANT_NORMALIZED: u32 = 1;
pub const VARIANT_DIRECTED: u32 = 2;
pub const VARIANT_CLAMPED: u32 = 3;
pub const VARIANT_DECAYED_NORMALIZED: u32 = 4;

pub const BOUNDARY_ZERO: u32 = 0;
pub const BOUNDARY_CLAMP: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct StencilParamsGpu {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub source_col: u32,
    pub target_col: u32,
    pub alpha_self_decay: f32,
    pub gamma_neighbor: f32,
    pub cap: f32,
    pub boundary_mode: u32,
    pub variant: u32,
    pub use_active_mask: u32,
    pub _pad: u32,
}

impl StencilParamsGpu {
    pub fn cells(&self) -> u32 {
        self.width * self.height
    }

    pub fn values_len(&self) -> usize {
        (self.cells() * self.n_dims) as usize
    }
}

pub struct StencilPrototype {
    params_buffer: Buffer,
    input_buffer: Buffer,
    output_buffer: Buffer,
    mask_buffer: Buffer,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    params: StencilParamsGpu,
}

impl StencilPrototype {
    pub fn new(ctx: &GpuContext, params: StencilParamsGpu) -> Self {
        let device = &ctx.device;
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("sead_tensor_stencil_prototype"),
            source: ShaderSource::Wgsl(
                include_str!("shaders/sead_tensor_stencil_prototype.wgsl").into(),
            ),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("sead_tensor_stencil_prototype_bgl"),
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

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("sead_tensor_stencil_prototype_pl"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("sead_tensor_stencil_prototype"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "stencil_step",
            compilation_options: Default::default(),
            cache: None,
        });

        let len = params.values_len();
        let cells = params.cells() as usize;
        let bytes = (len * std::mem::size_of::<f32>()) as u64;

        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("stencil_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let input_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("stencil_input"),
            size: bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("stencil_output"),
            size: bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let mask_init = vec![1u32; cells];
        let mask_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("stencil_mask"),
            contents: bytemuck::cast_slice(&mask_init),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        Self {
            params_buffer,
            input_buffer,
            output_buffer,
            mask_buffer,
            pipeline,
            bind_group_layout,
            params,
        }
    }

    pub fn upload_values(&self, ctx: &GpuContext, values: &[f32]) {
        assert_eq!(values.len(), self.params.values_len());
        ctx.queue
            .write_buffer(&self.input_buffer, 0, bytemuck::cast_slice(values));
    }

    pub fn upload_mask(&self, ctx: &GpuContext, mask: &[u32]) {
        assert_eq!(mask.len(), self.params.cells() as usize);
        ctx.queue
            .write_buffer(&self.mask_buffer, 0, bytemuck::cast_slice(mask));
    }

    pub fn set_use_active_mask(&mut self, ctx: &GpuContext, enabled: bool) {
        self.params.use_active_mask = u32::from(enabled);
        ctx.queue.write_buffer(
            &self.params_buffer,
            0,
            bytemuck::bytes_of(&self.params),
        );
    }

    fn bind_group(&self, device: &wgpu::Device, input: &Buffer, output: &Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("stencil_bg"),
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

    pub fn dispatch_once_internal(&self, ctx: &GpuContext) -> u32 {
        self.dispatch_once(ctx, &self.input_buffer, &self.output_buffer)
    }

    pub fn readback_output(&self, ctx: &GpuContext) -> Vec<f32> {
        self.readback_buffer(ctx, &self.output_buffer)
    }

    pub fn dispatch_once(&self, ctx: &GpuContext, input: &Buffer, output: &Buffer) -> u32 {
        let device = &ctx.device;
        let queue = &ctx.queue;
        let bg = self.bind_group(device, input, output);
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("stencil_dispatch"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("stencil_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bg, &[]);
            let wx = self.params.width.div_ceil(WORKGROUP_SIZE);
            let wy = self.params.height.div_ceil(WORKGROUP_SIZE);
            pass.dispatch_workgroups(wx, wy, 1);
        }
        queue.submit(Some(encoder.finish()));
        1
    }

    pub fn dispatch_ping_pong(&self, ctx: &GpuContext, steps: u32) -> u32 {
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
        dispatches
    }

    pub fn readback_after_ping_pong(&self, ctx: &GpuContext, steps: u32) -> Vec<f32> {
        let final_on_output = steps % 2 == 1;
        let src = if final_on_output {
            &self.output_buffer
        } else {
            &self.input_buffer
        };
        self.readback_buffer(ctx, src)
    }

    pub fn readback_after_steps(&self, ctx: &GpuContext, steps: u32) -> (Vec<f32>, u32) {
        let dispatches = self.dispatch_ping_pong(ctx, steps);
        (self.readback_after_ping_pong(ctx, steps), dispatches)
    }

    pub fn readback_buffer(&self, ctx: &GpuContext, src: &Buffer) -> Vec<f32> {
        let device = &ctx.device;
        let queue = &ctx.queue;
        let len = self.params.values_len();
        let bytes = (len * std::mem::size_of::<f32>()) as u64;

        let staging = device.create_buffer(&BufferDescriptor {
            label: Some("stencil_readback"),
            size: bytes,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("stencil_readback_enc"),
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

    pub fn params(&self) -> &StencilParamsGpu {
        &self.params
    }
}

pub fn cpu_stencil_step(values: &[f32], params: &StencilParamsGpu) -> Vec<f32> {
    let mut out = values.to_vec();
    let w = params.width;
    let h = params.height;
    let nd = params.n_dims;
    let sc = params.source_col;
    let tc = params.target_col;

    let sample = |buf: &[f32], x: i32, y: i32| -> f32 {
        if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
            if params.boundary_mode == BOUNDARY_CLAMP {
                let cx = x.clamp(0, w as i32 - 1) as u32;
                let cy = y.clamp(0, h as i32 - 1) as u32;
                let idx = cy * w + cx;
                return buf[(idx * nd + sc) as usize];
            }
            return 0.0;
        }
        let idx = y as u32 * w + x as u32;
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

            let (neighbor_sum, neighbor_count) = if params.variant == VARIANT_DIRECTED {
                (south + east, 2.0f32)
            } else {
                (north + south + east + west, 4.0f32)
            };

            let mut next = params.alpha_self_decay * center;
            if params.variant == VARIANT_NORMALIZED || params.variant == VARIANT_DECAYED_NORMALIZED {
                if neighbor_count > 0.0 {
                    next += params.gamma_neighbor * (neighbor_sum / neighbor_count);
                }
            } else {
                next += params.gamma_neighbor * neighbor_sum;
            }

            if params.variant == VARIANT_CLAMPED && params.cap > 0.0 {
                next = next.clamp(0.0, params.cap);
            }

            out[(idx * nd + tc) as usize] = next;
        }
    }
    out
}

pub fn cpu_horizon(values: &[f32], params: &StencilParamsGpu, hops: u32) -> Vec<f32> {
    let mut cur = values.to_vec();
    for _ in 0..hops {
        cur = cpu_stencil_step(&cur, params);
    }
    cur
}

pub fn default_params_for_variant(
    variant: u32,
    width: u32,
    height: u32,
    n_dims: u32,
    source_col: u32,
    target_col: u32,
) -> StencilParamsGpu {
    let (alpha, gamma, cap) = match variant {
        VARIANT_RAW => (1.0, 0.8, 0.0),
        VARIANT_NORMALIZED => (1.0, 0.8, 0.0),
        VARIANT_DIRECTED => (0.8, 0.8, 0.0),
        VARIANT_CLAMPED => (1.0, 0.8, 10_000.0),
        VARIANT_DECAYED_NORMALIZED => (0.8, 0.2, 0.0),
        _ => (0.8, 0.2, 0.0),
    };
    StencilParamsGpu {
        width,
        height,
        n_dims,
        source_col,
        target_col,
        alpha_self_decay: alpha,
        gamma_neighbor: gamma,
        cap,
        boundary_mode: BOUNDARY_ZERO,
        variant,
        use_active_mask: 0,
        _pad: 0,
    }
}
