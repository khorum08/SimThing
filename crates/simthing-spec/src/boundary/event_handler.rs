//! Boundary-time execution of compiled scripted event definitions.
//!
//! [`ScriptedEventBoundaryHandler`] is called once per boundary tick and
//! handles **both predicate-triggered and threshold-triggered** scripted
//! events under unified cooldown and priority gating.
//!
//! ## Trigger paths
//!
//! - **Predicate triggers** ([`CompiledTrigger::Predicate`]) — evaluated
//!   CPU-side every tick against `ctx.shadow` via [`ScriptPredicate::eval`].
//!   If the predicate returns `true` and the event isn't on cooldown, it fires.
//! - **Threshold triggers** ([`CompiledTrigger::Threshold`]) — pre-registered
//!   with the GPU's Pass 7 threshold buffer via
//!   `simthing_sim::ThresholdBuilder::build_with_scripted_event_triggers`.
//!   When the GPU fires a `ThresholdEvent`, the session/driver layer resolves
//!   it to a `ScriptedEventTriggerEvent` (via
//!   `ThresholdRegistry::extract_scripted_event_triggers`) and passes the
//!   slice to `handle_tick`. The handler matches `event_id` against its
//!   definitions and fires under the same cooldown rules. The trigger
//!   condition is *not* re-evaluated CPU-side — the GPU has already decided.
//!
//! ## Execution protocol per call to [`ScriptedEventBoundaryHandler::handle_tick`]
//!
//! 1. **Tick cooldowns** — decrement every active cooldown counter by 1 and
//!    drop entries that reach 0.
//! 2. **Index threshold-fired events** — collect `event_id`s from
//!    `threshold_events` into a `HashSet` for O(1) lookup. Unknown event_ids
//!    push an [`UnknownEventId`](ScriptedEventDiagnosticKind::UnknownEventId)
//!    diagnostic.
//! 3. **Sort by priority** — process definitions descending
//!    (Critical → High → Normal → Low). [`EventPriority`] derives `Ord`.
//! 4. **Skip on cooldown** — if `ctx.cooldowns` contains the event's key, skip.
//! 5. **Determine fire condition by trigger kind**:
//!    - [`CompiledTrigger::Predicate`]: evaluate; eval errors push a
//!      [`TriggerEvalError`](ScriptedEventDiagnosticKind::TriggerEvalError)
//!      diagnostic and skip.
//!    - [`CompiledTrigger::Threshold`]: fire iff `event_id` is in the
//!      threshold-fired set built in step 2.
//! 6. **Resolve and emit effects** — for each [`CompiledEffect`], resolve the
//!    [`ScopeRef`] target against `ctx.slot_to_thing`. Missing slots push an
//!    [`UnresolvedEffectTarget`](ScriptedEventDiagnosticKind::UnresolvedEffectTarget)
//!    diagnostic; resolved targets push a [`BoundaryRequest`].
//! 7. **Arm cooldown** — if the event has a `cooldown`, insert its `ticks`
//!    value into `ctx.cooldowns`.

use crate::runtime::{CompiledEffect, CompiledTrigger, ScriptedEventDefinition};
use crate::spec::event::EventKey;
use crate::spec::script::{ScopeRef, ScriptEvalContext, ScriptEvalError};
use simthing_core::{DimensionRegistry, SimThingId};
use simthing_feeder::{BoundaryRequest, ScriptedEventTriggerEvent};
use std::collections::{HashMap, HashSet};

// ── Handler ───────────────────────────────────────────────────────────────────

/// Stateless handler that executes compiled scripted event definitions each
/// boundary tick. Processes both predicate triggers (CPU-evaluated) and
/// threshold triggers (GPU-fired, resolved by the session/driver layer).
pub struct ScriptedEventBoundaryHandler<'a> {
    pub registry: &'a DimensionRegistry,
    /// All event definitions to evaluate. The handler sorts internally by
    /// [`EventPriority`]; the caller does not need to pre-sort.
    pub definitions: &'a [ScriptedEventDefinition],
}

// ── Context ───────────────────────────────────────────────────────────────────

/// Per-tick mutable state threaded through event execution.
pub struct ScriptedEventBoundaryContext<'a> {
    /// Number of property dimensions in the shadow buffer.
    pub n_dims: usize,
    /// Flat CPU shadow buffer: `shadow[slot * n_dims .. (slot + 1) * n_dims]`.
    pub shadow: &'a [f32],
    /// The "home" slot for `ScopeRef::Current` in both trigger evaluation and
    /// effect resolution.
    pub current_slot: u32,
    /// Maps raw slot index (u32) → [`SimThingId`] for resolving [`ScopeRef`]
    /// targets in [`CompiledEffect`]s. A missing key means the slot is
    /// unoccupied; the effect is skipped with a diagnostic.
    pub slot_to_thing: &'a HashMap<u32, SimThingId>,
    /// Remaining-ticks map for events currently on cooldown. The handler ticks
    /// this down at the start of each call and skips events whose key is still
    /// present. Entries are removed when they reach 0. Keyed by [`EventKey`]
    /// only (not per-owner). For per-owner semantics, callers maintain separate
    /// context instances.
    pub cooldowns: &'a mut HashMap<EventKey, u32>,
    /// Accumulates [`BoundaryRequest`]s emitted by triggered event effects.
    pub requests: &'a mut Vec<BoundaryRequest>,
    /// Accumulates soft errors from trigger evaluation and effect resolution.
    /// A diagnostic here does not abort the tick; subsequent events still run.
    pub diagnostics: &'a mut Vec<ScriptedEventDiagnostic>,
}

// ── Diagnostic ────────────────────────────────────────────────────────────────

/// A non-fatal error produced during scripted event execution.
#[derive(Clone, Debug)]
pub struct ScriptedEventDiagnostic {
    /// The event that encountered an error.
    pub event_id: EventKey,
    /// The specific error.
    pub kind: ScriptedEventDiagnosticKind,
}

impl ScriptedEventDiagnostic {
    /// Owner-removed diagnostic. Emitted by the driver's per-instance
    /// slot refresh when an instance's owner no longer has a slot.
    /// O4 / `docs/adr/scripted_event_scope_model.md`.
    pub fn owner_removed(owner_id: simthing_core::SimThingId, event_id: EventKey) -> Self {
        Self {
            event_id,
            kind: ScriptedEventDiagnosticKind::OwnerRemoved { owner_id },
        }
    }
}

#[derive(Clone, Debug)]
pub enum ScriptedEventDiagnosticKind {
    /// Predicate evaluation failed at runtime.
    TriggerEvalError(ScriptEvalError),
    /// An effect's `ScopeRef` target resolved to a slot with no entry in
    /// `slot_to_thing`.
    UnresolvedEffectTarget { slot: u32 },
    /// A threshold-fired event referenced an `event_id` that doesn't match any
    /// definition in `self.definitions`. Most commonly a stale registration
    /// whose definition was unloaded.
    UnknownEventId,
    /// A scripted-event instance was dropped because its owner is no
    /// longer in the allocator (fission removal, manual delete). Emitted
    /// by the driver during per-instance slot refresh. O4.
    OwnerRemoved { owner_id: simthing_core::SimThingId },
}

impl std::fmt::Display for ScriptedEventDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TriggerEvalError(err) => write!(f, "trigger evaluation failed: {err}"),
            Self::UnresolvedEffectTarget { slot } => {
                write!(f, "effect target slot {slot} has no SimThing mapping")
            }
            Self::UnknownEventId => write!(f, "threshold fired for unknown event id"),
            Self::OwnerRemoved { owner_id } => {
                write!(
                    f,
                    "instance owner {owner_id:?} has no slot — instance dropped"
                )
            }
        }
    }
}

impl std::fmt::Display for ScriptedEventDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "scripted event `{}` failed: {}",
            self.event_id.0, self.kind
        )
    }
}

// ── Implementation ────────────────────────────────────────────────────────────

impl<'a> ScriptedEventBoundaryHandler<'a> {
    /// Execute all compiled event definitions for one boundary tick.
    ///
    /// `threshold_events` is the slice of GPU-fired scripted-event thresholds
    /// resolved by the session/driver this tick. Pass `&[]` for ticks where
    /// no threshold fired.
    ///
    /// See the module-level doc for the full execution protocol.
    pub fn handle_tick(
        &self,
        threshold_events: &[ScriptedEventTriggerEvent],
        ctx: &mut ScriptedEventBoundaryContext<'_>,
    ) {
        tick_cooldowns(ctx);

        // Index threshold-fired events for O(1) lookup. Track unknown ids so
        // we can emit diagnostics for stale registrations.
        let mut fired_threshold_ids: HashSet<&str> = HashSet::with_capacity(threshold_events.len());
        for evt in threshold_events {
            fired_threshold_ids.insert(evt.event_id.as_str());
        }
        // Remove any ids that DO match a definition; what remains are unknowns.
        let mut unknown_ids: HashSet<&str> = fired_threshold_ids.clone();
        for def in self.definitions {
            unknown_ids.remove(def.id.0.as_str());
        }
        for event_id in unknown_ids {
            ctx.diagnostics.push(ScriptedEventDiagnostic {
                event_id: EventKey::new(event_id),
                kind: ScriptedEventDiagnosticKind::UnknownEventId,
            });
        }

        // Sort by priority descending: Critical(3) > High(2) > Normal(1) > Low(0).
        // `EventPriority` derives `Ord` with variants in ascending declaration order.
        let mut sorted: Vec<&ScriptedEventDefinition> = self.definitions.iter().collect();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));

        for def in sorted {
            if ctx.cooldowns.contains_key(&def.id) {
                continue;
            }

            let fire = match &def.trigger {
                CompiledTrigger::Predicate(predicate) => {
                    let eval_ctx = ScriptEvalContext {
                        registry: self.registry,
                        shadow: ctx.shadow,
                        n_dims: ctx.n_dims,
                        current_slot: ctx.current_slot,
                    };
                    match predicate.eval(&eval_ctx) {
                        Err(err) => {
                            ctx.diagnostics.push(ScriptedEventDiagnostic {
                                event_id: def.id.clone(),
                                kind: ScriptedEventDiagnosticKind::TriggerEvalError(err),
                            });
                            continue;
                        }
                        Ok(b) => b,
                    }
                }
                CompiledTrigger::Threshold(_) => fired_threshold_ids.contains(def.id.0.as_str()),
            };

            if fire {
                resolve_effects(def, ctx);
                if let Some(cooldown) = def.cooldown {
                    ctx.cooldowns.insert(def.id.clone(), cooldown.ticks);
                }
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Decrement every active cooldown by 1 and drop entries that reach 0.
fn tick_cooldowns(ctx: &mut ScriptedEventBoundaryContext<'_>) {
    ctx.cooldowns.retain(|_, remaining| {
        *remaining = remaining.saturating_sub(1);
        *remaining > 0
    });
}

/// Resolve `CompiledEffect` targets to `SimThingId`s and push `BoundaryRequest`s.
/// Effects whose target slot is missing in `slot_to_thing` push a diagnostic.
fn resolve_effects(def: &ScriptedEventDefinition, ctx: &mut ScriptedEventBoundaryContext<'_>) {
    for effect in &def.effects {
        let (scope, overlay_arm) = match effect {
            CompiledEffect::Remove { target } => (target, None),
            CompiledEffect::ActivateOverlay { target, overlay_id } => {
                (target, Some((true, *overlay_id)))
            }
            CompiledEffect::SuspendOverlay { target, overlay_id } => {
                (target, Some((false, *overlay_id)))
            }
        };

        let slot = match scope {
            ScopeRef::Current => ctx.current_slot,
            ScopeRef::Slot(s) => *s,
        };

        let Some(&target_id) = ctx.slot_to_thing.get(&slot) else {
            ctx.diagnostics.push(ScriptedEventDiagnostic {
                event_id: def.id.clone(),
                kind: ScriptedEventDiagnosticKind::UnresolvedEffectTarget { slot },
            });
            continue;
        };

        let request = match overlay_arm {
            None => BoundaryRequest::Remove { target: target_id },
            Some((true, oid)) => BoundaryRequest::ActivateOverlay {
                target: target_id,
                overlay_id: oid,
            },
            Some((false, oid)) => BoundaryRequest::SuspendOverlay {
                target: target_id,
                overlay_id: oid,
            },
        };
        ctx.requests.push(request);
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;
    use crate::spec::script::ScriptEvalError;

    #[test]
    fn scripted_event_diagnostic_kind_display_is_non_empty() {
        let kind = ScriptedEventDiagnosticKind::TriggerEvalError(ScriptEvalError::DivisionByZero);
        let text = format!("{kind}");
        assert!(!text.is_empty());
        assert!(text.contains("division by zero"));
    }

    #[test]
    fn scripted_event_diagnostic_display_includes_event_id() {
        let diagnostic = ScriptedEventDiagnostic {
            event_id: EventKey::new("low_loyalty"),
            kind: ScriptedEventDiagnosticKind::UnknownEventId,
        };
        let text = format!("{diagnostic}");
        assert!(text.contains("low_loyalty"));
        assert!(text.contains("unknown event id"));
    }

    #[test]
    fn scripted_event_diagnostic_unresolved_target_display_includes_slot() {
        let diagnostic = ScriptedEventDiagnostic {
            event_id: EventKey::new("spawn_rebel"),
            kind: ScriptedEventDiagnosticKind::UnresolvedEffectTarget { slot: 7 },
        };
        let text = format!("{diagnostic}");
        assert!(text.contains("spawn_rebel"));
        assert!(text.contains("slot 7"));
    }
}
