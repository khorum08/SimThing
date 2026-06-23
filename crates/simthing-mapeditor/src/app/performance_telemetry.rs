//! Bevy systems for Studio Settings performance telemetry.

use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameCount};
use bevy::prelude::*;
use bevy::render::{renderer::RenderAdapterInfo, RenderApp};
use bevy::window::{PresentMode, PrimaryWindow, Window};
use simthing_tools::{
    natural_run_aspect_from_glyphs, TextGlyphInstances, WorldTextBillboard,
    WorldTextNameplateLodPatch, WorldTextPlacementMode,
};

use crate::star_render::{
    compute_star_falloff_visual, estimate_world_vertical_span_screen_px,
    nameplate_label_passes_density_gate, nameplate_label_passes_readability_gate,
    nameplate_unselected_global_lod_alpha, normalized_billboard_camera_depth_percent,
    star_nameplate_envelope_height_ratio, StarBillboardRenderSettings, StarNameplateDebugMode,
    MIN_FOCUSED_LABEL_HEIGHT_PX, MIN_UNSELECTED_LABEL_HEIGHT_PX,
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
}

pub fn begin_main_update_timing(mut state: ResMut<StudioPerformanceTelemetryState>) {
    state.update_pass_started = Some(std::time::Instant::now());
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
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    state.telemetry.window_width = Some(window.resolution.width() as u32);
    state.telemetry.window_height = Some(window.resolution.height() as u32);
    state.telemetry.render_scale = Some(window.resolution.scale_factor() as f32);
    state.telemetry.present_mode = Some(format_present_mode(window.present_mode));
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

pub fn update_nameplate_diagnostics_system(
    state: Res<StudioAppState>,
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
    mut telemetry_state: ResMut<StudioPerformanceTelemetryState>,
    mut lod_patch: ResMut<WorldTextNameplateLodPatch>,
) {
    let debug_mode = state.star_nameplate_debug_mode;
    let mut glyph_instances = 0u64;
    let mut screen_companion_count = 0usize;
    let mut culled_too_small_count = 0usize;
    let mut culled_over_density_count = 0usize;
    let mut culled_alpha_zero_count = 0usize;
    let mut culled_offscreen_count = 0usize;
    let mut visible_label_estimate = 0usize;
    let mut visible_glyph_estimate = 0u64;
    let mut unselected_visible_after_lod = 0usize;
    let mut focused_visible_after_lod = 0usize;
    let mut sample_billboard = None;
    let mut sample_instance = None;
    let mut sample_run_aspect = None;
    let mut sample_label_height_px = 24.0_f32;
    let mut sample_label_width_px = 96.0_f32;

    for (nameplate, billboard, glyphs) in &nameplates {
        if billboard.placement_mode == WorldTextPlacementMode::ScreenCompanion {
            screen_companion_count += 1;
        }
        if let Some(glyphs) = glyphs {
            glyph_instances = glyph_instances.saturating_add(glyphs.0.len() as u64);
            if sample_run_aspect.is_none() {
                sample_run_aspect = Some(natural_run_aspect_from_glyphs(&glyphs.0));
            }
        }
        if sample_billboard.is_none() {
            sample_billboard = Some(*billboard);
            sample_instance = Some(nameplate.instance);
        }
    }

    telemetry_state.telemetry.nameplate_count = nameplates.iter().count();
    telemetry_state.telemetry.nameplate_glyph_instances = glyph_instances;
    telemetry_state.telemetry.nameplate_screen_companion_count = screen_companion_count;
    telemetry_state.telemetry.nameplate_natural_run_aspect = sample_run_aspect;
    telemetry_state.telemetry.nameplate_min_unselected_label_px = MIN_UNSELECTED_LABEL_HEIGHT_PX;
    telemetry_state.telemetry.nameplate_min_focused_label_px = MIN_FOCUSED_LABEL_HEIGHT_PX;

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

    if let (Some(sample), Some(instance)) = (sample_billboard, sample_instance) {
        let distance = camera_pos.distance(instance.anchor_position);
        let depth_percent = normalized_billboard_camera_depth_percent(distance, &star_settings);
        let height_ratio =
            star_nameplate_envelope_height_ratio(instance, &star_settings, depth_percent);
        let envelope_world = sample.visual_envelope_world_height * height_ratio;
        let projected_star_visual_height_px = estimate_world_vertical_span_screen_px(
            instance.anchor_position,
            camera_pos,
            envelope_world,
            viewport_height,
        );
        sample_label_height_px = projected_star_visual_height_px;
        sample_label_width_px =
            sample_run_aspect.unwrap_or(1.0) * sample_label_height_px * sample.width_ratio;

        let star_falloff =
            compute_star_falloff_visual(depth_percent, star_settings.falloff_settings());
        let sample_alpha = sample.base_alpha_ratio
            * star_falloff.opacity.clamp(0.0, 1.0)
            * sample.relative_target_alpha.max(0.0).min(1.0);

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

    let auto_density_alpha = nameplate_unselected_global_lod_alpha(
        screen_companion_count,
        sample_label_height_px,
        sample_label_width_px,
        viewport_area_px,
    );
    let lod_globals = debug_mode.lod_globals(auto_density_alpha);
    telemetry_state.telemetry.nameplate_global_lod_alpha = lod_globals.unselected_global_alpha;
    telemetry_state.telemetry.nameplate_label_coverage_estimate = screen_companion_count as f32
        * sample_label_height_px.max(0.0)
        * sample_label_width_px.max(0.0)
        / viewport_area_px.max(1.0);
    *lod_patch = WorldTextNameplateLodPatch {
        min_focused_px: lod_globals.min_focused_px,
        unselected_global_alpha: lod_globals.unselected_global_alpha,
        min_unselected_px: lod_globals.min_unselected_px,
    };

    for (nameplate, billboard, glyphs) in &nameplates {
        if billboard.placement_mode != WorldTextPlacementMode::ScreenCompanion {
            continue;
        }
        let instance = nameplate.instance;
        let focused = billboard.screen_companion_focused;
        let distance = camera_pos.distance(instance.anchor_position);
        let depth_percent = normalized_billboard_camera_depth_percent(distance, &star_settings);
        let height_ratio =
            star_nameplate_envelope_height_ratio(instance, &star_settings, depth_percent);
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

        let star_falloff =
            compute_star_falloff_visual(depth_percent, star_settings.falloff_settings());
        let final_alpha = billboard.base_alpha_ratio
            * star_falloff.opacity.clamp(0.0, 1.0)
            * billboard.relative_target_alpha.max(0.0).min(1.0);

        let offscreen = camera
            .world_to_ndc(camera_transform, instance.anchor_position)
            .is_none_or(|ndc| ndc.x.abs() > 1.0 || ndc.y.abs() > 1.0 || ndc.z > 1.0);

        let mut culled = false;
        if debug_mode != StarNameplateDebugMode::ForceAll {
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
            if final_alpha < 0.01 {
                culled_alpha_zero_count += 1;
                culled = true;
            }
            if offscreen {
                culled_offscreen_count += 1;
                culled = true;
            }
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
    telemetry_state.telemetry.nameplate_visible_label_estimate = visible_label_estimate;
    telemetry_state.telemetry.nameplate_visible_glyph_estimate = visible_glyph_estimate;
    telemetry_state
        .telemetry
        .nameplate_unselected_visible_after_lod = unselected_visible_after_lod;
    telemetry_state
        .telemetry
        .nameplate_focused_visible_after_lod = focused_visible_after_lod;
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
