//! RF-5A — semantic need_binding authoring (full-cell resolve at admission).
//!
//! Clause names **identity** `(entity, property, role)`, never raw slots/columns.
//! Session admission resolves each source to exactly one `(SlotIndex, ColumnIndex)`.

use crate::spec::eml_gadget::EmlGadgetStackSpec;
use crate::spec::script::PropertyKey;
use serde::{Deserialize, Serialize};
use simthing_core::SubFieldRole;

/// One authored full-cell source: named entity + property key + role.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SemanticPropertyLocusSpec {
    /// Scenario entity id (`owner` / `location` header id).
    pub entity: String,
    pub property: PropertyKey,
    pub role: SubFieldRole,
}

/// Authored need binding (semantic). Stack may be filled from profile-id join.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeedBindingSpec {
    /// Join key to field_economy `weight_profile` id (and binding identity).
    pub id: String,
    /// Admitted profile kind (`expansion-need`, …).
    pub profile: String,
    /// Named entity that must already be an admitted Arena participant.
    pub participant: String,
    /// Arena name in `ResourceFlowSpec.arenas`.
    pub arena: String,
    /// WeightedAccumulator stack (from weight_profile join or explicit).
    pub stack: EmlGadgetStackSpec,
    pub inputs: Vec<SemanticPropertyLocusSpec>,
    pub weights: Vec<SemanticPropertyLocusSpec>,
    #[serde(default)]
    pub threshold: Option<f32>,
    #[serde(default)]
    pub event_kind: Option<u32>,
}
