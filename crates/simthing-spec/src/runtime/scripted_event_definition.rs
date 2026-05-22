use crate::runtime::{CompiledEffect, CompiledTrigger};
use crate::spec::{CooldownSpec, EventKey, EventPriority};

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptedEventDefinition {
    pub id:       EventKey,
    pub trigger:  CompiledTrigger,
    pub effects:  Vec<CompiledEffect>,
    pub cooldown: Option<CooldownSpec>,
    pub priority: EventPriority,
}
