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
    /// Explicit production execution opt-in for AccumulatorOp transfer/emission.
    ///
    /// Presence of a resource economy spec is not enough to enable execution;
    /// scenarios must choose one of the non-disabled modes.
    #[serde(default)]
    pub opt_in_mode: ResourceEconomyOptInMode,
    #[serde(default)]
    pub transfers: Vec<ResourceTransferSpec>,
    #[serde(default)]
    pub recipes: Vec<ResourceRecipeSpec>,
    #[serde(default)]
    pub emissions: Vec<ResourceEmissionSpec>,
    #[serde(default)]
    pub emit_on_threshold: Vec<EmitOnThresholdSpec>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResourceEconomyOptInMode {
    #[default]
    Disabled,
    TransferOnly,
    EmissionOnly,
    TransferAndEmission,
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
    /// Explicit entity host for the source property instance.
    #[serde(default)]
    pub source_host_entity: Option<String>,
    /// Explicit entity host for the target property instance.
    #[serde(default)]
    pub target_host_entity: Option<String>,
    /// Clause token for source host field (diagnostics; not serialised authority).
    #[serde(skip)]
    pub source_host_span_token: Option<usize>,
    #[serde(skip)]
    pub target_host_span_token: Option<usize>,
}

/// Conjunctive production recipe (E-3 authoring surface).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceRecipeSpec {
    pub id: String,
    pub inputs: Vec<RecipeInputSpec>,
    pub target: PropertyKey,
    pub target_role: SubFieldRole,
    /// Explicit entity host for the target property instance.
    #[serde(default)]
    pub target_host_entity: Option<String>,
    #[serde(skip)]
    pub target_host_span_token: Option<usize>,
    /// Authored target units credited per exact affordable recipe unit.
    /// This consumes the existing transfer planner's `output_scale`; it does
    /// not add a kernel/WGSL operation.
    pub output_coefficient: f32,
    /// Existing transfer OrderBand used to sequence generic recipe/coupling
    /// stages without introducing a second execution path.
    pub order_band: u32,
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
    /// Explicit entity host for this consumed property instance.
    #[serde(default)]
    pub host_entity: Option<String>,
    #[serde(skip)]
    pub host_span_token: Option<usize>,
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
    /// Explicit entity host for the property instance (install_targets key).
    /// Required for entity-hosted placement; never inferred from property names.
    #[serde(default)]
    pub host_entity: Option<String>,
    /// Clause token for host_entity field (diagnostics).
    #[serde(skip)]
    pub host_span_token: Option<usize>,
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
    /// Explicit entity host for the observed property instance.
    #[serde(default)]
    pub host_entity: Option<String>,
    /// Clause token for the host field (diagnostics only).
    #[serde(skip)]
    pub host_span_token: Option<usize>,
}

/// Which GPU buffer a threshold registration observes for crossing detection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EmitBufferSpec {
    #[default]
    Values,
    Output,
}
