use simthing_tools::{
    load_font, GlyphAtlasCore, IconError, IconLayerRole, IconSet, IconVector, ShapingEngine,
    ICON_PUA_START,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const PX: f32 = 32.0;

const SIMPLE_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>
"##;

const ROLE_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <rect data-simthing-role="background" x="1" y="1" width="14" height="14" fill="#202020"/>
  <circle data-simthing-role="accent" cx="8" cy="8" r="4" fill="#ffffff"/>
  <path data-simthing-role="outline" d="M 1 1 L 15 1 L 15 15 L 1 15 Z" fill="none" stroke="#ffffff"/>
</svg>
"##;

fn fresh_atlas() -> GlyphAtlasCore {
    GlyphAtlasCore::new(128)
}

fn fixture_font() -> simthing_tools::ProbeFont {
    load_font(FIXTURE).expect("fixture font")
}

fn has_nonzero_alpha(bytes: &[u8]) -> bool {
    bytes.chunks(4).any(|px| px[3] > 0)
}

#[test]
fn normalizes_static_svg_to_icon_vector() {
    let vector = IconVector::from_svg(SIMPLE_SVG).expect("normalize");
    assert_eq!(vector.view_box, [0.0, 0.0, 16.0, 16.0]);
    assert_eq!(vector.layers.len(), 1);
    assert_eq!(vector.layers[0].role, IconLayerRole::Primary);
    assert!(!vector.layers[0].paths.is_empty());
    assert!(matches!(
        vector.layers[0].paths[0].commands.first(),
        Some(simthing_tools::IconPathCommand::MoveTo { .. })
    ));
}

#[test]
fn rejects_dynamic_or_external_svg() {
    let dynamic = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">
  <script>alert(1)</script>
  <rect onload="alert(1)" x="1" y="1" width="14" height="14"/>
</svg>
"##;
    assert!(matches!(
        IconVector::from_svg(dynamic),
        Err(IconError::StaticOnly(_))
    ));

    let external = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">
  <image href="https://example.invalid/icon.png" x="0" y="0" width="16" height="16"/>
</svg>
"##;
    assert!(matches!(
        IconVector::from_svg(external),
        Err(IconError::StaticOnly(_))
    ));
}

#[test]
fn registers_svg_icon_tile() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let registration = icons
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, PX, &mut atlas)
        .expect("register icon");

    assert!(registration.tile.w > 0);
    assert!(registration.tile.h > 0);
    assert!(has_nonzero_alpha(&atlas.tile_pixels(registration.tile)));
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

#[test]
fn same_icon_same_px_is_cached() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();

    let first = icons
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, PX, &mut atlas)
        .expect("first");
    let after_first = atlas.stats();
    let second = icons
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, PX, &mut atlas)
        .expect("second");
    let after_second = atlas.stats();

    assert_eq!(first.tile, second.tile);
    assert_eq!(after_first.rasterize_count, after_second.rasterize_count);
    assert_eq!(
        after_first.dirty_region_count,
        after_second.dirty_region_count
    );
}

#[test]
fn invalid_svg_errors_no_panic_no_atlas_mutation() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let before = atlas.stats();
    let before_tiles = atlas.tile_count();

    let err = icons
        .register_svg(ICON_PUA_START + 1, "<svg><path", PX, &mut atlas)
        .expect_err("invalid SVG");

    assert!(matches!(err, IconError::Parse(_)));
    assert_eq!(before, atlas.stats());
    assert_eq!(before_tiles, atlas.tile_count());
}

#[test]
fn pua_codepoint_renders_in_mixed_run() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let font = fixture_font();
    let mut shaper = ShapingEngine::new_with_font(FIXTURE.to_vec()).expect("shaper");
    let icon_tile = icons
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, PX, &mut atlas)
        .expect("register icon")
        .tile;

    let instances = icons
        .build_mixed_instances(
            &font,
            &mut shaper,
            &mut atlas,
            "Sol \u{F0001} 42",
            PX,
            [1.0, 1.0, 1.0, 1.0],
        )
        .expect("mixed instances");

    let inv = 1.0 / atlas.atlas_size() as f32;
    let icon_uv = [
        icon_tile.x as f32 * inv,
        icon_tile.y as f32 * inv,
        (icon_tile.x + icon_tile.w) as f32 * inv,
        (icon_tile.y + icon_tile.h) as f32 * inv,
    ];
    assert!(instances.len() > 1);
    assert_eq!(instances.iter().filter(|i| i.uv_rect == icon_uv).count(), 1);
}

#[test]
fn icon_and_glyph_share_one_atlas() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let font = fixture_font();
    let glyph = font.glyph_metrics('S').expect("glyph metrics");
    let glyph_tile = atlas
        .get_or_rasterize(&font, glyph.glyph_id, PX)
        .expect("glyph tile");
    let icon_tile = icons
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, PX, &mut atlas)
        .expect("icon tile")
        .tile;

    assert_ne!(glyph_tile, icon_tile);
    assert!(has_nonzero_alpha(&atlas.tile_pixels(glyph_tile)));
    assert!(has_nonzero_alpha(&atlas.tile_pixels(icon_tile)));
    assert_eq!(atlas.tile_count(), 1);
    assert_eq!(icons.tile_for(ICON_PUA_START + 1), Some(icon_tile));
}

#[test]
fn role_tags_are_preserved_in_icon_vector_ir() {
    let vector = IconVector::from_svg(ROLE_SVG).expect("role vector");
    let roles = vector
        .layers
        .iter()
        .map(|layer| layer.role)
        .collect::<Vec<_>>();

    assert_eq!(
        roles,
        vec![
            IconLayerRole::Background,
            IconLayerRole::Accent,
            IconLayerRole::Outline,
        ]
    );
}
