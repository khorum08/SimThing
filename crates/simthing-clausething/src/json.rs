//! Deterministic JSON serialization for CT-0b goldens.

use crate::error::ParseError;
use crate::raw::RawDocument;

/// Serialize the raw model to stable, pretty-printed JSON for golden comparison.
pub fn to_canonical_json(document: &RawDocument) -> Result<String, ParseError> {
    serde_json::to_string_pretty(document)
        .map_err(|err| ParseError::new(format!("raw model JSON serialization failed: {err}")))
}
