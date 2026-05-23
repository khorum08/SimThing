use crate::keys::{CapabilityEntryKey, CategoryKey};
use crate::runtime::CapabilityTreeDefinitionId;
use crate::spec::capability::ActivationMode;
use serde::{Deserialize, Serialize};
use simthing_core::{OverlayId, SimThingId};
use std::collections::HashMap;

/// One per faction instance. Immutable after session init.
///
/// `by_overlay` is per-instance: each cloned tree gets fresh `OverlayId`s at
/// install time, so the reverse map (overlay id → entry key) cannot live on
/// the shared [`CapabilityTreeDefinition`](super::CapabilityTreeDefinition).
/// See `docs/adr/game_mode_session_installation.md` consequence (c.i).
#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityTreeInstance {
    pub owner_id: SimThingId,
    pub definition_id: CapabilityTreeDefinitionId,
    pub tree_thing_id: SimThingId,
    pub tree_slot: u32,
    /// Fast lookup `overlay_id -> entry_key` for this clone's overlays.
    /// Built at install time from the template `by_overlay` + the per-clone
    /// `OverlayId` re-stamping. Empty when constructed by hand (e.g., in
    /// older PR 5/11 tests that don't need the lookup).
    pub by_overlay: HashMap<OverlayId, CapabilityEntryKey>,
    /// Per-overlay host SimThing — the node the overlay actually lives on
    /// in the world tree. Determined by `EffectTarget` at install time
    /// (`Owner` → owner; `CapabilityTree` → clone; `SessionRoot` → root).
    /// The boundary handler reads this to pick `target` on
    /// `ActivateOverlay`/`SuspendOverlay`. Missing entries default to
    /// `tree_thing_id` (v0 behavior, used by older hand-built tests).
    pub overlay_hosts: HashMap<OverlayId, SimThingId>,
}

/// One per faction instance. Mutable at boundary time.
#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityTreeState {
    pub owner_id: SimThingId,
    pub definition_id: CapabilityTreeDefinitionId,
    /// Tracks runtime activation mode per entry. Entries not present
    /// default to their authored ActivationMode.
    pub activation_mode_by_entry: HashMap<CapabilityEntryKey, ActivationMode>,
    /// Tracks currently active entries per category for mutual exclusivity.
    /// Vec order = activation order (oldest first, newest last).
    pub active_by_category: HashMap<CategoryKey, Vec<CapabilityEntryKey>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CapabilityTreeNotification {
    IdeaSwitched {
        owner_id: SimThingId,
        category: CategoryKey,
        suspended: CapabilityEntryKey,
        activated: CapabilityEntryKey,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum CapabilityTreeDiagnostic {
    UnknownThresholdSimThing {
        sim_thing_id: SimThingId,
    },
    UnknownDefinition {
        definition_id: CapabilityTreeDefinitionId,
    },
    EntryNotInTree {
        definition_id: CapabilityTreeDefinitionId,
        entry: CapabilityEntryKey,
    },
}

impl std::fmt::Display for CapabilityTreeDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownThresholdSimThing { sim_thing_id } => {
                write!(
                    f,
                    "capability unlock referenced unknown SimThing `{sim_thing_id:?}`"
                )
            }
            Self::UnknownDefinition { definition_id } => {
                write!(
                    f,
                    "capability tree definition `{definition_id:?}` is not loaded"
                )
            }
            Self::EntryNotInTree { definition_id, entry } => {
                write!(
                    f,
                    "entry `{entry:?}` is not in capability tree definition `{definition_id:?}`"
                )
            }
        }
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;
    use crate::keys::CapabilityEntryKey;

    #[test]
    fn capability_tree_diagnostic_display_includes_sim_thing_id() {
        let id = SimThingId::new();
        let diagnostic = CapabilityTreeDiagnostic::UnknownThresholdSimThing {
            sim_thing_id: id,
        };
        let text = format!("{diagnostic}");
        assert!(!text.is_empty());
        assert!(text.contains(&format!("{id:?}")));
    }

    #[test]
    fn capability_tree_diagnostic_display_includes_definition_id() {
        let definition_id = CapabilityTreeDefinitionId::new();
        let diagnostic = CapabilityTreeDiagnostic::UnknownDefinition { definition_id };
        let text = format!("{diagnostic}");
        assert!(!text.is_empty());
        assert!(text.contains(&format!("{definition_id:?}")));
    }

    #[test]
    fn capability_tree_diagnostic_entry_not_in_tree_display_includes_entry() {
        let definition_id = CapabilityTreeDefinitionId::new();
        let entry = CapabilityEntryKey {
            category: CategoryKey {
                namespace: "tech".into(),
                name:      "propulsion".into(),
            },
            entry_id: "warp_drive".into(),
        };
        let diagnostic = CapabilityTreeDiagnostic::EntryNotInTree {
            definition_id,
            entry: entry.clone(),
        };
        let text = format!("{diagnostic}");
        assert!(text.contains("warp_drive"));
        assert!(text.contains(&format!("{definition_id:?}")));
    }
}
