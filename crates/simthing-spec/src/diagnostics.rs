use serde::{Deserialize, Serialize};

/// Non-fatal authoring warnings collected during spec compilation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpecDiagnostics {
    pub warnings: Vec<SpecWarning>,
}

impl SpecDiagnostics {
    pub fn push(&mut self, warning: SpecWarning) {
        self.warnings.push(warning);
    }

    pub fn merge(&mut self, other: SpecDiagnostics) {
        self.warnings.extend(other.warnings);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SpecWarning {
    EmptyEffects {
        tree_id:  String,
        entry_id: String,
    },
    MaxActiveWithoutPlayerSelection {
        tree_id:      String,
        category_key: String,
    },
    UnusedCategory {
        tree_id:      String,
        category_key: String,
    },
    EmptyCapabilityContainerKinds {
        context: String,
    },
}

/// Capability-tree-specific diagnostic emitted at boundary time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CapabilityTreeDiagnostic {
    PrereqsUnmet {
        entry: CapabilityEntryKeyRef,
    },
    ProgressReset {
        entry: CapabilityEntryKeyRef,
        value: f32,
    },
    ActivationModeChanged {
        entry: CapabilityEntryKeyRef,
        from:  crate::spec::capability::ActivationMode,
        to:    crate::spec::capability::ActivationMode,
    },
    MutualExclusivitySuspended {
        entry: CapabilityEntryKeyRef,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityEntryKeyRef {
    pub tree_id:  String,
    pub category: String,
    pub entry_id: String,
}

impl CapabilityEntryKeyRef {
    pub fn from_key(tree_id: &str, key: &crate::keys::CapabilityEntryKey) -> Self {
        Self {
            tree_id:  tree_id.to_owned(),
            category: key.category.to_string(),
            entry_id: key.entry_id.clone(),
        }
    }
}

pub type SpecResult<T> = Result<(T, SpecDiagnostics), crate::error::CapabilityTreeError>;
