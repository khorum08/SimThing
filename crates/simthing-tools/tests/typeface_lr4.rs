use simthing_tools::{GlyphAtlasCore, IconSet, ICON_PUA_START};

const PX: f32 = 32.0;

const SIMPLE_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>
"##;

fn fresh_atlas() -> GlyphAtlasCore {
    GlyphAtlasCore::new(128)
}

#[test]
fn icon_tile_bytes_deterministic() {
    let mut atlas_a = fresh_atlas();
    let mut atlas_b = fresh_atlas();
    let mut icons_a = IconSet::new();
    let mut icons_b = IconSet::new();

    let tile_a = icons_a
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, PX, &mut atlas_a)
        .expect("register a")
        .tile;
    let tile_b = icons_b
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, PX, &mut atlas_b)
        .expect("register b")
        .tile;

    assert_eq!(atlas_a.tile_pixels(tile_a), atlas_b.tile_pixels(tile_b));
}
