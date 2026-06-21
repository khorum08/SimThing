use std::collections::HashMap;

use thiserror::Error;
use tiny_skia::Pixmap;
use usvg::{
    roxmltree::{Document, Node as XmlNode},
    tiny_skia_path::PathSegment,
};

use crate::{
    atlas::{quantize_px, AtlasTile, GlyphAtlasCore},
    bevy::GlyphInstanceGpu,
    font::ProbeFont,
    shaping::{ShapedGlyph, ShapingEngine},
};

pub const ICON_PUA_START: u32 = 0xF0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IconCodepoint(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconLayerRole {
    Primary,
    Secondary,
    Accent,
    Outline,
    Background,
    Mask,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconVectorLayer {
    pub role: IconLayerRole,
    pub path_signature: String,
    pub bounds: [f32; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconVector {
    pub layers: Vec<IconVectorLayer>,
    pub view_box: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IconRegistration {
    pub codepoint: IconCodepoint,
    pub tile: AtlasTile,
}

#[derive(Debug, Error)]
pub enum IconError {
    #[error("icon codepoint {0:#x} is outside Supplementary PUA-A")]
    CodepointOutOfRange(u32),
    #[error("dynamic or external SVG feature rejected: {0}")]
    StaticOnly(String),
    #[error("unknown icon layer role `{0}`")]
    UnknownRole(String),
    #[error("SVG parse: {0}")]
    Parse(String),
    #[error("SVG has no renderable path layers")]
    EmptyVector,
    #[error("invalid icon pixel size {0}")]
    InvalidPixelSize(f32),
    #[error("SVG rasterization failed")]
    RasterizeFailed,
    #[error("icon atlas is full")]
    AtlasFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IconCacheKey {
    codepoint: IconCodepoint,
    px_bucket: u16,
}

#[derive(Debug, Clone)]
struct IconEntry {
    tile: AtlasTile,
    vector: IconVector,
}

#[derive(Default)]
pub struct IconSet {
    entries: HashMap<IconCacheKey, IconEntry>,
    latest_by_codepoint: HashMap<IconCodepoint, IconCacheKey>,
}

impl IconSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_svg(
        &mut self,
        codepoint: u32,
        svg: &str,
        px: f32,
        atlas: &mut GlyphAtlasCore,
    ) -> Result<IconRegistration, IconError> {
        let codepoint = validate_codepoint(codepoint)?;
        validate_px(px)?;
        let key = IconCacheKey {
            codepoint,
            px_bucket: quantize_px(px),
        };
        if let Some(entry) = self.entries.get(&key) {
            return Ok(IconRegistration {
                codepoint,
                tile: entry.tile,
            });
        }

        let vector = IconVector::from_svg(svg)?;
        let raster = rasterize_svg_icon(svg, px)?;
        let tile = atlas
            .insert_rgba8_tile(&raster.pixels, raster.w, raster.h, 0, 0)
            .ok_or(IconError::AtlasFull)?;

        self.entries.insert(key, IconEntry { tile, vector });
        self.latest_by_codepoint.insert(codepoint, key);
        Ok(IconRegistration { codepoint, tile })
    }

    pub fn tile_for(&self, codepoint: u32) -> Option<AtlasTile> {
        let codepoint = IconCodepoint(codepoint);
        let key = self.latest_by_codepoint.get(&codepoint)?;
        self.entries.get(key).map(|entry| entry.tile)
    }

    pub fn vector_for(&self, codepoint: u32) -> Option<&IconVector> {
        let codepoint = IconCodepoint(codepoint);
        let key = self.latest_by_codepoint.get(&codepoint)?;
        self.entries.get(key).map(|entry| &entry.vector)
    }

    pub fn build_mixed_instances(
        &self,
        font: &ProbeFont,
        shaper: &mut ShapingEngine,
        atlas: &mut GlyphAtlasCore,
        text: &str,
        px: f32,
        color: [f32; 4],
    ) -> Result<Vec<GlyphInstanceGpu>, IconError> {
        validate_px(px)?;
        let shaped = shaper.shape(text, px);
        let mut instances = Vec::new();
        for glyph in &shaped.glyphs {
            if let Some(codepoint) = codepoint_at_cluster(text, glyph.cluster) {
                if let Some(tile) = self.tile_for_px(codepoint, px) {
                    instances.push(build_icon_instance(glyph, tile, color, atlas.atlas_size()));
                    continue;
                }
            }

            if let Some(tile) = atlas.get_or_rasterize(font, glyph.glyph_id, px) {
                instances.push(build_text_instance(glyph, tile, color, atlas.atlas_size()));
            }
        }
        Ok(instances)
    }

    fn tile_for_px(&self, codepoint: u32, px: f32) -> Option<AtlasTile> {
        let key = IconCacheKey {
            codepoint: IconCodepoint(codepoint),
            px_bucket: quantize_px(px),
        };
        self.entries.get(&key).map(|entry| entry.tile)
    }
}

impl IconVector {
    pub fn from_svg(svg: &str) -> Result<Self, IconError> {
        reject_dynamic_or_external_svg(svg)?;
        let doc = Document::parse(svg).map_err(|err| IconError::Parse(err.to_string()))?;
        let raw_roles = collect_raw_roles(&doc)?;
        let tree = usvg::Tree::from_str(svg, &static_svg_options())
            .map_err(|err| IconError::Parse(err.to_string()))?;
        let size = tree.size();
        let view_box = [0.0, 0.0, size.width(), size.height()];
        let mut layers = Vec::new();
        collect_layers(tree.root(), &raw_roles, &mut layers);
        if layers.is_empty() {
            return Err(IconError::EmptyVector);
        }

        Ok(Self { layers, view_box })
    }
}

struct RasterizedIcon {
    pixels: Vec<u8>,
    w: u32,
    h: u32,
}

fn rasterize_svg_icon(svg: &str, px: f32) -> Result<RasterizedIcon, IconError> {
    reject_dynamic_or_external_svg(svg)?;
    validate_px(px)?;
    let tree = usvg::Tree::from_str(svg, &static_svg_options())
        .map_err(|err| IconError::Parse(err.to_string()))?;
    let size = tree.size();
    let longest_edge = size.width().max(size.height());
    if longest_edge <= 0.0 || !longest_edge.is_finite() {
        return Err(IconError::RasterizeFailed);
    }

    let side = px.round().max(1.0) as u32;
    let scale = side as f32 / longest_edge;
    let mut pixmap = Pixmap::new(side, side).ok_or(IconError::RasterizeFailed)?;
    let mut pixmap_mut = pixmap.as_mut();
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap_mut,
    );

    Ok(RasterizedIcon {
        pixels: pixmap.take_demultiplied(),
        w: side,
        h: side,
    })
}

fn validate_codepoint(codepoint: u32) -> Result<IconCodepoint, IconError> {
    if (ICON_PUA_START..=0xFFFFD).contains(&codepoint) {
        Ok(IconCodepoint(codepoint))
    } else {
        Err(IconError::CodepointOutOfRange(codepoint))
    }
}

fn validate_px(px: f32) -> Result<(), IconError> {
    if px.is_finite() && px > 0.0 {
        Ok(())
    } else {
        Err(IconError::InvalidPixelSize(px))
    }
}

fn static_svg_options() -> usvg::Options<'static> {
    usvg::Options {
        image_href_resolver: usvg::ImageHrefResolver {
            resolve_data: Box::new(|_, _, _| None),
            resolve_string: Box::new(|_, _| None),
        },
        ..Default::default()
    }
}

fn reject_dynamic_or_external_svg(svg: &str) -> Result<(), IconError> {
    let doc = Document::parse(svg).map_err(|err| IconError::Parse(err.to_string()))?;
    for node in doc.descendants().filter(|node| node.is_element()) {
        let tag = node.tag_name().name();
        let tag_lower = tag.to_ascii_lowercase();
        if matches!(
            tag_lower.as_str(),
            "script"
                | "image"
                | "foreignobject"
                | "a"
                | "audio"
                | "video"
                | "iframe"
                | "object"
                | "embed"
        ) || tag_lower.starts_with("animate")
            || tag_lower == "set"
        {
            return Err(IconError::StaticOnly(tag.to_string()));
        }

        for attr in node.attributes() {
            let name = attr.name().to_ascii_lowercase();
            let value = attr.value().trim().to_ascii_lowercase();
            if name.starts_with("on") {
                return Err(IconError::StaticOnly(attr.name().to_string()));
            }
            if name == "href" || name.ends_with(":href") || name == "src" {
                if value.starts_with("http://")
                    || value.starts_with("https://")
                    || value.starts_with("file:")
                    || value.starts_with("data:")
                    || !value.starts_with('#')
                {
                    return Err(IconError::StaticOnly(attr.name().to_string()));
                }
            }
            if value.contains("http://")
                || value.contains("https://")
                || value.contains("url(http")
                || value.contains("url(file:")
                || value.contains("url(data:")
            {
                return Err(IconError::StaticOnly(attr.name().to_string()));
            }
        }
    }
    Ok(())
}

fn collect_raw_roles(doc: &Document<'_>) -> Result<Vec<IconLayerRole>, IconError> {
    let mut roles = Vec::new();
    for node in doc.descendants().filter(|node| node.is_element()) {
        if !is_shape_node(node) {
            continue;
        }
        let role = node
            .attribute("data-simthing-role")
            .map(parse_role)
            .transpose()?
            .unwrap_or(IconLayerRole::Primary);
        roles.push(role);
    }
    Ok(roles)
}

fn is_shape_node(node: XmlNode<'_, '_>) -> bool {
    matches!(
        node.tag_name().name(),
        "path" | "rect" | "circle" | "ellipse" | "polygon" | "polyline" | "line"
    )
}

fn parse_role(value: &str) -> Result<IconLayerRole, IconError> {
    match value {
        "primary" => Ok(IconLayerRole::Primary),
        "secondary" => Ok(IconLayerRole::Secondary),
        "accent" => Ok(IconLayerRole::Accent),
        "outline" => Ok(IconLayerRole::Outline),
        "background" => Ok(IconLayerRole::Background),
        "mask" => Ok(IconLayerRole::Mask),
        other => Err(IconError::UnknownRole(other.to_string())),
    }
}

fn collect_layers(
    group: &usvg::Group,
    raw_roles: &[IconLayerRole],
    layers: &mut Vec<IconVectorLayer>,
) {
    for node in group.children() {
        match node {
            usvg::Node::Group(group) => collect_layers(group, raw_roles, layers),
            usvg::Node::Path(path) => {
                let role = raw_roles
                    .get(layers.len())
                    .copied()
                    .unwrap_or(IconLayerRole::Primary);
                let bounds = path.abs_bounding_box();
                layers.push(IconVectorLayer {
                    role,
                    path_signature: path_signature(path.data()),
                    bounds: [bounds.x(), bounds.y(), bounds.width(), bounds.height()],
                });
            }
            usvg::Node::Image(_) | usvg::Node::Text(_) => {}
        }
    }
}

fn path_signature(path: &usvg::tiny_skia_path::Path) -> String {
    let mut signature = String::new();
    for segment in path.segments() {
        match segment {
            PathSegment::MoveTo(p) => push_point(&mut signature, "M", &[p]),
            PathSegment::LineTo(p) => push_point(&mut signature, "L", &[p]),
            PathSegment::QuadTo(p0, p1) => push_point(&mut signature, "Q", &[p0, p1]),
            PathSegment::CubicTo(p0, p1, p2) => push_point(&mut signature, "C", &[p0, p1, p2]),
            PathSegment::Close => signature.push_str("Z;"),
        }
    }
    signature
}

fn push_point(signature: &mut String, op: &str, points: &[usvg::tiny_skia_path::Point]) {
    signature.push_str(op);
    for point in points {
        signature.push_str(&format!("{:.3},{:.3};", point.x, point.y));
    }
}

fn codepoint_at_cluster(text: &str, cluster: usize) -> Option<u32> {
    text.get(cluster..)?.chars().next().map(u32::from)
}

fn build_text_instance(
    glyph: &ShapedGlyph,
    tile: AtlasTile,
    color: [f32; 4],
    atlas_size: u32,
) -> GlyphInstanceGpu {
    build_instance(
        glyph.x + tile.left as f32,
        glyph.y + tile.top as f32,
        tile,
        color,
        atlas_size,
    )
}

fn build_icon_instance(
    glyph: &ShapedGlyph,
    tile: AtlasTile,
    color: [f32; 4],
    atlas_size: u32,
) -> GlyphInstanceGpu {
    build_instance(glyph.x, glyph.y, tile, color, atlas_size)
}

fn build_instance(
    x: f32,
    y: f32,
    tile: AtlasTile,
    color: [f32; 4],
    atlas_size: u32,
) -> GlyphInstanceGpu {
    let inv = 1.0 / atlas_size as f32;
    GlyphInstanceGpu {
        pos_size: [x, y, tile.w as f32, tile.h as f32],
        uv_rect: [
            tile.x as f32 * inv,
            tile.y as f32 * inv,
            (tile.x + tile.w) as f32 * inv,
            (tile.y + tile.h) as f32 * inv,
        ],
        color,
    }
}
