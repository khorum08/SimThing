use std::collections::HashMap;

use guillotiere::{size2, AtlasAllocator};
use msdf_font::ttf_parser::{Face, GlyphId};
use msdf_font::{Glyph, GlyphBitmapData, GlyphBuilder};

use crate::{
    atlas::{quantize_px, AtlasDirtyRect, AtlasTile},
    bevy::GlyphInstanceGpu,
    font::ProbeFont,
    icons::IconVector,
};

/// Instance render mode encoded in `GlyphInstanceGpu.sdf_params.x`.
pub const DISTANCE_FIELD_RENDER_RASTER: f32 = 0.0;
pub const DISTANCE_FIELD_RENDER_SDF: f32 = 1.0;
pub const DISTANCE_FIELD_RENDER_MSDF: f32 = 2.0;

const DEFAULT_PX_RANGE: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistanceFieldKind {
    Sdf,
    Msdf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DistanceFieldKey {
    pub source_id: u64,
    pub glyph_or_icon_id: u32,
    pub px_bucket: u16,
    pub kind: DistanceFieldKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistanceFieldTile {
    pub atlas_tile: AtlasTile,
    pub px_range: f32,
    pub kind: DistanceFieldKind,
}

#[derive(Debug, thiserror::Error)]
pub enum DistanceFieldError {
    #[error("font parse: {0}")]
    FontParse(String),

    #[error("glyph {0} has no outline for distance-field generation")]
    MissingOutline(u32),

    #[error("atlas full")]
    AtlasFull,

    #[error("icon MSDF deferred to LR6A: {0}")]
    IconDeferred(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DistanceFieldDiagnostics {
    pub glyph_msdf_generate_count: u64,
    pub glyph_sdf_generate_count: u64,
    pub icon_msdf_generate_count: u64,
    pub msdf_cache_hit_count: u64,
    pub msdf_cache_miss_count: u64,
    pub shader_smoke_draw_count: u64,
}

#[derive(Debug, Clone, PartialEq)]
struct GeneratedDistanceField {
    pixels: Vec<u8>,
    w: u32,
    h: u32,
    left: i32,
    top: i32,
    px_range: f32,
    kind: DistanceFieldKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirtyRect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl From<DirtyRect> for AtlasDirtyRect {
    fn from(value: DirtyRect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            w: value.w,
            h: value.h,
        }
    }
}

/// CPU-side distance-field atlas: import-time generation, guillotiere packing, cache.
pub struct DistanceFieldAtlasCore {
    size: u32,
    cpu_pixels: Vec<u8>,
    allocator: AtlasAllocator,
    cache: HashMap<DistanceFieldKey, DistanceFieldTile>,
    dirty_regions: Vec<DirtyRect>,
    diagnostics: DistanceFieldDiagnostics,
}

impl DistanceFieldAtlasCore {
    pub fn new(atlas_size: u32) -> Self {
        Self {
            size: atlas_size,
            cpu_pixels: vec![0u8; (atlas_size * atlas_size * 4) as usize],
            allocator: AtlasAllocator::new(size2(atlas_size as i32, atlas_size as i32)),
            cache: HashMap::new(),
            dirty_regions: Vec::new(),
            diagnostics: DistanceFieldDiagnostics::default(),
        }
    }

    pub fn atlas_size(&self) -> u32 {
        self.size
    }

    pub fn diagnostics(&self) -> DistanceFieldDiagnostics {
        self.diagnostics
    }

    pub fn staging_pixels(&self) -> &[u8] {
        &self.cpu_pixels
    }

    pub fn dirty_regions(&self) -> impl Iterator<Item = AtlasDirtyRect> + '_ {
        self.dirty_regions.iter().copied().map(Into::into)
    }

    pub fn dirty_region_byte_count(&self) -> u64 {
        self.dirty_regions
            .iter()
            .map(|rect| rect.w as u64 * rect.h as u64 * 4)
            .sum()
    }

    pub fn clear_dirty_regions(&mut self) {
        self.dirty_regions.clear();
    }

    pub fn font_source_id(font: &ProbeFont) -> u64 {
        stable_hash_bytes(font.bytes())
    }

    pub fn get_or_generate_glyph_msdf(
        &mut self,
        font: &ProbeFont,
        glyph_id: u32,
        px: f32,
    ) -> Result<DistanceFieldTile, DistanceFieldError> {
        self.get_or_generate_glyph(font, glyph_id, px, DistanceFieldKind::Msdf)
    }

    pub fn get_or_generate_glyph_sdf(
        &mut self,
        font: &ProbeFont,
        glyph_id: u32,
        px: f32,
    ) -> Result<DistanceFieldTile, DistanceFieldError> {
        self.get_or_generate_glyph(font, glyph_id, px, DistanceFieldKind::Sdf)
    }

    fn get_or_generate_glyph(
        &mut self,
        font: &ProbeFont,
        glyph_id: u32,
        px: f32,
        kind: DistanceFieldKind,
    ) -> Result<DistanceFieldTile, DistanceFieldError> {
        let px_bucket = quantize_px(px);
        let key = DistanceFieldKey {
            source_id: Self::font_source_id(font),
            glyph_or_icon_id: glyph_id,
            px_bucket,
            kind,
        };
        if let Some(tile) = self.cache.get(&key).copied() {
            self.diagnostics.msdf_cache_hit_count += 1;
            return Ok(tile);
        }
        self.diagnostics.msdf_cache_miss_count += 1;

        let generated = generate_glyph_distance_field(font, glyph_id, px, kind)?;
        match kind {
            DistanceFieldKind::Msdf => self.diagnostics.glyph_msdf_generate_count += 1,
            DistanceFieldKind::Sdf => self.diagnostics.glyph_sdf_generate_count += 1,
        }

        let atlas_tile = self
            .insert_generated(&generated)
            .ok_or(DistanceFieldError::AtlasFull)?;
        let tile = DistanceFieldTile {
            atlas_tile,
            px_range: generated.px_range,
            kind,
        };
        self.cache.insert(key, tile);
        Ok(tile)
    }

    pub fn get_or_generate_icon_msdf(
        &mut self,
        _icon: &IconVector,
        _codepoint: u32,
        _px: f32,
    ) -> Result<DistanceFieldTile, DistanceFieldError> {
        Err(DistanceFieldError::IconDeferred(
            "SVG icon vector MSDF generation deferred; LR4 raster icon path preserved",
        ))
    }

    fn insert_generated(&mut self, generated: &GeneratedDistanceField) -> Option<AtlasTile> {
        let allocation = self
            .allocator
            .allocate(size2(generated.w as i32, generated.h as i32))?;
        let tile = AtlasTile {
            x: allocation.rectangle.min.x as u32,
            y: allocation.rectangle.min.y as u32,
            w: generated.w,
            h: generated.h,
            left: generated.left,
            top: generated.top,
        };
        blit_rgba_to_atlas(
            &mut self.cpu_pixels,
            self.size,
            tile.x,
            tile.y,
            generated.w,
            generated.h,
            &generated.pixels,
        );
        self.dirty_regions.push(DirtyRect {
            x: tile.x,
            y: tile.y,
            w: tile.w,
            h: tile.h,
        });
        Some(tile)
    }
}

fn generate_glyph_distance_field(
    font: &ProbeFont,
    glyph_id: u32,
    px: f32,
    kind: DistanceFieldKind,
) -> Result<GeneratedDistanceField, DistanceFieldError> {
    let face = Face::parse(font.bytes(), 0)
        .map_err(|err| DistanceFieldError::FontParse(err.to_string()))?;
    let mut glyph =
        build_glyph_from_id(&face, GlyphId(glyph_id as u16), px, DEFAULT_PX_RANGE as u32)
            .ok_or(DistanceFieldError::MissingOutline(glyph_id))?;

    let (pixels, w, h) = match kind {
        DistanceFieldKind::Sdf => {
            let bitmap: GlyphBitmapData<u8, 1> = glyph.sdf();
            (
                sdf_l8_to_rgba(bitmap.bytes(), bitmap.width, bitmap.height),
                bitmap.width as u32,
                bitmap.height as u32,
            )
        }
        DistanceFieldKind::Msdf => {
            let bitmap: GlyphBitmapData<u8, 3> = glyph.msdf(3.0, true);
            (
                msdf_rgb_to_rgba(bitmap.bytes(), bitmap.width, bitmap.height),
                bitmap.width as u32,
                bitmap.height as u32,
            )
        }
    };

    let plane = glyph.data.plane_bounds;
    Ok(GeneratedDistanceField {
        pixels,
        w,
        h,
        left: plane.min[0].round() as i32,
        top: plane.min[1].round() as i32,
        px_range: DEFAULT_PX_RANGE,
        kind,
    })
}

fn build_glyph_from_id(
    face: &Face<'_>,
    glyph_id: GlyphId,
    px: f32,
    px_range: u32,
) -> Option<Glyph> {
    let ch = char_for_glyph_id(face, glyph_id)?;
    GlyphBuilder::new(face)
        .px_size(px.round() as u32)
        .px_range(px_range)
        .build(ch)
}

fn char_for_glyph_id(face: &Face<'_>, glyph_id: GlyphId) -> Option<char> {
    for cp in ' '..='~' {
        if face.glyph_index(cp) == Some(glyph_id) {
            return Some(cp);
        }
    }
    for cp in 'À'..='ÿ' {
        if face.glyph_index(cp) == Some(glyph_id) {
            return Some(cp);
        }
    }
    None
}

fn sdf_l8_to_rgba(bytes: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; width * height * 4];
    for (idx, alpha) in bytes.iter().enumerate() {
        let base = idx * 4;
        out[base] = 255;
        out[base + 1] = 255;
        out[base + 2] = 255;
        out[base + 3] = *alpha;
    }
    out
}

fn msdf_rgb_to_rgba(bytes: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let src = (y * width + x) * 3;
            let dst = (y * width + x) * 4;
            out[dst] = bytes[src];
            out[dst + 1] = bytes[src + 1];
            out[dst + 2] = bytes[src + 2];
            out[dst + 3] = 255;
        }
    }
    out
}

fn blit_rgba_to_atlas(
    atlas: &mut [u8],
    atlas_size: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    src: &[u8],
) {
    for row in 0..h {
        let src_row = row * w * 4;
        let dst_row = ((y + row) * atlas_size + x) * 4;
        let len = (w * 4) as usize;
        atlas[dst_row as usize..dst_row as usize + len]
            .copy_from_slice(&src[src_row as usize..src_row as usize + len]);
    }
}

fn stable_hash_bytes(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Build a GPU instance for a cached distance-field tile.
pub fn build_distance_field_instance(
    x: f32,
    y: f32,
    tile: &DistanceFieldTile,
    atlas_size: u32,
    color: [f32; 4],
) -> GlyphInstanceGpu {
    let inv = 1.0 / atlas_size as f32;
    let mode = match tile.kind {
        DistanceFieldKind::Sdf => DISTANCE_FIELD_RENDER_SDF,
        DistanceFieldKind::Msdf => DISTANCE_FIELD_RENDER_MSDF,
    };
    GlyphInstanceGpu {
        pos_size: [
            x + tile.atlas_tile.left as f32,
            y + tile.atlas_tile.top as f32,
            tile.atlas_tile.w as f32,
            tile.atlas_tile.h as f32,
        ],
        uv_rect: [
            tile.atlas_tile.x as f32 * inv,
            tile.atlas_tile.y as f32 * inv,
            (tile.atlas_tile.x + tile.atlas_tile.w) as f32 * inv,
            (tile.atlas_tile.y + tile.atlas_tile.h) as f32 * inv,
        ],
        color,
        sdf_params: [mode, tile.px_range, atlas_size as f32, 0.0],
    }
}

#[cfg(test)]
pub fn generate_test_glyph_msdf(
    font: &ProbeFont,
    glyph_id: u32,
    px: f32,
) -> Result<GeneratedDistanceField, DistanceFieldError> {
    generate_glyph_distance_field(font, glyph_id, px, DistanceFieldKind::Msdf)
}
