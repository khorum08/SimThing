use simthing_workshop::typeface::{
    ascii_sample_chars, format_measurement_report, load_font, measure_chars, TypefaceError,
};

const FIXTURE: &[u8] = include_bytes!("../assets/typeface/test_font.ttf");

fn load_fixture() -> simthing_workshop::typeface::ProbeFont {
    load_font(FIXTURE).expect("fixture font should parse")
}

#[test]
fn loads_fixture_font_units_per_em() {
    let font = load_fixture();
    assert!(font.units_per_em() > 0, "units_per_em must be nonzero");
}

#[test]
fn glyph_metrics_for_known_ascii_is_stable() {
    let font = load_fixture();
    let metrics = font
        .glyph_metrics('A')
        .expect("'A' must map in fixture font");

    assert_ne!(
        metrics.glyph_id, 0,
        "glyph_id should be stable and nonzero for 'A'"
    );
    assert!(metrics.advance > 0.0, "advance must be positive");
    assert!(
        metrics.bounds.iter().all(|v| v.is_finite()),
        "bounds must be finite"
    );

    let again = font
        .glyph_metrics('A')
        .expect("'A' metrics must be repeatable");
    assert_eq!(metrics, again);
}

#[test]
fn unmapped_char_returns_none() {
    let font = load_fixture();
    // Private-use code point U+E000 is not mapped in the Noto Sans fixture cmap.
    assert!(font.glyph_metrics('\u{E000}').is_none());
}

#[test]
fn glyph_count_positive() {
    let font = load_fixture();
    assert!(font.glyph_count() > 0);
}

#[test]
fn load_garbage_bytes_errors() {
    let err = load_font(b"not a font").expect_err("garbage bytes must error");
    match err {
        TypefaceError::Parse(_) => {}
        other => panic!("expected Parse error, got {other:?}"),
    }
}

#[test]
fn metrics_are_deterministic_across_repeated_calls() {
    let font = load_fixture();
    let first = font.glyph_metrics('A').expect("'A' must map");
    for _ in 0..8 {
        assert_eq!(first, font.glyph_metrics('A').expect("repeatable metrics"));
    }
}

#[test]
fn measurement_harness_reports_ascii_sample() {
    let font = load_fixture();
    let chars = ascii_sample_chars();
    let measured = measure_chars(&font, chars);
    assert!(
        !measured.is_empty(),
        "ASCII sample should yield mapped glyphs"
    );

    let report = format_measurement_report(&font, chars);
    assert!(report.contains("units_per_em="));
    assert!(report.contains("glyph_count="));
    assert!(report.contains("ch='A'"));
}
