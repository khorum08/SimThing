//! Authored SimThing specification layer.
//!
//! This crate owns external RON-facing schemas, validation, diagnostics,
//! logical keys, and compilation of spec structures into live SimThing runtime
//! artifacts.
//!
//! ## What is here (PRs 1–8)
//!
//! - **Authoring structs** (`spec::*`): `PropertySpec`, `OverlaySpec`,
//!   `CapabilityTreeSpec`, `EventSpec`, `TriggerSpec`, `EffectSpec`, Script IR.
//! - **Compilers** (`compile::*`): `compile_property`, `compile_overlay`,
//!   `compile_trigger`, `compile_effect`, `compile_event`,
//!   `CapabilityTreeBuilder`.
//! - **Runtime artifacts** (`runtime::*`): `CapabilityTreeDefinition`,
//!   `CapabilityTreeInstance`, `CapabilityTreeState`,
//!   `ScriptedEventDefinition`, `CompiledTrigger`, `CompiledEffect`.
//! - **Boundary handlers** (`boundary::*`): `CapabilityTreeBoundaryHandler`
//!   (threshold activation, prereq reset, fixpoint sweeps, player selection).
//! - **Impact preview** (`preview::*`): `CapabilityPreviewReport`.
//! - **RON loaders**, validation, diagnostics, logical keys.
//!
//! ## Deferred
//!
//! - Session/driver assembly for capability tree instances and per-faction
//!   state maps.
//! - B2 append-only capability unlock integration.
//! - B2 append-only integration for scripted-event triggers (the PR 10 path
//!   is full-rebuild only).
//!
//! ## Crate boundary
//!
//! Production code depends on `simthing-core` and `simthing-feeder` only.
//! Integration tests in `tests/` may pull `simthing-gpu` / `simthing-sim` as
//! dev-dependencies to exercise end-to-end paths. Fired GPU threshold events
//! are resolved into [`simthing_feeder::CapabilityUnlockEvent`]s by the caller
//! (via `ThresholdRegistry::extract_capability_unlocks` in `simthing-sim`)
//! before reaching `CapabilityTreeBoundaryHandler::handle_capability_unlock_events`.

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
    compile_effect, compile_event, compile_overlay, compile_property, compile_trigger,
    CapabilityTreeBuildOutput, CapabilityTreeBuilder, CompileContext,
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
    CompiledThresholdTrigger, CompiledTrigger, ScriptedEventDefinition,
};
pub use spec::capability::{
    ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec,
    CapabilitySpec, CapabilityTreeSpec, MaxActivePolicy, ReplacementPolicy, ResearchRateSpec,
};
pub use spec::domain_pack::DomainPackSpec;
pub use spec::effect::EffectSpec;
pub use spec::event::{CooldownSpec, EventKey, EventPriority, EventSpec};
pub use spec::game_mode::GameModeSpec;
pub use spec::overlay::OverlaySpec;
pub use spec::property::PropertySpec;
pub use spec::script::{
    PropertyKey, ScopeRef, ScriptEvalContext, ScriptEvalError, ScriptExpr, ScriptPredicate,
};
pub use spec::trigger::{TriggerDirection, TriggerSpec};
pub use validate::validate_capability_tree;
pub use version::SpecVersion;
