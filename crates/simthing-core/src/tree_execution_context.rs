//! Realm-qualified tree identity and the checked execution-context seam.
//!
//! The canonical context is deliberately small and host-agnostic. Runtime
//! attachments are borrowed through [`TreeExecutionBinding`] and never enter
//! canonical bytes, persistence, or cross-tree identity.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{DimensionRegistry, GenerationStamp, IntegrationSchedule, SimThing, SimThingId};

/// Durable identity of one independently executing tree.
///
/// A realm survives migration. A speculative fork derives a new realm from
/// an explicit semantic fork key; no host/process allocator participates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TreeRealmId([u8; 16]);

impl TreeRealmId {
    pub fn from_bytes(bytes: [u8; 16]) -> Result<Self, TreeIdentityError> {
        if bytes == [0; 16] {
            return Err(TreeIdentityError::ZeroRealm);
        }
        Ok(Self(bytes))
    }

    pub fn from_u128(value: u128) -> Result<Self, TreeIdentityError> {
        Self::from_bytes(value.to_le_bytes())
    }

    pub const fn canonical_bytes(self) -> [u8; 16] {
        self.0
    }

    /// Deterministically mint a distinct realm for one semantic fork.
    pub fn fork(self, fork_key: u64) -> Result<Self, TreeIdentityError> {
        if fork_key == 0 {
            return Err(TreeIdentityError::ZeroForkKey);
        }
        let mut material = [0_u8; 24];
        material[..16].copy_from_slice(&self.0);
        material[16..].copy_from_slice(&fork_key.to_le_bytes());
        let left = stable_hash64(0xcbf2_9ce4_8422_2325, &material);
        let right = stable_hash64(0x8422_2325_cbf2_9ce4, &material);
        let mut bytes = [0_u8; 16];
        bytes[..8].copy_from_slice(&left.to_le_bytes());
        bytes[8..].copy_from_slice(&right.to_le_bytes());
        if bytes == [0; 16] || bytes == self.0 {
            bytes[0] ^= 0xa5;
        }
        Self::from_bytes(bytes)
    }
}

/// One transient execution incarnation of a durable realm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecutionIncarnation(u64);

impl ExecutionIncarnation {
    pub fn new(value: u64) -> Result<Self, TreeIdentityError> {
        if value == 0 {
            return Err(TreeIdentityError::ZeroIncarnation);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn next(self) -> Result<Self, TreeIdentityError> {
        self.0
            .checked_add(1)
            .ok_or(TreeIdentityError::IncarnationOverflow)
            .and_then(Self::new)
    }
}

/// A local semantic id qualified by its durable tree realm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RealmQualified<TLocalId> {
    realm: TreeRealmId,
    local: TLocalId,
}

impl<TLocalId> RealmQualified<TLocalId> {
    pub const fn new(realm: TreeRealmId, local: TLocalId) -> Self {
        Self { realm, local }
    }

    pub const fn realm(&self) -> TreeRealmId {
        self.realm
    }

    pub const fn local(&self) -> &TLocalId {
        &self.local
    }

    pub fn into_local(self) -> TLocalId {
        self.local
    }
}

/// Retry identity of one fact crossing an independently executing tree seam.
///
/// `source_ordinal` is source-local evidence only. Receiving plans never use
/// it as their destination ordinal; they remap the realm-qualified subject.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeamFactId {
    source_realm: TreeRealmId,
    seam_id: u64,
    source_generation: GenerationStamp,
    source_ordinal: u32,
}

impl SeamFactId {
    pub const fn new(
        source_realm: TreeRealmId,
        seam_id: u64,
        source_generation: GenerationStamp,
        source_ordinal: u32,
    ) -> Self {
        Self {
            source_realm,
            seam_id,
            source_generation,
            source_ordinal,
        }
    }

    pub const fn source_realm(self) -> TreeRealmId {
        self.source_realm
    }

    pub const fn seam_id(self) -> u64 {
        self.seam_id
    }

    pub const fn source_generation(self) -> GenerationStamp {
        self.source_generation
    }

    pub const fn source_ordinal(self) -> u32 {
        self.source_ordinal
    }

    pub fn canonical_bytes(self) -> [u8; 32] {
        let mut bytes = [0_u8; 32];
        bytes[..16].copy_from_slice(&self.source_realm.canonical_bytes());
        bytes[16..24].copy_from_slice(&self.seam_id.to_le_bytes());
        bytes[24..28].copy_from_slice(&self.source_generation.get().to_le_bytes());
        bytes[28..32].copy_from_slice(&self.source_ordinal.to_le_bytes());
        bytes
    }
}

/// A canonical cross-tree fact carrying durable subject identity and the
/// source incarnation that produced it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeamFact<TLocalId> {
    id: SeamFactId,
    source_incarnation: ExecutionIncarnation,
    subject: RealmQualified<TLocalId>,
}

impl<TLocalId> SeamFact<TLocalId> {
    pub fn new(
        id: SeamFactId,
        source_incarnation: ExecutionIncarnation,
        subject: RealmQualified<TLocalId>,
    ) -> Result<Self, TreeExecutionContextError> {
        if id.source_realm() != subject.realm() {
            return Err(TreeExecutionContextError::FactSubjectRealmMismatch);
        }
        Ok(Self {
            id,
            source_incarnation,
            subject,
        })
    }

    pub const fn id(&self) -> SeamFactId {
        self.id
    }

    pub const fn source_incarnation(&self) -> ExecutionIncarnation {
        self.source_incarnation
    }

    pub const fn subject(&self) -> &RealmQualified<TLocalId> {
        &self.subject
    }
}

/// Canonical, O(1) identity of one tree execution context.
///
/// Schedule, registry, residency, device, queue, addresses, and physical rows
/// are intentionally absent. [`Self::bind`] borrows those existing runtime
/// authorities without cloning or serializing them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeExecutionContext {
    realm: TreeRealmId,
    incarnation: ExecutionIncarnation,
    root: SimThingId,
    generation: GenerationStamp,
}

impl TreeExecutionContext {
    pub const fn new(
        realm: TreeRealmId,
        incarnation: ExecutionIncarnation,
        root: SimThingId,
        generation: GenerationStamp,
    ) -> Self {
        Self {
            realm,
            incarnation,
            root,
            generation,
        }
    }

    pub const fn realm(self) -> TreeRealmId {
        self.realm
    }

    pub const fn incarnation(self) -> ExecutionIncarnation {
        self.incarnation
    }

    pub const fn root(self) -> SimThingId {
        self.root
    }

    pub const fn generation(self) -> GenerationStamp {
        self.generation
    }

    pub const fn qualify<TLocalId>(self, local: TLocalId) -> RealmQualified<TLocalId> {
        RealmQualified::new(self.realm, local)
    }

    /// Canonical context bytes. No transient attachment or host coordinate is
    /// representable in this fixed-width form.
    pub fn canonical_bytes(self) -> [u8; 32] {
        let mut bytes = [0_u8; 32];
        bytes[..16].copy_from_slice(&self.realm.canonical_bytes());
        bytes[16..24].copy_from_slice(&self.incarnation.get().to_le_bytes());
        bytes[24..28].copy_from_slice(&self.root.raw().to_le_bytes());
        bytes[28..32].copy_from_slice(&self.generation.get().to_le_bytes());
        bytes
    }

    /// Stable semantic plan binding. Incarnation is deliberately excluded so
    /// migration recreation preserves semantic plan bytes and digest.
    pub fn semantic_plan_binding_bytes(self) -> [u8; 24] {
        let mut bytes = [0_u8; 24];
        bytes[..16].copy_from_slice(&self.realm.canonical_bytes());
        bytes[16..20].copy_from_slice(&self.root.raw().to_le_bytes());
        bytes[20..24].copy_from_slice(&self.generation.get().to_le_bytes());
        bytes
    }

    pub fn migrate(
        self,
        new_incarnation: ExecutionIncarnation,
    ) -> Result<Self, TreeExecutionContextError> {
        if new_incarnation == self.incarnation {
            return Err(TreeExecutionContextError::MigrationRequiresNewIncarnation);
        }
        Ok(Self {
            incarnation: new_incarnation,
            ..self
        })
    }

    pub fn fork(
        self,
        fork_key: u64,
        fork_incarnation: ExecutionIncarnation,
    ) -> Result<Self, TreeIdentityError> {
        Ok(Self {
            realm: self.realm.fork(fork_key)?,
            incarnation: fork_incarnation,
            ..self
        })
    }

    pub const fn at_generation(self, generation: GenerationStamp) -> Self {
        Self { generation, ..self }
    }

    /// Reject a fact from a stale execution incarnation. Generation lag is
    /// not rejected here: async staleness remains governed by the existing
    /// authored seam policy.
    pub fn admit_seam_fact<TLocalId>(
        self,
        fact: &SeamFact<TLocalId>,
    ) -> Result<(), TreeExecutionContextError> {
        if fact.id().source_realm() != self.realm || fact.subject().realm() != self.realm {
            return Err(TreeExecutionContextError::ForeignSourceRealm);
        }
        if fact.source_incarnation() != self.incarnation {
            return Err(TreeExecutionContextError::StaleIncarnation {
                expected: self.incarnation,
                observed: fact.source_incarnation(),
            });
        }
        Ok(())
    }

    /// Borrow the one existing runtime root, generation authority, schedule,
    /// registry, and residency attachment named by this context.
    pub fn bind<'a, TResidency>(
        self,
        root: &'a SimThing,
        generation_authority: &'a GenerationStamp,
        schedule: &'a IntegrationSchedule,
        registry: &'a DimensionRegistry,
        residency: &'a TResidency,
    ) -> Result<TreeExecutionBinding<'a, TResidency>, TreeExecutionContextError> {
        if root.id != self.root {
            return Err(TreeExecutionContextError::RootMismatch {
                expected: self.root,
                observed: root.id,
            });
        }
        if *generation_authority != self.generation {
            return Err(TreeExecutionContextError::GenerationAuthorityMismatch {
                expected: self.generation,
                observed: *generation_authority,
            });
        }
        Ok(TreeExecutionBinding {
            context: self,
            root,
            generation_authority,
            schedule,
            registry,
            residency,
        })
    }
}

/// Transient checked attachment of canonical tree identity to existing
/// caller-owned runtime authorities.
pub struct TreeExecutionBinding<'a, TResidency> {
    context: TreeExecutionContext,
    root: &'a SimThing,
    generation_authority: &'a GenerationStamp,
    schedule: &'a IntegrationSchedule,
    registry: &'a DimensionRegistry,
    residency: &'a TResidency,
}

impl<'a, TResidency> TreeExecutionBinding<'a, TResidency> {
    pub const fn context(&self) -> TreeExecutionContext {
        self.context
    }

    pub const fn root(&self) -> &'a SimThing {
        self.root
    }

    pub const fn generation_authority(&self) -> &'a GenerationStamp {
        self.generation_authority
    }

    pub const fn schedule(&self) -> &'a IntegrationSchedule {
        self.schedule
    }

    pub const fn registry(&self) -> &'a DimensionRegistry {
        self.registry
    }

    pub const fn residency(&self) -> &'a TResidency {
        self.residency
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum TreeIdentityError {
    #[error("tree realm id must be non-zero")]
    ZeroRealm,
    #[error("execution incarnation must be non-zero")]
    ZeroIncarnation,
    #[error("execution incarnation overflow")]
    IncarnationOverflow,
    #[error("tree fork key must be non-zero")]
    ZeroForkKey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum TreeExecutionContextError {
    #[error("tree execution root mismatch: expected {expected:?}, observed {observed:?}")]
    RootMismatch {
        expected: SimThingId,
        observed: SimThingId,
    },
    #[error("tree generation authority mismatch: expected {expected:?}, observed {observed:?}")]
    GenerationAuthorityMismatch {
        expected: GenerationStamp,
        observed: GenerationStamp,
    },
    #[error("migration must change execution incarnation")]
    MigrationRequiresNewIncarnation,
    #[error("seam fact subject realm does not match its source realm")]
    FactSubjectRealmMismatch,
    #[error("seam fact belongs to a foreign source realm")]
    ForeignSourceRealm,
    #[error("stale execution incarnation: expected {expected:?}, observed {observed:?}")]
    StaleIncarnation {
        expected: ExecutionIncarnation,
        observed: ExecutionIncarnation,
    },
}

fn stable_hash64(seed: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(seed, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}
