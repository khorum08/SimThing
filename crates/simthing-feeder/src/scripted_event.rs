//! Scripted event registration + resolved-event types.
//!
//! These are the threshold-path equivalents of [`CapabilityUnlockRegistration`]
//! and [`CapabilityUnlockEvent`] (in `capability.rs`), specialized for the
//! `simthing-spec` scripted event system.
//!
//! Lifecycle:
//! 1. A session/driver layer compiles `EventSpec`s via `simthing_spec::compile_event`
//!    and obtains `ScriptedEventDefinition`s with `CompiledTrigger::Threshold(...)`
//!    triggers.
//! 2. For each owner that should run the event, the session calls
//!    `ScriptedEventDefinition::to_trigger_registration(current_slot)` to produce
//!    a [`ScriptedEventTriggerRegistration`].
//! 3. The session hands the registrations to `simthing_sim::ThresholdBuilder::build_with_scripted_event_triggers`
//!    at the day boundary, which materializes them as GPU `ThresholdRegistration`s
//!    plus parallel `ThresholdSemantic::ScriptedEventTrigger` entries in the CPU
//!    registry.
//! 4. When the GPU fires a `ThresholdEvent`, the session calls
//!    `ThresholdRegistry::extract_scripted_event_triggers` to filter and resolve
//!    them to [`ScriptedEventTriggerEvent`]s.
//! 5. The session passes those events to
//!    `ScriptedEventBoundaryHandler::handle_tick(threshold_events, ctx)`, which
//!    fires the matching scripted events under the same cooldown/priority rules
//!    as predicate-driven events.
//!
//! `event_id` is a plain `String` (not `EventKey`) so this crate stays
//! independent of `simthing-spec`. The spec's `EventKey` is a transparent
//! `String` newtype; conversion is `event_key.0.clone()`.

use serde::{Deserialize, Serialize};
use simthing_core::Direction;

/// Authored-side request: when `(slot, col)` crosses `threshold` in the given
/// direction, fire the scripted event identified by `event_id`.
///
/// Produced by `simthing_spec::ScriptedEventDefinition::to_trigger_registration`
/// for `CompiledTrigger::Threshold` events. Consumed by
/// `simthing_sim::ThresholdBuilder::build_with_scripted_event_triggers`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptedEventTriggerRegistration {
    /// Matches `simthing_spec::EventKey.0`. Identifies which scripted event
    /// definition the boundary handler should fire when this threshold trips.
    pub event_id: String,
    /// Concrete slot in the GPU `values` buffer to watch. `ScopeRef::Current`
    /// must be resolved to a real slot before constructing this struct.
    pub slot: u32,
    /// Absolute column in the dense `values` row (already resolved via
    /// `DimensionRegistry::column_range` + `col_for_role`).
    pub col: u32,
    /// Trigger threshold value, copied from the `CompiledThresholdTrigger`.
    pub threshold: f32,
    /// Rising = fire when value crosses upward through `threshold`;
    /// Falling = fire when value crosses downward through `threshold`.
    pub direction: Direction,
}

/// A fired scripted-event threshold, already resolved from
/// `ThresholdSemantic::ScriptedEventTrigger` by whoever drains the GPU's
/// `ThresholdEvent` stream (typically via
/// `ThresholdRegistry::extract_scripted_event_triggers` in `simthing-sim`).
///
/// Consumed by `simthing_spec::ScriptedEventBoundaryHandler::handle_tick`.
#[derive(Clone, Debug, PartialEq)]
pub struct ScriptedEventTriggerEvent {
    /// Matches `simthing_spec::EventKey.0`.
    pub event_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registration_serde_round_trip() {
        let original = ScriptedEventTriggerRegistration {
            event_id: "rebellion_warning".into(),
            slot: 4,
            col: 12,
            threshold: 0.25,
            direction: Direction::Falling,
        };
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: ScriptedEventTriggerRegistration =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored, original);
    }
}
