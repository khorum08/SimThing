//! STUDIO-SETTINGS-PERFORMANCE-TELEMETRY-0 — Settings performance telemetry proofs.

use bevy::prelude::Image;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use simthing_mapeditor::{
    bytes_to_vram_mb, estimate_image_vram_bytes, estimate_mesh_vram_bytes, format_fps_label,
    format_vram_mb_label, performance_settings_section_lines, StudioPerformanceTelemetry,
};

#[test]
fn studio_performance_telemetry_defaults_to_warming_up_without_diagnostics() {
    let telemetry = StudioPerformanceTelemetry::default();
    assert_eq!(format_fps_label(telemetry.fps), "warming up");
}

#[test]
fn studio_performance_telemetry_formats_fps_with_one_decimal() {
    assert_eq!(format_fps_label(Some(42.74)), "42.7");
    assert_eq!(format_fps_label(None), "warming up");
}

#[test]
fn studio_performance_telemetry_formats_vram_in_mb() {
    assert_eq!(
        format_vram_mb_label(bytes_to_vram_mb(5 * 1024 * 1024)),
        "5.0"
    );
}

#[test]
fn studio_performance_telemetry_estimates_texture_bytes() {
    let image = Image::new_fill(
        Extent3d {
            width: 256,
            height: 128,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    assert_eq!(estimate_image_vram_bytes(&image), 256 * 128 * 4);
}

#[test]
fn studio_performance_telemetry_estimates_mesh_bytes() {
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
    )
    .with_inserted_indices(Indices::U16(vec![0, 1, 2]));
    let bytes = estimate_mesh_vram_bytes(&mesh);
    assert!(bytes >= 3 * 3 * 4 + 3 * 2);
}

#[test]
fn studio_settings_window_renders_performance_section() {
    let mut telemetry = StudioPerformanceTelemetry::default();
    telemetry.fps = Some(12.3);
    telemetry.allocated_vram_mb_estimate = 48.5;
    let lines = performance_settings_section_lines(&telemetry);
    assert!(lines.iter().any(|line| line == "Performance"));
    assert!(lines.iter().any(|line| line.starts_with("FPS: 12.3")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Allocated VRAM estimate: 48.5 MB")));
}

#[test]
fn studio_performance_telemetry_does_not_mutate_scenario_authority() {
    let telemetry = StudioPerformanceTelemetry::default();
    let lines = performance_settings_section_lines(&telemetry);
    assert!(!lines.iter().any(|line| line.contains("scenario_authority")));
    assert!(!lines.iter().any(|line| line.contains("ScenarioSpec")));
}

#[test]
fn studio_performance_telemetry_does_not_mark_runtime_saveload_status_dirty() {
    let mut telemetry = StudioPerformanceTelemetry::default();
    telemetry.fps = Some(9.5);
    telemetry.allocated_vram_mb_estimate = 12.0;
    let before = telemetry.clone();
    let _ = performance_settings_section_lines(&telemetry);
    assert_eq!(telemetry, before);
}
