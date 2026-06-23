//! STUDIO-FRAME-PHASE-GPU-TELEMETRY-0 — frame-phase, GPU-context, and diagnostic helpers.

use crate::star_render::StarRenderMode;
use crate::studio_performance_telemetry::StudioPerformanceTelemetry;
use crate::studio_render_loop_dirty_gate::render_loop_telemetry_record_timing;

/// Snapshot of presentation render settings for Restore Normal Render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerformanceNormalRenderSnapshot {
    pub show_stars: bool,
    pub show_hyperlanes: bool,
    pub star_render_mode: StarRenderMode,
    pub hide_star_aura: bool,
    pub hide_panels: bool,
    pub freeze_camera: bool,
}

/// Presentation-only diagnostic isolation flags (non-authoritative).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PerformanceDiagnosticFlags {
    pub hide_panels: bool,
    pub freeze_camera: bool,
    pub hide_star_aura: bool,
}

pub fn studio_build_profile_label() -> &'static str {
    if cfg!(debug_assertions) {
        "debug/unoptimized"
    } else {
        "release/optimized"
    }
}

pub fn studio_build_profile_warning() -> Option<&'static str> {
    if cfg!(debug_assertions) {
        Some(
            "Debug build can dominate Bevy/egui render performance. Re-test with release before judging GPU/render bottlenecks.",
        )
    } else {
        None
    }
}

pub fn format_gpu_device_type(device_type: Option<&str>) -> String {
    device_type
        .map(|value| value.to_string())
        .unwrap_or_else(|| "unavailable".into())
}

pub fn format_optional_label(prefix: &str, value: Option<&str>) -> String {
    format!("{prefix}: {}", value.unwrap_or("unavailable"))
}

pub fn format_present_mode_label(present_mode: Option<&str>) -> String {
    format_present_mode_label_raw(present_mode)
}

pub fn format_present_mode_label_raw(present_mode: Option<&str>) -> String {
    format!("Present mode: {}", present_mode.unwrap_or("unavailable"))
}

pub fn format_window_resolution(width: Option<u32>, height: Option<u32>) -> String {
    match (width, height) {
        (Some(w), Some(h)) => format!("Window resolution: {w}x{h}"),
        _ => "Window resolution: unavailable".into(),
    }
}

pub fn format_render_scale(scale: Option<f32>) -> String {
    match scale {
        Some(value) if value.is_finite() && value > 0.0 => format!("Render scale: {value:.2}"),
        _ => "Render scale: unavailable".into(),
    }
}

pub fn read_frame_time_ms_from_diagnostics(
    diagnostics: &bevy::diagnostic::DiagnosticsStore,
) -> Option<f64> {
    use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|diagnostic| diagnostic.smoothed().or_else(|| diagnostic.value()))
        .filter(|ms| ms.is_finite() && *ms > 0.0)
}

pub fn instrumented_render_loop_ms(telemetry: &StudioPerformanceTelemetry) -> f64 {
    [
        telemetry.hyperlane_mesh_rebuild_last_ms,
        telemetry.star_visual_sync_last_ms,
        telemetry.billboard_sync_last_ms,
        telemetry.picking_projection_last_ms,
        telemetry.vram_scan_last_ms,
    ]
    .into_iter()
    .flatten()
    .sum()
}

pub fn unexplained_frame_ms(
    frame_total_ms: Option<f64>,
    main_update_ms: Option<f64>,
    egui_pass_ms: Option<f64>,
    instrumented_ms: f64,
) -> Option<f64> {
    let total = frame_total_ms?;
    let accounted = main_update_ms.unwrap_or(0.0) + egui_pass_ms.unwrap_or(0.0) + instrumented_ms;
    Some((total - accounted).max(0.0))
}

pub fn capture_normal_render_snapshot(
    show_stars: bool,
    show_hyperlanes: bool,
    star_render_mode: StarRenderMode,
    flags: PerformanceDiagnosticFlags,
) -> PerformanceNormalRenderSnapshot {
    PerformanceNormalRenderSnapshot {
        show_stars,
        show_hyperlanes,
        star_render_mode,
        hide_star_aura: flags.hide_star_aura,
        hide_panels: flags.hide_panels,
        freeze_camera: flags.freeze_camera,
    }
}

pub fn apply_diagnostic_minimal_render(
    show_stars: &mut bool,
    show_hyperlanes: &mut bool,
    star_render_mode: &mut StarRenderMode,
    flags: &mut PerformanceDiagnosticFlags,
) {
    *show_hyperlanes = false;
    *show_stars = true;
    *star_render_mode = StarRenderMode::CrispCircle;
    flags.hide_star_aura = true;
}

pub fn restore_normal_render_from_snapshot(
    snapshot: PerformanceNormalRenderSnapshot,
    show_stars: &mut bool,
    show_hyperlanes: &mut bool,
    star_render_mode: &mut StarRenderMode,
    flags: &mut PerformanceDiagnosticFlags,
) {
    *show_stars = snapshot.show_stars;
    *show_hyperlanes = snapshot.show_hyperlanes;
    *star_render_mode = snapshot.star_render_mode;
    flags.hide_star_aura = snapshot.hide_star_aura;
    flags.hide_panels = snapshot.hide_panels;
    flags.freeze_camera = snapshot.freeze_camera;
}

pub fn record_frame_phase_timing(
    field_last: &mut Option<f64>,
    field_avg: &mut Option<f64>,
    sample_ms: f64,
    sample_count: u64,
) {
    render_loop_telemetry_record_timing(field_last, field_avg, sample_ms, sample_count);
}

fn format_timing_ms(value: Option<f64>) -> String {
    match value {
        Some(ms) if ms.is_finite() => format!("{ms:.2}"),
        _ => "—".into(),
    }
}

/// GPU adapter/backend/present subsection for Settings Performance.
pub fn gpu_context_settings_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    vec![
        format_optional_label("GPU adapter", telemetry.gpu_name.as_deref()),
        format!(
            "GPU vendor/device: {}",
            match (telemetry.gpu_vendor_id, telemetry.gpu_device_id) {
                (Some(vendor), Some(device)) => format!("{vendor:#06x}/{device:#06x}"),
                _ => "unavailable".into(),
            }
        ),
        format_optional_label("GPU backend", telemetry.gpu_backend.as_deref()),
        format!(
            "GPU device type: {}",
            telemetry
                .gpu_device_type
                .as_deref()
                .unwrap_or("unavailable")
        ),
        format_present_mode_label(telemetry.present_mode.as_deref()),
        format_window_resolution(telemetry.window_width, telemetry.window_height),
        format_render_scale(telemetry.render_scale),
        format!("Antialiasing: {}", telemetry.antialiasing_mode),
    ]
}

/// Frame-phase subsection for Settings Performance.
pub fn frame_phase_settings_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    let instrumented = instrumented_render_loop_ms(telemetry);
    let unexplained = unexplained_frame_ms(
        telemetry.frame_total_ms_last,
        telemetry.main_update_ms_last,
        telemetry.egui_pass_ms_last,
        instrumented,
    );
    let mut lines = vec![
        "Frame phase".into(),
        format!(
            "Frame total: {} ms / {} ms",
            format_timing_ms(telemetry.frame_total_ms_last),
            format_timing_ms(telemetry.frame_total_ms_avg),
        ),
        format!(
            "Main Update pass: {} ms / {} ms",
            format_timing_ms(telemetry.main_update_ms_last),
            format_timing_ms(telemetry.main_update_ms_avg),
        ),
        format!(
            "Egui/UI pass: {} ms / {} ms",
            format_timing_ms(telemetry.egui_pass_ms_last),
            format_timing_ms(telemetry.egui_pass_ms_avg),
        ),
        format!(
            "Settings/Telemetry dialogs: {} ms / {} ms",
            format_timing_ms(telemetry.egui_settings_ms_last),
            format_timing_ms(telemetry.egui_settings_ms_avg),
        ),
        format!(
            "Left panel: {} ms / {} ms",
            format_timing_ms(telemetry.egui_left_panel_ms_last),
            format_timing_ms(telemetry.egui_left_panel_ms_avg),
        ),
        format!(
            "Galaxy status panel: {} ms / {} ms",
            format_timing_ms(telemetry.egui_right_panel_ms_last),
            format_timing_ms(telemetry.egui_right_panel_ms_avg),
        ),
        format!("Instrumented render-loop (last): {:.2} ms", instrumented),
        format!(
            "Unexplained frame time (est.): {}",
            unexplained
                .map(|ms| format!("{ms:.2} ms"))
                .unwrap_or_else(|| "—".into())
        ),
    ];
    if telemetry.render_subapp_phases_unavailable {
        lines.push("Render sub-app phase timing: unavailable in this build".into());
    } else {
        lines.push(format!(
            "Render extract: {} ms",
            format_timing_ms(telemetry.render_extract_ms_last)
        ));
        lines.push(format!(
            "Render prepare: {} ms",
            format_timing_ms(telemetry.render_prepare_ms_last)
        ));
        lines.push(format!(
            "Render queue: {} ms",
            format_timing_ms(telemetry.render_queue_ms_last)
        ));
        lines.push(format!(
            "Render draw: {} ms",
            format_timing_ms(telemetry.render_render_ms_last)
        ));
        lines.push(format!(
            "Present/wait: {} ms",
            format_timing_ms(telemetry.render_present_or_wait_ms_last)
        ));
    }
    lines
}

/// VRAM tracked-asset breakdown for Settings Performance.
pub fn vram_tracked_asset_lines(telemetry: &StudioPerformanceTelemetry) -> Vec<String> {
    vec![
        format!(
            "Allocated VRAM estimate: {} MB (tracked assets only)",
            crate::studio_performance_telemetry::format_vram_mb_label(
                telemetry.allocated_vram_mb_estimate
            )
        ),
        format!(
            "Texture assets: {} MB",
            crate::studio_performance_telemetry::format_vram_mb_label(
                crate::studio_performance_telemetry::bytes_to_vram_mb(
                    telemetry.texture_bytes_estimate
                )
            )
        ),
        format!(
            "Mesh assets: {} MB",
            crate::studio_performance_telemetry::format_vram_mb_label(
                crate::studio_performance_telemetry::bytes_to_vram_mb(
                    telemetry.mesh_bytes_estimate
                )
            )
        ),
        format!(
            "Buffers tracked by Studio: {} MB",
            crate::studio_performance_telemetry::format_vram_mb_label(
                crate::studio_performance_telemetry::bytes_to_vram_mb(
                    telemetry.buffer_bytes_estimate
                )
            )
        ),
        "Render targets / swapchain: untracked".into(),
        "Bloom/postprocess intermediates: untracked".into(),
    ]
}

pub fn performance_capture_steps_lines() -> Vec<String> {
    vec![
        "Performance capture steps:".into(),
        "1. Baseline".into(),
        "2. Hide hyperlanes".into(),
        "3. Minimal stars".into(),
        "4. Hide egui panels".into(),
        "5. Release build".into(),
    ]
}

pub const DIAGNOSTIC_MINIMAL_RENDER_BUTTON: &str = "Apply Diagnostic Minimal Render";
pub const RESTORE_NORMAL_RENDER_BUTTON: &str = "Restore Normal Render";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_minimal_render_does_not_touch_authority_fields() {
        let mut show_stars = true;
        let mut show_hyperlanes = true;
        let mut mode = StarRenderMode::BloomStarburst;
        let mut flags = PerformanceDiagnosticFlags::default();
        apply_diagnostic_minimal_render(
            &mut show_stars,
            &mut show_hyperlanes,
            &mut mode,
            &mut flags,
        );
        assert!(!show_hyperlanes);
        assert_eq!(mode, StarRenderMode::CrispCircle);
        assert!(flags.hide_star_aura);
    }
}
