//! ORDER-WEIGHT-CLASS-0 — authored finite dominance class for operator directives.
//!
//! Orders are price injections (`OverlaySource::Player` overlays), never command
//! channels. Dominance magnitudes are typed data (this class), finite always.
//! The class magnitude must dominate an authored ambient price envelope under
//! ordinary arena weight normalization — never a magic floor.

use serde::{Deserialize, Serialize};

/// Authored finite order-weight class (P2 family (4)).
///
/// Declared as data — not a scattered literal. An operator directive that
/// claims dominance MUST reference a class id; the class magnitude is the
/// only sanctioned dominance band, and it must exceed `ambient_ceiling`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderWeightClassSpec {
    /// Stable class id (e.g. `"destination_order"`).
    pub id: String,
    /// Finite dominance magnitude. Must be finite, > 0, and strictly greater
    /// than `ambient_ceiling` so the class dominates ambient under ordinary
    /// proportional weight allocation (same-arena normalization).
    pub magnitude: f32,
    /// Authored ambient price envelope this class must dominate. Derived from
    /// arena ambient weights / need prices — not a scattered magic floor.
    pub ambient_ceiling: f32,
    /// Optional source span from a spanned authoring frontend.
    #[serde(default)]
    pub source_span_token: Option<usize>,
}
