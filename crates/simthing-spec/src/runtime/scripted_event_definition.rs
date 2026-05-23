use crate::runtime::{CompiledEffect, CompiledTrigger};
use crate::spec::script::ScopeRef;
use crate::spec::trigger::TriggerDirection;
use crate::spec::{CooldownSpec, EventKey, EventPriority};
use simthing_core::Direction;
use simthing_feeder::ScriptedEventTriggerRegistration;

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptedEventDefinition {
    pub id:       EventKey,
    pub trigger:  CompiledTrigger,
    pub effects:  Vec<CompiledEffect>,
    pub cooldown: Option<CooldownSpec>,
    pub priority: EventPriority,
}

impl ScriptedEventDefinition {
    /// Produce a [`ScriptedEventTriggerRegistration`] suitable for handing to
    /// `simthing_sim::ThresholdBuilder::build_with_scripted_event_triggers`.
    ///
    /// Returns `None` if this definition's trigger is a predicate (predicates
    /// are evaluated CPU-side every tick, not registered as GPU thresholds).
    ///
    /// [`ScopeRef::Current`] is resolved against the supplied `current_slot`;
    /// [`ScopeRef::Slot`] is used verbatim. The caller is responsible for
    /// ensuring the slot is live in the allocator.
    pub fn to_trigger_registration(
        &self,
        current_slot: u32,
    ) -> Option<ScriptedEventTriggerRegistration> {
        let CompiledTrigger::Threshold(trigger) = &self.trigger else {
            return None;
        };
        let slot = match trigger.target {
            ScopeRef::Current  => current_slot,
            ScopeRef::Slot(s)  => s,
        };
        Some(ScriptedEventTriggerRegistration {
            event_id:  self.id.0.clone(),
            slot,
            col:       trigger.col as u32,
            threshold: trigger.threshold,
            direction: trigger_direction_to_core(&trigger.direction),
        })
    }
}

fn trigger_direction_to_core(dir: &TriggerDirection) -> Direction {
    match dir {
        TriggerDirection::Rising  => Direction::Rising,
        TriggerDirection::Falling => Direction::Falling,
    }
}
