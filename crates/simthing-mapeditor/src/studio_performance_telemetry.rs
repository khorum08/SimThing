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
    pub antialiasing_mode: String,
    pub antialiasing_mode_source: String,
    pub aa_primary_camera_found: bool,
    pub aa_camera_entity_index: Option<u32>,
    pub aa_fxaa_present: bool,
    pub aa_smaa_present: bool,
    pub aa_smaa_preset: String,
    pub aa_dual_post_aa_active: bool,
    pub aa_state_mismatch: bool,
    pub aa_apply_generation: u64,
    pub aa_last_applied_mode: String,
    pub aa_last_applied_frame: u64,

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
    pub hyperlane_rebuild_count: u64,
    pub hyperlane_mesh_rebuilds: u64,
    pub hyperlane_mesh_rebuild_last_ms: Option<f64>,
    pub hyperlane_mesh_rebuild_avg_ms: Option<f64>,
    pub hyperlane_segments_last_count: usize,
    pub hyperlane_vertices_last_count: usize,
    pub hyperlane_indices_last_count: usize,
    pub hyperlane_last_camera_key: String,
    pub hyperlane_current_camera_key: String,
    pub hyperlane_camera_right: [f32; 3],
    pub hyperlane_camera_up: [f32; 3],
    pub hyperlane_camera_forward: [f32; 3],
    pub hyperlane_view_mode: u8,
    pub hyperlane_source_segment_count: usize,
    pub hyperlane_bucket_segment_count: [usize; 3],
    pub hyperlane_bucket_vertex_count: [usize; 3],
    pub hyperlane_bucket_index_count: [usize; 3],
    pub hyperlane_degenerate_width_dir_count: u32,
    pub hyperlane_nan_inf_vertex_count: u32,
    pub hyperlane_zero_length_segment_count: u32,
    pub hyperlane_invalid_rebuild_rejected: u64,
    pub hyperlane_mesh_build_camera_right: [f32; 3],
    pub hyperlane_mesh_build_camera_up: [f32; 3],
    pub hyperlane_mesh_build_camera_forward: [f32; 3],
    pub hyperlane_mesh_build_camera_key: String,
    pub hyperlane_basis_mismatch_right_deg: f32,
    pub hyperlane_basis_mismatch_up_deg: f32,
    pub hyperlane_basis_mismatch_forward_deg: f32,
    pub hyperlane_frames_since_rebuild: u64,
    pub hyperlane_rmb_orbit_active: bool,
    pub hyperlane_rotation_delta_since_rebuild_deg: f32,
    pub hyperlane_stale_basis_rebuild_count: u64,
    pub hyperlane_basis_mismatch_active: bool,

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
    pub nameplate_gpu_screen_label_count: usize,
    pub nameplate_drawn_labels: usize,
    pub nameplate_focused_labels_drawn: usize,
    pub nameplate_glyph_instances_drawn: u64,
    pub nameplate_natural_run_aspect: Option<f32>,
    pub nameplate_star_visual_envelope_world: Option<f32>,
    pub nameplate_projected_star_visual_height_px: Option<f32>,
    pub nameplate_label_height_px: Option<f32>,
    pub nameplate_label_width_px: Option<f32>,
    pub nameplate_sample_alpha: Option<f32>,
    pub nameplate_culled_too_small_count: usize,
    pub nameplate_visible_label_estimate: usize,
    pub nameplate_visible_glyph_estimate: u64,
    pub nameplate_unselected_visible_after_lod: usize,
    pub nameplate_focused_visible_after_lod: usize,
    pub nameplate_min_unselected_label_px: f32,
    pub nameplate_min_focused_label_px: f32,
    pub nameplate_label_coverage_estimate: f32,
    pub nameplate_global_lod_alpha: f32,
    pub nameplate_culled_over_density_count: usize,
    pub nameplate_culled_alpha_zero_count: usize,
    pub nameplate_culled_offscreen_count: usize,
    pub nameplate_selected_star_id: Option<u32>,
    pub nameplate_selected_anchor_px: Option<[f32; 2]>,
    pub nameplate_selected_projected_diameter_px: Option<f32>,
    pub nameplate_selected_label_height_px: Option<f32>,
    pub nameplate_selected_local_x_min: Option<f32>,
    pub nameplate_selected_local_x_max: Option<f32>,
    pub nameplate_selected_computed_width_px: Option<f32>,
    pub nameplate_selected_final_alpha: Option<f32>,
    pub nameplate_selected_cull_reason: Option<String>,
    pub nameplate_visibility_mode: String,
    pub nameplate_settings_relative_size_pct: Option<f32>,
    pub nameplate_settings_base_transparency_pct: Option<f32>,
    pub nameplate_settings_relative_falloff_distance_pct: Option<f32>,
    pub nameplate_settings_relative_falloff_transparency_pct: Option<f32>,
    pub nameplate_debug_override_active: bool,
    pub nameplate_star_falloff_distance_pct: Option<f32>,
    pub nameplate_effective_falloff_distance_pct: Option<f32>,
    pub nameplate_effective_falloff_screen_y_pct: Option<f32>,
    pub nameplate_sample_depth_percent: Option<f32>,
    pub nameplate_sample_falloff_alpha: Option<f32>,
    pub nameplate_culled_past_effective_falloff_count: usize,
    pub nameplate_falloff_metric: String,
    pub map_falloff_view_origin: Option<[f32; 2]>,
    pub map_falloff_max_view_distance: Option<f32>,
    pub map_falloff_origin_source: String,
    pub map_falloff_viewport_convention: String,
    pub map_falloff_bottom_center_viewport_px: Option<[f32; 2]>,
    pub map_falloff_raw_ray_origin: Option<[f32; 3]>,
    pub map_falloff_raw_ray_direction: Option<[f32; 3]>,
    pub map_falloff_raw_map_plane_hit: Option<[f32; 2]>,
    pub map_falloff_origin_clamped: bool,
    pub map_falloff_bounds_min: Option<[f32; 2]>,
    pub map_falloff_bounds_max: Option<[f32; 2]>,
    pub map_falloff_sample_star_map_distance: Option<f32>,
    pub map_falloff_sample_star_progress_pct: Option<f32>,
    pub map_falloff_context_frame: Option<u64>,
    pub map_falloff_updated_after_camera: bool,
    pub map_falloff_retained_previous_context: bool,
    pub nameplate_falloff_ruler_base_px: Option<[f32; 2]>,
    pub nameplate_falloff_ruler_vanishing_px: Option<[f32; 2]>,
    pub nameplate_sample_screen_px: Option<[f32; 2]>,
    pub nameplate_sample_visual_progress_pct: Option<f32>,
    pub nameplate_sample_star_falloff_alpha: Option<f32>,

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
            antialiasing_mode: "Off".into(),
            antialiasing_mode_source: "default fallback".into(),
            aa_primary_camera_found: false,
            aa_camera_entity_index: None,
            aa_fxaa_present: false,
            aa_smaa_present: false,
            aa_smaa_preset: "none".into(),
            aa_dual_post_aa_active: false,
            aa_state_mismatch: false,
            aa_apply_generation: 0,
            aa_last_applied_mode: "—".into(),
            aa_last_applied_frame: 0,
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
            hyperlane_rebuild_count: 0,
            hyperlane_mesh_rebuilds: 0,
            hyperlane_mesh_rebuild_last_ms: None,
            hyperlane_mesh_rebuild_avg_ms: None,
            hyperlane_segments_last_count: 0,
            hyperlane_vertices_last_count: 0,
            hyperlane_indices_last_count: 0,
            hyperlane_last_camera_key: "—".into(),
            hyperlane_current_camera_key: "—".into(),
            hyperlane_camera_right: [0.0; 3],
            hyperlane_camera_up: [0.0; 3],
            hyperlane_camera_forward: [0.0, 0.0, -1.0],
            hyperlane_view_mode: 0,
            hyperlane_source_segment_count: 0,
            hyperlane_bucket_segment_count: [0; 3],
            hyperlane_bucket_vertex_count: [0; 3],
            hyperlane_bucket_index_count: [0; 3],
            hyperlane_degenerate_width_dir_count: 0,
            hyperlane_nan_inf_vertex_count: 0,
            hyperlane_zero_length_segment_count: 0,
            hyperlane_invalid_rebuild_rejected: 0,
            hyperlane_mesh_build_camera_right: [f32::NAN; 3],
            hyperlane_mesh_build_camera_up: [f32::NAN; 3],
            hyperlane_mesh_build_camera_forward: [f32::NAN; 3],
            hyperlane_mesh_build_camera_key: "—".into(),
            hyperlane_basis_mismatch_right_deg: 0.0,
            hyperlane_basis_mismatch_up_deg: 0.0,
            hyperlane_basis_mismatch_forward_deg: 0.0,
            hyperlane_frames_since_rebuild: 0,
            hyperlane_rmb_orbit_active: false,
            hyperlane_rotation_delta_since_rebuild_deg: 0.0,
            hyperlane_stale_basis_rebuild_count: 0,
            hyperlane_basis_mismatch_active: false,
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
            nameplate_gpu_screen_label_count: 0,
            nameplate_drawn_labels: 0,
            nameplate_focused_labels_drawn: 0,
            nameplate_glyph_instances_drawn: 0,
            nameplate_natural_run_aspect: None,
            nameplate_star_visual_envelope_world: None,
            nameplate_projected_star_visual_height_px: None,
            nameplate_label_height_px: None,
            nameplate_label_width_px: None,
            nameplate_sample_alpha: None,
            nameplate_culled_too_small_count: 0,
            nameplate_visible_label_estimate: 0,
            nameplate_visible_glyph_estimate: 0,
            nameplate_unselected_visible_after_lod: 0,
            nameplate_focused_visible_after_lod: 0,
            nameplate_min_unselected_label_px: 0.0,
            nameplate_min_focused_label_px: 0.0,
            nameplate_label_coverage_estimate: 0.0,
            nameplate_global_lod_alpha: 1.0,
            nameplate_culled_over_density_count: 0,
            nameplate_culled_alpha_zero_count: 0,
            nameplate_culled_offscreen_count: 0,
            nameplate_selected_star_id: None,
            nameplate_selected_anchor_px: None,
            nameplate_selected_projected_diameter_px: None,
            nameplate_selected_label_height_px: None,
            nameplate_selected_local_x_min: None,
            nameplate_selected_local_x_max: None,
            nameplate_selected_computed_width_px: None,
            nameplate_selected_final_alpha: None,
            nameplate_selected_cull_reason: None,
            nameplate_visibility_mode: "All labels — settings driven".into(),
            nameplate_settings_relative_size_pct: None,
            nameplate_settings_base_transparency_pct: None,
            nameplate_settings_relative_falloff_distance_pct: None,
            nameplate_settings_relative_falloff_transparency_pct: None,
            nameplate_debug_override_active: false,
            nameplate_star_falloff_distance_pct: None,
            nameplate_effective_falloff_distance_pct: None,
            nameplate_effective_falloff_screen_y_pct: None,
            nameplate_sample_depth_percent: None,
            nameplate_sample_falloff_alpha: None,
            nameplate_culled_past_effective_falloff_count: 0,
            nameplate_falloff_metric: "Map radius plateau".into(),
            map_falloff_view_origin: None,
            map_falloff_max_view_distance: None,
            map_falloff_origin_source: "—".into(),
            map_falloff_viewport_convention: "—".into(),
            map_falloff_bottom_center_viewport_px: None,
            map_falloff_raw_ray_origin: None,
            map_falloff_raw_ray_direction: None,
            map_falloff_raw_map_plane_hit: None,
            map_falloff_origin_clamped: false,
            map_falloff_bounds_min: None,
            map_falloff_bounds_max: None,
            map_falloff_sample_star_map_distance: None,
            map_falloff_sample_star_progress_pct: None,
            map_falloff_context_frame: None,
            map_falloff_updated_after_camera: false,
            map_falloff_retained_previous_context: false,
            nameplate_falloff_ruler_base_px: None,
            nameplate_falloff_ruler_vanishing_px: None,
            nameplate_sample_screen_px: None,
            nameplate_sample_visual_progress_pct: None,
            nameplate_sample_star_falloff_alpha: None,
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

/// Nameplate debug subsection for the Telemetry dialog.
pub fn nameplate_debug_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    let mut lines = vec![
        "Renderer: simthing-tools GPU screen-label".into(),
        format!("Visibility mode: {}", telemetry.nameplate_visibility_mode),
    ];
    if telemetry.nameplate_debug_override_active {
        lines.push(
            "DEBUG OVERRIDE ACTIVE: debug nameplate mode bypasses normal settings-driven visibility."
                .into(),
        );
    }
    lines.push(format!(
        "Settings: relative size {}% | base transparency {}% | label falloff distance {}% | label falloff transparency {}%",
        telemetry
            .nameplate_settings_relative_size_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_settings_base_transparency_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_settings_relative_falloff_distance_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_settings_relative_falloff_transparency_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
    ));
    lines.push(format!(
        "Falloff metric: {}",
        telemetry.nameplate_falloff_metric,
    ));
    lines.push(format!(
        "View origin: {} | map max view distance: {} | origin source: {}",
        telemetry
            .map_falloff_view_origin
            .map(|[x, y]| format!("({x:.1}, {y:.1})"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .map_falloff_max_view_distance
            .map(|d| format!("{d:.1}"))
            .unwrap_or_else(|| "—".into()),
        telemetry.map_falloff_origin_source,
    ));
    lines.push(format!(
        "Star falloff plateau %: {} | effective nameplate plateau %: {}",
        telemetry
            .nameplate_star_falloff_distance_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_effective_falloff_distance_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
    ));
    lines.push(format!(
        "Ruler base px: {} | ruler vanishing px: {}",
        telemetry
            .nameplate_falloff_ruler_base_px
            .map(|p| format!("[{:.0}, {:.0}]", p[0], p[1]))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_falloff_ruler_vanishing_px
            .map(|p| format!("[{:.0}, {:.0}]", p[0], p[1]))
            .unwrap_or_else(|| "—".into()),
    ));
    lines.push(format!(
        "Star falloff %: {} | nameplate relative falloff %: {} | effective nameplate falloff %: {}",
        telemetry
            .nameplate_star_falloff_distance_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_settings_relative_falloff_distance_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_effective_falloff_distance_pct
            .map(|v| format!("{v:.1}"))
            .unwrap_or_else(|| "—".into()),
    ));
    lines.push(format!(
        "Effective nameplate falloff screen y %: {} | sample visual progress %: {} | sample final alpha: {}",
        telemetry
            .nameplate_effective_falloff_screen_y_pct
            .map(|v| format!("{v:.2}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_sample_visual_progress_pct
            .or(telemetry.nameplate_sample_depth_percent)
            .map(|v| format!("{v:.1}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_selected_final_alpha
            .or(telemetry.nameplate_sample_alpha)
            .map(|v| format!("{v:.2}"))
            .unwrap_or_else(|| "—".into()),
    ));
    lines.push(format!(
        "Sample screen px: {} | relative falloff transparency: {}% | sample star alpha: {} | sample falloff alpha: {}",
        telemetry
            .nameplate_sample_screen_px
            .or(telemetry.nameplate_selected_anchor_px)
            .map(|p| format!("[{:.0}, {:.0}]", p[0], p[1]))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_settings_relative_falloff_transparency_pct
            .map(|v| format!("{v:.0}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_sample_star_falloff_alpha
            .map(|v| format!("{v:.2}"))
            .unwrap_or_else(|| "—".into()),
        telemetry
            .nameplate_sample_falloff_alpha
            .map(|v| format!("{v:.2}"))
            .unwrap_or_else(|| "—".into()),
    ));
    lines.push(format!(
        "LOD patch: min unselected px {:.0} | unselected global alpha {:.2} | debug override: {}",
        telemetry.nameplate_min_unselected_label_px,
        telemetry.nameplate_global_lod_alpha,
        if telemetry.nameplate_debug_override_active {
            "yes"
        } else {
            "no"
        },
    ));
    lines.extend([
        format!(
            "Candidate labels: {} | drawn labels: {} | focused drawn: {}",
            telemetry.nameplate_count,
            telemetry.nameplate_drawn_labels,
            telemetry.nameplate_focused_labels_drawn,
        ),
        format!(
            "Glyph instances: {} | drawn: {} | GPU_SCREEN_LABEL: {}",
            telemetry.nameplate_glyph_instances,
            telemetry.nameplate_glyph_instances_drawn,
            telemetry.nameplate_gpu_screen_label_count,
        ),
        format!(
            "Culled LOD/readability: {} | culled falloff/alpha: {} | offscreen: {} | past effective falloff: {}",
            telemetry.nameplate_culled_too_small_count
                + telemetry.nameplate_culled_over_density_count,
            telemetry.nameplate_culled_alpha_zero_count,
            telemetry.nameplate_culled_offscreen_count,
            telemetry.nameplate_culled_past_effective_falloff_count,
        ),
        format!(
            "Nameplate relative size %: {}",
            telemetry
                .nameplate_settings_relative_size_pct
                .map(|v| format!("{v:.0}"))
                .unwrap_or_else(|| "—".into()),
        ),
        format!(
            "Sample label height px: {}",
            telemetry
                .nameplate_selected_label_height_px
                .or(telemetry.nameplate_label_height_px)
                .map(|v| format!("{v:.1}"))
                .unwrap_or_else(|| "—".into()),
        ),
        format!(
            "Sample natural run aspect: {}",
            telemetry
                .nameplate_natural_run_aspect
                .map(|v| format!("{v:.2}"))
                .unwrap_or_else(|| "—".into()),
        ),
        format!(
            "Sample computed width px: {}",
            telemetry
                .nameplate_selected_computed_width_px
                .or(telemetry.nameplate_label_width_px)
                .map(|v| format!("{v:.1}"))
                .unwrap_or_else(|| "—".into()),
        ),
        format!(
            "Sample projected visual diameter px: {}",
            telemetry
                .nameplate_selected_projected_diameter_px
                .or(telemetry.nameplate_projected_star_visual_height_px)
                .map(|v| format!("{v:.1}"))
                .unwrap_or_else(|| "—".into()),
        ),
        format!(
            "Sample base transparency: {}%",
            telemetry
                .nameplate_settings_base_transparency_pct
                .map(|v| format!("{v:.0}"))
                .unwrap_or_else(|| "—".into()),
        ),
        format!(
            "Readability floor active (unselected/focused px): {:.0} / {:.0}",
            telemetry.nameplate_min_unselected_label_px, telemetry.nameplate_min_focused_label_px,
        ),
    ]);
    if telemetry.nameplate_selected_star_id.is_some() {
        lines.push(format!(
            "Selected star id: {} | anchor px: {} | cull: {}",
            telemetry
                .nameplate_selected_star_id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "—".into()),
            telemetry
                .nameplate_selected_anchor_px
                .map(|[x, y]| format!("({x:.1}, {y:.1})"))
                .unwrap_or_else(|| "—".into()),
            telemetry
                .nameplate_selected_cull_reason
                .clone()
                .unwrap_or_else(|| "—".into()),
        ));
    }
    lines
}

/// Map-radius falloff origin debug subsection for the Telemetry dialog.
pub fn falloff_debug_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    vec![
        format!("Falloff metric: {}", telemetry.nameplate_falloff_metric),
        format!(
            "Viewport coordinate convention: {}",
            telemetry.map_falloff_viewport_convention
        ),
        format!(
            "Bottom-center viewport px: {}",
            telemetry
                .map_falloff_bottom_center_viewport_px
                .map(|[x, y]| format!("({x:.1}, {y:.1})"))
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Raw bottom-center ray origin: {}",
            telemetry
                .map_falloff_raw_ray_origin
                .map(|[x, y, z]| format!("({x:.2}, {y:.2}, {z:.2})"))
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Raw bottom-center ray direction: {}",
            telemetry
                .map_falloff_raw_ray_direction
                .map(|[x, y, z]| format!("({x:.3}, {y:.3}, {z:.3})"))
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Raw map-plane hit x/z: {}",
            telemetry
                .map_falloff_raw_map_plane_hit
                .map(|[x, z]| format!("({x:.1}, {z:.1})"))
                .unwrap_or_else(|| "—".into())
        ),
        format!("Origin source: {}", telemetry.map_falloff_origin_source),
        format!(
            "Origin clamped: {}",
            if telemetry.map_falloff_origin_clamped {
                "yes"
            } else {
                "no"
            }
        ),
        format!(
            "Final view origin x/z: {}",
            telemetry
                .map_falloff_view_origin
                .map(|[x, z]| format!("({x:.1}, {z:.1})"))
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Map bounds min/max x/z: {} / {}",
            telemetry
                .map_falloff_bounds_min
                .map(|[x, z]| format!("({x:.1}, {z:.1})"))
                .unwrap_or_else(|| "—".into()),
            telemetry
                .map_falloff_bounds_max
                .map(|[x, z]| format!("({x:.1}, {z:.1})"))
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Map max view distance: {}",
            telemetry
                .map_falloff_max_view_distance
                .map(|d| format!("{d:.1}"))
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Sample star map distance: {}",
            telemetry
                .map_falloff_sample_star_map_distance
                .map(|d| format!("{d:.1}"))
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Sample star progress %: {}",
            telemetry
                .map_falloff_sample_star_progress_pct
                .map(|p| format!("{p:.1}"))
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Context frame: {}",
            telemetry
                .map_falloff_context_frame
                .map(|f| f.to_string())
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "Context updated after camera: {}",
            if telemetry.map_falloff_updated_after_camera {
                "yes"
            } else {
                "no"
            }
        ),
        format!(
            "Retained previous valid context: {}",
            if telemetry.map_falloff_retained_previous_context {
                "yes"
            } else {
                "no"
            }
        ),
    ]
}

/// Video Options Debug subsection — selected AA mode vs active Camera3d components.
pub fn video_options_debug_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    let mut lines = vec![
        format!("Selected AA mode: {}", telemetry.antialiasing_mode),
        format!("Mode source: {}", telemetry.antialiasing_mode_source),
        format!(
            "Primary Camera3d found: {}",
            if telemetry.aa_primary_camera_found {
                "yes"
            } else {
                "no"
            }
        ),
        format!(
            "Camera entity: {}",
            telemetry
                .aa_camera_entity_index
                .map(|index| index.to_string())
                .unwrap_or_else(|| "—".into())
        ),
        format!(
            "FXAA component present: {}",
            if telemetry.aa_fxaa_present {
                "yes"
            } else {
                "no"
            }
        ),
        format!(
            "SMAA component present: {}",
            if telemetry.aa_smaa_present {
                "yes"
            } else {
                "no"
            }
        ),
        format!("SMAA preset: {}", telemetry.aa_smaa_preset),
        format!(
            "Dual post-AA components active: {}",
            if telemetry.aa_dual_post_aa_active {
                "yes"
            } else {
                "no"
            }
        ),
    ];
    if telemetry.aa_state_mismatch {
        lines.push("AA STATE MISMATCH".into());
    }
    lines.push("MSAA: deferred / not implemented in Studio AA mode".into());
    lines.push("TAA: deferred / not implemented in Studio AA mode".into());
    lines.push(format!(
        "AA settings generation: {}",
        telemetry.aa_apply_generation
    ));
    lines.push(format!(
        "Last applied AA mode: {}",
        telemetry.aa_last_applied_mode
    ));
    lines.push(format!(
        "Last applied frame: {}",
        telemetry.aa_last_applied_frame
    ));
    if let Some(name) = telemetry.gpu_name.as_deref() {
        lines.push(format!("GPU adapter: {name}"));
    }
    if let Some(backend) = telemetry.gpu_backend.as_deref() {
        lines.push(format!("Backend: {backend}"));
    }
    if let Some(scale) = telemetry.render_scale {
        lines.push(format!("Window scale factor: {scale:.2}"));
    }
    lines
}

/// Hyperlane ribbon debug subsection for the Telemetry dialog.
pub fn hyperlane_debug_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    let bucket_names = ["Near", "Mid", "Far"];
    let mut lines = vec![
        format!("Hyperlane sync calls: {}", telemetry.hyperlane_sync_calls),
        format!(
            "Hyperlane rebuild count: {} (rejected invalid: {})",
            telemetry.hyperlane_rebuild_count, telemetry.hyperlane_invalid_rebuild_rejected
        ),
        format!("Last camera key: {}", telemetry.hyperlane_last_camera_key),
        format!("Current camera key: {}", telemetry.hyperlane_current_camera_key),
        format!(
            "Camera right: [{:.3}, {:.3}, {:.3}]",
            telemetry.hyperlane_camera_right[0],
            telemetry.hyperlane_camera_right[1],
            telemetry.hyperlane_camera_right[2],
        ),
        format!(
            "Camera up: [{:.3}, {:.3}, {:.3}]",
            telemetry.hyperlane_camera_up[0],
            telemetry.hyperlane_camera_up[1],
            telemetry.hyperlane_camera_up[2],
        ),
        format!(
            "Camera forward: [{:.3}, {:.3}, {:.3}]",
            telemetry.hyperlane_camera_forward[0],
            telemetry.hyperlane_camera_forward[1],
            telemetry.hyperlane_camera_forward[2],
        ),
        format!("View mode key: {}", telemetry.hyperlane_view_mode),
        format!(
            "Source segments: {} | aggregate verts/indices: {} / {}",
            telemetry.hyperlane_source_segment_count,
            telemetry.hyperlane_vertices_last_count,
            telemetry.hyperlane_indices_last_count,
        ),
        format!(
            "Degenerate width-dir (fallback-handled): {} | NaN/Inf verts: {} | zero-length segments: {}",
            telemetry.hyperlane_degenerate_width_dir_count,
            telemetry.hyperlane_nan_inf_vertex_count,
            telemetry.hyperlane_zero_length_segment_count,
        ),
        format!(
            "Mesh-build camera key: {}",
            telemetry.hyperlane_mesh_build_camera_key
        ),
        format!(
            "Mesh-build right: [{:.3}, {:.3}, {:.3}]",
            telemetry.hyperlane_mesh_build_camera_right[0],
            telemetry.hyperlane_mesh_build_camera_right[1],
            telemetry.hyperlane_mesh_build_camera_right[2],
        ),
        format!(
            "Mesh-build up: [{:.3}, {:.3}, {:.3}]",
            telemetry.hyperlane_mesh_build_camera_up[0],
            telemetry.hyperlane_mesh_build_camera_up[1],
            telemetry.hyperlane_mesh_build_camera_up[2],
        ),
        format!(
            "Mesh-build forward: [{:.3}, {:.3}, {:.3}]",
            telemetry.hyperlane_mesh_build_camera_forward[0],
            telemetry.hyperlane_mesh_build_camera_forward[1],
            telemetry.hyperlane_mesh_build_camera_forward[2],
        ),
        format!(
            "Basis mismatch deg (R/U/F): {:.3} / {:.3} / {:.3} | active: {}",
            telemetry.hyperlane_basis_mismatch_right_deg,
            telemetry.hyperlane_basis_mismatch_up_deg,
            telemetry.hyperlane_basis_mismatch_forward_deg,
            telemetry.hyperlane_basis_mismatch_active,
        ),
        format!(
            "Frames since rebuild: {} | RMB orbit: {} | rotation delta since rebuild: {:.3}°",
            telemetry.hyperlane_frames_since_rebuild,
            telemetry.hyperlane_rmb_orbit_active,
            telemetry.hyperlane_rotation_delta_since_rebuild_deg,
        ),
        format!(
            "Stale-basis rebuild count: {}",
            telemetry.hyperlane_stale_basis_rebuild_count
        ),
    ];
    for (idx, name) in bucket_names.iter().enumerate() {
        lines.push(format!(
            "Bucket {name}: {} segments, {} verts, {} indices",
            telemetry.hyperlane_bucket_segment_count[idx],
            telemetry.hyperlane_bucket_vertex_count[idx],
            telemetry.hyperlane_bucket_index_count[idx],
        ));
    }
    lines
}

/// Compact performance summary for collapsed Telemetry section.
pub fn performance_summary_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    use crate::studio_frame_phase_gpu_telemetry::{
        frame_phase_settings_lines, studio_build_profile_label, studio_build_profile_warning,
    };

    let mut lines = vec![
        format!("FPS: {}", format_fps_label(telemetry.fps)),
        format!("Build: {}", studio_build_profile_label()),
    ];
    if let Some(warning) = studio_build_profile_warning() {
        lines.push(warning.into());
    }
    lines.extend(frame_phase_settings_lines(telemetry));
    lines
}

/// Render loop, GPU context, and VRAM lines for collapsed Telemetry section.
pub fn render_loop_gpu_vram_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    use crate::studio_frame_phase_gpu_telemetry::{
        gpu_context_settings_lines, performance_capture_steps_lines, vram_tracked_asset_lines,
    };

    let mut lines = Vec::new();
    lines.extend(vram_tracked_asset_lines(telemetry));
    lines.extend(gpu_context_settings_lines(telemetry));
    lines.extend(render_loop_diagnostics_lines(telemetry));
    lines.extend(performance_capture_steps_lines());
    lines
}

/// Build Settings-window performance section labels.
pub fn performance_settings_section_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    let mut lines = vec!["Performance".into()];
    lines.extend(performance_summary_lines(telemetry));
    lines.extend(render_loop_gpu_vram_lines(telemetry));
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
            "VRAM scan: {} ms",
            format_timing_ms(telemetry.vram_scan_last_ms),
        ),
    ]
}
