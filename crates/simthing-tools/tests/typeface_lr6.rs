use std::{fs, path::PathBuf};

use bevy::prelude::*;
use msdf_font::ttf_parser::Face;
use simthing_tools::{
    build_distance_field_instance, distance_field_diagnostics, load_font,
    numeric_damage_lane_diagnostics, spawn_static_and_numeric_damage_labels, text_perf_diagnostics,
    wgpu_instanced_text_smoke, wgpu_sdf_instanced_text_smoke, DistanceFieldAtlasCore,
    DistanceFieldError, DistanceFieldKind, GlyphInstanceGpu, IconVector, SimthingToolsTextPlugin,
    TextGlyphInstances, TextLabel, WgpuSmokeTarget, DISTANCE_FIELD_RENDER_MSDF,
    DISTANCE_FIELD_RENDER_RASTER,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const TEST_PX: f32 = 32.0;
const ATLAS_SIZE: u32 = 512;
const SMOKE_WIDTH: u32 = 256;
const SMOKE_HEIGHT: u32 = 128;

fn fixture_bytes() -> Vec<u8> {
    FIXTURE.to_vec()
}

fn probe_font() -> simthing_tools::ProbeFont {
    load_font(&fixture_bytes()).expect("fixture font")
}

fn glyph_id_for_char(ch: char) -> u32 {
    let face = Face::parse(FIXTURE, 0).expect("parse fixture");
    face.glyph_index(ch).expect("glyph index").0.into()
}

fn atlas_region_pixels(
    atlas: &DistanceFieldAtlasCore,
    tile: &simthing_tools::DistanceFieldTile,
) -> Vec<u8> {
    let staging = atlas.staging_pixels();
    let size = atlas.atlas_size();
    let mut out = vec![0u8; (tile.atlas_tile.w * tile.atlas_tile.h * 4) as usize];
    for row in 0..tile.atlas_tile.h {
        let src_row = ((tile.atlas_tile.y + row) * size + tile.atlas_tile.x) * 4;
        let dst_row = row * tile.atlas_tile.w * 4;
        let len = (tile.atlas_tile.w * 4) as usize;
        out[dst_row as usize..dst_row as usize + len]
            .copy_from_slice(&staging[src_row as usize..src_row as usize + len]);
    }
    out
}

fn cpu_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(SimthingToolsTextPlugin::with_atlas_size(
            fixture_bytes(),
            4096,
        ));
    app
}

fn wgpu_adapter_available() -> bool {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .is_some()
}

fn encode_png_rgba(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buf, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().expect("png header");
        writer.write_image_data(pixels).expect("png data");
    }
    buf
}

#[test]
fn msdf_glyph_tile_is_deterministic() {
    let font = probe_font();
    let glyph_id = glyph_id_for_char('A');
    let mut a = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let mut b = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile_a = a
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("msdf a");
    let tile_b = b
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("msdf b");
    assert_eq!(tile_a, tile_b);
    assert_eq!(
        atlas_region_pixels(&a, &tile_a),
        atlas_region_pixels(&b, &tile_b)
    );
}
