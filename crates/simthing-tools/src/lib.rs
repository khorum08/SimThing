pub mod atlas;
pub mod bench;
pub mod bevy;
pub mod font;
pub mod harness;
pub mod icons;
mod numeric_damage;
pub mod shaping;
mod text_render;
mod wgpu_smoke;

pub use atlas::{
    format_atlas_report, quantize_px, rasterize_glyph_cpu, AtlasTile, GlyphAtlas, GlyphAtlasCore,
    GlyphAtlasKey, GlyphAtlasStats, RasterizedGlyph, ATLAS_TEXTURE_FORMAT,
};
pub use bevy::{
    create_atlas_image_from_cpu, create_render_target_image, numeric_damage_lane_diagnostics,
    profile_bevy_fixed_width_numeric_damage_bench, profile_bevy_text_bench,
    reset_text_damage_phase_profile, spawn_static_and_damage_labels,
    spawn_static_and_numeric_damage_labels, spawn_static_text_labels, text_damage_phase_profile,
    text_label_entity_counts, text_perf_diagnostics, BevyTextBenchProfile, GlyphInstanceGpu,
    LabelAggregateSegment, SimthingToolsTextPlugin, TextAggregateVersion, TextDamagePhaseProfile,
    TextDrawExtract, TextGlyphInstances, TextInstanceAggregate, TextLabel, TextPerfDiagnostics,
    TextRebuildDiagnostics, TypefaceAtlas,
};
pub use numeric_damage::{
    NumericDamageDiagnostics, NumericDamageLabel, NumericGlyphRunTable,
    NUMERIC_DAMAGE_DEFAULT_WIDTH,
};
pub use text_render::{
    text_instanced_pipeline_initialized, text_render_camera_bundle, text_render_queue_state,
    TextAtlasImageHandle, TextInstancedDraw, TextInstancedPipeline, TextRenderPerfDiagnostics,
    TextRenderQueueState,
};

pub use bench::{
    icon_tile_in_atlas, run_typeface_bench, TypefaceBenchConfig, TypefaceBenchDiagnostics,
    TypefaceBenchError, TypefaceBenchHarness, TypefaceBenchResult, CI_BENCH_CONFIG,
    HEAVY_BENCH_CONFIG,
};
pub use font::{load_font, GlyphMetrics, ProbeFont, TypefaceError};
pub use harness::{ascii_sample_chars, format_measurement_report, measure_chars, MeasuredGlyph};
pub use icons::{
    IconCodepoint, IconError, IconLayerRole, IconRegistration, IconSet, IconVector,
    IconVectorLayer, ICON_PUA_START,
};
pub use shaping::{format_shaping_report, ShapedGlyph, ShapedRun, ShapingEngine};
pub use wgpu_smoke::{wgpu_instanced_text_smoke, WgpuSmokeTarget, WgpuTextSmokeResult};
