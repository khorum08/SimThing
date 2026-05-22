use crate::spec::script::ScopeRef;
use crate::spec::trigger::TriggerDirection;
use simthing_core::{SimPropertyId, SubFieldRole};

#[derive(Clone, Debug, PartialEq)]
pub enum CompiledTrigger {
    Threshold(CompiledThresholdTrigger),
    Predicate(crate::spec::script::ScriptPredicate),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledThresholdTrigger {
    pub target:    ScopeRef,
    pub property:  SimPropertyId,
    pub role:      SubFieldRole,
    pub col:       usize,
    pub threshold: f32,
    pub direction: TriggerDirection,
}
