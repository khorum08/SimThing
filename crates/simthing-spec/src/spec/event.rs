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
    pub id:       String,
    pub trigger:  TriggerSpec,
    pub effects:  Vec<EffectSpec>,
    #[serde(default)]
    pub cooldown: Option<CooldownSpec>,
    #[serde(default)]
    pub priority: EventPriority,
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

    #[test]
    fn event_key_from_str() {
        let key: EventKey = "low_loyalty".into();
        assert_eq!(key.0, "low_loyalty");
    }

    #[test]
    fn event_key_from_string() {
        let id = String::from("faction_collapse");
        let key: EventKey = id.clone().into();
        assert_eq!(key.0, id);
    }
}
