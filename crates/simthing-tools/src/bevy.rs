use bevy::{
    asset::{load_internal_asset, Assets, Handle},
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, Shader, TextureDimension, TextureFormat, TextureUsages},
    },
};

use crate::{
    atlas::{AtlasTile, GlyphAtlasCore, GlyphAtlasStats},
    font::{load_font, ProbeFont},
    shaping::{ShapedGlyph, ShapedRun, ShapingEngine},
};

const TEXT_SHADER_HANDLE: Handle<Shader> =
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

        app.init_resource::<TextRebuildDiagnostics>()
            .init_resource::<TextInstanceAggregate>()
            .insert_resource(TypefaceFontBytes(self.font_bytes.clone()))
            .add_systems(Startup, init_typeface_state)
            .add_systems(Update, rebuild_changed_labels)
            .add_systems(
                Update,
                aggregate_label_instances.after(rebuild_changed_labels),
            )
            .add_plugins(ExtractComponentPlugin::<TextDrawExtract>::default());
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
struct TextDrawExtract {
    instances: Vec<GlyphInstanceGpu>,
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

fn init_typeface_state(mut commands: Commands, bytes: Res<TypefaceFontBytes>) {
    let font = load_font(&bytes.0).expect("typeface font must parse");
    let shaper = ShapingEngine::new_with_font(bytes.0.clone()).expect("typeface shaper must init");
    commands.insert_resource(TypefaceFont(font));
    commands.insert_resource(TypefaceShaper(shaper));
    commands.insert_resource(TypefaceAtlas::new_cpu(512));
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

fn aggregate_label_instances(
    q: Query<&TextGlyphInstances>,
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
