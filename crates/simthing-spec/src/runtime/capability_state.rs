use crate::keys::{CapabilityEntryKey, CapabilityTreeDefinitionId, CategoryKey};
use crate::spec::capability::ActivationMode;
use simthing_core::SimThingId;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityTreeInstance {
    pub owner_id:      SimThingId,
    pub definition_id: CapabilityTreeDefinitionId,
    pub tree_thing_id: SimThingId,
    pub tree_slot:     u32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CapabilityTreeState {
    pub owner_id:                 SimThingId,
    pub definition_id:            CapabilityTreeDefinitionId,
    pub activation_mode_by_entry: HashMap<CapabilityEntryKey, ActivationMode>,
    pub active_by_category:       HashMap<CategoryKey, Vec<CapabilityEntryKey>>,
}

impl CapabilityTreeState {
    pub fn new(owner_id: SimThingId, definition_id: CapabilityTreeDefinitionId) -> Self {
        Self {
            owner_id,
            definition_id,
            activation_mode_by_entry: HashMap::new(),
            active_by_category:       HashMap::new(),
        }
    }

    pub fn activation_mode(
        &self,
        entry: &CapabilityEntryKey,
        default: ActivationMode,
    ) -> ActivationMode {
        self.activation_mode_by_entry
            .get(entry)
            .copied()
            .unwrap_or(default)
    }

    pub fn set_activation_mode(&mut self, entry: CapabilityEntryKey, mode: ActivationMode) {
        self.activation_mode_by_entry.insert(entry, mode);
    }

    pub fn is_active(&self, category: &CategoryKey, entry: &CapabilityEntryKey) -> bool {
        self.active_by_category
            .get(category)
            .is_some_and(|active| active.contains(entry))
    }
}
