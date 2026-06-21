use super::font::{GlyphMetrics, ProbeFont};

#[derive(Debug, Clone, PartialEq)]
pub struct MeasuredGlyph {
    pub ch: char,
    pub metrics: GlyphMetrics,
}

/// Sample glyph metrics for the given characters, skipping unmapped code points.
pub fn measure_chars(font: &ProbeFont, chars: &[char]) -> Vec<MeasuredGlyph> {
    chars
        .iter()
        .filter_map(|&ch| {
            font.glyph_metrics(ch)
                .map(|metrics| MeasuredGlyph { ch, metrics })
        })
        .collect()
}

/// Deterministic ASCII workshop sample used by measurement probes and reports.
pub fn ascii_sample_chars() -> &'static [char] {
    &['A', 'V', '0', ' ']
}

/// Build a compact, deterministic text report for workshop measurement runs.
pub fn format_measurement_report(font: &ProbeFont, chars: &[char]) -> String {
    let mut lines = vec![
        format!("units_per_em={}", font.units_per_em()),
        format!("glyph_count={}", font.glyph_count()),
    ];

    for sample in measure_chars(font, chars) {
        let GlyphMetrics {
            advance,
            bounds,
            glyph_id,
        } = sample.metrics;
        lines.push(format!(
            "ch='{}' glyph_id={glyph_id} advance={advance} bounds=[{:.3},{:.3},{:.3},{:.3}]",
            sample.ch, bounds[0], bounds[1], bounds[2], bounds[3]
        ));
    }

    lines.join("\n")
}
