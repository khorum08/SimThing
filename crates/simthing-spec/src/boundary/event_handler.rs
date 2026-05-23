//! Boundary-time execution of compiled scripted event definitions.
//!
//! [`ScriptedEventBoundaryHandler`] is called once per boundary tick. It
//! handles **predicate-triggered events only**. Threshold-triggered scripted
//! events are deferred to a later PR that will upload `CompiledThresholdTrigger`
//! data to `ThresholdBuilder` and receive GPU `ThresholdEvent`s — do not add
//! shadow-polling as a substitute for that path.
//!
//! ## Execution protocol per call to [`ScriptedEventBoundaryHandler::handle_tick`]
//!
//! 1. **Tick cooldowns** — decrement every active cooldown counter by 1 and
//!    drop entries that reach 0.
//! 2. **Sort by priority** — process definitions descending
//!    (Critical → High → Normal → Low). [`EventPriority`] derives `Ord`.
//! 3. **Skip on cooldown** — if `ctx.cooldowns` contains the event's key, skip.
//! 4. **Skip threshold triggers** — `CompiledTrigger::Threshold` events are not
//!    evaluated here; they produce no diagnostic, just a no-op.
//! 5. **Evaluate predicate** — call [`ScriptPredicate::eval`] with a
//!    [`ScriptEvalContext`] built from `ctx`. Hard eval errors push a
//!    [`ScriptedEventDiagnostic`] and skip the event.
//! 6. **Resolve and emit effects** — for each [`CompiledEffect`], resolve the
//!    [`ScopeRef`] target against `ctx.slot_to_thing`. Missing slots push a
//!    `ScriptedEventDiagnostic`; resolved targets push a [`BoundaryRequest`].
//! 7. **Arm cooldown** — if the event has a `cooldown`, insert its `ticks`
//!    value into `ctx.cooldowns`.

use crate::runtime::{CompiledEffect, CompiledTrigger, ScriptedEventDefinition};
use crate::spec::event::EventKey;
use crate::spec::script::{ScriptEvalContext, ScriptEvalError, ScopeRef};
use simthing_core::{DimensionRegistry, SimThingId};
use simthing_feeder::BoundaryRequest;
use std::collections::HashMap;

// ── Handler ───────────────────────────────────────────────────────────────────

/// Stateless handler that executes compiled scripted event definitions each
/// boundary tick. Handles predicate triggers only; threshold-triggered events
/// are a future GPU-path concern.
pub struct ScriptedEventBoundaryHandler<'a> {
    pub registry:    &'a DimensionRegistry,
    /// All event definitions to evaluate. The handler sorts internally by
    /// [`EventPriority`]; the caller does not need to pre-sort.
    pub definitions: &'a [ScriptedEventDefinition],
}

// ── Context ───────────────────────────────────────────────────────────────────

/// Per-tick mutable state threaded through event execution.
pub struct ScriptedEventBoundaryContext<'a> {
    /// Number of property dimensions in the shadow buffer.
    pub n_dims:         usize,
    /// Flat CPU shadow buffer: `shadow[slot * n_dims .. (slot + 1) * n_dims]`.
    pub shadow:         &'a [f32],
    /// The "home" slot for `ScopeRef::Current` in both trigger evaluation and
    /// effect resolution.
    pub current_slot:   u32,
    /// Maps raw slot index (u32) → [`SimThingId`] for resolving [`ScopeRef`]
    /// targets in [`CompiledEffect`]s. A missing key means the slot is
    /// unoccupied; the effect is skipped with a diagnostic.
    pub slot_to_thing:  &'a HashMap<u32, SimThingId>,
    /// Remaining-ticks map for events currently on cooldown. The handler ticks
    /// this down at the start of each call and skips events whose key is still
    /// present. Entries are removed when they reach 0. Keyed by [`EventKey`]
    /// only (not per-owner). For per-owner semantics, callers maintain separate
    /// context instances.
    pub cooldowns:      &'a mut HashMap<EventKey, u32>,
    /// Accumulates [`BoundaryRequest`]s emitted by triggered event effects.
    pub requests:       &'a mut Vec<BoundaryRequest>,
    /// Accumulates soft errors from trigger evaluation and effect resolution.
    /// A diagnostic here does not abort the tick; subsequent events still run.
    pub diagnostics:    &'a mut Vec<ScriptedEventDiagnostic>,
}

// ── Diagnostic ────────────────────────────────────────────────────────────────

/// A non-fatal error produced during scripted event execution.
#[derive(Clone, Debug)]
pub struct ScriptedEventDiagnostic {
    /// The event that encountered an error.
    pub event_id: EventKey,
    /// The specific error.
    pub kind:     ScriptedEventDiagnosticKind,
}

#[derive(Clone, Debug)]
pub enum ScriptedEventDiagnosticKind {
    /// Predicate evaluation failed at runtime.
    TriggerEvalError(ScriptEvalError),
    /// An effect's `ScopeRef` target resolved to a slot with no entry in
    /// `slot_to_thing`.
    UnresolvedEffectTarget { slot: u32 },
}

// ── Implementation ────────────────────────────────────────────────────────────

impl<'a> ScriptedEventBoundaryHandler<'a> {
    /// Execute all compiled event definitions for one boundary tick.
    ///
    /// See the module-level doc for the full execution protocol.
    pub fn handle_tick(&self, ctx: &mut ScriptedEventBoundaryContext<'_>) {
        tick_cooldowns(ctx);

        // Sort by priority descending: Critical(3) > High(2) > Normal(1) > Low(0).
        // `EventPriority` derives `Ord` with variants in ascending declaration order.
        let mut sorted: Vec<&ScriptedEventDefinition> = self.definitions.iter().collect();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));

        for def in sorted {
            if ctx.cooldowns.contains_key(&def.id) {
                continue;
            }

            // Threshold triggers are deferred to the GPU-path PR; skip silently.
            let CompiledTrigger::Predicate(predicate) = &def.trigger else {
                continue;
            };

            let eval_ctx = ScriptEvalContext {
                registry:     self.registry,
                shadow:       ctx.shadow,
                n_dims:       ctx.n_dims,
                current_slot: ctx.current_slot,
            };

            match predicate.eval(&eval_ctx) {
                Err(err) => {
                    ctx.diagnostics.push(ScriptedEventDiagnostic {
                        event_id: def.id.clone(),
                        kind:     ScriptedEventDiagnosticKind::TriggerEvalError(err),
                    });
                }
                Ok(false) => {}
                Ok(true) => {
                    resolve_effects(def, ctx);
                    if let Some(cooldown) = def.cooldown {
                        ctx.cooldowns.insert(def.id.clone(), cooldown.ticks);
                    }
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
fn resolve_effects(
    def: &ScriptedEventDefinition,
    ctx: &mut ScriptedEventBoundaryContext<'_>,
) {
    for effect in &def.effects {
        let (scope, overlay_arm) = match effect {
            CompiledEffect::Remove          { target }             => (target, None),
            CompiledEffect::ActivateOverlay { target, overlay_id } => (target, Some((true,  *overlay_id))),
            CompiledEffect::SuspendOverlay  { target, overlay_id } => (target, Some((false, *overlay_id))),
        };

        let slot = match scope {
            ScopeRef::Current  => ctx.current_slot,
            ScopeRef::Slot(s)  => *s,
        };

        let Some(&target_id) = ctx.slot_to_thing.get(&slot) else {
            ctx.diagnostics.push(ScriptedEventDiagnostic {
                event_id: def.id.clone(),
                kind:     ScriptedEventDiagnosticKind::UnresolvedEffectTarget { slot },
            });
            continue;
        };

        let request = match overlay_arm {
            None               => BoundaryRequest::Remove { target: target_id },
            Some((true,  oid)) => BoundaryRequest::ActivateOverlay { target: target_id, overlay_id: oid },
            Some((false, oid)) => BoundaryRequest::SuspendOverlay  { target: target_id, overlay_id: oid },
        };
        ctx.requests.push(request);
    }
}
