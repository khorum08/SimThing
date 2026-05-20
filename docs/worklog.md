# SimThing — Session Worklog

Running log of what's done and what's next, across sessions.

---

## Next session pickup

Master is at `6ef455b` (PR #22, output-vector thresholds merged). 128/128
tests passing, zero warnings, no uncommitted work.

### Todo (recommended order)

- [x] **Per-entity ids in outcome structs** — PR #20.
- [x] **`WeightedMean { by: SimPropertyId }` reduction variant** — PR #21.
- [x] **Thresholds on `output_vectors`** — PR #22.
- [ ] **Replay serialization + playback** (Opus). Format choice (binary frame
  + delta stream, or line-delimited JSON), file I/O, driver consuming
  `BoundaryDeltaEntry`s. Still needs full `Overlay` payload in
  `OverlayAttached` (id-only today). Unblocked on entity ids from PR #20.

**Next up:** Replay serialization.

**Tabled (not on this list):** `simthing-studio` designer UI.

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
