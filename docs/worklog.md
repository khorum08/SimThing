# SimThing — Session Worklog

Running log of what's done and what's next, across sessions.

**Canonical spec:** `docs/design_v6.md` | **Historical spec:** `docs/design_v5.md` | **Agent map:** `docs/agents.md`

---

## 2026-05-22 — B2 fission-growth Approach A: targeted value upload across growth

**Status:** Landed (local). Buffer-preserving slot growth + coalesced
dirty-row upload means growth boundaries no longer flush the entire shadow.

**Problem:**

Before this change, any boundary that grew the GPU slot capacity (fission
pre-grow, AddChild pre-grow, final-capacity ensure) forced
`force_full_value_upload = true`. The reason: `WorldGpuState::rebuild_for_slots`
allocated fresh buffers and the new GPU memory was uninitialized, so the
caller had to re-upload every slot's shadow row to restore consistency.

For sparse fission in real gameplay (1–10 fissions per boundary across an
N-slot world), that meant N slot rows uploaded per growth boundary — most
of which were unchanged.

**Change:**

1. `simthing-gpu/world_state.rs::rebuild_for_slots` now preserves existing
   GPU contents across the resize. One `wgpu::CommandEncoder` issues four
   `copy_buffer_to_buffer` calls (one each for `values`, `previous_values`,
   `output_vectors`, `previous_output_vectors`) before swapping buffers in.
   The new region `[old_n_slots..new_n_slots]` is zero-initialized by
   wgpu's buffer allocation, matching the CPU shadow's `resize` fill.
   Preservation only runs when `n_dims` is unchanged — dimension shifts
   still take the full-rebuild path.
2. `simthing-feeder/dispatcher.rs::upload_row_range(state, slot_start, count)`
   writes a contiguous block of slot rows in a single `queue.write_buffer`,
   avoiding the per-row driver overhead that dominates at thousands of
   dirty slots.
3. `simthing-sim/gpu_sync.rs` value-upload path sorts/dedups
   `dirty_value_slots`, walks them to find contiguous runs, and emits one
   `upload_row_range` per run.
4. `simthing-sim/boundary.rs` no longer sets `force_full_value_upload = true`
   after fission pre-grow, AddChild pre-grow, or final-capacity ensure.
   The previously-allocated slots' shadow data is now correct on GPU
   (preserved), and newly-allocated slot ids are already tracked in
   `dirty_value_slots` via `out.fission.fission_pairs` and
   `out.maintainer.allocated`. Tombstone-induced full-upload and
   dimension-rebuild full-upload paths are unchanged.

**Regression guard:**

- `fission_beyond_initial_headroom_grows_gpu_state` in
  `crates/simthing-sim/tests/boundary_integration.rs` now asserts
  `!outcome.gpu_sync.full_value_upload` and `value_rows_uploaded == 1`
  across a boundary that grows the GPU capacity for a single fission.

**Benchmark deltas (local):**

| Scenario | Metric | Before | After |
|---|---|---|---|
| `fission_stress` (20k fissions in 1 boundary) | `ms_per_sim_day` | ~55 | ~55 |
| `fission_stress` | `boundary_value_rows_uploaded` | 40,000 | 19,999 |
| `fission_stress` | `boundary_full_value_uploads` | 1 | 0 |
| `fission_stress` | `boundary_upload_bytes` | 2,719,944 | 2,479,932 |
| `intent_stress` | `ms_per_sim_day` | ~17 | ~17 |

`fission_stress` is the worst case (every slot dirty), so the per-row
savings are mostly offset by coalescing overhead. The optimization shines
on sparse fission (real gameplay), where upload becomes O(fission_count)
instead of O(n_slots).

**Tests:** `cargo test --workspace` → **202** passed, 1 ignored timing
diagnostic, zero warnings. `bench_stress_scenarios_within_ceiling` still
inside its ceiling.

**Open B2 work (Approaches B and C):**

- Approach B: append-only threshold registry rebuild on growth boundaries.
  Expected ~3–5 ms savings on `fission_stress`.
- Approach C: incremental reduction-topology patching. Higher risk —
  reduction CSR ordering must remain deterministic across growth events.

---

## 2026-05-22 — V6 guardrails complete: Priorities 1, 2, and 3

**Status:** All three V6 guardrail tests landed (local, ahead of `origin/master`).
The Suspended → Permanent overlay contract, the capability-cloning fission
replay contract, and the serde default for `clone_capability_children` are
all locked down.

**Priority 2 — Capability fission replay test:**

- `replay_fission_with_cloned_capability_subtree_reconstructs_full_payload`
  in `crates/simthing-sim/tests/boundary_integration.rs`.
- Tree: `World → Location → Faction(loyalty Amount=0.5, Velocity=-0.21)`,
  Faction has a `Custom("tech_tree")` child with its own `Custom("propulsion")`
  child.
- `FissionTemplate { child_kind: Faction, clone_capability_children: true,
  capability_container_kinds: ["tech_tree"] }` — the spawned faction inherits
  a deep clone of the tech_tree subtree.
- Verified live:
  - Spawned Faction has a cloned tech_tree with fresh id.
  - Cloned tech_tree has its `propulsion` child with fresh id.
  - All cloned nodes have allocated slots.
- Verified delta log payload:
  - `BoundaryDeltaEntry::FissionOccurred { parent, node }` carries the
    full spawned faction subtree, with the cloned tech_tree (id-matched
    to the live tree) and its propulsion child as nested children of
    the `node` payload.
- Verified replay reconstruction:
  - `ReplayWriter` → `ReplayReader` round-trip preserves the snapshot
    and the FissionOccurred frame.
  - `ReplayDriver::apply_frame` re-attaches the spawned faction under the
    original faction, the cloned tech_tree under the spawned faction, and
    the propulsion node under the cloned tech_tree.
  - `populate_from_tree` allocates slots for every node in the cloned
    subtree (spawned faction, tech_tree, propulsion) on the replay side.
  - `FissionLineageAdded` round-trips: `driver.fission_lineage` has the
    same `(parent_id, child_id)` pair as the live boundary.

**Priority 3 — `clone_capability_children` serde default test:**

- `fission_template_deserializes_without_clone_capability_children` in
  `crates/simthing-core/src/property.rs` (unit test alongside the existing
  `capability_container_kinds` default test from PR #38).
- Asserts: legacy JSON without `clone_capability_children` deserializes to
  `false` AND `capability_container_kinds` defaults to `[]`. Together these
  defaults guarantee old saves/scenarios produce pre-V6 fission behavior
  (no capability cloning runs without explicit studio opt-in).

**Tests:** `cargo test --workspace` → **202** passed (up from 200 after
Priority 1, 199 before), 1 ignored timing diagnostic, zero warnings.

**Next:** B2 fission-growth topology batching (Priority 4). With V6
guardrails done, the fission-growth optimization is unblocked. `fission_stress`
is ~60 ms/sim-day locally; the remaining costs are threshold registration
rebuild, reduction topology upload, fission seeding, full value upload after
slot growth, and delta emission. Batch or incrementally patch growth only
while keeping `event_kind` semantics and slot topology provably correct.

---

## 2026-05-22 — V6 guardrail Priority 1: activated overlay GPU test

**Status:** Test landed on `master`. V6 suspension/activation contract is now
locked down end-to-end against the real GPU pipeline.

**Landed:**

- New GPU integration test in
  `crates/simthing-sim/tests/boundary_integration.rs`:
  `activated_suspended_overlay_appears_in_gpu_delta_and_affects_values`.
- Test scenario: cohort with loyalty (Amount=0.5, Velocity=0) carries a
  `Suspended { when_activated: Permanent }` overlay applying Multiply(1.5)
  to loyalty Amount.
- Verified four-step contract end-to-end:
  1. `initial_gpu_sync` + Tick 1: suspended overlay produces zero Pass 3
     deltas; GPU `values[Amount]` stays at 0.5 (verifies `build_overlay_deltas`
     filtering via `Overlay::is_active`).
  2. Empty boundary execute: `overlay_activations == 0`; lifecycle still
     `Suspended` on the CPU tree.
  3. `tx.submit_boundary(BoundaryRequest::ActivateOverlay { .. })` →
     Tick 2 drains it to `patcher.pending_boundary` (value still 0.5 because
     Pass 3 deltas haven't been rebuilt yet).
  4. `proto.execute()` runs `activate_overlay` in `apply_structural_mutations`,
     flipping lifecycle to `Permanent`; `outcome.maintainer.overlays_activated
     == [(cohort_id, overlay_id)]`; `outcome.gpu_sync.overlay_deltas_uploaded
     >= 1`.
  5. Tick 3: Pass 3 applies Multiply(1.5) → `values[Amount] = 0.75`
     (asserted to within 1e-5).

**Why this is the right shape:**

- dt=0 throughout isolates Pass 3 from Pass 1/2 integration so the overlay
  is the only thing that can move the value.
- Two boundaries before activation prove suspended overlays don't trigger
  spurious boundary work (`overlay_activations == 0`).
- One boundary at activation proves the lifecycle transition is observable
  in `MaintainerOutcome`.
- One post-activation tick proves the GPU delta buffer was rebuilt and
  Pass 3 picked it up.

**Tests:** `cargo test --workspace` → **200** passed (up from 199), 1
ignored timing diagnostic, zero warnings.

**Next:** V6 guardrail Priority 2 — end-to-end replay test for fission with
`clone_capability_children: true` and a populated `capability_container_kinds`
list, verifying `FissionOccurred { node }` reconstructs the spawned subtree
including cloned capability children. Then Priority 3 (serde default test
for `clone_capability_children` bool), then B2 fission-growth batching.

---

## 2026-05-22 - Parameterize capability container kinds (PR #38)

**Status:** Merged to `master` (`a8aab5b`, PR #38).

**Problem resolved:**

`simthing-sim` hardcoded `"tech_tree" | "national_ideas" | "talent_tree"` in
two places (`fission.rs` and `boundary.rs`), violating the studio/simulation
boundary: simulation crates must not embed capability-tree semantics.

**Landed:**

- `FissionTemplate::capability_container_kinds: Vec<String>` added in
  `simthing-core/src/property.rs` with `#[serde(default)]`.
- Hardcoded kind matchers removed from production code.
- `pub(crate) fn is_capability_container(kind, container_kinds)` lives in
  `fission.rs` and is reused by `boundary.rs` for `projected_fission_slots`
  pre-grow headroom.
- `execute_fission` passes `&ft.template.capability_container_kinds` into
  `clone_capability_children`.
- **Option A:** empty kinds list + `clone_capability_children: true` clones
  nothing — caller must populate the list explicitly; no sim fallback.
- Backward compat: omitted JSON field deserializes to `[]`; old templates
  without capability semantics therefore clone nothing even if the bool were
  true (safe default).

**Files touched:**

| Crate / doc | Change |
|---|---|
| `simthing-core/property.rs` | New field + serde default test |
| `simthing-sim/fission.rs` | Parameterized filter, shared helper, tests |
| `simthing-sim/boundary.rs` | Pre-grow uses template kinds; test updated |
| `simthing-sim/threshold_registry.rs` | Struct literal field |
| `simthing-sim/tests/boundary_integration.rs` | Struct literal field |
| `simthing-driver/scenario.rs` | Struct literal field |
| `docs/design_v6.md` | Addendum + §8/implementation-status updates |
| `docs/capability_tree_v1.md` | Addendum §11 |
| `docs/agents.md`, `docs/todo.md` | Brief sync |

**Tests added / updated:**

- `fission_template_deserializes_without_capability_container_kinds` (core)
- `clone_capability_children_empty_kinds_clones_nothing` (sim unit)
- `fission_clone_capability_children_remaps_affects_and_copies_shadow` —
  now sets `capability_container_kinds: ["tech_tree"]`
- `projected_fission_slots_counts_cloned_capability_subtrees` —
  now sets `capability_container_kinds: ["tech_tree"]` (asserts 3 slots;
  would fail at 1 if pre-grow still ignored the list)

**Verification:**

- `cargo test --workspace` → **199** passed, **1** ignored, zero warnings.
- No `"tech_tree"` / `"national_ideas"` / `"talent_tree"` string literals
  remain in simulation production paths — only test fixtures and docs.

**Still open after this PR:** V6 guardrails Priorities 1–3 (see `docs/todo.md`).
Priority 3 partially done: `capability_container_kinds` serde default tested;
`clone_capability_children` serde default test still outstanding.

---

## 2026-05-22 - Ingest v5/v6/capability-tree docs into agent briefing

**Status:** Doc sync on `master` after PR #37 (`capability_tree_v1.md`,
`workshop/tech_tree_decisions.md`) and V6 implementation parking.

**Updated:**

- `docs/agents.md` — canonical spec is now `design_v6.md`; added capability-tree
  doc set, V6 implementation summary (`Suspended`, activate/suspend boundary
  requests, capability fission clone), studio-vs-simulation boundary, V6 guardrail
  next items, test count **197** + 1 ignored.
- Cross-reference: `design_v5.md` addendum + `design_v6.md` implementation status
  remain the authoritative spec deltas; `capability_tree_v1.md` is the studio RON
  reference; `workshop/tech_tree_decisions.md` records decided/open workshop items.

**Unchanged implementation queue:** V6 guardrails (Priorities 1–3), then B2
fission-growth topology batching (Priority 4). See `docs/todo.md`.

---

## 2026-05-22 - Parking note: next V6 guardrails queued

**Status:** Todo/worklog-only parking update after documentation commit
`95516b9`.

**Queued next:**

- Priority 1: GPU boundary integration test proving `ActivateOverlay` makes a
  formerly suspended overlay enter the next Pass 3 upload and affect values on
  the following tick.
- Priority 2: End-to-end replay test proving `FissionOccurred { node }`
  reconstructs a fissioned child with its cloned capability subtree payload.
- Priority 3: Serialization compatibility test for old `FissionTemplate` data
  without `clone_capability_children`, confirming serde default `false`.
- Priority 4: Resume B2 fission-growth topology/threshold batching only after
  those V6 guardrails are in place.

**Parking rationale:**

The next work is test-heavy and should not be squeezed into a low-context
window. The todo log now records the exact order: lock V6 behavior down first,
then return to GPU-forward late-game fission optimization.

---

## 2026-05-21 - Parking note after used-range threshold readback

**Status:** Documentation parking update after `5cc4254`.

**Current state:**

- Last shipped optimization: threshold event candidate readback maps only the
  used event range instead of the full candidate buffer.
- Bench output now includes `tick_event_readback_bytes`, making the remaining
  event-readback cost visible in stress runs.
- Verified before parking:
  - `cargo test --workspace` => 188 passed, 1 ignored timing diagnostic.
  - `simthing bench --scenario scenarios/fission_stress.ron --days 1 --check`
    => pass, about 63 ms/sim-day on this machine.
  - `simthing bench --scenario scenarios/intent_stress.ron --days 1 --check`
    => pass, about 18 ms/sim-day on this machine.

**Parking rationale:**

The repo is clean for tracked files and pushed. The next B2 step is not a
one-sitting cleanup; it should be a careful design/implementation pass around
fission-growth topology and threshold registration batching. Do not start it
without enough room to run full GPU integration tests and stress guards.

**Next safe target:**

Design a fission-growth batching plan that preserves the current authority
doctrine. Prefer retaining or append-patching GPU topology/threshold buffers
only when slot assignment and event-kind semantics remain provably stable.

---

## 2026-05-22 - V6 suspended overlays and capability fission landed

**Status:** Merged to master (`f39fe6d`) and documented for parking.

**Landed:**

- `OverlayLifecycle::Suspended { when_activated }` is now part of the core
  overlay model.
- CPU evaluation and GPU overlay prep ignore suspended overlays; Pass 3 only
  receives active overlay deltas.
- Boundary requests now include `ActivateOverlay` and `SuspendOverlay`.
- Tree mutation activates suspended overlays by restoring their parked lifecycle
  and suspends active overlays by wrapping the current lifecycle.
- Delta log and replay now capture `OverlayActivated` and `OverlaySuspended`.
- Observability reports `OverlayContribution.active`, allowing UI/debug tools
  to distinguish present-but-inert overlays from active effects.
- Empty static boundaries can still skip when only suspended overlays are
  present.
- `FissionTemplate::clone_capability_children` landed with serde default
  `false`, preserving existing fission behavior unless explicitly enabled.
- Opted-in fission now deep-clones capability containers listed in
  `FissionTemplate::capability_container_kinds` into the spawned child (see
  PR #38 — hardcoded kind names removed 2026-05-22), assigns fresh IDs,
  allocates slots, copies shadow rows, and remaps overlay `affects` from parent
  owner to spawned owner.
- Boundary fission pre-grow now accounts for cloned capability subtree slots
  before fission writes shadow rows.

**Tests:**

- `cargo test` passed across the workspace before the implementation commit.
- Focused new coverage includes suspended overlay GPU-prep filtering,
  activation/suspension tree mutation, lifecycle replay, delta-log entries,
  observability active attribution, empty-boundary skip behavior, capability
  subtree cloning, overlay-affects remap, shadow-row copy, and fission slot
  headroom estimation.

**Docs updated:**

- `docs/design_v5.md` now points at V6 and includes a V6 implementation
  addendum.
- `docs/design_v6.md` now has an implementation-status addendum.
- `docs/todo.md` was created as the current parking todo log.

**Next safe targets:**

- Add a GPU boundary integration test for activation causing next-tick Pass 3
  effect.
- Add an end-to-end replay test for fission with cloned capability subtree.
- Continue B2 topology/threshold batching for fission-growth boundaries, with
  slot ordering and `event_kind` determinism treated as hard invariants.

---

## 2026-05-21 - Fission path lookup optimization

**Status:** Merged to master (`166eb5b`).

**Landed:**

- Fission resolution now builds a one-time `SimThingId -> tree path` index for
  the boundary and reuses it for secondary-condition checks, child seeding, and
  child attachment.
- This removes repeated root-to-node scans for every fission event. The old
  shape was quadratic on wide trees, which is exactly what `fission_stress`
  exposed.

**Observed smoke result:**

- `fission_stress`, 20k to 40k slots in one boundary, dropped from ~6.3s
  boundary time to ~1.23s boundary time while still executing 19,999 fissions.

**Tests:** `cargo test --workspace` => 182 passed, 1 ignored timing diagnostic.

**Next optimization:** Continue splitting the remaining fission boundary cost:
threshold registry rebuild, topology rebuild, full shadow upload, and delta-log
generation are now more likely than parent lookup to dominate.

---

## 2026-05-21 - Fission delta-log indexing and boundary attribution

**Status:** Merged to master (`26dc4e8`).

**Landed:**

- `BoundaryOutcome` now carries `BoundaryTiming`, and `simthing bench` prints
  boundary phase totals: GPU readback, alert collection, lifecycle, expiry,
  fission pregrow, fission, lineage, request drain, AddChild pregrow,
  structural mutation, dimension rebuild, final capacity growth, GPU sync, and
  delta-log generation.
- `delta_log::entries_from_outcome` now builds a one-pass tree index for
  `SimThingId -> &SimThing` and `SimThingId -> parent_id` lookup, then emits
  fission/add/overlay payload entries with O(1) lookups instead of rescanning
  the whole tree per emitted delta.

**Observed smoke result:**

- `fission_stress`, 20k to 40k slots in one boundary, now runs at ~53
  ms/sim-day. Boundary time is ~30 ms and delta-log generation is ~7.6 ms,
  down from ~1.09 s before indexing.

**Tests:** `cargo test --workspace` => 182 passed, 1 ignored timing
diagnostic.

**Next optimization:** With parent lookup and delta-log generation no longer
dominating, the remaining fission stress cost is the useful GPU-facing work:
threshold event readback, fission seeding, GPU sync/topology upload, and
threshold/reduction rebuilds. Next pass should target batching/retaining those
GPU buffer updates rather than adding more CPU-side semantics.

---

## 2026-05-21 - Benchmark attribution and boundary fast path

**Status:** Merged to master (`0af46f4`).

**Landed:**

- `TickOutcome` now reports phase timing for queue drain / intent folding,
  intent upload, dirty-row upload, GPU pipeline submission, and threshold event
  readback.
- `RunSummary` and `simthing bench` now aggregate tick phase timing, boundary
  time, boundary readback bytes, boundary upload bytes, overlay deltas,
  threshold registrations, reduction edges, reduction slots, and reduction
  depth counts.
- Boundary GPU sync reports reduction edge/slot counts and an estimated upload
  byte total for values, overlays, thresholds, topology, and column rules.
- Dispatcher skips threshold event readback entirely when no thresholds are
  registered, and skips candidate-buffer readback when the event count is zero.
- Static no-op boundaries now skip full GPU value readback, lifecycle passes,
  GPU buffer rebuild, and full shadow upload when there are no threshold events,
  no pending boundary/intents, and no transient overlay or CPU-decay work.
- Dirty-row tracking now keeps a sparse slot list instead of scanning the full
  slot bitmap every tick, removing hidden O(n_slots) overhead from static
  million-slot runs.

**Observed smoke result:**

- `intent_stress`, 100k slots, 4 ticks/day now runs at ~20 ms/sim-day with
  `boundaries_skipped: 1`, zero boundary readback/upload bytes, and zero RMW
  readbacks.
- `map_1m_light`, 1M slots, 8 ticks/day now runs at ~25 ms/sim-day with
  `boundaries_skipped: 1`; sparse dirty rows reduce dirty upload accounting to
  ~0.001 ms/day when no rows are dirty.
- `fission_stress`, 20k to 40k slots, reports boundary-dominant runtime:
  ~6.25 s boundary time, ~60k threshold regs, ~40k reduction slots, and
  ~40k reduction edges.

**Tests:** `cargo test --workspace` => 182 passed, 1 ignored timing diagnostic.

**Next optimization:** Profile and reduce CPU fission/tree-growth cost in
`fission_stress`; static map and intent scenarios are now mostly GPU-submit /
queue-drain bound rather than boundary-sync bound.

---

## 2026-05-20 - GPU intent delta hot path

**Status:** Merged to master (`8fe858b`).

**Landed:**

- Tick-time feeder/player/AI transforms now fold into per-cell affine
  `IntentDelta` records and apply on the GPU before Pass 0.
- Same-cell operation order is preserved while eliminating blocking
  `read_values_row` RMW refreshes from the dispatcher hot path.
- `TickOutcome`, `RunSummary`, and `simthing bench` now report
  `intent_deltas_uploaded` and `intent_delta_bytes`; RMW row-sync metrics
  remain and should stay zero for normal tick transforms.
- Feeder integration coverage now verifies Set folding, Add/Multiply folding,
  zero RMW readback, and one intent delta for many same-cell patches.

**Tests:** `cargo test --workspace` => 177 passed, 1 ignored timing diagnostic.

**Next optimization:** Expand benchmark metrics so stress runs attribute time
to upload, tick, boundary, reduction, threshold, and growth work.

---

## 2026-05-20 - Consolidated tick command submission

**Status:** Merged to master (`8fe858b`).

**Landed:**

- `Pipelines::run_tick_pipeline(state, dt)` records intent deltas, snapshot,
  velocity, intensity, overlay application, reduction, and threshold scan into
  one command encoder and submits once.
- Dispatcher ticks now call the consolidated pipeline instead of submitting
  each pass separately.
- Reduction depths use per-depth uniform buffers in the consolidated path, so
  queued depth dispatches preserve their individual `(depth_offset, bucket_size)`
  parameters.
- Linear GPU workloads now dispatch across 2D workgroup grids when needed,
  keeping `snapshot`, velocity, intensity, overlays, intents, reduction, and
  threshold scan inside WebGPU's per-axis dispatch limit at large slot counts.
- Added GPU parity coverage:
  `run_tick_pipeline_matches_manual_pass_sequence`.

**Next optimization:** Add per-phase benchmark attribution and counters for the
stress scenarios now on master.

---

## 2026-05-20 - Builtin benchmark stress scenarios

**Status:** Merged to master (`8fe858b`).

**Landed:**

- Added builtin benchmark scenario selectors:
  - `scenarios/map_1m_light.ron`
  - `scenarios/pop_heavy.ron`
  - `scenarios/intent_stress.ron`
  - `scenarios/fission_stress.ron`
  - `scenarios/threshold_stress.ron`
- Scenario construction now projects the semantic tree into the initial shadow
  before applying explicit shadow seed overrides, so large benchmark trees do
  not need one seed entry per node.
- Added `Scenario::tick_patches` and session submission so `intent_stress`
  exercises the normal feeder/dispatcher GPU intent-delta path every tick.
- Session startup projects initial semantic trees into the allocated prefix of
  the shadow and preserves scenario headroom, avoiding seed-time panics when
  `n_slots` is intentionally larger than the tree's current allocation.

**Smoke measurements:**

- `intent_stress`, 100k slots, 4 ticks/day: ~295 ms/sim-day, 80k intent deltas,
  0 RMW readback bytes.
- `pop_heavy`, 250k slots, 32 dims, 4 ticks/day: ~241 ms/sim-day.
- `map_1m_light`, 1M slots, 3 dims, 8 ticks/day: ~4566 ms/sim-day.
- `fission_stress`, 20k to 40k slots in one boundary: ~4889 ms/sim-day,
  19,999 fissions.

**Next optimization:** Extend benchmark output with overlay delta counts,
threshold registrations, reduction edges/depths, and boundary readback/sync
bytes so stress runs explain where time is going.

---

## 2026-05-20 - GPU growth and semantic hardening

**Status:** Merged to master (`4b5f1c6`).

**Landed:**

- `overlay_lifecycle` now requires semantic property presence before reading
  dense shadow values for `PropertyBelow` / `PropertyReaches`, so absent
  properties no longer dissolve overlays because their column happens to be 0.
- Overlay expiration uses safe registry accessors; invalid or inactive
  transform property ids no longer panic lifecycle resolution.
- `FissionThreshold.dimension` was removed. Fission thresholds now clearly
  watch the owning property's `sub_field`; future cross-property fission should
  use explicit `watched_property` / `fission_property` fields.
- `TransformPatcher::apply_one` now takes `ShadowFreshness`. Add/Multiply skip
  with `unsafe_rmw_skipped` unless the caller supplies `GpuSynced`; the
  dispatcher still refreshes RMW rows before applying collected work.
- Boundary slot growth now resizes `DispatchCoordinator`, `TransformPatcher`,
  and `WorldGpuState` with amortized doubling. Fission/AddChild can grow past
  initial headroom without panicking, with shadow as the preservation source.
- Tick/session outcomes now accumulate RMW row-sync count and readback bytes.
  `simthing bench --scenario <file.ron> [--days N]` reports timing, slot growth,
  RMW readback cost, and final GPU buffer bytes.

**Tests:** `cargo test --workspace` => 173 passed, 1 ignored timing diagnostic.

**Next optimization (superseded — landed `8fe858b`):** Replace per-slot blocking
RMW row readbacks with a GPU-side intent delta buffer/pass.

---

## 2026-05-22 — A1–A4: fold reuse, observability docs, smoke, tree index

**Status:** Merged to master (`de1d16d`, PR #34).

**Landed:**

- **A1:** `TransformPatcher` reuses `fold_order` / `fold_accum` across ticks
  (`clear()` per drain) instead of allocating a fresh `HashMap` every tick.
- **A2:** `state-authority.md` and `observability.rs` document mid-tick shadow
  staleness on intent-patched rows; `observe_live` is the GPU-fresh path.
- **A3:** Smoke pass — `rebellion_demo.ron` record (3 days) → `demo.replay.ldjson`
  → replay: 3 frames, 4 tree nodes, 1 fission + 1 lineage entry. Pass.
- **A4:** New `tree_index` module (`build_node_paths`, `detach_at_path`).
  Fission takes a pre-built index; boundary rebuilds index before structural
  mutations; `apply_structural_mutations` uses O(1) path lookup when indexed.

**Tests:** `cargo test --workspace` => 184 passed, 1 ignored timing diagnostic.

---

## 2026-05-22 — R2 remainder, bench guard, replay hardening

**Status:** Merged to master (`8a0f28f`, PR #36).

**Landed:**

- **R2:** `tree_index::paths_preorder`; lifecycle + expiry use shared boundary index;
  fission reuses the same pre-fission index (lifecycle/expiry do not change tree shape).
- **Bench guard:** `simthing bench --check` + `bench_limits` ceilings for
  `intent_stress` / `fission_stress`; GPU integration test `bench_stress_scenarios_within_ceiling`.
- **Replay hardening:** record/replay test asserts frame count, final day, entry kinds
  (`FissionOccurred`, `FissionLineageAdded`), lineage parity with live session.

**Tests:** `cargo test --workspace` => 186 passed, 1 ignored timing diagnostic.

---

## 2026-05-22 — B1 targeted boundary value upload

**Status:** Ready to land; tests passing.

**Landed:**

- `sync_gpu_buffers` accepts an optional boundary dirty-slot list. When safe,
  it uploads only rows touched by boundary CPU work instead of always flushing
  the full `values` shadow back to GPU.
- Full value upload remains the fallback after slot growth, dimension rebuild,
  or conservative tombstone cases. The full boundary GPU readback is unchanged.
- Boundary/bench metrics now report `boundary_value_rows_uploaded` and
  `boundary_full_value_uploads`.
- Added GPU integration coverage proving an overlay-only active boundary
  attaches the overlay, preserves the GPU intent value, and avoids a full
  value flush.

**Tests:** `cargo test --workspace` => 187 passed, 1 ignored timing diagnostic.
`simthing bench --scenario scenarios/fission_stress.ron --days 1 --check` and
`simthing bench --scenario scenarios/intent_stress.ron --days 1 --check` pass.

**Next optimization:** B2 — retain or batch threshold/reduction topology on
fission growth boundaries. B1 deliberately keeps full value upload after GPU
buffer rebuilds, so topology/threshold upload now remains the larger fission
growth target.

---

## Next session pickup

**202/202** tests passing plus 1 ignored timing diagnostic, zero warnings.
`master` includes V6 guardrails Priorities 1–3 (PR #39, `e275789`). Local
ahead of `origin/master`:

- V6 suspended overlays + capability fission clone (`f39fe6d`)
- Parameterized `capability_container_kinds` — no sim hardcoding (PR #38,
  `a8aab5b`)
- V6 Priority 1 guardrail: activated overlay GPU integration test
  (PR #39, `e275789`)
- V6 Priority 2 guardrail: capability fission replay test (PR #39, `e275789`)
- V6 Priority 3 guardrail: `clone_capability_children` serde default test
  (PR #39, `e275789`)
- B2 Approach A: targeted value upload across fission growth (local,
  2026-05-22)
- Capability-tree concept docs (PR #37), agent briefing sync (`07076b4`)
- GPU intent-delta hot path, consolidated tick submission, stress scenarios,
  benchmark attribution, static-boundary skipping, fission path indexing,
  R2 tree-index sharing, bench guards, replay hardening, B1 targeted boundary
  value upload, B2 stable-buffer retention, used-range threshold readback

**Design reference:** `docs/design_v6.md` (current, incl. addenda) ·
`docs/design_v5.md` (historical) · `docs/capability_tree_v1.md` (studio) ·
`docs/chatgpt_implementation_review.md`

### Todo (recommended order)

#### Done

- [x] **Per-entity ids in outcome structs** — PR #20.
- [x] **`WeightedMean { by: SimPropertyId }` reduction variant** — PR #21.
- [x] **Thresholds on `output_vectors`** — PR #22.
- [x] **State authority hardening** — PR #23.
- [x] **Replay serialization + playback v1** — PR #25.
- [x] **Fusion lineage registration + scar semantics** — PR #26.
- [x] **Replay v2** — PR #27.
- [x] **State authority doctrine + lineage prune fix** — PR #28.
- [x] **Fission re-fire policy** — recurring rebellions intentional (no suppression).
- [x] **Recording harness + sim driver + rebellion demo scenario** — PR #29.
- [x] **Driver GPU integration tests** — `session_integration.rs` (run + record/replay).

- [x] **GPU growth + patch-authority hardening** - `4b5f1c6`.
- [x] **GPU intent deltas + stress harness + dispatch scaling** - `8fe858b`.
- [x] **Eliminate per-slot blocking RMW readbacks** — GPU intent delta buffer/pass
      (`8fe858b`).
- [x] **Consolidate GPU command submission** — one-encoder `run_tick_pipeline`
      (`8fe858b`).
- [x] **Add synthetic performance stress scenarios** — `map_1m_light`, `pop_heavy`,
      `intent_stress`, `fission_stress`, `threshold_stress` (`8fe858b`).
- [x] **Expand benchmark metrics** — overlay/threshold/reduction counts, boundary
      sync/readback bytes, per-phase timing (`0af46f4`).
- [x] **Profile benchmark bottlenecks** — attribution separates tick vs boundary
      work (`0af46f4`).
- [x] **Optimize boundary sync/readback** — static skip + sparse dirty rows
      (`0af46f4`).
- [x] **Profile fission/tree-growth CPU cost** — boundary phase timing + indexed
      delta-log emission (`26dc4e8`, `166eb5b`).
- [x] **Reuse intent-fold accumulators on `TransformPatcher`** — PR #34 (A1).
- [x] **Document mid-tick `observe` vs `observe_live` staleness** — PR #34 (A2).
- [x] **Record/replay smoke (`rebellion_demo`)** — PR #34 (A3).
- [x] **Share boundary tree index with structural mutations** — PR #34 (A4,
      `tree_index` module).
- [x] **Extend shared tree index to lifecycle/expiry (R2).** PR #36.
- [x] **Bench regression guard (`simthing bench --check`).** PR #36.
- [x] **Replay record/replay integration hardening.** PR #36.
- [x] **Boundary dirty-row shadow upload (B1).** Targeted boundary value-row
      uploads with full-upload fallback for rebuild/tombstone cases.
- [x] **Safe B2 stable-buffer retention.** Topology-stable active boundaries
      retain threshold and reduction buffers (`f470c5e`).
- [x] **Used-range threshold event readback.** Candidate readback maps only
      fired-event bytes and reports `tick_event_readback_bytes` (`5cc4254`).
- [x] **V6 simulation core** — suspended overlays, activate/suspend, capability
      fission clone (`f39fe6d`).
- [x] **Parameterize capability container kinds (PR #38).** No hardcoded
      `Custom(...)` labels in `simthing-sim`; `capability_container_kinds`
      on `FissionTemplate`; Option A empty-list semantics; serde default test
      for kinds field.
- [x] **V6 guardrail Priority 1 — activated overlay GPU test (2026-05-22).**
      `activated_suspended_overlay_appears_in_gpu_delta_and_affects_values`
      in `crates/simthing-sim/tests/boundary_integration.rs`. Verifies
      Suspended → Permanent transition via `BoundaryRequest::ActivateOverlay`
      makes a formerly-suspended overlay enter the Pass 3 delta buffer and
      apply on the following tick (0.5 → 0.75 via Multiply(1.5)).
- [x] **V6 guardrail Priority 2 — capability fission replay test (2026-05-22).**
      `replay_fission_with_cloned_capability_subtree_reconstructs_full_payload`
      in `crates/simthing-sim/tests/boundary_integration.rs`. Drives a faction
      fission with `clone_capability_children: true` + `["tech_tree"]`; verifies
      `FissionOccurred { node }` carries the full 2-level cloned tech_tree
      subtree and `ReplayDriver` reconstructs every node with allocated slots
      and lineage round-trip.
- [x] **V6 guardrail Priority 3 — `clone_capability_children` serde default
      (2026-05-22).** `fission_template_deserializes_without_clone_capability_children`
      in `crates/simthing-core/src/property.rs`. Legacy JSON without the
      field deserializes to `false`; capability cloning never runs without
      explicit studio opt-in.

#### Next

- [ ] **Retain/batch topology on fission growth boundaries (B2, Priority 4).**
      `fission_stress` is roughly 60 ms/sim-day on the current local smoke run.
      Stable-boundary retention and used-range event readback are done; remaining
      work is fission growth itself: threshold registration rebuild, reduction
      topology upload, fission seeding, full value upload after slot growth, and
      delta emission. Batch or incrementally patch growth only if event-kind
      semantics and slot topology remain provably correct.
- [ ] **Capability-tree studio layer.** Authoring/instantiation per
      `capability_tree_v1.md` and `workshop/tech_tree_decisions.md` — studio
      populates `capability_container_kinds` on faction fission templates;
      simulation crates stay agnostic.
- [ ] **Document/prototype map-scale representation.** Keep current `SimThing` as
      semantic authoring state; evaluate arena/topology sidecars only after benchmark
      data shows tree representation pressure.
- [ ] **Scenario format expansion.** Full RON tree/registry/shadow seeds — behind
      the GPU performance path.

**Recent:** V6 guardrails Priorities 1–3 all landed on 2026-05-22. Suspended →
Permanent overlay transitions are GPU-verified; capability fission cloning
round-trips through the replay log with full payload; legacy
`FissionTemplate` JSON without `clone_capability_children` defaults to `false`.
Next: B2 fission-growth batching (Priority 4).

**Tabled:** `simthing-studio` designer UI; unified `BoundaryIndex` single-pass
boundary walk (review item 4 / C1 — Opus-tier, defer until B2 lands).

---

## 2026-05-20 — Replay v2: full spawned-subtree payload + lineage entries (PR #27)

**Status:** Merged to master (`c1f9b07`). Delta log is no longer lossy.

**Landed:**

- `simthing-sim::fission`:
  - `FissionLineageRecord` now derives `Serialize, Deserialize` (required
    for embedding in delta log entries).

- `simthing-sim::delta_log`:
  - `BoundaryDeltaEntry::SimThingAdded` changed from `{ id }` to
    `{ parent: SimThingId, node: SimThing }`. `entries_from_outcome` walks
    the post-boundary tree via new `find_node_with_parent` helper to embed
    the full subtree; silently skipped when not found.
  - `BoundaryDeltaEntry::FissionOccurred` changed from `{ parent, child }`
    to `{ parent: SimThingId, node: SimThing }`. Tree-walk approach; node.id
    is the former child.
  - New `FissionLineageAdded { record: FissionLineageRecord }` — emitted once
    per entry in `outcome.fission.lineage_added`.
  - New `FissionLineageRemoved { record: FissionLineageRecord }` — emitted once
    per entry in `outcome.fission.lineage_removed`.
  - All delta_log tests updated to build proper trees so fission/add entries
    are actually emitted (previously fake ids returned None from tree walk).
  - New test: `fission_lineage_changes_produce_entries`.
  - New test: `sim_thing_added_skipped_when_id_not_in_tree`.

- `simthing-sim::replay`:
  - `ReplaySnapshot` gains `fission_lineage: Vec<FissionLineageRecord>`
    with `#[serde(default)]` for backward compat.
  - `ReplayDriver` gains `pub fission_lineage: Vec<FissionLineageRecord>`,
    seeded from the snapshot's lineage vec.
  - `ReplayDriver::apply_entry` handles all previously-lossy variants:
    - `SimThingAdded { parent, node }`: `allocator.populate_from_tree(&node)`,
      then attach under parent.
    - `FissionOccurred { parent, node }`: same as SimThingAdded.
    - `FissionLineageAdded { record }`: push to `self.fission_lineage`.
    - `FissionLineageRemoved { record }`: retain filter.
  - New tests: `driver_replays_sim_thing_added`,
    `driver_replays_fission_occurred_with_node`,
    `driver_replays_fission_lineage_round_trip`,
    `snapshot_carries_fission_lineage_through_serde`.

- `simthing-sim::boundary`:
  - `BoundaryProtocol::snapshot()` now includes `fission_lineage` field.

**Test count:** 151/151 passing (was 145), 1 ignored, zero warnings.

---

## 2026-05-20 — Fusion lineage registration + scar semantics

**Status:** Landed on `claude/fusion-lineage`. The fusion path is real:
fission produces a lineage record, the next boundary's threshold
registration adds a `FusionTrigger` watching the child's Intensity, and
on fire the parent's activating-property Amount is scarred multiplicatively.

**Landed:**

- `simthing-sim::fission`:
  - `FissionLineageRecord { parent_id, child_id, property_id, template_idx }`
    — one per successful fission, the durable handle that subsequent
    boundaries use to reconstruct the fusion threshold.
  - `FissionOutcome.lineage_added` / `.lineage_removed` carriers.
  - `execute_fission` emits a `lineage_added` entry per spawned child.
  - `execute_fusion` now takes the values shadow + n_dims and calls
    `apply_fusion_scar`: `parent.amount *= (1 - fusion_scar_coefficient)`
    on the activating property's Amount column. Skips silently on any
    lookup miss (tombstoned property, out-of-range template, missing
    slot, no Amount sub-field).
- `simthing-sim::threshold_registry`:
  - `ThresholdBuilder::build_with_lineage` accepts `&[FissionLineageRecord]`
    in addition to velocity/aggregate alerts. For each record it emits one
    `FusionTrigger` registration: child slot + activating property's
    Intensity column, threshold = `template.fusion_intensity_threshold`,
    direction = Upward. Tombstoned property / unallocated child silently
    skipped.
  - `build_with_alerts` now delegates with an empty lineage slice; old
    callers keep their behavior.
- `simthing-sim::boundary`:
  - `BoundaryProtocol.fission_lineage: Vec<FissionLineageRecord>` —
    persistent across boundaries.
  - `execute` appends `lineage_added`, removes `lineage_removed`, then
    prunes any record whose parent or child no longer has a slot
    (catches Remove + post-fusion tombstones).
  - `sync_gpu_buffers` now takes `&fission_lineage` and threads it to
    `build_with_lineage`.
  - `BoundaryProtocol::fission_lineage()` read-only accessor.

**Tests (145 passing, up from 140 — zero warnings):**

- `fission::tests::fission_emits_lineage_record_per_successful_spawn` —
  verifies one record per fission with the right ids + template_idx.
- `fission::tests::fusion_applies_scar_to_parent_amount_and_tombstones_child`
  — direct unit: feeds a `FusionTrigger` event, asserts parent Amount goes
  from 1.0 → 0.95 and child tombstoned.
- `threshold_registry::tests::fusion_lineage_emits_one_intensity_threshold_per_record`
  — lineage record produces a registration on the child's Intensity (col 2)
  at threshold 0.85, direction Upward.
- `threshold_registry::tests::fusion_lineage_skipped_when_child_has_no_slot`
  — tombstoned child gets no FusionTrigger registration (no GPU upload of
  a phantom slot).
- `tests/boundary_integration.rs::fission_then_fusion_applies_scar_and_tombstones_child`
  — GPU end-to-end. Drives a cohort across the 0.3 loyalty threshold
  (fission fires), patches the spawned child's velocity to +0.21 so Pass 2
  builds its Intensity past 0.85 over five ticks (fusion fires), runs
  another boundary, asserts parent Amount was scarred to ~95% of its
  pre-fusion value, child is gone from tree + allocator, lineage record
  pruned.

**Carry-over (not blocking, documented in Next session pickup):**

- Replay v2 needs to record `FissionLineageRecord`s in the delta log too,
  otherwise replay reconstructs a tree where fission happened but no fusion
  threshold gets registered on subsequent boundaries. The lineage vec is
  in-memory only today.
- Fission re-fire suppression: a parent that already fissioned still carries
  a `FissionTrigger` registration on its Amount column. A second crossing
  spawns another child. May be desired (recurring rebellions); design call
  needed if not.

---

## 2026-05-20 — Replay serialization + playback v1

**Status:** Landed on `claude/replay-serialization`. Replay is real:
captured-state snapshot + per-boundary delta frames → LDJSON file →
read back into a `ReplayDriver` that reconstructs the tree, registry,
and slot allocator.

**Landed:**

- `crates/simthing-sim/src/replay.rs` — new module:
  - `ReplaySnapshot { day, root, registry }` — initial-state baseline.
  - `ReplayFrame { day, entries: Vec<BoundaryDeltaEntry> }` — one
    boundary's structural deltas.
  - `ReplayRecord` discriminated record (snapshot vs frame) with
    `#[serde(tag = "kind")]`, written one-per-line.
  - `ReplayWriter<W: Write>` — `write_snapshot` then any number of
    `write_frame`s. Refuses frames before snapshot.
  - `ReplayReader<R: BufRead>` — `read_snapshot` + iterated
    `next_frame -> Option<...>`. Refuses unexpected snapshots
    mid-stream.
  - `ReplayDriver { day, root, registry, allocator }` —
    `from_snapshot` allocates slots, `apply_frame` walks entries.
    `OverlayAttached`, `PropertyExpired`, `SimThingReparented`,
    `DimensionAdded`, `SimThingRemoved`, `FusionOccurred` reconstruct
    structurally; `SimThingAdded` / `FissionOccurred` are lossy
    (id-only payload — see "Replay v2" in Next session pickup).
- `BoundaryDeltaEntry`:
  - `#[derive(Serialize, Deserialize)]` (PartialEq dropped — `Overlay`
    carries `f32`s via `PropertyTransformDelta`).
  - `OverlayAttached` now carries `{ target: SimThingId, overlay:
    Overlay }`. `entries_from_outcome(outcome, root)` walks the tree
    to resolve the full `Overlay` payload from the maintainer's
    `(target, OverlayId)` pair.
- `MaintainerOutcome::overlays_attached` changed to
  `Vec<(SimThingId, OverlayId)>` so the delta log can look up the full
  overlay struct without losing the target.
- `BoundaryProtocol::snapshot(day)` — returns a `ReplaySnapshot` clone
  of current state. Cheap; intended for once-per-recording.
- `simthing-core`:
  - `DimensionRegistry` now derives `Clone`.
  - `SimThing.properties` and `DimensionRegistry.by_name` use
    `#[serde_as(as = "Vec<(_, _)>")]` to serialize non-string-keyed
    maps as JSON arrays of pairs.
- `serde_with` added to workspace + simthing-core deps.

**Format chosen:** line-delimited JSON. Trades raw throughput for
grep/diff debuggability; binary frame format can replace `Write` /
`Read` impls behind the same trait surface later.

**Scope:** structural reproduction. Float values from velocity
integration + overlay application are recomputed each session and are
not part of the replay surface. Verifying bit-exact value
reproduction across hardware would require capturing GPU readbacks
alongside the delta log — a separate feature.

**Tests (140 passing, up from 132 — zero warnings):**
- 1 new delta_log unit (`overlay_attached_skipped_when_not_in_tree`).
- 6 new replay unit:
  - `snapshot_round_trips_through_ldjson`
  - `writer_rejects_frame_before_snapshot`
  - `reader_returns_none_after_last_frame`
  - `driver_replays_overlay_attached`
  - `driver_replays_property_expired`
  - `driver_replays_reparent`
- 1 new GPU integration test
  (`replay_round_trip_reconstructs_overlay_and_dimension_changes`):
  drives a real `BoundaryProtocol` through `AttachOverlay` and
  `AddDimension` requests, captures snapshot + 2 frames into an
  in-memory LDJSON buffer, reads back, replays, asserts the overlay
  is re-attached on the right SimThing.

**Carry-over for replay v2 (Sonnet-feasible once shape is decided):**
`SimThingAdded` / `FissionOccurred` lose the spawned subtree payload
in the log today. Extending `MaintainerOutcome::allocated` and
`FissionOutcome::fission_pairs` to carry the full spawned `SimThing`
(or adding a `SimThingSpawned { parent, node }` variant) closes the
gap. The `ReplayDriver` already has the helpers (`find_node_mut`,
slot allocation via `populate_from_tree`) to consume it.

---

## 2026-05-20 — State authority hardening (PR #23)

**Status:** Merged to `master` as PR #23 (`77357ad`).

**Why:** Cursor's feature expansion left several authority/lifecycle edges
ambiguous: stale within-day shadow read-modify-write, stale TowardZero expiry,
local-subtree tombstoning, AddChild/Remove shadow hygiene, and secondary fission
checks using the wrong property.

**Landed:**
- `Pipelines::run_threshold_scan` resets `event_count` before the zero-threshold
  early return.
- `TransformPatcher` applies only safe `Set` writes in the within-day shadow
  path; `Add`/`Multiply` are skipped and counted via `unsafe_rmw_skipped`.
- `resolve_property_expiry` now receives allocator + synchronized shadow +
  `n_dims`; TowardZero checks shadow values and tombstones only after a
  whole-tree liveness pass.
- `AddChild` projects initialized child/subtree properties into the CPU shadow;
  `Remove` zeros tombstoned subtree rows.
- Fission secondary checks read Amount/Intensity from the triggering property.
- Fusion docs now state the current truth: placeholder handler exists, but
  automatic fusion threshold registration/scar semantics remain unwired.

**Tests:** 132 passing, 1 ignored timing diagnostic, zero warnings.

---

## 2026-05-19 — Session cutoff (after PR #22)

**Status:** Stopping here. Step 1 (output-vector thresholds) shipped as PR #22.
Sonnet-tier pickup exhausted; replay is the sole remaining recommended todo.

**Handoff for Opus replay:**
1. Decide on-disk format (binary frames vs line-delimited JSON).
2. Embed full `Overlay` in `OverlayAttached` (or a parallel replay record).
3. Implement write path from `take_delta_log()` + optional periodic snapshots.
4. Implement playback driver that reapplies deltas through `BoundaryProtocol`.

---

## 2026-05-19 — Thresholds on `output_vectors` (Step 1)

**Status:** Merged to `master` as PR #22 (`6ef455b`).

**Landed:**
- `ThresholdRegistration.buffer` (`THRESH_BUF_VALUES` / `THRESH_BUF_OUTPUT`).
- `previous_output_vectors` buffer; Pass 0 snapshots `output_vectors` into it.
- Pass 7 shader + CPU oracle select values vs output buffer pair.
- `AggregateAlertRegistration`, `AggregateAlertEvent`, `ThresholdSemantic::AggregateAlert`.
- `BoundaryOutcome::aggregate_alerts`; `build_with_alerts` in gpu sync.
- GPU unit test `threshold_scan_on_output_vectors_matches_cpu_oracle`.
- Integration test `aggregate_alert_registration_surfaces_at_boundary`.

**Tests:** 128 passing (2 new), zero warnings.

---

## 2026-05-20 — WeightedMean reduction variant

**Status:** Merged to `master` as PR #21 (`97959bd`).

**Landed:**

- `simthing-core`: `ReductionRule::WeightedMean { by: SimPropertyId }`.
- `simthing-gpu`:
  - `ColumnRuleDescriptor`, `build_column_rule_descriptors`,
    `encode_column_rules` — weight column = `Amount` of property `by`.
  - `column_rules` GPU buffer doubled (`n_dims * 2` u32s).
  - `reduction.wgsl` — `RULE_WEIGHTED_MEAN = 5`, explicit multiply/add for
    `weighted_sum / weight_total`; zero total weight → 0.0.
  - CPU oracle + unit test `weighted_mean_uses_child_amount_as_weight`.
  - GPU parity `weighted_mean_reduction_matches_cpu_oracle`.

**Usage:** set `SubFieldSpec::reduction_override =
Some(ReductionRule::WeightedMean { by: pop_property_id })` on the column
being aggregated (e.g. loyalty `Amount` weighted by cohort population).

**126/126 tests passing, zero warnings.**

---

## 2026-05-20 — Per-entity ids in boundary outcomes (PR #20)

**Status:** Merged to `master` as PR #20 (`21c326f`).

**Landed:**

- `FissionOutcome`: `fission_pairs`, `fusion_pairs` — `(parent, child)` per
  successful fission/fusion; populated in `execute_fission` / `execute_fusion`.
- `MaintainerOutcome`: `reparented` — `(child, new_parent)` per successful
  reparent in `tree_mutation`.
- `ExpiryOutcome`: `expired` — `(sim_thing_id, property_id)` per threshold
  removal and CPU decay sweep.
- `delta_log.rs`: `BoundaryDeltaEntry` variants now carry full ids (no
  count-only `FissionOccurred` / `FusionOccurred` / `PropertyExpired` /
  `SimThingReparented`). `entries_from_outcome` iterates the new vecs.
  Diagnostic counters on outcome structs unchanged.

**Still deferred for replay:** embed full `Overlay` in `OverlayAttached`;
serialization format + playback driver.

**124/124 tests passing, zero warnings.**

---

## 2026-05-19 — GPU Passes 4–6: presentation reduction

**Status:** Merged (PR #19, `93bbe36`). The full GPU reduction pipeline lands: per-sub-field `ReductionRule`,
bottom-up tree reduction with a bit-exact CPU oracle, GPU shader, boundary
topology sync, and a `ReducedField` accessor on `BoundaryProtocol`.

**Landed in this session:**

- `simthing-core`:
  - `crates/simthing-core/src/reduction.rs` — new module. `ReductionRule`
    enum (`Mean`, `Sum`, `Max`, `Min`, `First`), `default_for_role()`.
    Role defaults: Amount/Velocity/Named/Custom → Mean, Intensity → Max.
  - `SubFieldSpec.reduction_override: Option<ReductionRule>` field +
    `resolved_reduction()` helper.
- `simthing-gpu`:
  - `crates/simthing-gpu/src/reduction.rs` — CPU oracle + helpers:
    `Topology` (CSR child layout + depth buckets), `build_topology`,
    `build_column_rules`, `cpu_reduce_oracle`. Children iterated in
    canonical (ascending slot) order so CPU and GPU sum/mean accumulate
    in identical sequence.
  - `WorldGpuState` gains `child_starts`, `child_indices`, `column_rules`,
    `depth_slots` buffers + `depth_bucket_ranges` CPU-side. Constants:
    `RULE_MEAN`/`SUM`/`MAX`/`MIN`/`FIRST`. `ReduceParams` uniform.
  - `upload_reduction_topology()` uploads all four buffers in one call.
  - `read_output_vectors()` readback helper.
  - `shaders/reduction.wgsl` — single shader, one dispatch per depth
    (deepest first). Leaf branch copies `values → output_vectors`; inner
    branch loops children, accumulates per-rule. Mean uses explicit
    division (not reciprocal multiply) to match CPU bit-for-bit.
  - `Pipelines::run_reduction_passes` walks `depth_bucket_ranges` in
    reverse, writing the uniform + dispatching once per depth.
- `simthing-feeder`:
  - `DispatchCoordinator::tick` calls `run_reduction_passes` between
    Pass 3 and Pass 7. No-op until boundary uploads topology.
- `simthing-sim`:
  - `gpu_sync.rs` step 9 now also builds + uploads topology + column
    rules at every boundary (cheap, tree-shape changes are boundary-only).
    `GpuSyncOutcome.reduction_depths` reports bucket count.
  - `crates/simthing-sim/src/reduced_field.rs` — new module.
    `ReducedField { n_dims, values: Vec<f32> }` with `row(slot)` and
    `property_value(slot, registry, prop_id)` accessors.
  - `BoundaryProtocol::read_reduced_field(state)` returns a fresh
    `ReducedField` from GPU `output_vectors`.

**Tests (124 passing, zero warnings — up from 116):**
- core: 2 new (`role_defaults`, `override_resolves_via_subfield_spec`).
- gpu: 4 new unit (`topology_csr_and_depth_buckets`,
  `cpu_oracle_mean_intensity_max`, `column_rules_respect_override`,
  `sum_rule_sums_children`); 1 new parity (`reduction_matches_cpu_oracle`)
  — GPU output matches CPU oracle bit-exactly on a 3-tier tree.
- sim integration: 1 new (`reduction_pipeline_produces_aggregated_output_vectors`)
  — full BoundaryProtocol + tick path, verifies Mean on Amount and Max on
  Intensity at the Location row.

**Determinism contract:**
Both CPU oracle and GPU shader iterate children in
`Topology::child_indices` order (ascending slot), accumulate left-to-right,
and divide by `f32(n_children)` for Mean. Float sums are not associative,
so reorder = divergence; this contract is the only thing keeping parity.

**Still deferred (Opus):**
- Replay serialization + playback (delta log → on-disk format + driver).
- `WeightedMean { by: SimPropertyId }` reduction variant — population-
  weighted aggregates require extending the shader's per-column rule
  encoding to carry a second column reference.
- Thresholds on reduced (`output_vectors`) values, not just `values` —
  e.g. world-level `instability` thresholds for AI early warning.

---

## 2026-05-19 — Replay delta capture (Opus prep)

**Status:** Merged. `BoundaryProtocol` now accumulates a per-boundary
delta log; callers drain it with `take_delta_log()`.

**Landed in this session:**
- `crates/simthing-sim/src/delta_log.rs` — new module:
  - `BoundaryDeltaEntry` enum covering: `OverlayAttached`, `SimThingAdded`,
    `SimThingRemoved`, `DimensionAdded`, `FissionOccurred`, `FusionOccurred`,
    `PropertyExpired`, `SimThingReparented`, `VelocityAlert`.
  - `entries_from_outcome(outcome: &BoundaryOutcome) -> Vec<BoundaryDeltaEntry>` —
    derives entries from the existing outcome fields. Per-entry ids for
    structural mutations, fission/fusion, expiry, reparents, and velocity alerts.
    *(Count-only fission/expiry/reparent entries superseded by PR #20.)*
  - 6 unit tests covering empty, counts, ids, combined expiry, alert
    structure, and step ordering.
- `BoundaryProtocol`:
  - `delta_log: Vec<BoundaryDeltaEntry>` field.
  - `execute()` calls `entries_from_outcome` and appends at the end.
  - `delta_log() -> &[BoundaryDeltaEntry]` and `take_delta_log()` accessors.

**What remains for full replay (see Next session pickup):**
- `OverlayAttached`: embed full `Overlay` data (not just id) for deterministic playback.
- Serialization format, file I/O, determinism guarantees, playback driver.
- *(Per-entity outcome ids — done in PR #20.)*

**116/116 tests passing, zero warnings.**

**Sonnet work complete.** Next: Opus for Step 5 (Passes 4–6 reduction
semantics) and Step 6 (replay serialization + playback).

---

## 2026-05-19 — Observability query (Week 4 complete)

**Status:** Week 4 Step 4 merged. `BoundaryProtocol::observe` answers
"why is X high on Y?" without touching the GPU.

**Landed in this session:**
- `crates/simthing-sim/src/observability.rs` — new module with:
  - `SubFieldObservation { role, value }` — current shadow value per
    sub-field.
  - `OverlayContribution { overlay_id, source, deltas, inherited }` —
    one overlay's contribution, flagged `inherited` when it lives on an
    ancestor.
  - `PropertyObservation { property_id, property_name, sub_fields,
    overlay_contributions }` — full decomposition per property.
  - `ObservabilityReport { sim_thing_id, properties }`.
  - `observe(root, registry, allocator, shadow, n_dims, target)` — free
    function; depth-first path-finding then one pass over the ancestor
    chain per property.
- `BoundaryProtocol::observe(&self, coord, target)` — delegates to the
  free function using `self.root`, `self.registry`, `self.allocator`, and
  `coord.shadow`.
- Unit tests (6):
  - `observe_returns_none_for_unknown_target`
  - `observe_reports_sub_field_values_from_shadow`
  - `local_overlay_is_not_inherited`
  - `ancestor_overlay_is_marked_inherited`
  - `inherited_and_local_overlays_both_reported_in_path_order`
  - `overlays_on_unrelated_properties_are_excluded`

**Design note:** shadow is the right source between boundaries — doing a
full GPU readback every observe call would be prohibitively expensive.
After `BoundaryProtocol::execute` the shadow reflects the GPU readback
(execute pulls GPU values at the start of each boundary), giving accurate
values when called post-boundary.

**110/110 tests passing, zero warnings. Week 4 complete.**

**Next session:** Week 5 — Passes 4–6 (reduction) for the presentation
layer, or network-play semantic delta log. Both are Opus-tier architecture
work per the original proposal.

---

## 2026-05-19 — AI intent overlay API

**Status:** Week 4 Step 3 merged. AI subsystems can now submit intent
overlays through a dedicated channel that is separate from the player
feeder queue.

**Landed in this session:**
- `AiIntentOverlay { target, overlay, urgency: f32 }` — AI-authored overlay
  with an urgency hint. `urgency` does not change how the overlay is applied;
  it is metadata for downstream systems (observability, UI prioritisation).
- `AiSender` (Clone) + `AiReceiver` + `ai_channel()` — separate mpsc channel
  so AI and player submissions don't contend. `AiSender::submit_ai_intent`.
- `TransformPatcher::set_ai_receiver(rx)` — attaches the AI channel. `drain()`
  drains it automatically after the feeder queue with the same mid-day fast
  path: transform delta applied to CPU shadow immediately, structural
  `attach_overlay` deferred to boundary. No changes to `tick()` signature.
- `take_ai_intents() -> Vec<AiIntentOverlay>` and `ai_intents_parked` stat.
- `BoundaryProtocol::execute`: pulls AI intents alongside player intents,
  converts each to `BoundaryRequest::AttachOverlay`. `BoundaryOutcome::
  ai_intents_attached` counter.
- Tests added:
  - `ai_intent_applies_transform_to_shadow_and_parks_with_urgency`
    (patcher unit, no GPU): Set(0.42) on slot 1, urgency=0.9 preserved.
  - `ai_intent_mid_day_effect_and_boundary_attach` (GPU integration):
    ticks_per_day=2; GPU shows Set(0.8) after tick 1; overlay attached
    after tick 2 boundary.

**104/104 tests passing, zero warnings.**

**Next session:** Week 4 Step 4 — observability query. A read-only
`BoundaryProtocol` method that, for a given `SimThingId`, returns amount /
velocity / intensity snapshot plus which overlays are contributing and by
how much (walking the ancestor chain the same way `build_overlay_deltas`
does but returning an `ObservabilityReport` instead of GPU buffer rows).

---

## 2026-05-19 — PlayerIntent mid-day fast path

**Status:** Week 4 Step 2 merged. Player intent transform delta is now
applied to the CPU shadow immediately on receipt (mid-day), making the
effect visible on the GPU within the same tick. Structural `attach_overlay`
still fires at the day boundary.

**Landed in this session:**
- `TransformPatcher::drain`: on `FeederWork::PlayerIntent`, constructs a
  synthetic `PatchTransform` from `pi.overlay.transform` and calls
  `apply_one` before parking — reuses the full `col_for_role` resolution
  path, dirty-row tracking, and skip-stats of a regular patch.
- Tests added:
  - `player_intent_applies_transform_to_shadow_and_marks_row_dirty`
    (patcher unit, no GPU): verifies Set(0.75) lands in shadow at the
    right slot + col and marks the row dirty.
  - `player_intent_mid_day_effect_lands_on_gpu_before_boundary`
    (GPU integration): ticks_per_day=2; after tick 1 (mid-day), GPU
    values confirm Set(0.6) is present; overlay is not yet in tree; after
    tick 2 (boundary), overlay is structurally attached.

**102/102 tests passing, zero warnings.**

**Next session:** Week 4 Step 3 — AI intent overlay API. `AiIntentOverlay`
type, separate `AiSender` channel so AI and player submissions don't
contend, boundary protocol processes them via the same `AttachOverlay`
path. Decide whether `urgency: f32` lives on the overlay or as a
side-channel field.

---

## 2026-05-19 — PlayerIntent overlay submission API

**Status:** Week 4 Step 1 merged as PR #14. Player-authored overlays can
now be submitted through the feeder channel and attach at the day boundary.

**Landed in this session:**
- `PlayerIntentOverlay { target: SimThingId, overlay: Overlay }` — new type
  in `simthing-feeder::work`.
- `FeederWork::PlayerIntent` — third channel variant alongside `Patch` and
  `Boundary`. Keeps player intent distinct from structural boundary work so
  a future mid-day shadow-effect path can handle it independently.
- `FeederSender::submit_player_intent(target, overlay)` — convenience method
  for gameplay/UI code.
- `TransformPatcher`: `pending_player_intents` vec, drain routing,
  `take_player_intents()`, `player_intents_parked` stat counter.
- `BoundaryProtocol::execute`: pulls player intents via
  `patcher.take_player_intents()`, converts each to
  `BoundaryRequest::AttachOverlay`, merges into the existing request list
  before `apply_structural_mutations`. `BoundaryOutcome::player_intents_attached`
  surfaces the count.
- Tests added:
  - `player_intent_parks_in_pending_and_take_drains_it` (patcher unit, no GPU)
  - `player_intent_overlay_arrives_attached_at_boundary` (GPU integration)

**100/100 tests passing, zero warnings.**

**Next session:** Week 4 Step 2 — player overlay mid-day fast path. Extend
`TransformPatcher` to apply an intent overlay's transform deltas to the CPU
shadow on receipt (same `col_for_role` path Patcher already uses), while
still parking the structural `attach_overlay` for boundary time. Effect
visible within the tick; tree attachment still at day boundary.

---

## 2026-05-19 — velocity alert registration

**Status:** Step 3 landed locally. AI-facing velocity alerts can now be
registered, uploaded to Pass 7, and surfaced through the boundary outcome.

**Landed in this session:**
- `VelocityAlertRegistration` describes the SimThing/property/sub-field
  trajectory an AI layer wants to watch.
- `ThresholdBuilder::build_with_velocity_alerts` appends those registrations
  to the ordinary fission/fusion/expiry threshold buffer and records matching
  `ThresholdSemantic::VelocityAlert` entries in the CPU lookup.
- `BoundaryProtocol` owns alert registrations, includes them during initial
  and boundary GPU sync, and reports fired alerts as
  `BoundaryOutcome::velocity_alerts`.
- Tests added:
  - `velocity_alert_registration_targets_requested_sub_field`
  - `velocity_alert_registration_surfaces_at_boundary`

**Focused verification:** targeted threshold-registry and boundary integration
tests for the new velocity-alert path pass.

**Next session:** Continue Week 4 with player input handling or AI intent
overlays. Session intentionally cut off here with `master` synced to
`origin/master` and only `.claude/worktrees/` untracked/untouched; start next
time with player input handling as intent overlays, plus any small doc cleanup
found during that patch.

---

## 2026-05-19 — AddDimension execution

**Status:** Step 2 landed locally. Boundary-time dimension expansion now
widens the CPU shadow and rebuilds GPU buffers instead of deferring.

**Landed in this session:**
- `DispatchCoordinator::resize_dimensions(new_n_dims)` preserves each row's
  existing columns and appends zeroed new columns.
- `WorldGpuState::rebuild_for_registry(registry)` reallocates layout-dependent
  buffers after `registry.total_columns` grows and rebuilds governed-pair /
  intensity-param buffers from the active registry.
- `apply_structural_mutations` now executes `AddDimension` for a registered
  property id: it restores/adopts the property, records it in
  `dimensions_added`, and no longer increments `deferred`.
- `BoundaryProtocol::execute` detects registry growth after structural
  mutations, widens `coord.shadow`, projects sparse values for newly-added
  properties into the new columns, rebuilds `WorldGpuState`, then continues
  the normal step-9 sync.
- Tests added:
  - `resize_dimensions_preserves_existing_columns`
  - `rebuild_for_registry_expands_layout_buffers`
  - `add_dimension_restores_property`
  - `add_dimension_request_rebuilds_gpu_layout`

**Focused verification:** targeted feeder/GPU/sim tests for the new paths pass.

**Next session:** Continue Week 4 with player input handling or AI intent
overlays. Velocity-alert handling landed later on 2026-05-19.

---

## 2026-05-19 — fission child property seeding

**Status:** Week 4 follow-up landed locally. Fission-spawned children now
inherit live property state from the parent's current GPU row.

**Landed in this session:**
- `crates/simthing-sim/src/fission.rs`:
  - `resolve_fission_fusion` now receives a mutable values shadow.
  - New fission children copy every active sparse parent property from the
    boundary GPU readback row into the child's `properties` map.
  - The activating property's `Amount` sub-field is reset to `0.0` on the
    child, matching the design note that the child represents a newly
    expressing force.
  - The child's GPU shadow row is cleared before seeding, so reused tombstone
    slots do not retain stale values.
- `BoundaryProtocol::execute` now passes `coord.shadow` mutably into fission,
  so step 9's full shadow upload carries seeded child rows to the GPU.
- Tests updated:
  - New unit test `fission_child_inherits_parent_properties_from_shadow`.
  - Boundary integration now asserts the spawned child has loyalty and that
    parent + child threshold registrations are rebuilt.

**Focused verification:** `cargo test -p simthing-sim` and
`cargo test -p simthing-sim --test boundary_integration` pass.

**Next session:** Continue Week 4 with player input handling or AI intent
overlays. `AddDimension` execution landed later on 2026-05-19.

---

## 2026-05-18 — simthing-sim crate complete (Week 3 closeout)

**Status:** Full vertical slice operational on `claude/boundary-execution`.
Day-boundary protocol is real, integration-tested end-to-end against GPU.

**Landed in this session:**
- Cherry-picked the `simthing-sim` scaffold (from the closed PR #8) onto a
  fresh branch and brought it to full execution.
- New module `crates/simthing-sim/src/tree_mutation.rs`:
  - `apply_structural_mutations(requests, root, allocator, registry, shadow, n_dims) -> MaintainerOutcome`.
  - Real bodies for every `BoundaryRequest` variant: `AddChild` (alloc subtree
    slots + zero shadow rows), `Remove` (recursive tombstone of detached subtree),
    `Reparent` (subtree move with cycle detection + slot preservation),
    `AttachOverlay` (depth-first attach), `AddDimension` (deferred).
  - 8 unit tests covering happy paths, unknown-target rejection, cycle
    rejection, and slot-preservation invariants.
- `BoundaryProtocol::execute` reworked:
  - Now takes `&mut DispatchCoordinator` so it can resize shadow + write back.
  - **Reads GPU `values` back into `coord.shadow` at the start** — critical:
    integration output (Pass 1/2) lives only on the GPU; otherwise the
    eventual `upload_full_shadow` would wipe a day's worth of work.
  - Routes all `BoundaryRequest` variants through `apply_structural_mutations`
    instead of the old separate step-7 attach loop + step-8 maintainer stub.
  - Resizes shadow after fission (step 6) AND after structural mutations
    (step 7/8) to cover newly-allocated slots.
  - Asserts `allocator.capacity() <= state.n_slots` before GPU upload —
    catches buffer-overflow misuse loudly.
- `gpu_sync::sync_gpu_buffers` now pads `slot_delta_ranges` to `state.n_slots`
  before upload (Pass 3 expects exactly n_slots ranges; `build_overlay_deltas`
  returns one per allocated slot, which can be less).
- `BoundaryOutcome` carries a real `MaintainerOutcome` with allocated /
  tombstoned ids, replacing the previous diagnostic-only counter field.
- `crates/simthing-sim/tests/boundary_integration.rs` — 2 GPU integration
  tests:
  - `fission_event_spawns_child_and_day_n_plus_1_tick_runs_clean` — cohort
    with Amount=0.5 / Velocity=-0.21 integrates across the 0.3 fission
    threshold; Pass 7 fires; boundary executes; new SimThing spawned + slot
    allocated; next-day tick runs cleanly; amount continues falling.
  - `boundary_requests_apply_structural_mutations` — `AddChild` request via
    channel reaches the maintainer at boundary time and attaches a fleet under
    the cohort.

**92/92 tests passing (14 core + 36 GPU + 17 feeder unit + 4 feeder integration
+ 19 sim unit + 2 sim integration), zero warnings.**

**Key design calls made this session:**
- *GPU-read at boundary start.* Reading `state.read_values()` into the shadow
  costs one full readback per day (~3 MB at endgame scale). Without it, any
  `upload_full_shadow` at boundary end wipes Pass 1/2 integration output.
  This is the right tradeoff — daily readback is cheap, lost integration is
  not recoverable.
- *Pad slot_delta_ranges in gpu_sync.* `build_overlay_deltas` returns
  `Vec<SlotDeltaRange>` of length `allocator.capacity()` (correct: one per
  live slot). But `WorldGpuState::upload_overlay_deltas` requires
  `n_slots`-long. The pad is a zero-length range that Pass 3 naturally skips.
  Alternative (allocator phantom slots up to n_slots) would have polluted the
  semantic slot table.
- *Shadow resize at multiple points in `execute`.* After fission (step 6) AND
  after `apply_structural_mutations` (step 7/8). Both can grow the allocator.
  Single resize at end isn't enough because step 7/8 reads from shadow and
  needs it sized to current capacity.
- *All BoundaryRequest variants through one function.* The original scaffold
  had step 7 (AttachOverlay loop) separate from step 8 (TreeMaintainer stub).
  Unified through `apply_structural_mutations` for one clean call site;
  diagnostic counts come from the real `MaintainerOutcome` now.

**Note on the closed PR:** The previous Sonnet session opened PR #8 with the
scaffold and reported it "merged" — actually closed without merging. This
session recovered the scaffold via `git fetch refs/pull/8/head` + `cherry-pick`
and completed the execution work in one PR.

**Branch state:** `claude/boundary-execution` — merged as PR #9.

**Next session:** Week 4. Either player input handling (overlay submission
from a UI/script interface) or AI intent overlays (velocity-threshold
registrations + AI consumer of `ThresholdSemantic::VelocityAlert`).
Property seeding for newly-spawned fission children landed on 2026-05-19.

---

## 2026-05-16 — simthing-feeder crate scaffolding

**Status:** `simthing-feeder` crate landed on `claude/feeder-scaffolding`.
Three sub-roles from design_v4.md §11 wired together with a full
GPU-integration test proving the end-to-end chain.

**Landed in this session:**
- New workspace member `crates/simthing-feeder/` (added to root `Cargo.toml`).
- `src/work.rs` — `PatchTransform`, `BoundaryRequest`, `FeederWork`,
  `FeederSender` (Clone) + `FeederReceiver` over `std::sync::mpsc`,
  `feeder_channel()`. `FeederError::Disconnected` surfaces dropped-receiver
  failures cleanly. 5 unit tests.
- `src/patcher.rs` — `TransformPatcher`. `drain(receiver, registry,
  allocator, n_dims, &mut shadow) -> PatcherStats` resolves
  `SubFieldRole → col` via `col_for_role` only (I1, I5), mutates the CPU
  shadow, parks boundary requests, tracks dirty rows for coalesced GPU
  uploads. 8 unit tests covering all op kinds, all skip paths, and
  dirty-row bitmap semantics.
- `src/dispatcher.rs` — `DispatchCoordinator`. Owns the CPU shadow.
  `tick(...)` runs drain → dirty-row upload → Pass 0 → 1 → 2 → 3 → 7 →
  event readback → counter advance. Upload-before-snapshot ordering
  prevents phantom threshold crossings on patched cells.
- `src/maintainer.rs` — `TreeMaintainer` scaffold. `execute(Vec<BoundaryRequest>)
  -> MaintainerOutcome` classifies and counts each request; execution body
  lands in `simthing-sim`. The dispatch surface is final.
- `src/lib.rs` — public re-exports + topology diagram.
- `tests/integration.rs` — 4 GPU-required end-to-end tests:
  patch-through-channel-lands-on-GPU, day-boundary-fires-on-ticks-per-day,
  boundary-requests-reach-maintainer, many-patches-coalesce-to-one-upload.
- `docs/agents.md` updated: file layout includes the new crate, current
  state reflects Week 3 progress, "Not yet built" focuses on `simthing-sim`,
  test count bumped to 71.

**71/71 tests passing (14 core + 36 GPU + 17 feeder unit + 4 feeder integration),
zero warnings.**

**Design decisions made this session:**
- *CPU shadow over direct GPU writes.* The Patcher mutates a `Vec<f32>`,
  not GPU memory. Read-modify-write for `Multiply`/`Add` would otherwise
  need a per-patch GPU readback. The shadow also enables coalesced
  uploads (10 patches to the same row → 1 `queue.write_buffer`).
- *Upload before Pass 0.* Pass 0 snapshots `values → previous_values`.
  Uploading patches after the snapshot would make every threshold
  registered on a patched cell fire spuriously. Uploading first absorbs
  the patch into the previous-state reference frame, matching how the
  CPU evaluator already treats continuous overlays.
- *Tree Maintainer is a scaffold, not a stub.* The dispatch surface,
  outcome type, and request-routing are real and tested. Only the
  mutation execution body is deferred to `simthing-sim`. This keeps
  Invariant I7 ("structural mutations only at the day boundary")
  enforceable today: the Maintainer never sees the channel directly, and
  the within-day Patcher physically cannot touch the tree.
- *No OS threads in this crate.* The struct names match the design doc's
  "feeder thread architecture" terminology, but `tick()` is a method, not
  a loop. Thread placement is a top-level policy decision the eventual
  `simthing-sim` driver makes.

**Branch state:** `claude/feeder-scaffolding` — ready to push and PR.

**Next session:** `simthing-sim` crate. Day-boundary protocol orchestration
(design_v4.md §10), Tree Maintainer execution body, fission/fusion. The
`build_overlay_deltas` + `upload_overlay_deltas` + `upload_thresholds`
sequence at boundary time also lives there.

---

## 2026-05-16 — Week 3 begins: Pass 7 (threshold scan)

**Status:** Pass 7 fully built and parity-tested on `claude/week3-threshold-scan`.

**Landed in this session:**
- `crates/simthing-gpu/src/world_state.rs`:
  - New Pod types: `ThresholdRegistration` (24 B) and `ThresholdEvent` (16 B).
  - Direction constants: `DIR_UPWARD`, `DIR_DOWNWARD`, `DIR_EITHER`.
  - Three new buffers on `WorldGpuState`: `threshold_registry`, `event_count`
    (4 B atomic `u32`), `event_candidates`. Placeholder allocations keep them
    bindable when no thresholds are registered.
  - New methods: `upload_thresholds`, `reset_event_count`, `read_event_count`,
    `read_event_candidates(n)`. `total_buffer_bytes()` updated.
- `crates/simthing-gpu/src/shaders/threshold_scan.wgsl` — Pass 7. One thread per
  registration; strict crossing detection in three direction modes; `atomicAdd`
  into `event_count` for sparse output indexing.
- `crates/simthing-gpu/src/passes.rs` — Pass 7 pipeline (6-binding layout).
  `run_threshold_scan(state)` resets the counter internally, then dispatches
  `ceil(n_thresholds / 64)` workgroups. New CPU oracle helper in tests.
- `crates/simthing-gpu/src/lib.rs` — exports new types + direction constants.

**Tests added:**
- `upload_thresholds_grows_buffer_and_tracks_count` — buffer reallocates correctly.
- `reset_event_count_writes_zero` — counter reset works.
- `threshold_scan_matches_cpu_oracle` — bit-exact GPU/CPU parity across all
  three direction modes; covers stationary-on-threshold non-event case.
- `threshold_scan_no_registrations_is_noop` — empty registry doesn't panic.
- `threshold_scan_after_full_pipeline` — end-to-end Pass 0+1+2+3+7 with a
  velocity-driven crossing.

**50/50 tests passing (14 core + 36 GPU), zero warnings.**

**Branch state:** `claude/week3-threshold-scan` — ready to merge.

**Next session:** `simthing-feeder` crate scaffolding. Work queue + Transform
Patcher + Dispatch Coordinator per design_v4.md section 11.

---

## 2026-05-16 — Pass 3 complete

**Status:** Pass 3 (iterative overlay transform application) fully built, tested, and pushed on `claude/pass3-iterative-deltas`.

**Landed in this session:**
- `crates/simthing-gpu/src/overlay_prep.rs` — CPU prep pass. `build_overlay_deltas(root, registry, allocator)` walks the tree depth-first mirroring `Evaluator::evaluate_node` step 5: ancestor overlays first, local overlays after, only emitting deltas for properties the node actually has. 5 unit tests cover the empty case, single local overlay, ancestor-before-local ordering, absent-property skipping, and all three op kinds.
- `crates/simthing-gpu/src/shaders/transform_application.wgsl` — Pass 3 shader. One thread per slot. Walks `slot_delta_ranges[slot]` and applies each `OverlayDelta` in place to `values[]` via `switch (op_kind)`. n_slots/n_dims derived from `arrayLength()` so no uniform buffer is needed.
- `crates/simthing-gpu/src/passes.rs` — Pass 3 pipeline (3-binding layout: `values` rw, `overlay_deltas` r, `slot_delta_ranges` r). `run_apply_overlays()` early-returns when `n_overlay_deltas == 0`. New test `pass3_overlay_matches_evaluator` covers Multiply + Add + Set at ancestor and local levels; bit-exact parity confirmed.
- `crates/simthing-gpu/src/lib.rs` — exports `build_overlay_deltas`.
- 30/30 tests passing, zero warnings.

**Branch state:** `claude/pass3-iterative-deltas` — ready to merge (PR #4 open).

**What's left after merge:**
- Passes 4–6 (reduction) and Pass 7 (threshold scan) — deferred. Threshold registration API doesn't exist yet.
- `EvaluationBatch` struct (wrapper around WorldGpuState + per-tick upload) — Week 3 work.
- Feeder thread + day boundary protocol — Week 3.

---

## 2026-05-15 — Pass 3 scaffolding (rate-limited; not finished)

**Status:** session interrupted by rate limits before Pass 3 shader work could land. Scaffolding (decision + types + buffers + upload API) is in this branch and ready to merge.

**Decision adopted:** transform application is **iterative on GPU**, not affine matrix composition. See `docs/agents.md` → "Transform application — iterative on GPU (decided)" for the full rationale. Short version: bit-exact CPU/GPU parity becomes trivial (both sides walk the same delta list in stack order), GPU memory drops by ~370 MB at endgame scale, and per-tick GPU work is proportional to active overlays rather than `n_dims²`.

**Landed in this branch:**
- `docs/agents.md` — iterative-on-GPU section added; `WorldGpuState` buffer list updated; FMA section gained an "Outcome (Week 2)" note; `EvaluationBatch` sketch updated.
- `crates/simthing-gpu/src/world_state.rs`:
  - Removed dead `local_transforms` / `ancestor_transforms` buffers (no shader ever read them; their memory was the cost of an architectural plan we reversed).
  - Added `OverlayDelta` (`{col, op_kind, value, _pad}`, 16 B, Pod) and `SlotDeltaRange` (`{offset, length}`, 8 B, Pod).
  - Added `OP_MULTIPLY` / `OP_ADD` / `OP_SET` constants matching `TransformOp` cases.
  - Added `overlay_deltas` buffer (grows on demand via upload) and `slot_delta_ranges` buffer (fixed size = `n_slots × 8 B`).
  - Added `upload_overlay_deltas(&mut self, deltas, ranges)` — reallocates `overlay_deltas` if too small, then queues writes.
- 38/38 tests still passing, zero warnings.

**What's left for the next session to finish Pass 3:**
1. **CPU prep pass for delta collection.** New module (e.g. `crates/simthing-gpu/src/overlay_prep.rs`) with a tree walker that builds `(Vec<OverlayDelta>, Vec<SlotDeltaRange>)` from a `SimThing` tree + `DimensionRegistry` + `SlotAllocator`. Must carry the ancestor stack and emit ancestor deltas before local deltas in evaluation order, exactly mirroring `Evaluator::evaluate_node` step 5 (`local_stack.apply_to`). Resolve `SubFieldRole → col` via `col_for_role` only (Invariant I1).
2. **Pass 3 WGSL shader** (`crates/simthing-gpu/src/shaders/transform_application.wgsl`). Sketch in `docs/agents.md`. One thread per slot. `switch (d.op_kind) { 0 → Multiply; 1 → Add; 2 → Set }`. Workgroup size 64. Dispatch `ceil(n_slots / 64)` workgroups.
3. **Wire Pass 3 into `Pipelines`** (`crates/simthing-gpu/src/passes.rs`). Mirror the existing `run_velocity_integration` / `run_intensity_update` pattern: bind group layout with `values` (rw), `overlay_deltas` (read), `slot_delta_ranges` (read), uniform with `n_dims`. Add `run_apply_overlays(&self, state: &WorldGpuState)` — no `dt` parameter; Pass 3 is dt-independent. Early-return if `state.n_overlay_deltas == 0`.
4. **Parity test.** New test in `passes.rs` that builds a multi-node tree with non-trivial overlay stacks (mix of `Multiply` / `Add` / `Set` at different levels, ancestor and local), runs `Evaluator` on the CPU side and Pass 0+1+2+3 on the GPU, and asserts bit-exact match. Should be straightforward because both sides iterate deltas in the same order — no rounding-order divergence to worry about.
5. **Commit + push + PR.** Should be one focused PR titled something like "Pass 3 iterative transform application + parity test".

**Branch state:** `claude/pass3-iterative-deltas` is the active worktree branch.

**Gotchas to remember:**
- `upload_overlay_deltas` requires `&mut self` (it can reallocate). Tests will need `let mut state = WorldGpuState::new(...)` rather than the existing `let state = ...` pattern.
- The placeholder allocation strategy: empty `deltas` slice still uploads with `n_overlay_deltas = 0`, and the shader checks `range.length == 0` per slot rather than reading the buffer's overall length. So the placeholder 1-entry buffer is never actually read.
- `OverlayDelta` is 16 bytes with explicit `_pad` to keep the storage-buffer array stride unambiguous. Don't drop the pad.
- The CPU `Evaluator` is unchanged — that's the whole point of going iterative. Don't refactor `apply_to_data`.

**Open questions for the next session (low-priority, can be deferred):**
- Should `upload_overlay_deltas` reuse a staging buffer rather than recreating `overlay_deltas` each grow? At realistic overlay churn this rarely fires, so probably fine as-is.
- Pass 3's per-thread loop has variable length per slot. If some slots have very long stacks and most have none, GPU warps will idle. At our scale this is not a concern, but worth profiling once we have realistic overlay loads.

