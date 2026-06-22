use bevy::{
    asset::{load_internal_asset, Assets, Handle},
    math::primitives::Rectangle,
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        extract_resource::ExtractResource,
        extract_resource::ExtractResourcePlugin,
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
            TextureViewDimension,
        },
        sync_world::SyncToRenderWorld,
        view::NoFrustumCulling,
    },
};

use crate::{
    atlas::{AtlasDirtyRect, AtlasTile, GlyphAtlasCore, GlyphAtlasStats},
    deform::{
        deform_params_for_slot, tess_level_for_deform_slot, ExtractedTextDeformTable,
        TextDeformDiagnostics, TextDeformTableResource, TextDeformTessMesh,
        DEFORM_TESS_LEVEL_DEFORM,
    },
    font::{load_font, ProbeFont},
    msdf::{DistanceFieldDiagnostics, DistanceFieldKind, DistanceFieldTile},
    numeric_damage::{
        build_numeric_glyph_run_table, numeric_damage_diagnostics, patch_numeric_instances,
        NumericDamageDiagnostics, NumericDamageLabel, NumericGlyphRunTable,
        NUMERIC_DAMAGE_DEFAULT_WIDTH,
    },
    path::{
        path_params_for_slot, ExtractedTextPathTable, TextPathTableResource,
        TextPathWarpDiagnostics,
    },
    shaping::{ShapedGlyph, ShapedRun, ShapingEngine},
    style::{
        style_params_for_slot, ExtractedTextStyleTable, TextStyleDiagnostics, TextStyleSlot,
        TextStyleTableResource,
    },
    text_render::TextInstancedRenderPlugin,
    warp::{warp_params_for_slot, ExtractedTextWarpTable, TextWarpTableResource},
};

pub(crate) const TEXT_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x5459_5045_4c52_3300_0000_0000_0000_0001);

/// Bevy plugin for instanced atlas text labels.
#[derive(Clone)]
pub struct SimthingToolsTextPlugin {
    font_bytes: Vec<u8>,
    atlas_size: u32,
}

impl SimthingToolsTextPlugin {
    pub fn new(font_bytes: Vec<u8>) -> Self {
        Self {
            font_bytes,
            atlas_size: 512,
        }
    }

    pub fn with_atlas_size(font_bytes: Vec<u8>, atlas_size: u32) -> Self {
        Self {
            font_bytes,
            atlas_size,
        }
    }
}

impl Plugin for SimthingToolsTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Shader>();
        load_internal_asset!(
            app,
            TEXT_SHADER_HANDLE,
            "shaders/text_instanced.wgsl",
            Shader::from_wgsl
        );

        app.init_asset::<Mesh>()
            .init_asset::<Image>()
            .init_resource::<TextPerfDiagnostics>()
            .init_resource::<NumericDamageDiagnostics>()
            .init_resource::<TextDamagePhaseProfile>()
            .init_resource::<TextStyleDiagnostics>()
            .init_resource::<TextStyleTableResource>()
            .init_resource::<ExtractedTextStyleTable>()
            .init_resource::<TextDeformDiagnostics>()
            .init_resource::<TextDeformTableResource>()
            .init_resource::<ExtractedTextDeformTable>()
            .init_resource::<TextPathWarpDiagnostics>()
            .init_resource::<TextPathTableResource>()
            .init_resource::<ExtractedTextPathTable>()
            .init_resource::<TextWarpTableResource>()
            .init_resource::<ExtractedTextWarpTable>()
            .init_resource::<TextInstanceAggregate>()
            .init_resource::<TextAggregateLayout>()
            .init_resource::<TextAggregateVersion>()
            .init_resource::<crate::studio_labels::StudioTypefaceLabelDiagnostics>()
            .insert_resource(TypefaceFontBytes(self.font_bytes.clone()))
            .insert_resource(PluginAtlasSize(self.atlas_size))
            .add_plugins(ExtractResourcePlugin::<TextAggregateVersion>::default())
            .add_plugins(ExtractResourcePlugin::<ExtractedTextStyleTable>::default())
            .add_plugins(ExtractResourcePlugin::<ExtractedTextDeformTable>::default())
            .add_plugins(ExtractResourcePlugin::<ExtractedTextPathTable>::default())
            .add_plugins(ExtractResourcePlugin::<ExtractedTextWarpTable>::default())
            .add_systems(
                Startup,
                (fix_volume_image_view_descriptors, init_typeface_state).chain(),
            )
            .add_systems(PostStartup, fix_volume_image_view_descriptors)
            .add_systems(
                Update,
                (
                    advance_style_table_time,
                    sync_style_table_rows_if_changed,
                    sync_deform_table_rows_if_changed,
                    sync_path_table_rows_if_changed,
                    sync_warp_table_rows_if_changed,
                    emit_studio_damage_text_labels,
                    sync_studio_typeface_labels,
                    update_numeric_damage_labels,
                    rebuild_changed_labels,
                    ApplyDeferred,
                    mark_aggregate_dirty_on_label_lifecycle,
                    aggregate_label_instances,
                    sync_draw_entity_instances,
                    sync_draw_entity_mesh_for_deformation,
                    sync_path_warp_instance_diagnostics,
                    sync_atlas_image_to_gpu,
                    force_text_draw_visible,
                )
                    .chain(),
            )
            .add_plugins(ExtractComponentPlugin::<TextDrawExtract>::default())
            .add_plugins(TextInstancedRenderPlugin);
    }

    fn finish(&self, app: &mut App) {
        let _ = app;
    }
}

/// Render mode for a text label instance path.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextLabelRenderMode {
    #[default]
    Raster,
    Sdf,
    Msdf,
}

impl TextLabelRenderMode {
    fn distance_field_kind(self) -> Option<DistanceFieldKind> {
        match self {
            Self::Raster => None,
            Self::Sdf => Some(DistanceFieldKind::Sdf),
            Self::Msdf => Some(DistanceFieldKind::Msdf),
        }
    }
}

/// Workshop/production text label component.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct TextLabel {
    pub text: String,
    pub px: f32,
    pub color: [f32; 4],
    pub render_mode: TextLabelRenderMode,
    pub style_slot: TextStyleSlot,
    pub deform_slot: crate::deform::TextDeformSlot,
    pub path_slot: crate::path::TextPathSlot,
    pub warp_slot: crate::warp::TextWarpSlot,
}

impl TextLabel {
    pub fn raster(text: impl Into<String>, px: f32, color: [f32; 4]) -> Self {
        Self {
            text: text.into(),
            px,
            color,
            render_mode: TextLabelRenderMode::Raster,
            style_slot: 0,
            deform_slot: 0,
            path_slot: 0,
            warp_slot: 0,
        }
    }

    pub fn msdf(text: impl Into<String>, px: f32, color: [f32; 4]) -> Self {
        Self {
            text: text.into(),
            px,
            color,
            render_mode: TextLabelRenderMode::Msdf,
            style_slot: 0,
            deform_slot: 0,
            path_slot: 0,
            warp_slot: 0,
        }
    }

    pub fn with_style_slot(mut self, slot: TextStyleSlot) -> Self {
        self.style_slot = slot;
        self
    }

    pub fn with_deform_slot(mut self, slot: crate::deform::TextDeformSlot) -> Self {
        self.deform_slot = slot;
        self
    }

    pub fn with_path_slot(mut self, slot: crate::path::TextPathSlot) -> Self {
        self.path_slot = slot;
        self
    }

    pub fn with_warp_slot(mut self, slot: crate::warp::TextWarpSlot) -> Self {
        self.warp_slot = slot;
        self
    }
}

/// Consolidated main-world perf diagnostics for LR5/LR5R/LR5S tests.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextPerfDiagnostics {
    pub shape_rebuild_count: u64,
    pub shape_cache_hit_count: u64,
    pub shape_cache_miss_count: u64,
    pub instance_rebuild_count: u64,
    pub aggregate_rebuild_count: u64,
    pub aggregate_patch_count: u64,
    pub aggregate_full_rebuild_count: u64,
    pub aggregate_repack_count: u64,
    pub aggregate_patched_instance_count: u64,
    pub aggregate_full_rebuild_instance_count: u64,
    pub draw_entity_sync_count: u64,
    pub extract_clone_count: u64,
    pub extracted_instance_count: u64,
    pub instance_buffer_create_count: u64,
    pub instance_buffer_reuse_count: u64,
    pub instance_buffer_upload_count: u64,
    pub atlas_sync_count: u64,
    pub atlas_sync_bytes: u64,
    pub atlas_dirty_region_count: u64,
    pub queued_draw_count: u64,
    pub queued_instance_count: u64,
}

/// Back-compat alias used by LR3 tests.
pub type TextRebuildDiagnostics = TextPerfDiagnostics;

/// Per-frame damage-path phase timings (nanoseconds, cumulative).
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextDamagePhaseProfile {
    pub mutation_ns: u64,
    pub shaping_ns: u64,
    pub rasterize_ns: u64,
    pub instance_rebuild_ns: u64,
    pub aggregate_patch_ns: u64,
    pub aggregate_full_rebuild_ns: u64,
    pub draw_sync_ns: u64,
    pub atlas_sync_ns: u64,
    pub sample_frames: u64,
}

/// Stable segment metadata for aggregate patching.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelAggregateSegment {
    pub offset: usize,
    pub len: usize,
}

#[derive(Component, Default)]
struct SegmentDirty;

#[derive(Resource, Default, Debug)]
struct TextAggregateLayout {
    label_order: Vec<Entity>,
    needs_full_rebuild: bool,
}

/// Aggregate versioning: rebuild/sync only when `dirty`.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq, ExtractResource)]
pub struct TextAggregateVersion {
    pub current: u64,
    pub dirty: bool,
}

/// CPU atlas used by the text plugin.
#[derive(Resource)]
pub struct TypefaceAtlas {
    pub cpu: GlyphAtlasCore,
    pub distance_field: crate::msdf::DistanceFieldAtlasCore,
    pub atlas_size: u32,
}

impl TypefaceAtlas {
    pub fn new_cpu(size: u32) -> Self {
        Self {
            cpu: GlyphAtlasCore::new(size),
            distance_field: crate::msdf::DistanceFieldAtlasCore::new(size),
            atlas_size: size,
        }
    }

    pub fn distance_field_diagnostics(&self) -> DistanceFieldDiagnostics {
        self.distance_field.diagnostics()
    }

    pub fn cpu_stats(&self) -> GlyphAtlasStats {
        self.cpu.stats()
    }

    pub fn cpu_core(&self) -> &GlyphAtlasCore {
        &self.cpu
    }
}

#[derive(Resource)]
struct TypefaceFontBytes(pub Vec<u8>);

#[derive(Resource)]
struct PluginAtlasSize(u32);

#[derive(Resource)]
struct TypefaceFont(pub ProbeFont);

#[derive(Resource)]
struct TypefaceShaper(pub ShapingEngine);

#[derive(Resource)]
struct TextQuadMesh(pub Handle<Mesh>);

#[derive(Resource)]
pub struct TextDrawEntity(pub Entity);

#[derive(Component, Clone)]
struct TextLabelCache {
    text: String,
    px: f32,
    color: [f32; 4],
    render_mode: TextLabelRenderMode,
    style_slot: TextStyleSlot,
    deform_slot: crate::deform::TextDeformSlot,
    path_slot: crate::path::TextPathSlot,
    warp_slot: crate::warp::TextWarpSlot,
    shaped: ShapedRun,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct GlyphInstanceGpu {
    pub pos_size: [f32; 4],
    pub uv_rect: [f32; 4],
    pub color: [f32; 4],
    /// x = render mode (0 raster, 1 SDF, 2 MSDF), y = px_range, z/w reserved.
    pub sdf_params: [f32; 4],
    /// x = style_slot, y = role_slot, z/w reserved.
    pub style_params: [f32; 4],
    /// x = deform_slot, y = tess_level, z/w reserved.
    pub deform_params: [f32; 4],
    /// x = path_slot, y = path_u_offset, z = path_u_scale, w reserved.
    pub path_params: [f32; 4],
    /// x = warp_slot, y = strength_mul, z/w reserved.
    pub warp_params: [f32; 4],
}

#[derive(Component, Clone, Default, Debug)]
pub struct TextGlyphInstances(pub Vec<GlyphInstanceGpu>);

#[derive(Resource, Default)]
pub struct TextInstanceAggregate(pub Vec<GlyphInstanceGpu>);

#[derive(Component, Clone)]
pub struct TextDrawExtract {
    pub(crate) instances: Vec<GlyphInstanceGpu>,
    pub(crate) data_version: u64,
}

impl bevy::render::extract_component::ExtractComponent for TextDrawExtract {
    type QueryData = &'static TextGlyphInstances;
    type QueryFilter = With<crate::text_render::TextInstancedDraw>;
    type Out = Self;

    fn extract_component(item: bevy::ecs::query::QueryItem<Self::QueryData>) -> Option<Self> {
        if item.0.is_empty() {
            return None;
        }
        Some(Self {
            instances: item.0.clone(),
            data_version: 0,
        })
    }
}

/// Merge main-world and render-world perf counters for tests.
pub fn text_perf_diagnostics(app: &App) -> TextPerfDiagnostics {
    let mut diag = app
        .world()
        .get_resource::<TextPerfDiagnostics>()
        .copied()
        .unwrap_or_default();
    if let Some(render_app) = app.get_sub_app(bevy::render::RenderApp) {
        if let Some(render_diag) = render_app
            .world()
            .get_resource::<crate::text_render::TextRenderPerfDiagnostics>()
        {
            diag.extract_clone_count += render_diag.extract_clone_count;
            diag.extracted_instance_count = render_diag.extracted_instance_count;
            diag.instance_buffer_create_count += render_diag.instance_buffer_create_count;
            diag.instance_buffer_reuse_count += render_diag.instance_buffer_reuse_count;
            diag.instance_buffer_upload_count += render_diag.instance_buffer_upload_count;
            diag.queued_draw_count = render_diag.queued_draw_count;
            diag.queued_instance_count = render_diag.queued_instance_count;
        }
    }
    diag
}

/// Read cumulative damage-frame phase timings from the main world.
pub fn text_damage_phase_profile(app: &App) -> TextDamagePhaseProfile {
    app.world()
        .get_resource::<TextDamagePhaseProfile>()
        .copied()
        .unwrap_or_default()
}

/// Reset damage phase timings before a profiled damage run.
pub fn reset_text_damage_phase_profile(app: &mut App) {
    if let Some(mut phase) = app.world_mut().get_resource_mut::<TextDamagePhaseProfile>() {
        *phase = TextDamagePhaseProfile::default();
    }
}

/// Ensure volume/LUT images expose D3 texture views for mesh2d tonemapping bind groups.
fn fix_volume_image_view_descriptors(mut images: ResMut<Assets<Image>>) {
    for (_, image) in images.iter_mut() {
        if image.texture_descriptor.dimension == TextureDimension::D2
            && image.texture_descriptor.size.depth_or_array_layers > 1
        {
            image.texture_descriptor.dimension = TextureDimension::D3;
        }
        if image.texture_descriptor.dimension != TextureDimension::D3 {
            continue;
        }
        let needs_fix = image
            .texture_view_descriptor
            .as_ref()
            .and_then(|desc| desc.dimension)
            != Some(TextureViewDimension::D3);
        if needs_fix {
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::D3),
                ..default()
            });
        }
    }
}

fn build_tessellated_glyph_mesh(subdivisions: u32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};

    let n = subdivisions + 1;
    let mut positions = Vec::with_capacity((n * n) as usize);
    let mut uvs = Vec::with_capacity((n * n) as usize);
    let mut normals = Vec::with_capacity((n * n) as usize);
    for y in 0..n {
        for x in 0..n {
            let u = x as f32 / subdivisions as f32;
            let v = y as f32 / subdivisions as f32;
            positions.push([u, v, 0.0]);
            uvs.push([u, v]);
            normals.push([0.0, 0.0, 1.0]);
        }
    }
    let mut indices = Vec::with_capacity((subdivisions * subdivisions * 6) as usize);
    for y in 0..subdivisions {
        for x in 0..subdivisions {
            let i0 = y * n + x;
            let i1 = i0 + 1;
            let i2 = i0 + n;
            let i3 = i2 + 1;
            indices.extend_from_slice(&[i0, i2, i1, i1, i2, i3]);
        }
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn init_typeface_state(
    mut commands: Commands,
    bytes: Res<TypefaceFontBytes>,
    atlas_size: Res<PluginAtlasSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = load_font(&bytes.0).expect("typeface font must parse");
    let mut shaper =
        ShapingEngine::new_with_font(bytes.0.clone()).expect("typeface shaper must init");
    let mut atlas = TypefaceAtlas::new_cpu(atlas_size.0);
    prewarm_digit_glyphs(&font, &mut shaper, &mut atlas, PREWARM_PX);
    let mut numeric_diag = NumericDamageDiagnostics::default();
    let numeric_table = build_numeric_glyph_run_table(
        &font,
        &mut shaper,
        &mut atlas,
        PREWARM_PX,
        NUMERIC_DAMAGE_DEFAULT_WIDTH,
        &mut numeric_diag,
    );
    commands.insert_resource(TypefaceFont(font));
    commands.insert_resource(TypefaceShaper(shaper));
    commands.insert_resource(numeric_diag);
    commands.insert_resource(numeric_table);

    let atlas_image = create_atlas_image_from_cpu(&mut images, &atlas.cpu);
    commands.insert_resource(crate::text_render::TextAtlasImageHandle(atlas_image));
    commands.insert_resource(atlas);

    let quad = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    let tess = meshes.add(build_tessellated_glyph_mesh(u32::from(
        DEFORM_TESS_LEVEL_DEFORM,
    )));
    commands.insert_resource(TextQuadMesh(quad.clone()));
    commands.insert_resource(TextDeformTessMesh(tess));

    let draw_entity = commands
        .spawn((
            Mesh2d(quad),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            TextGlyphInstances::default(),
            crate::text_render::TextInstancedDraw,
            bevy::render::batching::NoAutomaticBatching,
            SyncToRenderWorld,
            NoFrustumCulling,
        ))
        .id();
    commands.insert_resource(TextDrawEntity(draw_entity));
}

const PREWARM_PX: f32 = 24.0;

fn prewarm_digit_glyphs(
    font: &ProbeFont,
    shaper: &mut ShapingEngine,
    atlas: &mut TypefaceAtlas,
    px: f32,
) {
    for ch in "0123456789-".chars() {
        let text = ch.to_string();
        let shaped = shaper.shape(&text, px);
        for glyph in &shaped.glyphs {
            let _ = atlas.cpu.get_or_rasterize(font, glyph.glyph_id, px);
        }
    }
    atlas.cpu.clear_dirty_regions();
}

fn is_numeric_damage_label(text: &str) -> bool {
    let Some(rest) = text.strip_prefix('-') else {
        return false;
    };
    !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit())
}

fn update_numeric_damage_labels(
    table: Res<NumericGlyphRunTable>,
    mut num_diag: ResMut<NumericDamageDiagnostics>,
    mut diagnostics: ResMut<TextPerfDiagnostics>,
    mut phase: ResMut<TextDamagePhaseProfile>,
    mut aggregate_version: ResMut<TextAggregateVersion>,
    mut q: Query<
        (Entity, &NumericDamageLabel, Option<&mut TextGlyphInstances>),
        Or<(Added<NumericDamageLabel>, Changed<NumericDamageLabel>)>,
    >,
    mut commands: Commands,
) {
    for (entity, label, existing_instances) in &mut q {
        num_diag.numeric_label_update_count += 1;
        num_diag.numeric_shape_bypass_count += 1;
        let instance_start = std::time::Instant::now();
        if let Some(mut existing) = existing_instances {
            if existing.0.len() == table.glyph_count {
                patch_numeric_instances(
                    &table,
                    label.value,
                    label.color,
                    &mut existing.0,
                    &mut num_diag,
                );
            } else {
                existing.0.clear();
                existing
                    .0
                    .resize(table.glyph_count, GlyphInstanceGpu::default());
                table.write_run(label.value, label.color, &mut existing.0);
                num_diag.numeric_glyph_instance_patch_count += table.glyph_count as u64;
            }
        } else {
            commands.entity(entity).insert(TextGlyphInstances(
                table.compose_run(label.value, label.color),
            ));
        }
        phase.instance_rebuild_ns += instance_start.elapsed().as_nanos() as u64;
        diagnostics.instance_rebuild_count += 1;
        aggregate_version.dirty = true;
        commands.entity(entity).insert(SegmentDirty);
    }
}

fn sync_studio_typeface_labels(
    mut q: Query<
        (
            Entity,
            &crate::studio_labels::StudioTypefaceLabel,
            Option<&TextLabel>,
            Option<&NumericDamageLabel>,
        ),
        Or<(
            Added<crate::studio_labels::StudioTypefaceLabel>,
            Changed<crate::studio_labels::StudioTypefaceLabel>,
        )>,
    >,
    icons: Option<Res<crate::studio_labels::TypefaceIconSet>>,
    mut studio_diag: ResMut<crate::studio_labels::StudioTypefaceLabelDiagnostics>,
    mut commands: Commands,
) {
    for (entity, studio, existing_text, existing_numeric) in &mut q {
        if existing_text.is_some() || existing_numeric.is_some() {
            studio_diag.labels_updated += 1;
        } else {
            studio_diag.labels_spawned += 1;
        }

        let bake = icons.as_deref().map(|set| &set.bake);
        let display = crate::studio_labels::resolve_studio_display_text(
            &studio.text,
            &studio.icon_name,
            bake,
            &mut studio_diag,
        );

        if studio.kind == crate::studio_labels::StudioLabelKind::DamageText
            && crate::studio_labels::try_parse_damage_value(&studio.text).is_some()
        {
            let value = crate::studio_labels::try_parse_damage_value(&studio.text).unwrap_or(0);
            commands.entity(entity).insert(NumericDamageLabel {
                value,
                width: NUMERIC_DAMAGE_DEFAULT_WIDTH,
                px: studio.px,
                color: studio.color,
            });
            commands.entity(entity).remove::<TextLabel>();
            continue;
        }

        let text_label = studio_typeface_to_text_label(studio, &display);
        commands.entity(entity).insert(text_label);
        commands.entity(entity).remove::<NumericDamageLabel>();
    }
}

fn studio_typeface_to_text_label(
    studio: &crate::studio_labels::StudioTypefaceLabel,
    display: &str,
) -> TextLabel {
    let mut label = match studio.render_mode {
        TextLabelRenderMode::Msdf => TextLabel::msdf(display, studio.px, studio.color),
        TextLabelRenderMode::Sdf => TextLabel {
            text: display.to_string(),
            px: studio.px,
            color: studio.color,
            render_mode: TextLabelRenderMode::Sdf,
            style_slot: studio.style_slot,
            deform_slot: studio.deform_slot,
            path_slot: studio.path_slot,
            warp_slot: studio.warp_slot,
        },
        TextLabelRenderMode::Raster => TextLabel::raster(display, studio.px, studio.color),
    };
    label.style_slot = studio.style_slot;
    label.deform_slot = studio.deform_slot;
    label.path_slot = studio.path_slot;
    label.warp_slot = studio.warp_slot;
    label.render_mode = studio.render_mode;
    label
}

fn emit_studio_damage_text_labels(
    mut emitters: Query<&mut crate::studio_labels::StudioDamageTextEmitter>,
    mut commands: Commands,
) {
    for mut emitter in &mut emitters {
        if emitter.pending_values.is_empty() {
            continue;
        }
        for value in emitter.pending_values.drain(..) {
            commands.spawn(crate::studio_labels::StudioTypefaceLabel::damage_value(
                value,
                24.0,
                [1.0, 0.35, 0.25, 1.0],
            ));
        }
    }
}

fn label_text_uses_manifest_icons(
    text: &str,
    icons: &crate::studio_labels::TypefaceIconSet,
) -> bool {
    text.chars()
        .any(|ch| icons.bake.codepoint_to_name.contains_key(&(ch as u32)))
}

fn rebuild_changed_labels(
    mut diagnostics: ResMut<TextPerfDiagnostics>,
    mut phase: ResMut<TextDamagePhaseProfile>,
    mut aggregate_version: ResMut<TextAggregateVersion>,
    font: Res<TypefaceFont>,
    mut shaper: ResMut<TypefaceShaper>,
    mut atlas: ResMut<TypefaceAtlas>,
    icons: Option<Res<crate::studio_labels::TypefaceIconSet>>,
    mut q: Query<
        (
            Entity,
            &TextLabel,
            Option<&mut TextGlyphInstances>,
            Option<&mut TextLabelCache>,
        ),
        Or<(Added<TextLabel>, Changed<TextLabel>)>,
    >,
    mut commands: Commands,
) {
    for (entity, label, existing_instances, cache) in &mut q {
        diagnostics.shape_rebuild_count += 1;
        let shape_start = std::time::Instant::now();
        let (shaped, cache_hit) = if is_numeric_damage_label(&label.text) {
            shaper.0.shape_cached(&label.text, label.px)
        } else {
            (shaper.0.shape(&label.text, label.px), false)
        };
        phase.shaping_ns += shape_start.elapsed().as_nanos() as u64;
        if cache_hit {
            diagnostics.shape_cache_hit_count += 1;
        } else {
            diagnostics.shape_cache_miss_count += 1;
        }

        let instance_start = std::time::Instant::now();
        let raster_start = std::time::Instant::now();
        let is_distance_field = label.render_mode != TextLabelRenderMode::Raster;
        let icon_instances = icons.as_deref().and_then(|set| {
            if !label_text_uses_manifest_icons(&label.text, set) {
                return None;
            }
            set.icons
                .build_mixed_instances(
                    &font.0,
                    &mut shaper.0,
                    &mut atlas.cpu,
                    &label.text,
                    label.px,
                    label.color,
                )
                .ok()
        });
        let run_width = shaped
            .glyphs
            .last()
            .map(|g| g.x + g.advance)
            .unwrap_or(1.0)
            .max(1.0);
        if let Some(mut instances) = icon_instances {
            for instance in &mut instances {
                instance.style_params = style_params_for_slot(label.style_slot, 0);
                instance.deform_params = deform_params_for_slot(
                    label.deform_slot,
                    tess_level_for_deform_slot(label.deform_slot),
                );
                instance.path_params = path_params_for_slot(label.path_slot, 0.0, 1.0);
                instance.warp_params = warp_params_for_slot(label.warp_slot, 1.0);
            }
            if let Some(mut existing) = existing_instances {
                existing.0.clear();
                existing.0.extend_from_slice(&instances);
            } else {
                commands
                    .entity(entity)
                    .insert(TextGlyphInstances(instances));
            }
        } else if let Some(mut existing) = existing_instances {
            existing.0.clear();
            existing.0.reserve(shaped.glyphs.len());
            for glyph in &shaped.glyphs {
                if let Some(instance) =
                    build_glyph_instance(glyph, label, &font.0, &mut atlas, run_width)
                {
                    existing.0.push(instance);
                }
            }
            if is_distance_field {
                atlas
                    .distance_field
                    .record_production_msdf_label(existing.0.len());
            }
        } else {
            let mut instances = Vec::with_capacity(shaped.glyphs.len());
            for glyph in &shaped.glyphs {
                if let Some(instance) =
                    build_glyph_instance(glyph, label, &font.0, &mut atlas, run_width)
                {
                    instances.push(instance);
                }
            }
            if is_distance_field {
                atlas
                    .distance_field
                    .record_production_msdf_label(instances.len());
            }
            commands
                .entity(entity)
                .insert(TextGlyphInstances(instances));
        }
        phase.rasterize_ns += raster_start.elapsed().as_nanos() as u64;
        phase.instance_rebuild_ns += instance_start.elapsed().as_nanos() as u64;
        diagnostics.instance_rebuild_count += 1;
        aggregate_version.dirty = true;
        commands.entity(entity).insert(SegmentDirty);

        if let Some(mut cache) = cache {
            cache.text = label.text.clone();
            cache.px = label.px;
            cache.color = label.color;
            cache.render_mode = label.render_mode;
            cache.style_slot = label.style_slot;
            cache.deform_slot = label.deform_slot;
            cache.path_slot = label.path_slot;
            cache.warp_slot = label.warp_slot;
            cache.shaped = shaped;
        } else {
            commands.entity(entity).insert(TextLabelCache {
                text: label.text.clone(),
                px: label.px,
                color: label.color,
                render_mode: label.render_mode,
                style_slot: label.style_slot,
                deform_slot: label.deform_slot,
                path_slot: label.path_slot,
                warp_slot: label.warp_slot,
                shaped,
            });
        }
    }
}

fn build_glyph_instance(
    glyph: &ShapedGlyph,
    label: &TextLabel,
    font: &ProbeFont,
    atlas: &mut TypefaceAtlas,
    run_width: f32,
) -> Option<GlyphInstanceGpu> {
    let path_u = if run_width > 0.0 {
        glyph.x / run_width
    } else {
        0.0
    };
    match label.render_mode {
        TextLabelRenderMode::Raster => {
            let tile = atlas.cpu.get_or_rasterize(font, glyph.glyph_id, label.px)?;
            Some(build_instance(
                glyph,
                tile,
                label.color,
                atlas.atlas_size,
                None,
                label.style_slot,
                label.deform_slot,
                label.path_slot,
                label.warp_slot,
                path_u,
            ))
        }
        TextLabelRenderMode::Sdf | TextLabelRenderMode::Msdf => {
            let kind = label.render_mode.distance_field_kind()?;
            let df_tile = atlas
                .distance_field
                .get_or_generate_glyph_into_shared_atlas(
                    &mut atlas.cpu,
                    font,
                    glyph.glyph_id as u32,
                    label.px,
                    kind,
                )
                .ok()?;
            Some(build_instance(
                glyph,
                df_tile.atlas_tile,
                label.color,
                atlas.atlas_size,
                Some(&df_tile),
                label.style_slot,
                label.deform_slot,
                label.path_slot,
                label.warp_slot,
                path_u,
            ))
        }
    }
}

fn mark_aggregate_dirty_on_label_lifecycle(
    mut aggregate_version: ResMut<TextAggregateVersion>,
    mut layout: ResMut<TextAggregateLayout>,
    added_text: Query<(), Added<TextLabel>>,
    added_numeric: Query<(), Added<NumericDamageLabel>>,
    mut removed_text: RemovedComponents<TextLabel>,
    mut removed_numeric: RemovedComponents<NumericDamageLabel>,
) {
    if added_text.iter().next().is_some()
        || added_numeric.iter().next().is_some()
        || removed_text.read().next().is_some()
        || removed_numeric.read().next().is_some()
    {
        aggregate_version.dirty = true;
        layout.needs_full_rebuild = true;
    }
}

fn sync_atlas_image_to_gpu(
    mut atlas: ResMut<TypefaceAtlas>,
    atlas_handle: Res<crate::text_render::TextAtlasImageHandle>,
    mut images: ResMut<Assets<Image>>,
    mut diagnostics: ResMut<TextPerfDiagnostics>,
    mut phase: ResMut<TextDamagePhaseProfile>,
) {
    let dirty_bytes = atlas.cpu.dirty_region_byte_count();
    if dirty_bytes == 0 {
        return;
    }

    let sync_start = std::time::Instant::now();
    let dirty_regions: Vec<AtlasDirtyRect> = atlas.cpu.dirty_regions().collect();
    let dirty_count = dirty_regions.len() as u64;
    diagnostics.atlas_sync_count += 1;
    diagnostics.atlas_sync_bytes += dirty_bytes;
    diagnostics.atlas_dirty_region_count += dirty_count;

    if let Some(image) = images.get_mut(&atlas_handle.0) {
        let size = atlas.atlas_size;
        image.resize(Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        });
        if let Some(data) = image.data.as_mut() {
            let staging = atlas.cpu.staging_pixels();
            for rect in dirty_regions {
                blit_dirty_rect_to_image(data, size, staging, rect);
            }
        }
    }
    atlas.cpu.clear_dirty_regions();
    phase.atlas_sync_ns += sync_start.elapsed().as_nanos() as u64;
}

fn force_text_draw_visible(draw_entity: Res<TextDrawEntity>, mut q: Query<&mut ViewVisibility>) {
    if let Ok(mut visibility) = q.get_mut(draw_entity.0) {
        visibility.set();
    }
}

fn sync_draw_entity_instances(
    aggregate: Res<TextInstanceAggregate>,
    aggregate_version: Res<TextAggregateVersion>,
    draw_entity: Res<TextDrawEntity>,
    mut q: Query<&mut TextGlyphInstances>,
    mut diagnostics: ResMut<TextPerfDiagnostics>,
    mut phase: ResMut<TextDamagePhaseProfile>,
    mut last_synced_version: Local<u64>,
) {
    if *last_synced_version == aggregate_version.current {
        return;
    }
    let sync_start = std::time::Instant::now();
    *last_synced_version = aggregate_version.current;
    diagnostics.draw_entity_sync_count += 1;
    if let Ok(mut instances) = q.get_mut(draw_entity.0) {
        if instances.0.len() != aggregate.0.len() {
            instances.0.clear();
            instances.0.extend_from_slice(&aggregate.0);
        } else if instances.0 != aggregate.0 {
            instances.0.copy_from_slice(&aggregate.0);
        }
    }
    phase.draw_sync_ns += sync_start.elapsed().as_nanos() as u64;
}

fn aggregate_label_instances(
    all_labels: Query<
        (Entity, &TextGlyphInstances),
        Or<(With<TextLabel>, With<NumericDamageLabel>)>,
    >,
    dirty_labels: Query<
        (Entity, &TextGlyphInstances, &LabelAggregateSegment),
        (
            Or<(With<TextLabel>, With<NumericDamageLabel>)>,
            With<SegmentDirty>,
        ),
    >,
    mut aggregate: ResMut<TextInstanceAggregate>,
    mut aggregate_version: ResMut<TextAggregateVersion>,
    mut layout: ResMut<TextAggregateLayout>,
    mut diagnostics: ResMut<TextPerfDiagnostics>,
    mut phase: ResMut<TextDamagePhaseProfile>,
    mut commands: Commands,
) {
    if !aggregate_version.dirty {
        return;
    }
    aggregate_version.dirty = false;
    aggregate_version.current += 1;
    phase.sample_frames += 1;

    let dirty: Vec<_> = dirty_labels.iter().collect();
    let total_labels = all_labels.iter().count();
    let width_stable = dirty
        .iter()
        .all(|(_, instances, segment)| instances.0.len() == segment.len);
    let can_patch = !layout.needs_full_rebuild
        && !dirty.is_empty()
        && width_stable
        && dirty.len() < total_labels;

    if can_patch {
        let patch_start = std::time::Instant::now();
        for (entity, instances, segment) in &dirty {
            aggregate.0[segment.offset..segment.offset + segment.len].copy_from_slice(&instances.0);
            diagnostics.aggregate_patch_count += 1;
            diagnostics.aggregate_patched_instance_count += instances.0.len() as u64;
            commands.entity(*entity).remove::<SegmentDirty>();
        }
        phase.aggregate_patch_ns += patch_start.elapsed().as_nanos() as u64;
        return;
    }

    let full_start = std::time::Instant::now();
    if !layout.needs_full_rebuild && !dirty.is_empty() && !width_stable {
        diagnostics.aggregate_repack_count += 1;
    }

    let mut ordered: Vec<(Entity, &TextGlyphInstances)> = all_labels.iter().collect();
    ordered.sort_by_key(|(entity, _)| entity.index());

    let required: usize = ordered.iter().map(|(_, instances)| instances.0.len()).sum();
    aggregate.0.clear();
    aggregate.0.reserve(required);

    let mut offset = 0usize;
    for (entity, instances) in &ordered {
        let len = instances.0.len();
        commands
            .entity(*entity)
            .insert(LabelAggregateSegment { offset, len });
        commands.entity(*entity).remove::<SegmentDirty>();
        aggregate.0.extend_from_slice(&instances.0);
        offset += len;
    }

    layout.label_order = ordered.into_iter().map(|(entity, _)| entity).collect();
    layout.needs_full_rebuild = false;

    diagnostics.aggregate_rebuild_count += 1;
    diagnostics.aggregate_full_rebuild_count += 1;
    diagnostics.aggregate_full_rebuild_instance_count += aggregate.0.len() as u64;
    phase.aggregate_full_rebuild_ns += full_start.elapsed().as_nanos() as u64;
}

fn build_instance(
    glyph: &ShapedGlyph,
    tile: AtlasTile,
    color: [f32; 4],
    atlas_size: u32,
    distance_field: Option<&DistanceFieldTile>,
    style_slot: TextStyleSlot,
    deform_slot: crate::deform::TextDeformSlot,
    path_slot: crate::path::TextPathSlot,
    warp_slot: crate::warp::TextWarpSlot,
    path_u: f32,
) -> GlyphInstanceGpu {
    let inv = 1.0 / atlas_size as f32;
    let sdf_params = distance_field
        .map(|df| crate::msdf::sdf_params_for_distance_field_tile(df, atlas_size))
        .unwrap_or([0.0; 4]);
    let tess = tess_level_for_deform_slot(deform_slot);
    GlyphInstanceGpu {
        pos_size: [
            glyph.x + tile.left as f32,
            glyph.y + tile.top as f32,
            tile.w as f32,
            tile.h as f32,
        ],
        uv_rect: [
            tile.x as f32 * inv,
            tile.y as f32 * inv,
            (tile.x + tile.w) as f32 * inv,
            (tile.y + tile.h) as f32 * inv,
        ],
        color,
        sdf_params,
        style_params: style_params_for_slot(style_slot, 0),
        deform_params: deform_params_for_slot(deform_slot, tess),
        path_params: path_params_for_slot(path_slot, path_u, 1.0),
        warp_params: warp_params_for_slot(warp_slot, 1.0),
    }
}

fn advance_style_table_time(mut style_table: ResMut<TextStyleTableResource>, time: Res<Time>) {
    style_table.time = time.elapsed_secs();
}

fn sync_style_table_rows_if_changed(
    mut style_table: ResMut<TextStyleTableResource>,
    mut extracted: ResMut<ExtractedTextStyleTable>,
    mut diagnostics: ResMut<TextStyleDiagnostics>,
) {
    extracted.globals = style_table.table.to_globals(style_table.time);
    if !style_table.rows_dirty {
        diagnostics.style_table_cache_hit_count += 1;
        return;
    }
    extracted.rows = style_table.table.to_rows_uniform();
    style_table.rows_generation += 1;
    extracted.rows_generation = style_table.rows_generation;
    style_table.mark_rows_clean();
    diagnostics.style_table_upload_count += 1;
}

pub fn text_style_diagnostics(app: &App) -> TextStyleDiagnostics {
    app.world()
        .get_resource::<TextStyleDiagnostics>()
        .copied()
        .unwrap_or_default()
}

fn sync_deform_table_rows_if_changed(
    mut deform_table: ResMut<TextDeformTableResource>,
    mut extracted: ResMut<ExtractedTextDeformTable>,
    mut diagnostics: ResMut<TextDeformDiagnostics>,
) {
    if !deform_table.rows_dirty {
        diagnostics.deform_table_cache_hit_count += 1;
        return;
    }
    extracted.rows = deform_table.table.to_rows_uniform();
    deform_table.rows_generation += 1;
    extracted.rows_generation = deform_table.rows_generation;
    deform_table.mark_rows_clean();
    diagnostics.deform_table_upload_count += 1;
}

pub fn text_deform_diagnostics(app: &App) -> TextDeformDiagnostics {
    app.world()
        .get_resource::<TextDeformDiagnostics>()
        .copied()
        .unwrap_or_default()
}

fn sync_path_table_rows_if_changed(
    mut path_table: ResMut<TextPathTableResource>,
    mut extracted: ResMut<ExtractedTextPathTable>,
    mut diagnostics: ResMut<TextPathWarpDiagnostics>,
) {
    if !path_table.rows_dirty {
        diagnostics.path_table_cache_hit_count += 1;
        return;
    }
    extracted.rows = path_table.table.to_rows_uniform();
    path_table.rows_generation += 1;
    extracted.rows_generation = path_table.rows_generation;
    path_table.mark_rows_clean();
    diagnostics.path_table_upload_count += 1;
}

fn sync_warp_table_rows_if_changed(
    mut warp_table: ResMut<TextWarpTableResource>,
    mut extracted: ResMut<ExtractedTextWarpTable>,
    mut diagnostics: ResMut<TextPathWarpDiagnostics>,
) {
    if !warp_table.rows_dirty {
        diagnostics.warp_table_cache_hit_count += 1;
        return;
    }
    extracted.rows = warp_table.table.to_rows_uniform();
    warp_table.rows_generation += 1;
    extracted.rows_generation = warp_table.rows_generation;
    warp_table.mark_rows_clean();
    diagnostics.warp_table_upload_count += 1;
}

pub fn text_path_warp_diagnostics(app: &App) -> TextPathWarpDiagnostics {
    app.world()
        .get_resource::<TextPathWarpDiagnostics>()
        .copied()
        .unwrap_or_default()
}

fn sync_path_warp_instance_diagnostics(
    aggregate: Res<TextInstanceAggregate>,
    mut diagnostics: ResMut<TextPathWarpDiagnostics>,
    mut last_counts: Local<(u64, u64)>,
) {
    let path_count = aggregate
        .0
        .iter()
        .filter(|i| i.path_params[0] > 0.0)
        .count() as u64;
    let warp_count = aggregate
        .0
        .iter()
        .filter(|i| i.warp_params[0] > 0.0)
        .count() as u64;
    diagnostics.path_instance_count = path_count;
    diagnostics.warp_instance_count = warp_count;
    if *last_counts == (path_count, warp_count) {
        diagnostics.path_warp_noop_reuse_count += 1;
    } else {
        diagnostics.path_warp_rebuild_count += 1;
        *last_counts = (path_count, warp_count);
    }
}

fn sync_draw_entity_mesh_for_deformation(
    aggregate: Res<TextInstanceAggregate>,
    draw_entity: Res<TextDrawEntity>,
    quad: Res<TextQuadMesh>,
    tess: Res<TextDeformTessMesh>,
    mut q: Query<&mut Mesh2d>,
    mut deform_diag: ResMut<TextDeformDiagnostics>,
    mut last_tess: Local<bool>,
) {
    let needs_tess = aggregate
        .0
        .iter()
        .any(|instance| instance.deform_params[0] > 0.0);
    deform_diag.deform_instance_count = aggregate
        .0
        .iter()
        .filter(|instance| instance.deform_params[0] > 0.0)
        .count() as u64;
    deform_diag.tessellated_label_count = aggregate
        .0
        .iter()
        .filter(|instance| instance.deform_params[1] > 0.0)
        .count() as u64;
    if needs_tess {
        deform_diag.tessellated_vertex_count =
            crate::deform::tessellated_vertex_count(u32::from(DEFORM_TESS_LEVEL_DEFORM)) as u64;
    } else {
        deform_diag.tessellated_vertex_count = 0;
    }

    let Ok(mut mesh2d) = q.get_mut(draw_entity.0) else {
        return;
    };
    let target = if needs_tess { &tess.0 } else { &quad.0 };
    if mesh2d.0 != *target {
        mesh2d.0 = target.clone();
        deform_diag.deformation_rebuild_count += 1;
        *last_tess = needs_tess;
    } else if *last_tess == needs_tess {
        deform_diag.deformation_noop_reuse_count += 1;
    }
}

fn blit_dirty_rect_to_image(
    image: &mut [u8],
    atlas_size: u32,
    staging: &[u8],
    rect: AtlasDirtyRect,
) {
    for row in 0..rect.h {
        let src_row = (rect.y + row) * atlas_size;
        let src_start = (src_row * 4 + rect.x * 4) as usize;
        let dst_row = (rect.y + row) * atlas_size;
        let dst_start = (dst_row * 4 + rect.x * 4) as usize;
        let len = rect.w as usize * 4;
        image[dst_start..dst_start + len].copy_from_slice(&staging[src_start..src_start + len]);
    }
}

/// Build a deterministic offscreen atlas image handle for smoke/readback tests.
pub fn create_atlas_image_from_cpu(
    images: &mut Assets<Image>,
    core: &GlyphAtlasCore,
) -> Handle<Image> {
    let size = core.atlas_size();
    let mut image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        core.staging_pixels(),
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.texture_descriptor.usage |=
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST;
    images.add(image)
}

/// Build an offscreen RGBA8 render target for shader smoke readback.
pub fn create_render_target_image(
    images: &mut Assets<Image>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &vec![0u8; (width * height * 4) as usize],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT
        | TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_SRC
        | TextureUsages::COPY_DST;
    images.add(image)
}

/// Spawn many deterministic static labels for Bevy-path perf tests.
pub fn spawn_static_text_labels(app: &mut App, count: usize, px: f32) {
    let world = app.world_mut();
    for index in 0..count {
        let text = format!("Label {index}");
        world.spawn(TextLabel::raster(text, px, [1.0, 1.0, 1.0, 1.0]));
    }
}

/// Count text/numeric label entities and the single aggregate draw entity.
pub fn text_label_entity_counts(app: &mut App) -> (usize, usize) {
    let world = app.world_mut();
    let mut text = world.query_filtered::<(), With<TextLabel>>();
    let mut numeric = world.query_filtered::<(), With<NumericDamageLabel>>();
    let mut draws = world.query_filtered::<(), With<crate::text_render::TextInstancedDraw>>();
    let label_count = text.iter(world).count() + numeric.iter(world).count();
    let draw_entities = draws.iter(world).count();
    (label_count, draw_entities)
}

pub fn distance_field_diagnostics(app: &App) -> DistanceFieldDiagnostics {
    app.world()
        .get_resource::<TypefaceAtlas>()
        .map(|atlas| atlas.distance_field_diagnostics())
        .unwrap_or_default()
}

pub fn numeric_damage_lane_diagnostics(app: &App) -> NumericDamageDiagnostics {
    numeric_damage_diagnostics(app)
}

/// Measured Bevy Update-path profile for LR5R/LR5S binding proof.
#[derive(Debug, Clone, PartialEq)]
pub struct BevyTextBenchProfile {
    pub labels: usize,
    pub damage_labels: usize,
    pub noop_frames: usize,
    pub damage_frames: usize,
    pub avg_noop_update_ms: f64,
    pub max_noop_update_ms: f64,
    pub avg_damage_update_ms: f64,
    pub max_damage_update_ms: f64,
    pub diagnostics_after_noop: TextPerfDiagnostics,
    pub diagnostics_after_damage: TextPerfDiagnostics,
    pub phase_after_damage: TextDamagePhaseProfile,
}

/// Spawn static + damage labels; returns damage label entities for churn mutation.
pub fn spawn_static_and_damage_labels(
    app: &mut App,
    static_count: usize,
    damage_count: usize,
    px: f32,
) -> Vec<Entity> {
    spawn_static_text_labels(app, static_count, px);
    let mut damage_entities = Vec::with_capacity(damage_count);
    let world = app.world_mut();
    for index in 0..damage_count {
        let entity = world
            .spawn(TextLabel::raster(
                format!("-{index}"),
                px,
                [1.0, 0.35, 0.2, 1.0],
            ))
            .id();
        damage_entities.push(entity);
    }
    damage_entities
}

/// Spawn static text labels + fixed-width [`NumericDamageLabel`] entities for binding proof.
pub fn spawn_static_and_numeric_damage_labels(
    app: &mut App,
    static_count: usize,
    damage_count: usize,
    px: f32,
) -> Vec<Entity> {
    spawn_static_text_labels(app, static_count, px);
    let mut damage_entities = Vec::with_capacity(damage_count);
    let world = app.world_mut();
    for index in 0..damage_count {
        let entity = world
            .spawn(NumericDamageLabel::new(
                -(index as i32),
                px,
                [1.0, 0.35, 0.2, 1.0],
            ))
            .id();
        damage_entities.push(entity);
    }
    damage_entities
}

fn variable_width_damage_text(value: u32) -> String {
    let mut text = String::with_capacity(6);
    text.push('-');
    text.push_str(&value.to_string());
    text
}

fn clear_app_exit(app: &mut App) {
    if let Some(mut exits) = app.world_mut().get_resource_mut::<Events<AppExit>>() {
        exits.clear();
    }
}

fn timed_updates(app: &mut App, frames: usize) -> (f64, f64) {
    if frames == 0 {
        return (0.0, 0.0);
    }
    let mut total_ms = 0.0_f64;
    let mut max_ms = 0.0_f64;
    for _ in 0..frames {
        clear_app_exit(app);
        let start = std::time::Instant::now();
        app.update();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        total_ms += elapsed;
        max_ms = max_ms.max(elapsed);
    }
    (total_ms / frames as f64, max_ms)
}

/// Warm up labels, run noop frames, then damage churn; returns timing profile.
pub fn profile_bevy_text_bench(
    app: &mut App,
    damage_entities: &[Entity],
    noop_frames: usize,
    damage_frames: usize,
) -> BevyTextBenchProfile {
    let (labels, _) = text_label_entity_counts(app);
    clear_app_exit(app);
    app.update();

    let (avg_noop_update_ms, max_noop_update_ms) = timed_updates(app, noop_frames);
    let diagnostics_after_noop = text_perf_diagnostics(app);

    reset_text_damage_phase_profile(app);
    let mut total_damage_ms = 0.0_f64;
    let mut max_damage_update_ms = 0.0_f64;
    for frame in 0..damage_frames {
        for (index, entity) in damage_entities.iter().enumerate() {
            let value =
                ((index.wrapping_mul(17).wrapping_add(frame.wrapping_mul(13))) % 9999) as u32;
            if let Some(mut label) = app.world_mut().get_mut::<TextLabel>(*entity) {
                label.text = variable_width_damage_text(value);
            }
        }
        clear_app_exit(app);
        let start = std::time::Instant::now();
        app.update();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        total_damage_ms += elapsed;
        max_damage_update_ms = max_damage_update_ms.max(elapsed);
    }
    let avg_damage_update_ms = if damage_frames > 0 {
        total_damage_ms / damage_frames as f64
    } else {
        0.0
    };

    BevyTextBenchProfile {
        labels,
        damage_labels: damage_entities.len(),
        noop_frames,
        damage_frames,
        avg_noop_update_ms,
        max_noop_update_ms,
        avg_damage_update_ms,
        max_damage_update_ms,
        diagnostics_after_noop,
        diagnostics_after_damage: text_perf_diagnostics(app),
        phase_after_damage: text_damage_phase_profile(app),
    }
}

/// Fixed-width numeric damage binding profile — no cosmic-text or string formatting per frame.
pub fn profile_bevy_fixed_width_numeric_damage_bench(
    app: &mut App,
    damage_entities: &[Entity],
    noop_frames: usize,
    damage_frames: usize,
) -> BevyTextBenchProfile {
    let (labels, _) = text_label_entity_counts(app);
    clear_app_exit(app);
    app.update();

    let (avg_noop_update_ms, max_noop_update_ms) = timed_updates(app, noop_frames);
    let diagnostics_after_noop = text_perf_diagnostics(app);
    let shape_miss_before = diagnostics_after_noop.shape_cache_miss_count;
    let repack_before = diagnostics_after_noop.aggregate_repack_count;

    reset_text_damage_phase_profile(app);
    let mut total_damage_ms = 0.0_f64;
    let mut max_damage_update_ms = 0.0_f64;
    for frame in 0..damage_frames {
        for (index, entity) in damage_entities.iter().enumerate() {
            let raw = ((index.wrapping_mul(17).wrapping_add(frame.wrapping_mul(13))) % 9999) as i32;
            if let Some(mut label) = app.world_mut().get_mut::<NumericDamageLabel>(*entity) {
                label.value = -raw;
            }
        }
        clear_app_exit(app);
        let start = std::time::Instant::now();
        app.update();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        total_damage_ms += elapsed;
        max_damage_update_ms = max_damage_update_ms.max(elapsed);
    }
    let avg_damage_update_ms = if damage_frames > 0 {
        total_damage_ms / damage_frames as f64
    } else {
        0.0
    };

    let diagnostics_after_damage = text_perf_diagnostics(app);
    debug_assert_eq!(
        diagnostics_after_damage.shape_cache_miss_count, shape_miss_before,
        "fixed-width numeric lane must not miss shape cache during timed damage frames"
    );
    debug_assert_eq!(
        diagnostics_after_damage.aggregate_repack_count, repack_before,
        "fixed-width numeric lane must not repack aggregate during timed damage frames"
    );

    BevyTextBenchProfile {
        labels,
        damage_labels: damage_entities.len(),
        noop_frames,
        damage_frames,
        avg_noop_update_ms,
        max_noop_update_ms,
        avg_damage_update_ms,
        max_damage_update_ms,
        diagnostics_after_noop,
        diagnostics_after_damage,
        phase_after_damage: text_damage_phase_profile(app),
    }
}
