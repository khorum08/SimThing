//! Runtime artifacts produced by the spec compilers.
//!
//! These types are the live counterpart of authored spec structs. They are
//! consumed by:
//! - the session coordinator (faction-instance allocation, state owner)
//! - the capability boundary handler (PR 5)
//! - the impact preview routine (PR 6)
//!
//! Built by [`crate::compile::CapabilityTreeBuilder`].

pub mod capability_definition;
pub mod capability_state;

pub use capability_definition::{
    CapabilityCategoryDefinition, CapabilityDefinition, CapabilityPrereq, CapabilityTreeDefinition,
    CapabilityTreeDefinitionId, CapabilityUnlockRegistration,
};
pub use capability_state::{
    CapabilityTreeDiagnostic, CapabilityTreeInstance, CapabilityTreeNotification,
    CapabilityTreeState,
};
