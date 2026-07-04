use crate::spec::{EffectSpec, TriggerSpec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventKey(pub String);

impl EventKey {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for EventKey {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for EventKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSpec {
    pub id: String,
    pub trigger: TriggerSpec,
    pub effects: Vec<EffectSpec>,
    #[serde(default)]
    pub cooldown: Option<CooldownSpec>,
    #[serde(default)]
    pub priority: EventPriority,
    /// Which scenario owners receive an instance of this event at install
    /// time. Defaults to `SessionRoot` (one instance on `Scenario::root.id`,
    /// matching pre-O4 behavior). Use `AllOfKind { kind: "Faction" }` for
    /// per-faction events. See `docs/adr/scripted_event_scope_model.md`.
    #[serde(default = "default_event_install")]
    pub install: crate::spec::install_target::InstallTargetSpec,
}

fn default_event_install() -> crate::spec::install_target::InstallTargetSpec {
    crate::spec::install_target::InstallTargetSpec::SessionRoot
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CooldownSpec {
    pub ticks: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::EventKey;

}
