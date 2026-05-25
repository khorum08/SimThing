> **SUPERSEDED** — Folded into [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md) § Open work.

# PR 11 Post Track A Handoff Digest

**Date:** 2026-05-22
**Prepared by:** Codex 5.5 high after completing Track A
**Target agents:** Opus 4.7 for architectural follow-up, Composer 2.5 for mechanical/doc tasks
**Current branch:** `master`
**Current HEAD:** `01fb572` (`simthing-driver PR 11 Track A: session assembly`)
**Baseline before Track A:** `866a467` (`docs: park state after PR #47 Track B merge.`)
**Verification:** `cargo test --workspace` -> 311 passed, 1 ignored, zero warnings. `cargo build --workspace --tests` clean.

## Executive Summary

PR 11 Track A is complete and pushed. The live `SimSession` now owns
`simthing-spec` runtime state and invokes the capability and scripted-event
boundary handlers during the real day-boundary path. `simthing-sim` remains
free of a `simthing-spec` dependency: the sim crate exposes a generic
post-readback boundary hook and stores only feeder-level threshold registration
types.

The strongest new proof is a GPU E2E test in `simthing-driver`:

```text
capability progress threshold -> GPU Pass 7 event -> session hook ->
CapabilityTreeBoundaryHandler -> ActivateOverlay request -> boundary mutation ->
next tick value change
```

This closes the Track A coverage gap noted in the PR 11 handoff. The old
`capability_unlock_fires_in_boundary_integration_test` only proved GPU
threshold semantics; the new test proves the live session calls the spec
handler and the activated overlay affects values on the next tick.

## Completed Tasks

### Architecture

- Added `docs/adr/pr11_track_a_session_assembly.md`.
- Chose `simthing-driver` as the owner of spec runtime state.
- Rejected embedding handlers inside `simthing-sim::BoundaryProtocol`.
- Added a generic sim-side boundary hook that runs after canonical GPU value
  readback and before lifecycle, expiry, fission/fusion, and structural
  mutation.
- Kept `simthing-sim` independent of `simthing-spec`.

### Driver Session State

Added `crates/simthing-driver/src/spec_session.rs`:

- `SpecSessionState`
- `CapabilityInstanceKey`
- capability definitions
- capability instances and states keyed by `(owner_id, definition_id, tree_thing_id)`
- capability unlock registrations
- scripted event definitions
- scripted event cooldowns
- scripted current slot
- capability notifications and diagnostics
- scripted-event diagnostics
- handler error capture
- queued player selections

The multi-tree session key is important. The PR 5 handler still accepts maps
keyed by owner id, but the driver stores state by owner plus definition plus
tree id, then calls the handler with temporary one-instance maps. This avoids
the session-level `tech_tree + national_ideas + talent_tree` collision without
forcing a handler API migration inside Track A.

### Sim Boundary Hook

Changed `crates/simthing-sim/src/boundary.rs`:

- Added `BoundaryHookContext`.
- Added `BoundaryProtocol::execute_with_boundary_hook`.
- Preserved `BoundaryProtocol::execute` as a no-hook wrapper.
- Added external registration storage:
  - `Vec<CapabilityUnlockRegistration>`
  - `Vec<ScriptedEventTriggerRegistration>`
- Added setters:
  - `set_capability_unlock_registrations`
  - `set_scripted_event_trigger_registrations`

Changed `crates/simthing-sim/src/gpu_sync.rs`:

- Full threshold rebuilds now append external capability unlock registrations.
- Full threshold rebuilds now append external scripted-event trigger registrations.
- No `simthing-spec` import was added to sim.

Changed `crates/simthing-sim/src/lib.rs`:

- Re-exported `BoundaryHookContext`.

### SimSession Wiring

Changed `crates/simthing-driver/src/session.rs`:

- Added `pub spec_state: SpecSessionState`.
- Added `SimSession::install_spec_state`.
- `install_spec_state` syncs external threshold registrations into
  `BoundaryProtocol` and refreshes GPU threshold buffers with `initial_gpu_sync`.
- `run` and `record_to_path` call `execute_with_boundary_hook`.
- Empty-boundary skipping is disabled when spec state needs boundary ticking,
  such as scripted events, queued player selections, or `OnPrereqMet` sweeps.

Changed `crates/simthing-driver/Cargo.toml`:

- Added `simthing-spec` as a driver dependency.

### Tests

Added CPU unit coverage in `spec_session.rs`:

- queued player selection runs through the capability handler.
- scripted-event predicate dispatch runs through `SpecSessionState`.

Added GPU E2E coverage in `crates/simthing-driver/tests/session_integration.rs`:

- `spec_session_capability_unlock_activates_overlay_for_next_tick`

## Current Caveats And Footguns

### Stale Parking Text

`docs/todo.md` and `docs/worklog.md` were updated before commit/push and still
say Track A is "implemented locally" on top of `866a467`. The current source of
truth is this digest plus Git HEAD `01fb572`. Composer should clean those
parking lines as a small docs task.

### Replay Is Not Complete

Existing structural overlay activation/suspension still appears in the
boundary delta log. However, these are not serialized yet:

- capability runtime state
- scripted-event cooldowns
- queued player selections
- capability notifications
- scripted/capability diagnostics

Replay v3 needs a deliberate design. Do not claim full replay support for spec
runtime state yet.

### Session Init Is Manual

`SimSession::install_spec_state` is the live seam, but there is no RON or
`GameModeSpec` assembly path that compiles domain packs, clones capability
trees per faction, and installs scripted events automatically.

### External Threshold Append Path Is Deferred

Track A full threshold rebuilds include external capability/scripted-event
registrations. Append-only registration helpers exist from Track B, but
eligibility detection and append handling for external registrations on cloned
capability trees is still deferred.

### Scripted Event Scope Is V0 Global

`SpecSessionState` has one `scripted_current_slot` and one cooldown map keyed by
`EventKey`. Per-owner scripted event expansion is not implemented. If Opus
chooses per-owner semantics later, cooldown keys may need owner or scope.

### Capability Handler API Still Uses Owner-Keyed Maps

Driver storage is multi-tree-safe. Handler API is still PR 5 shape. The driver
bridges with temporary maps. That bridge is correct for v0 but not elegant.
Future cleanup may add a tree-instance-keyed handler context.

## Next Steps For Opus 4.7

Opus should take tasks that require architecture, ownership, replay, or
cross-crate design.

### O1. Session Init From Authored Specs

Goal: Build a real `SpecSessionState` from `GameModeSpec`, domain packs, or a
new scenario/session descriptor.

Recommended scope:

- Decide where authored packs live in `Scenario`.
- Compile properties/overlays/capability trees/events during session open.
- Clone capability tree templates per owner/faction.
- Stamp capability overlay `affects` fields for the correct runtime target.
- Populate:
  - capability definitions
  - capability instances
  - capability states
  - capability unlock registrations
  - scripted event definitions
  - scripted current slot or owner slots
- Add an integration test that starts from authored spec data rather than a
  hand-built `SpecSessionState`.

Do not make `simthing-sim` depend on `simthing-spec`.

### O2. Replay V3 Design For Spec Runtime State

Goal: Decide how capability/scripted runtime state is persisted and replayed.

Questions to answer:

- Are capability definitions serialized, rebuilt from authored specs, or
  referenced by stable logical ids?
- Are `OverlayId`s stable enough for replay, or must logical
  `CapabilityEffectKey`s be used?
- Are cooldowns replay state, derived state, or event-log state?
- Should notifications/diagnostics be replayed, logged separately, or treated
  as UI-only?
- How does replay reconstruct queued player selections?

Acceptance:

- ADR or design doc.
- At least one replay test if implementation is included.
- If deferred, explicit docs in `design_v6.md` or a replay-specific doc.

### O3. Player Selection Input Path

Goal: Turn `SpecSessionState::queue_player_selection` into a live player/UI
input path.

Design choices:

- Is player selection a feeder work item, a session API, or a UI command queue?
- How is owner/tree/entry addressed from authored logical keys?
- How are invalid selections reported?
- Should queued selections be replay-recorded?

Acceptance:

- Public API or feeder path.
- Test that selection activates a player-selection capability through a real
  `SimSession`.
- Diagnostics for wrong activation mode or missing entry.

### O4. Per-Owner Scripted Event Semantics

Goal: Decide whether scripted events remain global or become per-owner/per-slot.

If per-owner:

- Define owner slots.
- Generate one threshold registration per owner for threshold events.
- Key cooldowns by `(EventKey, owner_id)` or equivalent.
- Ensure priority ordering is well-defined across owners.

If global:

- Document why one `scripted_current_slot` is sufficient for v0.
- Clarify how modders author global events versus owner-scoped events later.

### O5. External Registration Append-Only Integration

Goal: Extend B2 append-only threshold handling to external capability and
scripted-event registrations.

This is architecture-sensitive because cloned capability trees may introduce
new capability unlock registrations at the same time fission grows the tree.
Eligibility must account for:

- pure fission growth
- cloned capability subtrees
- new tree slots
- new scripted-event trigger slots
- event_kind stability
- threshold config revision

## Tasks Appropriate For Composer 2.5

Composer should take small, mechanical tasks with constrained file lists and
clear acceptance criteria. Do not give Composer replay architecture, session
ownership, or per-owner semantics.

### C1. Consolidate Workshop simthing-spec Progress Logs

**Goal:** Assemble all `simthing-spec` progress currently scattered across the
workshop folder into one chronological progress log.

**Create:** `docs/workshop/simthing_spec_progress_log.md`

**Source files to read:**

- `docs/workshop/simthing_spec_master_handoff.md`
- `docs/workshop/pr5_handoff_digest.md`
- `docs/workshop/opus_current_state_handoff.md`
- `docs/workshop/pr11_session_assembly_handoff.md`
- `docs/workshop/pr11_track_a_handoff.md`
- `docs/workshop/pr11_post_track_a_handoff.md`
- `docs/workshop/simthing_spec_workshop.md`
- relevant top entries in `docs/worklog.md`
- relevant `simthing-spec` section in `docs/todo.md`

**Output format:**

- One chronological section per PR/track:
  - PR 1
  - PR 2
  - PR 3
  - PR 4
  - PR 5
  - PR 6
  - PR 7
  - PR 8
  - PR 9
  - PR 10
  - PR 11 Track B
  - PR 11 Track A
- Each section has:
  - commit(s)
  - status
  - files/surfaces added
  - tests added
  - caveats/deferred items
- End with a "Current Source Of Truth" section naming HEAD `01fb572` and
  verification `311 passed, 1 ignored`.

**Do not:**

- Edit code.
- Invent missing commit hashes.
- Remove historical workshop docs.
- Rewrite master handoff docs.

**Acceptance:**

- New progress log exists.
- No code files changed.
- It explicitly marks stale pre-PR9/PR10 handoffs as historical.
- It calls out that `docs/todo.md` and `docs/worklog.md` needed parking text
  cleanup after Track A push.

### C2. Clean Parking Text In Todo And Worklog

**Goal:** Fix stale lines that still say Track A is local/on top of `866a467`.

**Files:**

- `docs/todo.md`
- `docs/worklog.md`

**Expected content:**

- `master` and `origin/master` synced at `01fb572`.
- Track A pushed.
- Verification: `cargo test --workspace` -> 311 passed, 1 ignored; build clean.
- Remaining untracked workshop files are local-only unless deliberately staged.

**Do not:**

- Touch code.
- Change historical entries except the top parking/current-state text.

### C3. Add Display/Debug Tests For Driver Spec Session Diagnostics

**Goal:** Add small tests around driver-side `handler_errors` and diagnostic
collection, if useful for logs.

**Files:**

- `crates/simthing-driver/src/spec_session.rs`

**Acceptance:**

- One or two unit tests only.
- No handler logic changes.
- `cargo test -p simthing-driver spec_session` passes.

### C4. Release Smoke Check After Track A

**Goal:** Confirm Track A did not disturb release profile.

**Commands:**

```powershell
cargo build --workspace --release --tests
cargo test --workspace --release
```

**Acceptance:**

- Both commands clean.
- Update `docs/worklog.md` with result.
- No code changes unless required to fix a release-only issue.

### C5. Workshop Index Cleanup

**Goal:** Add a short index at the top of `docs/workshop/simthing_spec_workshop.md`
or create `docs/workshop/README.md` listing which workshop docs are current and
which are historical.

**Acceptance:**

- Marks `pr11_post_track_a_handoff.md` and the new consolidated progress log as
  current.
- Marks `opus_current_state_handoff.md` as pre-PR9/10/11 historical.
- Leaves all existing docs in place.

## Read Order For Next Agent

1. `docs/workshop/pr11_post_track_a_handoff.md`
2. `docs/adr/pr11_track_a_session_assembly.md`
3. `crates/simthing-driver/src/spec_session.rs`
4. `crates/simthing-driver/src/session.rs`
5. `crates/simthing-sim/src/boundary.rs`
6. `crates/simthing-sim/src/gpu_sync.rs`
7. `crates/simthing-driver/tests/session_integration.rs`
8. `docs/todo.md` and `docs/worklog.md` after Composer parking cleanup

## Verification Commands

```powershell
cargo test --workspace
cargo build --workspace --tests
git status --short --branch
```

Expected after Track A: 311 passed, 1 ignored, zero warnings.

