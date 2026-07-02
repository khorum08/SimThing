//! STUDIO-FRAME-PHASE-GPU-TELEMETRY-0 — frame-phase and GPU-context telemetry proofs.

use simthing_mapeditor::star_render::StarRenderMode;
use simthing_mapeditor::{
    apply_diagnostic_minimal_render, format_present_mode_label, frame_phase_settings_lines,
    gpu_context_settings_lines, performance_capture_steps_lines,
    performance_settings_section_lines, studio_build_profile_label, vram_tracked_asset_lines,
    PerformanceDiagnosticFlags, StudioPerformanceTelemetry,
};

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

