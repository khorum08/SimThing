//! Bevy systems for Studio Settings performance telemetry.

use bevy::app::{App, Plugin};
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::smaa::Smaa;
use bevy::diagnostic::{DiagnosticsStore, FrameCount};
use bevy::prelude::*;
use bevy::render::view::Msaa;
use bevy::render::{renderer::RenderAdapterInfo, RenderApp};
use bevy::window::{PresentMode, PrimaryWindow, Window};
use simthing_tools::{
    natural_run_aspect_from_glyphs, normalized_label_local_x_range_from_glyphs, GlyphInstanceGpu,
    TextGlyphInstances, TextPerfDiagnostics, TypefaceAtlas, WorldTextBillboard,
    WorldTextFalloffRulerPatch, WorldTextNameplateLodPatch, WorldTextPlacementMode,
    RASTER_GLYPH_ATLAS_GUTTER_PX, RASTER_GLYPH_ATLAS_UV_INSET,
};

use crate::falloff_metric::{
    compute_map_radius_falloff_context, map_radius_progress_percent, origin_source_label,
    stabilize_map_radius_falloff_output, MapPlaneBounds,
};
use crate::star_render::{
    estimate_world_vertical_span_screen_px, nameplate_effective_falloff_distance_percent,
    nameplate_gpu_screen_label_falloff_alpha, nameplate_label_passes_density_gate,
    nameplate_label_passes_readability_gate, nameplate_scaled_label_height_px,
    nameplate_unselected_global_lod_alpha, star_falloff_alpha_at_progress,
    star_falloff_progress_percent, star_nameplate_envelope_height_ratio,
    visual_horizon_ruler_screen_y_fraction_from_top, world_anchor_screen_px, StarBillboardInstance,
    StarBillboardRenderSettings, StarNameplateDebugMode, VisualHorizonFalloffRuler,
};
use crate::studio_antialiasing::{
    antialiasing_component_state_mismatch, dual_post_aa_components_active, msaa_active,
    msaa_sample_count_label, post_aa_component_active, smaa_expected_active, smaa_preset_label,
    snapshot_from_camera_components, AntialiasingComponentSnapshot, StudioAntialiasingApplyState,
};
use crate::studio_frame_phase_gpu_telemetry::{
    read_frame_time_ms_from_diagnostics, record_frame_phase_timing,
};
use crate::studio_performance_telemetry::{
    bytes_to_vram_mb, estimate_studio_allocated_vram_bytes, read_fps_from_diagnostics,
    StudioPerformanceTelemetry,
};

use super::camera::MainCamera;
use super::galaxy_render::GalaxyStarNameplate;
use super::resources::StudioSettings;
use super::StudioAppState;

const VRAM_ESTIMATE_INTERVAL_SECS: f32 = 0.5;

/// Cached VRAM scan scheduling state (presentation only).
#[derive(Resource, Default)]
pub struct StudioPerformanceTelemetryState {
    pub telemetry: StudioPerformanceTelemetry,
    pub vram_dirty: bool,
    last_vram_scan_elapsed_secs: f32,
    pub main_update_sample_count: u64,
    pub frame_total_sample_count: u64,
    pub egui_pass_sample_count: u64,
    update_pass_started: Option<std::time::Instant>,
}

pub fn init_studio_performance_telemetry(mut commands: Commands) {
    commands.init_resource::<StudioPerformanceTelemetryState>();
    commands.init_resource::<StudioAntialiasingApplyState>();
}

pub fn begin_main_update_timing(mut state: ResMut<StudioPerformanceTelemetryState>) {
    state.update_pass_started = Some(std::time::Instant::now());
}

pub fn update_map_radius_falloff_context_system(
    state: Res<super::StudioAppState>,
    studio_camera: Res<super::camera::StudioCamera>,
    camera: Query<(&Camera, &Transform), With<super::camera::MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut falloff_state: ResMut<super::StudioMapRadiusFalloffState>,
    mut falloff_patch: ResMut<WorldTextFalloffRulerPatch>,
) {
    falloff_patch.falloff_mode = state.star_falloff_metric.gpu_falloff_mode();
    falloff_state.diagnostics.updated_after_camera = true;
    let Some(session) = state.session.as_ref() else {
        falloff_state.valid = false;
        return;
    };
    let Ok((camera, camera_transform)) = camera.single() else {
        falloff_state.valid = false;
        return;
    };
    let Ok(window) = windows.single() else {
        falloff_state.valid = false;
        return;
    };
    let positions: Vec<[f32; 3]> = session
        .view_model
        .render_anchors
        .iter()
        .map(|a| a.world_position)
        .collect();
    let bounds = MapPlaneBounds::from_world_positions(&positions);
    let camera_global = GlobalTransform::from(*camera_transform);
    falloff_state.context_frame = falloff_state.context_frame.saturating_add(1);
    let computed = compute_map_radius_falloff_context(
        camera,
        &camera_global,
        window.resolution.width(),
        window.resolution.height(),
        bounds,
        Some(studio_camera.orbit_target),
        0.0,
        falloff_state.context_frame,
        true,
    );
    let previous = falloff_state
        .valid
        .then_some((falloff_state.context, falloff_state.diagnostics));
    let stabilized = stabilize_map_radius_falloff_output(computed, previous);
    let context = stabilized.context;
    falloff_state.bounds = bounds;
    falloff_state.context = context;
    falloff_state.diagnostics = stabilized.diagnostics;
    falloff_state.valid = stabilized.valid;
    if !stabilized.valid {
        return;
    }
    falloff_patch.map_view_origin_x = context.view_origin.x;
    falloff_patch.map_view_origin_z = context.view_origin.y;
    falloff_patch.map_max_view_distance = context.map_max_view_distance();
}

pub fn finalize_main_update_timing(
    diagnostics: Res<DiagnosticsStore>,
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    if let Some(started) = state.update_pass_started.take() {
        state.main_update_sample_count = state.main_update_sample_count.saturating_add(1);
        let sample_ms = started.elapsed().as_secs_f64() * 1000.0;
        let count = state.main_update_sample_count;
        {
            let telemetry = &mut state.telemetry;
            record_frame_phase_timing(
                &mut telemetry.main_update_ms_last,
                &mut telemetry.main_update_ms_avg,
                sample_ms,
                count,
            );
        }
    }

    if let Some(frame_ms) = read_frame_time_ms_from_diagnostics(&diagnostics) {
        state.frame_total_sample_count = state.frame_total_sample_count.saturating_add(1);
        let count = state.frame_total_sample_count;
        let telemetry = &mut state.telemetry;
        record_frame_phase_timing(
            &mut telemetry.frame_total_ms_last,
            &mut telemetry.frame_total_ms_avg,
            frame_ms,
            count,
        );
    }
}

pub fn update_studio_window_gpu_context(
    windows: Query<&Window, With<PrimaryWindow>>,
    app_state: Res<super::StudioAppState>,
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    state.telemetry.window_width = Some(window.resolution.width() as u32);
    state.telemetry.window_height = Some(window.resolution.height() as u32);
    state.telemetry.render_scale = Some(window.resolution.scale_factor() as f32);
    state.telemetry.present_mode = Some(format_present_mode(window.present_mode));
    state.telemetry.antialiasing_mode = app_state.antialiasing_mode.label().to_string();
}

pub fn update_studio_antialiasing_video_debug_system(
    app_state: Res<super::StudioAppState>,
    apply_state: Res<StudioAntialiasingApplyState>,
    pattern_runtime: Res<crate::studio_aa_test_pattern::AaTestPatternRuntime>,
    camera: Query<(Entity, Option<&Fxaa>, Option<&Smaa>, &Msaa), With<super::camera::MainCamera>>,
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    let telemetry = &mut state.telemetry;
    let selected = app_state.antialiasing_mode;
    telemetry.antialiasing_mode = selected.label().to_string();
    telemetry.antialiasing_mode_source = app_state.antialiasing_mode_source.label().to_string();

    let snapshot = match camera.single() {
        Ok((entity, fxaa, smaa, msaa)) => {
            snapshot_from_camera_components(entity, fxaa, smaa, Some(msaa))
        }
        Err(_) => AntialiasingComponentSnapshot::default(),
    };

    telemetry.aa_primary_camera_found = snapshot.primary_camera_found;
    telemetry.aa_camera_entity_index = snapshot.camera_entity_index;
    telemetry.aa_fxaa_present = snapshot.fxaa_present;
    telemetry.aa_smaa_present = snapshot.smaa_present;
    telemetry.aa_smaa_preset = snapshot
        .smaa_preset
        .map(smaa_preset_label)
        .unwrap_or("none")
        .to_string();
    telemetry.aa_smaa_selected = selected.is_smaa();
    telemetry.aa_smaa_expected_active = smaa_expected_active(selected);
    telemetry.aa_msaa_active = msaa_active(snapshot.msaa_samples);
    telemetry.aa_msaa_sample_count = msaa_sample_count_label(snapshot.msaa_samples).to_string();
    telemetry.aa_msaa_scope = if snapshot.msaa_component_present {
        "primary Camera3d component".into()
    } else {
        "MSAA state unknown: Msaa component missing on primary Camera3d".into()
    };
    telemetry.aa_post_aa_component_active = post_aa_component_active(snapshot);
    telemetry.aa_dual_post_aa_active = dual_post_aa_components_active(snapshot);
    telemetry.aa_state_mismatch = antialiasing_component_state_mismatch(selected, snapshot);
    telemetry.aa_apply_generation = apply_state.apply_generation;
    telemetry.aa_last_applied_mode = apply_state
        .last_applied_mode
        .map(|mode| mode.label().to_string())
        .unwrap_or_else(|| "—".into());
    telemetry.aa_last_applied_frame = apply_state.last_applied_frame;
    telemetry.aa_test_pattern_visible = pattern_runtime.visible;
    telemetry.aa_test_pattern_geometry_instances = pattern_runtime.geometry_instances;
    telemetry.aa_test_pattern_material = if pattern_runtime.visible {
        pattern_runtime.material_label().to_string()
    } else {
        "—".into()
    };
}

fn format_present_mode(mode: PresentMode) -> String {
    format!("{mode:?}")
}

pub fn record_egui_pass_timing(
    state: &mut StudioPerformanceTelemetryState,
    total_ms: f64,
    settings_ms: f64,
    left_panel_ms: f64,
    right_panel_ms: f64,
) {
    state.egui_pass_sample_count = state.egui_pass_sample_count.saturating_add(1);
    let count = state.egui_pass_sample_count;
    let telemetry = &mut state.telemetry;
    record_frame_phase_timing(
        &mut telemetry.egui_pass_ms_last,
        &mut telemetry.egui_pass_ms_avg,
        total_ms,
        count,
    );
    record_frame_phase_timing(
        &mut telemetry.egui_settings_ms_last,
        &mut telemetry.egui_settings_ms_avg,
        settings_ms,
        count,
    );
    record_frame_phase_timing(
        &mut telemetry.egui_left_panel_ms_last,
        &mut telemetry.egui_left_panel_ms_avg,
        left_panel_ms,
        count,
    );
    record_frame_phase_timing(
        &mut telemetry.egui_right_panel_ms_last,
        &mut telemetry.egui_right_panel_ms_avg,
        right_panel_ms,
        count,
    );
}

pub fn update_studio_fps_telemetry(
    diagnostics: Res<DiagnosticsStore>,
    frame_count: Res<FrameCount>,
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    state.telemetry.fps = read_fps_from_diagnostics(&diagnostics);
    state.telemetry.last_update_frame = frame_count.0 as u64;
}

pub fn update_studio_vram_telemetry(
    time: Res<Time>,
    images: Res<Assets<Image>>,
    meshes: Res<Assets<Mesh>>,
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    let elapsed = time.elapsed_secs();
    let due = state.vram_dirty
        || state.last_vram_scan_elapsed_secs == 0.0
        || elapsed - state.last_vram_scan_elapsed_secs >= VRAM_ESTIMATE_INTERVAL_SECS;
    if !due {
        return;
    }

    let started = std::time::Instant::now();
    let (total, texture_bytes, mesh_bytes, buffer_bytes) =
        estimate_studio_allocated_vram_bytes(&images, &meshes);
    state.telemetry.allocated_vram_bytes_estimate = total;
    state.telemetry.allocated_vram_mb_estimate = bytes_to_vram_mb(total);
    state.telemetry.texture_bytes_estimate = texture_bytes;
    state.telemetry.mesh_bytes_estimate = mesh_bytes;
    state.telemetry.buffer_bytes_estimate = buffer_bytes;
    state.last_vram_scan_elapsed_secs = elapsed;
    state.vram_dirty = false;
    state.telemetry.vram_scan_last_ms = Some(started.elapsed().as_secs_f64() * 1000.0);
}

const NAMEPLATE_ALPHA_ZERO_THRESHOLD: f32 = 0.02;

fn nameplate_cull_reason(
    projected_height: f32,
    focused: bool,
    final_alpha: f32,
    offscreen: bool,
    debug_mode: StarNameplateDebugMode,
    unselected_global_alpha: f32,
) -> Option<String> {
    if debug_mode == StarNameplateDebugMode::ForceAllDebug {
        if final_alpha < NAMEPLATE_ALPHA_ZERO_THRESHOLD {
            return Some("alpha_zero".into());
        }
        return None;
    }
    if offscreen {
        return Some("offscreen".into());
    }
    if final_alpha < NAMEPLATE_ALPHA_ZERO_THRESHOLD {
        return Some("alpha_zero".into());
    }
    if !nameplate_label_passes_readability_gate(projected_height, focused, debug_mode) {
        return Some("too_small".into());
    }
    if !nameplate_label_passes_density_gate(focused, unselected_global_alpha, debug_mode) {
        return Some("over_density".into());
    }
    None
}

pub fn update_nameplate_diagnostics_system(
    state: Res<StudioAppState>,
    settings: Res<StudioSettings>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    nameplates: Query<
        (
            &GalaxyStarNameplate,
            &WorldTextBillboard,
            Option<&TextGlyphInstances>,
        ),
        With<GalaxyStarNameplate>,
    >,
    atlas: Option<Res<TypefaceAtlas>>,
    text_perf: Option<Res<TextPerfDiagnostics>>,
    mut telemetry_state: ResMut<StudioPerformanceTelemetryState>,
    mut lod_patch: ResMut<WorldTextNameplateLodPatch>,
    falloff_state: Res<super::StudioMapRadiusFalloffState>,
) {
    let debug_mode = state.star_nameplate_debug_mode;
    let nameplate_settings = settings.star_nameplate_settings().clamped();
    telemetry_state.telemetry.raster_atlas_gutter_px = RASTER_GLYPH_ATLAS_GUTTER_PX;
    telemetry_state.telemetry.raster_atlas_uv_inset = RASTER_GLYPH_ATLAS_UV_INSET;
    if let Some(atlas) = atlas.as_ref() {
        telemetry_state.telemetry.atlas_tile_count = atlas.cpu.tile_count();
        telemetry_state.telemetry.atlas_dirty_region_count = atlas.cpu_stats().dirty_region_count;
    }
    if let Some(perf) = text_perf.as_ref() {
        telemetry_state.telemetry.atlas_dirty_region_count = telemetry_state
            .telemetry
            .atlas_dirty_region_count
            .max(perf.atlas_dirty_region_count as usize);
    }
    telemetry_state.telemetry.nameplate_visibility_mode = debug_mode.label().into();
    telemetry_state.telemetry.nameplate_debug_override_active = debug_mode.is_debug_override();
    telemetry_state
        .telemetry
        .nameplate_settings_relative_size_pct = Some(nameplate_settings.relative_width_percent);
    telemetry_state
        .telemetry
        .nameplate_settings_base_transparency_pct =
        Some(nameplate_settings.base_transparency_percent);
    telemetry_state
        .telemetry
        .nameplate_settings_relative_falloff_distance_pct =
        Some(nameplate_settings.relative_falloff_distance_percent);
    telemetry_state
        .telemetry
        .nameplate_settings_relative_falloff_transparency_pct =
        Some(nameplate_settings.relative_falloff_transparency_percent);
    telemetry_state.telemetry.nameplate_falloff_metric = state.star_falloff_metric.label().into();
    let mut glyph_instances = 0u64;
    let mut gpu_screen_label_count = 0usize;
    let mut culled_too_small_count = 0usize;
    let mut culled_over_density_count = 0usize;
    let mut culled_alpha_zero_count = 0usize;
    let mut culled_offscreen_count = 0usize;
    let mut culled_past_effective_falloff_count = 0usize;
    let mut visible_label_estimate = 0usize;
    let mut visible_glyph_estimate = 0u64;
    let mut unselected_visible_after_lod = 0usize;
    let mut focused_visible_after_lod = 0usize;
    let mut sample_billboard = None;
    let mut sample_instance = None;
    let mut sample_run_aspect = None;
    let mut sample_label_height_px = 24.0_f32;
    let mut sample_label_width_px = 96.0_f32;

    let mut selected_sample: Option<(
        StarBillboardInstance,
        WorldTextBillboard,
        Option<Vec<GlyphInstanceGpu>>,
    )> = None;

    for (nameplate, billboard, glyphs) in &nameplates {
        if billboard.placement_mode == WorldTextPlacementMode::GpuScreenLabel {
            gpu_screen_label_count += 1;
        }
        if let Some(glyphs) = glyphs {
            glyph_instances = glyph_instances.saturating_add(glyphs.0.len() as u64);
            if sample_run_aspect.is_none() {
                sample_run_aspect = Some(natural_run_aspect_from_glyphs(&glyphs.0));
            }
        }
        if state.selection.selected_system_id == Some(nameplate.instance.system_id) {
            selected_sample = Some((nameplate.instance, *billboard, glyphs.map(|g| g.0.clone())));
        } else if selected_sample.is_none()
            && state.selection.hovered_system_id == Some(nameplate.instance.system_id)
        {
            selected_sample = Some((nameplate.instance, *billboard, glyphs.map(|g| g.0.clone())));
        }
        if sample_billboard.is_none() {
            sample_billboard = Some(*billboard);
            sample_instance = Some(nameplate.instance);
        }
    }

    telemetry_state.telemetry.nameplate_count = nameplates.iter().count();
    telemetry_state.telemetry.nameplate_glyph_instances = glyph_instances;
    telemetry_state.telemetry.nameplate_gpu_screen_label_count = gpu_screen_label_count;
    telemetry_state.telemetry.nameplate_natural_run_aspect = sample_run_aspect;

    let Some(session) = state.session.as_ref() else {
        telemetry_state
            .telemetry
            .nameplate_star_visual_envelope_world = None;
        telemetry_state
            .telemetry
            .nameplate_projected_star_visual_height_px = None;
        telemetry_state.telemetry.nameplate_label_height_px = None;
        telemetry_state.telemetry.nameplate_label_width_px = None;
        telemetry_state.telemetry.nameplate_sample_alpha = None;
        telemetry_state.telemetry.nameplate_culled_too_small_count = 0;
        telemetry_state
            .telemetry
            .nameplate_culled_over_density_count = 0;
        telemetry_state.telemetry.nameplate_culled_alpha_zero_count = 0;
        telemetry_state.telemetry.nameplate_culled_offscreen_count = 0;
        telemetry_state.telemetry.nameplate_visible_label_estimate = 0;
        telemetry_state.telemetry.nameplate_visible_glyph_estimate = 0;
        telemetry_state
            .telemetry
            .nameplate_unselected_visible_after_lod = 0;
        telemetry_state
            .telemetry
            .nameplate_focused_visible_after_lod = 0;
        telemetry_state.telemetry.nameplate_label_coverage_estimate = 0.0;
        telemetry_state.telemetry.nameplate_selected_star_id = None;
        telemetry_state.telemetry.nameplate_selected_anchor_px = None;
        telemetry_state
            .telemetry
            .nameplate_selected_projected_diameter_px = None;
        telemetry_state.telemetry.nameplate_selected_label_height_px = None;
        telemetry_state.telemetry.nameplate_selected_local_x_min = None;
        telemetry_state.telemetry.nameplate_selected_local_x_max = None;
        telemetry_state
            .telemetry
            .nameplate_selected_computed_width_px = None;
        telemetry_state.telemetry.nameplate_selected_final_alpha = None;
        telemetry_state.telemetry.nameplate_selected_cull_reason = None;
        telemetry_state.telemetry.nameplate_drawn_labels = 0;
        telemetry_state.telemetry.nameplate_focused_labels_drawn = 0;
        telemetry_state.telemetry.nameplate_glyph_instances_drawn = 0;
        telemetry_state.telemetry.nameplate_global_lod_alpha = 1.0;
        *lod_patch = {
            let lg = debug_mode.lod_globals(1.0);
            WorldTextNameplateLodPatch {
                min_focused_px: lg.min_focused_px,
                unselected_global_alpha: lg.unselected_global_alpha,
                min_unselected_px: lg.min_unselected_px,
            }
        };
        return;
    };

    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };

    let star_settings = StarBillboardRenderSettings::from_meta(&session.view_model.render_meta);
    let camera_pos = camera_transform.translation();
    let viewport_height = window.resolution.height();
    let viewport_width = window.resolution.width();
    let viewport_area_px = viewport_width * viewport_height;
    let falloff_metric = state.star_falloff_metric;
    let use_plateau = falloff_metric.uses_plateau_curve();
    let map_context = falloff_state.valid.then_some(&falloff_state.context);
    if falloff_state.valid {
        let ctx = falloff_state.context;
        let diag = falloff_state.diagnostics;
        telemetry_state.telemetry.map_falloff_view_origin =
            Some([ctx.view_origin.x, ctx.view_origin.y]);
        telemetry_state.telemetry.map_falloff_max_view_distance = Some(ctx.map_max_view_distance());
        telemetry_state.telemetry.map_falloff_origin_source = if diag.retained_previous_context {
            "retained previous valid context".into()
        } else {
            origin_source_label(ctx.origin_source).into()
        };
        telemetry_state.telemetry.map_falloff_viewport_convention = diag.viewport_convention.into();
        telemetry_state
            .telemetry
            .map_falloff_bottom_center_viewport_px = Some(diag.bottom_center_viewport_px);
        telemetry_state.telemetry.map_falloff_raw_ray_origin = Some(diag.raw_ray_origin);
        telemetry_state.telemetry.map_falloff_raw_ray_direction = Some(diag.raw_ray_direction);
        telemetry_state.telemetry.map_falloff_raw_map_plane_hit = diag.raw_map_plane_hit;
        telemetry_state.telemetry.map_falloff_origin_clamped = diag.origin_clamped;
        telemetry_state.telemetry.map_falloff_bounds_min = Some(diag.bounds_min);
        telemetry_state.telemetry.map_falloff_bounds_max = Some(diag.bounds_max);
        telemetry_state.telemetry.map_falloff_context_frame = Some(falloff_state.context_frame);
        telemetry_state.telemetry.map_falloff_updated_after_camera = diag.updated_after_camera;
        telemetry_state
            .telemetry
            .map_falloff_retained_previous_context = diag.retained_previous_context;
    } else {
        telemetry_state.telemetry.map_falloff_view_origin = None;
        telemetry_state.telemetry.map_falloff_max_view_distance = None;
        telemetry_state.telemetry.map_falloff_origin_source = "—".into();
        telemetry_state.telemetry.map_falloff_viewport_convention = "—".into();
        telemetry_state
            .telemetry
            .map_falloff_bottom_center_viewport_px = None;
        telemetry_state.telemetry.map_falloff_raw_ray_origin = None;
        telemetry_state.telemetry.map_falloff_raw_ray_direction = None;
        telemetry_state.telemetry.map_falloff_raw_map_plane_hit = None;
        telemetry_state.telemetry.map_falloff_origin_clamped = false;
        telemetry_state.telemetry.map_falloff_bounds_min = None;
        telemetry_state.telemetry.map_falloff_bounds_max = None;
        telemetry_state.telemetry.map_falloff_context_frame = None;
        telemetry_state.telemetry.map_falloff_updated_after_camera = false;
        telemetry_state
            .telemetry
            .map_falloff_retained_previous_context = false;
    }
    let horizon_ruler = VisualHorizonFalloffRuler::from_viewport(viewport_width, viewport_height);
    telemetry_state.telemetry.nameplate_falloff_ruler_base_px = Some(horizon_ruler.base_px);
    telemetry_state
        .telemetry
        .nameplate_falloff_ruler_vanishing_px = Some(horizon_ruler.vanishing_px);

    if let (Some(sample), Some(instance)) = (sample_billboard, sample_instance) {
        let distance = camera_pos.distance(instance.anchor_position);
        let progress_percent = star_falloff_progress_percent(
            falloff_metric,
            camera,
            camera_transform,
            instance.anchor_position,
            distance,
            &star_settings,
            viewport_width,
            viewport_height,
            map_context,
        );
        let height_ratio =
            star_nameplate_envelope_height_ratio(instance, &star_settings, progress_percent);
        let envelope_world = sample.visual_envelope_world_height * height_ratio;
        let projected_star_visual_height_px = estimate_world_vertical_span_screen_px(
            instance.anchor_position,
            camera_pos,
            envelope_world,
            viewport_height,
        );
        sample_label_height_px = nameplate_scaled_label_height_px(
            projected_star_visual_height_px,
            sample.width_ratio,
            false,
        );
        sample_label_width_px = sample_run_aspect.unwrap_or(1.0) * sample_label_height_px;

        let falloff_alpha =
            nameplate_gpu_screen_label_falloff_alpha(progress_percent, &sample, use_plateau);
        let star_alpha = star_falloff_alpha_at_progress(progress_percent, &sample, use_plateau);
        let sample_alpha = sample.base_alpha_ratio * falloff_alpha;
        telemetry_state.telemetry.nameplate_sample_depth_percent = Some(progress_percent);
        telemetry_state
            .telemetry
            .nameplate_sample_visual_progress_pct = Some(progress_percent);
        if let Some(ctx) = map_context {
            let star_xz = Vec2::new(instance.anchor_position.x, instance.anchor_position.z);
            let map_distance = ctx.view_origin.distance(star_xz);
            telemetry_state
                .telemetry
                .map_falloff_sample_star_map_distance = Some(map_distance);
            telemetry_state
                .telemetry
                .map_falloff_sample_star_progress_pct =
                Some(map_radius_progress_percent(ctx, star_xz));
        }
        telemetry_state
            .telemetry
            .nameplate_sample_star_falloff_alpha = Some(star_alpha);
        telemetry_state.telemetry.nameplate_sample_falloff_alpha = Some(falloff_alpha);
        telemetry_state.telemetry.nameplate_sample_screen_px = world_anchor_screen_px(
            camera,
            camera_transform,
            instance.anchor_position,
            viewport_width,
            viewport_height,
        );
        telemetry_state
            .telemetry
            .nameplate_star_visual_envelope_world = Some(envelope_world);
        telemetry_state
            .telemetry
            .nameplate_projected_star_visual_height_px = Some(projected_star_visual_height_px);
        telemetry_state.telemetry.nameplate_label_height_px = Some(sample_label_height_px);
        telemetry_state.telemetry.nameplate_label_width_px = Some(sample_label_width_px);
        telemetry_state.telemetry.nameplate_sample_alpha = Some(sample_alpha);
    } else {
        telemetry_state
            .telemetry
            .nameplate_star_visual_envelope_world = None;
        telemetry_state
            .telemetry
            .nameplate_projected_star_visual_height_px = None;
        telemetry_state.telemetry.nameplate_label_height_px = None;
        telemetry_state.telemetry.nameplate_label_width_px = None;
        telemetry_state.telemetry.nameplate_sample_alpha = None;
    }

    let star_falloff_settings = star_settings.falloff_settings();
    telemetry_state
        .telemetry
        .nameplate_star_falloff_distance_pct = Some(star_falloff_settings.falloff_distance_percent);
    let effective_falloff = nameplate_effective_falloff_distance_percent(
        star_falloff_settings.falloff_distance_percent,
        nameplate_settings.relative_falloff_distance_percent,
    );
    telemetry_state
        .telemetry
        .nameplate_effective_falloff_distance_pct = Some(effective_falloff);
    telemetry_state
        .telemetry
        .nameplate_effective_falloff_screen_y_pct =
        Some(visual_horizon_ruler_screen_y_fraction_from_top(effective_falloff) * 100.0);
    let auto_density_alpha = nameplate_unselected_global_lod_alpha(
        gpu_screen_label_count,
        sample_label_height_px,
        sample_label_width_px,
        viewport_area_px,
    );
    let lod_globals = debug_mode.lod_globals(auto_density_alpha);
    telemetry_state.telemetry.nameplate_global_lod_alpha = lod_globals.unselected_global_alpha;
    telemetry_state.telemetry.nameplate_min_unselected_label_px = lod_globals.min_unselected_px;
    telemetry_state.telemetry.nameplate_min_focused_label_px = lod_globals.min_focused_px;
    telemetry_state.telemetry.nameplate_label_coverage_estimate = gpu_screen_label_count as f32
        * sample_label_height_px.max(0.0)
        * sample_label_width_px.max(0.0)
        / viewport_area_px.max(1.0);
    *lod_patch = WorldTextNameplateLodPatch {
        min_focused_px: lod_globals.min_focused_px,
        unselected_global_alpha: lod_globals.unselected_global_alpha,
        min_unselected_px: lod_globals.min_unselected_px,
    };

    for (nameplate, billboard, glyphs) in &nameplates {
        if billboard.placement_mode != WorldTextPlacementMode::GpuScreenLabel {
            continue;
        }
        let instance = nameplate.instance;
        let focused = billboard.gpu_screen_label_focused;
        let distance = camera_pos.distance(instance.anchor_position);
        let progress_percent = star_falloff_progress_percent(
            falloff_metric,
            camera,
            camera_transform,
            instance.anchor_position,
            distance,
            &star_settings,
            viewport_width,
            viewport_height,
            map_context,
        );
        let height_ratio =
            star_nameplate_envelope_height_ratio(instance, &star_settings, progress_percent);
        let envelope_world = billboard.visual_envelope_world_height * height_ratio;
        let projected_height = estimate_world_vertical_span_screen_px(
            instance.anchor_position,
            camera_pos,
            envelope_world,
            viewport_height,
        );
        let run_aspect = glyphs
            .map(|g| natural_run_aspect_from_glyphs(&g.0))
            .unwrap_or(sample_run_aspect.unwrap_or(1.0));
        let _label_width_px = run_aspect * projected_height * billboard.width_ratio;

        let falloff_alpha =
            nameplate_gpu_screen_label_falloff_alpha(progress_percent, billboard, use_plateau);
        let final_alpha = billboard.base_alpha_ratio * falloff_alpha;

        let effective_falloff_at = billboard
            .relative_falloff_percent
            .min(billboard.ceiling_falloff_percent);
        if progress_percent > effective_falloff_at + f32::EPSILON
            && falloff_alpha < NAMEPLATE_ALPHA_ZERO_THRESHOLD
        {
            culled_past_effective_falloff_count += 1;
        }

        let offscreen = camera
            .world_to_ndc(camera_transform, instance.anchor_position)
            .is_none_or(|ndc| ndc.x.abs() > 1.0 || ndc.y.abs() > 1.0 || ndc.z > 1.0);

        let mut culled = false;
        if debug_mode != StarNameplateDebugMode::ForceAllDebug {
            if !nameplate_label_passes_readability_gate(projected_height, focused, debug_mode) {
                culled_too_small_count += 1;
                culled = true;
            }
            if !nameplate_label_passes_density_gate(
                focused,
                lod_globals.unselected_global_alpha,
                debug_mode,
            ) {
                culled_over_density_count += 1;
                culled = true;
            }
            if offscreen {
                culled_offscreen_count += 1;
                culled = true;
            }
        }
        if final_alpha < NAMEPLATE_ALPHA_ZERO_THRESHOLD {
            culled_alpha_zero_count += 1;
            culled = true;
        }

        if culled {
            continue;
        }

        visible_label_estimate += 1;
        if let Some(glyphs) = glyphs {
            visible_glyph_estimate = visible_glyph_estimate.saturating_add(glyphs.0.len() as u64);
        }
        if focused {
            focused_visible_after_lod += 1;
        } else {
            unselected_visible_after_lod += 1;
        }
    }

    telemetry_state.telemetry.nameplate_culled_too_small_count = culled_too_small_count;
    telemetry_state
        .telemetry
        .nameplate_culled_over_density_count = culled_over_density_count;
    telemetry_state.telemetry.nameplate_culled_alpha_zero_count = culled_alpha_zero_count;
    telemetry_state.telemetry.nameplate_culled_offscreen_count = culled_offscreen_count;
    telemetry_state
        .telemetry
        .nameplate_culled_past_effective_falloff_count = culled_past_effective_falloff_count;
    telemetry_state.telemetry.nameplate_visible_label_estimate = visible_label_estimate;
    telemetry_state.telemetry.nameplate_visible_glyph_estimate = visible_glyph_estimate;
    telemetry_state
        .telemetry
        .nameplate_unselected_visible_after_lod = unselected_visible_after_lod;
    telemetry_state
        .telemetry
        .nameplate_focused_visible_after_lod = focused_visible_after_lod;
    telemetry_state.telemetry.nameplate_drawn_labels = visible_label_estimate;
    telemetry_state.telemetry.nameplate_focused_labels_drawn = focused_visible_after_lod;
    telemetry_state.telemetry.nameplate_glyph_instances_drawn = visible_glyph_estimate;

    if let Some((instance, billboard, glyph_list)) = selected_sample {
        let distance = camera_pos.distance(instance.anchor_position);
        let progress_percent = star_falloff_progress_percent(
            falloff_metric,
            camera,
            camera_transform,
            instance.anchor_position,
            distance,
            &star_settings,
            viewport_width,
            viewport_height,
            map_context,
        );
        let height_ratio =
            star_nameplate_envelope_height_ratio(instance, &star_settings, progress_percent);
        let envelope_world = billboard.visual_envelope_world_height * height_ratio;
        let projected_diameter = estimate_world_vertical_span_screen_px(
            instance.anchor_position,
            camera_pos,
            envelope_world,
            viewport_height,
        );
        let effective_height =
            nameplate_scaled_label_height_px(projected_diameter, billboard.width_ratio, true);
        let (local_x_min, local_x_max) = glyph_list
            .as_ref()
            .map(|g| normalized_label_local_x_range_from_glyphs(g))
            .unwrap_or((0.0, 0.0));
        let computed_width = (local_x_max - local_x_min).max(0.0) * effective_height;
        let falloff_alpha =
            nameplate_gpu_screen_label_falloff_alpha(progress_percent, &billboard, use_plateau);
        let star_alpha = star_falloff_alpha_at_progress(progress_percent, &billboard, use_plateau);
        let final_alpha = billboard.base_alpha_ratio * falloff_alpha;
        telemetry_state.telemetry.nameplate_sample_depth_percent = Some(progress_percent);
        telemetry_state
            .telemetry
            .nameplate_sample_visual_progress_pct = Some(progress_percent);
        telemetry_state
            .telemetry
            .nameplate_sample_star_falloff_alpha = Some(star_alpha);
        telemetry_state.telemetry.nameplate_sample_falloff_alpha = Some(falloff_alpha);
        let offscreen = camera
            .world_to_ndc(camera_transform, instance.anchor_position)
            .is_none_or(|ndc| ndc.x.abs() > 1.0 || ndc.y.abs() > 1.0 || ndc.z > 1.0);
        telemetry_state.telemetry.nameplate_selected_star_id = Some(instance.system_id);
        telemetry_state.telemetry.nameplate_selected_anchor_px = world_anchor_screen_px(
            camera,
            camera_transform,
            instance.anchor_position,
            viewport_width,
            viewport_height,
        );
        telemetry_state
            .telemetry
            .nameplate_selected_projected_diameter_px = Some(projected_diameter);
        telemetry_state.telemetry.nameplate_selected_label_height_px = Some(effective_height);
        telemetry_state.telemetry.nameplate_selected_local_x_min = Some(local_x_min);
        telemetry_state.telemetry.nameplate_selected_local_x_max = Some(local_x_max);
        telemetry_state
            .telemetry
            .nameplate_selected_computed_width_px = Some(computed_width);
        telemetry_state.telemetry.nameplate_selected_final_alpha = Some(final_alpha);
        telemetry_state.telemetry.nameplate_selected_cull_reason = nameplate_cull_reason(
            projected_diameter,
            true,
            final_alpha,
            offscreen,
            debug_mode,
            lod_globals.unselected_global_alpha,
        );
    } else {
        telemetry_state.telemetry.nameplate_selected_star_id = None;
        telemetry_state.telemetry.nameplate_selected_anchor_px = None;
        telemetry_state
            .telemetry
            .nameplate_selected_projected_diameter_px = None;
        telemetry_state.telemetry.nameplate_selected_label_height_px = None;
        telemetry_state.telemetry.nameplate_selected_local_x_min = None;
        telemetry_state.telemetry.nameplate_selected_local_x_max = None;
        telemetry_state
            .telemetry
            .nameplate_selected_computed_width_px = None;
        telemetry_state.telemetry.nameplate_selected_final_alpha = None;
        telemetry_state.telemetry.nameplate_selected_cull_reason = None;
    }
}

/// Copies render-subapp adapter identity into main-world telemetry after renderer init.
pub struct StudioGpuIdentityInitPlugin;

impl Plugin for StudioGpuIdentityInitPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let adapter_info = app.get_sub_app(RenderApp).and_then(|render_sub_app| {
            render_sub_app
                .world()
                .get_resource::<RenderAdapterInfo>()
                .cloned()
        });
        let Some(adapter_info) = adapter_info else {
            return;
        };
        let Some(mut state) = app
            .world_mut()
            .get_resource_mut::<StudioPerformanceTelemetryState>()
        else {
            return;
        };
        let info = &adapter_info.0;
        state.telemetry.gpu_name = Some(info.name.clone());
        state.telemetry.gpu_backend = Some(format!("{:?}", info.backend));
        state.telemetry.gpu_vendor_id = Some(info.vendor);
        state.telemetry.gpu_device_id = Some(info.device);
        state.telemetry.gpu_device_type = Some(format!("{:?}", info.device_type));
    }
}
