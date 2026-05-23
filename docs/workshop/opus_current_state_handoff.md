> **SUPERSEDED** — Pre-PR 9/10/11 snapshot. Use [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md).

# Opus Handoff - Current State After simthing-spec PRs 5-8

**Date:** 2026-05-22
**Prepared by:** Codex 5.5
**Target agent:** Claude Opus
**Current branch:** `master`
**Current head:** `d871518` (`docs: park simthing-spec PR 8 state`)
**Last code commit:** `8a8061c` (`simthing-spec PR 8: scripted event compiler templates.`)

This document is a live-state handoff after the capability runtime, preview,
Script IR, and event compiler-template slices landed. It is meant to prevent
the next agent from re-reading stale PR1 notes as current truth.

## Read First

Read in this order:

1. `docs/todo.md`
2. `docs/worklog.md`
3. `docs/invariants.md`
4. `docs/design_v6.md`
5. `docs/workshop/simthing_spec_master_handoff.md`
6. The current code in `crates/simthing-spec/src/`

Important: `docs/workshop/simthing_spec_master_handoff.md` is still valuable,
but PRs 2-8 have already consumed and amended parts of it. Treat current code
and this handoff as newer than old PR1-era divergence notes.

## Repository State

`master` and `origin/master` include:

- `d871518` - parking docs after PR 8.
- `8a8061c` - PR 8 trigger/effect/event compiler templates.
- `991e35d` - PR 7 canonical Script IR.
- `7fb1311` - PR 6 capability preview reports.
- `a0d3501` - PR 5 capability runtime boundary handler.
- `aac6d1f` - PR 4 capability unlock registration bridge.
- `f1dbfa1` - PR 3 `CapabilityTreeBuilder`.

Verification at parking:

- `cargo test --workspace` -> 277 passed, 1 ignored, zero warnings.
- `cargo build --workspace --tests` -> zero warnings.

Known untracked local files at handoff time:

- `.claude/worktrees/`
- `demo.replay.ldjson`
- `docs/workshop/simthing_modder_object_guide.md`

Do not delete or stage those unless explicitly asked.

## What Is Implemented

### PR 5: Capability Runtime State + Boundary Handler

Key files:

- `crates/simthing-spec/src/runtime/capability_state.rs`
- `crates/simthing-spec/src/boundary/capability_handler.rs`
- `crates/simthing-spec/tests/pr5_capability_handler.rs`

Main surfaces:

- `CapabilityTreeInstance`
- `CapabilityTreeState`
- `CapabilityTreeNotification`
- `CapabilityTreeDiagnostic`
- `CapabilityTreeBoundaryHandler`
- `CapabilityBoundaryContext`
- `CapabilityTreeError`

Behavior:

- Threshold events with `ThresholdSemantic::CapabilityUnlock` activate
  capability overlays.
- Failed prereqs reset progress to `research_cost - 0.01` and put the entry in
  runtime `ActivationMode::OnPrereqMet`.
- `sweep_on_prereq_met` runs a fixpoint sweep and activates newly satisfied
  runtime entries.
- `handle_player_selection` activates `PlayerSelection` entries directly.
- `max_active: Limited { count: 1, replacement: SuspendOldest }` suspends the
  oldest active sibling and emits `IdeaSwitched`.

Important implementation choice:

- Path A was taken for `max_active`.
- `CapabilityCategorySpec.max_active` is now `Option<MaxActivePolicy>`.
- `MaxActivePolicy::Limited` includes `replacement: ReplacementPolicy`.

### PR 6: Capability Preview + Mutual Exclusivity Completion

Key files:

- `crates/simthing-spec/src/preview/capability_preview.rs`
- `crates/simthing-spec/tests/pr6_capability_preview.rs`

Main surfaces:

- `preview_capability_effect`
- `CapabilityPreviewInput`
- `CapabilityPreviewReport`
- `CapabilityPreviewDelta`
- `CapabilityPreviewOverlayBreakdown`

Behavior:

- Preview is definition-only and CPU-only.
- `CapabilityDefinition.effect_transforms` is parallel to `overlay_ids` and
  `effect_keys`, so preview does not need the template `SimThing`.
- PR 6 also verifies the PR 5 activate/suspend request flow against real
  structural overlay lifecycle mutation.

### PR 7: Canonical Script IR + CPU Evaluator

Key files:

- `crates/simthing-spec/src/spec/script.rs`
- `crates/simthing-spec/tests/pr7_script_ir.rs`

Main surfaces:

- `PropertyKey`
- `ScopeRef`
- `ScriptExpr`
- `ScriptPredicate`
- `ScriptEvalContext`
- `ScriptEvalError`

Behavior:

- Expressions support constants, property reads, arithmetic, min/max, clamp,
  division, and predicate gates.
- Predicates support true/false, greater/less/equalish, and/or/not.
- Evaluation reads dense shadow values through `DimensionRegistry` and
  `n_dims`.
- Scope is intentionally minimal: `Current` or explicit `Slot(u32)`.

Out of scope:

- No EML backend.
- No parser.
- No derived-field integration.
- No GPU evaluator.
- No symbolic owner/faction scope resolution.

### PR 8: Trigger/Effect/Event Compiler Templates

Key files:

- `crates/simthing-spec/src/spec/trigger.rs`
- `crates/simthing-spec/src/spec/effect.rs`
- `crates/simthing-spec/src/spec/event.rs`
- `crates/simthing-spec/src/runtime/compiled_trigger.rs`
- `crates/simthing-spec/src/runtime/compiled_effect.rs`
- `crates/simthing-spec/src/runtime/scripted_event_definition.rs`
- `crates/simthing-spec/src/compile/trigger.rs`
- `crates/simthing-spec/src/compile/effect.rs`
- `crates/simthing-spec/src/compile/event.rs`
- `crates/simthing-spec/tests/pr8_event_compiler.rs`

Main surfaces:

- `TriggerSpec`
- `EffectSpec`
- `EventSpec`
- `CooldownSpec`
- `EventPriority`
- `CompiledTrigger`
- `CompiledThresholdTrigger`
- `CompiledEffect`
- `ScriptedEventDefinition`
- `compile_trigger`
- `compile_effect`
- `compile_event`

Behavior:

- Threshold triggers resolve `PropertyKey + SubFieldRole` to
  `SimPropertyId + col`.
- Predicate triggers preserve `ScriptPredicate`.
- Effects compile to boundary request templates:
  `Remove`, `ActivateOverlay`, `SuspendOverlay`.
- Events combine one trigger, a vector of effects, optional cooldown, and
  priority.

Out of scope:

- No event runner.
- No threshold registry upload for scripted event thresholds.
- No parser or EML.
- No AddChild/Reparent effect payloads.
- No event cooldown state.
- No boundary event handler.

## Recommended Next Slice

The most natural next PR is a PR 9 that executes compiled event definitions at
boundary time, but keep it narrow:

1. Add a CPU-only event boundary handler in `simthing-spec`, likely under
   `src/boundary/event_handler.rs`.
2. Evaluate `CompiledTrigger::Predicate` directly with `ScriptEvalContext`.
3. For `CompiledTrigger::Threshold`, either:
   - accept already-fired `ThresholdEvent`s and match them against compiled
     threshold metadata, or
   - explicitly defer threshold-triggered scripted events until threshold
     semantic layering is cleaned up.
4. Resolve `ScopeRef` to concrete `SimThingId` and slot.
5. Translate `CompiledEffect` into `BoundaryRequest`s.
6. Add per-event runtime state for cooldowns if cooldown behavior is included.

Suggested PR 9 acceptance tests:

- Predicate event emits `BoundaryRequest::Remove`.
- Predicate false emits no requests.
- `ScopeRef::Current` resolves to the caller/current slot.
- `ScopeRef::Slot(n)` resolves to the corresponding `SimThingId` through a
  supplied slot/id map.
- Unknown slot or missing target produces diagnostics, not a panic.
- Effect order is preserved.
- Cooldown, if implemented, suppresses repeat firing until elapsed.
- Priority ordering, if implemented, runs `Critical > High > Normal > Low`.

Do not combine PR 9 with session/driver assembly unless explicitly asked. That
would make the dependency and ownership questions too wide.

## Caveats And Latent Footguns

### 1. `simthing-spec` now depends on `simthing-sim` and `simthing-gpu`

PR 5 took the pragmatic path:

- `simthing-spec -> simthing-sim` for `ThresholdRegistry` and
  `ThresholdSemantic`.
- `simthing-spec -> simthing-gpu` for `ThresholdEvent`.

This works today, but it is not an ideal long-term crate graph. Before real
driver/session assembly hardens, move the threshold semantic surface into a
lower crate, probably `simthing-feeder` or a small shared crate.

Do not add more upward dependencies from `simthing-spec` unless there is a very
clear reason.

### 2. The parked docs mention `8a8061c`, but current head is `d871518`

`8a8061c` is the last code commit. `d871518` is a docs-only parking commit.
Use `d871518` when checking branch parity. Use `8a8061c` when comparing code
state before the handoff docs.

### 3. `simthing-spec/src/lib.rs` still has PR1 crate-level prose

The module exports are current, but the crate doc comment still says PR 1
"intentionally contains authoring data structures only" and lists many things
that now exist. This is stale documentation. Update it in a doc cleanup PR or
as part of the next spec PR if you touch crate docs.

### 4. Threshold event handling and scripted event handling are not the same

Capability unlocks already flow through `ThresholdSemantic::CapabilityUnlock`
and `CapabilityTreeBoundaryHandler`.

PR 8 scripted event threshold triggers are only compiled templates. They are
not registered with `ThresholdBuilder`, do not get `event_kind`s, and are not
uploaded to the GPU threshold buffer.

If PR 9 executes predicate events, do not imply threshold events are also wired
unless you actually add the threshold registration path.

### 5. `ScopeRef::Slot(u32)` is a raw slot, not a `SimThingId`

PR 7 kept scope intentionally small. PR 8 effects store `ScopeRef`, while
`BoundaryRequest`s need `SimThingId`.

Any event runner must receive enough context to resolve:

- `ScopeRef::Current` -> current slot -> current `SimThingId`
- `ScopeRef::Slot(n)` -> slot -> `SimThingId`

The current `SlotAllocator` maps ids to slots, but the reverse mapping may not
be exposed in the exact form you want. Do not fake it by casting slot numbers
to ids.

### 6. Dense shadow reads must respect `n_dims`

All shadow reads follow:

```text
shadow[slot * n_dims + col]
```

PR 7 checks slot and column bounds. PR 5 prereq checks use `get(idx)` and treat
missing cells as prereq failure. Keep that distinction deliberate: script eval
returns hard errors; capability prereqs are defensive and non-panicking.

### 7. Boundary requests do not apply immediately

`BoundaryRequest::ActivateOverlay` and `SuspendOverlay` mutate overlay
lifecycle at the boundary. The effect appears in the next tick's pass order.
Do not write mid-tick values in PR 9 unless you are explicitly implementing a
continuous patch path.

### 8. `OverlayId` is nondeterministic

`OverlayId::new()` uses an atomic counter. Tests should not assert stable raw
ids across independent builds. Capability tests use `CapabilityEffectKey` or
look up ids from the built definition.

### 9. `DimensionRegistry::register` panics on duplicates

Use `compile_property`, which checks `id_of` first and returns
`SpecError::DuplicateProperty`. Do not directly register duplicate fixture
properties in tests unless the panic is the thing under test.

### 10. Capability categories have no separate authored `id`

The current identity is `CategoryKey { namespace, name }`, generally from
`property_namespace::property_name`. Old handoff notes mention a category `id`;
that is not the current code shape.

### 11. Authored `ActivationMode::OnPrereqMet` is invalid

`OnPrereqMet` exists for runtime state only. Validation rejects it as an
authored default. Do not add fixtures that author `OnPrereqMet`.

### 12. `research_rate` is vestigial

Capability runtime reads `research_cost: f32` for threshold/reset behavior.
`research_rate: ResearchRateSpec` remains in the authoring struct but is not
the runtime value. Avoid "fixing" this by renaming `research_cost`; that would
be serde-breaking.

### 13. `max_active` v0 only supports unlimited or `Limited(1)`

`ReplacementPolicy::SuspendOldest` is the implemented v0 policy.
`Limited { count: n }` where `n != 1` is rejected/unsupported. Do not write a
handler path that silently accepts larger counts without a real design.

### 14. `CapabilityBoundaryContext.instances` is keyed by owner id

Threshold events carry the tree `SimThingId`, not the owner id. PR 5 resolves
this by scanning `ctx.instances.values()` for `tree_thing_id`.

This is fine for v0 but awkward for session assembly. If many instances appear,
consider adding a reverse lookup instead of repeatedly scanning.

### 15. `sweep_on_prereq_met` uses a fixpoint loop

Activating one entry can satisfy another entry's prereqs. Preserve the
candidate-collection pattern before mutating `activation_mode_by_entry`.
Naive recursive mutation will fight the borrow checker and can duplicate
requests.

### 16. Duplicate activation semantics are asymmetric

For unlimited categories, PR 5 avoids pushing duplicate active entries into
`active_by_category`, but `emit_activation` pushes `ActivateOverlay` requests
before checking whether the entry is already active.

For `Limited(1)`, duplicate active entries return early after the activate
request has already been emitted. The structural mutation is a no-op if the
overlay is already active, but tests that count requests can be surprised.
If PR 9 needs strict idempotent event firing, add an explicit policy instead of
assuming current capability behavior is request-idempotent.

### 17. `CompiledEffect` is a template, not yet executable

`CompiledEffect::Remove { target }` still contains `ScopeRef`, not
`SimThingId`. It must be resolved before creating `BoundaryRequest::Remove`.

Likewise overlay effects need a resolved target id and should preserve the
authored `OverlayId`.

### 18. Predicate event execution needs error policy

Script evaluation can fail on:

- unknown property
- unknown role
- slot out of bounds
- column out of bounds
- division by zero
- invalid clamp

Decide whether an event handler returns hard errors or records diagnostics and
continues. Existing pattern: authored/compile errors are hard; runtime data
inconsistencies often become diagnostics if the handler can keep going.

### 19. Cooldown semantics are not designed beyond storage

`CooldownSpec { ticks }` is compiled into `ScriptedEventDefinition`, but no
runtime state exists for last-fired tick, next-eligible tick, or per-owner
cooldowns. Do not pretend cooldown works until you add state and tests.

### 20. Priority is stored but not executed

`EventPriority` derives ordering, but no scheduler uses it. If PR 9 fires a
set of events, define whether priority affects evaluation order, effect order,
conflict resolution, or only display.

## Suggested PR 9 Data Shapes

Keep these local to a new `boundary/event_handler.rs` unless they prove shared:

```rust
pub struct ScriptedEventBoundaryHandler<'a> {
    pub registry: &'a DimensionRegistry,
    pub definitions: &'a [ScriptedEventDefinition],
}

pub struct ScriptedEventBoundaryContext<'a> {
    pub n_dims: usize,
    pub shadow: &'a [f32],
    pub current_slot: u32,
    pub slot_to_thing: &'a HashMap<u32, SimThingId>,
    pub requests: &'a mut Vec<BoundaryRequest>,
    pub diagnostics: &'a mut Vec<ScriptedEventDiagnostic>,
}
```

Open question: if cooldowns are in scope, add mutable state keyed by
`EventKey`, possibly also by owner/current target depending on intended
semantics.

## Recommended Commands

Fast checks while developing:

```powershell
cargo test -p simthing-spec --test pr8_event_compiler
cargo test -p simthing-spec --test pr7_script_ir
cargo test -p simthing-spec
```

Full parking checks before commit:

```powershell
cargo test --workspace
cargo build --workspace --tests
git status --short --branch
```

## Definition Of Done For The Next Slice

For a PR 9 event executor:

- CPU-only unit tests cover predicate execution and effect emission.
- Scope resolution is explicit and tested.
- Runtime eval errors have a documented policy.
- Cooldown and priority are either implemented with tests or explicitly
  documented as out of scope.
- No new crate dependency cycle is introduced.
- `docs/todo.md` and `docs/worklog.md` are updated.
- `cargo test --workspace` and `cargo build --workspace --tests` pass with
  zero warnings.

