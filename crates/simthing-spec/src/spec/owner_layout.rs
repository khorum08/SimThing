//! Session-local persistent RF layout keys (rung 9.2).
//!
//! Layout identity is interned owner id + [`SimThingId`] (graduated 6.4
//! stable logical identity) + resource/scope. It is never `OwnerRef`
//! lexical order, physical [`SlotIndex`], or allocation/vector order.

use std::collections::{HashMap, HashSet};

use simthing_core::owner_channel::{OwnerInternError, OwnerInterner, OwnerLayoutId, OwnerRef};
use simthing_core::SimThingId;

use super::channel_key::{OwnerChannelScopeKey, ResourceKey, ScopeId};
use super::owner_channel_rf::{OwnerChannelRfBucket, OwnerChannelRfOwnAggregate};

/// Persistent RF layout key. Session-local; not seam-serializable.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersistentRfLayoutKey {
    pub owner: OwnerLayoutId,
    pub object: SimThingId,
    pub resource_key: ResourceKey,
    pub scope_id: ScopeId,
}

/// Session-local intern + append-only layout index assignment.
///
/// Held by the driver `SpecSessionState`. First-seen interned owner
/// ids and layout indices survive owner flag-switch, owner add/remove, and
/// 6.4 `EpochRebind`. They are never rebuilt from vector enumeration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentRfLayout {
    pub interner: OwnerInterner,
    object_owners: HashMap<SimThingId, OwnerRef>,
    keys: Vec<PersistentRfLayoutKey>,
    index: HashMap<PersistentRfLayoutKey, u32>,
}

impl PersistentRfLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern_owner(&mut self, owner: &OwnerRef) -> OwnerLayoutId {
        self.interner.intern(owner)
    }

    /// Reuse held interned ids: flag-switch rebinds when the old owner is
    /// no longer live; a new owner appends. Unrelated ids are never
    /// renumbered. `AlreadyInterned` on rebind means the target string is
    /// already a live interned owner — intern that existing id instead of
    /// remapping anyone else.
    pub fn sync_owners(
        &mut self,
        resolved: &[(SimThingId, OwnerRef)],
    ) -> Result<(), OwnerInternError> {
        let live: HashSet<SimThingId> = resolved.iter().map(|(id, _)| *id).collect();
        for (object, new_owner) in resolved {
            match self.object_owners.get(object).cloned() {
                Some(old) if old != *new_owner => {
                    let old_still_live = resolved
                        .iter()
                        .any(|(id, owner)| *id != *object && owner == &old);
                    if !old_still_live {
                        match self.interner.rebind(&old, new_owner.clone()) {
                            Ok(_) => {}
                            Err(OwnerInternError::AlreadyInterned { .. }) => {
                                self.interner.intern(new_owner);
                            }
                            Err(err) => return Err(err),
                        }
                    } else {
                        self.interner.intern(new_owner);
                    }
                    self.object_owners.insert(*object, new_owner.clone());
                }
                Some(_) => {
                    self.interner.intern(new_owner);
                }
                None => {
                    self.interner.intern(new_owner);
                    self.object_owners.insert(*object, new_owner.clone());
                }
            }
        }
        self.object_owners.retain(|id, _| live.contains(id));
        Ok(())
    }

    /// Append layout keys for current buckets. Existing keys keep their
    /// first-seen indices; new keys are appended. Never re-sorts.
    pub fn assign_from_buckets(
        &mut self,
        buckets: &[OwnerChannelRfBucket],
        aggregates: &[OwnerChannelRfOwnAggregate],
    ) {
        for bucket in buckets {
            let owner = self.interner.intern(&bucket.scope.owner_ref);
            for &row in &bucket.source_row_indices {
                let Some(aggregate) = aggregates.get(row) else {
                    continue;
                };
                let key = PersistentRfLayoutKey {
                    owner,
                    object: aggregate.simthing_id,
                    resource_key: bucket.scope.resource_key.clone(),
                    scope_id: bucket.scope.scope_id.clone(),
                };
                if self.index.contains_key(&key) {
                    continue;
                }
                let idx = self.keys.len() as u32;
                self.keys.push(key.clone());
                self.index.insert(key, idx);
            }
        }
    }

    pub fn index_of(&self, key: &PersistentRfLayoutKey) -> Option<u32> {
        self.index.get(key).copied()
    }

    pub fn keys(&self) -> &[PersistentRfLayoutKey] {
        &self.keys
    }

    pub fn layout_key_for_object(
        &self,
        scope: &OwnerChannelScopeKey,
        object: SimThingId,
    ) -> Option<PersistentRfLayoutKey> {
        let owner = self.interner.id_of(&scope.owner_ref)?;
        Some(PersistentRfLayoutKey {
            owner,
            object,
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
