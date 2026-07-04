use std::collections::HashMap;

use guillotiere::{size2, AtlasAllocator};
use swash::scale::{image::Content, Render, ScaleContext, Source};
use swash::FontRef;
use wgpu::{
    Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, Queue, Texture, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
};

use crate::font::ProbeFont;

/// Transparent gutter around raster glyph tiles to prevent bilinear atlas bleed.
pub const RASTER_GLYPH_ATLAS_GUTTER_PX: u32 = 1;

/// Half-texel UV inset on inner glyph bounds (requires gutter for stable edge sampling).
pub const RASTER_GLYPH_ATLAS_UV_INSET: bool = true;

/// Workshop atlas texture format: RGBA8 with glyph coverage in the alpha channel.
pub const ATLAS_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtlasTile {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub left: i32,
    pub top: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphAtlasKey {
    pub glyph_id: u16,
    pub px_bucket: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphAtlasStats {
    pub rasterize_count: u64,
    pub cache_hit_count: u64,
    pub dirty_region_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RasterizedGlyph {
    pub pixels: Vec<u8>,
    pub w: u32,
    pub h: u32,
    pub left: i32,
    pub top: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasDirtyRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
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

/// CPU-side atlas: rasterization, guillotiere packing, cache, and dirty-region tracking.
pub struct GlyphAtlasCore {
    size: u32,
    cpu_pixels: Vec<u8>,
    allocator: AtlasAllocator,
    cache: HashMap<GlyphAtlasKey, AtlasTile>,
    dirty_regions: Vec<DirtyRect>,
    rasterize_count: u64,
    cache_hit_count: u64,
}

/// GPU-backed atlas wrapping a [`GlyphAtlasCore`] with wgpu texture upload.
pub struct GlyphAtlas {
    core: GlyphAtlasCore,
    texture: Texture,
    texture_view: TextureView,
}

/// Quantize a pixel size to a stable cache bucket (quarter-pixel resolution).
pub fn quantize_px(px: f32) -> u16 {
    let bucket = (px * 4.0).round();
    bucket.clamp(1.0, u16::MAX as f32) as u16
}

pub fn rasterize_glyph_cpu(font: &ProbeFont, glyph_id: u16, px: f32) -> Option<RasterizedGlyph> {
    let swash_font = FontRef::from_index(font.bytes(), 0)?;
    let mut context = ScaleContext::new();
    let mut scaler = context.builder(swash_font).size(px).hint(true).build();
    let image = Render::new(&[Source::Outline]).render(&mut scaler, glyph_id)?;
    if image.content != Content::Mask {
        return None;
    }

    let w = image.placement.width;
    let h = image.placement.height;
    if w == 0 || h == 0 {
        return None;
    }

    let mut pixels = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let alpha = image.data[(y * w + x) as usize];
            let idx = ((y * w + x) * 4) as usize;
            pixels[idx] = 255;
            pixels[idx + 1] = 255;
            pixels[idx + 2] = 255;
            pixels[idx + 3] = alpha;
        }
    }

    Some(RasterizedGlyph {
        pixels,
        w,
        h,
        left: image.placement.left,
        top: image.placement.top,
    })
}

impl GlyphAtlasCore {
    pub fn new(size: u32) -> Self {
        Self {
            size,
            cpu_pixels: vec![0u8; (size * size * 4) as usize],
            allocator: AtlasAllocator::new(size2(size as i32, size as i32)),
            cache: HashMap::new(),
            dirty_regions: Vec::new(),
            rasterize_count: 0,
            cache_hit_count: 0,
        }
    }

    pub fn get_or_rasterize(
        &mut self,
        font: &ProbeFont,
        glyph_id: u16,
        px: f32,
    ) -> Option<AtlasTile> {
        let key = GlyphAtlasKey {
            glyph_id,
            px_bucket: quantize_px(px),
        };
        if let Some(tile) = self.cache.get(&key).copied() {
            self.cache_hit_count += 1;
            return Some(tile);
        }

        let raster = rasterize_glyph_cpu(font, glyph_id, px)?;
        let tile =
            self.insert_rgba8_tile(&raster.pixels, raster.w, raster.h, raster.left, raster.top)?;
        self.cache.insert(key, tile);
        Some(tile)
    }

    pub fn insert_rgba8_tile(
        &mut self,
        pixels: &[u8],
        w: u32,
        h: u32,
        left: i32,
        top: i32,
    ) -> Option<AtlasTile> {
        if w == 0 || h == 0 || pixels.len() != (w * h * 4) as usize {
            return None;
        }

        let gutter = RASTER_GLYPH_ATLAS_GUTTER_PX;
        let alloc_w = w + 2 * gutter;
        let alloc_h = h + 2 * gutter;
        let allocation = self
            .allocator
            .allocate(size2(alloc_w as i32, alloc_h as i32))?;
        let rect = self.allocator[allocation.id];
        let alloc_x = rect.min.x as u32;
        let alloc_y = rect.min.y as u32;
        let content_x = alloc_x + gutter;
        let content_y = alloc_y + gutter;

        blit_rgba8_with_gutter(
            &mut self.cpu_pixels,
            self.size,
            alloc_x,
            alloc_y,
            content_x,
            content_y,
            w,
            h,
            gutter,
            pixels,
        );

        let tile = AtlasTile {
            x: content_x,
            y: content_y,
            w,
            h,
            left,
            top,
        };
        self.rasterize_count += 1;
        self.dirty_regions.push(DirtyRect {
            x: alloc_x,
            y: alloc_y,
            w: alloc_w,
            h: alloc_h,
        });
        Some(tile)
    }

    pub fn stats(&self) -> GlyphAtlasStats {
        GlyphAtlasStats {
            rasterize_count: self.rasterize_count,
            cache_hit_count: self.cache_hit_count,
            dirty_region_count: self.dirty_regions.len(),
        }
    }

    pub fn atlas_size(&self) -> u32 {
        self.size
    }

    pub fn tile_count(&self) -> usize {
        self.cache.len()
    }

    pub fn texture_format(&self) -> TextureFormat {
        ATLAS_TEXTURE_FORMAT
    }

    pub fn tile_pixels(&self, tile: AtlasTile) -> Vec<u8> {
        let mut out = vec![0u8; (tile.w * tile.h * 4) as usize];
        for row in 0..tile.h {
            let src_row = (tile.y + row) * self.size;
            let src_start = (src_row * 4 + tile.x * 4) as usize;
            let dst_start = row as usize * tile.w as usize * 4;
            let len = tile.w as usize * 4;
            out[dst_start..dst_start + len]
                .copy_from_slice(&self.cpu_pixels[src_start..src_start + len]);
        }
        out
    }

    pub fn dirty_regions(&self) -> impl Iterator<Item = AtlasDirtyRect> + '_ {
        self.dirty_regions.iter().copied().map(AtlasDirtyRect::from)
    }

    pub fn dirty_region_byte_count(&self) -> u64 {
        self.dirty_regions
            .iter()
            .map(|rect| rect.w as u64 * rect.h as u64 * 4)
            .sum()
    }

    /// Clears dirty-rect tracking after a successful GPU upload (or CPU-only test validation).
    pub fn clear_dirty_regions(&mut self) {
        self.dirty_regions.clear();
    }

    pub fn staging_pixels(&self) -> &[u8] {
        &self.cpu_pixels
    }

    fn dirty_rects(&self) -> &[DirtyRect] {
        &self.dirty_regions
    }

    fn cpu_pixels(&self) -> &[u8] {
        &self.cpu_pixels
    }

    fn atlas_size_internal(&self) -> u32 {
        self.size
    }
}

impl GlyphAtlas {
    pub fn new(device: &wgpu::Device, size: u32) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("typeface_glyph_atlas"),
            size: Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: ATLAS_TEXTURE_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            core: GlyphAtlasCore::new(size),
            texture,
            texture_view,
        }
    }

    pub fn get_or_rasterize(
        &mut self,
        font: &ProbeFont,
        glyph_id: u16,
        px: f32,
    ) -> Option<AtlasTile> {
        self.core.get_or_rasterize(font, glyph_id, px)
    }

    pub fn upload(&mut self, queue: &Queue) {
        if self.core.dirty_rects().is_empty() {
            return;
        }

        for dirty_rect in self.core.dirty_rects() {
            let bytes_per_row = align_bytes_per_row(dirty_rect.w);
            let mut staging = vec![0u8; (bytes_per_row * dirty_rect.h) as usize];
            copy_rect_to_staging(
                self.core.cpu_pixels(),
                self.core.atlas_size_internal(),
                dirty_rect.x,
                dirty_rect.y,
                dirty_rect.w,
                dirty_rect.h,
                bytes_per_row,
                &mut staging,
            );

            queue.write_texture(
                ImageCopyTexture {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: Origin3d {
                        x: dirty_rect.x,
                        y: dirty_rect.y,
                        z: 0,
                    },
                    aspect: TextureAspect::All,
                },
                &staging,
                ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(dirty_rect.h),
                },
                Extent3d {
                    width: dirty_rect.w,
                    height: dirty_rect.h,
                    depth_or_array_layers: 1,
                },
            );
        }

        self.core.clear_dirty_regions();
    }

    pub fn texture_view(&self) -> &TextureView {
        &self.texture_view
    }

    pub fn stats(&self) -> GlyphAtlasStats {
        self.core.stats()
    }

    pub fn atlas_size(&self) -> u32 {
        self.core.atlas_size()
    }

    pub fn tile_count(&self) -> usize {
        self.core.tile_count()
    }

    pub fn texture_format(&self) -> TextureFormat {
        self.core.texture_format()
    }

    pub fn tile_pixels(&self, tile: AtlasTile) -> Vec<u8> {
        self.core.tile_pixels(tile)
    }

    pub fn gpu_texture(&self) -> &Texture {
        &self.texture
    }
}

pub fn format_atlas_report(core: &GlyphAtlasCore) -> String {
    let stats = core.stats();
    format!(
        "atlas_size={}\ntile_count={}\nrasterize_count={}\ncache_hit_count={}\ndirty_region_count={}\nraster_gutter_px={}\nraster_uv_inset={}\ntexture_format={:?}",
        core.atlas_size(),
        core.tile_count(),
        stats.rasterize_count,
        stats.cache_hit_count,
        stats.dirty_region_count,
        RASTER_GLYPH_ATLAS_GUTTER_PX,
        RASTER_GLYPH_ATLAS_UV_INSET,
        core.texture_format()
    )
}

/// Full padded allocation rect for a tile (includes gutter bands).
pub fn tile_alloc_rect(tile: AtlasTile) -> AtlasDirtyRect {
    let gutter = RASTER_GLYPH_ATLAS_GUTTER_PX;
    AtlasDirtyRect {
        x: tile.x.saturating_sub(gutter),
        y: tile.y.saturating_sub(gutter),
        w: tile.w + 2 * gutter,
        h: tile.h + 2 * gutter,
    }
}

/// UV rect for the inner glyph content with optional half-texel inset.
pub fn tile_uv_rect(tile: AtlasTile, atlas_size: u32) -> [f32; 4] {
    let inv = 1.0 / atlas_size as f32;
    if !RASTER_GLYPH_ATLAS_UV_INSET || tile.w == 0 || tile.h == 0 {
        return [
            tile.x as f32 * inv,
            tile.y as f32 * inv,
            (tile.x + tile.w) as f32 * inv,
            (tile.y + tile.h) as f32 * inv,
        ];
    }
    let (u0, u1) = inset_axis_uv(tile.x, tile.w, atlas_size);
    let (v0, v1) = inset_axis_uv(tile.y, tile.h, atlas_size);
    [u0, v0, u1, v1]
}

fn inset_axis_uv(origin: u32, size: u32, atlas_size: u32) -> (f32, f32) {
    let atlas = atlas_size as f32;
    if size <= 1 {
        let center = (origin as f32 + 0.5) / atlas;
        return (center, center);
    }
    let min = (origin as f32 + 0.5) / atlas;
    let max = (origin as f32 + size as f32 - 0.5) / atlas;
    (min, max)
}

fn blit_rgba8_with_gutter(
    atlas: &mut [u8],
    atlas_size: u32,
    alloc_x: u32,
    _alloc_y: u32,
    content_x: u32,
    content_y: u32,
    w: u32,
    h: u32,
    gutter: u32,
    src: &[u8],
) {
    blit_rgba8(atlas, atlas_size, content_x, content_y, w, h, src);
    if gutter == 0 {
        return;
    }
    duplicate_gutter_edges(
        atlas, atlas_size, alloc_x, content_x, content_y, w, h, gutter,
    );
}

fn duplicate_gutter_edges(
    atlas: &mut [u8],
    atlas_size: u32,
    alloc_x: u32,
    content_x: u32,
    content_y: u32,
    w: u32,
    h: u32,
    gutter: u32,
) {
    let mut pixel = [0u8; 4];
    let alloc_w = w + 2 * gutter;
    for row in 0..h {
        let y = content_y + row;
        let left_off = pixel_offset(atlas_size, content_x, y);
        let right_off = pixel_offset(atlas_size, content_x + w - 1, y);
        for g in 1..=gutter {
            pixel.copy_from_slice(&atlas[left_off..left_off + 4]);
            let dst_left = pixel_offset(atlas_size, content_x - g, y);
            atlas[dst_left..dst_left + 4].copy_from_slice(&pixel);
            pixel.copy_from_slice(&atlas[right_off..right_off + 4]);
            let dst_right = pixel_offset(atlas_size, content_x + w - 1 + g, y);
            atlas[dst_right..dst_right + 4].copy_from_slice(&pixel);
        }
    }
    for col in 0..alloc_w {
        let x = alloc_x + col;
        let top_off = pixel_offset(atlas_size, x, content_y);
        let bottom_off = pixel_offset(atlas_size, x, content_y + h - 1);
        for g in 1..=gutter {
            pixel.copy_from_slice(&atlas[top_off..top_off + 4]);
            let dst_top = pixel_offset(atlas_size, x, content_y - g);
            atlas[dst_top..dst_top + 4].copy_from_slice(&pixel);
            pixel.copy_from_slice(&atlas[bottom_off..bottom_off + 4]);
            let dst_bottom = pixel_offset(atlas_size, x, content_y + h - 1 + g);
            atlas[dst_bottom..dst_bottom + 4].copy_from_slice(&pixel);
        }
    }
}

fn pixel_offset(atlas_size: u32, x: u32, y: u32) -> usize {
    ((y * atlas_size + x) * 4) as usize
}

fn blit_rgba8(atlas: &mut [u8], atlas_size: u32, x: u32, y: u32, w: u32, h: u32, src: &[u8]) {
    for row in 0..h {
        let dst_row = (y + row) * atlas_size;
        let dst_start = (dst_row * 4 + x * 4) as usize;
        let src_start = row as usize * w as usize * 4;
        let len = w as usize * 4;
        atlas[dst_start..dst_start + len].copy_from_slice(&src[src_start..src_start + len]);
    }
}

fn align_bytes_per_row(width: u32) -> u32 {
    let unpadded = width * 4;
    (unpadded + 255) & !255
}

fn copy_rect_to_staging(
    cpu_pixels: &[u8],
    atlas_size: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    bytes_per_row: u32,
    staging: &mut [u8],
) {
    for row in 0..h {
        let src_row = (y + row) * atlas_size;
        let src_start = (src_row * 4 + x * 4) as usize;
        let dst_start = row as usize * bytes_per_row as usize;
        let len = w as usize * 4;
        staging[dst_start..dst_start + len]
            .copy_from_slice(&cpu_pixels[src_start..src_start + len]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid_tile_pixels(w: u32, h: u32, alpha: u8) -> Vec<u8> {
        let mut pixels = vec![0u8; (w * h * 4) as usize];
        for px in pixels.chunks_mut(4) {
            px[0] = 255;
            px[1] = 255;
            px[2] = 255;
            px[3] = alpha;
        }
        pixels
    }

}
