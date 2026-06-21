use skrifa::{
    instance::{LocationRef, Size},
    metrics::GlyphMetrics as SkrifaGlyphMetrics,
    raw::TableProvider,
    FontRef, MetadataProvider,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphMetrics {
    pub advance: f32,
    pub bounds: [f32; 4],
    pub glyph_id: u16,
}

#[derive(Debug, thiserror::Error)]
pub enum TypefaceError {
    #[error("font parse: {0}")]
    Parse(String),

    #[error("font table missing: {0}")]
    MissingTable(String),

    #[error("font collection index out of range: {0}")]
    CollectionIndex(usize),
}

#[derive(Debug)]
pub struct ProbeFont {
    bytes: Vec<u8>,
    units_per_em: u16,
    glyph_count: u16,
}

pub fn load_font(bytes: &[u8]) -> Result<ProbeFont, TypefaceError> {
    let font = parse_font_ref(bytes)?;
    let units_per_em = font
        .head()
        .map_err(|err| map_read_error("head", err))?
        .units_per_em();

    let glyph_metrics = SkrifaGlyphMetrics::new(
        &font,
        Size::new(f32::from(units_per_em)),
        LocationRef::default(),
    );
    let glyph_count = glyph_metrics.glyph_count();
    if glyph_count == 0 {
        return Err(TypefaceError::Parse("font reports zero glyphs".to_string()));
    }
    if glyph_count > u16::MAX as u32 {
        return Err(TypefaceError::Parse(format!(
            "glyph count {glyph_count} exceeds u16::MAX"
        )));
    }

    let mut db = fontdb::Database::new();
    db.load_font_data(bytes.to_vec());

    Ok(ProbeFont {
        bytes: bytes.to_vec(),
        units_per_em,
        glyph_count: glyph_count as u16,
    })
}

impl ProbeFont {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }

    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }

    pub fn glyph_metrics(&self, ch: char) -> Option<GlyphMetrics> {
        let font = self.font_ref().ok()?;
        let glyph_id = font.charmap().map(ch as u32)?;
        let skrifa_metrics = SkrifaGlyphMetrics::new(
            &font,
            Size::new(f32::from(self.units_per_em)),
            LocationRef::default(),
        );

        let advance = skrifa_metrics.advance_width(glyph_id)?;
        let bounds = skrifa_metrics.bounds(glyph_id)?;
        let glyph_id_u32 = glyph_id.to_u32();
        if glyph_id_u32 > u16::MAX as u32 {
            return None;
        }

        Some(GlyphMetrics {
            advance,
            bounds: [bounds.x_min, bounds.y_min, bounds.x_max, bounds.y_max],
            glyph_id: glyph_id_u32 as u16,
        })
    }

    fn font_ref(&self) -> Result<FontRef<'_>, TypefaceError> {
        parse_font_ref(&self.bytes)
    }
}

fn parse_font_ref(bytes: &[u8]) -> Result<FontRef<'_>, TypefaceError> {
    FontRef::new(bytes).map_err(|err| TypefaceError::Parse(err.to_string()))
}

fn map_read_error(table: &str, err: skrifa::raw::ReadError) -> TypefaceError {
    match err {
        skrifa::raw::ReadError::TableIsMissing(_) | skrifa::raw::ReadError::MetricIsMissing(_) => {
            TypefaceError::MissingTable(table.to_string())
        }
        skrifa::raw::ReadError::InvalidCollectionIndex(index) => {
            TypefaceError::CollectionIndex(index as usize)
        }
        other => TypefaceError::Parse(other.to_string()),
    }
}
