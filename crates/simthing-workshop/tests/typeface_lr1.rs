use simthing_workshop::typeface::{format_shaping_report, load_font, ShapingEngine, TypefaceError};

const FIXTURE: &[u8] = include_bytes!("../assets/typeface/test_font.ttf");
const SHAPE_PX: f32 = 32.0;
const KERN_EPSILON: f32 = 0.01;

fn load_engine() -> ShapingEngine {
    ShapingEngine::new_with_font(FIXTURE.to_vec()).expect("fixture font should load")
}

fn naive_pair_width(
    font: &simthing_workshop::typeface::ProbeFont,
    left: char,
    right: char,
    px: f32,
) -> f32 {
    let upe = font.units_per_em() as f32;
    let scale = px / upe;
    let left_advance = font
        .glyph_metrics(left)
        .expect("left glyph must map")
        .advance;
    let right_advance = font
        .glyph_metrics(right)
        .expect("right glyph must map")
        .advance;
    (left_advance + right_advance) * scale
}

#[test]
fn shaping_is_deterministic() {
    let mut engine = load_engine();
    let first = engine.shape("Hello", SHAPE_PX);
    let second = engine.shape("Hello", SHAPE_PX);
    assert_eq!(first, second);

    let mut fresh_engine =
        ShapingEngine::new_with_font(FIXTURE.to_vec()).expect("fixture font should load");
    let fresh = fresh_engine.shape("Hello", SHAPE_PX);
    assert_eq!(first, fresh);
}
