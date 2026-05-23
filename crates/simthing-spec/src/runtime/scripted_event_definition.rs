use crate::runtime::{CompiledEffect, CompiledTrigger};
use crate::spec::script::ScopeRef;
use crate::spec::trigger::TriggerDirection;
use crate::spec::{CooldownSpec, EventKey, EventPriority};
use simthing_core::{Direction, SimThingId};
use simthing_feeder::ScriptedEventTriggerRegistration;
use std::sync::atomic::{AtomicU32, Ordering};

/// Atomic runtime id for a `ScriptedEventDefinition`. Parallel to
/// `CapabilityTreeDefinitionId` — allocated once per definition
/// install, used as the foreign key on `ScriptedEventInstance`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScriptedEventDefinitionId(u32);

impl ScriptedEventDefinitionId {
    pub fn new() -> Self {
        static NEXT: AtomicU32 = AtomicU32::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
    pub fn raw(self) -> u32 {
        self.0
    }
}

impl Default for ScriptedEventDefinitionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptedEventDefinition {
    pub id:       EventKey,
    pub trigger:  CompiledTrigger,
    pub effects:  Vec<CompiledEffect>,
    pub cooldown: Option<CooldownSpec>,
    pub priority: EventPriority,
}

/// Per-owner, per-definition instance — what actually fires in the world.
/// See `docs/adr/scripted_event_scope_model.md` (Option B).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScriptedEventInstanceKey {
    pub owner_id: SimThingId,
    pub event_id: EventKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptedEventInstance {
    pub key:                ScriptedEventInstanceKey,
    pub definition_id:      ScriptedEventDefinitionId,
    /// Slot used to resolve `ScopeRef::Current` for this instance. Refreshed
    /// from the allocator on slot churn (fission, removal).
    pub current_slot:       u32,
    /// Boundaries remaining until this instance may fire again. 0 = ready.
    pub cooldown_remaining: u32,
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
