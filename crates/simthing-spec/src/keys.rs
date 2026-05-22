use serde::{Deserialize, Serialize};
use std::fmt;

/// Stable logical id for a capability tree spec (`CapabilityTreeSpec.tree_id`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityTreeKey(pub String);

impl CapabilityTreeKey {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for CapabilityTreeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Category identity within a tree (`property_namespace::property_name`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CategoryKey {
    pub namespace: String,
    pub name:      String,
}

impl CategoryKey {
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name:      name.into(),
        }
    }

    pub fn from_category_spec(spec: &crate::spec::capability::CapabilityCategorySpec) -> Self {
        Self::new(&spec.property_namespace, &spec.property_name)
    }
}

impl fmt::Display for CategoryKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/// Entry identity within a tree (`category_key` + entry `id`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityEntryKey {
    pub category: CategoryKey,
    pub entry_id: String,
}

impl CapabilityEntryKey {
    pub fn new(category: CategoryKey, entry_id: impl Into<String>) -> Self {
        Self {
            category,
            entry_id: entry_id.into(),
        }
    }
}

impl fmt::Display for CapabilityEntryKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.category, self.entry_id)
    }
}

/// Logical effect identity (`entry` + zero-based effect index).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityEffectKey {
    pub entry:        CapabilityEntryKey,
    pub effect_index: usize,
}

/// Runtime definition handle (monotonic within a session build).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct CapabilityTreeDefinitionId(pub u32);

impl CapabilityTreeDefinitionId {
    pub fn new(raw: u32) -> Self {
        Self(raw)
    }
}
