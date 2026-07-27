//! ORDER-WEIGHT-CLASS-0 — authored finite dominance class for operator directives.
//!
//! Orders are price injections (`OverlaySource::Player` overlays), never command
//! channels. Dominance magnitudes are typed data (this class), finite always.
//! Driver admission derives and proves the actual arena normalization envelope;
//! no self-authored ceiling or magic floor is trusted.

use super::script::PropertyKey;
use serde::{Deserialize, Serialize};
use simthing_core::SubFieldRole;

/// Authored finite order-weight class (P2 family (4)).
///
/// Declared as data — not a scattered literal. An operator directive that
/// claims dominance MUST reference a class id; the class magnitude is the
/// only sanctioned dominance band.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderWeightClassSpec {
    /// Stable class id (e.g. `"destination_order"`).
    pub id: String,
    /// Finite dominance magnitude. Driver install proves it is strictly
    /// greater than the bound arena's resolved ambient weight sum.
    pub magnitude: f32,
    /// Exact admitted Resource Flow arena binding. The class cannot be reused
    /// outside this normalization arena.
    pub arena: String,
    /// Exact property containing the arena's `AllocatorWeight` role.
    pub property: PropertyKey,
    /// Exact `AllocatorWeight` sub-field role.
    pub sub_field: SubFieldRole,
    /// Loader-derived scalar position for diagnostics; never authored.
    #[serde(skip)]
    pub source_span_token: Option<usize>,
}
