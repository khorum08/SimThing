use crate::keys::{CapabilityEntryKey, CategoryKey};
use crate::runtime::CapabilityTreeDefinitionId;
use crate::spec::capability::ActivationMode;
use simthing_core::SimThingId;
use std::collections::HashMap;

/// One per faction instance. Immutable after session init.
#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityTreeInstance {
    pub owner_id: SimThingId,
    pub definition_id: CapabilityTreeDefinitionId,
    pub tree_thing_id: SimThingId,
    pub tree_slot: u32,
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

#[derive(Clone, Debug, PartialEq)]
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
