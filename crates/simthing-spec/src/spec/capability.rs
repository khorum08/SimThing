use serde::{Deserialize, Serialize};

/// How a capability entry is authored to become active.
///
/// `Threshold` and `PlayerSelection` are valid authored defaults.
/// `OnPrereqMet` is **runtime-only** — entries transition into it when their
/// progress threshold fires but prereqs are not yet met. Authoring an entry
/// directly as `OnPrereqMet` is a hard error (`SpecError::OnPrereqMetAuthoredDefault`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum ActivationMode {
    /// Pass 7 threshold registered at `research_cost`; fires automatically.
    #[default]
    Threshold,
    /// No GPU threshold; activated by explicit player/UI selection.
    PlayerSelection,
    /// Runtime-only state. The progress threshold fired but at least one prereq
    /// was not yet satisfied; a CPU sweep re-checks after every activation in
    /// the same tree and once at session init. Never valid as an authored default.
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

/// Category-level mutual exclusivity policy (authored).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MaxActivePolicy {
    #[default]
    Unlimited,
    Limited {
        count: usize,
    },
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
    pub category: String,
    pub entry_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityEffectSpec {
    pub targets_property: String,
    pub sub_field_deltas: Vec<(simthing_core::SubFieldRole, simthing_core::TransformOp)>,
    pub when_activated:   simthing_core::OverlayLifecycle,
}
