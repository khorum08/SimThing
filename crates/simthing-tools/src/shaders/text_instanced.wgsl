#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip

struct GlyphInstance {
    @location(5) pos_size: vec4<f32>,
    @location(6) uv_rect: vec4<f32>,
    @location(7) color: vec4<f32>,
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

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(atlas_tex, atlas_smp, in.uv).a;
    return vec4(in.color.rgb, in.color.a * alpha);
}