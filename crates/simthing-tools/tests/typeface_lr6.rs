use read_fonts::FontRef;
use simthing_tools::{DistanceFieldAtlasCore, DistanceFieldError, GlyphSource};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const TEST_PX: f32 = 32.0;
const ATLAS_SIZE: u32 = 512;

fn probe_font() -> simthing_tools::ProbeFont {
    simthing_tools::load_font(FIXTURE).expect("fixture font")
}

fn glyph_id_for_char(ch: char) -> u32 {
    let font = probe_font();
    font.glyph_id_for_char(ch).expect("glyph").to_u32()
}

fn atlas_region_pixels(
    atlas: &DistanceFieldAtlasCore,
    tile: &simthing_tools::DistanceFieldTile,
) -> Vec<u8> {
    let atlas_size = atlas.atlas_size() as usize;
    let mut buf = Vec::new();
    let pixels = atlas.staging_pixels();
    for y in tile.atlas_tile.y..tile.atlas_tile.y + tile.atlas_tile.h {
        let start = ((y as usize * atlas_size) + tile.atlas_tile.x as usize) * 4;
        let end = start + tile.atlas_tile.w as usize * 4;
        buf.extend_from_slice(&pixels[start..end]);
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

#[test]
fn glyph_source_api_does_not_silently_claim_unsupported_glyph_ids() {
    let font = probe_font();
    let face = FontRef::new(FIXTURE).expect("parse fixture");
    let invalid = u32::from(face.maxp().num_glyphs()) + 100;
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let err = atlas
        .get_or_generate_glyph_msdf(&font, invalid, TEST_PX)
        .expect_err("glyph id without outline must error explicitly");
    assert!(matches!(err, DistanceFieldError::MissingOutline(_)));
}
