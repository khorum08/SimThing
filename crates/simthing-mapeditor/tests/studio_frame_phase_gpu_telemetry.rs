//! STUDIO-FRAME-PHASE-GPU-TELEMETRY-0 — frame-phase and GPU-context telemetry proofs.

use simthing_mapeditor::star_render::StarRenderMode;
use simthing_mapeditor::{
    apply_diagnostic_minimal_render, format_present_mode_label, frame_phase_settings_lines,
    gpu_context_settings_lines, performance_capture_steps_lines,
    performance_settings_section_lines, studio_build_profile_label, vram_tracked_asset_lines,
    PerformanceDiagnosticFlags, StudioPerformanceTelemetry,
};

#[test]
fn studio_performance_telemetry_formats_build_profile() {
    let lines = performance_settings_section_lines(&StudioPerformanceTelemetry::default());
    let expected = format!("Build: {}", studio_build_profile_label());
    assert!(lines.iter().any(|line| line == &expected));
    if cfg!(debug_assertions) {
        assert!(lines
            .iter()
            .any(|line| line.contains("Debug build can dominate")));
    }
}

#[test]
fn studio_performance_telemetry_formats_gpu_adapter_unknown_gracefully() {
    let lines = gpu_context_settings_lines(&StudioPerformanceTelemetry::default());
    assert!(lines.iter().any(|line| line == "GPU adapter: unavailable"));
    assert!(lines
        .iter()
        .any(|line| line == "GPU vendor/device: unavailable"));
    assert!(lines.iter().any(|line| line == "GPU backend: unavailable"));
    assert!(lines
        .iter()
        .any(|line| line == "GPU device type: unavailable"));
}

#[test]
fn studio_performance_telemetry_formats_present_mode_unknown_gracefully() {
    assert_eq!(format_present_mode_label(None), "Present mode: unavailable");
    let lines = gpu_context_settings_lines(&StudioPerformanceTelemetry::default());
    assert!(lines.iter().any(|line| line == "Present mode: unavailable"));
}

#[test]
fn studio_performance_telemetry_formats_frame_phase_ms() {
    let mut telemetry = StudioPerformanceTelemetry::default();
    telemetry.frame_total_ms_last = Some(238.0);
    telemetry.frame_total_ms_avg = Some(240.0);
    telemetry.main_update_ms_last = Some(2.5);
    telemetry.egui_pass_ms_last = Some(180.0);
    telemetry.egui_settings_ms_last = Some(12.0);
    telemetry.hyperlane_mesh_rebuild_last_ms = Some(0.47);
    telemetry.star_visual_sync_last_ms = Some(0.11);
    let lines = frame_phase_settings_lines(&telemetry);
    assert!(lines.iter().any(|line| line == "Frame phase"));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Frame total: 238.00 ms / 240.00 ms")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Egui/UI pass: 180.00 ms")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Unexplained frame time (est.):")));
    assert!(lines
        .iter()
        .any(|line| line == "Render sub-app phase timing: unavailable in this build"));
}

#[test]
fn studio_performance_telemetry_formats_vram_as_tracked_assets_only() {
    let mut telemetry = StudioPerformanceTelemetry::default();
    telemetry.allocated_vram_mb_estimate = 3.7;
    telemetry.texture_bytes_estimate = 2 * 1024 * 1024;
    telemetry.mesh_bytes_estimate = 1024 * 1024;
    let lines = vram_tracked_asset_lines(&telemetry);
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Allocated VRAM estimate: 3.7 MB (tracked assets only)")));
    assert!(lines.iter().any(|line| line.starts_with("Texture assets:")));
    assert!(lines
        .iter()
        .any(|line| line == "Render targets / swapchain: untracked"));
    assert!(lines
        .iter()
        .any(|line| line == "Bloom/postprocess intermediates: untracked"));
}

#[test]
fn studio_performance_settings_renders_frame_phase_section() {
    let lines = performance_settings_section_lines(&StudioPerformanceTelemetry::default());
    assert!(lines.iter().any(|line| line == "Frame phase"));
    assert!(lines.iter().any(|line| line.starts_with("Frame total:")));
    assert!(lines.iter().any(|line| line.starts_with("Egui/UI pass:")));
}

#[test]
fn studio_performance_settings_renders_diagnostic_controls() {
    let lines = performance_settings_section_lines(&StudioPerformanceTelemetry::default());
    assert!(lines
        .iter()
        .any(|line| line == "Performance capture steps:"));
    let capture = performance_capture_steps_lines();
    assert!(capture.iter().any(|line| line == "1. Baseline"));
    assert!(capture.iter().any(|line| line == "5. Release build"));
}

#[test]
fn studio_performance_diagnostic_minimal_render_preserves_scenario_authority() {
    let mut show_stars = true;
    let mut show_hyperlanes = true;
    let mut mode = StarRenderMode::BloomStarburst;
    let mut flags = PerformanceDiagnosticFlags::default();
    apply_diagnostic_minimal_render(&mut show_stars, &mut show_hyperlanes, &mut mode, &mut flags);
    assert!(!show_hyperlanes);
    assert_eq!(mode, StarRenderMode::CrispCircle);
    assert!(flags.hide_star_aura);
}

#[test]
fn studio_performance_telemetry_does_not_mark_runtime_saveload_status_dirty() {
    let telemetry = StudioPerformanceTelemetry::default();
    let lines = performance_settings_section_lines(&telemetry);
    assert!(!lines
        .iter()
        .any(|line| line.contains("runtime_saveload_status_dirty")));
    assert!(!lines.iter().any(|line| line.contains("ScenarioSpec")));
}
