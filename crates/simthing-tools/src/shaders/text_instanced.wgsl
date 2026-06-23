#ifdef WORLD_TEXT
#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::view_transformations::position_world_to_clip
#else
#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip
#endif

struct GlyphInstance {
    @location(5) pos_size: vec4<f32>,
    @location(6) uv_rect: vec4<f32>,
    @location(7) color: vec4<f32>,
    @location(8) sdf_params: vec4<f32>,
    @location(9) style_params: vec4<f32>,
    @location(10) deform_params: vec4<f32>,
    @location(11) path_params: vec4<f32>,
    @location(12) warp_params: vec4<f32>,
#ifdef WORLD_TEXT
    @location(13) anchor_height: vec4<f32>,
    @location(14) size_params: vec4<f32>,
    @location(15) distance_params: vec4<f32>,
#endif
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) sdf_params: vec4<f32>,
    @location(3) local_uv: vec2<f32>,
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

@group(2) @binding(0) var atlas_tex: texture_2d<f32>;
@group(2) @binding(1) var atlas_smp: sampler;
@group(3) @binding(0) var<uniform> style_globals: vec4<f32>;
@group(3) @binding(1) var<uniform> style_rows: array<StyleRow, 32>;
@group(4) @binding(0) var<uniform> deform_rows: array<DeformRow, 32>;
@group(5) @binding(0) var<uniform> path_rows: array<PathRow, 16>;
@group(6) @binding(0) var<uniform> warp_rows: array<WarpRow, 16>;

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

fn path_row_at(slot: u32) -> PathRow {
    if slot >= 16u {
        return path_rows[0];
    }
    return path_rows[slot];
}

fn warp_row_at(slot: u32) -> WarpRow {
    if slot >= 16u {
        return warp_rows[0];
    }
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
    if kind < 0.5 {
        return local_xy;
    }
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

fn world_axis_span_screen_px(world_pos: vec3<f32>, axis: vec3<f32>, span: f32) -> f32 {
    let p0 = position_world_to_clip(world_pos);
    let p1 = position_world_to_clip(world_pos + axis * span);
    if p0.w <= 0.0001 || p1.w <= 0.0001 {
        return 0.0;
    }
    let ndc_span = length((p1.xy / p1.w) - (p0.xy / p0.w));
    return ndc_span * view.viewport.w * 0.5;
}

fn is_screen_companion_mode(mode_or_taper: f32) -> bool {
    return mode_or_taper < -0.5 && mode_or_taper >= -1.5;
}

fn is_gpu_screen_label_mode(mode_or_taper: f32) -> bool {
    return mode_or_taper < -1.5;
}

fn clip_from_screen_px_offset(anchor_clip: vec4<f32>, offset_px: vec2<f32>) -> vec4<f32> {
    let px_to_clip = vec2(
        2.0 / max(view.viewport.z, 1.0),
        -2.0 / max(view.viewport.w, 1.0),
    );
    return vec4(
        anchor_clip.x + offset_px.x * px_to_clip.x * anchor_clip.w,
        anchor_clip.y + offset_px.y * px_to_clip.y * anchor_clip.w,
        anchor_clip.z,
        anchor_clip.w,
    );
}

fn anchor_screen_px_from_clip(anchor_clip: vec4<f32>) -> vec2<f32> {
    let ndc = anchor_clip.xy / max(anchor_clip.w, 0.0001);
    let vw = view.viewport.z;
    let vh = view.viewport.w;
    return vec2(
        (ndc.x * 0.5 + 0.5) * vw,
        (1.0 - ndc.y) * 0.5 * vh,
    );
}

// Visual falloff ruler: 0% at viewport bottom center, 100% at central vanishing point.
fn visual_horizon_falloff_progress_percent(screen_px: vec2<f32>) -> f32 {
    let vw = view.viewport.z;
    let vh = view.viewport.w;
    let base = vec2(vw * 0.5, vh);
    let vanishing = vec2(vw * 0.5, vh * 0.5);
    let ruler = vanishing - base;
    let len_sq = dot(ruler, ruler);
    if len_sq <= 0.0001 {
        return 100.0;
    }
    let progress = dot(screen_px - base, ruler) / len_sq;
    return clamp(progress, 0.0, 1.0) * 100.0;
}

fn camera_distance_depth_percent(anchor: vec3<f32>, distance_params: vec4<f32>) -> f32 {
    let camera_distance = length(view.world_position.xyz - anchor);
    return clamp(
        (camera_distance - distance_params.x)
            / max(distance_params.y - distance_params.x, 0.0001),
        0.0,
        1.0,
    ) * 100.0;
}

fn falloff_depth_percent_for_anchor(anchor: vec3<f32>, distance_params: vec4<f32>, use_visual_horizon: bool) -> f32 {
    if !use_visual_horizon {
        return camera_distance_depth_percent(anchor, distance_params);
    }
    let anchor_clip = position_world_to_clip(anchor);
    if anchor_clip.w <= 0.0001 {
        return 100.0;
    }
    return visual_horizon_falloff_progress_percent(anchor_screen_px_from_clip(anchor_clip));
}

fn world_text_falloff_alpha(
    depth_percent: f32,
    distance_params: vec4<f32>,
    style_params: vec4<f32>,
    horizon_taper: f32,
) -> f32 {
    let star_falloff_at = distance_params.z;
    let star_opacity_at = distance_params.w;
    // style_params.z = effective nameplate falloff distance (star × relative), never above star.
    let effective_falloff_at = min(style_params.z, star_falloff_at);
    let label_target = style_params.w;
    let star_alpha = distance_falloff(
        depth_percent,
        star_falloff_at,
        star_opacity_at,
        horizon_taper,
    );
    let label_ramp = distance_falloff(
        depth_percent,
        effective_falloff_at,
        label_target,
        horizon_taper,
    );
    return star_alpha * label_ramp;
}

fn apply_warp_field(pos: vec2<f32>, warp_slot: u32, local_norm: vec2<f32>) -> vec2<f32> {
    let row = warp_row_at(warp_slot);
    let kind = row.params0.x;
    if kind < 0.5 {
        return pos;
    }
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
fn vertex(vertex: Vertex, instance: GlyphInstance) -> VertexOutput {
    var out: VertexOutput;
#ifdef WORLD_TEXT
    // World-text draws a procedural non-indexed 6-vertex quad per glyph. The unit-quad corner is
    // reconstructed from the vertex index rather than the mesh vertex buffer: the
    // MeshPipeline-specialized world-text pipeline does not deliver the quad mesh's per-vertex
    // attributes (position/uv) to this shader, which collapsed every glyph quad into a dash/dot.
    var wt_corners = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
    );
    let source_uv = wt_corners[vertex.vertex_index % 6u];
    let placement_mode = instance.size_params.w;
    let gpu_screen_label = is_gpu_screen_label_mode(placement_mode);
    let screen_companion = is_screen_companion_mode(placement_mode);

    var local_xy: vec2<f32>;
    if gpu_screen_label {
        // GPU screen-label: raw label-local glyph quad coords — no deform/path/warp on position.
        local_xy = source_uv * instance.pos_size.zw + instance.pos_size.xy;
    } else {
        let deform_slot = u32(clamp(instance.deform_params.x, 0.0, 31.0));
        let path_slot = u32(clamp(instance.path_params.x, 0.0, 15.0));
        let warp_slot = u32(clamp(instance.warp_params.x, 0.0, 15.0));
        let deformed_uv = apply_parametric_deform(source_uv, deform_slot);
        let path_u = instance.path_params.y + source_uv.x * instance.path_params.z;
        local_xy = deformed_uv * instance.pos_size.zw + instance.pos_size.xy;
        local_xy = apply_text_path(local_xy, path_slot, path_u);
        local_xy = apply_warp_field(local_xy, warp_slot, source_uv);
    }

    const HORIZON_TAPER: f32 = 0.75;
    const NAMEPLATE_HEIGHT_RATIO: f32 = 1.0;
    const MIN_SELECTED_READABLE_PX: f32 = 16.0;

    let anchor = instance.anchor_height.xyz;
    let use_visual_horizon = gpu_screen_label || screen_companion;
    let depth_percent = falloff_depth_percent_for_anchor(anchor, instance.distance_params, use_visual_horizon);
    let horizon_taper = select(
        instance.size_params.w,
        HORIZON_TAPER,
        gpu_screen_label || screen_companion,
    );
    let target_height_ratio = instance.style_params.y;
    let height_ratio = distance_falloff(
        depth_percent,
        instance.distance_params.z,
        target_height_ratio,
        horizon_taper,
    );
    let up = normalize(view.world_from_view[1].xyz);
    // anchor_height.w = near rendered star visual envelope (world units).
    let star_visual_world = instance.anchor_height.w * height_ratio;
    let star_visual_height_px = world_axis_span_screen_px(anchor, up, star_visual_world);
    var label_height_px = star_visual_height_px * NAMEPLATE_HEIGHT_RATIO;
    let falloff_alpha = world_text_falloff_alpha(
        depth_percent,
        instance.distance_params,
        instance.style_params,
        horizon_taper,
    );
    // style_globals: x=time, y=min_focused_px, z=unselected_global_alpha, w=min_unselected_px
    let min_focused_px = style_globals.y;
    let unselected_global_alpha = style_globals.z;
    let min_unselected_px = style_globals.w;
    // screen_companion legacy path: zero thresholds mean "no LOD patch" (all labels eligible).
    let force_all_labels = min_unselected_px < 0.5 && min_focused_px < 0.5;
    let force_all_debug = min_unselected_px < 0.0;
    let focused = instance.size_params.z > 0.5;

    if gpu_screen_label {
        let anchor_clip = position_world_to_clip(anchor);
        var culled = false;
        var effective_label_height_px = label_height_px;
        if focused && effective_label_height_px < MIN_SELECTED_READABLE_PX {
            effective_label_height_px = MIN_SELECTED_READABLE_PX;
        }
        if !focused {
            if min_unselected_px > 0.5 && effective_label_height_px < min_unselected_px {
                culled = true;
            }
            if unselected_global_alpha < 0.5 {
                culled = true;
            }
        } else if min_focused_px > 0.5 && effective_label_height_px < min_focused_px {
            culled = true;
        }
        if !force_all_debug {
            let clip_w = max(abs(anchor_clip.w), 0.0001);
            if abs(anchor_clip.x) > clip_w || abs(anchor_clip.y) > clip_w {
                culled = true;
            }
        }
        if falloff_alpha < 0.02 {
            culled = true;
        }
        if culled {
            out.clip_position = vec4(0.0, 0.0, -1.0, 1.0);
            out.color = vec4(instance.color.rgb, 0.0);
        } else {
            // Contract A: size_params.x is uniform relative size (historical field name width_ratio).
            // local_xy.x spans natural run aspect; both axes scale with label height.
            let relative_size = instance.size_params.x;
            let scaled_label_height_px = effective_label_height_px * relative_size;
            let gap_px = instance.size_params.y * scaled_label_height_px;
            // Screen y grows downward, so a positive y offset places the label below the star
            // with glyph-top up (a negated offset put it above the star and upside-down).
            let offset_px = vec2(
                local_xy.x * scaled_label_height_px,
                star_visual_height_px * 0.5 + gap_px + (0.5 - local_xy.y) * scaled_label_height_px,
            );
            out.clip_position = clip_from_screen_px_offset(anchor_clip, offset_px);
            out.color = vec4(instance.color.rgb, instance.color.a * falloff_alpha);
        }
    } else if screen_companion {
        let anchor_clip = position_world_to_clip(anchor);
        var culled = false;
        if !force_all_labels {
            let min_height_px = select(min_unselected_px, min_focused_px, focused);
            if label_height_px < min_height_px {
                culled = true;
            }
            if !focused && unselected_global_alpha < 0.5 {
                culled = true;
            }
            let clip_w = max(abs(anchor_clip.w), 0.0001);
            if abs(anchor_clip.x) > clip_w || abs(anchor_clip.y) > clip_w {
                culled = true;
            }
            if falloff_alpha < 0.02 {
                culled = true;
            }
        }
        if culled {
            out.clip_position = vec4(0.0, 0.0, -1.0, 1.0);
            out.color = vec4(instance.color.rgb, 0.0);
        } else {
            let gap_px = instance.size_params.y * label_height_px;
            let offset_px = vec2(
                local_xy.x * label_height_px * instance.size_params.x,
                -star_visual_height_px * 0.5 - gap_px - (0.5 - local_xy.y) * label_height_px,
            );
            out.clip_position = clip_from_screen_px_offset(anchor_clip, offset_px);
            out.color = vec4(instance.color.rgb, instance.color.a * falloff_alpha);
        }
    } else {
        let right = normalize(view.world_from_view[0].xyz);
        let label_height = star_visual_world;
        let label_width = label_height * instance.size_params.x;
        let vertical_offset = -label_height * (1.0 + instance.size_params.y);
        let world_position = anchor
            + right * (local_xy.x * label_width)
            + up * (local_xy.y * label_height + vertical_offset);
        out.clip_position = position_world_to_clip(world_position);
        out.color = vec4(instance.color.rgb, instance.color.a * falloff_alpha);
    }
#else
    // Screen-space 2D path keeps the mesh's per-vertex UV (Mesh2dPipeline delivers it correctly).
    let source_uv = vertex.uv;
    let deform_slot = u32(clamp(instance.deform_params.x, 0.0, 31.0));
    let path_slot = u32(clamp(instance.path_params.x, 0.0, 15.0));
    let warp_slot = u32(clamp(instance.warp_params.x, 0.0, 15.0));
    let deformed_uv = apply_parametric_deform(source_uv, deform_slot);
    let path_u = instance.path_params.y + source_uv.x * instance.path_params.z;
    var local_xy = deformed_uv * instance.pos_size.zw + instance.pos_size.xy;
    local_xy = apply_text_path(local_xy, path_slot, path_u);
    local_xy = apply_warp_field(local_xy, warp_slot, source_uv);
    let local = vec4(local_xy, 0.0, 1.0);
    out.clip_position = mesh2d_position_local_to_clip(identity_mat4(), local);
    out.color = instance.color;
#endif
    out.uv = mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv);
    out.sdf_params = vec4(instance.sdf_params.xyz, instance.style_params.x);
    out.local_uv = source_uv;
    return out;
}

fn distance_falloff(
    depth_percent: f32,
    falloff_percent: f32,
    target_value: f32,
    horizon_taper: f32,
) -> f32 {
    let depth = clamp(depth_percent, 0.0, 100.0);
    let falloff_at = clamp(falloff_percent, 0.0001, 100.0);
    if depth <= falloff_at {
        return mix(1.0, target_value, clamp(depth / falloff_at, 0.0, 1.0));
    }
    let horizon_t = clamp((depth - falloff_at) / max(100.0 - falloff_at, 0.0001), 0.0, 1.0);
    return target_value * mix(1.0, horizon_taper, horizon_t);
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
    let slot = u32(clamp(in.sdf_params.w, 0.0, 31.0));
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
