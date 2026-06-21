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
fn shapes_ascii_advances_monotonic() {
    let mut engine = load_engine();
    let run = engine.shape("SimThing", SHAPE_PX);

    assert!(!run.glyphs.is_empty());
    assert!(run.width > 0.0);

    let mut prev_x = f32::NEG_INFINITY;
    for glyph in &run.glyphs {
        assert!(glyph.x.is_finite());
        assert!(glyph.advance.is_finite());
        assert!(glyph.advance >= 0.0);
        assert!(glyph.x >= prev_x, "glyph x positions must be nondecreasing");
        prev_x = glyph.x;
    }
}

#[test]
fn kerning_pair_av_tighter_than_naive() {
    let font = load_font(FIXTURE).expect("fixture font should parse");
    let mut engine = load_engine();
    let naive = naive_pair_width(&font, 'A', 'V', SHAPE_PX);
    let shaped = engine.shape("AV", SHAPE_PX);

    assert!(
        shaped.width <= naive,
        "kerning-shaped width {} should be <= naive width {}",
        shaped.width,
        naive
    );
    assert!(
        shaped.width < naive - KERN_EPSILON,
        "Noto Sans AV kerning should be strictly tighter than naive metrics (shaped={}, naive={})",
        shaped.width,
        naive
    );
}

#[test]
fn ligature_fi_collapses_when_font_has_it_else_two_glyphs() {
    let mut engine = load_engine();
    let run = engine.shape("fi", SHAPE_PX);

    assert!(
        run.glyphs.len() == 1 || run.glyphs.len() == 2,
        "fi should shape to one ligature glyph or two separate glyphs, got {}",
        run.glyphs.len()
    );

    if run.glyphs.len() == 1 {
        eprintln!("fixture supports fi ligature (single glyph)");
    } else {
        eprintln!("fixture/cosmic-text path did not collapse fi (two glyphs)");
    }
}

#[test]
fn empty_string_is_empty_run() {
    let mut engine = load_engine();
    let run = engine.shape("", SHAPE_PX);

    assert!(run.glyphs.is_empty());
    assert_eq!(run.width, 0.0);
    assert!(run.height.is_finite());
    assert!(run.height >= 0.0);
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

#[test]
fn shape_garbage_font_errors() {
    let err = ShapingEngine::new_with_font(vec![0, 1, 2, 3]).expect_err("garbage must error");
    match err {
        TypefaceError::Parse(_) => {}
        other => panic!("expected Parse error, got {other:?}"),
    }
}

#[test]
fn shape_fixture_sample_report_is_stable() {
    let mut engine = load_engine();
    let report = format_shaping_report(&mut engine, "AV", SHAPE_PX);

    assert!(report.contains("text="));
    assert!(report.contains("px="));
    assert!(report.contains("glyph_count="));
    assert!(report.contains("width="));
    assert!(report.contains("height="));

    let again = format_shaping_report(&mut engine, "AV", SHAPE_PX);
    assert_eq!(report, again);
}
