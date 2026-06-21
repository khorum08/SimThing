use std::collections::HashMap;

use guillotiere::{size2, AtlasAllocator};
use swash::scale::{image::Content, Render, ScaleContext, Source};
use swash::FontRef;
use wgpu::{
    Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, Queue, Texture, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
};

use super::font::ProbeFont;

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
struct DirtyRegion {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

/// CPU-side atlas: rasterization, guillotiere packing, cache, and dirty-region tracking.
pub struct GlyphAtlasCore {
    size: u32,
    cpu_pixels: Vec<u8>,
    allocator: AtlasAllocator,
    cache: HashMap<GlyphAtlasKey, AtlasTile>,
    dirty_regions: Vec<DirtyRegion>,
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
        let allocation = self
            .allocator
            .allocate(size2(raster.w as i32, raster.h as i32))?;
        let rect = self.allocator[allocation.id];
        let x = rect.min.x as u32;
        let y = rect.min.y as u32;

        blit_rgba8(
            &mut self.cpu_pixels,
            self.size,
            x,
            y,
            raster.w,
            raster.h,
            &raster.pixels,
        );

        let tile = AtlasTile {
            x,
            y,
            w: raster.w,
            h: raster.h,
            left: raster.left,
            top: raster.top,
        };
        self.cache.insert(key, tile);
        self.rasterize_count += 1;
        self.dirty_regions.push(DirtyRegion {
            x,
            y,
            w: raster.w,
            h: raster.h,
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

    /// Clears dirty-region tracking after a successful GPU upload (or CPU-only test validation).
    pub fn clear_dirty_regions(&mut self) {
        self.dirty_regions.clear();
    }

    fn dirty_regions(&self) -> &[DirtyRegion] {
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
        if self.core.dirty_regions().is_empty() {
            return;
        }

        for region in self.core.dirty_regions() {
            let bytes_per_row = align_bytes_per_row(region.w);
            let mut staging = vec![0u8; (bytes_per_row * region.h) as usize];
            copy_region_to_staging(
                self.core.cpu_pixels(),
                self.core.atlas_size_internal(),
                region.x,
                region.y,
                region.w,
                region.h,
                bytes_per_row,
                &mut staging,
            );

            queue.write_texture(
                ImageCopyTexture {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: Origin3d {
                        x: region.x,
                        y: region.y,
                        z: 0,
                    },
                    aspect: TextureAspect::All,
                },
                &staging,
                ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(region.h),
                },
                Extent3d {
                    width: region.w,
                    height: region.h,
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
        "atlas_size={}\ntile_count={}\nrasterize_count={}\ncache_hit_count={}\ndirty_region_count={}\ntexture_format={:?}",
        core.atlas_size(),
        core.tile_count(),
        stats.rasterize_count,
        stats.cache_hit_count,
        stats.dirty_region_count,
        core.texture_format()
    )
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

fn copy_region_to_staging(
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
