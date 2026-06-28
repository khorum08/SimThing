//! RF channel identity newtypes — owner, resource, and scope identifiers are not
//! interchangeable at the planet-child RF admission boundary.
//!
//! Transposition between channel identity slots is uncompilable:
//!
//! ```compile_fail
//! use simthing_spec::{OwnerRef, ResourceKey};
//!
//! fn takes_owner(_: OwnerRef) {}
//!
//! fn channel_owner_ref_rejects_resource_key_compile_fail(key: ResourceKey) {
//!     takes_owner(key);
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_spec::{OwnerRef, ResourceKey};
//!
//! fn takes_resource(_: ResourceKey) {}
//!
//! fn channel_resource_key_rejects_owner_ref_compile_fail(owner: OwnerRef) {
//!     takes_resource(owner);
//! }
//! ```

/// Metadata owner/channel reference after admission resolution.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerRef(String);

impl OwnerRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for OwnerRef {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Resource key within an owner RF channel after admission resolution.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceKey(String);

impl ResourceKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for ResourceKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Local RF scope identifier (e.g. planet id within a star-system arena).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeId(String);

impl ScopeId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for ScopeId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
