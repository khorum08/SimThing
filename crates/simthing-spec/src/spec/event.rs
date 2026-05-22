use crate::spec::{EffectSpec, TriggerSpec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventKey(pub String);

impl EventKey {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
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
