//! RF-5 — generic need / weight_profile transport binding.
//!
//! Binds an existing hydrated `EmlGadgetStackSpec` (WeightedAccumulator) to an
//! admitted Arena participant cell. Installed through ordinary GameMode /
//! session-open; executed by existing EvalEML + Accumulator machinery.
//! No new ClauseScript syntax, kernel opcode, or WGSL.

use crate::spec::eml_gadget::EmlGadgetStackSpec;
use crate::spec::install_target::InstallTargetSpec;
use crate::spec::script::PropertyKey;
use serde::{Deserialize, Serialize};

/// One authored binding from a weight_profile EML stack to a live need cell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeedWeightProfileBindingSpec {
    pub id: String,
    /// Admitted profile kind (`expansion-need`, `manufacturing-need`, …).
    pub profile: String,
    /// Hydrated EML stack (must contain exactly one WeightedAccumulator).
    pub stack: EmlGadgetStackSpec,
    /// Arena name in `ResourceFlowSpec.arenas` whose participant hosts the cell.
    pub arena: String,
    /// Install target resolving to an admitted arena participant.
    pub install: InstallTargetSpec,
    /// Authored weight values, parallel to the stack's `weight_cols`.
    /// Empty is fail-closed at install (no silent default-weight of 1.0).
    pub weight_seeds: Vec<f32>,
    /// Input magnitudes, parallel to the stack's `input_cols`.
    pub inputs: Vec<NeedWeightProfileInputSpec>,
    /// Optional FIELD_POLICY threshold over the live need cell.
    #[serde(default)]
    pub threshold: Option<NeedWeightProfileThresholdSpec>,
}

/// One input to the WeightedAccumulator need formula.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NeedWeightProfileInputSpec {
    /// Constant authored pressure (paired-authoring control).
    Literal(f32),
    /// Live Amount sub-field of a registered property on the same participant.
    Property(PropertyKey),
}

/// Threshold + EmitEvent over the live need cell (sealed FIELD_POLICY ingress).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeedWeightProfileThresholdSpec {
    pub threshold: f32,
    pub event_kind: u32,
}
