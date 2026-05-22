//! Authored SimThing specification layer.
//!
//! This crate owns external RON-facing schemas, validation, diagnostics,
//! logical keys, and future compilation into live SimThing runtime artifacts.
//!
//! PR 1 intentionally contains authoring data structures only. Runtime builders,
//! boundary handlers, threshold plumbing, Script IR, and Studio integration
//! land in later PRs.
//!
//! ## PR 1 non-goals
//!
//! PR 1 does not:
//! - register `SimProperty` entries in a live `DimensionRegistry`
//! - build `Overlay` instances or attach them to `SimThing` nodes
//! - create live `SimThing` trees
//! - emit `BoundaryRequest` values
//! - integrate with `ThresholdBuilder` or GPU threshold registration
//! - mutate CPU shadow buffers
//! - create capability runtime state (`OnPrereqMet`, active-by-category, etc.)
//! - execute capability unlocks at boundary time
//! - implement Script IR, EML, scripted events, or effect/trigger compilers

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

pub use boundary::{CapabilityBoundaryContext, CapabilityTreeBoundaryHandler, CapabilityTreeError};
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
