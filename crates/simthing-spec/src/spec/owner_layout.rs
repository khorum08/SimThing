//! Session-local persistent RF layout keys (rung 9.2).
//!
//! Layout identity is interned owner id + logical [`SlotIndex`] + resource/scope.
//! It is never `OwnerRef` lexical order, physical row, or allocation order.

use simthing_core::owner_channel::{OwnerInterner, OwnerLayoutId};
use simthing_core::SlotIndex;

use super::channel_key::{OwnerChannelScopeKey, ResourceKey, ScopeId};
use super::owner_channel_rf::OwnerChannelRfBucket;
use simthing_core::owner_channel::OwnerRef;

/// Persistent RF layout key. Session-local; not seam-serializable.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersistentRfLayoutKey {
    pub owner: OwnerLayoutId,
    pub logical_slot: SlotIndex,
    pub resource_key: ResourceKey,
    pub scope_id: ScopeId,
}

/// Session-local intern + dense layout index assignment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentRfLayout {
    pub interner: OwnerInterner,
    keys: Vec<PersistentRfLayoutKey>,
}

impl PersistentRfLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern_owner(&mut self, owner: &OwnerRef) -> OwnerLayoutId {
        self.interner.intern(owner)
    }

    /// Assign dense layout indices by interned owner, then resource, then scope, then logical slot.
    pub fn assign_from_buckets(
        &mut self,
        buckets: &[OwnerChannelRfBucket],
        logical_slots: &[SlotIndex],
    ) {
        let mut keys = Vec::new();
        for (bucket, slot) in buckets.iter().zip(logical_slots.iter()) {
            let owner = self.interner.intern(&bucket.scope.owner_ref);
            keys.push(PersistentRfLayoutKey {
                owner,
                logical_slot: *slot,
                resource_key: bucket.scope.resource_key.clone(),
                scope_id: bucket.scope.scope_id.clone(),
            });
        }
        keys.sort();
        self.keys = keys;
    }

    pub fn index_of(&self, key: &PersistentRfLayoutKey) -> Option<u32> {
        self.keys.binary_search(key).ok().map(|i| i as u32)
    }

    pub fn keys(&self) -> &[PersistentRfLayoutKey] {
        &self.keys
    }

    pub fn layout_key_for_scope(
        &self,
        scope: &OwnerChannelScopeKey,
        logical_slot: SlotIndex,
    ) -> Option<PersistentRfLayoutKey> {
        let owner = self.interner.id_of(&scope.owner_ref)?;
        Some(PersistentRfLayoutKey {
            owner,
            logical_slot,
            resource_key: scope.resource_key.clone(),
            scope_id: scope.scope_id.clone(),
        })
    }

    /// Intern-order of semantic scopes for GPU/layout plans.
    pub fn sort_scopes<'a>(
        &self,
        scopes: &'a [OwnerChannelScopeKey],
    ) -> Vec<&'a OwnerChannelScopeKey> {
        let mut indexed: Vec<(OwnerLayoutId, &OwnerChannelScopeKey)> = scopes
            .iter()
            .filter_map(|scope| self.interner.id_of(&scope.owner_ref).map(|id| (id, scope)))
            .collect();
        indexed.sort_by(|(a_id, a_scope), (b_id, b_scope)| {
            a_id.cmp(b_id)
                .then_with(|| a_scope.resource_key.cmp(&b_scope.resource_key))
                .then_with(|| a_scope.scope_id.cmp(&b_scope.scope_id))
        });
        indexed.into_iter().map(|(_, scope)| scope).collect()
    }
}
