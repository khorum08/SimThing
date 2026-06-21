use bevy::{
    asset::{load_internal_asset, Assets, Handle},
    math::primitives::Rectangle,
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        render_asset::RenderAssetUsages,
        sync_world::SyncToRenderWorld,
        render_resource::{
            Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
            TextureViewDimension,
        },
        view::NoFrustumCulling,
    },
};

use crate::{
    atlas::{AtlasTile, GlyphAtlasCore, GlyphAtlasStats},
    font::{load_font, ProbeFont},
    shaping::{ShapedGlyph, ShapedRun, ShapingEngine},
    text_render::TextInstancedRenderPlugin,
};

pub(crate) const TEXT_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x5459_5045_4c52_3300_0000_0000_0000_0001);

/// Bevy plugin for instanced atlas text labels.
#[derive(Clone)]
pub struct SimthingToolsTextPlugin {
    font_bytes: Vec<u8>,
}

impl SimthingToolsTextPlugin {
    pub fn new(font_bytes: Vec<u8>) -> Self {
        Self { font_bytes }
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
            .init_resource::<TextRebuildDiagnostics>()
            .init_resource::<TextInstanceAggregate>()
            .insert_resource(TypefaceFontBytes(self.font_bytes.clone()))
            .add_systems(
                Startup,
                (fix_volume_image_view_descriptors, init_typeface_state).chain(),
            )
            .add_systems(PostStartup, fix_volume_image_view_descriptors)
            .add_systems(Update, rebuild_changed_labels)
            .add_systems(
                Update,
                (
                    aggregate_label_instances.after(rebuild_changed_labels),
                    sync_draw_entity_instances.after(aggregate_label_instances),
                    sync_atlas_image_to_gpu.after(rebuild_changed_labels),
                    force_text_draw_visible,
                ),
            )
            .add_plugins(ExtractComponentPlugin::<TextDrawExtract>::default())
            .add_plugins(TextInstancedRenderPlugin);
    }

    fn finish(&self, app: &mut App) {
        let _ = app;
    }
}

/// Workshop/production text label component.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct TextLabel {
    pub text: String,
    pub px: f32,
    pub color: [f32; 4],
}

/// Diagnostics counters for LR3 changed-detection tests.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRebuildDiagnostics {
    pub shape_rebuild_count: u64,
    pub instance_rebuild_count: u64,
}

/// CPU atlas used by the text plugin.
#[derive(Resource)]
pub struct TypefaceAtlas {
    pub cpu: GlyphAtlasCore,
    pub atlas_size: u32,
}

impl TypefaceAtlas {
    pub fn new_cpu(size: u32) -> Self {
        Self {
            cpu: GlyphAtlasCore::new(size),
            atlas_size: size,
        }
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
    shaped: ShapedRun,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct GlyphInstanceGpu {
    pub pos_size: [f32; 4],
    pub uv_rect: [f32; 4],
    pub color: [f32; 4],
}

#[derive(Component, Clone, Default, Debug)]
pub struct TextGlyphInstances(pub Vec<GlyphInstanceGpu>);

#[derive(Resource, Default)]
pub struct TextInstanceAggregate(pub Vec<GlyphInstanceGpu>);

#[derive(Component, Clone)]
pub struct TextDrawExtract {
    pub(crate) instances: Vec<GlyphInstanceGpu>,
}

impl bevy::render::extract_component::ExtractComponent for TextDrawExtract {
    type QueryData = &'static TextGlyphInstances;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: bevy::ecs::query::QueryItem<Self::QueryData>) -> Option<Self> {
        if item.0.is_empty() {
            return None;
        }
        Some(Self {
            instances: item.0.clone(),
        })
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

fn init_typeface_state(
    mut commands: Commands,
    bytes: Res<TypefaceFontBytes>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = load_font(&bytes.0).expect("typeface font must parse");
    let shaper = ShapingEngine::new_with_font(bytes.0.clone()).expect("typeface shaper must init");
    commands.insert_resource(TypefaceFont(font));
    commands.insert_resource(TypefaceShaper(shaper));

    let atlas = TypefaceAtlas::new_cpu(512);
    let atlas_image = create_atlas_image_from_cpu(&mut images, &atlas.cpu);
    commands.insert_resource(crate::text_render::TextAtlasImageHandle(atlas_image));
    commands.insert_resource(atlas);

    let quad = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    commands.insert_resource(TextQuadMesh(quad.clone()));

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

fn rebuild_changed_labels(
    mut diagnostics: ResMut<TextRebuildDiagnostics>,
    font: Res<TypefaceFont>,
    mut shaper: ResMut<TypefaceShaper>,
    mut atlas: ResMut<TypefaceAtlas>,
    mut q: Query<
        (Entity, &TextLabel, Option<&mut TextLabelCache>),
        Or<(Added<TextLabel>, Changed<TextLabel>)>,
    >,
    mut commands: Commands,
) {
    for (entity, label, cache) in &mut q {
        diagnostics.shape_rebuild_count += 1;
        let shaped = shaper.0.shape(&label.text, label.px);
        let mut instances = Vec::new();
        for glyph in &shaped.glyphs {
            if let Some(tile) = atlas
                .cpu
                .get_or_rasterize(&font.0, glyph.glyph_id, label.px)
            {
                instances.push(build_instance(glyph, tile, label.color, atlas.atlas_size));
            }
        }
        diagnostics.instance_rebuild_count += 1;

        if let Some(mut cache) = cache {
            cache.text = label.text.clone();
            cache.px = label.px;
            cache.color = label.color;
            cache.shaped = shaped;
        } else {
            commands.entity(entity).insert(TextLabelCache {
                text: label.text.clone(),
                px: label.px,
                color: label.color,
                shaped,
            });
        }
        commands
            .entity(entity)
            .insert(TextGlyphInstances(instances));
    }
}

fn sync_atlas_image_to_gpu(
    atlas: Res<TypefaceAtlas>,
    atlas_handle: Res<crate::text_render::TextAtlasImageHandle>,
    mut images: ResMut<Assets<Image>>,
    rebuild: Res<TextRebuildDiagnostics>,
    mut last_sync: Local<u64>,
) {
    if *last_sync == rebuild.shape_rebuild_count {
        return;
    }
    *last_sync = rebuild.shape_rebuild_count;
    if let Some(image) = images.get_mut(&atlas_handle.0) {
        let size = atlas.atlas_size;
        let pixels = atlas.cpu.staging_pixels();
        image.resize(Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        });
        if let Some(data) = image.data.as_mut() {
            data.copy_from_slice(pixels);
        }
    }
}

fn force_text_draw_visible(
    draw_entity: Res<TextDrawEntity>,
    mut q: Query<&mut ViewVisibility>,
) {
    if let Ok(mut visibility) = q.get_mut(draw_entity.0) {
        visibility.set();
    }
}

fn sync_draw_entity_instances(
    aggregate: Res<TextInstanceAggregate>,
    draw_entity: Res<TextDrawEntity>,
    mut q: Query<&mut TextGlyphInstances>,
) {
    if let Ok(mut instances) = q.get_mut(draw_entity.0) {
        instances.0.clone_from(&aggregate.0);
    }
}

fn aggregate_label_instances(
    q: Query<&TextGlyphInstances, With<TextLabel>>,
    mut aggregate: ResMut<TextInstanceAggregate>,
) {
    aggregate.0.clear();
    for instances in &q {
        aggregate.0.extend_from_slice(&instances.0);
    }
}

fn build_instance(
    glyph: &ShapedGlyph,
    tile: AtlasTile,
    color: [f32; 4],
    atlas_size: u32,
) -> GlyphInstanceGpu {
    let inv = 1.0 / atlas_size as f32;
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
pub fn create_render_target_image(images: &mut Assets<Image>, width: u32, height: u32) -> Handle<Image> {
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