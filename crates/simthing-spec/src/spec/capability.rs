use serde::{Deserialize, Serialize};

/// How a capability entry becomes active at runtime.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum ActivationMode {
    /// Pass 7 threshold registered at `research_cost`; fires automatically.
    #[default]
    Threshold,
    /// No GPU threshold; activated by explicit player/UI selection.
    PlayerSelection,
    /// Runtime-only: threshold fired but prereqs were unmet; swept each boundary.
    OnPrereqMet,
}

/// Authored research-rate seam (Script arm reserved for future).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResearchRateSpec {
    Literal(f32),
}

impl Default for ResearchRateSpec {
    fn default() -> Self {
        Self::Literal(0.0)
    }
}

impl ResearchRateSpec {
    pub fn value(&self) -> f32 {
        match self {
            Self::Literal(v) => *v,
        }
    }
}

/// Category-level mutual exclusivity policy.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MaxActivePolicy {
    #[default]
    Unlimited,
    Limited {
        count: usize,
    },
}

impl MaxActivePolicy {
    pub fn from_option(max_active: Option<usize>) -> Result<Self, crate::error::CapabilityTreeError> {
        match max_active {
            None => Ok(Self::Unlimited),
            Some(0) => Err(crate::error::CapabilityTreeError::InvalidMaxActive(
                0,
                crate::keys::CategoryKey::new("", ""),
            )),
            Some(1) => Ok(Self::Limited { count: 1 }),
            Some(n) => Err(crate::error::CapabilityTreeError::UnsupportedMaxActive(n)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityTreeSpec {
    pub tree_id:    String,
    pub tree_kind:  String,
    pub owner_kind: String,
    pub categories: Vec<CapabilityCategorySpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityCategorySpec {
    pub property_namespace: String,
    pub property_name:      String,
    pub display_name:       String,
    #[serde(default)]
    pub tier:               u32,
    #[serde(default)]
    pub max_active:         Option<usize>,
    pub entries:            Vec<CapabilitySpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilitySpec {
    pub id:            String,
    pub display_name:  String,
    #[serde(default)]
    pub description:   String,
    #[serde(default)]
    pub flavor_text:   String,
    pub research_cost: f32,
    #[serde(default)]
    pub activation:    ActivationMode,
    #[serde(default)]
    pub research_rate: ResearchRateSpec,
    #[serde(default)]
    pub icon:          String,
    #[serde(default)]
    pub thumbnail:     String,
    #[serde(default)]
    pub card_image:    String,
    #[serde(default)]
    pub unlock_video:  Option<String>,
    #[serde(default)]
    pub model_preview: Option<String>,
    #[serde(default)]
    pub prereqs:       Vec<CapabilityPrereqSpec>,
    #[serde(default)]
    pub unlocks_ship_components: Vec<String>,
    #[serde(default)]
    pub unlocks_buildings:       Vec<String>,
    #[serde(default)]
    pub unlocks_units:           Vec<String>,
    #[serde(default)]
    pub unlocks_weapons:         Vec<String>,
    pub effects:                 Vec<CapabilityEffectSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityPrereqSpec {
    pub category:  String,
    pub entry_id:  String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityEffectSpec {
    pub targets_property: String,
    pub sub_field_deltas: Vec<(simthing_core::SubFieldRole, simthing_core::TransformOp)>,
    pub when_activated:   simthing_core::OverlayLifecycle,
}
