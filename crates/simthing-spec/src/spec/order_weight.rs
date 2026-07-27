//! ORDER-WEIGHT-CLASS-0 — authored finite dominance class for operator directives.
//!
//! Orders are price injections (`OverlaySource::Player` overlays), never command
//! channels. Dominance magnitudes are typed data (this class), finite always.

use serde::{Deserialize, Serialize};

/// Authored finite order-weight class (P2 family (4)).
///
/// Declared as data — not a scattered literal. An operator directive that
/// claims dominance MUST reference a class id; the class magnitude is the
/// only sanctioned dominance band.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderWeightClassSpec {
    /// Stable class id (e.g. `"destination_order"`).
    pub id: String,
    /// Finite dominance magnitude (e.g. `10000.0`). Must be finite and > 0.
    pub magnitude: f32,
    /// Optional source span from a spanned authoring frontend.
    #[serde(default)]
    pub source_span_token: Option<usize>,
}
