use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap};

use crate::font::{load_font, TypefaceError};

const LINE_HEIGHT_RATIO: f32 = 1.2;

#[derive(Debug, Clone, PartialEq)]
pub struct ShapedGlyph {
    pub glyph_id: u16,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
    /// UTF-8 byte index of the cluster start in the shaped source string (`LayoutGlyph::start`).
    pub cluster: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapedRun {
    pub glyphs: Vec<ShapedGlyph>,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub struct ShapingEngine {
    #[allow(dead_code)] // retained so workshop engines own font bytes alongside fontdb state
    font_bytes: Vec<u8>,
    family_name: String,
    font_system: FontSystem,
    line_height_ratio: f32,
}

impl ShapingEngine {
    pub fn new_with_font(bytes: Vec<u8>) -> Result<Self, TypefaceError> {
        let probe = load_font(&bytes)?;

        let mut db = fontdb::Database::new();
        db.load_font_data(bytes.clone());
        let font_id = db
            .faces()
            .next()
            .map(|face| face.id)
            .ok_or_else(|| TypefaceError::Parse("loaded font database is empty".to_string()))?;
        let family_name = db
            .face(font_id)
            .map(|face| face.families.first().map(|(name, _)| name.clone()))
            .flatten()
            .ok_or_else(|| TypefaceError::Parse("loaded font has no family name".to_string()))?;

        let _ = probe;

        Ok(Self {
            font_bytes: bytes,
            family_name,
            font_system: FontSystem::new_with_locale_and_db("en-US".to_string(), db),
            line_height_ratio: LINE_HEIGHT_RATIO,
        })
    }

    pub fn shape(&mut self, text: &str, px: f32) -> ShapedRun {
        let line_height = px * self.line_height_ratio;
        if text.is_empty() {
            return ShapedRun {
                glyphs: Vec::new(),
                width: 0.0,
                height: line_height,
            };
        }

        let metrics = Metrics::new(px, line_height);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        let mut buffer = buffer.borrow_with(&mut self.font_system);
        buffer.set_wrap(Wrap::None);
        buffer.set_size(Some(f32::MAX), None);

        let attrs = Attrs::new().family(Family::Name(&self.family_name));
        buffer.set_text(text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll(true);

        let mut glyphs = Vec::new();
        let mut width = 0.0f32;
        let mut height = line_height;

        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            height = height.max(run.line_top + run.line_height);

            for glyph in run.glyphs {
                glyphs.push(ShapedGlyph {
                    glyph_id: glyph.glyph_id,
                    x: glyph.x,
                    y: run.line_y + glyph.y,
                    advance: glyph.w,
                    cluster: glyph.start,
                });
            }
        }

        ShapedRun {
            glyphs,
            width,
            height,
        }
    }
}

/// Build a compact, deterministic shaping report for workshop diagnostics.
pub fn format_shaping_report(engine: &mut ShapingEngine, text: &str, px: f32) -> String {
    let run = engine.shape(text, px);
    format!(
        "text={text:?}\npx={px}\nglyph_count={}\nwidth={}\nheight={}",
        run.glyphs.len(),
        run.width,
        run.height
    )
}
