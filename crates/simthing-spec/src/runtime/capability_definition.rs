use crate::keys::{CapabilityEffectKey, CapabilityEntryKey};
use serde::{Deserialize, Serialize};
use simthing_core::{OverlayId, SimPropertyId, SimThingId, SubFieldRole};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

/// Globally unique identifier for a built `CapabilityTreeDefinition`.
/// Allocated by `CapabilityTreeDefinitionId::new()` at build time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CapabilityTreeDefinitionId(u32);

impl CapabilityTreeDefinitionId {
    pub fn new() -> Self {
        static NEXT: AtomicU32 = AtomicU32::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }

    pub fn raw(self) -> u32 {
        self.0
    }
}

impl Default for CapabilityTreeDefinitionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared, read-only template for a capability tree. One per `CapabilityTreeSpec`
/// after `CapabilityTreeBuilder::build`. Faction instances reference this by id
/// and carry their own mutable `CapabilityTreeState` (PR 5).
#[derive(Clone, Debug)]
pub struct CapabilityTreeDefinition {
    pub id:           CapabilityTreeDefinitionId,
    pub tree_id:      String,
    pub entries:      HashMap<CapabilityEntryKey, CapabilityDefinition>,
    /// Fast lookup for the boundary handler when a Pass 7 threshold fires:
    /// `(property_id, sub_field_role) -> entry`.
    pub by_threshold: HashMap<(SimPropertyId, SubFieldRole), CapabilityEntryKey>,
    /// Fast lookup for UI/preview: `overlay_id -> entry`.
    pub by_overlay:   HashMap<OverlayId, CapabilityEntryKey>,
}

/// One compiled capability entry.
#[derive(Clone, Debug)]
pub struct CapabilityDefinition {
    pub key:          CapabilityEntryKey,
    pub display_name: String,
    pub description:  String,
    pub flavor_text:  Option<String>,
    /// One `OverlayId` per effect. Activated together when the entry unlocks.
    pub overlay_ids:  Vec<OverlayId>,
    /// Logical effect keys, parallel-indexed with `overlay_ids`. Stable across
    /// builds — the runtime atomic `OverlayId::new()` is not, so debug/studio
    /// tools key off `CapabilityEffectKey` instead.
    pub effect_keys:  Vec<CapabilityEffectKey>,
    pub prereqs:      Vec<CapabilityPrereq>,
}

/// A resolved prereq reference. The boundary handler reads
/// `shadow[tree_slot * n_dims + col]` and compares to `min_value`.
#[derive(Clone, Debug)]
pub struct CapabilityPrereq {
    pub property_id: SimPropertyId,
    pub role:        SubFieldRole,
    /// Column index resolved at build time via `col_for_role`. The boundary
    /// handler does array reads, not name lookups (per `docs/invariants.md`).
    pub col:         usize,
    pub min_value:   f32,
}

/// Placeholder for the type that lives in `simthing-feeder` after PR 4.
/// Defined here so PR 3 can land before the feeder side ships. PR 4 will
/// replace this with a re-export from `simthing-feeder`.
#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityUnlockRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id:  SimPropertyId,
    pub sub_field:    SubFieldRole,
    pub threshold:    f32,
}
