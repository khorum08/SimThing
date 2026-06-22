use wgpu::{
    util::DeviceExt, Extent3d, ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, Maintain,
    MapMode, Origin3d, TextureAspect,
};

use crate::bevy::GlyphInstanceGpu;

/// Offscreen target dimensions for LR3R wgpu smoke draws.
#[derive(Debug, Clone, Copy)]
pub struct WgpuSmokeTarget {
    pub width: u32,
    pub height: u32,
}

impl WgpuSmokeTarget {
    pub fn has_alpha_text_pixels(&self, pixels: &[u8]) -> bool {
        let expected = (self.width * self.height * 4) as usize;
        if pixels.len() != expected {
            return false;
        }
        pixels
            .chunks(4)
            .any(|px| px[3] > 0 && (px[0] > 0 || px[1] > 0 || px[2] > 0))
    }

    pub fn readback_pixel_stats(&self, pixels: &[u8]) -> String {
        let non_zero_alpha = pixels.chunks(4).filter(|px| px[3] > 0).count();
        let non_zero_rgb = pixels
            .chunks(4)
            .filter(|px| px[0] > 0 || px[1] > 0 || px[2] > 0)
            .count();
        let max_r = pixels.chunks(4).map(|px| px[0]).max().unwrap_or(0);
        let max_g = pixels.chunks(4).map(|px| px[1]).max().unwrap_or(0);
        let max_b = pixels.chunks(4).map(|px| px[2]).max().unwrap_or(0);
        let max_a = pixels.chunks(4).map(|px| px[3]).max().unwrap_or(0);
        format!(
            "len={}, non_zero_alpha={non_zero_alpha}, non_zero_rgb={non_zero_rgb}, max_rgba=({max_r},{max_g},{max_b},{max_a})",
            pixels.len(),
        )
    }
}

/// Result of an offscreen instanced glyph shader draw used by LR3R smoke tests.
pub struct WgpuTextSmokeResult {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

const WGPU_SMOKE_SHADER: &str = r#"
struct TargetSize {
    size: vec2<f32>,
};
@group(0) @binding(0) var<uniform> frame_size: TargetSize;

struct GlyphInstance {
    @location(1) pos_size: vec4<f32>,
    @location(2) uv_rect: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) sdf_params: vec4<f32>,
};

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) corner: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) sdf_params: vec4<f32>,
};

@group(1) @binding(0) var atlas_tex: texture_2d<f32>;
@group(1) @binding(1) var atlas_smp: sampler;

@vertex
fn vs_main(mesh: VertexInput, instance: GlyphInstance) -> VertexOutput {
    var out: VertexOutput;
    let pos = instance.pos_size.xy + mesh.corner * instance.pos_size.zw;
    let ndc_x = (pos.x / frame_size.size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / frame_size.size.y) * 2.0;
    out.clip_position = vec4(ndc_x, ndc_y, 0.0, 1.0);
    out.uv = mix(instance.uv_rect.xy, instance.uv_rect.zw, mesh.corner);
    out.color = instance.color;
    out.sdf_params = instance.sdf_params;
    return out;
}

fn median3(v: vec3<f32>) -> f32 {
    return max(min(v.r, v.g), min(max(v.r, v.g), v.b));
}

fn screen_px_range(px_range: f32, uv: vec2<f32>, atlas_size: f32) -> f32 {
    let unit_range = px_range / atlas_size;
    let dx = length(vec2<f32>(dpdx(uv.x), dpdy(uv.x)));
    let dy = length(vec2<f32>(dpdx(uv.y), dpdy(uv.y)));
    return max(0.5 * dot(vec2(unit_range), vec2(dx, dy)) * atlas_size, 1.0);
}

fn sdf_alpha(sample: vec4<f32>, mode: f32, px_range: f32, uv: vec2<f32>, atlas_size: f32) -> f32 {
    if mode < 0.5 {
        return sample.a;
    }
    let screen_range = screen_px_range(px_range, uv, atlas_size);
    if mode < 1.5 {
        let sd = sample.a;
        return clamp((sd - 0.5) * screen_range + 0.5, 0.0, 1.0);
    }
    let sd = median3(sample.rgb);
    let fw = max(fwidth(sd), 0.001);
    return clamp((sd - 0.5) / fw + 0.5, 0.0, 1.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(atlas_tex, atlas_smp, in.uv);
    let alpha = sdf_alpha(sample, in.sdf_params.x, in.sdf_params.y, in.uv, in.sdf_params.z);
    return vec4(in.color.rgb, in.color.a * alpha);
}
"#;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct WgpuTargetSize {
    size: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct WgpuCornerVertex {
    corner: [f32; 2],
}

pub fn wgpu_instanced_text_smoke(
    target: WgpuSmokeTarget,
    instances: &[GlyphInstanceGpu],
    atlas_pixels: &[u8],
    atlas_size: u32,
) -> Result<WgpuTextSmokeResult, String> {
    if instances.is_empty() {
        return Err("no glyph instances for wgpu smoke".into());
    }

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .ok_or("no wgpu adapter")?;
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("lr3r_wgpu_smoke"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    ))
    .map_err(|e| e.to_string())?;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("lr3r_wgpu_smoke_shader"),
        source: wgpu::ShaderSource::Wgsl(WGPU_SMOKE_SHADER.into()),
    });

    let target_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr3r_target_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
    let atlas_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr3r_atlas_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("lr3r_smoke_pipeline_layout"),
        bind_group_layouts: &[&target_bind_group_layout, &atlas_bind_group_layout],
        push_constant_ranges: &[],
    });

    let corners = [
        WgpuCornerVertex { corner: [0.0, 0.0] },
        WgpuCornerVertex { corner: [1.0, 0.0] },
        WgpuCornerVertex { corner: [1.0, 1.0] },
        WgpuCornerVertex { corner: [0.0, 0.0] },
        WgpuCornerVertex { corner: [1.0, 1.0] },
        WgpuCornerVertex { corner: [0.0, 1.0] },
    ];
    let corner_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr3r_corner_buffer"),
        contents: bytemuck::cast_slice(&corners),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr3r_instance_buffer"),
        contents: bytemuck::cast_slice(instances),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let target_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr3r_target_uniform"),
        contents: bytemuck::bytes_of(&WgpuTargetSize {
            size: [target.width as f32, target.height as f32],
        }),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let atlas_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("lr3r_atlas_texture"),
        size: wgpu::Extent3d {
            width: atlas_size,
            height: atlas_size,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        ImageCopyTexture {
            texture: &atlas_texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        atlas_pixels,
        ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(atlas_size * 4),
            rows_per_image: Some(atlas_size),
        },
        Extent3d {
            width: atlas_size,
            height: atlas_size,
            depth_or_array_layers: 1,
        },
    );
    let atlas_view = atlas_texture.create_view(&Default::default());
    let atlas_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

    let render_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("lr3r_render_target"),
        size: wgpu::Extent3d {
            width: target.width,
            height: target.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let render_view = render_texture.create_view(&Default::default());

    let target_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr3r_target_bind_group"),
        layout: &target_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: target_uniform.as_entire_binding(),
        }],
    });
    let atlas_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr3r_atlas_bind_group"),
        layout: &atlas_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&atlas_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&atlas_sampler),
            },
        ],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("lr3r_smoke_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<WgpuCornerVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GlyphInstanceGpu>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 0,
                            shader_location: 1,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 16,
                            shader_location: 2,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 32,
                            shader_location: 3,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 48,
                            shader_location: 4,
                        },
                    ],
                },
            ],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8Unorm,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("lr3r_smoke_encoder"),
    });
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("lr3r_smoke_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &render_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &target_bind_group, &[]);
        pass.set_bind_group(1, &atlas_bind_group, &[]);
        pass.set_vertex_buffer(0, corner_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.slice(..));
        pass.draw(0..6, 0..instances.len() as u32);
    }

    let bytes_per_row = ((target.width * 4) + 255) & !255;
    let staging_size = bytes_per_row as u64 * target.height as u64;
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("lr3r_smoke_readback"),
        size: staging_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        ImageCopyTexture {
            texture: &render_texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        ImageCopyBuffer {
            buffer: &staging,
            layout: ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(target.height),
            },
        },
        Extent3d {
            width: target.width,
            height: target.height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(encoder.finish()));

    let slice = staging.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    device.poll(Maintain::Wait);
    rx.recv()
        .expect("map_async callback")
        .map_err(|e| e.to_string())?;

    let mapped = slice.get_mapped_range();
    let row_bytes = target.width as usize * 4;
    let mut out = Vec::with_capacity(row_bytes * target.height as usize);
    for row in 0..target.height as usize {
        let start = row * bytes_per_row as usize;
        out.extend_from_slice(&mapped[start..start + row_bytes]);
    }
    drop(mapped);
    staging.unmap();

    if !target.has_alpha_text_pixels(&out) {
        return Err(format!(
            "wgpu instanced draw produced no text pixels ({})",
            target.readback_pixel_stats(&out)
        ));
    }

    Ok(WgpuTextSmokeResult {
        pixels: out,
        width: target.width,
        height: target.height,
    })
}

/// Raw-wgpu smoke draw through the SDF/MSDF shader path (mode encoded in `sdf_params.x`).
pub fn wgpu_sdf_instanced_text_smoke(
    target: WgpuSmokeTarget,
    instances: &[GlyphInstanceGpu],
    atlas_pixels: &[u8],
    atlas_size: u32,
) -> Result<WgpuTextSmokeResult, String> {
    if instances.iter().all(|i| i.sdf_params[0] < 0.5) {
        return Err("sdf smoke requires at least one SDF/MSDF instance".into());
    }
    wgpu_instanced_text_smoke(target, instances, atlas_pixels, atlas_size)
}
