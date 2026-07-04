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
