use crate::spec::install_target::InstallTargetSpec;
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

/// Category-level mutual exclusivity policy (authored).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MaxActivePolicy {
    #[default]
    Unlimited,
    Limited {
        count: usize,
        replacement: ReplacementPolicy,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReplacementPolicy {
    #[default]
    SuspendOldest,
    ExplicitSelectionRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityTreeSpec {
    pub tree_id: String,
    pub tree_kind: String,
    pub owner_kind: String,
    pub categories: Vec<CapabilityCategorySpec>,
    /// Authored install target. Defaults to `AllOfKind { kind: "Faction" }`
    /// so RON files that omit this field install on every faction in scope.
    /// See `docs/adr/game_mode_session_installation.md`.
    #[serde(default = "default_capability_install")]
    pub install: InstallTargetSpec,
}

fn default_capability_install() -> InstallTargetSpec {
    InstallTargetSpec::faction_default()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityCategorySpec {
    pub property_namespace: String,
    pub property_name: String,
    pub display_name: String,
    #[serde(default)]
    pub tier: u32,
    #[serde(default)]
    pub max_active: Option<MaxActivePolicy>,
    pub entries: Vec<CapabilitySpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilitySpec {
    pub id: String,
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub flavor_text: String,
    pub research_cost: f32,
    #[serde(default)]
    pub activation: ActivationMode,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub thumbnail: String,
    #[serde(default)]
    pub card_image: String,
    #[serde(default)]
    pub unlock_video: Option<String>,
    #[serde(default)]
    pub model_preview: Option<String>,
    #[serde(default)]
    pub prereqs: Vec<CapabilityPrereqSpec>,
    #[serde(default)]
    pub unlocks_ship_components: Vec<String>,
    #[serde(default)]
    pub unlocks_buildings: Vec<String>,
    #[serde(default)]
    pub unlocks_units: Vec<String>,
    #[serde(default)]
    pub unlocks_weapons: Vec<String>,
    pub effects: Vec<CapabilityEffectSpec>,
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
    pub when_activated: simthing_core::OverlayLifecycle,
    /// Which SimThing this effect's overlay transforms when activated.
    /// `Owner` (default) sends `affects` to the install-time owner —
    /// the natural target for faction/character bonuses. See
    /// `docs/adr/capability_effect_target_scope.md`.
    #[serde(default)]
    pub effect_target: EffectTarget,
}

/// Authored selector for which SimThing a `CapabilityEffectSpec`'s
/// overlay applies to when activated. Resolved at install time by
/// `simthing_driver::install::install_tree_for_owner`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EffectTarget {
    /// Install-time owner (the SimThing the capability tree was cloned for,
    /// not the cloned tree itself). v1 default — matches modder intuition
    /// for faction/character bonuses.
    #[default]
    Owner,
    /// Cloned capability-tree SimThing. v0 behavior — useful for
    /// tree-internal state (milestones, counters, bookkeeping).
    CapabilityTree,
    /// `Scenario::root.id`. Global era flags, world-state triggers.
    SessionRoot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_spec_ignores_legacy_research_rate_field() {
        let json = r#"{
            "id": "drive",
            "display_name": "Drive",
            "research_cost": 100.0,
            "effects": [],
            "research_rate": { "Literal": 0.5 }
        }"#;
        let spec: CapabilitySpec = serde_json::from_str(json).expect("deserialize");
        assert_eq!(spec.id, "drive");
        assert_eq!(spec.research_cost, 100.0);
    }
}
