//! Realm-qualified tree identity and the sealed execution-authority seam.
//!
//! Runtime authority is deliberately not serde data. A caller-owned
//! [`TreeExecutionAuthority`] borrows the one real root, generation authority,
//! schedule, registry, and residency attachment. It mints exactly one opaque
//! [`TreeExecutionContext`], whose private seal is checked against the live
//! authority record at every consuming door.

use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

use thiserror::Error;

use crate::{DimensionRegistry, GenerationStamp, IntegrationSchedule, SimThing, SimThingId};

/// Durable identity of one independently executing tree.
///
/// This runtime authority value intentionally has no serde implementation.
/// Durable products that carry a realm use their own validated wire form.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Deterministically derive a distinct realm for one semantic fork.
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
///
/// This authority value intentionally has no serde implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Caller-owned live generation authority for one executing tree.
///
/// Generation is transient execution state. It is intentionally absent from
/// semantic-plan identity and has no serde implementation.
#[derive(Debug)]
pub struct TreeGenerationAuthority {
    live: AtomicU32,
    execution_authority_minted: AtomicBool,
}

impl TreeGenerationAuthority {
    pub const fn new(initial: GenerationStamp) -> Self {
        Self {
            live: AtomicU32::new(initial.get()),
            execution_authority_minted: AtomicBool::new(false),
        }
    }

    pub fn current(&self) -> GenerationStamp {
        GenerationStamp::new(self.live.load(Ordering::Acquire))
    }

    fn mint_execution_authority(&self) -> Result<(), TreeExecutionContextError> {
        self.execution_authority_minted
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| TreeExecutionContextError::GenerationAuthorityAlreadySealed)?;
        Ok(())
    }

    /// Advance exactly N -> N+1 without rebuilding semantic state.
    pub fn advance(
        &self,
        next: GenerationStamp,
    ) -> Result<GenerationStamp, TreeExecutionContextError> {
        let current = self.live.load(Ordering::Acquire);
        let expected_next = current
            .checked_add(1)
            .ok_or(TreeExecutionContextError::GenerationOverflow)?;
        if next.get() != expected_next {
            return Err(TreeExecutionContextError::GenerationAdvanceOutOfSequence {
                current: GenerationStamp::new(current),
                requested: next,
            });
        }
        self.live
            .compare_exchange(current, next.get(), Ordering::AcqRel, Ordering::Acquire)
            .map_err(
                |observed| TreeExecutionContextError::GenerationAuthorityChanged {
                    expected: GenerationStamp::new(current),
                    observed: GenerationStamp::new(observed),
                },
            )?;
        Ok(next)
    }
}

/// A local semantic id qualified by its durable tree realm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RealmQualified<TLocalId> {
    realm: TreeRealmId,
    local: TLocalId,
}

impl<TLocalId> RealmQualified<TLocalId> {
    pub const fn new(realm: TreeRealmId, local: TLocalId) -> Self {
        Self { realm, local }
    }

    pub fn realm(&self) -> TreeRealmId {
        self.realm
    }

    pub const fn local(&self) -> &TLocalId {
        &self.local
    }

    pub fn into_local(self) -> TLocalId {
        self.local
    }
}

/// Source-record ordinal for one immutable seam emission.
///
/// There is deliberately no public raw constructor, `From<u32>`, serde path,
/// or conversion from a resident dictionary ordinal. The future source
/// emission recorder is the only component allowed to mint this value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeamEmissionOrdinal(u32);

impl SeamEmissionOrdinal {
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Retry identity of one fact crossing an independently executing tree seam.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeamFactId {
    source_realm: TreeRealmId,
    seam_id: u64,
    source_generation: GenerationStamp,
    source_ordinal: SeamEmissionOrdinal,
}

impl SeamFactId {
    /// Assemble an id from an ordinal already sealed by an immutable source
    /// emission record. This door cannot mint the ordinal itself.
    pub const fn from_recorded_emission(
        source_realm: TreeRealmId,
        seam_id: u64,
        source_generation: GenerationStamp,
        source_ordinal: SeamEmissionOrdinal,
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

    pub const fn source_ordinal(self) -> SeamEmissionOrdinal {
        self.source_ordinal
    }

    pub fn canonical_bytes(self) -> [u8; 32] {
        let mut bytes = [0_u8; 32];
        bytes[..16].copy_from_slice(&self.source_realm.canonical_bytes());
        bytes[16..24].copy_from_slice(&self.seam_id.to_le_bytes());
        bytes[24..28].copy_from_slice(&self.source_generation.get().to_le_bytes());
        bytes[28..32].copy_from_slice(&self.source_ordinal.get().to_le_bytes());
        bytes
    }
}

/// A canonical cross-tree fact carrying durable subject identity and the
/// source incarnation that produced it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeamFact<TLocalId> {
    id: SeamFactId,
    source_incarnation: ExecutionIncarnation,
    subject: RealmQualified<TLocalId>,
}

impl<TLocalId> SeamFact<TLocalId> {
    pub fn from_recorded_emission(
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

#[derive(Debug)]
struct TreeExecutionSeal {
    realm: TreeRealmId,
    root: SimThingId,
    live_incarnation: AtomicU64,
    context_minted: AtomicBool,
}

/// The one runtime authority capsule for an executing tree.
///
/// The capsule borrows, rather than duplicates, every existing authority. Its
/// private allocation identity is the context seal; equal raw ids or equal
/// generation values cannot cross-bind two capsules.
pub struct TreeExecutionAuthority<'a, TResidency> {
    seal: Arc<TreeExecutionSeal>,
    root: &'a SimThing,
    generation_authority: &'a TreeGenerationAuthority,
    schedule: &'a IntegrationSchedule,
    registry: &'a DimensionRegistry,
    residency: &'a TResidency,
}

impl<'a, TResidency> TreeExecutionAuthority<'a, TResidency> {
    #[allow(clippy::too_many_arguments)]
    pub fn seal(
        realm: TreeRealmId,
        incarnation: ExecutionIncarnation,
        root: &'a SimThing,
        generation_authority: &'a TreeGenerationAuthority,
        schedule: &'a IntegrationSchedule,
        registry: &'a DimensionRegistry,
        residency: &'a TResidency,
    ) -> Result<Self, TreeExecutionContextError> {
        if root.id.raw() == 0 {
            return Err(TreeExecutionContextError::ZeroRootId);
        }
        generation_authority.mint_execution_authority()?;
        Ok(Self {
            seal: Arc::new(TreeExecutionSeal {
                realm,
                root: root.id,
                live_incarnation: AtomicU64::new(incarnation.get()),
                context_minted: AtomicBool::new(false),
            }),
            root,
            generation_authority,
            schedule,
            registry,
            residency,
        })
    }

    /// Mint the sole context admitted by this runtime authority capsule.
    pub fn seal_context(&self) -> Result<TreeExecutionContext, TreeExecutionContextError> {
        self.seal
            .context_minted
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| TreeExecutionContextError::ContextAlreadyMinted)?;
        Ok(TreeExecutionContext {
            seal: Arc::clone(&self.seal),
            incarnation: self.live_incarnation(),
        })
    }

    /// Change the live incarnation and return the sole currently-valid context.
    /// Retained old contexts fail against the updated live record.
    pub fn migrate_context(
        &self,
        context: &TreeExecutionContext,
        new_incarnation: ExecutionIncarnation,
    ) -> Result<TreeExecutionContext, TreeExecutionContextError> {
        context.verify_authority(self)?;
        if new_incarnation == context.incarnation {
            return Err(TreeExecutionContextError::MigrationRequiresNewIncarnation);
        }
        self.seal
            .live_incarnation
            .compare_exchange(
                context.incarnation.get(),
                new_incarnation.get(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map_err(|observed| TreeExecutionContextError::StaleIncarnation {
                expected: incarnation_from_live(observed),
                observed: context.incarnation,
            })?;
        Ok(TreeExecutionContext {
            seal: Arc::clone(&self.seal),
            incarnation: new_incarnation,
        })
    }

    pub fn live_incarnation(&self) -> ExecutionIncarnation {
        incarnation_from_live(self.seal.live_incarnation.load(Ordering::Acquire))
    }

    pub fn current_generation(&self) -> GenerationStamp {
        self.generation_authority.current()
    }

    pub fn fork_realm(&self, fork_key: u64) -> Result<TreeRealmId, TreeIdentityError> {
        self.seal.realm.fork(fork_key)
    }
}

/// Opaque, non-cloneable authority handle sealed to exactly one live capsule.
///
/// There is no public constructor and no serde implementation. Incarnation is
/// a captured claim only until checked against the authority's live record.
///
/// ```compile_fail,E0277
/// fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}
/// requires_deserialize::<simthing_core::TreeRealmId>();
/// requires_deserialize::<simthing_core::ExecutionIncarnation>();
/// requires_deserialize::<simthing_core::TreeExecutionContext>();
/// ```
pub struct TreeExecutionContext {
    seal: Arc<TreeExecutionSeal>,
    incarnation: ExecutionIncarnation,
}

impl fmt::Debug for TreeExecutionContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TreeExecutionContext")
            .field("realm", &self.realm())
            .field("incarnation", &self.incarnation)
            .field("root", &self.root())
            .finish_non_exhaustive()
    }
}

impl TreeExecutionContext {
    pub fn realm(&self) -> TreeRealmId {
        self.seal.realm
    }

    pub const fn incarnation(&self) -> ExecutionIncarnation {
        self.incarnation
    }

    pub fn root(&self) -> SimThingId {
        self.seal.root
    }

    pub fn qualify<TLocalId>(&self, local: TLocalId) -> RealmQualified<TLocalId> {
        RealmQualified::new(self.seal.realm, local)
    }

    /// Stable semantic binding. Incarnation, generation, and the private
    /// runtime witness are deliberately excluded.
    pub fn semantic_plan_binding_bytes(&self) -> [u8; 20] {
        let mut bytes = [0_u8; 20];
        bytes[..16].copy_from_slice(&self.realm().canonical_bytes());
        bytes[16..].copy_from_slice(&self.root().raw().to_le_bytes());
        bytes
    }

    pub fn bind<'a, TResidency>(
        &'a self,
        authority: &'a TreeExecutionAuthority<'a, TResidency>,
    ) -> Result<TreeExecutionBinding<'a, TResidency>, TreeExecutionContextError> {
        self.verify_authority(authority)?;
        Ok(TreeExecutionBinding {
            context: self,
            authority,
        })
    }

    /// Reject facts from a foreign capsule or stale live incarnation.
    pub fn admit_seam_fact<TLocalId, TResidency>(
        &self,
        authority: &TreeExecutionAuthority<'_, TResidency>,
        fact: &SeamFact<TLocalId>,
    ) -> Result<(), TreeExecutionContextError> {
        self.verify_authority(authority)?;
        if fact.id().source_realm() != self.realm() || fact.subject().realm() != self.realm() {
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

    /// Actual destination-remap door: validate the source capsule and live
    /// incarnation before exposing only the realm-qualified subject to the
    /// destination's local mapping function.
    pub fn remap_seam_fact<TLocalId, TOutput, TResidency>(
        &self,
        authority: &TreeExecutionAuthority<'_, TResidency>,
        fact: &SeamFact<TLocalId>,
        remap: impl FnOnce(&RealmQualified<TLocalId>) -> TOutput,
    ) -> Result<TOutput, TreeExecutionContextError> {
        self.admit_seam_fact(authority, fact)?;
        Ok(remap(fact.subject()))
    }

    fn verify_authority<TResidency>(
        &self,
        authority: &TreeExecutionAuthority<'_, TResidency>,
    ) -> Result<(), TreeExecutionContextError> {
        if !Arc::ptr_eq(&self.seal, &authority.seal) {
            return Err(TreeExecutionContextError::AuthorityCapsuleMismatch);
        }
        let live = authority.live_incarnation();
        if live != self.incarnation {
            return Err(TreeExecutionContextError::StaleIncarnation {
                expected: live,
                observed: self.incarnation,
            });
        }
        Ok(())
    }
}

/// Freshly checked borrowing view of one runtime authority capsule.
pub struct TreeExecutionBinding<'a, TResidency> {
    context: &'a TreeExecutionContext,
    authority: &'a TreeExecutionAuthority<'a, TResidency>,
}

impl<'a, TResidency> TreeExecutionBinding<'a, TResidency> {
    pub fn validate(&self) -> Result<(), TreeExecutionContextError> {
        self.context.verify_authority(self.authority)
    }

    pub const fn context(&self) -> &'a TreeExecutionContext {
        self.context
    }

    pub const fn root(&self) -> &'a SimThing {
        self.authority.root
    }

    pub fn generation(&self) -> GenerationStamp {
        self.authority.generation_authority.current()
    }

    pub const fn generation_authority(&self) -> &'a TreeGenerationAuthority {
        self.authority.generation_authority
    }

    pub const fn schedule(&self) -> &'a IntegrationSchedule {
        self.authority.schedule
    }

    pub const fn registry(&self) -> &'a DimensionRegistry {
        self.authority.registry
    }

    pub const fn residency(&self) -> &'a TResidency {
        self.authority.residency
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
    #[error("tree execution root id must be non-zero")]
    ZeroRootId,
    #[error("this runtime authority capsule has already minted its context")]
    ContextAlreadyMinted,
    #[error("this tree generation authority has already minted its execution authority capsule")]
    GenerationAuthorityAlreadySealed,
    #[error("tree execution context belongs to a different runtime authority capsule")]
    AuthorityCapsuleMismatch,
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
    #[error("tree generation authority overflow")]
    GenerationOverflow,
    #[error("generation advance must be N -> N+1: current {current:?}, requested {requested:?}")]
    GenerationAdvanceOutOfSequence {
        current: GenerationStamp,
        requested: GenerationStamp,
    },
    #[error(
        "generation authority changed concurrently: expected {expected:?}, observed {observed:?}"
    )]
    GenerationAuthorityChanged {
        expected: GenerationStamp,
        observed: GenerationStamp,
    },
}

fn incarnation_from_live(value: u64) -> ExecutionIncarnation {
    ExecutionIncarnation::new(value).expect("sealed live incarnation is always non-zero")
}

fn stable_hash64(seed: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(seed, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimThingKind;

    #[test]
    fn old_context_and_fact_fail_through_actual_destination_remap_after_migration() {
        let tree = SimThing::new(SimThingKind::GameSession, 4);
        let generation = TreeGenerationAuthority::new(GenerationStamp::new(4));
        let schedule = IntegrationSchedule::new();
        let registry = DimensionRegistry::new();
        let residency = ();
        let authority = TreeExecutionAuthority::seal(
            TreeRealmId::from_u128(9).unwrap(),
            ExecutionIncarnation::new(3).unwrap(),
            &tree,
            &generation,
            &schedule,
            &registry,
            &residency,
        )
        .unwrap();
        let old_context = authority.seal_context().unwrap();

        // This private construction stands in only for an already-recorded
        // immutable source emission. No public raw mint exists in 14.2.
        let emission_ordinal = SeamEmissionOrdinal(0);
        let id = SeamFactId::from_recorded_emission(
            old_context.realm(),
            0xabc,
            generation.current(),
            emission_ordinal,
        );
        let fact = SeamFact::from_recorded_emission(
            id,
            old_context.incarnation(),
            old_context.qualify(tree.id),
        )
        .unwrap();
        assert_eq!(
            old_context
                .remap_seam_fact(&authority, &fact, |subject| *subject.local())
                .unwrap(),
            tree.id
        );

        let _new_context = authority
            .migrate_context(&old_context, old_context.incarnation().next().unwrap())
            .unwrap();

        // DA falsifier 2 (verbatim): after B migration, old-B-context +
        // old-B-fact fails through the actual destination-remap door.
        assert!(matches!(
            old_context.remap_seam_fact(&authority, &fact, |subject| *subject.local()),
            Err(TreeExecutionContextError::StaleIncarnation { .. })
        ));
    }
}
