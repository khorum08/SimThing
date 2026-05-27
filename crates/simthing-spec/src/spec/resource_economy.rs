//! Authored production transfer / recipe / emission / threshold-emit registrations (Phase T-1).
//!
//! Sibling to [`super::resource_flow::ResourceFlowSpec`]: discrete hard-currency and
//! event-shaped registrations are authored here, not under continuous Resource Flow.

use crate::spec::script::PropertyKey;
use crate::spec::trigger::TriggerDirection;
use serde::{Deserialize, Serialize};
use simthing_core::SubFieldRole;

/// Top-level authored economic registration content for a game mode.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceEconomySpec {
    #[serde(default)]
    pub transfers: Vec<ResourceTransferSpec>,
    #[serde(default)]
    pub recipes: Vec<ResourceRecipeSpec>,
    #[serde(default)]
    pub emissions: Vec<ResourceEmissionSpec>,
    #[serde(default)]
    pub emit_on_threshold: Vec<EmitOnThresholdSpec>,
}

/// Exact discrete source-debit transfer (E-2A authoring surface).
///
/// The compile pass maps this to `DiscreteTransferRegistration` with fixed
/// `ConsumeMode::SubtractFromSource`. Do not add rate, probability, consume
/// mode, or Resource Flow roles at this layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceTransferSpec {
    /// Stable authoring identity for replay and `reg_idx` assignment (T-3).
    pub id: String,
    pub source: PropertyKey,
    pub source_role: SubFieldRole,
    pub target: PropertyKey,
    pub target_role: SubFieldRole,
    pub amount: f32,
    /// OrderBand gate identity for the compiled AccumulatorOp.
    pub order_band: u32,
}

/// Conjunctive production recipe (E-3 authoring surface).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceRecipeSpec {
    pub id: String,
    pub inputs: Vec<RecipeInputSpec>,
    pub target: PropertyKey,
    pub target_role: SubFieldRole,
    /// Boundary/throttle metadata only (E-3R). Not an enforced GPU or CPU cap.
    pub throttle_hint_max_per_tick: u32,
}

/// One conjunctive recipe input channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecipeInputSpec {
    pub property: PropertyKey,
    pub role: SubFieldRole,
    pub unit_cost: f32,
}

/// Event-shaped emission registration (C-8d authoring surface).
///
/// `max_emit` is intentionally absent until a GPU clamp is designed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceEmissionSpec {
    pub id: String,
    pub source: PropertyKey,
    pub source_role: SubFieldRole,
    pub formula: EmissionFormulaSpec,
}

/// Landed emission formula shapes only (`ExactDeterministic` admission at compile time).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub enum EmissionFormulaSpec {
    IdentityFloor,
    Constant(f32),
    EvalEml {
        /// Stable designer formula key; resolved to `EmlTreeId` at compile time (T-2).
        formula_key: String,
    },
}

/// Threshold-crossing emit registration (E-1 authoring surface).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmitOnThresholdSpec {
    pub id: String,
    pub source: PropertyKey,
    pub source_role: SubFieldRole,
    pub threshold: f32,
    pub direction: TriggerDirection,
    pub event_kind: u32,
    #[serde(default)]
    pub buffer: EmitBufferSpec,
}

/// Which GPU buffer a threshold registration observes for crossing detection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EmitBufferSpec {
    #[default]
    Values,
    Output,
}
