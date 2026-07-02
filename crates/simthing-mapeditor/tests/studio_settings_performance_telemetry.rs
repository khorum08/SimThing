//! STUDIO-SETTINGS-PERFORMANCE-TELEMETRY-0 — Settings performance telemetry proofs.

use bevy::prelude::Image;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use simthing_mapeditor::{
    bytes_to_vram_mb, estimate_image_vram_bytes, estimate_mesh_vram_bytes, format_fps_label,
    format_vram_mb_label, performance_settings_section_lines, render_loop_diagnostics_lines,
    StudioPerformanceTelemetry,
};

#[test]
fn studio_performance_telemetry_does_not_mutate_scenario_authority() {
    let telemetry = StudioPerformanceTelemetry::default();
    let lines = performance_settings_section_lines(&telemetry);
    assert!(!lines.iter().any(|line| line.contains("scenario_authority")));
    assert!(!lines.iter().any(|line| line.contains("ScenarioSpec")));
}

