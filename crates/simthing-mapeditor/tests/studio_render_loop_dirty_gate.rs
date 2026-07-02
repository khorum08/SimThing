//! STUDIO-RENDER-LOOP-DIRTY-GATE-0 — render-loop dirty gate and diagnostics proofs.

use simthing_mapeditor::star_render::StarRenderMode;
use simthing_mapeditor::{
    hyperlane_render_should_rebuild, performance_settings_section_lines,
    picking_projection_should_rebuild, quantize_hyperlane_camera_key,
    quantize_picking_projection_key, render_loop_diagnostics_lines,
    render_loop_telemetry_record_timing, star_visuals_should_sync, HyperlaneCameraKey,
    HyperlaneRenderSettingsKey, StarFalloffSettingsKey, StarVisualSyncKey,
    StudioPerformanceTelemetry,
};

fn sample_hyperlane_camera() -> HyperlaneCameraKey {
    quantize_hyperlane_camera_key(
        [40.0, 35.0, 40.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, -1.0],
        0,
    )
}

fn sample_hyperlane_settings() -> HyperlaneRenderSettingsKey {
    HyperlaneRenderSettingsKey {
        base_thickness_percent_of_star: 80,
        base_opacity_percent: 750,
        falloff_distance_percent: 1000,
        falloff_thickness_percent: 240,
        falloff_opacity_percent: 500,
    }
}

fn sample_star_sync_key() -> StarVisualSyncKey {
    StarVisualSyncKey {
        camera_position: [80, 70, 80],
        selected_system_id: None,
        hovered_system_id: None,
        render_mode: StarRenderMode::BloomStarburst,
        falloff_settings: StarFalloffSettingsKey {
            base_blur_radius: 11,
            falloff_distance_percent: 1000,
            falloff_opacity_percent: 700,
        },
        view_model_generation: 1,
    }
}

#[test]
fn hyperlane_dirty_gate_skips_when_camera_and_settings_unchanged() {
    let camera = sample_hyperlane_camera();
    let settings = sample_hyperlane_settings();
    assert!(!hyperlane_render_should_rebuild(
        Some(camera),
        camera,
        Some(settings),
        settings,
        1,
        1,
        false,
        false,
        false,
        false,
    ));
}

#[test]
fn hyperlane_dirty_gate_rebuilds_when_settings_change() {
    let camera = sample_hyperlane_camera();
    let prev = sample_hyperlane_settings();
    let mut next = prev;
    next.falloff_opacity_percent += 5;
    assert!(hyperlane_render_should_rebuild(
        Some(camera),
        camera,
        Some(prev),
        next,
        1,
        1,
        false,
        false,
        false,
        false,
    ));
}

#[test]
fn hyperlane_dirty_gate_rebuilds_when_camera_key_changes() {
    let prev = sample_hyperlane_camera();
    let mut next = prev;
    next.position[0] += 2;
    let settings = sample_hyperlane_settings();
    assert!(hyperlane_render_should_rebuild(
        Some(prev),
        next,
        Some(settings),
        settings,
        1,
        1,
        false,
        false,
        false,
        false,
    ));
}

#[test]
fn hyperlane_dirty_gate_rebuilds_when_session_changes() {
    let camera = sample_hyperlane_camera();
    let settings = sample_hyperlane_settings();
    assert!(hyperlane_render_should_rebuild(
        Some(camera),
        camera,
        Some(settings),
        settings,
        1,
        2,
        false,
        false,
        false,
        false,
    ));
}

#[test]
fn star_visual_dirty_gate_skips_when_camera_selection_settings_unchanged() {
    let key = sample_star_sync_key();
    assert!(!star_visuals_should_sync(Some(key), key, false));
}

#[test]
fn star_visual_dirty_gate_rebuilds_when_selection_changes() {
    let prev = sample_star_sync_key();
    let mut next = prev;
    next.selected_system_id = Some(99);
    assert!(star_visuals_should_sync(Some(prev), next, false));
}

#[test]
fn star_visual_dirty_gate_rebuilds_when_hover_changes() {
    let prev = sample_star_sync_key();
    let mut next = prev;
    next.hovered_system_id = Some(12);
    assert!(star_visuals_should_sync(Some(prev), next, false));
}

#[test]
fn star_visual_dirty_gate_rebuilds_when_render_settings_change() {
    let prev = sample_star_sync_key();
    let mut next = prev;
    next.render_mode = StarRenderMode::CrispCircle;
    assert!(star_visuals_should_sync(Some(prev), next, false));
}

#[test]
fn render_loop_telemetry_records_timing_helper() {
    let mut last = None;
    let mut avg = None;
    render_loop_telemetry_record_timing(&mut last, &mut avg, 1.0, 1);
    render_loop_telemetry_record_timing(&mut last, &mut avg, 3.0, 2);
    assert_eq!(last, Some(3.0));
    assert!((avg.unwrap() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn picking_projection_cache_reuses_when_camera_window_session_unchanged() {
    let key =
        quantize_picking_projection_key([1.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0], 800, 600, 500, 2);
    assert!(!picking_projection_should_rebuild(Some(key), key));
}

#[test]
fn picking_projection_cache_rebuilds_when_camera_changes() {
    let prev =
        quantize_picking_projection_key([1.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0], 800, 600, 500, 2);
    let next =
        quantize_picking_projection_key([20.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0], 800, 600, 500, 2);
    assert!(picking_projection_should_rebuild(Some(prev), next));
}

#[test]
fn picking_projection_cache_rebuilds_when_window_size_changes() {
    let prev =
        quantize_picking_projection_key([1.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0], 800, 600, 500, 2);
    let next =
        quantize_picking_projection_key([1.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0], 1024, 768, 500, 2);
    assert!(picking_projection_should_rebuild(Some(prev), next));
}
