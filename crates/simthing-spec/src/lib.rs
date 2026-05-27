//! Authored SimThing specification layer.
//!
//! This crate compiles external RON-facing game data into native SimThing
//! runtime artifacts. It owns authored schemas, validation, diagnostics,
//! logical keys, compile-time conversion, runtime definition types, Script IR,
//! event/trigger/effect templates, boundary handlers, and impact preview.
//!
//! It does **not** execute the simulation, own GPU state, or orchestrate the
//! day-boundary protocol. The driver (`simthing-driver`) installs compiled
//! artifacts into `SpecSessionState` and invokes boundary handlers through
//! a generic sim-side hook after GPU value readback.
//!
//! ## Scope (PRs 1–11)
//!
//! - **Authoring structs** (`spec::*`): properties, overlays, capability trees,
//!   events, triggers, effects, Script IR.
//! - **Compilers** (`compile::*`): `compile_property`, `compile_overlay`,
//!   `CapabilityTreeBuilder`, `compile_event`, trigger/effect compilers.
//! - **Runtime artifacts** (`runtime::*`): `CapabilityTreeDefinition`,
//!   capability/session state types, `ScriptedEventDefinition`.
//! - **Boundary handlers** (`boundary::*`): capability unlock / player
//!   selection, scripted-event predicate + threshold dispatch (called by the
//!   driver hook — not embedded in `simthing-sim::BoundaryProtocol`).
//! - **Preview** (`preview::*`): `preview_capability_effect`.
//! - **RON loaders**, validation, diagnostics, logical keys.
//!
//! ## Out of scope / deferred
//!
//! - RON-driven session open from `GameModeSpec` (manual `install_spec_state`
//!   today — see progress log § Open work O1).
//! - Replay serialization of spec runtime state (O2).
//! - B2 append-only integration for external capability/scripted threshold
//!   registrations on growth boundaries (helpers exist; wiring deferred).
//! - EML backend, Studio GUI, full scenario RON expansion.
//!
//! ## Crate boundary
//!
//! Production code depends on `simthing-core` and `simthing-feeder` only.
//! Integration tests may use `simthing-gpu` / `simthing-sim` as dev-dependencies.
//! Fired GPU threshold events are resolved by the caller via
//! `ThresholdRegistry::extract_capability_unlocks` / `extract_scripted_event_triggers`
//! before reaching spec boundary handlers.

pub mod boundary;
pub mod compile;
pub mod diagnostics;
pub mod error;
pub mod keys;
pub mod metadata;
pub mod preview;
pub mod ron;
pub mod runtime;
pub mod spec;
pub mod validate;
pub mod version;

pub use boundary::{
    CapabilityBoundaryContext, CapabilityTreeBoundaryHandler, CapabilityTreeError,
    ScriptedEventBoundaryContext, ScriptedEventBoundaryHandler, ScriptedEventDiagnostic,
    ScriptedEventDiagnosticKind,
};
pub use compile::{
    compile_effect, compile_event, compile_overlay, compile_property, compile_resource_economy,
    compile_resource_flow_admission, compile_trigger, CapabilityTreeBuildOutput,
    CapabilityTreeBuilder, CompileContext, CompiledArenaAdmission, CompiledCouplingAdmission,
    CompiledCouplingDelay, CompiledEmissionFormula, CompiledEmitOnThreshold,
    CompiledResourceEconomy, CompiledResourceEmission, CompiledResourceFlowAdmission,
    CompiledResourceRecipe, CompiledResourceRecipeInput, CompiledResourceTransfer,
    ResourceEconomyDiagnostic, ResourceEconomyExpansionReport, ResourceFlowDiagnostic,
    ResourceFlowExpansionReport,
};
pub use diagnostics::{DiagnosticSeverity, SpecDiagnostic, SpecDiagnostics, SpecResult};
pub use error::SpecError;
pub use keys::{CapabilityEffectKey, CapabilityEntryKey, CapabilityTreeKey, CategoryKey};
pub use metadata::DisplayMeta;
pub use preview::{
    preview_capability_effect, CapabilityPreviewDelta, CapabilityPreviewInput,
    CapabilityPreviewOverlayBreakdown, CapabilityPreviewReport,
};
pub use ron::{deserialize_capability_tree_ron, deserialize_game_mode_ron};
pub use runtime::{
    CapabilityCategoryDefinition, CapabilityDefinition, CapabilityPrereq, CapabilityTreeDefinition,
    CapabilityTreeDefinitionId, CapabilityTreeDiagnostic, CapabilityTreeInstance,
    CapabilityTreeNotification, CapabilityTreeState, CapabilityUnlockRegistration, CompiledEffect,
    CompiledThresholdTrigger, CompiledTrigger, ScriptedEventDefinition, ScriptedEventDefinitionId,
    ScriptedEventInstance, ScriptedEventInstanceKey,
};
pub use spec::capability::{
    ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec,
    CapabilitySpec, CapabilityTreeSpec, EffectTarget, MaxActivePolicy, ReplacementPolicy,
};
pub use spec::domain_pack::DomainPackSpec;
pub use spec::effect::EffectSpec;
pub use spec::event::{CooldownSpec, EventKey, EventPriority, EventSpec};
pub use spec::game_mode::GameModeSpec;
pub use spec::install_target::InstallTargetSpec;
pub use spec::overlay::OverlaySpec;
pub use spec::property::PropertySpec;
pub use spec::resource_economy::{
    EmissionFormulaSpec, EmitBufferSpec, EmitOnThresholdSpec, RecipeInputSpec,
    ResourceEconomyOptInMode, ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec,
    ResourceTransferSpec,
};
pub use spec::resource_flow::{
    ArenaSpec, CouplingDelaySpec, CouplingSpec, ExplicitParticipantSpec, FissionPolicySpec,
    ResourceFlowSpec, WildcardAdmissionSpec,
};
pub use spec::script::{
    PropertyKey, ScopeRef, ScriptEvalContext, ScriptEvalError, ScriptExpr, ScriptPredicate,
};
pub use spec::trigger::{TriggerDirection, TriggerSpec};
pub use validate::validate_capability_tree;
pub use version::SpecVersion;
