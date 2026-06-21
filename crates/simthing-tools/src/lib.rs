pub mod atlas;
pub mod bevy;
pub mod font;
pub mod harness;
pub mod icons;
pub mod shaping;
mod text_render;
mod wgpu_smoke;

pub use atlas::{
    format_atlas_report, quantize_px, rasterize_glyph_cpu, AtlasTile, GlyphAtlas, GlyphAtlasCore,
    GlyphAtlasKey, GlyphAtlasStats, RasterizedGlyph, ATLAS_TEXTURE_FORMAT,
};
pub use bevy::{
    create_atlas_image_from_cpu, create_render_target_image, GlyphInstanceGpu,
    SimthingToolsTextPlugin, TextGlyphInstances, TextInstanceAggregate, TextLabel,
    TextRebuildDiagnostics, TypefaceAtlas,
};
pub use text_render::{
    text_instanced_pipeline_initialized, text_render_camera_bundle, text_render_queue_state,
    TextAtlasImageHandle, TextInstancedDraw, TextInstancedPipeline, TextRenderQueueState,
};

pub use font::{load_font, GlyphMetrics, ProbeFont, TypefaceError};
pub use harness::{ascii_sample_chars, format_measurement_report, measure_chars, MeasuredGlyph};
pub use icons::{
    IconCodepoint, IconError, IconLayerRole, IconRegistration, IconSet, IconVector,
    IconVectorLayer, ICON_PUA_START,
};
pub use shaping::{format_shaping_report, ShapedGlyph, ShapedRun, ShapingEngine};
pub use wgpu_smoke::{wgpu_instanced_text_smoke, WgpuSmokeTarget, WgpuTextSmokeResult};
