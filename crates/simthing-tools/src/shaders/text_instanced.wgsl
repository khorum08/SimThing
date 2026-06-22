#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip

struct GlyphInstance {
    @location(5) pos_size: vec4<f32>,
    @location(6) uv_rect: vec4<f32>,
    @location(7) color: vec4<f32>,
    @location(8) sdf_params: vec4<f32>,
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tangent: vec4<f32>,
    @location(4) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) sdf_params: vec4<f32>,
}

@group(2) @binding(0) var atlas_tex: texture_2d<f32>;
@group(2) @binding(1) var atlas_smp: sampler;

@vertex
fn vertex(vertex: Vertex, instance: GlyphInstance) -> VertexOutput {
    var out: VertexOutput;
    let local = vec4(
        vertex.position.xy * instance.pos_size.zw + instance.pos_size.xy,
        0.0,
        1.0,
    );
    out.clip_position = mesh2d_position_local_to_clip(identity_mat4(), local);
    out.uv = mix(instance.uv_rect.xy, instance.uv_rect.zw, vertex.uv);
    out.color = instance.color;
    out.sdf_params = instance.sdf_params;
    return out;
}

fn identity_mat4() -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    );
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
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(atlas_tex, atlas_smp, in.uv);
    let alpha = sdf_alpha(sample, in.sdf_params.x, in.sdf_params.y, in.uv, in.sdf_params.z);
    return vec4(in.color.rgb, in.color.a * alpha);
}
