//! Bevy systems for Studio Settings performance telemetry.

use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameCount};
use bevy::prelude::*;
use bevy::render::{renderer::RenderAdapterInfo, RenderApp};
use bevy::window::{PresentMode, PrimaryWindow};
use simthing_tools::{TextGlyphInstances, WorldTextBillboard};

use crate::studio_frame_phase_gpu_telemetry::{
    read_frame_time_ms_from_diagnostics, record_frame_phase_timing,
};
use crate::studio_performance_telemetry::{
    bytes_to_vram_mb, estimate_studio_allocated_vram_bytes, read_fps_from_diagnostics,
    StudioPerformanceTelemetry,
};

use super::galaxy_render::GalaxyStarNameplate;

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
    nameplates: Query<
        (&WorldTextBillboard, Option<&TextGlyphInstances>),
        With<GalaxyStarNameplate>,
    >,
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    let mut glyph_instances = 0u64;
    let mut sample_billboard = None;
    for (billboard, glyphs) in &nameplates {
        if let Some(glyphs) = glyphs {
            glyph_instances = glyph_instances.saturating_add(glyphs.0.len() as u64);
        }
        if sample_billboard.is_none() {
            sample_billboard = Some(*billboard);
        }
    }
    state.telemetry.nameplate_count = nameplates.iter().count();
    state.telemetry.nameplate_glyph_instances = glyph_instances;
    if let Some(sample) = sample_billboard {
        state.telemetry.nameplate_effective_near_height = Some(sample.near_height);
        state.telemetry.nameplate_base_alpha_ratio = Some(sample.base_alpha_ratio);
        state.telemetry.nameplate_ceiling_target_alpha = Some(sample.ceiling_target_alpha);
        state.telemetry.nameplate_relative_target_alpha = Some(sample.relative_target_alpha);
    } else {
        state.telemetry.nameplate_effective_near_height = None;
        state.telemetry.nameplate_base_alpha_ratio = None;
        state.telemetry.nameplate_ceiling_target_alpha = None;
        state.telemetry.nameplate_relative_target_alpha = None;
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
