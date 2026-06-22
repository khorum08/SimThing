#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip

struct GlyphInstance {
    @location(5) pos_size: vec4<f32>,
    @location(6) uv_rect: vec4<f32>,
    @location(7) color: vec4<f32>,
    @location(8) sdf_params: vec4<f32>,
    @location(9) style_params: vec4<f32>,
    @location(10) deform_params: vec4<f32>,
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

@group(2) @binding(0) var atlas_tex: texture_2d<f32>;
@group(2) @binding(1) var atlas_smp: sampler;
@group(3) @binding(0) var<uniform> style_globals: vec4<f32>;
@group(3) @binding(1) var<uniform> style_rows: array<StyleRow, 32>;
@group(4) @binding(0) var<uniform> deform_rows: array<DeformRow, 32>;

fn deform_row_at(slot: u32) -> DeformRow {
    if slot >= 32u {
        return deform_rows[0];
    }
    return deform_rows[slot];
}

fn apply_parametric_deform(local_uv: vec2<f32>, slot: u32) -> vec2<f32> {
    let row = deform_row_at(slot);
    let kind = row.params0.x;
    if kind < 0.5 {
        return local_uv;
    }
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

@vertex
fn vertex(vertex: Vertex, instance: GlyphInstance) -> VertexOutput {
    var out: VertexOutput;
    let deform_slot = u32(clamp(instance.deform_params.x, 0.0, 31.0));
    let source_uv = vertex.uv;
    let deformed_uv = apply_parametric_deform(source_uv, deform_slot);
    let local = vec4(
        deformed_uv * instance.pos_size.zw + instance.pos_size.xy,
        0.0,
        1.0,
    );
    out.clip_position = mesh2d_position_local_to_clip(identity_mat4(), local);
    out.uv = mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv);
    out.color = instance.color;
    out.sdf_params = instance.sdf_params;
    out.style_params = instance.style_params;
    out.local_uv = source_uv;
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

fn sdf_coverage(
    sample: vec4<f32>,
    mode: f32,
    px_range: f32,
    uv: vec2<f32>,
    atlas_size: f32,
) -> vec2<f32> {
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
        let pulse_freq = style.params1.y;
        let pulse_phase = style.params1.z;
        let pulse = sin(style_globals.x * pulse_freq + pulse_phase) * pulse_amp;
        opacity = clamp(opacity + pulse, 0.0, 1.0);
    }
    return vec4(base_color.rgb * fill_rgb, base_color.a * opacity);
}

fn apply_sdf_effects(
    coverage: vec2<f32>,
    style: StyleRow,
    mode: f32,
    px_range: f32,
    uv: vec2<f32>,
    atlas_size: f32,
) -> vec4<f32> {
    let alpha = coverage.x;
    let sd = coverage.y;
    var rgb = style.fill_rgba.rgb;
    var out_a = alpha * style.params0.x;

    let outline_w = style.params0.z;
    let glow_r = style.params0.w;
    if mode >= 0.5 && (outline_w > 0.0 || glow_r > 0.0) {
        let screen_range = screen_px_range(px_range, uv, atlas_size);
        let dist = (sd - 0.5) * screen_range;
        if outline_w > 0.0 && dist < 0.0 && dist > -outline_w {
            rgb = style.outline_rgba.rgb;
            out_a = max(out_a, style.outline_rgba.a);
        }
        if glow_r > 0.0 && dist < 0.0 {
            let glow = clamp(1.0 - (-dist / glow_r), 0.0, 1.0);
            rgb = mix(rgb, style.glow_rgba.rgb, glow * style.glow_rgba.a);
            out_a = max(out_a, glow * style.glow_rgba.a);
        }
    }
    return vec4(rgb, out_a);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(atlas_tex, atlas_smp, in.uv);
    let slot = u32(clamp(in.style_params.x, 0.0, 31.0));
    let style = style_row_at(slot);
    let styled_color = apply_style_fill(style, in.color, in.local_uv);
    let coverage = sdf_coverage(
        sample,
        in.sdf_params.x,
        in.sdf_params.y,
        in.uv,
        in.sdf_params.z,
    );
    let mode = in.sdf_params.x;
    var rgb = styled_color.rgb;
    var alpha = styled_color.a;
    if mode < 0.5 {
        alpha = alpha * coverage.x;
    } else {
        let effects = apply_sdf_effects(
            coverage,
            style,
            mode,
            in.sdf_params.y,
            in.uv,
            in.sdf_params.z,
        );
        alpha = styled_color.a * effects.a;
        if style.params0.z > 0.0 || style.params0.w > 0.0 {
            rgb = mix(styled_color.rgb, effects.rgb, clamp(effects.a - coverage.x, 0.0, 1.0));
        }
    }
    return vec4(rgb, alpha);
}
