> **SUPERSEDED** — PR 11 Track A and B complete. Use [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md).

# PR 11 — Session / Driver Assembly Handoff

**Date:** 2026-05-22
**Prepared by:** Claude Opus 4.7
**Master HEAD:** `3e4f6ea` (`simthing-spec PR 10: scripted event GPU threshold path.`)
**Test baseline:** `cargo test --workspace` → 298 passed, 1 ignored, zero warnings.
**Crate boundary:** `simthing-spec` production deps are `simthing-core` + `simthing-feeder` only.

This document splits the upcoming work into two tracks:

- **Track A (Opus):** the architectural problem — who owns runtime state, when
  handlers run, how the boundary protocol grows. Genuine design work, not
  pattern-matching.
- **Track B (Composer 2.5):** narrowly-scoped mechanical work that can land
  *in parallel* without conflicting with Track A. Each task in Track B has a
  concrete file list, type signatures, acceptance criteria, and a hard
  "do not touch" list.

---

## Track A — Opus: session/driver assembly

### The actual problem

`simthing-spec` now exposes three boundary handlers that *exist but are not called*:

1. `CapabilityTreeBoundaryHandler::handle_capability_unlock_events(&[CapabilityUnlockEvent], &mut ctx)`
2. `CapabilityTreeBoundaryHandler::handle_player_selection(owner, entry_key, &mut ctx)`
3. `ScriptedEventBoundaryHandler::handle_tick(&[ScriptedEventTriggerEvent], &mut ctx)`

Today nothing wires them into the day-boundary protocol. There is no owner for:

- `HashMap<SimThingId, CapabilityTreeInstance>` — per-owner capability instances
- `HashMap<SimThingId, CapabilityTreeState>` — per-owner runtime state
- `HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>` — shared defs
- `Vec<ScriptedEventDefinition>` — scripted event definitions
- `HashMap<EventKey, u32>` — scripted-event cooldowns (per owner? global?)
- `HashMap<u32, SimThingId>` — `slot_to_thing` map for `ScopeRef` resolution

The current `simthing-sim::BoundaryProtocol` does not know about any of these.
The current `simthing-driver` is a thin glue layer around feeder + sim.

### Design questions Opus must answer

1. **Where does session state live?** Pick one:
   - Extend `simthing-driver` with a `SessionState` struct
   - Add a new `simthing-session` crate above `simthing-driver`
   - Embed in `simthing-sim::BoundaryProtocol` (likely a layering mistake — sim
     would then depend on `simthing-spec`, reversing the cleanup just landed)

2. **Boundary protocol step order.** Capability unlocks fire after Pass 7,
   before fission/fusion (so newly-unlocked capabilities can affect overlay
   activation before the next tick's overlays are applied). Where exactly does
   the scripted event handler slot in? Before or after capability unlocks?
   Before or after fission? Decide and document.

3. **GPU event drain plumbing.** `DispatchCoordinator::tick()` returns
   `TickOutcome`. The session needs to:
   - Drain `outcome.threshold_events`
   - Call `ThresholdRegistry::extract_capability_unlocks` → spec handler
   - Call `ThresholdRegistry::extract_scripted_event_triggers` → spec handler
   - Decide what happens to events that match neither (the existing
     `FissionTrigger` / `FusionTrigger` / `PropertyExpiry` paths stay in sim)

4. **Per-owner vs global scripted events.** A scripted event's
   `current_slot` is the slot the predicate evaluates against. Options:
   - Global events: one definition, one `current_slot = 0` (or world slot)
   - Per-owner events: same definition, evaluated once per owner with that
     owner's slot
   The data model and registration shape differ. Pick one for v0.

5. **`slot_to_thing` construction.** Spec handlers need a
   `&HashMap<u32, SimThingId>`. The `SlotAllocator` already has
   `owner_of(slot) -> Option<SimThingId>`. The session can either:
   - Build the HashMap fresh each tick from `SlotAllocator`
   - Maintain it incrementally on alloc/tombstone
   The choice depends on how often the session calls handlers.

6. **Notification + diagnostic routing.** `CapabilityTreeNotification`,
   `CapabilityTreeDiagnostic`, and `ScriptedEventDiagnostic` accumulate per
   call. Where do they go? Log? UI channel? Saved replay stream?

7. **Multi-tree-per-owner.** A faction may carry a tech_tree AND a
   national_ideas tree AND a talent_tree. The current per-owner state maps
   are keyed by `owner_id`, but multiple definitions share that key. Either
   key by `(owner_id, tree_definition_id)` or accept one state per owner-tree
   pair and adjust the data shape.

8. **Replay.** `ReplayDriver` in `simthing-sim` doesn't know about capability
   instances or scripted events today. Session state needs a save/load story
   that survives replay. **Open question:** is `OverlayId` (atomic counter)
   stable across replay? If not, capability definitions can't be serialized
   directly.

### Suggested data shape for Opus to refine

```rust
// likely in simthing-driver or a new simthing-session crate
pub struct SessionState {
    pub capability_definitions: HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>,
    pub capability_instances:   HashMap<SimThingId, CapabilityTreeInstance>,
    pub capability_states:      HashMap<SimThingId, CapabilityTreeState>,
    pub scripted_events:        Vec<ScriptedEventDefinition>,
    pub scripted_cooldowns:     HashMap<EventKey, u32>,
    pub slot_to_thing:          HashMap<u32, SimThingId>,
}

impl SessionState {
    pub fn boundary_step(
        &mut self,
        registry:         &DimensionRegistry,
        shadow:           &mut [f32],
        n_dims:           usize,
        threshold_events: &[ThresholdEvent],
        cpu_reg:          &ThresholdRegistry,
    ) -> BoundaryStepOutcome {
        // 1. Resolve threshold events
        let capability_unlocks      = cpu_reg.extract_capability_unlocks(threshold_events);
        let scripted_event_triggers = cpu_reg.extract_scripted_event_triggers(threshold_events);

        // 2. Run capability handler
        // 3. Run scripted-event handler
        // 4. Collect requests + notifications + diagnostics
        todo!()
    }
}
```

### Definition of done

- Session state lives somewhere coherent and documented.
- All three boundary handler methods are called from the boundary protocol.
- An integration test in `simthing-sim/tests/` (or the new crate) drives a
  capability unlock end-to-end from intent → GPU → handler → overlay
  activation → next-tick value change.
- Replay implications are documented even if the implementation is deferred.

### Why this is Opus work

Every question above is a design tradeoff with no obviously-correct answer.
The dep graph, layering, replay constraints, and multi-tree complexity
interlock — Composer agents will pick locally-optimal answers that paint the
session layer into corners.

---

## Track B — Composer 2.5: parallel mechanical tasks

Each task below is independent of Track A. None of them touch
`simthing-driver` or introduce new crates. All are narrowly scoped and
pattern-matched against existing code.

**Universal rules for Composer 2.5:**

- Run `cargo test --workspace` and `cargo build --workspace --tests` after
  every task. Both must report zero warnings and ≥298 passed tests.
- Each task should land as its own commit on `master` after completing.
- If a task expands beyond its acceptance criteria, **stop and flag it**
  rather than carrying the scope.
- Do not touch `simthing-driver`, do not add new crates, do not modify
  `boundary.rs` in `simthing-sim`, do not modify any handler in
  `simthing-spec/src/boundary/`.

### Task B1 — `Display` impls for diagnostic enums

**Goal:** Human-readable diagnostics for session-layer logging.

**Files:**
- `crates/simthing-spec/src/boundary/event_handler.rs`
- `crates/simthing-spec/src/runtime/capability_state.rs` (where
  `CapabilityTreeDiagnostic` lives)

**Add:**

```rust
impl std::fmt::Display for ScriptedEventDiagnosticKind { ... }
impl std::fmt::Display for ScriptedEventDiagnostic     { ... }
impl std::fmt::Display for CapabilityTreeDiagnostic    { ... }
```

Use `write!(f, "scripted event `{}` failed: {}", self.event_id.0, self.kind)`
patterns. Match formatting to existing `thiserror::Error` impls in the same
crate where they exist.

**Acceptance test:** in a new module-level `#[cfg(test)] mod tests` (or
existing one), assert that `format!("{}", diagnostic)` produces a non-empty
string containing the event id / sim_thing_id. One test per type.

**Do NOT:**
- Modify the diagnostic enum variants
- Add `Display` to runtime error types that already use `thiserror::Error`
- Touch any handler logic

### Task B2 — `EventKey: From<&str>` and `From<String>` ergonomic impls

**Goal:** Allow `"low_loyalty".into()` and `id_str.into()` in tests.

**File:** `crates/simthing-spec/src/spec/event.rs`

**Add:**

```rust
impl From<&str> for EventKey {
    fn from(s: &str) -> Self { Self(s.to_owned()) }
}
impl From<String> for EventKey {
    fn from(s: String) -> Self { Self(s) }
}
```

**Acceptance test:** add to the existing `#[cfg(test)] mod tests` (or create
one) covering both impls.

**Do NOT:**
- Touch any test file (callers can adopt the ergonomic form later)
- Modify the `EventKey` struct itself
- Add similar impls to other key types (`CategoryKey`, `CapabilityEntryKey`,
  `CapabilityEffectKey`) — those have multi-field shapes that don't fit

### Task B3 — Append-only registration entry points

**Goal:** Public append helpers so Track A's session layer can choose
append-vs-rebuild without re-exporting private functions.

**File:** `crates/simthing-sim/src/threshold_registry.rs`

**Add (next to existing `append_subtree` / `append_lineage`):**

```rust
pub fn append_capability_unlocks(
    dim_reg:            &DimensionRegistry,
    allocator:          &SlotAllocator,
    capability_unlocks: &[CapabilityUnlockRegistration],
    gpu_regs:           &mut Vec<ThresholdRegistration>,
    cpu_reg:            &mut ThresholdRegistry,
) {
    Self::push_capability_unlocks(dim_reg, allocator, capability_unlocks, gpu_regs, cpu_reg);
}

pub fn append_scripted_event_triggers(
    scripted_event_triggers: &[ScriptedEventTriggerRegistration],
    gpu_regs:                &mut Vec<ThresholdRegistration>,
    cpu_reg:                 &mut ThresholdRegistry,
) {
    Self::push_scripted_event_triggers(scripted_event_triggers, gpu_regs, cpu_reg);
}
```

**Acceptance test:** add to `#[cfg(test)] mod tests` at the bottom of
`threshold_registry.rs`. Two tests: each verifies that calling `append_*` on
a pre-populated `(gpu_regs, cpu_reg)` pair grows both buffers by the input
length and preserves existing `event_kind` indices (i.e., the new entries get
`event_kind = old_len, old_len + 1, ...`).

**Do NOT:**
- Modify the private `push_*` functions
- Touch the full-rebuild `build_with_*` methods
- Add a boundary protocol caller for these helpers — that's Track A
- Try to wire B2 append-vs-rebuild eligibility detection

### Task B4 — Docs addenda

**Goal:** Make the design docs reflect what's actually implemented.

**Files:**
- `docs/design_v6.md` — add an addendum section "Scripted event system v0
  (PRs 7-10)" near the existing capability tree addendum.
- `docs/capability_tree_v1.md` — add an addendum noting that capability
  unlock events now flow through `CapabilityUnlockEvent` (not raw
  `ThresholdEvent`s) and that `simthing-spec` no longer depends on
  `simthing-sim`/`simthing-gpu` in production.

**Content guidance:**

For `design_v6.md` scripted event addendum, cover:

- The 4-stage pipeline: `EventSpec` → `compile_event` → `ScriptedEventDefinition`
  → boundary handler dispatch.
- The two trigger sources (predicate, threshold) and the unified cooldown +
  priority gating.
- The cross-crate types: `ScriptedEventTriggerRegistration`,
  `ScriptedEventTriggerEvent` in feeder; `ThresholdSemantic::ScriptedEventTrigger`
  arm in sim.
- Explicit "session/driver assembly not yet implemented" note.

For `capability_tree_v1.md` addendum, cover:

- `CapabilityUnlockEvent` is the resolved input to the handler now.
- The conversion bridge is `ThresholdRegistry::extract_capability_unlocks`.
- The handler entry point renamed from `handle_threshold_events` to
  `handle_capability_unlock_events`.

**Acceptance:** the addenda should be readable in 5 minutes by someone who
read `design_v6.md` and is coming back after PR 8. Do not rewrite existing
sections — append addenda only.

**Do NOT:**
- Edit any non-`.md` file
- Rewrite or contradict existing prose
- Promise features not in `master`

### Task B5 — Verify release-mode parity (smoke check)

**Goal:** Confirm the release profile also builds and tests clean.

**Commands:**

```powershell
cargo build --workspace --release --tests
cargo test --workspace --release
```

**Acceptance:** both succeed with zero warnings. If any test fails only in
release, **stop immediately** and report — release-only failures usually
indicate UB or test-ordering bugs and require Opus.

**Do NOT:**
- "Fix" warnings or failures by silencing them
- Add `#[allow]` attributes without flagging to a human

---

## Suggested execution order for Track B

If Composer is doing the prep work before Opus picks up Track A:

1. **B5 first** (smoke check baseline)
2. **B2** (cheapest; ergonomic win)
3. **B1** (independent, useful for B4 docs to reference Display output)
4. **B3** (Track A may need these helpers from the start)
5. **B4** (docs absorb the impact of B1-B3)

Total estimated effort: ~2-4 hours for a Composer agent who follows the
"stop and flag" rule.

---

## Interface contract between Track A and Track B

When Opus picks up Track A, these Track B outputs become available as
load-bearing API:

| Track B output | Track A relies on |
|---|---|
| `EventKey: From<&str>` | Fixture ergonomics in session integration tests |
| `Display` for diagnostics | Session-layer log line construction |
| `append_capability_unlocks` | B2 append-only path in boundary protocol |
| `append_scripted_event_triggers` | Same, for scripted events |
| Docs addenda | Onboarding context (not load-bearing) |

If Track B has not landed when Track A starts, Opus should treat the absent
helpers as TODOs in the session layer and continue — Track A is not blocked
on Track B.

---

## Verification before parking either track

```powershell
cargo test --workspace
cargo build --workspace --tests
git status --short --branch
```

Expected: 298+ tests passing, 1 ignored, zero warnings, clean working tree
(or only the standard `.claude/worktrees/` + `demo.replay.ldjson` untracked).
