# Scripted Event Scope Model

**Date:** 2026-05-23
**Status:** Accepted (O4 implementation landed)
**Blocks:** O4 (per-owner scripted events) — ✅ landed
**Related:** [`game_mode_session_installation.md`](game_mode_session_installation.md), [`spec_session_state_replay.md`](spec_session_state_replay.md)

**Implementation notes:** Landed essentially as proposed. Two API split clarifications
discovered during integration: (1) `SpecSessionState` exposes
`register_scripted_event_definition(def) → ScriptedEventDefinitionId` and
`attach_scripted_event_instance(def_id, event_id, owner, slot)` separately so
install can register one definition and attach N per-owner instances pointing
at it. The convenience `add_scripted_event_instance(def, owner, slot)` wraps both
for single-instance use; the back-compat `add_scripted_event(def)` keeps PR 11
tests working by attaching one instance against a new
`session_root_owner` field. (2) `EventSpec.install` defaults to `SessionRoot`
(`#[serde(default)]`), so existing event RON files install as a single
instance on `Scenario::root.id` — matching pre-O4 behavior verbatim.

## Context

In PR 11 Track A the driver added scripted-event support with a deliberately
narrow scope model: a session-global `scripted_current_slot: u32` on
`SpecSessionState`. Every `ScriptedEventDefinition` resolves
`ScopeRef::Current` through that single slot at boundary time. Cooldowns are
similarly global — `HashMap<EventKey, u32>`. See
[`spec_session.rs`](../../crates/simthing-driver/src/spec_session.rs):45–46,
[`scripted_event_definition.rs`](../../crates/simthing-spec/src/runtime/scripted_event_definition.rs):27–46.

This was correct for PR 11's E2E proof (one world, one slot, one event) and
is documented as "session-global" in the PR 11 ADR. It does not survive
contact with realistic authoring:

- Two factions both want a `low_loyalty` event. Today they share a single
  cooldown bucket; firing for faction A blocks faction B.
- `ScopeRef::Current` resolves to the same slot regardless of which entity
  the event "belongs to" — so per-owner predicates either fight over
  `scripted_current_slot` or must use `ScopeRef::Slot(literal)` and lose
  authoring portability.
- Cooldowns are not per-instance, so an event tuned for "once per faction
  per N ticks" cannot be authored at all.

The session-installation ADR explicitly defers scripted-event ownership to
this document. The capability-tree code path already solved the analogous
problem with `CapabilityInstanceKey { owner_id, definition_id, tree_thing_id }`
and per-instance `CapabilityTreeState` storage.
[`spec_session.rs`](../../crates/simthing-driver/src/spec_session.rs):20–35.

The forces shaping this ADR:

1. Two-owner correctness — independent cooldown state per owner is
   non-negotiable for any non-trivial mod.
2. `ScopeRef::Current` semantics must stay authorable (a designer writes
   "self" once, gets the right slot regardless of who installs it).
3. The threshold registration path is GPU-bound: each per-owner instance
   that authors a threshold trigger needs its own `ScriptedEventTriggerRegistration`
   pointing at the owner's slot.
4. `simthing-sim` stays spec-free. All per-owner storage lives in
   `simthing-driver::SpecSessionState`.
5. Replay needs cooldown state to round-trip — see the
   [replay ADR](spec_session_state_replay.md) for the serialization story.

## Decision

**Adopt Option B: per-owner `ScriptedEventInstance` keyed by owner +
definition.** Cooldowns and `current_slot` become per-instance fields.
`ScopeRef::Current` resolves through the per-instance slot. The handler
runs once per instance per boundary.

This mirrors the capability-instance pattern almost exactly, and is what
the handoff explicitly recommended.

### 1. Definition stays shared; instance is new

```rust
// crates/simthing-spec/src/runtime/scripted_event_definition.rs (unchanged shape)
pub struct ScriptedEventDefinition {
    pub id:       EventKey,
    pub trigger:  CompiledTrigger,
    pub effects:  Vec<CompiledEffect>,
    pub cooldown: Option<CooldownSpec>,
    pub priority: EventPriority,
}
```

```rust
// crates/simthing-driver/src/spec_session.rs (new)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScriptedEventInstanceKey {
    pub owner_id: SimThingId,
    pub event_id: EventKey,
}

#[derive(Debug)]
pub struct ScriptedEventInstance {
    pub key: ScriptedEventInstanceKey,
    pub definition_id: ScriptedEventDefinitionId, // new id alongside EventKey
    pub current_slot: u32,        // resolved at install; rebuilt on slot churn
    pub cooldown_remaining: u32,  // 0 when ready to fire
}
```

`ScriptedEventDefinitionId` (atomic-allocated, parallel to
`CapabilityTreeDefinitionId`) lets the driver store one definition once and
have many instances point at it. `EventKey` stays as the authored stable id
for diagnostics and cooldown introspection.

### 2. Storage on `SpecSessionState`

Replace the three flat fields:

```rust
// before
pub scripted_events: Vec<ScriptedEventDefinition>,
pub scripted_cooldowns: HashMap<EventKey, u32>,
pub scripted_current_slot: u32,
```

with:

```rust
// after
pub scripted_event_definitions: HashMap<ScriptedEventDefinitionId, ScriptedEventDefinition>,
pub scripted_event_instances: HashMap<ScriptedEventInstanceKey, ScriptedEventInstance>,
```

The handler walks `scripted_event_instances`, resolves the definition by id,
and invokes the existing PR 9/10 boundary handler with a per-instance
`ScriptedEventBoundaryContext { current_slot, cooldowns: &mut single-entry,
… }`. The handler is unchanged structurally — it operates on one (definition,
instance) pair at a time.

### 3. Install paths

The session-installation ADR's `compile_and_install` gains a third step
after capability tree install:

```rust
for event_spec in scripted_events(game_mode) {
    let definition = compile_event(event_spec, registry, …)?;
    let definition_id = ScriptedEventDefinitionId::new();
    state.scripted_event_definitions.insert(definition_id, definition);

    let owners = resolve_install_target(&event_spec.install, scenario, root);
    for owner_id in owners {
        let slot = allocator.slot_of(owner_id)
            .ok_or(InstallError::OwnerHasNoSlot { owner_id })?;
        let key = ScriptedEventInstanceKey { owner_id, event_id: event_spec.id.clone().into() };
        state.scripted_event_instances.insert(key, ScriptedEventInstance {
            key, definition_id, current_slot: slot, cooldown_remaining: 0,
        });
    }
}
```

`EventSpec` gains the same `#[serde(default)] install: InstallTargetSpec`
field used by `CapabilityTreeSpec`. The default for scripted events is
`InstallTargetSpec::SessionRoot` (matches today's behavior — one instance
on the world).

### 4. GPU threshold registrations: one per instance

`ScriptedEventDefinition::to_trigger_registration(current_slot)` is the
existing per-instance escape hatch — see
[`scripted_event_definition.rs`](../../crates/simthing-spec/src/runtime/scripted_event_definition.rs):27–46.
`SpecSessionState::scripted_event_trigger_registrations()` is rewritten:

```rust
pub fn scripted_event_trigger_registrations(&self) -> Vec<ScriptedEventTriggerRegistration> {
    self.scripted_event_instances
        .values()
        .filter_map(|inst| {
            let def = self.scripted_event_definitions.get(&inst.definition_id)?;
            def.to_trigger_registration(inst.current_slot)
        })
        .collect()
}
```

A threshold event fired by GPU Pass 7 carries an `event_id` (the
`EventKey`'s string). The handler looks up matching instances by `event_id`
and dispatches each. Two instances on different slots produce two
registrations — distinguished by their slot in the registration payload.
The driver disambiguates instances by walking `scripted_event_instances`
and matching `current_slot` against the fired registration's slot.

### 5. `ScopeRef::Current` semantics

`ScopeRef::Current` resolves through `inst.current_slot` for the running
instance — not through a session-global. `ScopeRef::Slot(n)` continues to
work verbatim. Authoring impact: writing `Current` means "the entity this
event is installed on," which is the intuitive read.

### 6. Slot churn handling

If `owner_id`'s slot is reallocated mid-session (fission, removal), the
instance's `current_slot` goes stale. PR 11 already handles this for
capability instances by rebuilding `slot_to_thing` from the allocator each
boundary — same approach here. Add a `refresh_scripted_event_slots(&mut self,
allocator: &SlotAllocator)` step at the top of `run_scripted_event_handler`:

```rust
fn refresh_scripted_event_slots(&mut self, allocator: &SlotAllocator) {
    let mut stale = Vec::new();
    for inst in self.scripted_event_instances.values_mut() {
        match allocator.slot_of(inst.key.owner_id) {
            Some(slot) => inst.current_slot = slot,
            None => stale.push(inst.key),
        }
    }
    for key in stale {
        self.scripted_event_instances.remove(&key);
        self.scripted_event_diagnostics.push(
            ScriptedEventDiagnostic::owner_removed(key.owner_id, key.event_id),
        );
    }
}
```

A new `ScriptedEventDiagnosticKind::OwnerRemoved` variant captures the
event; the handler does not silently keep firing against a stale slot.

If the threshold registration set changed (an instance dropped), the
driver bumps the threshold config revision via the same
`sync_spec_threshold_registrations` path used at install time.

### 7. Cooldown semantics

Cooldown lives on the instance: `cooldown_remaining: u32`. The handler
decrements it each boundary, fires on 0 (plus trigger predicate true), and
resets to `definition.cooldown.ticks` after firing. The shared
`HashMap<EventKey, u32>` goes away. Two instances of the same event have
independent cooldowns by construction.

## Consequences

(a) **Migration is mechanical.** Today's PR 11 Track A test
(`scripted_event_handler_runs_from_spec_session_state`) constructs the
state with `set_scripted_current_slot(slot)` + `add_scripted_event(event)`.
After this ADR, the equivalent is one helper that creates a definition +
single `SessionRoot` instance. A backwards-compat shim
`SpecSessionState::add_scripted_event(definition)` can keep installing one
instance against `Scenario::root.id` until tests migrate.

(b) **Replay surface widens.** `scripted_event_instances` is authoritative
mutable state (cooldowns mutate per boundary). The
[replay ADR](spec_session_state_replay.md) classifies it as
"authoritative mutable" and serializes per-instance `cooldown_remaining`.
The instance set itself reconstructs from spec + install targets, so only
the cooldown field needs frame-by-frame coverage.

(c) **Threshold registrations multiply with owner count.** N factions × M
threshold-triggered events = N×M registrations on every full GPU sync. Same
shape as capability unlocks today; existing append-only optimization (S5)
applies if instance churn becomes a problem.

(d) **Two diagnostics paths converge.** `ScriptedEventDiagnostic` already
exists for handler-side issues. The new `OwnerRemoved` variant slots in
cleanly; no new top-level diagnostic enum needed.

(e) **`scripted_current_slot` is gone from `SpecSessionState`'s public
surface.** Existing `set_scripted_current_slot` callers must move to
per-instance install. The PR 11 test cited above and any downstream code
that touched this field need a one-line migration.

(f) **The `EventKey` → `ScriptedEventDefinitionId` split mirrors the
capability story.** Logical `EventKey` is the authored stable name (used
in cooldown UI, diagnostics, RON references). `ScriptedEventDefinitionId`
is the runtime atomic id (used for `definition_id` foreign keys). Both are
needed; the capability case proved the pattern.

(g) **`InstallTargetSpec::SessionRoot` carries scripted events through PR
11 equivalence.** Authored events without explicit install land on
`Scenario::root.id`. The old behavior is the new default.

## Alternatives considered

### Option A: Keep global; pass owner-slot explicitly to `handle_tick`

Change `ScriptedEventBoundaryContext::current_slot` from a field to a
runtime arg, and have the driver loop:

```rust
for owner in factions {
    ctx.current_slot = owner.slot;
    handler.handle_tick(events_for_slot(owner), &mut ctx);
}
```

**Rejected.** Three problems:
- Cooldowns are still global — firing for faction A still blocks faction B
  unless we add per-owner cooldown maps, at which point we are most of the
  way to Option B with a less coherent data model.
- Predicate triggers re-evaluate against the swapped slot, but the
  *definition list* is shared — the loop fires every authored event for
  every faction, even when an event should belong to only one. There is no
  install target to filter against.
- Threshold registrations still emit once per definition with the global
  slot; per-faction thresholds need per-faction registrations, which means
  per-faction state, which means Option B.

### Option C: Extend `ScopeRef` to include symbolic owner scopes

Add `ScopeRef::Owner` / `ScopeRef::OwnerOf(SimThingId)`. The handler
resolves `Owner` against the firing instance.

**Rejected as the lead model**, but the *enum extension itself is
compatible* with Option B and may land later for authoring ergonomics.
`ScopeRef::Owner` literally meaning "the owner of the instance running this
event" only makes sense once instances exist (Option B). Adding it now,
without per-owner instances, recreates the global-slot ambiguity — what
does "owner" resolve to in a global event?

The recommended forward path: ship Option B as-is, then add
`ScopeRef::Owner` as a syntactic convenience for "the instance's owner_id
as a SimThingId, resolved through allocator at read time" once a concrete
authoring need appears.

## V0 Scope (what O4 implements)

- `ScriptedEventInstanceKey`, `ScriptedEventInstance`,
  `ScriptedEventDefinitionId`.
- `SpecSessionState` storage migration (remove three flat fields, add two
  maps).
- `EventSpec.install: InstallTargetSpec` with `SessionRoot` default.
- `compile_and_install` integration (creates one instance per resolved
  owner).
- Per-instance cooldown semantics.
- Slot refresh + `OwnerRemoved` diagnostic.
- One integration test: two factions, one shared `low_loyalty` event spec,
  firing in faction A leaves faction B's cooldown untouched and vice versa.

## Out of scope (deferred)

- `ScopeRef::Owner` symbolic scope (Option C add-on).
- Cross-owner events ("any faction with loyalty < 0.3"). Either author N
  per-owner instances and OR the results, or add a `SessionAggregate`
  install target later.
- Event priority ordering across instances. Current PR 9 priority remains
  per-instance; cross-instance ordering at the same priority is
  unspecified. Document as a v0 limitation.
- Replay serialization of cooldowns — that is the replay ADR's job.
