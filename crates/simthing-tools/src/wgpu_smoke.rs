use wgpu::{
    util::DeviceExt, Extent3d, ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, Maintain,
    MapMode, Origin3d, TextureAspect,
};

use crate::{
    bevy::GlyphInstanceGpu, deform::TextDeformTable, path::TextPathTable, style::TextStyleTable,
    warp::TextWarpTable,
};

fn smoke_device_limits() -> wgpu::Limits {
    let mut limits = wgpu::Limits::default();
    limits.max_bind_groups = 8;
    limits
}

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
    @location(5) style_params: vec4<f32>,
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
    @location(3) style_params: vec4<f32>,
    @location(4) local_uv: vec2<f32>,
};

struct StyleRow {
    fill_rgba: vec4<f32>,
    accent_rgba: vec4<f32>,
    outline_rgba: vec4<f32>,
    glow_rgba: vec4<f32>,
    params0: vec4<f32>,
    params1: vec4<f32>,
}

@group(1) @binding(0) var atlas_tex: texture_2d<f32>;
@group(1) @binding(1) var atlas_smp: sampler;
@group(2) @binding(0) var<uniform> style_globals: vec4<f32>;
@group(2) @binding(1) var<uniform> style_rows: array<StyleRow, 32>;

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
    out.style_params = instance.style_params;
    out.local_uv = mesh.corner;
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

fn sdf_coverage(sample: vec4<f32>, mode: f32, px_range: f32, uv: vec2<f32>, atlas_size: f32) -> vec2<f32> {
    if mode < 0.5 {
        return vec2(sample.a, sample.a);
    }
    let screen_range = screen_px_range(px_range, uv, atlas_size);
    if mode < 1.5 {
        let sd = sample.a;
        let alpha = clamp((sd - 0.5) * screen_range + 0.5, 0.0, 1.0);
        return vec2(alpha, sd);
    }
    let sd = median3(sample.rgb);
    let fw = max(fwidth(sd), 0.001);
    let alpha = clamp((sd - 0.5) / fw + 0.5, 0.0, 1.0);
    return vec2(alpha, sd);
}

fn style_row_at(slot: u32) -> StyleRow {
    if slot >= 32u {
        return style_rows[0];
    }
    return style_rows[slot];
}

fn apply_style_fill(style: StyleRow, base_color: vec4<f32>, local_uv: vec2<f32>) -> vec4<f32> {
    var opacity = style.params0.x;
    let gradient_mode = style.params0.y;
    var t = 0.0;
    if gradient_mode > 0.5 && gradient_mode < 1.5 {
        t = local_uv.x;
    } else if gradient_mode >= 1.5 {
        t = local_uv.y;
    }
    let fill_rgb = mix(style.fill_rgba.rgb, style.accent_rgba.rgb, t);
    let pulse_amp = style.params1.x;
    if pulse_amp > 0.0 {
        let pulse = sin(style_globals.x * style.params1.y + style.params1.z) * pulse_amp;
        opacity = clamp(opacity + pulse, 0.0, 1.0);
    }
    return vec4(base_color.rgb * fill_rgb, base_color.a * opacity);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(atlas_tex, atlas_smp, in.uv);
    let slot = u32(clamp(in.style_params.x, 0.0, 31.0));
    let style = style_row_at(slot);
    let styled_color = apply_style_fill(style, in.color, in.local_uv);
    let coverage = sdf_coverage(sample, in.sdf_params.x, in.sdf_params.y, in.uv, in.sdf_params.z);
    var alpha = styled_color.a * coverage.x;
    return vec4(styled_color.rgb, alpha);
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
    wgpu_styled_instanced_text_smoke(
        target,
        instances,
        atlas_pixels,
        atlas_size,
        &TextStyleTable::with_defaults(),
        0.0,
    )
}

pub fn wgpu_styled_instanced_text_smoke(
    target: WgpuSmokeTarget,
    instances: &[GlyphInstanceGpu],
    atlas_pixels: &[u8],
    atlas_size: u32,
    style_table: &TextStyleTable,
    time: f32,
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
    let style_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr3r_style_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
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
        label: Some("lr3r_smoke_pipeline_layout"),
        bind_group_layouts: &[
            &target_bind_group_layout,
            &atlas_bind_group_layout,
            &style_bind_group_layout,
        ],
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

    let style_globals = style_table.to_globals(time);
    let style_rows = style_table.to_rows_uniform();
    let style_globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr3r_style_globals"),
        contents: bytemuck::bytes_of(&style_globals),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let style_rows_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr3r_style_rows"),
        contents: bytemuck::bytes_of(&style_rows),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let style_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr3r_style_bind_group"),
        layout: &style_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: style_globals_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: style_rows_buffer.as_entire_binding(),
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
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 64,
                            shader_location: 5,
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
        pass.set_bind_group(2, &style_bind_group, &[]);
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

const WGPU_DEFORM_SMOKE_SHADER: &str = r#"
struct TargetSize { size: vec2<f32>, }
@group(0) @binding(0) var<uniform> frame_size: TargetSize;

struct GlyphInstance {
    @location(1) pos_size: vec4<f32>,
    @location(2) uv_rect: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) sdf_params: vec4<f32>,
    @location(5) style_params: vec4<f32>,
    @location(6) deform_params: vec4<f32>,
    @location(7) path_params: vec4<f32>,
    @location(8) warp_params: vec4<f32>,
}

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) corner: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) sdf_params: vec4<f32>,
    @location(3) style_params: vec4<f32>,
    @location(4) local_uv: vec2<f32>,
}

struct StyleRow {
    fill_rgba: vec4<f32>,
    accent_rgba: vec4<f32>,
    outline_rgba: vec4<f32>,
    glow_rgba: vec4<f32>,
    params0: vec4<f32>,
    params1: vec4<f32>,
}

struct DeformRow {
    params0: vec4<f32>,
    params1: vec4<f32>,
    params2: vec4<f32>,
}

struct PathRow {
    params0: vec4<f32>,
    start: vec4<f32>,
    control0: vec4<f32>,
    control1: vec4<f32>,
    end: vec4<f32>,
}

struct WarpRow {
    params0: vec4<f32>,
    points0: vec4<f32>,
    points1: vec4<f32>,
    points2: vec4<f32>,
    points3: vec4<f32>,
}

@group(1) @binding(0) var atlas_tex: texture_2d<f32>;
@group(1) @binding(1) var atlas_smp: sampler;
@group(2) @binding(0) var<uniform> style_globals: vec4<f32>;
@group(2) @binding(1) var<uniform> style_rows: array<StyleRow, 32>;
@group(3) @binding(0) var<uniform> deform_rows: array<DeformRow, 32>;
@group(4) @binding(0) var<uniform> path_rows: array<PathRow, 16>;
@group(5) @binding(0) var<uniform> warp_rows: array<WarpRow, 16>;

fn deform_row_at(slot: u32) -> DeformRow {
    if slot >= 32u { return deform_rows[0]; }
    return deform_rows[slot];
}

fn apply_parametric_deform(local_uv: vec2<f32>, slot: u32) -> vec2<f32> {
    let row = deform_row_at(slot);
    let kind = row.params0.x;
    if kind < 0.5 { return local_uv; }
    var uv = local_uv;
    let amount_x = row.params0.y;
    let amount_y = row.params0.z;
    let phase = row.params0.w;
    let shear_x = row.params1.x;
    let shear_y = row.params1.y;
    let fold_axis = row.params1.zw;
    let fold_amount = row.params2.x;
    if kind < 1.5 {
        let c = uv - vec2(0.5);
        uv = c * vec2(1.0 + amount_x, 1.0 + amount_y) + vec2(0.5);
    } else if kind < 2.5 {
        uv.x = uv.x + amount_x * (uv.y - 0.5);
        uv.y = uv.y + amount_y * (uv.x - 0.5);
    } else if kind < 3.5 {
        uv.x = uv.x + shear_x * (uv.y - 0.5);
        uv.y = uv.y + shear_y * (uv.x - 0.5);
    } else if kind < 4.5 {
        let axis_len = max(length(fold_axis), 0.001);
        let axis = fold_axis / axis_len;
        let d = dot(uv - vec2(0.5), axis);
        uv = uv + axis * fold_amount * sin(d * 3.14159265);
    } else {
        let pulse = sin(style_globals.x + phase) * amount_x;
        let c = uv - vec2(0.5);
        uv = c * (1.0 + pulse) + vec2(0.5);
    }
    return uv;
}

fn path_row_at(slot: u32) -> PathRow {
    if slot >= 16u { return path_rows[0]; }
    return path_rows[slot];
}

fn warp_row_at(slot: u32) -> WarpRow {
    if slot >= 16u { return warp_rows[0]; }
    return warp_rows[slot];
}

fn eval_quadratic_bezier(a: vec2<f32>, b: vec2<f32>, c: vec2<f32>, t: f32) -> vec2<f32> {
    let ab = mix(a, b, t);
    let bc = mix(b, c, t);
    return mix(ab, bc, t);
}

fn apply_text_path(local_xy: vec2<f32>, path_slot: u32, path_u: f32) -> vec2<f32> {
    let row = path_row_at(path_slot);
    let kind = row.params0.x;
    if kind < 0.5 { return local_xy; }
    let t = clamp(path_u, 0.0, 1.0);
    let baseline = mix(row.start.xy, row.end.xy, t);
    var on_path = baseline;
    if kind < 1.5 {
        let radius = row.params0.y;
        let center = row.control0.xy;
        let angle = t * 3.14159265;
        on_path = center + vec2(cos(angle), sin(angle)) * radius;
    } else if kind < 2.5 {
        on_path = eval_quadratic_bezier(row.start.xy, row.control0.xy, row.end.xy, t);
    } else if kind < 3.5 {
        let ab = mix(row.start.xy, row.control0.xy, t);
        let bc = mix(row.control0.xy, row.control1.xy, t);
        let cd = mix(row.control1.xy, row.end.xy, t);
        let abc = mix(ab, bc, t);
        let bcd = mix(bc, cd, t);
        on_path = mix(abc, bcd, t);
    }
    let local_offset = local_xy - mix(row.start.xy, row.end.xy, t);
    return on_path + local_offset;
}

fn apply_warp_field(pos: vec2<f32>, warp_slot: u32, local_norm: vec2<f32>) -> vec2<f32> {
    let row = warp_row_at(warp_slot);
    let kind = row.params0.x;
    if kind < 0.5 { return pos; }
    let strength = row.params0.y;
    if kind < 2.5 {
        let top = mix(row.points0.xy, row.points1.xy, local_norm.x);
        let bot = mix(row.points2.xy, row.points3.xy, local_norm.x);
        let offset = mix(top, bot, local_norm.y) * strength;
        return pos + offset;
    }
    if kind < 4.5 {
        let c = vec2(0.5, 0.5);
        let d = local_norm - c;
        let r = length(d);
        let bend = sin(r * 3.14159265 + row.params0.z) * strength;
        return pos + normalize(d + vec2(0.001, 0.0)) * bend;
    }
    return pos;
}

@vertex
fn vs_main(mesh: VertexInput, instance: GlyphInstance) -> VertexOutput {
    var out: VertexOutput;
    let deform_slot = u32(clamp(instance.deform_params.x, 0.0, 31.0));
    let path_slot = u32(clamp(instance.path_params.x, 0.0, 15.0));
    let warp_slot = u32(clamp(instance.warp_params.x, 0.0, 15.0));
    let source_uv = mesh.corner;
    let deformed_uv = apply_parametric_deform(source_uv, deform_slot);
    let path_u = instance.path_params.y + source_uv.x * instance.path_params.z;
    var local_xy = deformed_uv * instance.pos_size.zw + instance.pos_size.xy;
    local_xy = apply_text_path(local_xy, path_slot, path_u);
    local_xy = apply_warp_field(local_xy, warp_slot, source_uv);
    let ndc_x = (local_xy.x / frame_size.size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (local_xy.y / frame_size.size.y) * 2.0;
    out.clip_position = vec4(ndc_x, ndc_y, 0.0, 1.0);
    out.uv = mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv);
    out.color = instance.color;
    out.sdf_params = instance.sdf_params;
    out.style_params = instance.style_params;
    out.local_uv = source_uv;
    return out;
}

fn style_row_at(slot: u32) -> StyleRow {
    if slot >= 32u { return style_rows[0]; }
    return style_rows[slot];
}

fn apply_style_fill(style: StyleRow, base_color: vec4<f32>, local_uv: vec2<f32>) -> vec4<f32> {
    var opacity = style.params0.x;
    let gradient_mode = style.params0.y;
    var t = 0.0;
    if gradient_mode > 0.5 && gradient_mode < 1.5 { t = local_uv.x; }
    else if gradient_mode >= 1.5 { t = local_uv.y; }
    let fill_rgb = mix(style.fill_rgba.rgb, style.accent_rgba.rgb, t);
    return vec4(base_color.rgb * fill_rgb, base_color.a * opacity);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(atlas_tex, atlas_smp, in.uv);
    let slot = u32(clamp(in.style_params.x, 0.0, 31.0));
    let style = style_row_at(slot);
    let styled_color = apply_style_fill(style, in.color, in.local_uv);
    var alpha = styled_color.a * sample.a;
    return vec4(styled_color.rgb, alpha);
}
"#;

/// Raw-wgpu smoke with style + deformation tables (LR6C); path/warp default to slot 0 no-op.
pub fn wgpu_deformed_instanced_text_smoke(
    target: WgpuSmokeTarget,
    instances: &[GlyphInstanceGpu],
    atlas_pixels: &[u8],
    atlas_size: u32,
    style_table: &TextStyleTable,
    deform_table: &TextDeformTable,
    time: f32,
) -> Result<WgpuTextSmokeResult, String> {
    wgpu_path_warp_instanced_text_smoke(
        target,
        instances,
        atlas_pixels,
        atlas_size,
        style_table,
        deform_table,
        &TextPathTable::with_defaults(),
        &TextWarpTable::with_defaults(),
        time,
    )
}

/// Raw-wgpu smoke with style + deformation + path/warp tables (LR6D).
pub fn wgpu_path_warp_instanced_text_smoke(
    target: WgpuSmokeTarget,
    instances: &[GlyphInstanceGpu],
    atlas_pixels: &[u8],
    atlas_size: u32,
    style_table: &TextStyleTable,
    deform_table: &TextDeformTable,
    path_table: &TextPathTable,
    warp_table: &TextWarpTable,
    time: f32,
) -> Result<WgpuTextSmokeResult, String> {
    if instances.is_empty() {
        return Err("no glyph instances for deformed wgpu smoke".into());
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
            label: Some("lr6d_path_warp_smoke_device"),
            required_features: wgpu::Features::empty(),
            required_limits: smoke_device_limits(),
            memory_hints: Default::default(),
        },
        None,
    ))
    .map_err(|e| e.to_string())?;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("lr6c_deform_smoke_shader"),
        source: wgpu::ShaderSource::Wgsl(WGPU_DEFORM_SMOKE_SHADER.into()),
    });

    let target_uniform = WgpuTargetSize {
        size: [target.width as f32, target.height as f32],
    };
    let target_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr6c_target"),
        contents: bytemuck::bytes_of(&target_uniform),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let target_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr6c_target_layout"),
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
    let target_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr6c_target_bg"),
        layout: &target_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: target_buffer.as_entire_binding(),
        }],
    });

    let corners = [
        WgpuCornerVertex { corner: [0.0, 0.0] },
        WgpuCornerVertex { corner: [1.0, 0.0] },
        WgpuCornerVertex { corner: [0.0, 1.0] },
        WgpuCornerVertex { corner: [0.0, 1.0] },
        WgpuCornerVertex { corner: [1.0, 0.0] },
        WgpuCornerVertex { corner: [1.0, 1.0] },
    ];
    let corner_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr6c_corners"),
        contents: bytemuck::cast_slice(&corners),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr6c_instances"),
        contents: bytemuck::cast_slice(instances),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let (atlas_view, atlas_sampler) = create_smoke_atlas(&device, &queue, atlas_pixels, atlas_size);
    let atlas_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr6c_atlas_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
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
    let atlas_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr6c_atlas_bg"),
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

    let style_globals = style_table.to_globals(time);
    let style_rows = style_table.to_rows_uniform();
    let style_globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr6c_style_globals"),
        contents: bytemuck::bytes_of(&style_globals),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let style_rows_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr6c_style_rows"),
        contents: bytemuck::bytes_of(&style_rows),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let style_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr6c_style_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
    let style_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr6c_style_bg"),
        layout: &style_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: style_globals_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: style_rows_buffer.as_entire_binding(),
            },
        ],
    });

    let deform_rows = deform_table.to_rows_uniform();
    let deform_rows_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr6c_deform_rows"),
        contents: bytemuck::bytes_of(&deform_rows),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let deform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr6c_deform_layout"),
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
    let deform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr6c_deform_bg"),
        layout: &deform_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: deform_rows_buffer.as_entire_binding(),
        }],
    });

    let path_rows = path_table.to_rows_uniform();
    let path_rows_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr6d_path_rows"),
        contents: bytemuck::bytes_of(&path_rows),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let path_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr6d_path_layout"),
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
    let path_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr6d_path_bg"),
        layout: &path_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: path_rows_buffer.as_entire_binding(),
        }],
    });

    let warp_rows = warp_table.to_rows_uniform();
    let warp_rows_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lr6d_warp_rows"),
        contents: bytemuck::bytes_of(&warp_rows),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let warp_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lr6d_warp_layout"),
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
    let warp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("lr6d_warp_bg"),
        layout: &warp_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: warp_rows_buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("lr6c_pipeline_layout"),
        bind_group_layouts: &[
            &target_bind_group_layout,
            &atlas_bind_group_layout,
            &style_bind_group_layout,
            &deform_bind_group_layout,
            &path_bind_group_layout,
            &warp_bind_group_layout,
        ],
        push_constant_ranges: &[],
    });

    let render_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("lr6c_render_tex"),
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
    let render_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("lr6c_pipeline"),
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
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 64,
                            shader_location: 5,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 80,
                            shader_location: 6,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 96,
                            shader_location: 7,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 112,
                            shader_location: 8,
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
        label: Some("lr6c_encoder"),
    });
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("lr6c_pass"),
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
        pass.set_bind_group(2, &style_bind_group, &[]);
        pass.set_bind_group(3, &deform_bind_group, &[]);
        pass.set_bind_group(4, &path_bind_group, &[]);
        pass.set_bind_group(5, &warp_bind_group, &[]);
        pass.set_vertex_buffer(0, corner_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.slice(..));
        pass.draw(0..6, 0..instances.len() as u32);
    }

    readback_smoke_pixels(&device, &queue, encoder, &render_texture, target)
}

fn create_smoke_atlas(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    atlas_pixels: &[u8],
    atlas_size: u32,
) -> (wgpu::TextureView, wgpu::Sampler) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("smoke_atlas"),
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
            texture: &texture,
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
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("smoke_atlas_sampler"),
        ..Default::default()
    });
    (view, sampler)
}

fn readback_smoke_pixels(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    mut encoder: wgpu::CommandEncoder,
    render_texture: &wgpu::Texture,
    target: WgpuSmokeTarget,
) -> Result<WgpuTextSmokeResult, String> {
    let bytes_per_row = ((target.width * 4) + 255) & !255;
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("smoke_readback"),
        size: bytes_per_row as u64 * target.height as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        ImageCopyTexture {
            texture: render_texture,
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
