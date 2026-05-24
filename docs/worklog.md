# SimThing — Session Worklog

Running log of what's done and what's next, across sessions.

**Canonical spec:** `docs/design_v6.5.md` (parking) · `docs/design_v6.md` (sim mechanics) | **Agent map:** `docs/agents.md`

---

## 2026-05-24 — `simthing-workshop` spikes: EML Phase 5 + WeightedMean parity (PRs #71–#77)

**Status:** `master` @ `bb09818` (PR #77 merged).

**Scope note:** All work under `crates/simthing-workshop/` is **non-production**. The crate
exists for **isolated viability tests** (CPU oracle vs workshop-local WGSL). It has zero
workspace dependents; passing a workshop gate does **not** mean production code should change.
Production WeightedMean remains in `simthing-gpu`; EML remains optional future backend research
per `docs/eml_integration_guidance.md`.

**Landed:**

| PR | Commit area | Summary |
|----|-------------|---------|
| **#71** | EML Phase 5 spike | Hand-authored 16-node tree; CPU + WGSL evaluators; 1k/10k/100k tests |
| **#72–#74** | EML harness hardening | Reusable `EmlGpuHarness`, hardcoded baseline, node-buffer cache, cold/warm split, overhead ratio, bit-exact test; `eml_phase5_reports_hardened.txt` |
| **#75** | WeightedMean parity v1 | Gather/combine/scatter kernel; CPU oracle; 6 tests; `weighted_mean_reports.txt` (v1) |
| **#76** | Full workshop reports | `workshop_full_reports.txt` — 3-run EML + WeightedMean capture |
| **#77** | WeightedMean hardening | Strict/loose tolerance classification, max-error diagnostics, range-level coverage, zero-weight generator fix, child-count sweep + production-shape fixture; `weighted_mean_reports.txt` replaced |

**Gate results (workshop only):**

- **EML Phase 5 @ 100k:** correctness/determinism **PASS**; `eml_vs_hardcoded_overhead_ratio` ~1.2–1.5× (soft gate < 3.0×).
- **WeightedMean @ 100k:** **`LOOSE_TOLERANCE`** / **`WEAK_PASS_REQUIRES_ADR`** (max abs error ~3e-5, deterministic); manual production-shape fixture **BIT_EXACT** / **STRONG_PASS**.
- **Do not claim:** production AccumulatorOp readiness, general EML backend, or production reduction migration without ADR.

**Tests:** `cargo test -p simthing-workshop` → **17** passed (8 EML + 9 WeightedMean).
Workspace total **362** passed, **1** ignored (includes workshop crate).

**Docs updated this session:** `docs/todo.md`, `docs/worklog.md` (this entry).

---

## 2026-05-23 — I1: Install clone-then-commit + Studio preview API (PR #67)

**Status:** `master` @ `0922908` (PR #67 merged, code `6b8de81`).

**Landed:** Per `docs/adr/install_clone_then_commit.md` (new, Accepted).

- `crates/simthing-gpu/src/slot.rs`: Added `Clone` to `SlotAllocator` derive.
- `crates/simthing-driver/src/install.rs`:
  - `InstallPreview` struct: `pub registry`, `pub root`, `pub allocator`, `pub state`.
  - `preview_install(game_mode, scenario, &registry, &root, &allocator) -> Result<InstallPreview, InstallError>` — clones inputs, runs `compile_and_install` against scratch; caller state never mutated.
  - `install_atomic(…&mut…) -> Result<SpecSessionState, InstallError>` — `preview_install` + commit on success.
  - `compile_and_install` doc: clarified as "in-place worker; prefer `install_atomic`."
  - 5 unit tests: success, atomicity-on-error, preview-then-commit, install_atomic equivalence, slot stability.
- `crates/simthing-driver/src/session.rs`:
  - `open_from_spec` switches to `install_atomic`.
  - `apply_install_preview(&mut self, preview: InstallPreview)` — swap registry/root/allocator + `install_spec_state`.
- Integration test: `i1_apply_install_preview_matches_open_from_spec_shape`.
- `docs/adr/install_clone_then_commit.md` — new ADR (Accepted). Alternatives: delta-recording, rollback, two-phase commit — all rejected.

**Test counts:** 345 passed, 1 ignored.

---

## 2026-05-23 — B3: Precise `requires_boundary_tick` classification (PR #66)

**Status:** `master` @ `bd71ba8` (PR #66 merged, code `defb42c`).

**Problem:** Old classification blocked every boundary skip for sessions with any scripted instance — Threshold-only quiet games never skipped.

**Landed:**

- `crates/simthing-sim/src/threshold_registry.rs`:
  - `has_capability_unlock_in(&self, events) -> bool` — zero-alloc early-return.
  - `has_scripted_event_trigger_in(&self, events) -> bool` — zero-alloc early-return.
- `crates/simthing-driver/src/spec_session.rs`:
  - `requires_boundary_tick(&self, events: &[ThresholdEvent], threshold_registry: &ThresholdRegistry) -> bool` — 6 precise force-tick conditions (queued selection, cooldown>0, Predicate trigger, OnPrereqMet, CapabilityUnlock event, ScriptedEventTrigger event).
  - 9 unit tests covering all 6 clauses.
- `crates/simthing-driver/src/session.rs`: both `run` and `record_to_path` pass events + registry to `requires_boundary_tick`.
- Integration tests: `b3_threshold_only_scripted_events_skip_quiet_boundaries`; `b3_predicate_scripted_event_blocks_boundary_skip`.

**Test counts:** ~338 passed, 1 ignored (≈ 326 + B3 tests).

---

## 2026-05-23 — O2: Replay v3 — spec session state snapshot + per-frame deltas (PR #65)

**Status:** `master` @ `745b9f0` (PR #65 merged, code `2f2a7b5`).

**Landed:** Per `docs/adr/spec_session_state_replay.md` (Status → Accepted; impl notes appended).

- `crates/simthing-spec/src/runtime/capability_state.rs`: `CapabilityTreeNotification` gains `Serialize, Deserialize`.
- `crates/simthing-sim/src/replay.rs`:
  - `ReplayFrame.spec_entries: Vec<serde_json::Value>` (serde default, skip-if-empty).
  - `ReplayWriter::write_extra<T: Serialize>` — opaque escape hatch, keeps `simthing-sim` spec-free.
  - `next_frame` skips unknown `kind` values (forward compat for `spec_snapshot` line).
- `crates/simthing-driver/src/spec_replay.rs` (new):
  - `SpecSnapshot`, `CapabilityStateSnapshot`, `ScriptedCooldownSnapshot`, `QueuedSelectionSnapshot`.
  - `SpecDelta` (7 variants, all logical keys — no raw `OverlayId`).
  - `collect_spec_snapshot`, `diff_and_emit`, `spec_deltas_to_json`, `json_to_spec_deltas`.
  - `apply_spec_snapshot`, `apply_spec_delta`, `LoadedReplay`, `read_spec_replay_file`, `open_replay_with_spec`.
  - `ReplayOpenError`.
- `crates/simthing-driver/src/session.rs`: `record_to_path` emits `spec_snapshot` line and attaches per-frame `spec_entries`.
- `crates/simthing-driver/src/lib.rs`: all O2 types re-exported.
- Integration tests: `record_and_replay_with_spec_round_trips_capability_state` (logical-key invariant asserted); `replay_reader_skips_spec_snapshot_line_for_sim_only_consumer`.

**Test counts:** ~326 + O2 tests at landing (O2 → B3 → I1 totals 345).

---

## 2026-05-23 — Parking doc sync (post Opus O2/B3/I1)

**Status:** `master` @ `2ff84bf` (PR #69 merged).

**Synced:** `design_v6.5.md`, `simthing_spec_sonnet_opus_handoff.md`, `adr/README.md`, `agents.md`, workshop README — Opus P0 complete, 345 tests, Sonnet D1/D2 next.

---

**Status:** `master` @ `9fd8b85`.

**Added:** `docs/workshop/simthing_spec_sonnet_opus_handoff.md` — outstanding work split (Opus: O2 + ADRs; Sonnet: tests/docs/examples).

---

**Status:** `master` @ `afcbd53` (PR #63 merged).

**Added:**

- `docs/workshop/simthing_modder_object_guide.md` — modder-facing core authoring objects
- `docs/workshop/simthing_base_economic_system_working_doc.md` — base economic system working doc

**Updated:** `docs/workshop/README.md` index.

---

**Status:** `master` @ `393db00` (parking sync committed).

**Context:** Opus landed O1b, EffectTarget, S5, S5 follow-up, and O4 (`2eff1e0`–`8904522`)
without updating parking synthesis docs. Worklog entries were current; `design_v6.5.md`,
`todo.md`, progress log, `adr/README.md`, and workshop index were stale.

**Synced:** HEAD `8904522`, **326** passed / **1** ignored, open work → O2 only, footguns
updated for EffectTarget/`overlay_hosts`, ADRs marked Accepted.

---

## 2026-05-23 — O4: Per-owner scripted events

**Status:** `master` @ `8904522`.

**Landed:** Per `docs/adr/scripted_event_scope_model.md` (now Accepted).

- `simthing-spec::runtime`: `ScriptedEventDefinitionId` (atomic),
  `ScriptedEventInstance`, `ScriptedEventInstanceKey { owner_id, event_id }`.
  Overlay re-stamping is not relevant here (definitions are shared, instances
  carry per-owner state).
- `simthing-spec::spec::event`: `EventSpec.install: InstallTargetSpec`,
  defaults to `SessionRoot` so every existing event RON deserializes as a
  single-instance install (pre-O4 behavior).
- `simthing-spec::boundary::event_handler`: new
  `ScriptedEventDiagnosticKind::OwnerRemoved { owner_id }` variant.
- `simthing-driver::SpecSessionState`:
  - Storage migrated from three flat fields (`scripted_events`,
    `scripted_cooldowns`, `scripted_current_slot`) to
    `scripted_event_definitions: HashMap<Id, _>` +
    `scripted_event_instances: HashMap<Key, _>`.
  - APIs: `register_scripted_event_definition(def) → Id`,
    `attach_scripted_event_instance(id, event_id, owner, slot) → Key`,
    convenience `add_scripted_event_instance(def, owner, slot)`,
    `refresh_scripted_event_slots(allocator)` (called every boundary;
    drops stale owners + emits `OwnerRemoved`).
  - Back-compat shims: `add_scripted_event(def)` and
    `set_scripted_current_slot(slot)` attach one instance against
    `session_root_owner` (defaulted, settable via `set_session_root_owner`).
    PR 11 tests migrate with one extra `set_session_root_owner(world_id)`.
  - Handler loop iterates instances sorted by `(owner_id, event_id)` for
    determinism. Per-instance cooldown bridges to the existing
    `ScriptedEventBoundaryHandler` with a one-entry slice + map; writes
    cooldown back to the instance.
  - `scripted_event_trigger_registrations()` emits one registration per
    instance (per-owner slot).
- `simthing-driver::install::compile_and_install`: events now install per
  `EventSpec.install` (one definition + N instances). `set_session_root_owner`
  initialized to `scenario.root.id` so the default `SessionRoot` events
  install correctly.
- Test:
  `open_from_spec_installs_one_scripted_event_instance_per_faction` —
  two factions, one event with `AllOfKind { kind: "Faction" }`, asserts
  one definition + two instances with distinct owner ids and correct slots.
- PR 11 test `scripted_event_handler_runs_from_spec_session_state` migrated
  with one line: `set_session_root_owner(world_id)` before
  `add_scripted_event`.

**Test counts:** 326 passed, 1 ignored (perf bench).

**Deferred (per ADR Out of scope):**
- `ScopeRef::Owner` symbolic scope.
- Cross-owner events.
- Cross-instance priority ordering (per-instance priority preserved; cross
  unspecified).
- Cooldown serialization for replay (O2).

---

## 2026-05-23 — S5 follow-up: register capability instances + thresholds for fission clones

**Status:** `master` @ `8904522`.

**Problem:** After the conservative Approach C disable, fission still left
fission-spawned capability subtrees with **no `CapabilityTreeInstance`** and
**no threshold registrations**. Unlocks on the cloned tree never fired —
the spawned owner had a tree-shaped SimThing but no spec runtime hooked up.

**Landed:**

- `simthing-sim::fission`:
  - `ClonedCapabilityRoot { spawned_owner_id, source_root_id, cloned_root_id,
    overlay_id_pairs }` — provenance per cloned capability subtree.
  - `FissionOutcome.cloned_capability_roots: Vec<ClonedCapabilityRoot>` —
    populated by `clone_capability_children`.
  - `clone_subtree_with_fresh_ids` now re-stamps **overlay ids** in addition
    to SimThingIds. Returns `(SimThing, Vec<(SimThingId, SimThingId)>,
    Vec<(OverlayId, OverlayId)>)`. Without overlay-id re-stamping, source
    and clone subtrees would share `OverlayId`s and `ActivateOverlay` would
    be ambiguous.
- `simthing-driver::session`: `react_to_fission_clones(&BoundaryOutcome)`
  helper. For each `ClonedCapabilityRoot`:
  - Look up source instance via `source_root_id`.
  - Translate source's `by_overlay` and `overlay_hosts` through
    `overlay_id_pairs`, remapping Owner hosts to the spawned owner and
    CapabilityTree hosts to the cloned root.
  - Synthesize threshold registrations targeting `cloned_root_id`.
  - Register via `spec_state.add_capability_tree_instance` and re-sync to
    the protocol so the GPU picks them up next boundary.
  - Called from both `run` and `record_to_path` loops post-execute.
- Test:
  `fission_cloned_capability_subtree_registers_new_instance_and_thresholds`
  — drives loyalty fission, asserts ≥2 capability instances post-fission
  (original + clone), new instance has populated `by_overlay`, and a
  threshold registration targets the cloned tree.

**Test counts:** 325 passed, 1 ignored (perf bench, unrelated).

**Why a full fix vs. minimum:** Overlay-id re-stamping was a sub-bug the
follow-up surfaced. Source and clone sharing overlay ids would have made
the threshold registration succeed mechanically while still corrupting
activation routing. Doing both at once means the clone behaves identically
to the original for the unlock pipeline.

---

## 2026-05-23 — S5: Approach C disabled for cloned capability subtrees

**Status:** `master` @ `8904522`.

**Landed:**

- `simthing-sim::fission`: `FissionOutcome.cloned_capability_subtrees: bool`
  flag set when any executed fission this boundary cloned a capability
  subtree with at least one new slot. `clone_capability_children` now
  returns the count of new slots so the caller can drive the flag.
- `simthing-sim::boundary`: Approach C eligibility predicate excludes
  fissions that cloned capability subtrees. Full-rebuild path in
  `gpu_sync` runs instead — correct, slightly slower than incremental
  append. The ignored S5 RED test now passes; `#[ignore]` removed.

**Why conservative:** Approach C's append loop only sees
`fission_pairs` edges (`original_parent → new_child`). A cloned
capability subtree adds further parent→child edges INSIDE the new
child (`new_child → cap_tree_clone → ...`); the append path missed
those and `cached_topology_state` drifted from a fresh `build_topology`
walk. Tighter incremental support (track every parent→child edge added
during fission) is future work.

**Deferred (out of scope, separate design):** "Append-only external
thresholds for new clones" per `design_v6.5.md:122`. Spec-layer
capability unlock thresholds for fission-spawned cloned subtrees have
no registration path today — `install::compile_and_install` runs only
at session open. Decision needed: should fission re-invoke install, or
should `FissionOutcome` carry threshold registrations for the new
clone? Tracked as follow-up.

**Test counts:** 324 passed, 1 ignored (perf bench, unrelated).

---

## 2026-05-23 — EffectTarget ADR implementation

**Status:** `master` @ `8904522`.

**Landed (code + docs):**

- `simthing-spec`: `EffectTarget` enum (`Owner` default, `CapabilityTree`,
  `SessionRoot`) on `CapabilityEffectSpec`. `#[serde(default)]` keeps every
  existing RON file parseable. Builder records `template_effect_targets:
  HashMap<OverlayId, EffectTarget>` and `CapabilityDefinition.effect_targets:
  Vec<EffectTarget>` parallel to `overlay_ids`.
- `simthing-driver::install`: `install_tree_for_owner` now resolves each
  cloned overlay's host SimThing per `EffectTarget` (Owner → owner;
  CapabilityTree → clone; SessionRoot → root), places the overlay on that
  host, seeds the target property on the host, and stamps
  `CapabilityTreeInstance.overlay_hosts` so the handler picks the right
  `target` on `ActivateOverlay`/`SuspendOverlay`. Discovery: GPU overlay-prep
  ignores `affects` and walks the SimThing tree, so overlay placement
  (not affects) drives transform routing. ADR §Implementation notes
  documents this.
- `simthing-spec::preview`: `CapabilityPreviewInput` gains `owner_slot`
  and `root_slot`. Source slot picked per-effect from `effect_targets`.
- Test: `open_from_spec_owner_targeted_effect_modifies_owner_slot` — Owner
  effect lands on owner slot; clone slot stays at 0. Asserts both.
- Existing v0 tests pin `effect_target: CapabilityTree` explicitly to
  preserve behavior.
- `docs/adr/capability_effect_target_scope.md` → Accepted. §14 of
  `capability_tree_v1.md` → "Accepted, implementation landed."

**Test counts:** 323 passed, 2 ignored (S5 fission, unrelated).

---

## 2026-05-23 — O1b: emit_activation per-clone overlay ids (PR 2eff1e0)

**Status:** `master` @ `2eff1e0`.

**Landed:**

- `simthing-spec::boundary::capability_handler`: `clone_overlay_ids_for_entry`
  helper resolves per-clone overlay ids from `instance.by_overlay`. Both
  activation and `Limited(1)/SuspendOldest` suspension paths use it.
  Sorted by OverlayId for cross-run determinism.
- `simthing-driver::install`: seeds effect-target properties on the cloned
  tree (needed by GPU overlay-prep filter) — discovered while landing the
  ignored E2E test. v0 path; replaced by per-target seeding in EffectTarget
  ADR commit.
- Test: `open_from_spec_capability_unlock_activates_overlay_for_next_tick`
  moved from `#[ignore]` to passing.

---

## 2026-05-23 — EffectTarget ADR (Opus P3, Proposed)

**Status:** `master` @ `927359f` + ADR.

**Landed (docs only, no code):**

- `docs/adr/capability_effect_target_scope.md` (Proposed) — `CapabilityEffectSpec.effect_target`
  selector with three variants (`Owner` default, `CapabilityTree`, `SessionRoot`); install-time
  resolution of `affects` in `install_tree_for_owner`; preview gains `owner_slot`/`root_slot`;
  O1b orthogonality made explicit; 6 alternatives considered and rejected.
- `docs/capability_tree_v1.md` §14 rewritten from "pending" → ADR pointer with decision table
  and authoring rule.

**Next (Codex):** Implement the ADR — `EffectTarget` enum, `CapabilityTreeBuildOutput`
effect-target provenance, install resolver, preview slot routing, test updates. Independent
of O1b (`emit_activation` overlay-id fix); can land in either order.

---

## 2026-05-23 — V6.5 Codex remediation (PR #62)

**Status:** `master` @ `14db14e`.

**Landed:** O1b/S5 repro commands, manual-install E2E clarification, B2 Approach C note,
EffectTarget P3 rationale, historical todo PR ladder label.

---

## 2026-05-23 — V6.5 doc synthesis + archive sunset

**Status:** `master` @ `030ef3e` (PR #61 merged).

**Landed (docs):**

- `docs/design_v6.5.md` — current-state synthesis (parking, open work, doc map)
- `docs/workshop/archive/SUNSET.md` + `README.md` — tracked sunset manifest
- `docs/adr/README.md` — ADR index
- Cross-links: `agents.md`, `todo.md`, `workshop/README.md`, progress log, historical workshop banners
- `game_mode_session_installation.md` ADR → Accepted (O1 landed)

**Archive:** Local handoff bodies remain gitignored; implement from V6.5 + progress log only.

---

## 2026-05-23 — Cursor safe-followup handoff parked

**Status:** `master` @ `ce904e8`; `origin/master` synced (PR #60).

**Cursor handoff complete (PRs #56–#59):**

| PR | Deliverable |
|----|-------------|
| #56 | O1b `open_from_spec` threshold unlock E2E test (**ignored/RED** — overlay-id remapping) |
| #57 | `docs/examples/` InstallTargetSpec RON fixtures + README |
| #58 | `capability_tree_v1.md` §13 kind strings, §14 v0 effect scope |
| #59 | S5 topology drift regression test (**ignored/RED** — Approach C append) |

**Findings for Codex:**

- O1c dimension sync **ruled out** (`n_dims == registry.total_columns` after install).
- O1b blocker: handler emits template `overlay_ids`, not per-clone `instance.by_overlay`.
- S5 blocker: Approach C append misses cloned capability-subtree edges.

**Next owners:** Codex (O1b fix, S5 fix, then O4/O2); Opus (EffectTarget ADR).

**Cursor follow-up when Codex lands:** un-ignore O1b + S5 tests; parking doc sync.

---

## 2026-05-23 — S5 regression test (Cursor, PR #59)

**Status:** `master` @ `61e62c1` (merge PR #59).

**Landed:**

- `BoundaryProtocol::reduction_topology_matches_tree()` test helper
- `fission_with_cloned_capability_subtree_reduction_topology_matches_full_rebuild`
  — **ignored/RED** (append cache drifts from full tree walk)
- Control: `fission_beyond_initial_headroom_grows_gpu_state` asserts helper passes
  on simple fission append path

**Next:** Codex S5 — disable Approach C append when `clone_capability_children`.

---

## 2026-05-23 — Kind strings + v0 effect-target docs (Cursor, PR #58)

**Status:** `master` @ `e97a9ea` (merge PR #58).

**Landed:**

- `capability_tree_v1.md` §13 — `InstallTargetSpec`, built-in/custom kind strings,
  exact matching, `NoMatchingOwners`
- `capability_tree_v1.md` §14 — v0 capability effect scope (cloned tree only);
  EffectTarget ADR pending (Opus P3)
- §2 overlay note corrected to point at §14 (removed stale “targets faction” claim)
- Progress log footguns + read order updated

**Next:** Cursor optional S5 regression test PR.

---

## 2026-05-23 — InstallTargetSpec examples (Cursor, PR #57)

**Status:** `master` @ `b0912bc` (merge PR #57).

**Landed:**

- `docs/examples/README.md` — `AllOfKind`, `ScenarioListed`, `SessionRoot` semantics
- `docs/examples/game_mode_install_all_factions.ron`
- `docs/examples/game_mode_install_scenario_listed.ron`
- `docs/examples/game_mode_install_session_root.ron`
- `pr1_spec.rs`: `loads_install_target_examples` parse smoke test

**Next:** Cursor PR 3 — kind strings + v0 effect-target warning docs.

---

## 2026-05-23 — O1b E2E test (Cursor, PR #56)

**Status:** `master` @ `7bc038e` (merge PR #56).

**Landed:**

- `open_from_spec_capability_unlock_activates_overlay_for_next_tick` in
  `session_integration.rs` — uses `SimSession::open_from_spec`, spec-introduced
  `core::power` + `tech::propulsion`, threshold unlock path.
- Test **`#[ignore]` / RED:** `core::power` stays 0 after 2 boundaries.

**Failure analysis (not O1c):**

- After install, `registry.total_columns == coord.n_dims()` (both 7) — dimension
  sync is **not** the current blocker.
- Install re-stamps overlay ids on clones (`instance.by_overlay`), but
  `CapabilityTreeBoundaryHandler::emit_activation` emits template
  `CapabilityDefinition.overlay_ids` → `ActivateOverlay` targets wrong ids.

**Next:** Codex — handler resolves overlay ids from `instance.by_overlay` per
entry; un-ignore O1b test. Then Cursor docs/fixtures PRs (Tasks 2–4).

---

## 2026-05-23 — Codex evaluation doc sync + work queue (Composer, PR #55)

**Status:** `master` @ `04867b1` (docs-only).

**Ingested:** Codex post-O1 evaluation (O1b blocking, registry/GPU dimension
sync risk in `open_from_spec`, S5 conservative fix, reordered O4/O2 after O1b).

**Updated:** `todo.md`, `workshop/simthing_spec_progress_log.md` (header +
open-work reorder), `workshop/README.md`, `design_v6.md`, this worklog.

**Code ownership (next):**

| Owner | Work |
|-------|------|
| **Codex** | O1b E2E unlock test via `open_from_spec`; O1c dimension/GPU sync (Option B); S5/O5; O4; O2 |
| **Opus** | EffectTarget scope ADR (Owner vs CapabilityTree) before modder/Studio |

**Next:** Codex **O1b** then **O1c** if red; do not start O4/O2 until green.

---

## 2026-05-23 — Doc parking sync after O1 (Composer, PR #54)

**Status:** `master` @ `7eb015a` (merge PR #54; O1 code @ `6ba4e0d` / PR #53).

**Updated:** `todo.md`, `workshop/simthing_spec_progress_log.md`,
`workshop/README.md`, `design_v6.md`, this worklog (O1 entry SHA, footguns,
O1 → Done).

**Next:** superseded by Codex evaluation sync above.

---

## 2026-05-23 — O1 session installation (Opus, PR #53)

**Status:** `master` @ `6ba4e0d` (merge PR #53, code `1f4ca97`).

**Landed:**

- **`InstallTargetSpec`** in `simthing-spec` (`AllOfKind` / `ScenarioListed` /
  `SessionRoot`); `CapabilityTreeSpec` gains `install` field with serde default
  matching the prior behavior (`AllOfKind { kind: "Faction" }`).
- **`GameModeSpec`** / **`DomainPackSpec`** gain `events: Vec<EventSpec>` field
  (serde-default empty).
- **`Scenario::install_targets`** — `HashMap<String, Vec<SimThingId>>` for
  `ScenarioListed` resolution.
- **`simthing_core::kind_matches`** — string-vs-`SimThingKind` comparison helper.
- **`by_overlay` migration** — removed from `CapabilityTreeDefinition`,
  added to `CapabilityTreeInstance`. `CapabilityTreeBuilder::build` returns
  the template-level map as `CapabilityTreeBuildOutput::template_by_overlay`;
  install module re-stamps per clone. Replay v3 (O2) picks up from this shape.
- **`simthing_driver::install`** — new module with `compile_and_install`,
  `install_tree_for_owner`, `resolve_install_target`, `InstallError`. Clones
  capability tree `SimThing`s per resolved owner with fresh `OverlayId`s,
  attaches under each owner, re-allocates slots.
- **`SimSession::open_from_spec(scenario, &game_mode)`** — RON-driven session
  open. Composes `open` + `compile_and_install` + `install_spec_state`.
- **Release-build fix (S3 follow-up):** `debug_assert_topology_cache_matches_tree`
  was defined `#[cfg(debug_assertions)]` but called unconditionally; gated the
  call site to match. Pre-existing on master; fixed inline to keep the parking
  gate green.
- **6 acceptance tests** in `session_integration.rs`: matching-owner install,
  multi-owner clone with distinct `OverlayId`s, scenario-listed targeting,
  `NoMatchingOwners` error, legacy `install_spec_state` regression, and
  `by_overlay` migration shape assertion.

**Tests:** 320 passed, 1 ignored, zero warnings (debug + release).
Release-profile build/tests clean.

**Next:** Codex O1b/O1c (see worklog 2026-05-23 Codex evaluation entry); then S5,
O4, O2. Opus EffectTarget ADR deferred.

---

## 2026-05-23 — Composer S3/S4 + doc parking sync (PR #52)

**Status:** `master` @ `7914528`.

**Landed:**

- **S4** — `capability_instance_by_tree` reverse map in `spec_session.rs`.
- **S3** — `debug_assert!` topology cache vs `build_topology` on full-rebuild
  path only (`boundary.rs`). Append-path assert excluded: Approach C drift on
  `clone_capability_children` fission documented for S5.
- Doc parking sync: `todo.md`, progress log, workshop README, `design_v6.md`.

**Tests:** 314 passed, 1 ignored, zero warnings.

**Next:** Codex **O1** — RON-driven session init per installation ADR.

---

## 2026-05-23 — Phase 1 ADRs + O3 (PRs #49–51)

**Status:** `master` @ `c3f3556`.

**Landed:**

- **PR #49** — Composer Phase 0: `simthing-spec` crate docs, boundary sequence
  header, remove `ResearchRateSpec` vestige.
- **PR #50** — Phase 1 ADRs: session installation, scripted event scope (Option B),
  spec session replay classification.
- **PR #51** — O3: `queue_player_selection_by_key`, `SpecSessionError`.

---

## 2026-05-22 — Phase 1 doc consolidation + PR 11 parking sync

**Status:** `master` @ `9e63718`. Release smoke check passed after Track A.

**Landed:**

- **`docs/workshop/simthing_spec_progress_log.md`** — unified PR 1–11 progress record;
  replaces PR-scoped handoff digests for implementation status.
- **`docs/workshop/README.md`** — workshop index; marks superseded vs current docs.
- Supersession banners on historical handoff/workshop files (see README).
- Parking sync in `docs/todo.md` and this worklog (311 tests, Track A merged).
- Superseded handoffs moved to `docs/workshop/archive/` (gitignored, local only).

**Release verification (C4):** `cargo build --workspace --release --tests` and
`cargo test --workspace --release` — both clean, zero warnings.

**Next:** Opus O1 — session init from authored specs (see progress log § Open work).

---

## 2026-05-22 — PR 11 Track A session/driver assembly

**Status:** Merged on `master` at `01fb572` (parking docs `9e63718`).

**Design:** Added `docs/adr/pr11_track_a_session_assembly.md`. The driver owns
spec runtime state; `simthing-sim` stays spec-free. A generic boundary hook
runs after canonical GPU value readback and before lifecycle/expiry/fission/
structural mutation, so spec handlers see authoritative shadow values and emit
ordinary `BoundaryRequest`s.

**Code:**

- Added `simthing-driver::SpecSessionState` with capability definitions,
  multi-tree-safe capability instance/state keys, scripted-event definitions,
  cooldowns, diagnostics, notifications, and queued player selections.
- Added `SimSession::install_spec_state` and wired `run` / `record_to_path`
  through `BoundaryProtocol::execute_with_boundary_hook`.
- Added `simthing-sim::BoundaryHookContext` and external feeder-level threshold
  registration storage for capability unlocks and scripted-event triggers.
- Extended GPU sync threshold rebuilds so external capability/scripted-event
  registrations are included without importing `simthing-spec` into sim.

**Tests:** `cargo test --workspace` passes with 311 tests, 1 ignored, zero
warnings. `cargo build --workspace --tests` is clean. New coverage:

- CPU unit coverage for queued player selection through the capability handler.
- CPU unit coverage for scripted-event dispatch through `SpecSessionState`.
- GPU E2E coverage for capability progress threshold -> spec session handler ->
  overlay activation -> next-tick value change.

**Deferred:** Replay serialization of capability/scripted runtime state, RON
session initialization from `GameModeSpec`, player input API plumbing beyond
the queue method, and append-only handling for external threshold
registrations on cloned capability trees.

---

## 2026-05-22 — PR 11 Track B merged (PR #47, `392992f`)

**Status:** Merged to `master` via PR #47 (`feat/pr11-track-b`). `master` and
`origin/master` synced at `392992f`.

**Landed (4 commits):**

- `84e03fc` — B2: `EventKey: From<&str>` / `From<String>`
- `f2ed680` — B1: `Display` for boundary diagnostics
- `e8d2980` — B3: `append_capability_unlocks` / `append_scripted_event_triggers`
- `795bc69` — B4: docs addenda + todo/worklog parking

**Verification:** 306 tests passing (+8), 1 ignored, zero warnings. Release
profile build and tests clean (B5).

**Next:** PR 11 **Track A (Opus)** — session state ownership, boundary protocol
step order, handler wiring, E2E integration test, replay implications documented.

---

## 2026-05-22 — PR 11 Track B: mechanical prep complete

**Status:** Track B tasks B1–B5 from `docs/workshop/pr11_session_assembly_handoff.md`
landed. **306** tests passing (+8), **1** ignored, zero warnings. Release profile
also builds and tests clean.

**Landed:**

- **B5** — `cargo build --workspace --release --tests` and
  `cargo test --workspace --release` both green.
- **B2** — `EventKey: From<&str>` and `From<String>` in `spec/event.rs`.
- **B1** — `Display` for `ScriptedEventDiagnosticKind`, `ScriptedEventDiagnostic`,
  and `CapabilityTreeDiagnostic` with format tests.
- **B3** — public `ThresholdBuilder::append_capability_unlocks` and
  `append_scripted_event_triggers` delegating to existing push helpers; index
  preservation tests.
- **B4** — addenda in `design_v6.md` (scripted events PRs 7–10) and
  `capability_tree_v1.md` (unlock event bridge + spec deps).

**Next:** PR 11 **Track A (Opus)** — session state ownership, boundary protocol
step order, handler wiring, E2E integration test, replay implications documented.

---

## 2026-05-22 — Parking state after PR 10 + PR 11 handoff digest

**Status:** `master` and `origin/master` parked at `a8355e7`
(`docs: PR 11 session/driver assembly handoff digest`). Last code commit
is `3e4f6ea` (PR 10). 298 tests passing, 1 ignored, zero warnings.

**Landed this session:**

- PR 9 — scripted event boundary handler (predicate path).
- Threshold dependency cleanup — `simthing-spec` production deps reduced to
  `simthing-core` + `simthing-feeder` only via the new
  `simthing_feeder::CapabilityUnlockEvent` type.
- PR 10 — scripted-event GPU threshold path. Full pipeline from `EventSpec`
  through GPU `ThresholdRegistration` to handler-emitted `BoundaryRequest`.
  `ScriptedEventBoundaryHandler::handle_tick` now unifies predicate and
  threshold paths under shared cooldown + priority gating.
- PR 11 handoff digest at `docs/workshop/pr11_session_assembly_handoff.md`.

**Next session:** session/driver assembly. The digest splits the work into
Track A (Opus, 8 design questions) and Track B (Composer 2.5, 5 mechanical
tasks with do-not-touch lists). Either track can start independently.

---

## 2026-05-22 — PR 10: scripted-event GPU threshold path

**Status:** Threshold-triggered scripted events now have a working
authoring → GPU → CPU → effect pipeline. Predicate-triggered events
(PR 9) and threshold-triggered events share `ScriptedEventBoundaryHandler`
with unified cooldown/priority gating.

**Architecture (mirrors the PR 4 capability-unlock pattern):**

- `simthing_feeder::ScriptedEventTriggerRegistration` — authored-side request:
  `{ event_id, slot, col, threshold, direction }`. Produced by
  `ScriptedEventDefinition::to_trigger_registration(current_slot)` for
  `CompiledTrigger::Threshold` definitions (returns `None` for predicates).
- `simthing_sim::ThresholdSemantic::ScriptedEventTrigger { event_id }` —
  new CPU semantic arm; parallel-indexed with the GPU registration buffer.
- `simthing_sim::ThresholdBuilder::build_with_scripted_event_triggers` —
  walks the tree, adds velocity alerts, then pushes one
  `ThresholdRegistration` per scripted-event trigger into the values buffer.
  Full-rebuild only; B2 append-only deferred.
- `simthing_sim::ThresholdRegistry::extract_scripted_event_triggers` —
  filters `&[ThresholdEvent]` to `Vec<ScriptedEventTriggerEvent>` for the
  spec handler.
- `simthing_spec::ScriptedEventBoundaryHandler::handle_tick(threshold_events,
  ctx)` — signature gained the threshold-events slice; predicate and
  threshold paths now compete under shared `EventPriority` ordering and
  share the `cooldowns` HashMap. Stale registration ids (no matching
  definition) push the new `UnknownEventId` diagnostic.

**Why this is the right shape:**

- Predicates and thresholds are conceptually two trigger *sources* but
  produce the same effect dispatch. Unifying them in a single
  priority-sorted loop guarantees:
  - Cross-source priority is correct (Critical threshold > Low predicate)
  - Cooldown is shared by `EventKey` (an event can't fire from both paths
    in the same tick)
  - The caller has exactly one entry point per tick

**Touch-up:** `simthing_core::Direction` now derives `Copy + PartialEq + Eq`.
The registration type needs these for serde round-trips and value equality
in tests.

**Verification:** `cargo test --workspace` → 298 passed (+12: 11 new
PR 10 acceptance tests + 1 feeder serde test), 1 ignored, zero warnings.

**Next candidates:** session/driver assembly (who owns capability instances
and scripted-event definitions per faction); B2 append-only integration for
both capability unlocks and scripted-event triggers.

---

## 2026-05-22 — Threshold dependency cleanup (spec → feeder)

**Status:** `simthing-spec` production code is now independent of
`simthing-sim` and `simthing-gpu`. Master is parked one commit past the PR 9
parking commit.

**Problem:** PR 5's `CapabilityTreeBoundaryHandler::handle_threshold_events`
took `&[ThresholdEvent]` (from `simthing-gpu`) and `&ThresholdRegistry` (from
`simthing-sim`), forcing the spec crate to depend upward on both. Recorded as
Known Issue #1 in the post-PR-8 handoff.

**Approach:** introduce a *resolved-event* type that lives below spec:

- `simthing_feeder::CapabilityUnlockEvent { sim_thing_id, property_id,
  sub_field }` — the post-resolution shape the spec handler actually consumed.
- Rename handler entry point to `handle_capability_unlock_events(&[CapabilityUnlockEvent], ctx)`.
- Add `ThresholdRegistry::extract_capability_unlocks(&[ThresholdEvent]) ->
  Vec<CapabilityUnlockEvent>` in `simthing-sim` as the conversion bridge for
  callers that hold raw GPU events.

This moves the `event_kind` → `ThresholdSemantic::CapabilityUnlock` resolution
out of spec and into sim, where the `ThresholdRegistry` already lives.

**Crate boundary now:**

- `simthing-spec` production deps: `simthing-core` + `simthing-feeder` only.
- `simthing-spec` dev-deps: `simthing-gpu` + `simthing-sim` (PR 6 integration
  test exercises the full activate/suspend lifecycle through real structural
  overlay mutation — needs both).

**Verification:** `cargo test --workspace` → 286 passed (+1 for the new
`extract_capability_unlocks_resolves_threshold_events_to_unlock_events` test),
1 ignored, zero warnings. `cargo build --workspace --tests` → zero warnings.

**Next candidates:** session/driver assembly; threshold-triggered scripted
event GPU registration (now unblocked by the cleaner crate boundary); B2
append-only capability unlock integration.

---

## 2026-05-22 — Parking state after simthing-spec PR 9

**Status:** `master` and `origin/master` parked at `dc61929`
(`simthing-spec PR 9: scripted event boundary handler.`).

**Landed this session:**
- PR 9 — scripted event boundary handler (`boundary/event_handler.rs`).

**Verification:** `cargo test --workspace` → 285 passed, 1 ignored, zero
warnings. `cargo build --workspace --tests` → zero warnings.

**Next candidates:** session/driver assembly for capability tree instances and
per-faction state maps; threshold dependency cleanup (move `ThresholdSemantic`
surface into a lower crate); threshold-triggered scripted event GPU registration
(follow-on to PR 9 predicate path); B2 append-only capability unlock integration.

---

## 2026-05-22 — PR 9 Sonnet prep (event handler scaffold)

**Status:** Pre-PR-9 prep complete. Branch still parked at `8a8061c` / `d871518`;
no new code commits yet.

**Verified:** `cargo test --workspace` → 277 passed, 1 ignored, zero warnings.
State matches the `opus_current_state_handoff.md` description exactly.

**Changes made:**

- `crates/simthing-spec/src/lib.rs` — replaced stale "PR 1 non-goals" crate doc
  comment with an accurate summary of what PRs 1–8 delivered and what is
  deferred.
- `crates/simthing-spec/src/boundary/event_handler.rs` — new file; compilable
  implementation of `ScriptedEventBoundaryHandler`, `ScriptedEventBoundaryContext`,
  `ScriptedEventDiagnostic`, and `ScriptedEventDiagnosticKind`.
- `crates/simthing-spec/src/boundary/mod.rs` — wired `pub mod event_handler` and
  re-exported the three new public types.
- `crates/simthing-spec/src/lib.rs` — added `ScriptedEventBoundaryContext`,
  `ScriptedEventBoundaryHandler`, `ScriptedEventDiagnostic` to the `boundary::`
  pub use block.

**Design decisions encoded in the scaffold:**

- **Predicate triggers only** — `CompiledTrigger::Threshold` events are skipped
  silently. Scripted-event threshold triggers need GPU registration (a separate
  later PR) and must not be faked with shadow polling.
- **Cooldowns implemented** — `ctx.cooldowns: &mut HashMap<EventKey, u32>` tracks
  remaining ticks per event; `tick_cooldowns` decrements and prunes at the start
  of each call; cooldown is armed with `CooldownSpec.ticks` after a successful
  fire. Per-owner semantics are achieved by the caller maintaining separate
  context instances.
- **Priority implemented** — definitions are sorted by `EventPriority` descending
  before iteration (`Critical > High > Normal > Low`).
- **Missing target → diagnostic** — `ScopeRef` resolution against
  `slot_to_thing: &HashMap<u32, SimThingId>` pushes a
  `ScriptedEventDiagnosticKind::UnresolvedEffectTarget { slot }` on miss.
- **Eval errors → diagnostic, not abort** — `ScriptPredicate::eval` errors push
  `ScriptedEventDiagnosticKind::TriggerEvalError(ScriptEvalError)` and skip the
  event; subsequent events still run.

**What PR 9 (Opus) still needs to do:**

- Write `tests/pr9_event_handler.rs` covering all 8 acceptance tests from the
  handoff doc.
- Verify edge cases (empty definitions slice, all-on-cooldown, error recovery).
- Update `docs/todo.md` and `docs/worklog.md` with the PR 9 landing entry.
- Commit, push, and merge.

---

## 2026-05-22 — Parking state after simthing-spec PRs 5-8

**Status:** `master` and `origin/master` are parked at `8a8061c`
(`simthing-spec PR 8: scripted event compiler templates.`). Tracked files were
clean before this parking-doc update; untracked `.claude/worktrees/` and
`demo.replay.ldjson` are present and left untouched.

**Landed this session:**
- PR 5 — capability runtime state and boundary handler.
- PR 6 — capability preview reports and full activate-switch verification.
- PR 7 — canonical Script IR and CPU evaluator.
- PR 8 — trigger/effect/event compiler templates.

**Verification:** `cargo test --workspace` passed with 277 tests, 1 ignored,
and zero warnings. `cargo build --workspace --tests` completed with zero
warnings.

**Next candidates:** PR 9 boundary-time event execution, session/driver
assembly for capability instances and state, threshold dependency cleanup, and
B2 append-only capability unlock integration.

---

## 2026-05-22 — PR 8 trigger/effect/event compiler templates

**Status:** Implemented PR 8 as a conservative compiler-template slice.

**Code:**
- Added `TriggerSpec`, `EffectSpec`, and `EventSpec` authoring structs.
- Added `CompiledTrigger`, `CompiledThresholdTrigger`, `CompiledEffect`, and
  `ScriptedEventDefinition` runtime structs.
- Added `compile_trigger`, `compile_effect`, and `compile_event`.
- Threshold triggers resolve property id and column via `DimensionRegistry` /
  `col_for_role`; predicate triggers preserve PR 7 `ScriptPredicate`.
- Effects compile to boundary request templates for `Remove`,
  `ActivateOverlay`, and `SuspendOverlay`.

**Out of scope:** No event runner, no threshold registry upload, no parser,
no EML backend, no boundary event handler, and no AddChild/Reparent template
payloads yet.

**Tests:** `cargo test -p simthing-spec --test pr8_event_compiler` passes
with 7 tests covering threshold compilation, predicate preservation, hard
errors, effect templates, event composition, and serde round-trips.

**Next:** Session/driver assembly or a PR 9 to execute compiled event
definitions at boundary time.

---

## 2026-05-22 — PR 7 canonical Script IR + CPU evaluator

**Status:** Implemented PR 7.

**Code:**
- Replaced `spec/script_stub.rs` with `spec/script.rs`.
- Added `PropertyKey`, `ScopeRef`, `ScriptExpr`, and `ScriptPredicate`.
- Added `ScriptEvalContext` and `ScriptEvalError`.
- Implemented CPU evaluation over `DimensionRegistry` + dense shadow rows:
  constants, property reads, arithmetic, min/max, clamp, predicate gates,
  comparisons, `And` / `Or` / `Not`, and short-circuiting boolean logic.

**Out of scope:** No EML backend, parser, trigger/effect compiler, event
system, derived-field integration, or GPU evaluator.

**Tests:** `cargo test -p simthing-spec --test pr7_script_ir` passes with
10 tests covering reads, explicit slot scope, arithmetic, predicates, gates,
error cases, and serde round-trips.

**Next:** PR 8 — trigger/effect/event compiler.

---

## 2026-05-22 — PR 6 capability preview + mutual exclusivity completion

**Status:** Implemented PR 6.

**Code:**
- Added `preview/capability_preview.rs` and public preview re-exports.
- Added `CapabilityDefinition.effect_transforms`, parallel to
  `overlay_ids` / `effect_keys`, so preview can run from the shared
  definition without reading the template SimThing.
- Implemented `preview_capability_effect`, returning per-overlay breakdowns
  plus combined net deltas for each `(property_id, role)` pair.
- Added a full national-ideas activate-switch test that drives PR 5's handler
  and then applies the emitted `ActivateOverlay` / `SuspendOverlay` requests
  through the real structural mutation path to verify overlay lifecycles.

**Tests:** `cargo test -p simthing-spec` passes, including the 5 PR 6
acceptance tests in `tests/pr6_capability_preview.rs`.

**Next:** PR 7 — canonical Script IR and CPU evaluator.

---

## 2026-05-22 — PR 5 capability runtime state + boundary handler

**Status:** Implemented Path A from the PR 5 handoff.

**Code:**
- Added `ReplacementPolicy` and changed `CapabilityCategorySpec.max_active`
  to `Option<MaxActivePolicy>` with `Limited { count, replacement }`.
- Added `CapabilityCategoryDefinition`, `CapabilityTreeDefinition.categories`,
  and per-entry `activation`, `progress_col`, and `research_cost`.
- Added `runtime/capability_state.rs` for `CapabilityTreeInstance`,
  `CapabilityTreeState`, `CapabilityTreeNotification`, and
  `CapabilityTreeDiagnostic`.
- Added `boundary/capability_handler.rs` with threshold-event handling,
  failed-prereq progress reset, `OnPrereqMet` fixpoint sweeps, player
  selection activation, per-faction active tracking, and `Limited(1)` /
  `SuspendOldest` mutual exclusivity.

**Layering note:** PR 5 consumes `ThresholdRegistry` / `ThresholdSemantic`
from `simthing-sim` and `ThresholdEvent` from `simthing-gpu`, so
`simthing-spec` now has temporary direct dependencies on those crates. This
matches the handoff digest's pragmatic path but diverges from the master
handoff's ideal dependency graph. A future cleanup should lift the threshold
semantic surface into a lower crate before driver/session assembly hardens.

**Tests:** `cargo test -p simthing-spec` passes, including the 10 new PR 5
acceptance tests in `tests/pr5_capability_handler.rs`.

**Next:** PR 6 — preview routine + full activate-switch verification.

---

## 2026-05-22 — Stability check: PR 1 lane ready (`7eb48dc`)

**Status:** Confirmed stable on `master` after PR #46 merge.

**Verification (`cargo test --workspace`):**
- **212 passed**, **1 ignored** (GPU pipeline timing diagnostic), zero warnings.
- All simulation/integration suites green (core, feeder, gpu, sim, driver).
- `simthing-spec`: 8 tests (2 unit + 6 integration) — RON load, round-trip,
  validation only.

**PR 1 boundary confirmed:**
- `crates/simthing-spec` — 16 source files; no `compile/`, `boundary/`,
  `preview/`, or `runtime/` modules.
- Depends on **`simthing-core` only** (not feeder/sim/gpu/driver).
- No `CapabilityUnlockRegistration`, `ThresholdSemantic::CapabilityUnlock`,
  or builder/handler symbols anywhere in `crates/`.

**Next:** PR 2 — property + overlay spec compiler.

---

## 2026-05-22 — Revert `simthing-spec` to PR 1 lane

**Status:** Merged PR #46 (`7eb48dc`).

**Kept:** `crates/simthing-spec` workspace membership; authoring structs
(`GameModeSpec`, `DomainPackSpec`, `CapabilityTreeSpec`, …); generic
`SpecDiagnostics`; RON loaders; lightweight validation.

**Removed/deferred:** `compile/`, `boundary/`, `preview/`, `runtime/` modules;
`CapabilityTreeBuilder`; boundary handler; preview API;
`CapabilityUnlockRegistration` (feeder); `ThresholdSemantic::CapabilityUnlock`
(sim). `ActivationMode::OnPrereqMet` removed from authored spec (runtime-only,
later PR).

**Tests:** 212 passed + 1 ignored.

**Next:** PR 2 property/overlay spec compiler per revised ladder in `todo.md`.

---

## 2026-05-22 — Phase 0 doc pivot + Phase 1 `simthing-spec` PRs 1–5 (superseded)

> **SUPERSEDED — do not implement from this section.** PR #45 was reverted by PR #46.
> The current lane is PR 1 authoring-only (merged), then **PR 2** property/overlay
> spec compiler. See the stability entry above and `docs/todo.md`.

**Status (historical):** Landed as PR #45, then fully reverted by PR #46 (`7eb48dc`).

**Phase 0 — doc ingestion:**
- Architectural pivot synced across canonical docs + workshop files.
- Renamed `simthing-spec worksheet.md` → `simthing_spec_workshop.md`.

**Phase 1 — `simthing-spec` vertical slice:**
- New crate `crates/simthing-spec` (depends on `simthing-core` + `simthing-feeder` only).
- RON spec model: `CapabilityTreeSpec`, categories, entries, effects, `ActivationMode`,
  `ResearchRateSpec`, `MaxActivePolicy`.
- `CapabilityTreeBuilder` → tree SimThing, suspended overlays, definition tables,
  unlock registrations.
- `CapabilityTreeBoundaryHandler` → activate/suspend, prereq reset, `OnPrereqMet` sweep,
  `max_active: 1` mutual exclusivity.
- `preview_capability_effect` API.
- PR 4 plumbing (historical numbering): `CapabilityUnlockRegistration` (feeder),
  `ThresholdSemantic::CapabilityUnlock` + `append_capability_unlocks` (sim).

**Tests (at time of PR #45):** 212 passed + 1 ignored (`cargo test --workspace`).

**Next (historical — invalid after PR #46):** ~~Driver session wiring~~ — do not
implement; follow revised PR ladder in `docs/todo.md` (PR 2 next).

---

## 2026-05-22 — Architectural pivot: `simthing-studio` → `simthing-spec`

**Status:** Doc sync (canonical docs updated; workshop files on disk).

**Pivot (approved in workshop 2026-05-22):**

- **`simthing-spec`** is the RON→runtime compiler crate — capability trees first,
  eventually all authored game data (`PropertySpec`, overlays, triggers, events).
- **`simthing-studio`** is deferred — GUI/editor/importer only; will depend on
  `simthing-spec`, not replace it.
- **`simthing-spec` depends on:** `simthing-core`, `simthing-feeder` only.
- **`simthing-spec` must not depend on:** `simthing-sim`, `simthing-gpu`.
- **`simthing-driver` may depend on** `simthing-spec` for session assembly.
- Minimal sim touch in **PR 4** (revised ladder): `CapabilityUnlockRegistration` in feeder,
  `ThresholdSemantic::CapabilityUnlock` in sim.

**Canonical handoff:** `docs/workshop/simthing_spec_workshop.md` (decision log D0–D21,
implementation path PRs 1–8). Source Q&A:
`docs/workshop/capability_tree_studio_workshop.md`. Older
`docs/workshop/tech_tree_decisions.md` §5 still says `simthing-studio` — superseded
for crate naming; mechanism decisions remain valid.

**Docs updated this session:** `agents.md`, `todo.md`, `worklog.md`,
`capability_tree_v1.md`, `design_v6.md`, `eml_integration_guidance.md`,
`tech_tree_decisions.md` (supersession note), `capability_tree_studio_workshop.md`
(pivot note). New: `workshop/simthing_spec_workshop.md` (renamed from worksheet).

**Next implementation:** PR 1 — `crates/simthing-spec` scaffold (worksheet §14).

---

## 2026-05-22 — PR 5 handoff digest for Codex 5.5

**Status:** No code change. Authored
`docs/workshop/pr5_handoff_digest.md` so the next agent can land PR 5
cold without re-discovering everything PRs 2-4 settled.

The digest covers:

- Files to create / modify (with exact paths).
- The five divergences PR 5 must resolve (`MaxActivePolicy` shape;
  add `categories` map to `CapabilityTreeDefinition`; add
  `progress_col` + `research_cost` to `CapabilityDefinition`;
  instance lookup by tree_thing_id vs owner_id; new
  `CapabilityTreeError` enum).
- All 10 handoff acceptance tests + suggested implementation order.
- Eight gotchas distilled from PRs 2-4, especially the GPU pass-order
  trap (`intent_deltas → snapshot → velocity → intensity → overlay →
  threshold` — intent and shadow paths can't fire single-tick threshold
  crossings; only overlay deltas can) and `OverlayId` non-determinism.
- Test fixture patterns from PR 3 to copy / adapt.
- Cross-crate layering recommendation: add
  `simthing-sim = { path = "../simthing-sim" }` to
  `simthing-spec/Cargo.toml` (safe — `simthing-sim` does not depend
  on `simthing-spec`).

Branch state at handoff: `master` @ `aac6d1f`, 245 tests passing, 1
ignored, zero warnings.

---

## 2026-05-22 — simthing-spec PR 4: capability unlock registration bridge

**Status:** Landed (local). First cross-crate PR of the spec lane.
`CapabilityUnlockRegistration` now lives in its permanent home in
`simthing-feeder`; `simthing-sim`'s `ThresholdBuilder` knows how to turn
them into Pass 7 registrations + matching CPU semantics.

**What landed:**

1. **`simthing-feeder/src/capability.rs`** — new file. Defines
   `CapabilityUnlockRegistration { sim_thing_id, property_id, sub_field,
   threshold }` with `Clone, Debug, PartialEq, Serialize, Deserialize`.
   Re-exported from `simthing-feeder/src/lib.rs`. `Cargo.toml` adds `serde`
   to dependencies (was missing — feeder didn't need it before).

2. **`simthing-sim::threshold_registry`** —
   - `ThresholdSemantic` gains `Serialize, Deserialize` derives and a new
     `CapabilityUnlock { sim_thing_id, property_id, sub_field }` arm.
   - `ThresholdBuilder::build_with_capability_unlocks(root, dim_reg,
     allocator, velocity_alerts, capability_unlocks)` walks the tree
     normally, pushes velocity alerts, then pushes one upward-direction
     Pass 7 registration per `CapabilityUnlockRegistration` on the
     `(slot, col)` resolved via `allocator.slot_of` + `col_for_role`.
   - `push_capability_unlocks` private helper. Skipping behavior mirrors
     velocity alerts (inactive property / unallocated sim_thing / missing
     role → silently skip).
   - Full-rebuild path only. B2 append-only integration with capability
     unlocks deferred to a future PR per the handoff — the first fission
     boundary after a capability tree initializes takes the full rebuild
     path anyway.

3. **`simthing-spec`** —
   - `Cargo.toml` gains `simthing-feeder` dependency.
   - `runtime/capability_definition.rs` removes the placeholder
     `CapabilityUnlockRegistration` and re-exports the canonical one from
     `simthing-feeder`. Public API of `simthing-spec` is unchanged —
     `CapabilityUnlockRegistration` still surfaces at the crate root via
     the existing `pub use runtime::...`.

**Tests:** 6 new, all passing.

- `simthing-feeder/src/capability.rs::tests::capability_unlock_registration_in_feeder_is_public`
  — acceptance #1, contract check.
- `simthing-sim/src/threshold_registry.rs::tests::threshold_builder_with_capability_unlocks_emits_correct_event_kind`
  — acceptance #2.
- `simthing-sim/src/threshold_registry.rs::tests::threshold_builder_capability_unlock_resolves_slot_and_col`
  — acceptance #3, seeds another property first so col is non-zero, and
  allocates the cap tree onto slot 7 (not 0) to prove the resolution.
- `simthing-sim/src/threshold_registry.rs::tests::threshold_semantic_capability_unlock_round_trips_serde`
  — acceptance #4, JSON round-trip via `serde_json`.
- `simthing-sim/src/threshold_registry.rs::tests::threshold_builder_capability_unlock_skips_unallocated_simthing`
  — supplementary, mirrors velocity-alert skipping behavior.
- `simthing-sim/tests/boundary_integration.rs::capability_unlock_fires_in_boundary_integration_test`
  — acceptance #5, end-to-end GPU pipeline. Builds a one-entry capability
  property, attaches a Permanent `Add(THRESHOLD + 1)` overlay to the cap
  tree, calls `build_with_capability_unlocks`, uploads thresholds, runs
  one tick, and verifies the returned `ThresholdEvent` resolves via
  `cpu_reg.get(event_kind)` to `CapabilityUnlock` with the right ids.

**Pass-order gotcha (documented in the test).** The GPU pipeline order is
`intent_deltas → snapshot(values→previous) → velocity → intensity → overlay → threshold`.
So neither `submit_player_intent` (intent_deltas land before snapshot) nor
`TransformOp::Set` via the patcher (shadow row uploaded to values before
snapshot) produces a Pass 7 crossing in a single tick — previous and
current both reflect the same value. Only the overlay path (Permanent
overlay attached to the SimThing → `build_overlay_deltas` → Pass 3 after
snapshot) leaves a visible delta for Pass 7 to detect. The test wires it
up that way and explains the constraint inline.

`cargo test --workspace` → **245 passed**, 1 ignored, zero warnings.
(Baseline 239 + 6 new.)

**Not in this PR:**

- B2 append-only integration with capability unlocks — `gpu_sync.rs`'s
  append path skips them today. The threshold buffer gets rebuilt
  in-full on every boundary, which is acceptable in v0 because the
  capability tree spawns once at session init.
- Runtime instance / state types (`CapabilityTreeInstance`,
  `CapabilityTreeState`) — PR 5.
- `CapabilityTreeBoundaryHandler` (handles fired `CapabilityUnlock`
  events) — PR 5.

---

## 2026-05-22 — simthing-spec PR 3: CapabilityTreeBuilder

**Status:** Landed (local). Authored `CapabilityTreeSpec` now compiles
end-to-end into a template `SimThing`, a shared `CapabilityTreeDefinition`,
and the `CapabilityUnlockRegistration`s that PR 4 will hand to the feeder.

**What landed:**

1. **`ActivationMode::OnPrereqMet`.** Third arm added to the enum.
   Runtime-only — `validate.rs` rejects authoring with the new
   `SpecError::OnPrereqMetAuthoredDefault`.

2. **`runtime/` module.**
   - `CapabilityTreeDefinitionId(u32)` — globally-unique newtype with an
     atomic `new()` allocator (same pattern as `OverlayId` / `SimThingId`).
   - `CapabilityTreeDefinition { id, tree_id, entries, by_threshold,
     by_overlay }` — shared, read-only template. `by_threshold` keys are
     `(SimPropertyId, SubFieldRole)`; `by_overlay` keys are `OverlayId`.
   - `CapabilityDefinition { key, display_name, description, flavor_text,
     overlay_ids, effect_keys, prereqs }` — `overlay_ids` and `effect_keys`
     are parallel-indexed; `effect_keys` are logical (`entry / effect_index`)
     and stable across builds, `overlay_ids` come from the runtime atomic
     so are not.
   - `CapabilityPrereq { property_id, role, col, min_value }` — column
     resolved at build time via `col_for_role`. Boundary handler (PR 5)
     does pure array reads.
   - `CapabilityUnlockRegistration` placeholder. PR 4 replaces with a
     re-export from `simthing-feeder`.

3. **`compile/capability.rs::CapabilityTreeBuilder::build`.** Order of operations:
   - Always-on validation (`validate_capability_tree` — extended below).
   - Per category: register a `SimProperty` with `PropertyLayout { sub_fields }`
     where each sub-field is `SubFieldSpec { role: Named(entry.id),
     reduction_override: Some(ReductionRule::Max), clamp: Floored(0.0),
     default: 0.0, governed_by: None, ... }`. `ReductionRule::Max` is
     forced unconditionally — capability progress sub-fields must not get
     `Mean` even though `SubFieldRole::Named` would default that way.
   - Build the template `SimThing { kind: Custom(tree_kind),
     properties: <progress seeded to 0.0>, overlays: [] }`.
   - For each effect: validate `targets_property` (`"ns::name"`) exists in
     registry, validate every delta's `SubFieldRole` is in the target
     layout, allocate an `OverlayId`, push the `Suspended { when_activated:
     effect.when_activated }` `Overlay` onto the tree.
   - For each prereq: parse `"ns::name"`, look up category property,
     resolve `col` via `col_for_role(Named(entry_id), layout)`, look up
     `min_value` from the prereq entry's `research_cost`.
   - For each `Threshold` entry: emit one `CapabilityUnlockRegistration
     { sim_thing_id: tree.id, property_id, sub_field, threshold }`.
     `PlayerSelection` and `OnPrereqMet` produce none.
   - Assemble and return `CapabilityTreeBuildOutput`.

4. **`validate.rs` extensions.** Hard errors for `OnPrereqMet` authored
   default, `Limited(n != 1)` (`UnsupportedMaxActive`), and single-entry
   self-referential prereqs (`SelfReferentialPrereq`).

5. **New `SpecError` variants:** `OnPrereqMetAuthoredDefault`,
   `UnknownPrereqCategory`, `UnknownPrereqEntry`, `SelfReferentialPrereq`,
   `UnsupportedMaxActive`, `InvalidEffectTarget`.

**Design decisions resolved (from prep survey divergences):**

- (1) Category prereq references use `"namespace::name"` format directly.
  The `CategoryKey { namespace, name }` already in `keys.rs` is the
  canonical lookup. `CapabilityCategorySpec` stays without an `id` field.
- (3) `OnPrereqMet` added to `ActivationMode` enum, rejected by validator.
- (4) Builder reads `CapabilitySpec.research_cost: f32` as both the
  threshold value and prereq `min_value`. The vestigial `research_rate`
  field is unused — kept for serde compatibility, can be removed later.
- (8) `ReductionRule::Max` enforced via `SubFieldSpec::reduction_override`
  baked into the `SimProperty` before `registry.register` (no fictional
  `registry.set_reduction_rule` method needed).

**Affects field:** all compiled capability overlays start `affects: vec![]`.
PR 5's boundary handler will fill it in at activation time (it has the
faction instance id and overlay id; the runtime resolves the target
SimThing).

**Tests:** `crates/simthing-spec/tests/pr3_capability_builder.rs` — 16 passing.
- All 11 acceptance criteria: properties/overlays registered, reduction
  Max enforced, duplicate entry id rejected, threshold positive cost
  enforced, `OnPrereqMet` authored default rejected, `PlayerSelection`
  produces no unlock, same-category prereq resolution, cross-category
  prereq resolution, overlay ids per effect, by_overlay lookup,
  logical effect keys stable across builds.
- 5 supplementary: self-referential prereq, unknown prereq category,
  unknown prereq entry, unsupported max_active, invalid effect target.

`cargo test --workspace` → **239 passed**, 1 ignored, zero warnings.
(Baseline 223 + 16 new.)

**Not in this PR:**

- `CapabilityUnlockRegistration` is a placeholder; PR 4 moves it to
  `simthing-feeder` and replaces the import.
- `ThresholdSemantic::CapabilityUnlock` and `ThresholdBuilder::build_with_capability_unlocks`
  — PR 4 in `simthing-sim`.
- Runtime instance / state types (`CapabilityTreeInstance`,
  `CapabilityTreeState`, `CapabilityTreeNotification`) — PR 5.
- `CapabilityTreeBoundaryHandler` — PR 5.
- Mutual exclusivity policy (`ReplacementPolicy::SuspendOldest`) — PR 5.
  Validator currently rejects any `Limited(n)` where n != 1, so the v0
  constraint is enforced; the handler-side semantics land later.
- Preview routine — PR 6.

---

## 2026-05-22 — simthing-spec PR 2: property + overlay spec compiler

**Status:** Landed (local). New `compile/` module turns authored
`PropertySpec` / `OverlaySpec` into live `SimProperty` registrations and
`Overlay` instances.

**What landed:**

1. **`PropertySpec` expansion.** Added `description: String` and
   `sub_fields: Vec<simthing_core::SubFieldSpec>`. Both `#[serde(default)]`
   so the existing `minimal_game_mode.ron` fixture still parses. Empty
   `sub_fields` falls back to `PropertyLayout::standard(0)` (Amount +
   Velocity + Intensity) — matching `SimProperty::simple` semantics.

2. **`OverlaySpec` expansion.** Added `targets_property: String`
   (canonical `"namespace::name"`), `sub_field_deltas`, `lifecycle`,
   `kind`, `source`. No defaults — PR 1 had `overlays: vec![]` everywhere,
   so no fixture rebreaks.

3. **`compile/property.rs`.** `compile_property(&PropertySpec, &mut DimensionRegistry) -> SpecResult<SimPropertyId>`.
   - Checks `registry.id_of(ns, name)` before `register` — avoids the
     `DimensionRegistry` panic on duplicate.
   - Validates each sub-field's `governed_by` references a role present
     in the same layout. Failed validation does NOT register the
     property (no partial state).

4. **`compile/overlay.rs`.** `compile_overlay(&OverlaySpec, &DimensionRegistry) -> SpecResult<Overlay>`.
   - Parses `"ns::name"` and rejects malformed strings.
   - Looks up the target property; rejects unknown.
   - Validates every `sub_field_deltas[i].0` role exists in the target's
     `PropertyLayout`. This catches authoring bugs at compile time that
     would otherwise silently no-op at runtime (`apply_to_data` skips
     unknown roles).
   - Builds the `Overlay` with `OverlayId::new()` and `affects: vec![]`
     (attachment is the caller's job).

5. **`compile/context.rs`.** `CompileContext { registry: &mut DimensionRegistry }`
   with `registry()` / `registry_mut()` accessors. The threading pattern
   for compiling multiple specs from one domain pack / game mode in
   sequence.

6. **New `SpecError` variants:** `DuplicateProperty`, `UnknownProperty`,
   `InvalidGovernedByRole`, `InvalidSubFieldRole`, `InvalidPropertyReference`.

**Tests:** `crates/simthing-spec/tests/pr2_compile.rs` — 11 tests covering
all 7 acceptance criteria from the handoff doc plus 4 supplementary
(`compile_property_uses_authored_sub_fields_when_provided`,
`compile_overlay_invalid_sub_field_role_is_hard_error`,
`compile_overlay_malformed_property_reference_is_hard_error`,
`compile_context_overlay_after_property_registration`).

`cargo test --workspace` → **223 passed**, 1 ignored timing diagnostic,
zero warnings. (Baseline 212 + 11 new.)

**Not in this PR:**

- Decay, intensity behavior, fission/fusion templates, intensity labels
  on `PropertySpec` — not needed for the acceptance tests; can be added
  later as authoring needs surface.
- Capability tree builder — PR 3.
- Threshold / feeder plumbing — PR 4.

---

## 2026-05-22 — simthing-spec PRs 2–6 prep survey

**Status:** Parked. No code changed. Pre-session codebase survey complete;
divergences between the handoff doc and PR 1 code are documented.

**Survey scope:** All `crates/simthing-spec/src/` files, `simthing-core`
type API (`OverlayId`, `DimensionRegistry`, `SubFieldRole`, `ReductionRule`,
`OverlayLifecycle`), `crates/simthing-feeder/src/lib.rs`,
`crates/simthing-sim/src/threshold_registry.rs`, `docs/invariants.md`.
`cargo test --workspace` → **212 passed**, 1 ignored, zero warnings.

**Key findings for Opus:**

1. **`PropertySpec` and `OverlaySpec` are thin stubs** — no layout info.
   PR 2's `compile_property` / `compile_overlay` must expand them or be
   scoped to minimal registration. Design call required.

2. **`ActivationMode` missing `OnPrereqMet`** — will be added in PR 3.
   `validate.rs` must reject it as an authored default.

3. **`MaxActivePolicy`** in code is `Limited { count: usize }` only — no
   `ReplacementPolicy` field or enum. Handoff §1.4 requires both.
   Added in PR 5 when the handler needs it.

4. **`CapabilityCategorySpec` has no `id` field** — `CategoryKey` in
   `keys.rs` already uses `{ namespace, name }`. Either add `id: String`
   to the struct or accept that category id == `namespace::name`.

5. **`research_cost: f32` vs `ResearchRateSpec`** — struct has both
   `research_cost: f32` (the literal threshold) and a vestigial
   `research_rate: ResearchRateSpec`. PR 3 builder reads the `f32`; the
   `research_rate` field is unused and can be dropped or ignored.

6. **`DimensionRegistry::register` panics on duplicates** — `compile_property`
   must check `id_of` first and return a `SpecError` instead.

7. **No `registry.set_reduction_rule` method** — set
   `SubFieldSpec::reduction_override: Some(ReductionRule::Max)` on each
   sub-field when constructing the `SimProperty` before calling `register`.
   Both `ReductionRule::Max` and the `reduction_override` field exist.

8. **`CapabilityTreeDefinitionId` type does not exist** — define in PR 3.

9. **`SpecError` needs more variants** — `DuplicateProperty`,
   `OnPrereqMetAuthoredDefault`, `UnknownPrereqEntry`, `UnknownProperty`,
   `UnsupportedMaxActive`, etc. Add per PR.

10. **`simthing-feeder` dep absent from `simthing-spec/Cargo.toml`** — added in PR 4.

Full divergence list + confirmed-working inventory in `docs/todo.md`.

---

## 2026-05-22 — B2 fission-growth Approach C: incremental reduction topology

**Status:** Landed (local). The reduction CSR is no longer rebuilt from
scratch on pure-fission growth boundaries — an incremental patch over a
cached `TopologyState` produces a byte-identical result.

**Problem:**

`build_topology` walked the full SimThing tree on every `topology_dirty`
boundary, sorted each parent's child list by slot index (the canonical
order CPU oracle and GPU shader both lock in for bit-exact `f32`
parity), then flattened to CSR. On `fission_stress` that walk is ~40k
nodes plus ~20k sorts every growth boundary.

The CSR layout makes "patch in place" impossible — inserting a child
into the middle of `child_indices` shifts every subsequent entry — so
the optimization keeps the canonical per-slot state cached on the
`BoundaryProtocol`, patches it, and re-flattens.

**Change:**

1. `simthing-gpu/reduction.rs::TopologyState` (new public type):
   - `per_slot_children: Vec<Vec<u32>>` and `depths: Vec<Option<u32>>`
     in canonical (ascending-slot) order.
   - `build(root, allocator)` walks the tree (same logic that
     `build_topology` previously inlined) and sorts each parent's
     child list once.
   - `ensure_capacity(n_slots)` extends both vecs.
   - `add_child(parent_slot, child_slot)` appends to
     `per_slot_children[parent_slot]` and derives the new depth from
     the parent's. `debug_assert!` enforces `child_slot > last_child`,
     locking in the ascending-slot invariant that the
     `SlotAllocator`'s monotonic indexing guarantees.
   - `flatten() -> Topology` produces the CSR + depth buckets — no
     sorts (the canonical state is already sorted by construction).
   - `build_topology` is now `TopologyState::build(...).flatten()`.

2. `simthing-sim/gpu_sync.rs::sync_gpu_buffers` takes
   `&mut TopologyState` and refreshes the cache via
   `*topology_state = TopologyState::build(root, allocator)` on the
   full-rebuild path. Boundary owns the cache; gpu_sync mutates it.

3. `simthing-sim/boundary.rs`:
   - `BoundaryProtocol` gains a `cached_topology_state: TopologyState`
     field initialized to `TopologyState::default()` (empty).
   - After Approach B's threshold append block, a parallel
     `topology_append_eligible` predicate fires under the same pure-
     fission conditions. When eligible, the boundary calls
     `cached_topology_state.add_child(parent_slot, child_slot)` for
     each `(parent_id, child_id)` in `out.fission.fission_pairs`, then
     re-flattens and uploads via `state.upload_reduction_topology(...)`.
     `topology_dirty` is cleared so `gpu_sync` skips the rebuild.
   - The full-rebuild path (called for any non-eligible mutation:
     fusion, expiry, AddChild, Remove, dimension change) goes
     through `gpu_sync` and refreshes the cache, keeping it in
     lockstep with the GPU buffer.
   - `GpuSyncOutcome::{reduction_depths,reduction_edges,reduction_slots}`
     report the counts uploaded — populated by exactly one of the two
     paths (full rebuild via `gpu_out.reduction_*`, or append via the
     local `topology_appended_*` counters).

**Safety: bit-exact determinism through the cache.**

Two new unit tests in `simthing-gpu::reduction::tests` prove the cache
produces byte-identical output:

- `topology_state_flatten_matches_build_topology` — round-tripping a
  fresh state through `flatten` matches `build_topology`'s output
  field-for-field (`child_starts`, `child_indices`, `depth_buckets`).
- `topology_state_incremental_add_child_matches_full_rebuild` —
  applying `add_child` for a fission to a cached state produces the
  same CSR as a full rebuild from the post-fission tree, AND
  `cpu_reduce_oracle` over both topologies produces bit-identical
  `f32` output. This catches any drift in canonical iteration order
  that would break Pass 4–6 reduction parity.

Integration regression in
`fission_beyond_initial_headroom_grows_gpu_state`:

- `reduction_edges == 3` (World→Loc, Loc→Cohort, Cohort→newChild)
- `reduction_depths == 4` (one bucket per depth)

confirming the post-fission topology shape is uploaded correctly via
the append path.

**Benchmark deltas (local, `fission_stress`, 20k fissions / boundary):**

| Metric | Pre-A | After A (PR #40) | After B (PR #41) | After C |
|---|---|---|---|---|
| `boundary_gpu_sync_ms` | ~6.7 | ~7.0 | ~3.8 | ~2.0 |
| `boundary_upload_bytes` | ~2.72 MB | ~2.48 MB | ~1.04 MB | ~1.04 MB |
| `threshold_regs_uploaded` | 59,997 | 59,997 | 39,998 | 39,998 |
| `reduction_edges_uploaded` | 39,998 | 39,998 | 39,998 | 39,998 |
| `boundary_value_rows_uploaded` | 40,000 | 19,999 | 19,999 | 19,999 |
| `ms_per_sim_day` | ~55 | ~55 | ~56 | ~60 |

`boundary_gpu_sync_ms` is down 70% over the session (~6.7 → ~2.0).
The wall-time field still hovers in the ~55–66 ms range — dominated by
`tick_event_readback_ms` (~21–24 ms) — so the session's combined GPU
sync wins are not user-visible on this scenario. But the work avoided
is real and the relative impact grows in larger / sparser
simulations where reductions and threshold registries get longer.

**Tests:** `cargo test --workspace` → **204 passed** (up from 202 with
two new `TopologyState` determinism tests), 1 ignored timing
diagnostic, zero warnings. `bench_stress_scenarios_within_ceiling`
still inside ceiling.

**Open follow-up:**

- Cache-integrity defensive check: a `debug_assert!` reflattening the
  cache and comparing to `build_topology` on every non-append-eligible
  boundary would catch any future drift between cache mutations and
  the tree shape.

---

## 2026-05-22 — Session park

Five-PR session. `master` at `a23820b`.

**Landed today:**

- PR #39 (`e275789`) — V6 guardrails Priorities 1, 2, 3
  (suspended-overlay GPU activation, fission-replay round-trip,
  `clone_capability_children` serde default).
- PR #40 (`14437f3`) — B2 Approach A: buffer-preserving slot growth +
  coalesced dirty-row uploads. Value upload becomes O(fission_count)
  instead of O(n_slots) on growth boundaries.
- PR #41 (`a23820b`) — B2 Approach B: append-only threshold registry on
  pure-fission growth. `gpu_sync` walks only new subtrees + new lineage
  records, appending at the tail of the GPU buffer with stable
  event_kind indices.

**Tests:** `cargo test --workspace` → **202 passed**, 1 ignored timing
diagnostic, zero warnings.

**Bench (local, `fission_stress`, 20k fissions/boundary):**

| Metric | Pre-session | After PR #40 (A) | After PR #41 (A+B) |
|---|---|---|---|
| `boundary_gpu_sync_ms` | ~6.7 | ~7.0 | ~3.8 |
| `boundary_upload_bytes` | ~2.72 MB | ~2.48 MB | ~1.04 MB |
| `threshold_regs_uploaded` | 59,997 | 59,997 | 39,998 |
| `boundary_value_rows_uploaded` | 40,000 | 19,999 | 19,999 |
| `boundary_full_value_uploads` | 1 | 0 | 0 |
| `ms_per_sim_day` | ~55 | ~55 | ~56 |

Wall-time on this synthetic stress scenario stayed flat — the savings
sit below the run-to-run variance of `tick_event_readback_ms` and
`boundary_fission_ms`. The work avoided is real (~1.7 MB less upload
per growth boundary; full registry walk replaced by walk-only-new) and
the relative win grows in longer / sparser simulations.

**Next session pickup (B2 complete; spec-layer track is primary):**

1. **`simthing-spec` PR 2** — property + overlay spec compiler only (PR 1 authoring
   lane stable on `master` @ `7eb48dc`).

**Alternate (parallel, not blocking PR 2):** `tick_event_readback_ms` deep dive (Opus) or
`TopologyState` cache-integrity `debug_assert!` (Sonnet). PRs 3–6 follow sequentially
after PR 2 — see revised ladder in `docs/todo.md`; do not implement from superseded
sections above.

**Open guardrails:**

- No GPU integration test yet for `BoundaryRequest::SuspendOverlay`
  (Priority 1 covered the activate path only). Cheap to add when
  starting a future suspended-overlay session.

---

## 2026-05-22 — B2 fission-growth Approach B: append-only threshold registry

**Status:** Landed (local). Pure-fission growth boundaries skip the full
threshold-registry walk and append only the new registrations.

**Problem:**

`ThresholdBuilder::build_with_lineage` walks the entire SimThing tree and
re-derives every registration from scratch when `threshold_dirty` is set.
On `fission_stress` that's ~60k registrations (~20k existing parents +
~20k new children + ~20k fusion-lineage records) walked every boundary —
~3 ms of pure CPU work even though only the new entries actually need
to land on the GPU.

**Change:**

1. `simthing-sim/threshold_registry.rs` exposes two new public helpers
   on `ThresholdBuilder`:
   - `append_subtree(node, dim_reg, allocator, &mut gpu_regs, &mut cpu_reg)`
     walks a single subtree, pushing registrations into existing vecs
     (event_kinds assigned as `cpu_reg.len()` and onwards).
   - `append_lineage(dim_reg, allocator, lineage, &mut gpu_regs, &mut cpu_reg)`
     does the same for `FissionLineageRecord`s.
2. `simthing-gpu/world_state.rs::append_thresholds(new_regs)` writes new
   registrations at offset `n_thresholds * sizeof(...)`. Grows the
   underlying buffer via `copy_buffer_to_buffer` when capacity is
   insufficient, preserving already-uploaded registrations and their
   event_kind indices. Companion to Approach A's preservation pattern.
3. `simthing-sim/boundary.rs` computes an `append_eligible` flag after
   structural mutations: `threshold_dirty` AND `fissions_executed > 0`
   AND none of `fusions_executed`, `expired`, `tombstoned`, `allocated`
   (AddChild), `dimensions_added`, `reparented`, `lineage_removed`, AND
   `threshold_config_revision == synced_threshold_config_revision`. When
   eligible, the boundary walks only the new fission children's subtrees
   (reusing `structural_paths` for O(1) lookup) and the new
   `lineage_added` records, appending the derived registrations to the
   existing GPU buffer + CPU registry. `threshold_dirty` is then
   cleared so `gpu_sync` skips the full rebuild.
4. The full rebuild path is still taken for all other dirty conditions
   (initial sync, fusion, expiry, structural add/remove, dimension
   change, config change), so safety isn't traded off — only the
   pure-growth case is optimized.

**Regression guard:**

- `fission_beyond_initial_headroom_grows_gpu_state` in
  `crates/simthing-sim/tests/boundary_integration.rs` now asserts
  `outcome.gpu_sync.threshold_regs_uploaded == 2` for a single fission:
  one new FissionTrigger (child's loyalty crossing) + one new
  FusionTrigger (the lineage record). Before Approach B that field
  reflected the full re-walked registry; after, it counts only what
  was actually written via `append_thresholds`.

**Benchmark deltas (local, `fission_stress`):**

| Metric | Pre-B (Approach A only) | Post-B (A+B) |
|---|---|---|
| `boundary_gpu_sync_ms` | ~7 | ~3.8 |
| `threshold_regs_uploaded` | 59,997 | 39,998 |
| `boundary_upload_bytes` | ~2.5 MB | ~1.0 MB |
| `ms_per_sim_day` | ~55 | ~56 |

The ~3 ms saved in `gpu_sync_ms` sits below the run-to-run variance of
`tick_event_readback_ms` and `boundary_fission_ms` on this machine, so
`ms_per_sim_day` is unchanged within noise. The work avoided is real,
though — ~1.5 MB less GPU upload bandwidth per growth boundary, and the
CPU walk over 60k entries replaced by a walk over the ~40k new ones
(plus zero entries for the already-resident ~20k parents). The relative
win grows with longer simulations (the resident threshold count keeps
climbing across boundaries when the world fissions but doesn't fuse).

**Tests:** `cargo test --workspace` → **202** passed, 1 ignored timing
diagnostic, zero warnings. `bench_stress_scenarios_within_ceiling`
still inside ceiling.

**Open B2 work (Approach C):**

Incremental reduction-topology patching. CSR child layout currently
rebuilt from scratch on growth; could be patched incrementally if slot
ordering and determinism are preserved. Highest risk of the three
approaches — Pass 4–6 reduction depends on deterministic child
ordering for bit-exact CPU/GPU parity.

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

**311** tests passing plus **1** ignored timing diagnostic, zero warnings.
`master` @ `9e63718` — `simthing-spec` PRs 1–11 complete including Track A
session assembly. Release profile build and tests clean.

**Canonical progress:** `docs/workshop/simthing_spec_progress_log.md`

**Primary next step:** **Session init from authored specs (O1)** — compile
`GameModeSpec`/domain packs, clone capability trees per faction, wire
`install_spec_state` from scenario open; integration test from RON.

**Recent on `master`:**
- PR 11 Track A — `SpecSessionState`, boundary hook, GPU E2E (`01fb572`)
- PR 11 Track B — PR #47 (`392992f`)
- PRs 2–10 — full spec compiler + handlers + GPU thresholds (`3e4f6ea`)
- PR 11 handoff digest (`a8355e7`) and parking doc sync (`865304d`)

**Design reference:** `docs/design_v6.md` (current, incl. addenda) ·
`docs/design_v5.md` (historical) · `docs/capability_tree_v1.md` (spec-layer RON) ·
`docs/workshop/simthing_spec_workshop.md` (canonical handoff) ·
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

- [x] **B2 Approach C — incremental reduction-topology patching.** Landed
      2026-05-22 (see entry above).

#### Next

- [ ] **Session init from authored specs (O1)** — see progress log § Open work.
- [ ] **Replay v3 for spec runtime state (O2)** — document-first acceptable.
- [ ] **Player selection input path (O3)**.
- [ ] **Document/prototype map-scale representation.**
- [ ] **Scenario format expansion.**

**Recent:** PR 11 complete (`9e63718`). Unified progress log at
`docs/workshop/simthing_spec_progress_log.md`. **311** tests passing.

**Tabled:** `simthing-studio` designer UI (depends on `simthing-spec`); unified
`BoundaryIndex` single-pass boundary walk (review item 4 / C1 — Opus-tier).

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
