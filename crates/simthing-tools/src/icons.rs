use std::collections::HashMap;

use kurbo::BezPath;
use thiserror::Error;
use tiny_skia::{FillRule as SkiaFillRule, Pixmap, Transform};
use usvg::{
    roxmltree::{Document, Node as XmlNode},
    tiny_skia_path::{Path as SkiaPath, PathBuilder, PathSegment},
};

use crate::{
    atlas::{quantize_px, AtlasTile, GlyphAtlasCore},
    bevy::GlyphInstanceGpu,
    font::ProbeFont,
    shaping::{ShapedGlyph, ShapingEngine},
    style::{role_slot_for_icon_layer, style_params_for_slot, TextStyleSlot},
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
pub enum IconPathCommand {
    MoveTo {
        x: f32,
        y: f32,
    },
    LineTo {
        x: f32,
        y: f32,
    },
    QuadTo {
        x1: f32,
        y1: f32,
        x: f32,
        y: f32,
    },
    CubicTo {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x: f32,
        y: f32,
    },
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconFillRule {
    NonZero,
    EvenOdd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconVectorPath {
    pub commands: Vec<IconPathCommand>,
    pub fill_rule: IconFillRule,
    pub bounds: [f32; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconVectorLayer {
    pub role: IconLayerRole,
    pub paths: Vec<IconVectorPath>,
    pub bounds: [f32; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconVector {
    pub layers: Vec<IconVectorLayer>,
    pub view_box: [f32; 4],
}

/// Per-role style-slot reference for LR6B (geometry + role-layer raster).
#[derive(Debug, Clone, PartialEq)]
pub struct IconStyleLayerRef {
    pub role: IconLayerRole,
    pub geometry_hash: u64,
    pub raster_tile: AtlasTile,
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
    #[error("unsupported icon path geometry: {0}")]
    UnsupportedGeometry(String),
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
    style_layers: Vec<IconStyleLayerRef>,
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

        let style_layers = build_role_layer_rasters(&vector, px, atlas)?;

        self.entries.insert(
            key,
            IconEntry {
                tile,
                vector,
                style_layers,
            },
        );
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

    pub fn style_layers_for(&self, codepoint: u32, px: f32) -> Option<Vec<IconStyleLayerRef>> {
        let key = IconCacheKey {
            codepoint: IconCodepoint(codepoint),
            px_bucket: quantize_px(px),
        };
        self.entries
            .get(&key)
            .map(|entry| entry.style_layers.clone())
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

    /// Number of cached icon entries (codepoint + px bucket).
    pub fn cache_entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Build per-role icon layer instances with distinct style slots (LR6B).
    pub fn build_layered_icon_style_instances(
        &self,
        codepoint: u32,
        px: f32,
        x: f32,
        y: f32,
        color: [f32; 4],
        role_style_slots: &[(IconLayerRole, TextStyleSlot)],
        atlas_size: u32,
    ) -> Result<Vec<GlyphInstanceGpu>, IconError> {
        let layers = self
            .style_layers_for(codepoint, px)
            .ok_or(IconError::EmptyVector)?;
        let inv = 1.0 / atlas_size as f32;
        let mut instances = Vec::with_capacity(role_style_slots.len());
        for (role, style_slot) in role_style_slots {
            let layer = layers
                .iter()
                .find(|layer| layer.role == *role)
                .ok_or(IconError::EmptyVector)?;
            let tile = layer.raster_tile;
            instances.push(GlyphInstanceGpu {
                pos_size: [x, y, tile.w as f32, tile.h as f32],
                uv_rect: [
                    tile.x as f32 * inv,
                    tile.y as f32 * inv,
                    (tile.x + tile.w) as f32 * inv,
                    (tile.y + tile.h) as f32 * inv,
                ],
                color,
                sdf_params: [0.0; 4],
                style_params: style_params_for_slot(*style_slot, role_slot_for_icon_layer(*role)),
                deform_params: [0.0; 4],
            });
        }
        Ok(instances)
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

    pub fn geometry_hash(&self) -> u64 {
        stable_hash_icon_vector(self)
    }

    pub fn layer_roles(&self) -> impl Iterator<Item = IconLayerRole> + '_ {
        self.layers.iter().map(|layer| layer.role)
    }

    pub fn paths_for_role(&self, role: IconLayerRole) -> Vec<&IconVectorPath> {
        self.layers
            .iter()
            .filter(|layer| layer.role == role)
            .flat_map(|layer| layer.paths.iter())
            .collect()
    }

    pub fn has_renderable_geometry(&self) -> bool {
        self.layers
            .iter()
            .any(|layer| layer.paths.iter().any(|path| !path.commands.is_empty()))
    }

    /// Build a kurbo bezpath in pixel space (Y-up) for MSDF generation.
    pub fn to_msdf_bezpath(&self, px: f32) -> Option<BezPath> {
        if !self.has_renderable_geometry() {
            return None;
        }
        let scale = icon_scale(self.view_box, px);
        let view_h = self.view_box[3];
        let mut path = BezPath::new();
        for layer in &self.layers {
            for icon_path in &layer.paths {
                append_path_to_bezpath(&mut path, icon_path, scale, view_h);
            }
        }
        if path.elements().is_empty() {
            None
        } else {
            Some(path)
        }
    }
}

impl IconVectorPath {
    pub fn geometry_hash(&self) -> u64 {
        stable_hash_path(self)
    }

    pub fn debug_signature(&self) -> String {
        let mut signature = String::new();
        for command in &self.commands {
            match command {
                IconPathCommand::MoveTo { x, y } => {
                    signature.push_str(&format!("M{x:.3},{y:.3};"));
                }
                IconPathCommand::LineTo { x, y } => {
                    signature.push_str(&format!("L{x:.3},{y:.3};"));
                }
                IconPathCommand::QuadTo { x1, y1, x, y } => {
                    signature.push_str(&format!("Q{x1:.3},{y1:.3},{x:.3},{y:.3};"));
                }
                IconPathCommand::CubicTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => {
                    signature.push_str(&format!("C{x1:.3},{y1:.3},{x2:.3},{y2:.3},{x:.3},{y:.3};"));
                }
                IconPathCommand::Close => signature.push('Z'),
            }
        }
        signature
    }
}

struct RasterizedIcon {
    pixels: Vec<u8>,
    w: u32,
    h: u32,
}

fn build_role_layer_rasters(
    vector: &IconVector,
    px: f32,
    atlas: &mut GlyphAtlasCore,
) -> Result<Vec<IconStyleLayerRef>, IconError> {
    let mut style_layers = Vec::with_capacity(vector.layers.len());
    for layer in &vector.layers {
        let raster = rasterize_layer_paths(&layer.paths, vector.view_box, px)?;
        let raster_tile = atlas
            .insert_rgba8_tile(&raster.pixels, raster.w, raster.h, 0, 0)
            .ok_or(IconError::AtlasFull)?;
        let geometry_hash = stable_hash_layer_paths(&layer.paths);
        style_layers.push(IconStyleLayerRef {
            role: layer.role,
            geometry_hash,
            raster_tile,
        });
    }
    Ok(style_layers)
}

fn rasterize_layer_paths(
    paths: &[IconVectorPath],
    view_box: [f32; 4],
    px: f32,
) -> Result<RasterizedIcon, IconError> {
    validate_px(px)?;
    let side = px.round().max(1.0) as u32;
    let mut pixmap = Pixmap::new(side, side).ok_or(IconError::RasterizeFailed)?;
    let scale = icon_scale(view_box, px);
    let transform = Transform::from_scale(scale, scale);
    let paint = tiny_skia::Paint {
        anti_alias: true,
        ..Default::default()
    };

    for path in paths {
        let sk_path = icon_path_to_skia(path)?;
        let fill_rule = match path.fill_rule {
            IconFillRule::NonZero => SkiaFillRule::Winding,
            IconFillRule::EvenOdd => SkiaFillRule::EvenOdd,
        };
        pixmap.fill_path(&sk_path, &paint, fill_rule, transform, None);
    }

    Ok(RasterizedIcon {
        pixels: pixmap.take_demultiplied(),
        w: side,
        h: side,
    })
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
    resvg::render(&tree, Transform::from_scale(scale, scale), &mut pixmap_mut);

    Ok(RasterizedIcon {
        pixels: pixmap.take_demultiplied(),
        w: side,
        h: side,
    })
}

fn icon_scale(view_box: [f32; 4], px: f32) -> f32 {
    let longest = view_box[2].max(view_box[3]).max(1.0);
    px / longest
}

fn append_path_to_bezpath(bezpath: &mut BezPath, path: &IconVectorPath, scale: f32, view_h: f32) {
    for command in &path.commands {
        match command {
            IconPathCommand::MoveTo { x, y } => {
                let (x, y) = msdf_point(*x, *y, scale, view_h);
                bezpath.move_to((x as f64, y as f64));
            }
            IconPathCommand::LineTo { x, y } => {
                let (x, y) = msdf_point(*x, *y, scale, view_h);
                bezpath.line_to((x as f64, y as f64));
            }
            IconPathCommand::QuadTo { x1, y1, x, y } => {
                let (x1, y1) = msdf_point(*x1, *y1, scale, view_h);
                let (x, y) = msdf_point(*x, *y, scale, view_h);
                bezpath.quad_to((x1 as f64, y1 as f64), (x as f64, y as f64));
            }
            IconPathCommand::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                let (x1, y1) = msdf_point(*x1, *y1, scale, view_h);
                let (x2, y2) = msdf_point(*x2, *y2, scale, view_h);
                let (x, y) = msdf_point(*x, *y, scale, view_h);
                bezpath.curve_to(
                    (x1 as f64, y1 as f64),
                    (x2 as f64, y2 as f64),
                    (x as f64, y as f64),
                );
            }
            IconPathCommand::Close => bezpath.close_path(),
        }
    }
}

fn msdf_point(x: f32, y: f32, scale: f32, view_h: f32) -> (f32, f32) {
    (x * scale, (view_h - y) * scale)
}

fn icon_path_to_skia(path: &IconVectorPath) -> Result<SkiaPath, IconError> {
    let mut builder = PathBuilder::default();
    for command in &path.commands {
        match command {
            IconPathCommand::MoveTo { x, y } => builder.move_to(*x, *y),
            IconPathCommand::LineTo { x, y } => builder.line_to(*x, *y),
            IconPathCommand::QuadTo { x1, y1, x, y } => builder.quad_to(*x1, *y1, *x, *y),
            IconPathCommand::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => builder.cubic_to(*x1, *y1, *x2, *y2, *x, *y),
            IconPathCommand::Close => builder.close(),
        }
    }
    builder
        .finish()
        .ok_or_else(|| IconError::UnsupportedGeometry("empty path".into()))
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
                let vector_path = match extract_vector_path(path) {
                    Ok(path) => path,
                    Err(_) => continue,
                };
                let bounds = layer_bounds(std::slice::from_ref(&vector_path));
                layers.push(IconVectorLayer {
                    role,
                    paths: vec![vector_path],
                    bounds,
                });
            }
            usvg::Node::Image(_) | usvg::Node::Text(_) => {}
        }
    }
}

fn transform_point(
    ts: &usvg::tiny_skia_path::Transform,
    mut point: usvg::tiny_skia_path::Point,
) -> usvg::tiny_skia_path::Point {
    ts.map_point(&mut point);
    point
}

fn extract_vector_path(path: &usvg::Path) -> Result<IconVectorPath, IconError> {
    let ts = path.abs_transform();
    let fill_rule = match path.fill().map(|fill| fill.rule()) {
        Some(usvg::FillRule::NonZero) | None => IconFillRule::NonZero,
        Some(usvg::FillRule::EvenOdd) => IconFillRule::EvenOdd,
    };
    let mut commands = Vec::new();
    for segment in path.data().segments() {
        match segment {
            PathSegment::MoveTo(p) => {
                let p = transform_point(&ts, p);
                commands.push(IconPathCommand::MoveTo { x: p.x, y: p.y });
            }
            PathSegment::LineTo(p) => {
                let p = transform_point(&ts, p);
                commands.push(IconPathCommand::LineTo { x: p.x, y: p.y });
            }
            PathSegment::QuadTo(p0, p1) => {
                let p0 = transform_point(&ts, p0);
                let p1 = transform_point(&ts, p1);
                commands.push(IconPathCommand::QuadTo {
                    x1: p0.x,
                    y1: p0.y,
                    x: p1.x,
                    y: p1.y,
                });
            }
            PathSegment::CubicTo(p0, p1, p2) => {
                let p0 = transform_point(&ts, p0);
                let p1 = transform_point(&ts, p1);
                let p2 = transform_point(&ts, p2);
                commands.push(IconPathCommand::CubicTo {
                    x1: p0.x,
                    y1: p0.y,
                    x2: p1.x,
                    y2: p1.y,
                    x: p2.x,
                    y: p2.y,
                });
            }
            PathSegment::Close => commands.push(IconPathCommand::Close),
        }
    }
    if commands.is_empty() {
        return Err(IconError::UnsupportedGeometry(
            "path has no segments".into(),
        ));
    }
    let bounds = bounds_from_commands(&commands);
    Ok(IconVectorPath {
        commands,
        fill_rule,
        bounds,
    })
}

fn bounds_from_commands(commands: &[IconPathCommand]) -> [f32; 4] {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for command in commands {
        for (x, y) in command_points(command) {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }
    if !min_x.is_finite() {
        return [0.0, 0.0, 0.0, 0.0];
    }
    [min_x, min_y, max_x - min_x, max_y - min_y]
}

fn layer_bounds(paths: &[IconVectorPath]) -> [f32; 4] {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for path in paths {
        let [x, y, w, h] = path.bounds;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + w);
        max_y = max_y.max(y + h);
    }
    if !min_x.is_finite() {
        return [0.0, 0.0, 0.0, 0.0];
    }
    [min_x, min_y, max_x - min_x, max_y - min_y]
}

fn command_points(command: &IconPathCommand) -> Vec<(f32, f32)> {
    match command {
        IconPathCommand::MoveTo { x, y } | IconPathCommand::LineTo { x, y } => {
            vec![(*x, *y)]
        }
        IconPathCommand::QuadTo { x1, y1, x, y } => vec![(*x1, *y1), (*x, *y)],
        IconPathCommand::CubicTo {
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        } => vec![(*x1, *y1), (*x2, *y2), (*x, *y)],
        IconPathCommand::Close => Vec::new(),
    }
}

fn stable_hash_icon_vector(icon: &IconVector) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for component in icon.view_box {
        hash = fnv_update_f32(hash, component);
    }
    for layer in &icon.layers {
        hash = fnv_update_u8(hash, role_discriminant(layer.role));
        hash = stable_hash_layer_paths_inner(hash, &layer.paths);
    }
    hash
}

fn stable_hash_path(path: &IconVectorPath) -> u64 {
    stable_hash_layer_paths_inner(0xcbf29ce484222325_u64, std::slice::from_ref(path))
}

fn stable_hash_layer_paths(paths: &[IconVectorPath]) -> u64 {
    stable_hash_layer_paths_inner(0xcbf29ce484222325_u64, paths)
}

fn stable_hash_layer_paths_inner(mut hash: u64, paths: &[IconVectorPath]) -> u64 {
    for path in paths {
        hash = fnv_update_u8(hash, path.fill_rule as u8);
        for command in &path.commands {
            hash = fnv_update_u8(hash, command_discriminant(command));
            for (x, y) in command_points(command) {
                hash = fnv_update_f32(hash, x);
                hash = fnv_update_f32(hash, y);
            }
        }
    }
    hash
}

fn command_discriminant(command: &IconPathCommand) -> u8 {
    match command {
        IconPathCommand::MoveTo { .. } => 0,
        IconPathCommand::LineTo { .. } => 1,
        IconPathCommand::QuadTo { .. } => 2,
        IconPathCommand::CubicTo { .. } => 3,
        IconPathCommand::Close => 4,
    }
}

fn fnv_update_f32(hash: u64, value: f32) -> u64 {
    fnv_update_u64(hash, u64::from(value.to_bits()))
}

fn fnv_update_u64(mut hash: u64, value: u64) -> u64 {
    for shift in (0..64).step_by(8) {
        hash = fnv_update_u8(hash, ((value >> shift) & 0xff) as u8);
    }
    hash
}

fn role_discriminant(role: IconLayerRole) -> u8 {
    match role {
        IconLayerRole::Primary => 0,
        IconLayerRole::Secondary => 1,
        IconLayerRole::Accent => 2,
        IconLayerRole::Outline => 3,
        IconLayerRole::Background => 4,
        IconLayerRole::Mask => 5,
    }
}

fn fnv_update_u8(mut hash: u64, byte: u8) -> u64 {
    hash ^= u64::from(byte);
    hash.wrapping_mul(0x100000001b3)
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
        sdf_params: [0.0; 4],
        style_params: [0.0; 4],
        deform_params: [0.0; 4],
    }
}
