use crate::keys::{CapabilityEffectKey, CapabilityEntryKey, CapabilityTreeDefinitionId, CategoryKey};
use crate::spec::capability::ActivationMode;
use simthing_core::{OverlayId, SimPropertyId, SubFieldRole};
use std::collections::HashMap;

/// Resolved runtime prereq (column indices resolved at build time).
#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityPrereq {
    pub property_id: SimPropertyId,
    pub role:        SubFieldRole,
    pub col:         usize,
    pub min_value:   f32,
}

#[derive(Clone, Debug)]
pub struct CapabilityDefinition {
    pub key:                  CapabilityEntryKey,
    pub display_name:         String,
    pub research_cost:        f32,
    pub default_activation:   ActivationMode,
    pub progress_property_id: SimPropertyId,
    pub progress_role:        SubFieldRole,
    pub progress_col:         usize,
    pub rate_col:             Option<usize>,
    pub overlay_ids:          Vec<OverlayId>,
    pub overlay_transforms:   Vec<simthing_core::PropertyTransformDelta>,
    pub effect_keys:          Vec<CapabilityEffectKey>,
    pub prereqs:              Vec<CapabilityPrereq>,
    pub category_key:         CategoryKey,
}

#[derive(Clone, Debug)]
pub struct CapabilityTreeDefinition {
    pub id:           CapabilityTreeDefinitionId,
    pub tree_id:      String,
    pub tree_kind:    String,
    pub owner_kind:   String,
    pub categories:   HashMap<CategoryKey, CategoryDefinition>,
    pub entries:      HashMap<CapabilityEntryKey, CapabilityDefinition>,
    pub by_threshold: HashMap<(SimPropertyId, SubFieldRole), CapabilityEntryKey>,
    pub by_overlay:   HashMap<OverlayId, CapabilityEntryKey>,
}

#[derive(Clone, Debug)]
pub struct CategoryDefinition {
    pub key:        CategoryKey,
    pub display_name: String,
    pub max_active: crate::spec::capability::MaxActivePolicy,
    pub property_id: SimPropertyId,
}

impl CapabilityTreeDefinition {
    pub fn entry(&self, key: &CapabilityEntryKey) -> Option<&CapabilityDefinition> {
        self.entries.get(key)
    }

    pub fn entry_by_id(&self, category: &CategoryKey, entry_id: &str) -> Option<&CapabilityDefinition> {
        self.entries.get(&CapabilityEntryKey::new(category.clone(), entry_id))
    }
}
