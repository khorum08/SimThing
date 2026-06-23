pub mod atlas;
pub mod bench;
pub mod bevy;
pub mod deform;
pub mod font;
pub mod harness;
pub mod icons;
pub mod lr9;
pub mod manifest;
pub mod msdf;
mod numeric_damage;
pub mod path;
pub mod shaping;
pub mod studio_labels;
pub mod style;
mod text_render;
pub mod warp;
mod wgpu_smoke;
pub mod world_text;

pub use atlas::{
    format_atlas_report, quantize_px, rasterize_glyph_cpu, AtlasTile, GlyphAtlas, GlyphAtlasCore,
    GlyphAtlasKey, GlyphAtlasStats, RasterizedGlyph, ATLAS_TEXTURE_FORMAT,
};
pub use bevy::{
    create_atlas_image_from_cpu, create_render_target_image, distance_field_diagnostics,
    numeric_damage_lane_diagnostics, profile_bevy_fixed_width_numeric_damage_bench,
    profile_bevy_text_bench, reset_text_damage_phase_profile, spawn_static_and_damage_labels,
    spawn_static_and_numeric_damage_labels, spawn_static_text_labels, text_damage_phase_profile,
    text_deform_diagnostics, text_label_entity_counts, text_path_warp_diagnostics,
    text_perf_diagnostics, text_style_diagnostics, BevyTextBenchProfile, GlyphInstanceGpu,
    LabelAggregateSegment, SimthingToolsTextPlugin, TextAggregateVersion, TextDamagePhaseProfile,
    TextDrawExtract, TextGlyphInstances, TextInstanceAggregate, TextLabel, TextLabelRenderMode,
    TextPerfDiagnostics, TextRebuildDiagnostics, TonemappingLutFixPlugin, TypefaceAtlas,
};
pub use numeric_damage::{
    NumericDamageDiagnostics, NumericDamageLabel, NumericGlyphRunTable,
    NUMERIC_DAMAGE_DEFAULT_WIDTH,
};
pub use text_render::{
    text_atlas_render_diagnostics, text_deform_render_diagnostics,
    text_instanced_pipeline_initialized, text_path_warp_render_diagnostics,
    text_render_camera_bundle, text_render_queue_state, text_style_render_diagnostics,
    TextAtlasImageHandle, TextInstancedDraw, TextInstancedPipeline, TextRenderPerfDiagnostics,
    TextRenderQueueState, TextStyleGpuResource, TextStyleRenderDiagnostics,
};

pub use bench::{
    icon_tile_in_atlas, run_typeface_bench, TypefaceBenchConfig, TypefaceBenchDiagnostics,
    TypefaceBenchError, TypefaceBenchHarness, TypefaceBenchResult, CI_BENCH_CONFIG,
    HEAVY_BENCH_CONFIG,
};
pub use deform::{
    deform_params_for_slot, tess_level_for_deform_slot, tessellated_vertex_count,
    test_deform_table_fold, test_deform_table_skew, test_deform_table_stretch,
    ExtractedTextDeformTable, TextDeformDiagnostics, TextDeformKind, TextDeformParams,
    TextDeformRowGpu, TextDeformTable, TextDeformTableResource, DEFORM_TESS_LEVEL_DEFORM,
    DEFORM_TESS_LEVEL_FLAT,
};
pub use font::{load_font, GlyphMetrics, ProbeFont, TypefaceError};
pub use harness::{ascii_sample_chars, format_measurement_report, measure_chars, MeasuredGlyph};
pub use icons::{
    IconCodepoint, IconError, IconFillRule, IconLayerRole, IconPathCommand, IconRegistration,
    IconSet, IconStyleLayerRef, IconVector, IconVectorLayer, IconVectorPath, ICON_PUA_START,
};
pub use lr9::{
    adapter_label, collect_lr9_metrics, fixture_font_bytes, format_lr9_scenario_report,
    install_dynamic_style_rows, install_warp_tables, lr9_cpu_bevy_app, lr9_studio_shell_app,
    lr9_timed_updates, profile_dynamic_style_labels, profile_flat_animated_labels,
    profile_numeric_damage_lane, profile_studio_seam_labels, profile_warped_nameplates,
    spawn_studio_seam_labels, spawn_styled_labels, spawn_warped_nameplate_labels,
    validation_host_label, Lr9Config, Lr9MetricsSnapshot, Lr9ScenarioProfile, LR9_ATLAS_SIZE,
    LR9_BINDING_CONFIG, LR9_CI_CONFIG, LR9_LABEL_PX,
};
pub use manifest::{
    bake_icon_manifest, fixture_manifest_path, load_icon_manifest, IconManifest, IconManifestBake,
    IconManifestEntry,
};
pub use msdf::{
    build_distance_field_instance, sdf_params_for_distance_field_tile, DistanceFieldAtlasCore,
    DistanceFieldDiagnostics, DistanceFieldError, DistanceFieldKey, DistanceFieldKind,
    DistanceFieldTile, DISTANCE_FIELD_RENDER_MSDF, DISTANCE_FIELD_RENDER_RASTER,
    DISTANCE_FIELD_RENDER_SDF,
};
pub use path::{
    path_params_for_slot, test_path_table_arc, test_path_table_quadratic_bezier,
    ExtractedTextPathTable, TextPathKind, TextPathParams, TextPathRowGpu, TextPathTable,
    TextPathTableResource, TextPathWarpDiagnostics,
};
pub use shaping::{format_shaping_report, ShapedGlyph, ShapedRun, ShapingEngine};
pub use studio_labels::{
    icon_name_to_codepoint, resolve_studio_display_text, spawn_studio_typeface_label,
    studio_typeface_label_diagnostics, try_parse_damage_value, StudioDamageTextEmitter,
    StudioLabelKind, StudioTypefaceLabel, StudioTypefaceLabelConfig,
    StudioTypefaceLabelDiagnostics, StudioTypefaceLabelPlugin, TypefaceIconSet,
};
pub use style::{
    role_slot_for_icon_layer, style_params_for_slot, test_style_table_gradient,
    test_style_table_solid_red, ExtractedTextStyleTable, StyleError, TextStyleDiagnostics,
    TextStyleGlobalsGpu, TextStyleRow, TextStyleRowGpu, TextStyleRowsUniform, TextStyleSlot,
    TextStyleTable, TextStyleTableResource, TextStyleTableUniform, GRADIENT_MODE_LINEAR_U,
    GRADIENT_MODE_LINEAR_V, GRADIENT_MODE_NONE, MAX_STYLE_SLOTS,
};
pub use warp::{
    test_warp_table_lattice2x2, warp_params_for_slot, ExtractedTextWarpTable, TextWarpKind,
    TextWarpParams, TextWarpRowGpu, TextWarpTable, TextWarpTableResource,
};
pub use wgpu_smoke::{
    wgpu_deformed_instanced_text_smoke, wgpu_instanced_text_smoke,
    wgpu_path_warp_instanced_text_smoke, wgpu_sdf_instanced_text_smoke,
    wgpu_styled_instanced_text_smoke, WgpuSmokeTarget, WgpuTextSmokeResult,
};
pub use world_text::{
    natural_run_aspect_from_glyphs, normalized_label_local_x_range_from_glyphs,
    world_text_diagnostics, WorldGlyphInstanceGpu, WorldTextBillboard, WorldTextDiagnostics,
    WorldTextGlyphInstances, WorldTextNameplateLodPatch, WorldTextPlacementMode,
    GPU_SCREEN_LABEL_MIN_SELECTED_HEIGHT_PX, WORLD_TEXT_GPU_SCREEN_LABEL_MODE,
    WORLD_TEXT_SCREEN_COMPANION_MODE,
};
