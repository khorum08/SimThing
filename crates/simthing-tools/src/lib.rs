pub mod atlas;
pub mod bevy;
pub mod font;
pub mod harness;
pub mod shaping;

pub use atlas::{
    format_atlas_report, quantize_px, rasterize_glyph_cpu, AtlasTile, GlyphAtlas, GlyphAtlasCore,
    GlyphAtlasKey, GlyphAtlasStats, RasterizedGlyph, ATLAS_TEXTURE_FORMAT,
};
pub use bevy::{
    GlyphInstanceGpu, SimthingToolsTextPlugin, TextGlyphInstances, TextInstanceAggregate,
    TextLabel, TextRebuildDiagnostics, TypefaceAtlas,
};
pub use font::{load_font, GlyphMetrics, ProbeFont, TypefaceError};
pub use harness::{ascii_sample_chars, format_measurement_report, measure_chars, MeasuredGlyph};
pub use shaping::{format_shaping_report, ShapedGlyph, ShapedRun, ShapingEngine};
