# Spec Session State Replay

**Date:** 2026-05-23
**Status:** Proposed
**Blocks:** O2 (Replay v3 — spec session state)
**Related:** [`game_mode_session_installation.md`](game_mode_session_installation.md), [`scripted_event_scope_model.md`](scripted_event_scope_model.md), `docs/replay_v2.md` (existing structural replay)

## Context

Replay v2 (see [`replay.rs`](../../crates/simthing-sim/src/replay.rs)) records
the `BoundaryProtocol`'s authoritative tree via a `ReplaySnapshot` plus a
per-boundary `ReplayFrame` of `BoundaryDeltaEntry`s. `ReplayDriver` reconstructs
**tree structure**, **dimension registry**, and **fission lineage**. It does
not replay GPU value integration; optional `shadow_values` checkpoints in each
frame enable numeric audit without bit-exact resimulation. Crucially, replay v2
knows nothing about `simthing-spec`.

PR 11 Track A introduced `simthing-driver::SpecSessionState` ([`spec_session.rs`](../../crates/simthing-driver/src/spec_session.rs):37–51).
Structural overlay activations emitted by spec handlers flow through the
existing delta log path (because `ActivateOverlay` is an ordinary
`BoundaryRequest`), so the structural side of the unlock pipeline is already
replay-covered. Everything else — runtime activation modes, mutual-exclusivity
state, scripted-event cooldowns, queued player selections, diagnostics, the
notification stream — is **not** captured anywhere durable. Re-opening a
replay produces a session whose spec runtime is empty.

The forces shaping this ADR:

1. **`OverlayId` is not stable across processes.** Every clone of a capability
   tree allocates fresh `OverlayId`s via `OverlayId::new()` (atomic counter at
   [`compile/capability.rs`](../../crates/simthing-spec/src/compile/capability.rs):188).
   The order in which capability trees install at session open determines the
   ids. A replay loaded in process B will not see the same ids process A
   recorded. Any replay payload that names an overlay by raw `OverlayId` is
   broken by design — and `BoundaryDeltaEntry::OverlayActivated` already does
   exactly that today.
2. **Spec runtime state is bimodal.** Some of it (capability definitions,
   instances, scripted event definitions) is *derivable* from the installed
   `GameModeSpec` + scenario. Some (cooldowns, runtime activation modes) is
   *authoritative mutable* and must be serialized to round-trip.
3. **Replay must be cheap.** Structural changes already serialize one entry
   per boundary mutation. Spec state churn should follow the same shape —
   emit deltas, not full snapshots, except at recording start.
4. **`simthing-sim` stays spec-free.** Replay format extensions for spec
   state must keep their type definitions out of `simthing-sim`. Either the
   serde types live in `simthing-spec`/`simthing-driver` and the replay
   stream embeds opaque blobs, or the replay format gets an optional
   spec-state appendix written by the driver and ignored by sim-only readers.

## Decision

Classify every field on `SpecSessionState` into one of four classes, with a
specific handling rule for each. Add a **`SpecSnapshot`** at recording start
(captured by the driver, not by `BoundaryProtocol`) and a per-frame
**`SpecDelta`** array. Resolve all overlay references through
`CapabilityEffectKey` (logical) — never through raw `OverlayId`.

### State classification

| Field | Class | Handling |
|---|---|---|
| `capability_definitions: HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>` | Reconstructable | Rebuilt by re-running `compile_and_install` on replay open. Not serialized. |
| `capability_instances: HashMap<CapabilityInstanceKey, CapabilityTreeInstance>` | Reconstructable | Rebuilt at replay open. Identity matches because the cloned tree `SimThingId`s are recorded in the snapshot's `root`. |
| `capability_states: HashMap<CapabilityInstanceKey, CapabilityTreeState>` | **Authoritative mutable** | Snapshot at start. Per-frame delta entries for `activation_mode_by_entry` and `active_by_category` changes. |
| `capability_unlock_registrations: Vec<CapabilityUnlockRegistration>` | Reconstructable | Rebuilt from definitions + instances. Not serialized. |
| `scripted_event_definitions: HashMap<…>` (post Option B) | Reconstructable | Rebuilt at replay open. |
| `scripted_event_instances: HashMap<…>` (post Option B) | **Authoritative mutable** for `cooldown_remaining` and `current_slot`; reconstructable for the rest | Snapshot covers the set; per-frame delta covers cooldown ticks and slot refreshes. |
| `player_selections: Vec<(CapabilityInstanceKey, CapabilityEntryKey)>` | **Authoritative mutable** (transient queue) | Snapshot if non-empty at recording start; per-frame `SpecDelta::PlayerSelectionQueued` for additions. Drained selections produce no entry (the activation they cause already lands in `OverlayActivated`). |
| `capability_notifications: Vec<CapabilityTreeNotification>` | **Notifications** | **Replay-visible.** Emitted as `SpecDelta::CapabilityNotification` per boundary, so downstream consumers reading a replay see the same `IdeaSwitched` stream the original session emitted. The vec is cleared after each emission. |
| `capability_diagnostics: Vec<CapabilityTreeDiagnostic>` | Transient | **Log-only**, not replayed. Reconstructable from the simulation if it were re-run, but diagnostics are debugging aids and should not be load-bearing for replay correctness. |
| `scripted_event_diagnostics: Vec<ScriptedEventDiagnostic>` | Transient | Log-only. |
| `handler_errors: Vec<String>` | Transient | Log-only. |

The terms "reconstructable / authoritative mutable / transient / notifications"
match the four classes in the handoff brief.

### `SpecSnapshot` shape

Written once at recording start by `SimSession::record_to_path`, on the line
**immediately after** the existing `ReplaySnapshot`:

```rust
// crates/simthing-driver/src/spec_replay.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecSnapshot {
    pub day: u32,
    /// Per-instance capability state at snapshot time.
    pub capability_states: Vec<CapabilityStateSnapshot>,
    /// Per-instance scripted-event cooldowns at snapshot time. Empty for
    /// instances at 0 cooldown.
    pub scripted_cooldowns: Vec<ScriptedCooldownSnapshot>,
    /// Pending player selections at snapshot time (rare; usually empty).
    pub queued_selections: Vec<QueuedSelectionSnapshot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityStateSnapshot {
    pub owner_id: SimThingId,
    pub tree_thing_id: SimThingId,
    pub definition_logical_id: String,        // CapabilityTreeDefinition.tree_id, not numeric id
    pub activation_modes: Vec<(CapabilityEntryKey, ActivationMode)>,
    pub active_by_category: Vec<(CategoryKey, Vec<CapabilityEntryKey>)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScriptedCooldownSnapshot {
    pub owner_id: SimThingId,
    pub event_id: EventKey,
    pub cooldown_remaining: u32,
}
```

Key choices:

- **Logical keys throughout.** No `CapabilityTreeDefinitionId`,
  no `ScriptedEventDefinitionId`, no `OverlayId`. Every cross-reference uses
  either an authored string id (`tree_id`, `event_id`) or a logical compound
  key already defined in `simthing-spec` (`CapabilityEntryKey`, `CategoryKey`).
  This is the *only* way replays survive process boundaries.
- **`owner_id` is stable across replay open.** It comes from the recorded
  `ReplaySnapshot.root` and is reconstructed by `ReplayDriver`. The driver
  joins on it to find the matching `CapabilityInstanceKey` after install.
- **No `tree_thing_id` join is strictly required** when `definition_logical_id`
  + `owner_id` is unique per session — but it is recorded for robustness
  against future multi-tree-per-spec-per-owner cases.

### `SpecDelta` per-frame entries

Each `ReplayFrame` gets a new optional `spec_entries: Vec<SpecDelta>` field
(serde `#[serde(default, skip_serializing_if = "Vec::is_empty")]` so old
replays parse cleanly). Driver-emitted, never written from sim.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SpecDelta {
    CapabilityActivationModeChanged {
        owner_id: SimThingId,
        definition_logical_id: String,
        entry: CapabilityEntryKey,
        mode: ActivationMode,                // None == clear to default
    },
    CapabilityActiveSetChanged {
        owner_id: SimThingId,
        definition_logical_id: String,
        category: CategoryKey,
        active: Vec<CapabilityEntryKey>,     // full new active list per category
    },
    CapabilityNotification(CapabilityTreeNotification),
    ScriptedCooldownChanged {
        owner_id: SimThingId,
        event_id: EventKey,
        cooldown_remaining: u32,
    },
    ScriptedInstanceSlotChanged {
        owner_id: SimThingId,
        event_id: EventKey,
        current_slot: u32,
    },
    ScriptedInstanceRemoved {
        owner_id: SimThingId,
        event_id: EventKey,
    },
    PlayerSelectionQueued {
        owner_id: SimThingId,
        definition_logical_id: String,
        entry: CapabilityEntryKey,
    },
}
```

Emission rule: the driver diffs `SpecSessionState` before and after each
boundary hook, and emits one `SpecDelta` per change. Notifications are
drained from `capability_notifications` and emitted as
`CapabilityNotification` entries, then cleared (matching today's behavior
where the vec is read-and-clear by external consumers).

### Replay format integration

LDJSON stream becomes:

```text
{ "kind": "snapshot", "snapshot": { … } }                         // existing
{ "kind": "spec_snapshot", "spec_snapshot": { … } }               // new, optional
{ "kind": "frame", "day": 1, "entries": [ … ], "spec_entries": [ … ] }
...
```

`spec_snapshot` is omitted entirely when the session has no installed spec
state. Old replays without `spec_snapshot` open cleanly via serde defaults.

### Replay open path

```rust
// crates/simthing-driver/src/spec_replay.rs
pub fn open_replay_with_spec(
    replay_path: &Path,
    game_mode: &GameModeSpec,
    scenario: Scenario,
) -> Result<(SimSession, ReplayDriver), ReplayOpenError> {
    let mut session = SimSession::open_from_spec(scenario, game_mode)?;
    let (replay_snapshot, spec_snapshot_opt, frames) = read_replay(replay_path)?;
    let mut replay_driver = ReplayDriver::from_snapshot(replay_snapshot);
    if let Some(spec_snap) = spec_snapshot_opt {
        apply_spec_snapshot(&mut session.spec_state, &spec_snap)?;
    }
    Ok((session, replay_driver))
}
```

`apply_spec_snapshot` resolves `definition_logical_id` → live
`CapabilityTreeDefinitionId` by walking `session.spec_state.capability_definitions`
and matching on `tree_id`. Mismatch (replay snapshot names a tree no longer
in the spec) is a hard error — replays must be opened against the same
`GameModeSpec` they were recorded with, or against one that is a superset.

For frame-by-frame replay use (e.g., a viewer scrubbing through history),
`replay_driver.apply_spec_delta(&mut session.spec_state, &delta)` is the
parallel of the existing `apply_entry`. It does the same logical-id →
live-id resolution per delta.

### `OverlayId` stability — the load-bearing claim

The hazard is real and would silently corrupt any naive serialization:
two replay runs of the same session produce different overlay ids because
`OverlayId::new()` is a process-local atomic. The two viable mitigations:

- **(M1, chosen)** Never serialize raw `OverlayId` for spec state. Use
  `CapabilityEffectKey` (already present and stable, see
  [`capability_definition.rs`](../../crates/simthing-spec/src/runtime/capability_definition.rs):72–74)
  for capability effects. The instance carries the per-clone `Vec<OverlayId>`
  in memory but they are looked up by `effect_keys` index at replay-apply
  time.
- **(M2, rejected)** Serialize raw `OverlayId`s and add a stable-id remap
  table to `SpecSnapshot`. Rejected because the same hazard re-emerges
  every time a new spec object allocates an `OverlayId` mid-session
  (mutually exclusive idea switch, hot-loaded domain pack, scripted-event
  effect with a fresh overlay). The remap table grows without bound.

(M1) requires the consequence below — `by_overlay` moves from the shared
definition to the per-clone instance — which is the same migration the
session-installation ADR already needs for multi-owner support.
**This ADR and the installation ADR must land in lockstep.**

For structural deltas already emitted by `BoundaryDeltaEntry::OverlayActivated`
(carrying raw `OverlayId`), the existing replay v2 path stays. Those ids are
unstable across processes too, but `ReplayDriver` consumes them only against
the in-memory reconstructed tree — never against spec state — so the
re-stamping problem is contained.

### Boundary ordering

`SpecDelta` emission happens inside `SimSession::execute_with_boundary_hook`,
after the spec hook runs and before `take_delta_log()` writes the frame.
Both `entries` and `spec_entries` reflect the same boundary's mutations.
Replay applies them in this order per frame:

1. `entries` (structural — tree + registry mutations land first).
2. `spec_entries` (capability/scripted state catches up).

Structural-before-spec ordering matches the live-session ordering: the
boundary hook fires, requests are queued, structural mutations apply, spec
state has already mutated locally. Replay does the structural apply then
walks `spec_entries` to mirror what spec state would have been.

## Consequences

(a) **Replay v3 is additive.** v2 replays (no `spec_snapshot`, no
`spec_entries`) open without error. v3 replays opened by a v2-only consumer
ignore the new fields. The bump to "v3" is documentation — the format is
forward/backward compatible.

(b) **`by_overlay` migration becomes mandatory.** Today it lives on the
shared `CapabilityTreeDefinition`. After (M1) + multi-owner install, the
shared map has the same logical key pointing at N different `OverlayId`s.
The map moves to `CapabilityTreeInstance` as `by_overlay: HashMap<OverlayId,
CapabilityEntryKey>`. The session-installation ADR's consequence (c.i) calls
out the same change.

(c) **A "snapshot mid-session" path opens up.** Once the snapshot+delta
serializer exists, the driver can write a fresh `SpecSnapshot` at any
boundary, not just session start. This enables save-game export by writing
`ReplaySnapshot` + `SpecSnapshot` + (current tick value buffer) and skipping
all prior frames. Not part of v3 scope, but the data model permits it.

(d) **Diagnostics are gone from replay deliberately.** A consumer that
wants diagnostics in replay can re-run the session against the replay's
inputs and observe them live. Recording them creates load-bearing state out
of debug spew, which is worse.

(e) **`CapabilityTreeNotification` *is* in replay deliberately.** Notifications
are a contract with downstream consumers (UI, AI hooks, audit logs) — losing
them on replay would cause user-visible divergence. They are emitted as
`SpecDelta`s and replayed.

(f) **Replay tests need fixtures.** A new integration test under
`crates/simthing-driver/tests/` records a session that exercises capability
unlock + scripted cooldown + player selection, reopens it via
`open_replay_with_spec`, and asserts the post-frame `SpecSessionState` is
field-equivalent. The diff-on-each-field assertion is the contract.

(g) **Driver gains a dep boundary check.** `spec_replay.rs` is the only
new file that depends on both `simthing-spec` and `simthing-sim`'s replay
types. It must live in `simthing-driver`. `simthing-sim` keeps its v2
replay code untouched.

(h) **Snapshot growth is bounded.** `CapabilityStateSnapshot` is O(entries
per instance) and `ScriptedCooldownSnapshot` is O(active cooldowns).
A "10 factions × 200 capability entries" save weighs in at ~tens of KB
plain JSON — small relative to the structural snapshot.

## V0 Scope (what O2 implements)

- `SpecSnapshot`, `SpecDelta`, `apply_spec_snapshot`, `apply_spec_delta`,
  `open_replay_with_spec`.
- `ReplayWriter` extension for the `spec_snapshot` line and `spec_entries`
  field. Driver writes both inside `record_to_path`.
- Per-boundary diff-emit in `SimSession::execute_with_boundary_hook` (or a
  small helper invoked from the existing `spec_state.run_boundary_handlers`
  call site).
- `by_overlay` migration from definition → instance (joint with the
  installation ADR).
- One integration test recording + replaying a session with capability
  unlock, scripted cooldown, queued player selection, and a notification.

## Out of scope (deferred)

- Bit-exact GPU value reproduction in replay (already deferred in v2).
- Spec hot-reload mid-replay (the `SpecSnapshot` format would survive it,
  but the install pipeline doesn't yet).
- Compression / binary framing of the LDJSON stream.
- Diagnostics replay (intentionally rejected, see consequence d).
- Save-game export (consequence c — out of scope but unblocked).

## Alternatives considered

- **(Alt-1) Snapshot the full `SpecSessionState` every N frames; no
  deltas.** Rejected — boundary mutation churn is small (a handful of
  cooldowns + occasional activation changes), and re-serializing every
  capability instance every boundary balloons the replay file by 100×.
  Delta-on-mutation is cheaper and matches the structural side.
- **(Alt-2) Embed spec deltas as opaque blobs in `BoundaryDeltaEntry`.**
  Rejected — forces `simthing-sim` to either know about a new
  `BoundaryDeltaEntry::SpecBlob(Vec<u8>)` variant or hand-route bytes
  through the existing serde enum. Cleaner to add `spec_entries` as a
  parallel field at the frame level, which keeps the entry enum sim-only.
- **(Alt-3) Serialize raw `OverlayId`s with a remap table.** Rejected (M2
  above) — the hazard re-emerges for every new id allocation.
- **(Alt-4) Make `OverlayId` itself process-stable (hash-based, or assigned
  at install from a deterministic counter).** Tempting but invasive — every
  `OverlayId::new()` call site in the codebase would need an authority to
  hand out ids. Out of proportion to the replay problem this ADR solves.
  The logical-key approach (M1) gets us the same correctness with no
  changes to `simthing-core`.
