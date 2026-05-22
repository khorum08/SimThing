use crate::keys::{CapabilityEffectKey, CapabilityEntryKey, CategoryKey};
use crate::spec::capability::{ActivationMode, MaxActivePolicy};
use serde::{Deserialize, Serialize};
use simthing_core::{OverlayId, PropertyTransformDelta, SimPropertyId, SubFieldRole};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

// Re-export the canonical `CapabilityUnlockRegistration` from the feeder so
// PR 3 callers continue to work without changes. PR 4 moved the type from a
// `simthing-spec`-local placeholder to its permanent home in `simthing-feeder`.
pub use simthing_feeder::CapabilityUnlockRegistration;

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
    pub id: CapabilityTreeDefinitionId,
    pub tree_id: String,
    pub categories: HashMap<CategoryKey, CapabilityCategoryDefinition>,
    pub entries: HashMap<CapabilityEntryKey, CapabilityDefinition>,
    /// Fast lookup for the boundary handler when a Pass 7 threshold fires:
    /// `(property_id, sub_field_role) -> entry`.
    pub by_threshold: HashMap<(SimPropertyId, SubFieldRole), CapabilityEntryKey>,
    /// Fast lookup for UI/preview: `overlay_id -> entry`.
    pub by_overlay: HashMap<OverlayId, CapabilityEntryKey>,
}

/// One compiled capability category. Category policy is shared by all faction
/// instances; per-faction active entries live in `CapabilityTreeState`.
#[derive(Clone, Debug)]
pub struct CapabilityCategoryDefinition {
    pub key: CategoryKey,
    pub property_id: SimPropertyId,
    pub max_active: Option<MaxActivePolicy>,
    pub tier: u32,
}

/// One compiled capability entry.
#[derive(Clone, Debug)]
pub struct CapabilityDefinition {
    pub key: CapabilityEntryKey,
    pub display_name: String,
    pub description: String,
    pub flavor_text: Option<String>,
    pub activation: ActivationMode,
    /// One `OverlayId` per effect. Activated together when the entry unlocks.
    pub overlay_ids: Vec<OverlayId>,
    /// Logical effect keys, parallel-indexed with `overlay_ids`. Stable across
    /// builds — the runtime atomic `OverlayId::new()` is not, so debug/studio
    /// tools key off `CapabilityEffectKey` instead.
    pub effect_keys: Vec<CapabilityEffectKey>,
    /// Compiled transform payloads, parallel-indexed with `overlay_ids`.
    /// Used by CPU preview without needing the template SimThing.
    pub effect_transforms: Vec<PropertyTransformDelta>,
    pub prereqs: Vec<CapabilityPrereq>,
    pub progress_col: usize,
    pub research_cost: f32,
}

/// A resolved prereq reference. The boundary handler reads
/// `shadow[tree_slot * n_dims + col]` and compares to `min_value`.
#[derive(Clone, Debug)]
pub struct CapabilityPrereq {
    pub property_id: SimPropertyId,
    pub role: SubFieldRole,
    /// Column index resolved at build time via `col_for_role`. The boundary
    /// handler does array reads, not name lookups (per `docs/invariants.md`).
    pub col: usize,
    pub min_value: f32,
}
