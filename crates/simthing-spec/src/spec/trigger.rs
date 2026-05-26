use crate::spec::script::{PropertyKey, ScopeRef, ScriptPredicate};
use serde::{Deserialize, Serialize};
use simthing_core::SubFieldRole;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerDirection {
    Rising,
    Falling,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerSpec {
    Threshold {
        target: ScopeRef,
        property: PropertyKey,
        role: SubFieldRole,
        threshold: f32,
        direction: TriggerDirection,
    },
    Predicate {
        predicate: ScriptPredicate,
    },
}
