#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct FrostedGlassSettings {
    source_texel_size: vec2<f32>,
    blur_texel_size: vec2<f32>,
    panel_rects: array<vec4<f32>, 8>,
    panel_count: u32,
    enabled: u32,
    padding: vec2<f32>,
};

@group(0) @binding(0) var source_texture: texture_2d<f32>;
@group(0) @binding(1) var auxiliary_texture: texture_2d<f32>;
@group(0) @binding(2) var source_sampler: sampler;
@group(0) @binding(3) var<uniform> settings: FrostedGlassSettings;

@fragment
fn downsample(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let d = settings.source_texel_size;
    return (
        textureSample(source_texture, source_sampler, in.uv + vec2(-d.x, -d.y)) +
        textureSample(source_texture, source_sampler, in.uv + vec2( d.x, -d.y)) +
        textureSample(source_texture, source_sampler, in.uv + vec2(-d.x,  d.y)) +
        textureSample(source_texture, source_sampler, in.uv + vec2( d.x,  d.y))
    ) * 0.25;
}

fn gaussian5(uv: vec2<f32>, axis: vec2<f32>) -> vec4<f32> {
    return textureSample(source_texture, source_sampler, uv) * 0.29411765
        + textureSample(source_texture, source_sampler, uv + axis * 1.38461538) * 0.35294118
        + textureSample(source_texture, source_sampler, uv - axis * 1.38461538) * 0.35294118;
}

@fragment
fn blur_horizontal(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return gaussian5(in.uv, vec2(settings.blur_texel_size.x, 0.0));
}

@fragment
fn blur_vertical(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return gaussian5(in.uv, vec2(0.0, settings.blur_texel_size.y));
}

@fragment
fn composite(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let original = textureSample(source_texture, source_sampler, in.uv);
    if settings.enabled == 0u {
        return original;
    }
    for (var i = 0u; i < min(settings.panel_count, 8u); i = i + 1u) {
        let rect = settings.panel_rects[i];
        if in.uv.x >= rect.x && in.uv.y >= rect.y && in.uv.x <= rect.z && in.uv.y <= rect.w {
            return textureSample(auxiliary_texture, source_sampler, in.uv);
        }
    }
    return original;
}
