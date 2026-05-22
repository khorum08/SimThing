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

pub use capability_definition::{
    CapabilityDefinition, CapabilityPrereq, CapabilityTreeDefinition,
    CapabilityTreeDefinitionId, CapabilityUnlockRegistration,
};
