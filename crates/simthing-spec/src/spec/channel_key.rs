//! RF channel identity newtypes — owner, resource, scope, and parent-location identifiers are not
//! interchangeable at the RF admission/report boundary.
//!
//! Transposition between channel identity slots is uncompilable:
//!
//! ```compile_fail,E0308
//! use simthing_spec::{OwnerRef, ResourceKey};
//!
//! fn takes_owner(_: OwnerRef) {}
//!
//! fn channel_owner_ref_rejects_resource_key_compile_fail(key: ResourceKey) {
//!     takes_owner(key);
//! }
//! ```
//!
//! ```compile_fail,E0308
//! use simthing_core::SimThingId;
//! use simthing_spec::OwnerRef;
//!
//! fn takes_spatial_parent(_: SimThingId) {}
//!
//! fn owner_ref_rejects_spatial_parent_compile_fail(owner: OwnerRef) {
//!     takes_spatial_parent(owner);
//! }
//! ```
//!
//! ```compile_fail,E0308
//! use simthing_spec::{OwnerRef, ResourceKey};
//!
//! fn takes_resource(_: ResourceKey) {}
//!
//! fn channel_resource_key_rejects_owner_ref_compile_fail(owner: OwnerRef) {
//!     takes_resource(owner);
//! }
//! ```
//!
//! ```compile_fail,E0308
//! use simthing_spec::{ResourceKey, ScopeId};
//!
//! fn takes_resource(_: ResourceKey) {}
//!
//! fn channel_resource_key_rejects_scope_id_compile_fail(scope: ScopeId) {
//!     takes_resource(scope);
//! }
//! ```
//!
//! ```compile_fail,E0308
//! use simthing_spec::{ResourceKey, ScopeId};
//!
//! fn takes_scope(_: ScopeId) {}
//!
//! fn channel_scope_id_rejects_resource_key_compile_fail(key: ResourceKey) {
//!     takes_scope(key);
//! }
//! ```
//!
//! ```compile_fail,E0308
//! use simthing_spec::{OwnerRef, ParentLocationId};
//!
//! fn takes_owner(_: OwnerRef) {}
//!
//! fn channel_owner_ref_rejects_parent_location_compile_fail(loc: ParentLocationId) {
//!     takes_owner(loc);
//! }
//! ```
//!
//! ```compile_fail,E0308
//! use simthing_spec::{OwnerRef, ParentLocationId};
//!
//! fn takes_parent_location(_: ParentLocationId) {}
//!
//! fn channel_parent_location_rejects_owner_ref_compile_fail(owner: OwnerRef) {
//!     takes_parent_location(owner);
//! }
//! ```

/// Metadata owner/channel reference after admission resolution.
///
/// **Re-exported from `simthing-core`** (rung 6.0). Ownership is intrinsic to the stem
/// cell, so the type is homed alongside `resolve_owner` rather than in the spec layer;
/// keeping a second spec-local definition would reintroduce exactly the two-notion split
/// that OWNER-CHANNEL-INTRINSIC-0 exists to collapse. Consumers importing it from
/// `simthing_spec` are unaffected.
pub use simthing_core::owner_channel::OwnerRef;
use simthing_core::SimThingId;

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

    /// Canonical execution-scope identity for a boundary node.
    ///
    /// The opaque prefix deliberately carries no domain shape.  Ownership crossings,
    /// reduce-up buckets, and execution plans all use this same identity.
    pub fn from_boundary(boundary: SimThingId) -> Self {
        Self(format!("stead/{}", boundary.raw()))
    }
}

impl AsRef<str> for ScopeId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Canonical RF bucket identity.
///
/// Same-owner and different-owner aggregation is structural: ordered-map insertion by
/// this key decides it without an owner-equality control-flow branch.  `ScopeId` is the
/// ownership/execution boundary; domain-shaped coordinates do not belong in this type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerChannelScopeKey {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: ScopeId,
}

/// Parent location id for RF channel grouping (raw gridcell/location id axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParentLocationId(u32);

impl ParentLocationId {
    pub fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub fn raw(self) -> u32 {
        self.0
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod production_adoption {
    use std::fs;
    use std::path::Path;

    const SPEC_DIR: &str = "src/spec";

    const FORBIDDEN_FIELD_PATTERNS: &[&str] = &[
        "pub owner_ref: String",
        "pub owner_ref: Option<String>",
        "pub resource_key: String",
        "pub resource_key: Option<String>",
        "pub scope_id: String",
        "pub scope_id: Option<String>",
    ];

    /// Explicit serialization/output boundary deviations allowed to carry raw strings.
    const ALLOWED_DEVIATIONS: &[(&str, &str)] = &[];

    fn spec_rs_files() -> Vec<std::path::PathBuf> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(SPEC_DIR);
        fs::read_dir(&root)
            .expect("spec dir")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "rs"))
            .collect()
    }

    fn is_allowed_deviation(path: &Path, line: &str) -> bool {
        let file = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        ALLOWED_DEVIATIONS
            .iter()
            .any(|(allowed_file, pattern)| file == *allowed_file && line.contains(pattern))
    }
}
