//! RF-5 — generic need / weight_profile transport binding.
//!
//! Binds a hydrated WeightedAccumulator EML stack to an **existing** Arena
//! participant `AllocatorWeight` cell. Inputs and weights are live Amount
//! columns of **existing** properties (overlay-modifiable). No synthetic need
//! host property, no Studio-authored seeds, no presentation arithmetic.
//!
//! Thresholds lower onto the existing `emit_on_threshold` / AccumulatorOp
//! sealed-event path (same substrate as field-economy disruption).

use crate::spec::eml_gadget::EmlGadgetStackSpec;
use crate::spec::install_target::InstallTargetSpec;
use crate::spec::script::PropertyKey;
use serde::{Deserialize, Serialize};

/// One authored binding from a weight_profile EML stack to an existing Arena cell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeedWeightProfileBindingSpec {
    pub id: String,
    /// Admitted profile kind (`expansion-need`, `manufacturing-need`, …).
    pub profile: String,
    /// Hydrated EML stack (exactly one WeightedAccumulator; column indices are
    /// structural only — live columns come from the property lists below).
    pub stack: EmlGadgetStackSpec,
    /// Arena name in `ResourceFlowSpec.arenas`.
    pub arena: String,
    /// Install target resolving to an admitted arena participant.
    pub install: InstallTargetSpec,
    /// Live Amount columns used as WeightedAccumulator inputs (parallel to stack inputs).
    pub input_properties: Vec<PropertyKey>,
    /// Live Amount columns used as weights (parallel to stack weights).
    /// Overlay-modifiable — paired authorings differ here with zero code change.
    pub weight_properties: Vec<PropertyKey>,
    /// Optional sealed FIELD_POLICY threshold over the AllocatorWeight need cell.
    #[serde(default)]
    pub threshold: Option<NeedWeightProfileThresholdSpec>,
}

/// Threshold + EmitEvent over the live AllocatorWeight need cell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeedWeightProfileThresholdSpec {
    pub threshold: f32,
    pub event_kind: u32,
}
