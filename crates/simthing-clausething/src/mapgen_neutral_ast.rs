//! MapGen PR2 — neutral-AST parse-only adapter (M1).
//!
//! Raw text → [`RawDocument`] via the existing jomini text path. No semantic mapping, no SimThing
//! structure generation, and no hydration.

use crate::error::ParseError;
use crate::parse::parse_raw_document;
use crate::raw::RawDocument;

/// Parse-only MapGen neutral document wrapper (M1).
///
/// Contains only the parsed raw AST plus source byte-length metadata. No interpreted mapgen entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapGenNeutralDocument {
    pub document: RawDocument,
    pub source_byte_len: usize,
}

/// Parse a hand-authored MapGen fixture into a neutral AST without semantic decisions.
pub fn parse_mapgen_neutral_document(source: &[u8]) -> Result<MapGenNeutralDocument, ParseError> {
    let document = parse_raw_document(source)?;
    Ok(MapGenNeutralDocument {
        document,
        source_byte_len: source.len(),
    })
}
