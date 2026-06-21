use pollster::FutureExt;
use simthing_workshop::typeface::{
    format_atlas_report, load_font, rasterize_glyph_cpu, GlyphAtlas, GlyphAtlasCore, ProbeFont,
};
use wgpu::{
    Backends, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, DeviceDescriptor, Extent3d,
    Features, ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, Instance, InstanceDescriptor,
    Maintain, MapMode, Origin3d, PowerPreference, RequestAdapterOptions, TextureAspect,
};

const FIXTURE: &[u8] = include_bytes!("../assets/typeface/test_font.ttf");
const SHAPE_PX: f32 = 32.0;

struct TestGpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

fn try_test_gpu_context() -> Option<TestGpuContext> {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::PRIMARY,
        ..Default::default()
    });
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .block_on()?;
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("typeface_lr2_test"),
                required_features: Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )
        .block_on()
        .ok()?;
    Some(TestGpuContext { device, queue })
}

fn load_fixture() -> ProbeFont {
    load_font(FIXTURE).expect("fixture font should parse")
}

fn glyph_id_for(font: &ProbeFont, ch: char) -> u16 {
    font.glyph_metrics(ch)
        .expect("fixture glyph must map")
        .glyph_id
}

fn tiles_overlap(
    a: simthing_workshop::typeface::AtlasTile,
    b: simthing_workshop::typeface::AtlasTile,
) -> bool {
    a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h
}

fn make_cpu_atlas(size: u32) -> GlyphAtlasCore {
    GlyphAtlasCore::new(size)
}

#[test]
fn rasterized_tile_bytes_match_cpu_oracle() {
    let font = load_fixture();
    let glyph_id = glyph_id_for(&font, 'A');
    let oracle =
        rasterize_glyph_cpu(&font, glyph_id, SHAPE_PX).expect("oracle raster must succeed");

    let mut atlas = make_cpu_atlas(256);
    let tile = atlas
        .get_or_rasterize(&font, glyph_id, SHAPE_PX)
        .expect("atlas raster must succeed");

    assert_eq!(tile.w, oracle.w);
    assert_eq!(tile.h, oracle.h);
    assert_eq!(tile.left, oracle.left);
    assert_eq!(tile.top, oracle.top);
    assert_eq!(atlas.tile_pixels(tile), oracle.pixels);
}

#[test]
fn same_glyph_same_px_is_cached_not_re_rasterized() {
    let font = load_fixture();
    let glyph_id = glyph_id_for(&font, 'A');
    let mut atlas = make_cpu_atlas(256);

    let first = atlas
        .get_or_rasterize(&font, glyph_id, SHAPE_PX)
        .expect("first raster must succeed");
    let second = atlas
        .get_or_rasterize(&font, glyph_id, SHAPE_PX)
        .expect("cached raster must succeed");

    assert_eq!(first, second);
    let stats = atlas.stats();
    assert_eq!(stats.rasterize_count, 1);
    assert_eq!(stats.cache_hit_count, 1);
}

#[test]
fn distinct_glyphs_get_distinct_tiles() {
    let font = load_fixture();
    let glyph_a = glyph_id_for(&font, 'A');
    let glyph_b = glyph_id_for(&font, 'B');
    let mut atlas = make_cpu_atlas(256);

    let tile_a = atlas
        .get_or_rasterize(&font, glyph_a, SHAPE_PX)
        .expect("glyph A must rasterize");
    let tile_b = atlas
        .get_or_rasterize(&font, glyph_b, SHAPE_PX)
        .expect("glyph B must rasterize");

    assert_ne!(tile_a, tile_b);
    assert!(!tiles_overlap(tile_a, tile_b));
}

#[test]
fn atlas_full_returns_none_no_panic() {
    let font = load_fixture();
    let mut atlas = make_cpu_atlas(64);

    let mut saw_some = false;
    let mut saw_none = false;
    for ch in 'A'..='Z' {
        let glyph_id = glyph_id_for(&font, ch);
        match atlas.get_or_rasterize(&font, glyph_id, SHAPE_PX) {
            Some(_) => saw_some = true,
            None => {
                saw_none = true;
                break;
            }
        }
    }

    assert!(
        saw_some,
        "at least one glyph should pack into the tiny atlas"
    );
    assert!(
        saw_none,
        "tiny atlas should eventually return None without panic"
    );
}

#[test]
fn upload_dirty_regions_clears_dirty_tracking() {
    let font = load_fixture();
    let glyph_id = glyph_id_for(&font, 'A');

    let mut core = make_cpu_atlas(256);
    assert!(core.get_or_rasterize(&font, glyph_id, SHAPE_PX).is_some());
    assert!(core.stats().dirty_region_count > 0);

    if let Some(ctx) = try_test_gpu_context() {
        let mut atlas = GlyphAtlas::new(&ctx.device, 256);
        assert!(atlas.get_or_rasterize(&font, glyph_id, SHAPE_PX).is_some());
        assert!(atlas.stats().dirty_region_count > 0);
        atlas.upload(&ctx.queue);
        assert_eq!(atlas.stats().dirty_region_count, 0);
    } else {
        core.clear_dirty_regions();
        assert_eq!(core.stats().dirty_region_count, 0);
        eprintln!("ADAPTER_SKIPPED: GPU upload path; CPU dirty tracking cleared via core");
    }
}

#[test]
fn cached_tile_does_not_mark_dirty() {
    let font = load_fixture();
    let glyph_id = glyph_id_for(&font, 'A');
    let mut atlas = make_cpu_atlas(256);

    let first = atlas
        .get_or_rasterize(&font, glyph_id, SHAPE_PX)
        .expect("first raster must succeed");
    atlas.clear_dirty_regions();
    assert_eq!(atlas.stats().dirty_region_count, 0);

    let second = atlas
        .get_or_rasterize(&font, glyph_id, SHAPE_PX)
        .expect("cached raster must succeed");
    assert_eq!(first, second);
    assert_eq!(atlas.stats().dirty_region_count, 0);
}

#[test]
fn headless_real_adapter_upload_readback_or_skip() {
    let font = load_fixture();
    let glyph_id = glyph_id_for(&font, 'A');

    let Some(ctx) = try_test_gpu_context() else {
        eprintln!("ADAPTER_SKIPPED: no real GPU adapter available for LR2 readback");
        return;
    };

    let mut atlas = GlyphAtlas::new(&ctx.device, 256);
    let tile = atlas
        .get_or_rasterize(&font, glyph_id, SHAPE_PX)
        .expect("atlas raster must succeed");
    let expected = atlas.tile_pixels(tile);
    atlas.upload(&ctx.queue);

    let bytes_per_row = ((tile.w * 4) + 255) & !255;
    let staging_size = bytes_per_row as u64 * tile.h as u64;
    let staging = ctx.device.create_buffer(&BufferDescriptor {
        label: Some("typeface_lr2_readback"),
        size: staging_size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("typeface_lr2_readback_enc"),
        });
    encoder.copy_texture_to_buffer(
        ImageCopyTexture {
            texture: atlas.gpu_texture(),
            mip_level: 0,
            origin: Origin3d {
                x: tile.x,
                y: tile.y,
                z: 0,
            },
            aspect: TextureAspect::All,
        },
        ImageCopyBuffer {
            buffer: &staging,
            layout: ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(tile.h),
            },
        },
        Extent3d {
            width: tile.w,
            height: tile.h,
            depth_or_array_layers: 1,
        },
    );
    ctx.queue.submit(Some(encoder.finish()));

    let slice = staging.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    slice.map_async(MapMode::Read, move |result| {
        let _ = sender.send(result);
    });
    ctx.device.poll(Maintain::Wait);
    receiver
        .recv()
        .expect("map_async callback")
        .expect("map_async must succeed");

    let mapped = slice.get_mapped_range();
    let mut readback = Vec::with_capacity(expected.len());
    for row in 0..tile.h {
        let start = row as usize * bytes_per_row as usize;
        let end = start + tile.w as usize * 4;
        readback.extend_from_slice(&mapped[start..end]);
    }
    drop(mapped);
    staging.unmap();

    assert_eq!(readback, expected);
    eprintln!("REAL_ADAPTER_OBSERVED: LR2 atlas upload/readback byte match");
}

#[test]
fn atlas_report_contains_expected_fields() {
    let font = load_fixture();
    let glyph_id = glyph_id_for(&font, 'A');
    let mut atlas = make_cpu_atlas(256);
    let _ = atlas.get_or_rasterize(&font, glyph_id, SHAPE_PX);
    let report = format_atlas_report(&atlas);
    assert!(report.contains("atlas_size="));
    assert!(report.contains("tile_count="));
    assert!(report.contains("rasterize_count="));
    assert!(report.contains("cache_hit_count="));
    assert!(report.contains("dirty_region_count="));
    assert!(report.contains("texture_format="));
}
