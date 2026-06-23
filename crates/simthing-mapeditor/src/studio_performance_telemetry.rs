//! STUDIO-SETTINGS-PERFORMANCE-TELEMETRY-0 — live FPS and allocated VRAM estimate helpers.
//! STUDIO-RENDER-LOOP-DIRTY-GATE-0 — render-loop per-system counters and timings.
//! STUDIO-FRAME-PHASE-GPU-TELEMETRY-0 — frame-phase, GPU context, and egui timing display.

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{Assets, Image, Mesh};
use bevy::render::mesh::Indices;

/// Presentation-only Studio performance telemetry (non-authoritative).
#[derive(Debug, Clone, PartialEq)]
pub struct StudioPerformanceTelemetry {
    pub fps: Option<f64>,
    pub allocated_vram_bytes_estimate: u64,
    pub allocated_vram_mb_estimate: f64,
    pub texture_bytes_estimate: u64,
    pub mesh_bytes_estimate: u64,
    pub buffer_bytes_estimate: u64,
    pub last_update_frame: u64,
    pub gpu_name: Option<String>,
    pub gpu_backend: Option<String>,
    pub gpu_vendor_id: Option<u32>,
    pub gpu_device_id: Option<u32>,
    pub gpu_device_type: Option<String>,
    pub present_mode: Option<String>,
    pub window_width: Option<u32>,
    pub window_height: Option<u32>,
    pub render_scale: Option<f32>,

    pub frame_total_ms_last: Option<f64>,
    pub frame_total_ms_avg: Option<f64>,
    pub main_update_ms_last: Option<f64>,
    pub main_update_ms_avg: Option<f64>,
    pub egui_pass_ms_last: Option<f64>,
    pub egui_pass_ms_avg: Option<f64>,
    pub egui_settings_ms_last: Option<f64>,
    pub egui_settings_ms_avg: Option<f64>,
    pub egui_left_panel_ms_last: Option<f64>,
    pub egui_left_panel_ms_avg: Option<f64>,
    pub egui_right_panel_ms_last: Option<f64>,
    pub egui_right_panel_ms_avg: Option<f64>,

    pub render_subapp_phases_unavailable: bool,
    pub render_extract_ms_last: Option<f64>,
    pub render_prepare_ms_last: Option<f64>,
    pub render_queue_ms_last: Option<f64>,
    pub render_render_ms_last: Option<f64>,
    pub render_present_or_wait_ms_last: Option<f64>,

    pub render_frame_index: u64,

    pub hyperlane_sync_calls: u64,
    pub hyperlane_mesh_rebuilds: u64,
    pub hyperlane_mesh_rebuild_last_ms: Option<f64>,
    pub hyperlane_mesh_rebuild_avg_ms: Option<f64>,
    pub hyperlane_segments_last_count: usize,
    pub hyperlane_vertices_last_count: usize,
    pub hyperlane_indices_last_count: usize,

    pub star_visual_sync_calls: u64,
    pub star_visual_entities_last_count: usize,
    pub star_visual_sync_last_ms: Option<f64>,
    pub star_visual_sync_avg_ms: Option<f64>,

    pub billboard_sync_calls: u64,
    pub billboard_entities_last_count: usize,
    pub billboard_sync_last_ms: Option<f64>,
    pub billboard_sync_avg_ms: Option<f64>,

    pub picking_projection_calls: u64,
    pub picking_projected_anchor_count: usize,
    pub picking_projection_last_ms: Option<f64>,
    pub picking_projection_avg_ms: Option<f64>,

    pub nameplate_count: usize,
    pub nameplate_glyph_instances: u64,
    pub nameplate_effective_near_height: Option<f32>,
    pub nameplate_base_alpha_ratio: Option<f32>,
    pub nameplate_ceiling_target_alpha: Option<f32>,
    pub nameplate_relative_target_alpha: Option<f32>,

    pub vram_scan_last_ms: Option<f64>,
}

impl Default for StudioPerformanceTelemetry {
    fn default() -> Self {
        Self {
            fps: None,
            allocated_vram_bytes_estimate: 0,
            allocated_vram_mb_estimate: 0.0,
            texture_bytes_estimate: 0,
            mesh_bytes_estimate: 0,
            buffer_bytes_estimate: 0,
            last_update_frame: 0,
            gpu_name: None,
            gpu_backend: None,
            gpu_vendor_id: None,
            gpu_device_id: None,
            gpu_device_type: None,
            present_mode: None,
            window_width: None,
            window_height: None,
            render_scale: None,
            frame_total_ms_last: None,
            frame_total_ms_avg: None,
            main_update_ms_last: None,
            main_update_ms_avg: None,
            egui_pass_ms_last: None,
            egui_pass_ms_avg: None,
            egui_settings_ms_last: None,
            egui_settings_ms_avg: None,
            egui_left_panel_ms_last: None,
            egui_left_panel_ms_avg: None,
            egui_right_panel_ms_last: None,
            egui_right_panel_ms_avg: None,
            render_subapp_phases_unavailable: true,
            render_extract_ms_last: None,
            render_prepare_ms_last: None,
            render_queue_ms_last: None,
            render_render_ms_last: None,
            render_present_or_wait_ms_last: None,
            render_frame_index: 0,
            hyperlane_sync_calls: 0,
            hyperlane_mesh_rebuilds: 0,
            hyperlane_mesh_rebuild_last_ms: None,
            hyperlane_mesh_rebuild_avg_ms: None,
            hyperlane_segments_last_count: 0,
            hyperlane_vertices_last_count: 0,
            hyperlane_indices_last_count: 0,
            star_visual_sync_calls: 0,
            star_visual_entities_last_count: 0,
            star_visual_sync_last_ms: None,
            star_visual_sync_avg_ms: None,
            billboard_sync_calls: 0,
            billboard_entities_last_count: 0,
            billboard_sync_last_ms: None,
            billboard_sync_avg_ms: None,
            picking_projection_calls: 0,
            picking_projected_anchor_count: 0,
            picking_projection_last_ms: None,
            picking_projection_avg_ms: None,
            nameplate_count: 0,
            nameplate_glyph_instances: 0,
            nameplate_effective_near_height: None,
            nameplate_base_alpha_ratio: None,
            nameplate_ceiling_target_alpha: None,
            nameplate_relative_target_alpha: None,
            vram_scan_last_ms: None,
        }
    }
}

/// Read smoothed FPS from Bevy diagnostics when available.
pub fn read_fps_from_diagnostics(diagnostics: &DiagnosticsStore) -> Option<f64> {
    diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed().or_else(|| diagnostic.value()))
        .filter(|fps| fps.is_finite() && *fps > 0.0)
}

/// Format FPS for Settings UI (one decimal, or warming-up placeholder).
pub fn format_fps_label(fps: Option<f64>) -> String {
    match fps {
        Some(value) if value.is_finite() && value > 0.0 => format!("{value:.1}"),
        _ => "warming up".into(),
    }
}

/// Convert byte estimate to megabytes.
pub fn bytes_to_vram_mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

/// Format VRAM megabyte estimate for Settings UI (one decimal).
pub fn format_vram_mb_label(mb: f64) -> String {
    format!("{mb:.1}")
}

/// Estimate GPU bytes for one Bevy image/texture asset.
pub fn estimate_image_vram_bytes(image: &Image) -> u64 {
    if let Some(data) = &image.data {
        return data.len() as u64;
    }

    let descriptor = &image.texture_descriptor;
    let width = descriptor.size.width.max(1) as u64;
    let height = descriptor.size.height.max(1) as u64;
    let layers = descriptor.size.depth_or_array_layers.max(1) as u64;
    let mips = descriptor.mip_level_count.max(1);
    let block_bytes = descriptor.format.block_copy_size(None).unwrap_or(4) as u64;

    let mut total = 0u64;
    let mut level_width = width;
    let mut level_height = height;
    for _ in 0..mips {
        total = total.saturating_add(level_width * level_height * layers * block_bytes);
        level_width = (level_width / 2).max(1);
        level_height = (level_height / 2).max(1);
    }
    total
}

/// Estimate GPU bytes for one Bevy mesh asset (vertex attributes + indices).
pub fn estimate_mesh_vram_bytes(mesh: &Mesh) -> u64 {
    let mut total = 0u64;
    for (_, values) in mesh.attributes() {
        total = total.saturating_add(values.get_bytes().len() as u64);
    }
    if let Some(indices) = mesh.indices() {
        total = total.saturating_add(match indices {
            Indices::U16(values) => values.len() as u64 * 2,
            Indices::U32(values) => values.len() as u64 * 4,
        });
    }
    total
}

/// Scan visible Bevy asset stores for a Studio allocation estimate.
pub fn estimate_studio_allocated_vram_bytes(
    images: &Assets<Image>,
    meshes: &Assets<Mesh>,
) -> (u64, u64, u64, u64) {
    let texture_bytes: u64 = images
        .iter()
        .map(|(_, image)| estimate_image_vram_bytes(image))
        .sum();
    let mesh_bytes: u64 = meshes
        .iter()
        .map(|(_, mesh)| estimate_mesh_vram_bytes(mesh))
        .sum();
    let buffer_bytes = 0u64;
    let total = texture_bytes
        .saturating_add(mesh_bytes)
        .saturating_add(buffer_bytes);
    (total, texture_bytes, mesh_bytes, buffer_bytes)
}

fn format_timing_ms(value: Option<f64>) -> String {
    match value {
        Some(ms) if ms.is_finite() => format!("{ms:.2}"),
        _ => "—".into(),
    }
}

/// Build Settings-window performance section labels.
pub fn performance_settings_section_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    use crate::studio_frame_phase_gpu_telemetry::{
        frame_phase_settings_lines, gpu_context_settings_lines, performance_capture_steps_lines,
        studio_build_profile_label, studio_build_profile_warning, vram_tracked_asset_lines,
    };

    let mut lines = vec![
        "Performance".into(),
        format!("FPS: {}", format_fps_label(telemetry.fps)),
        format!("Build: {}", studio_build_profile_label()),
    ];
    if let Some(warning) = studio_build_profile_warning() {
        lines.push(warning.into());
    }
    lines.extend(vram_tracked_asset_lines(telemetry));
    lines.extend(gpu_context_settings_lines(telemetry));
    lines.extend(frame_phase_settings_lines(telemetry));
    lines.extend(render_loop_diagnostics_lines(telemetry));
    lines.extend(performance_capture_steps_lines());
    lines
}

/// Render-loop diagnostics subsection for Settings Performance area.
pub fn render_loop_diagnostics_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    vec![
        "Render loop diagnostics".into(),
        format!(
            "Hyperlane rebuild: {} ms / {} ms, rebuilds: {}",
            format_timing_ms(telemetry.hyperlane_mesh_rebuild_last_ms),
            format_timing_ms(telemetry.hyperlane_mesh_rebuild_avg_ms),
            telemetry.hyperlane_mesh_rebuilds,
        ),
        format!(
            "Hyperlane geometry: {} lanes, {} verts, {} indices",
            telemetry.hyperlane_segments_last_count,
            telemetry.hyperlane_vertices_last_count,
            telemetry.hyperlane_indices_last_count,
        ),
        format!(
            "Star visual sync: {} ms / {} ms, entities: {}",
            format_timing_ms(telemetry.star_visual_sync_last_ms),
            format_timing_ms(telemetry.star_visual_sync_avg_ms),
            telemetry.star_visual_entities_last_count,
        ),
        format!(
            "Billboard sync: {} ms / {} ms, entities: {}",
            format_timing_ms(telemetry.billboard_sync_last_ms),
            format_timing_ms(telemetry.billboard_sync_avg_ms),
            telemetry.billboard_entities_last_count,
        ),
        format!(
            "Picking projection: {} ms / {} ms, anchors: {}",
            format_timing_ms(telemetry.picking_projection_last_ms),
            format_timing_ms(telemetry.picking_projection_avg_ms),
            telemetry.picking_projected_anchor_count,
        ),
        format!(
            "Nameplates: {} labels, {} glyph instances",
            telemetry.nameplate_count, telemetry.nameplate_glyph_instances,
        ),
        format!(
            "Nameplate sample: height {:.3}, base alpha {:.2}, ceiling {:.2}, falloff {:.2}",
            telemetry
                .nameplate_effective_near_height
                .map(|v| format!("{v:.3}"))
                .unwrap_or_else(|| "—".into()),
            telemetry
                .nameplate_base_alpha_ratio
                .map(|v| format!("{v:.2}"))
                .unwrap_or_else(|| "—".into()),
            telemetry
                .nameplate_ceiling_target_alpha
                .map(|v| format!("{v:.2}"))
                .unwrap_or_else(|| "—".into()),
            telemetry
                .nameplate_relative_target_alpha
                .map(|v| format!("{v:.2}"))
                .unwrap_or_else(|| "—".into()),
        ),
        format!(
            "VRAM scan: {} ms",
            format_timing_ms(telemetry.vram_scan_last_ms),
        ),
    ]
}
