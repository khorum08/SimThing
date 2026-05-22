# SimThing — Agent Briefing

This document is for AI agents picking up work on this project. Read it before touching any code.

**Doc set:** `design_v6.md` (current spec) · `design_v5.md` (v5 implementation-synced
historical) · `design_v4.md` (original blueprint) · `capability_tree_v1.md` (studio
capability-tree RON reference) · `workshop/tech_tree_decisions.md` (workshop handoff) ·
`state-authority.md` · `invariants.md` · `worklog.md` (session log + next pickup) ·
`chatgpt_implementation_review.md` (open perf/architecture notes).

---

## What this is

SimThing is a GPU-native grand strategy simulation kernel. The central idea: every entity in the
simulation — world, faction, star system, location, cohort — is the same recursive type (`SimThing`),
and the entire world state lives in GPU dense matrices that are evaluated continuously. The CPU
interprets GPU output as events; it does not drive the simulation.

The current design specification is in `docs/design_v6.md`. Read it before changing tick/boundary
behavior, overlay lifecycle, fission inheritance, GPU pass order, or feeder authority paths.
`docs/design_v5.md` remains valid for architecture and v5-era sections not superseded by v6.
`docs/design_v4.md` is the original blueprint; use it for historical context only.
Capability/tech-tree authoring is **studio-layer only** — the simulation crates never see
"tech tree" semantics. For that pattern read `docs/capability_tree_v1.md` and
`docs/workshop/tech_tree_decisions.md`. The key simulation ideas are:

- **One type:** `SimThing { properties, overlays, children }`
- **One mechanism for change:** overlay a `PropertyTransformDelta` on a SimThing
- **One mechanism for differentiation:** intensity threshold in the registry
- **One place to edit any property:** the `DimensionRegistry`
- **One overlay lifecycle model:** `Permanent | Transient | Suspended` (v6)

If you find yourself adding a special case for "rebel cohorts" or "civil war state" or "ethics
system flags," stop. Those are properties with thresholds, not special cases.

---

## Repository layout

```
SimThing/
├── Cargo.toml                         workspace manifest
├── docs/
│   ├── design_v6.md                   current architecture specification (read this first)
│   ├── design_v5.md                   v5 implementation-synced spec (historical reference)
│   ├── design_v4.md                   original blueprint (historical reference)
│   ├── capability_tree_v1.md          studio capability-tree concept + RON shapes
│   ├── workshop/tech_tree_decisions.md  capability pattern workshop handoff
│   ├── state-authority.md             tick vs boundary numeric truth
│   ├── invariants.md                  non-negotiable code rules (read this too)
│   ├── worklog.md                     session log + next-session pickup
│   ├── todo.md                        parking todo log (V6 guardrails + B2)
│   ├── chatgpt_implementation_review.md  perf review + recommended optimizations
│   └── agents.md                      this file
└── crates/
    ├── simthing-core/
    │   └── src/
    │       ├── lib.rs                 public re-exports
    │       ├── ids.rs                 SimThingId, SimPropertyId, OverlayId
    │       ├── property.rs            PropertyValue, PropertyLayout, SubFieldSpec,
    │       │                          ClampBehavior, SubFieldRole, SimProperty,
    │       │                          IntensityBehavior, DecayBehavior, fission types
    │       ├── registry.rs            DimensionRegistry, PropertyColumnRange
    │       ├── overlay.rs             Overlay, PropertyTransformDelta, TransformOp
    │       ├── simthing.rs            SimThing, SimThingKind
    │       └── evaluate.rs            Evaluator, TransformStack, FieldSnapshot (CPU oracle)
    ├── simthing-gpu/
    │   └── src/
    │       ├── lib.rs                 public re-exports
    │       ├── context.rs             GpuContext — device/queue/adapter init
│       ├── world_state.rs         GovernedPair, IntensityParams, IntentDelta,
│       │                          OverlayDelta, SlotDeltaRange (#[repr(C)] Pod),
│       │                          WorldGpuState, builders, upload helpers, readback
│       ├── reduction.rs           Topology, column rules, CPU reduction oracle
│       ├── slot.rs                SlotAllocator — stable SimThingId ↔ slot_idx
│       ├── projection.rs          project_tree_to_values — sparse → dense values
│       ├── overlay_prep.rs        build_overlay_deltas — tree walk → Pass 3 batch
│       ├── passes.rs              Pipelines (intent + Pass 0–3/4–6/7),
│       │                          run_tick_pipeline (one encoder/submit),
│       │                          individual pass runners for focused tests
│       └── shaders/
│           ├── intent_delta.wgsl          pre-Pass 0: fold tick-time affine deltas
│           ├── snapshot.wgsl              Pass 0: values → previous_values
│           ├── velocity_integration.wgsl  Pass 1: integrate + clamp + pin (I3)
│           ├── intensity_update.wgsl      Pass 2: build/decay intensity
│           ├── transform_application.wgsl Pass 3: iterative overlay apply
│           ├── reduction.wgsl             Passes 4–6: bottom-up parent aggregation
│           └── threshold_scan.wgsl        Pass 7: sparse crossing events
    ├── simthing-feeder/
    │   └── src/
    │       ├── lib.rs                 public re-exports + topology doc
    │       ├── work.rs                PatchTransform, BoundaryRequest, FeederWork,
    │       │                          FeederSender (Clone) + FeederReceiver (mpsc)
│       ├── patcher.rs             TransformPatcher — folds tick work into intent
│       │                          deltas (hot path) or mutates CPU shadow (legacy),
│       │                          tracks dirty rows + boundary parking
│       ├── dispatcher.rs          DispatchCoordinator — uploads intent deltas +
│       │                          dirty rows, run_tick_pipeline, advances tick/day,
│       │                          surfaces threshold events
    │       └── maintainer.rs          TreeMaintainer — diagnostic seam
    │                                  (real execution lives in simthing-sim)
    └── simthing-sim/
        └── src/
            ├── lib.rs                 public re-exports + module map
            ├── threshold_registry.rs  ThresholdSemantic + ThresholdRegistry +
            │                          ThresholdBuilder — derives GPU registrations
            │                          and parallel CPU semantic lookup
            ├── overlay_lifecycle.rs   step 4: dissolve conditions, AfterTicks
            │                          decrement, expire writeback
            ├── property_expiry.rs     step 5: threshold-driven property removal +
            │                          column tombstone; CPU TowardZero sweep
            ├── fission.rs             step 6: fission spawn, fusion scar handler
            │                          (with secondary-condition guard, dedup)
            ├── tree_mutation.rs       steps 7+8: apply_structural_mutations —
            │                          AddChild / Remove / Reparent /
            │                          AttachOverlay / AddDimension
            ├── gpu_sync.rs            step 9: build_overlay_deltas + upload,
            │                          ThresholdBuilder + upload, upload_full_shadow
            ├── boundary.rs            BoundaryProtocol — owns root + registry +
            │                          allocator + cpu ThresholdRegistry; execute()
            │                          sequences boundary steps; can_skip_empty_boundary
            ├── observability.rs       observe / observe_live decomposition
            └── replay.rs              ReplaySnapshot + ReplayFrame + ReplayWriter +
                                       ReplayReader + ReplayDriver — LDJSON
                                       structural-reproduction replay
    └── simthing-driver/
        └── src/
            ├── lib.rs                 SimSession, Scenario, bench/record/replay CLI
            └── session.rs             tick loop, boundary orchestration, metrics
```

---

## Current implementation state

**Weeks 1–4 complete plus replay delta capture (v1 + v2) plus Passes 4–6
(presentation reduction), per-entity boundary outcome ids (PR #20),
output-vector thresholds and aggregate alerts (PR #22), state-authority
hardening (PR #23), fusion lineage + scar semantics (PR #26), full replay
payload (PR #27), and GPU growth/patch-authority hardening (`4b5f1c6`).
Hot-path performance now includes GPU-side intent deltas, consolidated tick
command submission, static-boundary skipping, sparse dirty-row tracking,
fission tree path indexing, boundary phase attribution, and indexed delta-log
emission for fission-heavy growth.
**V6 landed (`f39fe6d`):** `OverlayLifecycle::Suspended { when_activated }`;
`BoundaryRequest::{ActivateOverlay, SuspendOverlay}`; delta-log/replay entries
`OverlayActivated` / `OverlaySuspended`; `OverlayContribution.active`; CPU evaluator
and GPU overlay prep skip inactive/suspended overlays; empty-boundary skip treats
suspended overlays as inert; `FissionTemplate::clone_capability_children` (serde
default `false`) with deep-clone of capability containers
(`Custom("tech_tree")`, `Custom("national_ideas")`, `Custom("talent_tree")`) on
opted-in faction fission — fresh ids, shadow-row copy, overlay `affects` remap,
pre-grow slot headroom. Studio capability-tree semantics live in
`capability_tree_v1.md`; simulation sees only floats, thresholds, and overlay lifecycle.
Normal tick-time feeder/player/AI transforms fold into GPU `IntentDelta` records
and apply before Pass 0 (`apply_collected_as_intents`). The legacy shadow path
(`drain` / `apply_collected` / `apply_one`) remains for direct and replay-style
callers; `apply_one` skips unsafe Add/Multiply unless called with
`ShadowFreshness::GpuSynced`. Boundary
expiry uses synchronized shadow for TowardZero checks, registry tombstoning
waits for whole-tree liveness, AddChild projects semantic property values into
shadow, Remove zeroes tombstoned rows, fission secondary checks use the
triggering property, and boundary slot growth rebuilds GPU state with amortized
doubling instead of panicking when fission/AddChild exceed initial headroom.
Both player and AI can submit overlays; `BoundaryProtocol::observe`
(cheap shadow) and `observe_live` (one GPU row readback for UI/debug) decompose
sub-field values and overlay contributions;
`BoundaryProtocol::take_delta_log()` drains a `Vec<BoundaryDeltaEntry>` with
one entry per fission/fusion/expiry/reparent/structural change — all variants
carry full payloads: `SimThingAdded { parent, node }`, `FissionOccurred {
parent, node }`, `FissionLineageAdded / Removed { record }`. Delta emission
builds a one-pass tree index before looking up replay payloads, so fission
stress does not rescan the tree once per spawned child. GPU Passes 4–6
reduce children into parents bottom-up using per-sub-field `ReductionRule`
(default per role: Amount/Velocity/Named → Mean, Intensity → Max, plus
`WeightedMean`). Pass 7 scans `values` or `output_vectors` via
`ThresholdRegistration.buffer`; `AggregateAlertRegistration` surfaces
post-reduction crossings at the boundary. CPU oracle matches GPU shader
bit-exactly. Fusion lineage: `FissionLineageRecord` persists on
`BoundaryProtocol`, `ThresholdBuilder` registers `FusionTrigger` per record,
`execute_fusion` applies multiplicative scar to parent Amount. `ReplayDriver`
reconstructs tree, registry, allocator, and fission lineage from LDJSON log.**

### simthing-core (complete)
- `PropertyLayout` fully declarative: `Vec<SubFieldSpec>` with computed stride
- `SubFieldSpec`: role, width, ClampBehavior, velocity_max, default, governed_by
- All index arithmetic in `PropertyLayout::offset_of` and `PropertyColumnRange::col_for_role`
- No global index constants — removed
- `PropertyValue::integrate` — governed_by driven, velocity pinning at boundaries (I3)
- `TransformStack::apply_to` and `PropertyTransformDelta::apply_to_data` take `&layout`
- 14 tests passing, zero warnings

`evaluate.rs::Evaluator` is the CPU reference oracle. GPU output must match it to the float bit.

### simthing-gpu (complete for Week 2)

**`context.rs` — `GpuContext`:**
- Device/queue/adapter init with `new_blocking()` and `async new()` entry points
- Primary backends (DX12 on Windows), default limits, no special features

**`world_state.rs` — `WorldGpuState` + Pod structs:**
- `GovernedPair` (24 B) — `(governed_col, governing_col, clamp_min, clamp_max, vel_max, clamp_kind)`.
  Encodes `ClampBehavior` as u32 tag with sentinel `±INFINITY`.
- `IntensityParams` (24 B) — `(velocity_col, intensity_col, velocity_threshold, build_coef, decay_coef, _pad)`.
- `OverlayDelta` (16 B) — `(col, op_kind, value, _pad)`. `op_kind`: 0=Multiply, 1=Add, 2=Set.
- `SlotDeltaRange` (8 B) — `(offset, length)` into the flat `overlay_deltas` buffer.
- `ThresholdRegistration` (24 B) — `(slot, col, threshold, direction, event_kind, buffer)`.
  `buffer`: `THRESH_BUF_VALUES` (0) scans `values`/`previous_values`;
  `THRESH_BUF_OUTPUT` (1) scans `output_vectors`/`previous_output_vectors`.
  `direction`: 0=Upward, 1=Downward, 2=Either. `event_kind` is an opaque u32 the CPU
  side maps back to fission stage / decay expiry / velocity warning / etc.
- `ThresholdEvent` (16 B) — `(slot, col, value, event_kind)`. Sparse output of Pass 7.
- Builders: `build_governed_pairs`, `build_intensity_params` walk the registry,
  skip tombstoned properties, resolve columns via `col_for_role` only (I1).
- `WorldGpuState` owns `GpuContext` + 10 persistent buffers:
  - `values`, `previous_values`, `output_vectors`: `n_slots × n_dims × 4B` each
  - `governed_pairs`: `max(1, n_pairs) × 24B`
  - `intensity_params`: `max(1, n_params) × 24B`
  - `overlay_deltas`: `max(1, n_deltas) × 16B` (grows on demand via `upload_overlay_deltas`)
  - `slot_delta_ranges`: `n_slots × 8B`
  - `threshold_registry`: `max(1, n_thresholds) × 24B` (grows on demand via `upload_thresholds`)
  - `event_count`: 4B (atomic `u32`, reset per tick)
  - `event_candidates`: `max(1, n_thresholds) × 16B` (sparse Pass 7 output)
  - All buffers: `STORAGE | COPY_SRC | COPY_DST`. Placeholder allocations keep
    bindings valid even with zero pairs / zero overlays / zero thresholds.
- `upload_overlay_deltas(&mut self, deltas, ranges)` — reallocates `overlay_deltas`
  if larger than current capacity, then writes both buffers via `queue.write_buffer`.
- `upload_thresholds(&mut self, regs)` — analogous to `upload_overlay_deltas`;
  grows `threshold_registry` and `event_candidates` together so capacity always
  covers the worst-case "every registration fires" case.
- `reset_event_count()` / `read_event_count()` / `read_event_candidates(n)` —
  Pass 7 result readback. `run_threshold_scan` resets the counter internally,
  so callers only call `read_event_count` + `read_event_candidates` at the
  day boundary.
- `total_buffer_bytes()` — sum of every persistent buffer's size, used by the
  VRAM budget test.
- Read helpers (`read_values`, `read_previous_values`, `read_governed_pairs`,
  `read_intensity_params`) use staging buffer + `map_async` + `device.poll(Wait)`.

**`slot.rs` — `SlotAllocator`:**
- Stable `SimThingId ↔ slot_idx` mapping with LIFO tombstone reuse.
- `populate_from_tree(root)` for batch allocation during the CPU prep pass.

**`projection.rs` — `project_tree_to_values`:**
- Walks the SimThing tree and copies each node's sparse `HashMap<SimPropertyId, PropertyValue>`
  into the dense row-major `[slot * n_dims + col]` flat buffer.

**`overlay_prep.rs` — `build_overlay_deltas`:**
- Walks the tree depth-first carrying an ancestor overlay stack.
- For each node's slot: emits ancestor deltas first, then local deltas, in the same
  order `Evaluator::evaluate_node` step 5 applies them. Resolves `SubFieldRole → col`
  via `col_for_role` only (I1). Skips overlays targeting properties the node
  doesn't have (mirrors `resolved` iteration in the CPU oracle).
- Skips `OverlayLifecycle::Suspended` overlays entirely — they never enter Pass 3
  until activated at boundary time.

**`passes.rs` — `Pipelines`:**
- Owns shared uniform buffer (`PassParams { delta_time, n_dims, _pad, _pad }`) and
  compute pipelines for intent delta, snapshot, velocity, intensity, overlay,
  reduction, and threshold.
- `run_tick_pipeline(state, dt)` — records intent delta (when present) → Pass 0 →
  1 → 2 → 3 → reduction depths → Pass 7 in one command encoder, one submit.
  Uses 2D workgroup grids when slot×dim products exceed WebGPU per-axis limits.
- Individual pass runners (`run_snapshot`, `run_velocity_integration`, etc.) remain
  for focused unit/parity tests.

**Shaders (`shaders/*.wgsl`):**
- `intent_delta.wgsl` — pre-Pass 0 affine fold: `values = values * mul + add` per
  `(slot, col)` intent record.
- `snapshot.wgsl` — Pass 0 memcpy `values → previous_values` (+ output_vectors pair).
- `velocity_integration.wgsl` — Pass 1, FMA-prevention via intermediate `let`,
  ClampBehavior dispatch, I3 velocity pinning at floor/ceiling.
- `intensity_update.wgsl` — Pass 2, build / decay branches with explicit
  `let scaled = coef * x; let delta = scaled * dt;` to prevent FMA fusion.
- `transform_application.wgsl` — Pass 3, switch on `op_kind` for Multiply / Add / Set.
  No uniform needed: `n_slots = arrayLength(&slot_delta_ranges)`,
  `n_dims = arrayLength(&values) / n_slots`.
- `reduction.wgsl` — Passes 4–6, one dispatch per depth bucket; per-column
  `ReductionRule` (Mean, Sum, Max, Min, First, WeightedMean).
- `threshold_scan.wgsl` — Pass 7, strict crossing detection in three direction
  modes. Atomic counter (`atomic<u32>`) bound directly as `event_count`; output
  index allocated via `atomicAdd`. Event order is therefore nondeterministic —
  callers sort by (slot, col, event_kind) for parity tests.

**Tests: 48 GPU tests + 1 ignored timing diagnostic, all passing, zero warnings.**

Highlights:
- `pass3_overlay_matches_evaluator` — bit-exact parity for Pass 0+1+2+3 against
  `Evaluator` on a tree with ancestor + local overlays covering all three op kinds.
- `tree_driven_pipeline_matches_evaluator` — 7-node tree, multiple properties,
  parity across every (slot, property, column).
- `velocity_integration_matches_cpu_oracle_fractional_dt` — FMA stress at `dt=0.5`.
- `vram_budget_at_100_slots_8_dims` — verifies buffer sizing matches the
  iterative-on-GPU layout within 5% of projection.
- `pipeline_timing_1000_slots_64_dims` (ignored, run with `--ignored`) — wall-clock
  diagnostic: Pass 0+1+2+3 at 1000 slots × 64 dims with 1000 overlay deltas
  completes in ~1.2 ms (50 ms budget; 40× headroom).
- `threshold_scan_matches_cpu_oracle` — Pass 7 parity test covering all three
  direction modes plus the stationary-on-threshold non-event case.
- `threshold_scan_after_full_pipeline` — end-to-end Pass 0+1+2+3+7 with a
  threshold crossed by velocity integration.

**Not yet built in simthing-gpu:**
- High-level threshold registration helpers (per-property derivation from
  `FissionThreshold` / `DecayBehavior`) — lives in `simthing-sim` boundary code.

### simthing-feeder (Week 3, scaffolding complete)

**`work.rs` — work queue:**
- `PatchTransform { target: SimThingId, delta: PropertyTransformDelta }` —
  the within-day continuous mutation work item. Carries `SubFieldRole`s,
  not column indices (I5).
- `BoundaryRequest` — enum covering `AddChild` / `Remove` / `Reparent` /
  `AttachOverlay` / `AddDimension` / `ActivateOverlay` / `SuspendOverlay`.
  Boundary-only per I7; the channel routes these through to the Tree Maintainer
  at boundary time. `ActivateOverlay` / `SuspendOverlay` transition overlay
  lifecycle (v6); see `design_v6.md` §6 and §11.
- `FeederWork = Patch | Boundary`, transported over `std::sync::mpsc`.
- `FeederSender` is `Clone` (multiple producers); `FeederReceiver` is
  single-consumer and drained non-blockingly by `drain_now()`.

**`work.rs` — work queue (updated):**
- `PlayerIntentOverlay { target: SimThingId, overlay: Overlay }` — player-authored
  overlay to be attached at the next day boundary.
- `FeederWork::PlayerIntent(PlayerIntentOverlay)` — third channel variant alongside
  `Patch` and `Boundary`.
- `FeederSender::submit_player_intent(target, overlay)` — convenience send method.

**`patcher.rs` — TransformPatcher:**
- **Hot tick path:** `apply_collected_as_intents(feeder_items, ai_items, registry,
  allocator) -> (PatcherStats, Vec<IntentDelta>)` folds Set/Add/Multiply into per-cell
  affine records without CPU readback. Boundary requests and player/AI intents park
  as before.
- **Legacy / direct path:** `drain(...)` and `apply_collected(...)` mutate the CPU
  shadow. When `sync_row_from_gpu` is supplied, Add/Multiply refresh affected rows
  from GPU before apply; otherwise `apply_one` skips unsafe RMW and increments
  `unsafe_rmw_skipped`.
- `take_player_intents() -> Vec<PlayerIntentOverlay>` — drains player intents
  for the boundary protocol to attach during step 7/8.
- `PatcherStats` counts applied writes, unsafe RMW skips, missing targets,
  inactive properties, unresolved roles, and parked boundary requests —
  diagnostic signal without crashing the sim when gameplay code drifts from
  registry.
- `take_dirty_rows() -> Vec<u32>` is the bandwidth optimization: 10
  patches to slot 0 produce 1 GPU upload.

**Current hot-path note (2026-05-20):** normal tick-time feeder/player/AI
transforms now fold into GPU-side affine `IntentDelta` records and apply before
Pass 0. The older shadow mutation path remains for direct/replay-style callers,
but dispatcher ticks no longer do per-slot RMW row readbacks for Add/Multiply.
Dispatcher ticks also use `Pipelines::run_tick_pipeline`, which records the full
tick pipeline into one command encoder / queue submit while keeping individual
pass methods available for focused tests.

**`dispatcher.rs` — DispatchCoordinator:**
- Owns the row-major `[n_slots × n_dims]` CPU shadow of `values`.
- `tick(receiver, patcher, registry, allocator, pipelines, state, dt) -> TickOutcome`
  runs: drain → upload GPU intent deltas / safe dirty rows → consolidated GPU
  tick pipeline → readback events → advance counters. `boundary_reached = true`
  on the tick that rolls `tick_in_day` past `ticks_per_day`.
- Intent-application-before-snapshot ordering is intentional: Pass 0 captures
  `previous_values` *after* same-tick intent deltas are in place, so no phantom
  threshold crossings get attributed to command submission work.
- `upload_full_shadow(state)` is the one-shot path for seeding GPU values
  with the projection of an initial SimThing tree.

**`maintainer.rs` — TreeMaintainer (scaffold):**
- `execute(Vec<BoundaryRequest>) -> MaintainerOutcome` — classifies each
  request, increments the relevant counter, marks it `deferred`. Mutation
  execution (tree edits, slot alloc/dealloc, `AddDimension` registry
  expansion, GPU buffer resizing) lands in `simthing-sim`.
- The dispatch surface is final; only the body of `execute` will change.

**Tests: 24 unit + 5 GPU integration tests, all passing, zero warnings.**

Integration highlights:
- `patch_through_channel_lands_on_gpu_after_one_tick` — full chain:
  Sender → Patcher → shadow → dirty-row upload → Pass 0/1/2/3/7 →
  `read_values` shows the patch landed.
- `day_boundary_fires_on_ticks_per_day` — `ticks_per_day=4`, only tick 4
  signals `boundary_reached=true` and bumps `day_index`.
- `boundary_requests_reach_tree_maintainer` — boundary requests survive
  the channel + Patcher park + boundary handoff and the Maintainer's
  classifier counts them correctly.
- `many_patches_same_cell_coalesce_to_one_intent_delta` — 10 Set patches → 10
  applied writes → 1 GPU intent delta and 0 dirty-row uploads.
- `add_and_multiply_patches_apply_on_gpu_without_rmw_readback` — Add/Multiply
  compose in order and apply on GPU with 0 RMW row readbacks.

**Not yet built in simthing-feeder:**
- OS-thread spawning. The structs are designed for a single feeder
  thread but `tick()` is a method, not a loop. The eventual driver in
  `simthing-sim` decides cadence and thread placement.
- `build_overlay_deltas` integration. Today the caller uploads Pass 3
  deltas separately at day boundaries; the dispatcher doesn't own that.

### simthing-sim (Week 3, complete)

**`threshold_registry.rs` — CPU-side event_kind ↔ semantic action mapping:**
- `ThresholdSemantic` enum: `FissionTrigger`, `FusionTrigger`, `PropertyExpiry`,
  `VelocityAlert` — each variant carries the ids and indices needed to
  reconstruct the meaning of a GPU `ThresholdEvent`.
- `ThresholdRegistry` — `Vec<ThresholdSemantic>` indexed by `event_kind: u32`.
  Rebuilt from scratch at each boundary.
- `ThresholdBuilder::build(root, registry, allocator) -> (Vec<ThresholdRegistration>, ThresholdRegistry)` —
  walks the tree once, emits both the GPU registration vec (for `state.upload_thresholds`)
  and the parallel CPU semantic lookup. Sources: `FissionThreshold` list per
  property, plus `DecayBehavior::{OnThreshold, IntensityGated, WhenProperty}`
  mapped to `PropertyExpiry`. `TowardZero`/`AfterTicks` are CPU-side only.
- `ThresholdBuilder::build_with_velocity_alerts(...)` extends the same upload
  with AI-facing `VelocityAlertRegistration` rows for a specific
  SimThing/property/sub-field trajectory.

**`overlay_lifecycle.rs` — step 4 + 7:**
- `OverlayLifecycle::Suspended { when_activated }` — present in CPU tree and
  observability; skipped by GPU overlay prep and Pass 3 until activated.
- `resolve_overlay_lifecycle(root, registry, allocator, shadow, n_dims, day)`
  walks the tree; for each transient overlay it evaluates every
  `DissolveCondition` (PropertyReaches, PropertyBelow, AfterTicks, OverrideReceived,
  Never), decrements `AfterTicks::remaining`, removes overlays whose conditions
  are all met, and applies `on_expire` `ExpireEffect`s (AddVelocity / SetIntensity)
  to the CPU shadow.
- `attach_overlay(root, target, overlay) -> bool` — depth-first attach (helper,
  superseded by `tree_mutation::AttachOverlay` in the boundary protocol).

**`property_expiry.rs` — step 5:**
- `resolve_property_expiry(root, registry, allocator, shadow, n_dims, events, cpu_reg)`:
  - For each `ThresholdEvent` with `event_kind` → `PropertyExpiry`:
    HashMap remove + registry tombstone if last live instance.
  - CPU-side sweep for `DecayBehavior::AfterTicks { remaining: 0 }` and
    `DecayBehavior::TowardZero` (|amount| < 1e-4). TowardZero reads the
    boundary-synchronized CPU shadow, not stale `PropertyValue::data`.
  - CPU-side tombstoning happens after collecting removals and checking
    property liveness from the root, so one branch cannot tombstone a property
    still present in a sibling subtree.

**`fission.rs` — step 6:**
- `resolve_fission_fusion(root, registry, allocator, events, cpu_reg, shadow, n_dims, day)`.
- Deduplicates by `(sim_thing_id, template_idx)` to prevent multiple events
  from firing the same fission twice in one boundary.
- Checks `SecondaryCondition` (IntensityAbove/Below, AmountAbove/Below) against
  the triggering property's shadow columns before mutating the tree.
- Fission: spawn `SimThing { kind: template.child_kind }`, alloc its slot,
  seed it from the parent's shadow row, zeroing the activating property's
  Amount, then attach as child of trigger SimThing.
- When `FissionTemplate::clone_capability_children: true`, deep-clone capability
  container subtrees from parent to spawned child (fresh ids, shadow rows, overlay
  `affects` remap). Default `false` — cohort/location fission unchanged.
- Fusion: `execute_fusion` applies the scar to the parent's activating-property
  Amount, tombstones the child, and removes the lineage record. Threshold
  registration for fusion comes from `ThresholdBuilder::build_with_lineage`.

**`tree_mutation.rs` — steps 7 + 8 (Tree Maintainer execution):**
- `apply_structural_mutations(requests, root, allocator, registry, shadow, n_dims) -> MaintainerOutcome`.
- `AddChild`: walk to parent, attach child, `populate_from_subtree` allocs slots
  for the entire attached subtree, zeroes each new row, then projects the
  child's initialized semantic properties into the shadow.
- `Remove`: detach subtree, zero each row, then tombstone every descendant's
  slot (otherwise descendant rows stay GPU-live but tree-unreachable).
- `Reparent`: detach + re-attach. Slots preserved — the whole point of slot
  stability. Cycle detection rejects `child → ancestor(child)`.
- `AttachOverlay`: depth-first find + push into target's overlay vec.
- `ActivateOverlay` / `SuspendOverlay`: transition overlay lifecycle in place
  (no slot alloc, no tree reshape). Activation unwraps `Suspended { when_activated }`.
- `AddDimension`: restores/adopts a registered property id, records it in
  `MaintainerOutcome::dimensions_added`, and lets `BoundaryProtocol` widen
  the CPU shadow + rebuild `WorldGpuState` before step 9.

**`gpu_sync.rs` — step 9:**
- `sync_gpu_buffers(root, registry, allocator, coord, state, velocity_alerts)`:
  1. `build_overlay_deltas` + pad ranges to `state.n_slots` + `upload_overlay_deltas`.
  2. `ThresholdBuilder::build_with_velocity_alerts` + `upload_thresholds`.
     Returns the new CPU registry.
  3. `coord.upload_full_shadow(state)` — every row, fresh.

**`boundary.rs` — `BoundaryProtocol`:**
- Owns the authoritative `SimThing` root, `DimensionRegistry`, `SlotAllocator`,
  CPU `ThresholdRegistry`, and registered velocity alerts.
- `can_skip_empty_boundary(events, patcher)` — static fast-path when there are no
  threshold events, no pending boundary/player/AI work, and no transient lifecycle
  or CPU-decay work due.
- `execute(events, patcher, coord, state, day) -> BoundaryOutcome` runs the full
  boundary sequence (13 steps when not skipped; see `design_v5.md` §11):
  1. **Reads GPU values back into `coord.shadow`** (critical: integration
     output lives only on GPU; otherwise the eventual `upload_full_shadow`
     would wipe out a day's worth of Pass 1/2 work).
  2. Collects fired `VelocityAlert` events into `BoundaryOutcome::velocity_alerts`.
  3. Overlay lifecycle (step 4).
  4. Property expiry (step 5).
  5. Fission/fusion (step 6). Fission-spawned children inherit active
     parent properties from the boundary GPU readback row; the activating
     property's Amount is reset to 0.0 on the child.
  6. Resize shadow for any new slots from step 6.
  7. Drain patcher boundary requests, call `apply_structural_mutations` (steps 7+8).
  8. If `AddDimension` expanded `registry.total_columns`, widen
     `coord.shadow`, project any newly-present sparse property values into
     the new columns, and rebuild `WorldGpuState` buffers.
  9. Resize shadow again if step 7 added slots.
  10. Assertion: `allocator.capacity() <= state.n_slots` (no GPU buffer overflow).
  11. `sync_gpu_buffers` (step 9).
  12. Adopt the new CPU `ThresholdRegistry`.
  13. Append `entries_from_outcome` to the boundary delta log.

**Outcome id vecs (PR #20):** `FissionOutcome::{fission_pairs,fusion_pairs}`,
`MaintainerOutcome::reparented`, `ExpiryOutcome::expired` — fed to
`delta_log::entries_from_outcome` for per-event replay entries.

**Tests: 79 unit + 19 GPU integration tests, all passing, zero warnings.**

Integration highlights:
- `fission_event_spawns_child_and_day_n_plus_1_tick_runs_clean` — full end-to-end:
  cohort with Amount=0.5, Velocity=-0.21 integrates across the 0.3 threshold;
  Pass 7 fires; `BoundaryProtocol::execute` spawns a child Cohort + allocates
  its slot; the child inherits loyalty from the parent's GPU row with Amount
  reset to 0.0; the subsequent tick runs without panic and the original
  cohort's amount continues integrating downward.
- `boundary_requests_apply_structural_mutations` — `AddChild` request submitted
  via the channel survives the patcher's boundary park, lands at the maintainer,
  attaches a new SimThing under the cohort, and allocates its slot.
- `add_dimension_request_rebuilds_gpu_layout` — property registered after GPU
  state creation is admitted at boundary time; CPU shadow and GPU buffers widen,
  and a sparse property value on the cohort appears in the new GPU columns.
- `velocity_alert_registration_surfaces_at_boundary` — an AI-facing velocity
  alert registered on a cohort's Velocity sub-field is uploaded to Pass 7 and
  returned through `BoundaryOutcome::velocity_alerts` after it fires.
- `aggregate_alert_registration_surfaces_at_boundary` — an AI-facing aggregate
  alert on a Location's reduced Amount column fires after reduction and surfaces
  through `BoundaryOutcome::aggregate_alerts`.
- `observe_live_reports_integrated_gpu_values_mid_day` — mid-tick Add/Multiply
  on GPU shows up in `observe_live` but not in shadow-based `observe`.

**Still open (see `docs/worklog.md` and `docs/todo.md`):**
- **V6 guardrails:** GPU boundary test for activated suspended overlay → next-tick Pass 3
  effect; end-to-end replay test for fission with cloned capability subtree; serde default
  test for `clone_capability_children`.
- **B2:** retain/batch threshold/reduction topology on fission growth boundaries.
- **Studio:** capability-tree authoring layer per `capability_tree_v1.md` (simulation
  crates stay agnostic).
- Full RON scenario files (tree + registry inline; today: `builtin` templates only).
- Designer UI (`simthing-studio`) — tabled

**Shipped recently:** intent-fold accumulator reuse, mid-tick observability docs,
`rebellion_demo` record/replay smoke, `tree_index` for fission + structural/lifecycle/expiry
lookups, B1 targeted boundary value-row upload, and safe threshold/reduction retention
for topology-stable active boundaries.

**Built (playability):**
- `crates/simthing-driver` exposes `simthing record`, `simthing replay`, and
  `simthing bench`.
- `crates/simthing-driver` — `SimSession`, `Scenario`, `simthing record` / `simthing replay` CLI.
- `scenarios/rebellion_demo.ron` — builtin rebellion demo (World → Location → Cohort).

**Design decisions (closed):**
- **Fission re-fire:** recurring rebellions are intentional. `FissionTrigger`
  stays live; no suppression when Amount re-crosses. See `docs/state-authority.md`.
- **Capability trees:** one `Custom(...)` SimThing per tree (not a tree of SimThings).
  Progress = GPU property sub-fields; unlock payload = `Suspended` overlay per entry;
  studio layer issues `ActivateOverlay` at boundary when Pass 7 threshold fires.
  Research costs, prereqs, display names, and RON metadata never enter simulation
  crates. See `capability_tree_v1.md` and `workshop/tech_tree_decisions.md`.

### simthing-sim::fission lineage + fusion scar
- `FissionLineageRecord { parent_id, child_id, property_id, template_idx }`
  emitted by `execute_fission`, accumulated on `BoundaryProtocol`.
- `ThresholdBuilder::build_with_lineage` walks the persistent lineage vec
  and emits one `FusionTrigger` registration per record on the child's
  activating-property Intensity column, threshold =
  `template.fusion_intensity_threshold`, direction = Upward.
- `execute_fusion` applies the scar via `apply_fusion_scar`: parent's
  activating-property Amount in the shadow is multiplied by `(1 -
  fusion_scar_coefficient)`.
- Lineage pruned on fusion (`lineage_removed`) and on any allocator
  tombstone at boundary start.

### simthing-sim::replay (LDJSON v2, complete)
- `ReplaySnapshot { day, root, registry, fission_lineage }` — initial state,
  serialized as the first line. `BoundaryProtocol::snapshot(day)` produces one.
- `ReplayFrame { day, entries, shadow_values? }` — one boundary's deltas plus
  optional post-boundary shadow checkpoint; written one per line after the snapshot.
- `ReplayWriter` enforces snapshot-first ordering; `ReplayReader` enforces
  snapshot-first read.
- `ReplayDriver` reconstructs tree + registry + allocator from a snapshot,
  then applies frames via structural mutations equivalent to the live
  `BoundaryProtocol`. Structural entries carry the payloads needed for
  reproduction, including `SimThingAdded { parent, node }`,
  `FissionOccurred { parent, node }`, and fission-lineage add/remove records.
- `BoundaryDeltaEntry::OverlayAttached` carries the full `Overlay` payload,
  resolved by `entries_from_outcome(outcome, root)` at log-build time from
  `MaintainerOutcome::overlays_attached: Vec<(SimThingId, OverlayId)>`.
- Format: LDJSON. Non-string-keyed maps (`SimThing.properties`,
  `DimensionRegistry.by_name`) serialize as pair arrays via `serde_with`.

**State authority:** see `docs/state-authority.md` for tick vs boundary numeric
truth, GPU intent-delta vs legacy shadow paths, player/AI intent two-phase behavior,
and `SimThing.properties` vs GPU/shadow roles.

---

## How to run tests

```
cd C:\Users\mvorm\SimThing
cargo test
```

All **197** tests must pass with zero warnings before any commit
(16 core + 3 driver unit + 3 driver integration + 48 GPU + 24 feeder unit +
5 feeder integration + 79 sim unit + 19 sim integration).
One additional ignored timing diagnostic runs with `cargo test -- --ignored`.

GPU tests skip themselves cleanly when no adapter is available
(`try_gpu()` returns `None`) — CI without a GPU still completes successfully.

### Run a recorded session

```
cargo run -p simthing-driver -- record --scenario scenarios/rebellion_demo.ron --out demo.replay.ldjson
cargo run -p simthing-driver -- replay --in demo.replay.ldjson
```

Requires a GPU adapter for `record`; `replay` is CPU-only structural playback.

The `custom_layout_ethics_axis` test is the proof that the generalization works beyond the
standard amount/velocity/intensity layout. If you add a new layout capability, add a test in
this pattern.

The `pass3_overlay_matches_evaluator` test is the proof that iterative GPU
transform application stays bit-exact with the CPU `Evaluator` across all three
`TransformOp` variants at both ancestor and local levels. Do not weaken this
test; any new transform variant must extend it with a parity assertion.

---

## The invariants

`docs/invariants.md` has the full list. The ones most likely to be violated accidentally:

**I1:** Column arithmetic has exactly one home.
`PropertyLayout::offset_of` for local offsets. `PropertyColumnRange::col_for_role(role, layout)`
for global columns. Nothing else does column math. No exceptions.

**I3:** Velocity pinning at boundaries.
This is in `PropertyValue::integrate`. Don't move it. Don't add a flag to disable it.
Hidden velocity debt is not a feature.

**I4:** No index constants.
`AMOUNT_IDX`, `VELOCITY_IDX`, `INTENSITY_IDX`, `VECTOR_START_IDX` are banned.
Access sub-fields via `layout.offset_of(&SubFieldRole::Amount)`.

**I5:** Overlays use roles, not column indices.
`PropertyTransformDelta` stores `SubFieldRole`, not `usize`. Column resolution happens in the
CPU preparation pass at dispatch time.

**I7:** Structural mutations only at the day boundary.
Enforced by architecture: the within-day Patcher cannot touch the tree; boundary
requests and player/AI intents park until `BoundaryProtocol::execute`.

---

## Design decisions already made — don't relitigate

**IntensityBehavior uses linear coefficients, not function pointers.**
Reason: function pointers don't serialize; linear coefficients map directly to WGSL uniforms.
If you need non-linear intensity dynamics, model it as a different property with a different
governed_by relationship, not as a function pointer.

**`SimProperty` equality and hashing are on namespace+name only.**
Reason: the registry key must be stable across layout changes (version migrations). Metadata
does not participate in key comparison.

**`stride()` is computed, not stored.**
Reason: eliminates the class of bugs where stored stride diverges from actual sub-field widths.

**Velocity pinning at floor/ceiling, not velocity clamping.**
Reason: velocity that pushes in the recovery direction must always be permitted through.
Only velocity that would push further into the already-saturated direction is zeroed.

**`GovernedPair` encodes `ClampBehavior` as a u32 tag with sentinel float values.**
Reason: WGSL structs must be `#[repr(C)]` with fixed-size fields. `ClampBehavior` is a Rust
enum which cannot be sent to the GPU directly. Encoding uses `clamp_kind: u32`
(0=Bounded, 1=Floored, 2=Unbounded) with `±INFINITY` sentinels in `clamp_min`/`clamp_max`
for the cases where bounds are not meaningful. The WGSL shader reads `clamp_kind` and branches.

**`threshold_registry` and `event_candidates` deferred to Pass 7.**
Reason: their shape depends on threshold registration (fission thresholds, velocity thresholds,
decay conditions) which doesn't exist yet. Adding empty placeholder buffers now produces
untestable dead code. Add them when threshold registration API is designed.

**`intensity_params` buffer is property-level, built from `IntensityBehavior`.**
Reason: Pass 2 needs per-property `velocity_threshold`, `build_coefficient`, `decay_coefficient`.
One entry per active property that has both `IntensityBehavior` and the required Velocity +
Intensity sub-fields in its layout — properties missing either role are silently skipped,
mirroring `PropertyValue::update_intensity`. Built in Week 2 alongside the Pass 2 shader.

---

## FMA divergence — decision required before writing Pass 1

WGSL allows `mul`+`add` fusion into FMA (fused multiply-add) at the compiler's discretion.
The Pass 1 integration expression `position + velocity * dt` may FMA-fuse on GPU but will
not on the CPU oracle (which uses standard sequential `f32` arithmetic). On some hardware
this produces 1-ULP divergence, which fails the `to_bits()` parity test (Invariant I8).

**Choose one approach before writing the Pass 1 shader. Do not defer this.**

**Option A — CPU uses `f32::mul_add` to match GPU FMA:**
Update `PropertyValue::integrate` to use `f32::mul_add(velocity, dt, current_value)`.
CPU oracle now produces FMA-equivalent results. GPU can fuse freely.
Pro: GPU runs at full hardware speed.
Con: CPU oracle no longer matches naive f32 arithmetic; may surprise future contributors.

**Option B — WGSL shader explicitly prevents FMA fusion:**
Write integration as two separate assignments: `let scaled = velocity * dt; position = position + scaled;`
WGSL spec: intermediate `let` bindings prevent FMA. GPU matches naive CPU f32.
Pro: CPU oracle needs no changes.
Con: marginally slower on FMA-capable hardware (negligible at this workload scale).

**Recommendation: Option B.** The performance difference is negligible. Explicit FMA prevention
is a one-line auditable shader decision. Changing the CPU oracle to use `mul_add` silently
alters the behavior of the authoritative reference path and may mask future precision bugs.

**Outcome (Week 2):** Option B implemented and bit-exact verified on naga + DX12.
The `velocity_integration_matches_cpu_oracle_fractional_dt` test stresses `dt = 0.5`
with non-power-of-2 inputs; `to_bits()` parity holds. If a future driver fuses despite
the `let` bindings, that test fails loudly and the fallback is `f32::mul_add` on the
CPU side + WGSL `fma()` in the shader.

---

## Transform application — iterative on GPU (decided)

`TransformOp::{Add, Multiply, Set}` is not a closed group under N×N matrix
multiplication. `Multiply(k)` is linear (diagonal entry `k`); `Add(k)` is a
translation (needs a bias term); `Set(k)` discards the input. An earlier draft
proposed affine `(M, b)` composition on the CPU prep pass with a single matmul
on the GPU. **That approach was considered and rejected.** Pass 3 instead
applies overlays **iteratively on the GPU**.

### Why iterative

- **Bit-exact parity is trivial.** Both `Evaluator::apply_to_data` and the
  Pass 3 shader walk a list of `(col, op, value)` deltas in stack order and
  apply each op the same way. No composition step means no rounding-order
  divergence. The `Evaluator` stays as-is.
- **Per-tick GPU work is proportional to active overlays, not `n_dims²`.**
  At realistic overlay loads (~10–20 deltas per slot's stack), iterative is
  ~10 ops/slot; the affine matmul would have been ~4096 ops/slot at `n_dims = 64`.
- **GPU memory plummets.** The affine path would have needed two
  `n_slots × n_dims²` matrix buffers and two `n_slots × n_dims` bias buffers
  — ~370 MB at endgame scale. Iterative replaces all of that with a flat
  delta array (~4 MB) and a per-slot range table (~90 KB).
- **Cross-property / cross-column transforms still work.** A future op
  variant that mixes columns (e.g. rotation, cross-property pressure) is a
  new `TransformOp` variant the shader branches on. Same flexibility as
  affine, less infrastructure.

The trade-off is variable per-thread work — slots with longer overlay stacks
run more iterations than others. At our scale this is fine; if it ever
matters, batch by stack length or pad to a fixed max.

### Data shape

```rust
#[repr(C)] #[derive(Pod, Zeroable)]
struct OverlayDelta {
    col:     u32,   // global column index (resolved via col_for_role at prep time)
    op_kind: u32,   // 0=Multiply, 1=Add, 2=Set
    value:   f32,
    _pad:    u32,   // align stride to 16 bytes
}

#[repr(C)] #[derive(Pod, Zeroable)]
struct SlotDeltaRange {
    offset: u32,   // index into overlay_deltas
    length: u32,   // number of deltas to apply for this slot
}
```

`overlay_deltas` is the flat concatenation of every slot's ancestor + local
stack, in evaluation order. `slot_delta_ranges` is indexed by `slot_idx`.
A slot with no overlays has `length = 0` and the shader is a no-op for it.

### CPU prep pass

```
fn build_overlay_deltas(root, registry, allocator) -> (Vec<OverlayDelta>, Vec<SlotDeltaRange>):
    walk tree depth-first carrying an ancestor stack of overlays
    for each node:
        slot = allocator.slot_of(node.id)
        record offset = deltas.len()
        for overlay in ancestor_stack:
            for (role, op) in overlay.transform.sub_field_deltas:
                col = registry.col_for_role(overlay.transform.property_id, role)
                deltas.push(OverlayDelta { col, op_kind, value })
        for overlay in node.overlays:
            ...same emission...
        record length = deltas.len() - offset
    return (deltas, ranges)
```

Mirrors `TransformStack` semantics exactly: ancestor overlays apply first, in
push order; then local overlays in registration order.

### Pass 3 shader (sketch)

```wgsl
@compute @workgroup_size(64)
fn pass_3(@builtin(global_invocation_id) gid: vec3<u32>) {
    let slot = gid.x;
    if (slot >= n_slots) { return; }
    let range = slot_delta_ranges[slot];
    let base = slot * n_dims;

    for (var i = 0u; i < range.length; i = i + 1u) {
        let d = overlay_deltas[range.offset + i];
        let addr = base + d.col;
        switch (d.op_kind) {
            case 0u: { values[addr] = values[addr] * d.value; }    // Multiply
            case 1u: { values[addr] = values[addr] + d.value; }    // Add
            case 2u: { values[addr] = d.value; }                    // Set
            default: { /* unreachable */ }
        }
    }
}
```

One thread per slot. Each thread walks its slot's delta range and applies
ops in place to `values`. Pass 3 reads from and writes to `values`; Passes
4–6 write reduced aggregates into `output_vectors` (Pass 7 can threshold
either buffer via `ThresholdRegistration.buffer`).

### Buffer changes in `WorldGpuState`

The earlier matrix-based plan reserved `local_transforms` /
`ancestor_transforms` (each `n_slots × n_dims² × 4B`). Those buffers are
**removed** in favor of:

```
overlay_deltas      : Vec<OverlayDelta>          uploaded each tick
slot_delta_ranges   : Vec<SlotDeltaRange>        uploaded each tick
```

Both are `STORAGE | COPY_SRC | COPY_DST`. Empty cases get a placeholder
allocation so the buffers remain bindable.

---

## Week 2 scope (complete — kept here for reference)

### Architecture note — governed_pairs is a separate buffer, not part of the transform matrices

Pass 1 (velocity integration) is a **pre-transform step**. It advances governed sub-fields
*before* the transform matrices in Pass 3 are applied. Do not fold `governed_by` pairs into
the transform matrix representation — that would conflict with Pass 3's transform application
and produce double-application on the same tick.

`EvaluationBatch` must carry a distinct `governed_pairs` buffer:

```rust
struct EvaluationBatch {
    base_vectors:      GpuMatrix,  // [N_slots × N_dims]
    overlay_deltas:    GpuBuffer,  // flat [OverlayDelta], ancestor stack then local, in evaluation order
    slot_delta_ranges: GpuBuffer,  // [N_slots × SlotDeltaRange { offset, length }]
    governed_pairs:    GpuBuffer,  // [(governed_col, governing_col, clamp_min, clamp_max, vel_max)]
    reduction_map:     GpuBuffer,
}
```

`overlay_deltas` and `slot_delta_ranges` replace the earlier matrix-based
`ancestor_xforms` / `local_xforms` plan. See "Transform application —
iterative on GPU" above for the reasoning.

`governed_pairs` is built from the `DimensionRegistry` during the CPU preparation pass by
iterating all active properties, finding sub-fields where `governed_by` is `Some`, and calling
`col_for_role` on both the governed and governing roles. It is a property-level buffer (same
pairs apply to every slot) — not a per-slot buffer. Pass 1 dispatches one thread per pair,
not one thread per (slot × pair); each thread handles all slots for its pair in a loop, or
alternatively dispatch is `(N_pairs × N_slots)` with the pair index in the workgroup.

The pass ordering is therefore:
```
Intent delta (when present): fold tick-time patches into values[]
Pass 0: snapshot
Pass 1: velocity integration     ← reads governed_pairs, writes values[]
Pass 2: intensity update          ← reads values[] (post-integration velocity)
Pass 3: transform application     ← reads overlay_deltas + slot_delta_ranges, writes values[] (in place)
Pass 4–6: reduction
Pass 7: threshold scan
```

---

Add `wgpu = "22"` and `rayon = "1"` to `[workspace.dependencies]` in `Cargo.toml`.

Create `crates/simthing-gpu/` with:

1. **`WorldGpuState`** — owns the wgpu device/queue and all GPU buffers:
   - `values`: `[slot * N_DIMS + col]` — current property values
   - `previous_values`: snapshot from Pass 0
   - `output_vectors`: per-slot output after reduction (Pass 4–6 destination)
   - `governed_pairs`: flat array of `(governed_col, governing_col, clamp_min, clamp_max, vel_max)`
   - `intensity_params`: flat array of per-property IntensityBehavior coefficients
   - `overlay_deltas`: flat `[OverlayDelta]` — ancestor stack then local, in evaluation order
   - `slot_delta_ranges`: `[N_slots × SlotDeltaRange]` — `(offset, length)` per slot
   - `threshold_registry`: flat array of threshold registrations *(deferred — Pass 7)*
   - `event_candidates`: sparse output from Pass 7 *(deferred)*

2. **`EvaluationBatch` builder** — CPU preparation pass:
   - Walk the SimThing tree
   - For each node, compose ancestor transforms using `TransformStack`
   - Resolve `PropertyTransformDelta` sub-field roles → column indices via `col_for_role`
   - Build `governed_pairs` from registry: for each active property, for each sub-field with
     `governed_by: Some(role)`, emit `(col_for_role(governed), col_for_role(governing), clamp_params)`
   - Write to `WorldGpuState` buffers (delta upload only)

3. **GPU Pass 1** (velocity integration) — WGSL compute shader:
   - One thread per `(slot, governed_pair_index)`
   - Read governing col value, apply velocity_max clamp, integrate, apply ClampBehavior
   - Write velocity pin if at boundary

4. **GPU Pass 2** (intensity update) — WGSL compute shader:
   - One thread per `(slot, intensity_col)` pair
   - Apply IntensityBehavior linear coefficients

5. **Verification harness**: run `Evaluator` (CPU oracle) and GPU pipeline on identical initial
   state, compare all output values with `assert_eq!(cpu_val.to_bits(), gpu_val.to_bits())`.

GPU output must match CPU oracle to the float bit. This is not optional. See Invariant I8.

---

## What success looks like at the end of Week 2 — achieved

```
cargo test                                  →  45/45 passing, zero warnings
                                               (14 core + 31 GPU)
VRAM usage at 100 SimThings, 8 dimensions   →  within 5% of projection
                                               (vram_budget_at_100_slots_8_dims test)
GPU pass timing at 1000 SimThings, 64 dims  →  ~1.2 ms (50 ms budget; 40× headroom)
                                               (pipeline_timing_1000_slots_64_dims,
                                                cargo test -- --ignored)
```

---

## Code style notes

- No comments explaining what the code does. Names should do that.
- Comments only for non-obvious WHY: a hidden constraint, a specific invariant reference,
  a workaround for a wgpu behavior, a simulation design decision.
- Reference invariants by number when a code comment explains a rule: `// I3: velocity pin`.
- Tests live in the module they test (`#[cfg(test)] mod tests` at the bottom of each file).
- New types go in the module that owns them. Don't create new files for small additions.
- No `unwrap()` in non-test code without a comment explaining why the None case is impossible.
