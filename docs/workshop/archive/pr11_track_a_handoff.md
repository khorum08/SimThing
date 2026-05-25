> **SUPERSEDED** — Track A complete. Use [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md).

# PR 11 — Track A Handoff Digest (post Track B)

**Date:** 2026-05-22  
**Prepared by:** Composer (Cursor) after completing Track B  
**Supersedes baseline in:** `docs/workshop/pr11_session_assembly_handoff.md` (Track B section is done)  
**Master HEAD:** `866a467` (`docs: park state after PR #47 Track B merge.`)  
**Last code merge:** `392992f` (PR #47 — Track B mechanical prep)  
**Test baseline:** `cargo test --workspace` → **306** passed, **1** ignored, zero warnings  
**Release baseline:** `cargo build --workspace --release --tests` and `cargo test --workspace --release` both clean (Track B5 verified)

**Crate boundary (unchanged):** `simthing-spec` production deps are `simthing-core` + `simthing-feeder` only. Handlers must **not** be embedded inside `simthing-sim::BoundaryProtocol`.

---

## Executive summary

PR 11 was split into two tracks. **Track B (Composer) is complete and merged.** Track A (Opus) — session/driver assembly — is the sole remaining PR 11 work and is **not started**.

The spec layer (`simthing-spec` PRs 2–10) is functionally complete through scripted-event GPU thresholds. Three boundary handlers exist and are unit/integration tested in isolation, but **nothing in `simthing-driver` or `simthing-sim::BoundaryProtocol` calls them yet.** There is no owner for per-session capability instances, runtime state, scripted-event definitions, or cooldown maps.

---

## Completed from the original PR 11 handoff

### Track B — all tasks merged (PR #47, `392992f`)

| Task | Status | Commit / location |
|------|--------|-------------------|
| **B5** Release smoke check | Done | Verified; no code change |
| **B2** `EventKey: From<&str>` / `From<String>` | Done | `84e03fc` — `crates/simthing-spec/src/spec/event.rs` |
| **B1** `Display` for diagnostics | Done | `f2ed680` — `event_handler.rs`, `capability_state.rs` |
| **B3** Append registration helpers | Done | `e8d2980` — `ThresholdBuilder::append_capability_unlocks`, `append_scripted_event_triggers` in `threshold_registry.rs` |
| **B4** Docs addenda | Done | `795bc69` — `design_v6.md` § scripted events v0; `capability_tree_v1.md` §12 unlock bridge |

**+8 tests** added (306 total vs 298 at handoff time). All Track B do-not-touch rules were followed: no changes to `simthing-driver`, `boundary.rs`, handler logic, or new crates.

### Load-bearing API now available for Track A

| API | Purpose for Track A |
|-----|---------------------|
| `EventKey: From<&str>` / `From<String>` | Ergonomic fixtures in session integration tests |
| `Display` for `ScriptedEventDiagnostic*`, `CapabilityTreeDiagnostic` | Session-layer log lines |
| `ThresholdBuilder::append_capability_unlocks` | B2 append-only threshold path (when eligible) |
| `ThresholdBuilder::append_scripted_event_triggers` | Same for scripted events |
| `ThresholdRegistry::extract_capability_unlocks` | GPU events → `CapabilityUnlockEvent` for spec handler |
| `ThresholdRegistry::extract_scripted_event_triggers` | GPU events → `ScriptedEventTriggerEvent` for spec handler |
| Docs addenda | Onboarding; not load-bearing |

### Context: `simthing-spec` PRs 2–10 (pre-Track B, already on master)

These are **not** Track B work but are prerequisites Track A will wire up:

| PR | Scope | Key surfaces |
|----|-------|--------------|
| 2 | Property + overlay compiler | `compile/property.rs`, `compile/overlay.rs` |
| 3 | `CapabilityTreeBuilder` | `compile/capability.rs`, `CapabilityTreeDefinition` |
| 4 | Unlock registration bridge | `simthing-feeder::CapabilityUnlockRegistration`, `ThresholdSemantic::CapabilityUnlock` |
| 5 | Capability boundary handler | `CapabilityTreeBoundaryHandler`, `CapabilityTreeState` |
| 6 | Preview + mutual exclusivity | `preview/capability_preview.rs` |
| 7 | Script IR | `spec/script.rs` |
| 8 | Event compiler templates | `compile/event.rs`, `ScriptedEventDefinition` |
| 9 | Scripted event handler (predicate) | `ScriptedEventBoundaryHandler::handle_tick` |
| 10 | Scripted event GPU thresholds | `ScriptedEventTriggerRegistration`, unified handler |

**Dependency cleanup (done):** spec consumes `CapabilityUnlockEvent` from feeder; handler entry point is `handle_capability_unlock_events` (not raw threshold events).

---

## Remaining: Track A — session/driver assembly (Opus)

### The problem (unchanged)

Three handler entry points exist but are **not called** from the live day-boundary path:

1. `CapabilityTreeBoundaryHandler::handle_capability_unlock_events(&[CapabilityUnlockEvent], &mut ctx)`
2. `CapabilityTreeBoundaryHandler::handle_player_selection(owner, entry_key, &mut ctx)`
3. `ScriptedEventBoundaryHandler::handle_tick(&[ScriptedEventTriggerEvent], &mut ctx)`

No owner exists for:

```text
HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>  — shared defs
HashMap<SimThingId, CapabilityTreeInstance>                  — per-owner instances (⚠ multi-tree issue)
HashMap<SimThingId, CapabilityTreeState>                       — per-owner runtime state (⚠ multi-tree issue)
Vec<ScriptedEventDefinition>                                   — compiled events
HashMap<EventKey, u32>                                         — scripted-event cooldowns (scope TBD)
HashMap<u32, SimThingId>                                       — slot_to_thing for ScopeRef resolution
```

`simthing-driver::SimSession` (`session.rs`) is still a thin glue loop: feeder + GPU + `BoundaryProtocol`. It does not depend on `simthing-spec`.

### Eight design questions Opus must answer

1. **Where does session state live?**
   - Extend `simthing-driver` with `SessionState`, **or**
   - New `simthing-session` crate above driver, **or**
   - ~~Embed in `BoundaryProtocol`~~ — **reject** (reverses dep cleanup; sim would depend on spec).

2. **Boundary protocol step order.**
   - Capability unlocks: after Pass 7, before fission/fusion (per original handoff).
   - Decide scripted-event handler slot: before/after capability unlocks? before/after fission?

3. **GPU event drain plumbing.**
   - `DispatchCoordinator::tick()` → `TickOutcome.threshold_events`
   - `extract_capability_unlocks` / `extract_scripted_event_triggers` → spec handlers
   - Remaining events stay on existing sim paths (fission, fusion, property expiry, alerts).

4. **Per-owner vs global scripted events (v0 pick required).**
   - Global: one definition, one `current_slot` (world slot).
   - Per-owner: same definition evaluated per owner slot — different registration shape.

5. **`slot_to_thing` construction.**
   - Fresh build from `SlotAllocator` each boundary tick, **or**
   - Incremental maintenance on alloc/tombstone.

6. **Notification + diagnostic routing.**
   - `CapabilityTreeNotification`, `CapabilityTreeDiagnostic`, `ScriptedEventDiagnostic` — log? UI channel? replay stream?

7. **Multi-tree-per-owner.**
   - Faction may have `tech_tree` + `national_ideas` + `talent_tree`.
   - Current state maps keyed by bare `SimThingId` are insufficient — key by `(owner_id, tree_definition_id)` or equivalent.

8. **Replay.**
   - `ReplayDriver` has no capability/scripted-event session state today.
   - Document save/load even if deferred.
   - Open: is `OverlayId` stable across replay?

### Suggested starting shape (refine, don't copy blindly)

See original handoff § "Suggested data shape" — `SessionState::boundary_step(...)` orchestrating extract → handlers → collect `BoundaryRequest`s.

Handler contexts already exist:

- `CapabilityBoundaryContext` — `boundary/capability_handler.rs`
- `ScriptedEventBoundaryContext` — `boundary/event_handler.rs`

Both need `slot_to_thing`, shadow slice, and mutable request/diagnostic vectors threaded from the session layer.

### Definition of done (Track A)

- [ ] Session state lives somewhere coherent and documented.
- [ ] All three handler methods called from the boundary protocol path (session coordinator, not inside `BoundaryProtocol` if layering is preserved).
- [ ] Integration test: intent → GPU → unlock handler → overlay activation → **next-tick value change** (stronger than existing `capability_unlock_fires_in_boundary_integration_test`, which only proves Pass 7 fires and semantic resolves — it does **not** call the spec handler or activate overlays).
- [ ] Replay implications documented (implementation may defer).

### Why Opus

Design questions interlock (layering, multi-tree keys, replay, event scope). Local choices paint the session layer into corners.

---

## Other remaining work (outside Track A scope but related)

### In `simthing-spec` / spec layer

| Item | Status | Notes |
|------|--------|-------|
| Session init from `GameModeSpec` / RON | Open | PR 1 loaders exist; no driver session assembly |
| `handle_player_selection` wiring | Open | Handler tested in `pr5_capability_handler.rs`; no live player input path |
| B2 append-only threshold integration | Deferred | `append_*` helpers exist; boundary eligibility detection not wired |
| EML / parser / full script surface | Out of scope v0 | PR 7–8 are typed-template slice only |
| `simthing-studio` GUI | Deferred | Depends on spec + session |

### In `simthing-sim` / `simthing-driver`

| Item | Status | Notes |
|------|--------|-------|
| `BoundaryProtocol` spec awareness | Open | Track A wires via session layer, not by importing spec into sim |
| `SimSession` spec compilation | Open | Driver may gain `simthing-spec` dep for session assembly |
| Scenario format expansion | Tabled | Full RON tree/registry/shadow seeds |
| Replay v3 for capability/scripted state | Open | Document in Track A even if not implemented |

### Existing test coverage gap (Track A target)

`capability_unlock_fires_in_boundary_integration_test` in `boundary_integration.rs`:

- Proves GPU Pass 7 + `ThresholdSemantic::CapabilityUnlock` resolution.
- Does **not** exercise `CapabilityTreeBoundaryHandler`, overlay activation, or next-tick effect.
- Track A E2E test should close this gap through the session layer.

---

## Key file map for Track A

```text
Session layer (to create or extend):
  crates/simthing-driver/src/session.rs          — current SimSession loop
  crates/simthing-driver/src/scenario.rs         — scenario loading (builtin only today)

Spec handlers (call, do not modify logic unless bug):
  crates/simthing-spec/src/boundary/capability_handler.rs
  crates/simthing-spec/src/boundary/event_handler.rs
  crates/simthing-spec/src/runtime/capability_state.rs
  crates/simthing-spec/src/runtime/capability_definition.rs
  crates/simthing-spec/src/runtime/scripted_event_definition.rs

Sim bridge (already exists):
  crates/simthing-sim/src/threshold_registry.rs   — extract_*, append_*, build_with_*
  crates/simthing-sim/src/boundary.rs             — BoundaryProtocol (do not embed spec)

Feeder types:
  crates/simthing-feeder/src/capability.rs        — CapabilityUnlockEvent, CapabilityUnlockRegistration
  crates/simthing-feeder/src/scripted_event.rs    — ScriptedEventTriggerRegistration/Event

Reference tests (isolated handler behavior):
  crates/simthing-spec/tests/pr5_capability_handler.rs
  crates/simthing-spec/tests/pr9_event_handler.rs
  crates/simthing-spec/tests/pr10_scripted_event_thresholds.rs
  crates/simthing-sim/tests/boundary_integration.rs  — GPU threshold only
```

---

## Read order for the next agent

1. This document
2. `docs/workshop/pr11_session_assembly_handoff.md` (Track A section; Track B is done)
3. `docs/todo.md` — parking state + PR ladder
4. `docs/worklog.md` — PR #47 merge entry
5. `docs/design_v6.md` — scripted event addendum + spec pivot addendum
6. `docs/capability_tree_v1.md` — §12 unlock bridge addendum
7. Handler source + `session.rs` in driver

**Stale docs to ignore:** `docs/workshop/opus_current_state_handoff.md` (pre-PR 9/10/11); `todo.md` "Known divergences" archaeology section (pre-PR 2–8).

---

## Verification before parking Track A

```powershell
cargo test --workspace
cargo build --workspace --tests
git status --short --branch
```

Expected: **306+** passed, **1** ignored, zero warnings.

---

## Suggested Track A execution order

1. **Design memo** — answer all 8 questions in a short ADR (even if choices are v0-conservative).
2. **Session state scaffold** — `SessionState` in driver or new crate; no handler wiring yet.
3. **Multi-tree key fix** — adjust instance/state maps before wiring handlers.
4. **`slot_to_thing` + shadow threading** — build map from `SlotAllocator`; pass into handler contexts.
5. **Boundary step** — extract threshold events → handlers → drain `BoundaryRequest`s into existing sim boundary request queue.
6. **E2E integration test** — full unlock → activate → next-tick value change.
7. **Replay doc** — document gaps; implement only if in scope for the session.

**Do not** start by modifying handler logic in `simthing-spec/src/boundary/` unless a bug is found during E2E.
