use serde::{Deserialize, Serialize};

/// Declares which entities a capability tree (or scripted event) attaches to
/// at session install time. Authored on the spec, resolved against a
/// `Scenario` by `simthing-driver`'s install module.
///
/// See `docs/adr/game_mode_session_installation.md`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InstallTargetSpec {
    /// Install on every `SimThing` in the session root whose kind matches
    /// the authored string. Comparison goes through
    /// `simthing_core::kind_matches`.
    AllOfKind { kind: String },

    /// Install on the `SimThing` ids the scenario lists under
    /// `Scenario::install_targets[target_id]`. Missing key or empty list is
    /// a hard install error.
    ScenarioListed { target_id: String },

    /// Install once on `Scenario::root.id`. V0 default for scripted events.
    SessionRoot,
}

impl InstallTargetSpec {
    /// Default capability-tree install target — covers existing RON files
    /// that omit the `install` field. Matches every `Faction` in the scene.
    pub fn faction_default() -> Self {
        Self::AllOfKind { kind: "Faction".into() }
    }
}
