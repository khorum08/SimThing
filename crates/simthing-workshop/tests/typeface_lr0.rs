use simthing_workshop::typeface::{
    ascii_sample_chars, format_measurement_report, load_font, measure_chars, TypefaceError,
};

const FIXTURE: &[u8] = include_bytes!("../assets/typeface/test_font.ttf");

fn load_fixture() -> simthing_workshop::typeface::ProbeFont {
    load_font(FIXTURE).expect("fixture font should parse")
}

#[test]
fn metrics_are_deterministic_across_repeated_calls() {
    let font = load_fixture();
    let first = font.glyph_metrics('A').expect("'A' must map");
    for _ in 0..8 {
        assert_eq!(first, font.glyph_metrics('A').expect("repeatable metrics"));
    }
}
