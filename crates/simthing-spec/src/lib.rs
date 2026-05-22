//! RON → runtime compiler for authored SimThing game data.
//!
//! Capability trees are the first vertical slice. The simulation crates never
//! see tech-tree semantics — they receive native overlays, threshold
//! registrations, and boundary requests produced here.

pub mod boundary;
pub mod compile;
pub mod diagnostics;
pub mod error;
pub mod keys;
pub mod preview;
pub mod ron;
pub mod runtime;
pub mod spec;

pub use boundary::{
    CapabilityBoundaryContext, CapabilityBoundaryOutcome, CapabilityTreeBoundaryHandler,
    CapabilityUnlockEvent,
};
pub use compile::{CapabilityTreeBuildOutput, CapabilityTreeBuilder, set_overlay_affects};
pub use diagnostics::{
    CapabilityEntryKeyRef, CapabilityTreeDiagnostic, SpecDiagnostics, SpecResult, SpecWarning,
};
pub use error::CapabilityTreeError;
pub use keys::{
    CapabilityEffectKey, CapabilityEntryKey, CapabilityTreeDefinitionId, CapabilityTreeKey,
    CategoryKey,
};
pub use preview::{
    preview_capability_effect, CapabilityPreviewDelta, CapabilityPreviewInput,
    CapabilityPreviewOverlayBreakdown, CapabilityPreviewReport,
};
pub use ron::deserialize_capability_tree_ron;
pub use runtime::{
    CapabilityDefinition, CapabilityPrereq, CapabilityTreeDefinition, CapabilityTreeInstance,
    CapabilityTreeState, CategoryDefinition,
};
pub use spec::capability::{
    ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec,
    CapabilitySpec, CapabilityTreeSpec, MaxActivePolicy, ResearchRateSpec,
};
