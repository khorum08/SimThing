# SimThing Todo Log

Current parking state: **`simthing-spec` PRs 1–11 complete**; Phase 1 ADRs,
**O3**, Composer **S3/S4**, **O1** (session installation), and post-O1 doc
sync landed. `master` and `origin/master` synced at **`b0912bc`** (PR #57).

**Tests:** `cargo test --workspace` → **322** passed, **2** ignored, zero
warnings. Debug and **release** profile build/tests clean.

**Canonical spec progress:** `docs/workshop/simthing_spec_progress_log.md`
(replaces PR-scoped workshop handoffs — see `docs/workshop/README.md`).

**Implementation:** `simthing-driver::SpecSessionState` owns spec runtime
state; `simthing-driver::install` compiles a `GameModeSpec` against a
`Scenario`, clones capability trees per owner with fresh `OverlayId`s, and
populates the spec state. `SimSession::open_from_spec(scenario, &game_mode)`
is the RON-driven entry point. `BoundaryProtocol::execute_with_boundary_hook`
invokes capability and scripted-event handlers after GPU readback;
`simthing-sim` remains spec-free.

**`by_overlay` migration:** moved from `CapabilityTreeDefinition` (shared)
to `CapabilityTreeInstance` (per-clone). `CapabilityTreeBuildOutput` exposes
`template_by_overlay` for the install module to re-stamp.

### Next — ordered (Codex evaluation 2026-05-23)

| Priority | ID | Owner | Scope |
|----------|-----|-------|-------|
| **P0** | **O1b** | **Codex** | Fix `open_from_spec` threshold unlock E2E — handler must emit `ActivateOverlay` with **per-clone** overlay ids from `instance.by_overlay`, not template ids in `CapabilityDefinition` |
| **P0** | **O1b-test** | **Cursor** | ✅ Test landed (`open_from_spec_capability_unlock_activates_overlay_for_next_tick`, **ignored/RED**) — un-ignore when Codex fix lands |
| **P1** | **O1c** | **Codex** | Registry/GPU dimension sync after install — **ruled out** by O1b run (`n_dims == registry.total_columns`); keep only if a future case reopens |
| **P1** | **S5/O5** | **Codex** | Append-only thresholds + disable Approach C append when `clone_capability_children` (conservative fix first) |
| **P2** | **O4** | **Codex** | Per-owner scripted events (Option B ADR) |
| **P2** | **O2** | **Codex** | Replay v3 (`SpecSnapshot`/`SpecDelta`; `by_overlay` precondition met) |
| **P3** | **EffectTarget** | **Opus** | ADR for capability effect target scope (Owner vs CapabilityTree vs SessionRoot) before Studio/modder exposure |

**Do not start O4/O2 until O1b handler fix is green (un-ignore E2E test).**

**Known O1 risks (documented, not yet fixed in code):**

- **O1b RED (2026-05-23):** install re-stamps overlay ids on each clone (`instance.by_overlay`), but `CapabilityTreeBoundaryHandler::emit_activation` still uses template `overlay_ids` from `CapabilityDefinition` → `ActivateOverlay` misses cloned tree overlays.
- `open_from_spec` runs `SimSession::open` before spec properties register — O1b test showed `coord.n_dims` matches expanded registry after `install_spec_state`; dimension lag is not the current blocker.
- Capability overlay `affects` currently target the **cloned tree**, not the owner (intentional v0; modder semantics TBD via Opus ADR).
- `compile_and_install` mutates registry/root in place on error (safe for `open_from_spec` discard; Studio preview needs clone-then-commit later).

**Worktree:** clean for tracked files. Untracked `.claude/worktrees/`,
`demo.replay.ldjson`, and draft workshop files may be present locally.

---

## Done

### V6 simulation core (`f39fe6d`)

- [x] Add `OverlayLifecycle::Suspended { when_activated }`.
- [x] Keep suspended overlays out of CPU evaluator and GPU overlay-delta prep.
- [x] Add boundary-time `ActivateOverlay` and `SuspendOverlay` requests.
- [x] Record overlay activation/suspension in the boundary delta log.
- [x] Replay overlay activation/suspension transitions.
- [x] Add `active` attribution to observability overlay contributions.
- [x] Ensure suspended overlays do not force empty-boundary work.
- [x] Add `FissionTemplate::clone_capability_children` (serde default `false`).
- [x] Clone capability containers on opted-in fission templates.
- [x] Allocate fresh IDs and slots for cloned capability subtrees.
- [x] Copy cloned capability shadow rows.
- [x] Remap cloned overlay `affects` from parent owner to spawned owner.
- [x] Pre-grow boundary slot headroom for cloned capability subtrees.

### Capability-container kind parameterization (PR #38, `a8aab5b`)

- [x] Add `FissionTemplate::capability_container_kinds: Vec<String>` with
      `#[serde(default)]` — empty vec when field omitted.
- [x] Remove hardcoded `"tech_tree" | "national_ideas" | "talent_tree"`
      checks from `simthing-sim` production code.
- [x] Shared `is_capability_container(kind, container_kinds)` in
      `fission.rs`; `boundary.rs` imports it for pre-grow headroom.
- [x] **Option A semantics:** `clone_capability_children: true` with empty
      `capability_container_kinds` clones nothing — no sim-crate fallback list.
- [x] Thread `&ft.template.capability_container_kinds` through
      `execute_fission` → `clone_capability_children` and through
      `projected_fission_slots` pre-grow estimation.
- [x] Serde compatibility test:
      `fission_template_deserializes_without_capability_container_kinds`
      (`simthing-core`).
- [x] Fission unit test: `clone_capability_children_empty_kinds_clones_nothing`.
- [x] Update existing clone/headroom tests to populate
      `capability_container_kinds` explicitly.
- [x] Doc addenda in `design_v6.md` and `capability_tree_v1.md`; agent briefing
      sync in `agents.md`.

### `simthing-spec` PR 1 — authoring-only scaffold (PR #46, `7eb48dc`)

- [x] Crate + workspace membership; depends on `simthing-core` only (PR 1).
- [x] `GameModeSpec`, `DomainPackSpec`, capability RON structs, `PropertySpec` /
      `OverlaySpec` placeholders.
- [x] Generic `SpecDiagnostics`, `SpecVersion`, `DisplayMeta`, logical keys.
- [x] RON loaders + lightweight `validate_capability_tree`.
- [x] PR 1 tests (`tests/pr1_spec.rs`, `validate` unit tests).
- [x] Reverted exploratory builder/boundary/threshold code from PR #45.

---

## Next

### `simthing-spec` (revised PR ladder — PR 1 done)

Authoritative spec: `simthing-spec — Master Implementation Handoff` (2026-05-22).
All PRs sequenced deliberately; do not skip ahead. **Use Opus for all five PRs.**

- [x] **PR 2** — property + overlay spec compiler (`compile/property.rs`,
      `compile/overlay.rs`, `compile/context.rs`). Landed 2026-05-22.
      `PropertySpec` expanded with `description` + `sub_fields`; empty
      `sub_fields` defaults to `PropertyLayout::standard(0)`. `OverlaySpec`
      expanded with `targets_property`, `sub_field_deltas`, `lifecycle`,
      `kind`, `source`. `compile_property` checks `id_of` before
      `register` (no panic on duplicate), validates `governed_by`
      against the same layout. `compile_overlay` parses `"ns::name"`,
      validates property existence, validates each sub-field role
      against the target's layout. New errors:
      `DuplicateProperty`, `UnknownProperty`, `InvalidGovernedByRole`,
      `InvalidSubFieldRole`, `InvalidPropertyReference`. Tests:
      `tests/pr2_compile.rs` — 11 passing (all 7 acceptance criteria
      from the handoff + 4 supplementary).
- [x] **PR 3** — `CapabilityTreeBuilder`. Landed 2026-05-22.
      `runtime/capability_definition.rs` defines `CapabilityTreeDefinitionId`
      (atomic newtype), `CapabilityTreeDefinition` (shared, immutable,
      `entries` / `by_threshold` / `by_overlay` lookups), `CapabilityDefinition`
      (per-entry with parallel `overlay_ids` / `effect_keys`), `CapabilityPrereq`
      (resolved `property_id` / `role` / `col` / `min_value`), and a
      placeholder `CapabilityUnlockRegistration` (PR 4 moves to feeder).
      `compile/capability.rs::CapabilityTreeBuilder::build` runs validation,
      registers one `SimProperty` per category with one `Named(entry.id)`
      sub-field each (`ReductionRule::Max` forced via `reduction_override`),
      constructs the template `SimThing` (`Custom(tree_kind)`), compiles
      each effect into a `Suspended { when_activated: ... }` `Overlay`,
      resolves prereqs (cross-category supported via `"ns::name"` strings),
      and emits one `CapabilityUnlockRegistration` per `Threshold` entry
      (`PlayerSelection` produces none). `ActivationMode` gains `OnPrereqMet`;
      `validate.rs` rejects it as an authored default plus `Limited(n != 1)`
      and self-referential prereqs. Tests:
      `tests/pr3_capability_builder.rs` — 16 passing (all 11 acceptance
      criteria from the handoff + 5 supplementary).
- [x] **PR 4** — capability unlock registration bridge. Landed 2026-05-22.
      `CapabilityUnlockRegistration` (with `Serialize/Deserialize` derives)
      lives in `simthing-feeder::capability`; `simthing-spec` re-exports it
      via `runtime::capability_definition` (placeholder removed) and gains
      a `simthing-feeder` dep. `simthing-sim::threshold_registry` adds
      `ThresholdSemantic::CapabilityUnlock { sim_thing_id, property_id,
      sub_field }` (with `Serialize/Deserialize` derives on the whole enum)
      plus `ThresholdBuilder::build_with_capability_unlocks(root, dim_reg,
      allocator, velocity_alerts, capability_unlocks)` and a
      `push_capability_unlocks` helper. The path is full-rebuild only; B2
      append-only integration deferred. Skipping behavior matches velocity
      alerts: inactive properties / unallocated sim_things / missing roles
      silently skip. `simthing-feeder/Cargo.toml` picks up `serde`. Tests:
      `simthing-feeder/src/capability.rs` (1), `threshold_registry.rs`
      tests (4 — 3 acceptance + 1 supplementary), and the GPU integration
      `capability_unlock_fires_in_boundary_integration_test` in
      `simthing-sim/tests/boundary_integration.rs` (uses a Permanent
      overlay attached to the cap tree to push progress across the
      threshold mid-Pass-3 — `submit_player_intent` doesn't work for
      this because intent_deltas apply BEFORE Pass 0's snapshot, so
      previous == current and the crossing isn't visible).
      All 5 handoff acceptance criteria met + 1 supplementary.
- [x] **PR 5** — capability runtime state + boundary handler
      (`boundary/capability_handler.rs`). Called by session coordinator,
      not embedded in `BoundaryProtocol`. Landed 2026-05-22 with Path A
      for `max_active`: `CapabilityCategorySpec.max_active` is now
      `Option<MaxActivePolicy>` with `Limited { count, replacement }`, and
      `ReplacementPolicy::SuspendOldest` is the supported v0 replacement.
      `CapabilityTreeDefinition` now carries category definitions; entries
      carry authored activation mode, `progress_col`, and `research_cost`.
      Added per-faction runtime state, notifications, diagnostics, and the
      boundary handler for threshold activation, failed-prereq reset into
      `OnPrereqMet`, fixpoint sweeps, player selection, per-faction active
      state, and `Limited(1)` sibling suspension. Tests:
      `tests/pr5_capability_handler.rs` — 10 passing acceptance tests.
- [x] **PR 6** — preview + mutual exclusivity completion
      (`preview/capability_preview.rs`). Landed 2026-05-22. Adds
      definition-only CPU preview for capability effects with per-overlay
      breakdowns and combined net deltas. `CapabilityDefinition` now carries
      compiled `effect_transforms` parallel to overlay/effect keys so preview
      does not need the template SimThing. Adds full national-ideas
      activate-switch verification by feeding PR 5 handler requests through
      real structural overlay activation/suspension. Tests:
      `tests/pr6_capability_preview.rs` — 5 passing acceptance tests.
- [x] **PR 7** — Script IR. Landed 2026-05-22. Replaces
      `spec/script_stub.rs` with canonical `ScriptExpr` / `ScriptPredicate`
      authoring IR, `PropertyKey`, `ScopeRef`, and a CPU evaluator over
      `DimensionRegistry + shadow + n_dims`. Supports constants, property
      reads, arithmetic, min/max, clamp, numeric gates, comparison predicates,
      boolean composition, serde round-trips, and hard evaluation errors for
      unknown property/role, bad slots/columns, division by zero, and invalid
      clamps. No EML, parser, trigger/effect compiler, or event system yet.
      Tests: `tests/pr7_script_ir.rs` — 10 passing acceptance/scaffold tests.
- [x] **PR 8** — trigger/effect/event compiler. Landed 2026-05-22 as a
      conservative typed-template slice: `TriggerSpec`, `EffectSpec`, and
      `EventSpec` compile into `CompiledTrigger`, `CompiledEffect`, and
      `ScriptedEventDefinition`. Simple threshold triggers resolve
      property/role/column against `DimensionRegistry`; predicate triggers
      preserve PR 7 `ScriptPredicate`; effects compile to boundary request
      templates for remove / activate overlay / suspend overlay. No event
      runner, threshold registry upload, parser, or EML. Tests:
      `tests/pr8_event_compiler.rs` — 7 passing scaffold tests.

### Parking notes / next candidates

- [x] **PR 9** — scripted event boundary handler. Landed 2026-05-22.
      `boundary/event_handler.rs` with `ScriptedEventBoundaryHandler`,
      `ScriptedEventBoundaryContext`, `ScriptedEventDiagnostic`,
      `ScriptedEventDiagnosticKind`. Predicate triggers only (threshold triggers
      deferred to GPU-path PR); cooldowns and priority ordering implemented.
      Missing slot targets push `UnresolvedEffectTarget` diagnostic. Eval errors
      push `TriggerEvalError` diagnostic. All 8 acceptance tests pass in
      `tests/pr9_event_handler.rs`.
- [x] **PR 10** — scripted-event GPU threshold path. Landed 2026-05-22.
      Adds `simthing_feeder::ScriptedEventTriggerRegistration` and
      `ScriptedEventTriggerEvent`; adds
      `ThresholdSemantic::ScriptedEventTrigger { event_id }` arm plus
      `ThresholdBuilder::build_with_scripted_event_triggers` and
      `ThresholdRegistry::extract_scripted_event_triggers` in
      `simthing-sim`; adds `ScriptedEventDefinition::to_trigger_registration`
      in spec. `ScriptedEventBoundaryHandler::handle_tick` now takes a
      `&[ScriptedEventTriggerEvent]` slice and fires threshold-triggered
      events under unified cooldown/priority gating with predicate-triggered
      events. New diagnostic variant: `UnknownEventId` for stale registrations.
      Bumps `simthing_core::Direction` with `Copy + PartialEq + Eq` derives.
      11 acceptance tests in `tests/pr10_scripted_event_thresholds.rs`.
- [x] **PR 11 Track A (Opus)** — session/driver assembly merged `01fb572`
      (2026-05-22). ADR: `docs/adr/pr11_track_a_session_assembly.md`.
      Driver-owned `SpecSessionState`, multi-tree-safe capability keys, generic
      post-readback boundary hook in sim, external threshold registration
      plumbing, `SimSession::install_spec_state`, GPU E2E unlock → handler →
      overlay → next-tick value change. **311** tests at landing.
- [x] **PR 11 Track B (Composer)** — mechanical prep merged PR #47 (`392992f`,
      2026-05-22): B5 release smoke check; B2 `EventKey: From<&str>`/`From<String>`;
      B1 `Display` for capability/scripted-event diagnostics; B3
      `append_capability_unlocks` / `append_scripted_event_triggers`;
      B4 docs addenda in `design_v6.md` and `capability_tree_v1.md`.
- [x] Assemble session/driver ownership for capability tree instances and
      runtime state maps. Driver storage is keyed by
      `(owner_id, definition_id, tree_thing_id)`; temporary one-instance maps
      are passed into the PR 5 handler to preserve current handler API while
      avoiding the session-level multi-tree footgun.
- [x] Clean up PR 5's temporary `simthing-spec -> simthing-sim` /
      `simthing-spec -> simthing-gpu` threshold dependencies. Done 2026-05-22.
      Approach: introduced `simthing-feeder::CapabilityUnlockEvent` as the
      resolved-event shape spec consumes; renamed handler entry point to
      `handle_capability_unlock_events`; added
      `ThresholdRegistry::extract_capability_unlocks` in `simthing-sim` as the
      bridge for callers that hold raw `ThresholdEvent`s. Spec production deps
      are now `simthing-core` + `simthing-feeder` only; `simthing-gpu` /
      `simthing-sim` remain as dev-dependencies for PR 6 integration tests.
- [ ] B2 append-only capability/scripted-event external registration integration
      remains deferred. Track A full rebuilds include external registrations;
      append-only handling for newly cloned capability trees is a later
      optimization/design item.
- [ ] Replay v3 for spec session state remains deferred. Existing structural
      overlay activations replay through the boundary delta log, but capability
      runtime state, scripted-event cooldowns, diagnostics, and notifications
      are not serialized yet.

**Known divergences between handoff doc and PR 1 code (Opus must resolve):**

Historical notes below were written before PRs 2-8 landed. Several are now
resolved; keep this section as archaeology until the handoff docs are folded
into the current code.

1. `CapabilityCategorySpec` has no `id` field — handoff §1.4 references one;
   actual struct identifies category by `property_namespace::property_name`.
   `CategoryKey { namespace, name }` in `keys.rs` already captures this.
   **Resolution:** add `id: String` to the struct and thread it through, OR
   accept that category id = `namespace::name` (matching `CategoryKey`).

2. `MaxActivePolicy` in `spec/capability.rs` is `Limited { count: usize }` — no
   `replacement: ReplacementPolicy` field; no `ReplacementPolicy` enum. Handoff
   §1.4 requires both. **Resolution:** add `ReplacementPolicy` enum and
   `replacement` field in PR 2/5 when needed.

3. `ActivationMode` is missing the `OnPrereqMet` arm — the comment says "will be
   added in later PRs." Handoff §1.3 defines all three arms.
   **Resolution:** add `OnPrereqMet` to the enum in PR 3; extend `validate.rs`
   to reject it as an authored default.

4. `CapabilitySpec.research_cost: f32` vs handoff `research_cost: ResearchRateSpec`
   — the struct also has a separate `research_rate: ResearchRateSpec` field,
   which is unused. **Resolution:** PR 3 builder reads `research_cost: f32` as
   the literal threshold value. The `research_rate` field is a vestige of an
   earlier design; either remove it or leave it unused. Do not rename `research_cost`
   (serde-breaking).

5. `PropertySpec` is a stub (`id`, `namespace`, `name`, `display_name` only) — no
   layout, no sub-field specs, no decay, no clamp, no governed_by. PR 2's
   `compile_property` enforces layout validity, so the struct must grow.
   **Resolution:** expand `PropertySpec` with at least a `sub_fields` layout
   description before writing the compiler, OR keep `compile_property` minimal
   (namespace+name registration with a default layout) and accept simpler tests.

6. `OverlaySpec` is a stub (`id`, `display_name` only) — no `targets_property`,
   `sub_field_deltas`, or `lifecycle`. PR 2's `compile_overlay` needs these.
   **Resolution:** expand `OverlaySpec` with those fields, or scope PR 2's
   `compile_overlay` to the standalone (non-capability) overlay use-case and
   note that capability overlays are built inline by the PR 3 builder.

7. `DimensionRegistry::register` panics on duplicate `namespace+name` — `compile_property`
   must check `registry.id_of(ns, name).is_some()` and return
   `Err(SpecError::DuplicateProperty(...))` before calling `register`.
   Add the error variant to `error.rs`.

8. No `registry.set_reduction_rule` method exists — handoff prose mentions it but the
   correct implementation is to set `reduction_override: Some(ReductionRule::Max)` on
   each `SubFieldSpec` when constructing the `SimProperty`, before calling `register`.
   `ReductionRule::Max` and `SubFieldSpec::reduction_override` both exist.

9. `SpecError` needs more variants for PR 2/3: at minimum `DuplicateProperty`,
   `OnPrereqMetAuthoredDefault`, `UnknownPrereqEntry`, `UnknownPrereqCategory`,
   `UnknownProperty`, `UnsupportedMaxActive`. Add as needed per PR.

10. `CapabilityTreeDefinitionId` type does not exist — needs to be defined in PR 3
    (likely a newtype wrapping `CapabilityTreeKey` or a `u32` index).

**Confirmed working (no surprises):**
- `OverlayId::new()` ✓ (atomic counter in `ids.rs`)
- `col_for_role` ✓ (method on `PropertyColumnRange` in `registry.rs`)
- `SubFieldRole::Named(String)` ✓
- `OverlayLifecycle::Suspended { when_activated: Box<OverlayLifecycle> }` ✓
- `ReductionRule::Max` ✓ (`reduction.rs`; `SubFieldSpec::reduction_override: Option<ReductionRule>`)
- `ThresholdSemantic` (5 arms; PR 4 adds `CapabilityUnlock`) ✓
- `CapabilityTreeKey`, `CategoryKey`, `CapabilityEntryKey`, `CapabilityEffectKey` ✓ (`keys.rs`)
- `SpecDiagnostics`, `SpecError`, `SpecResult<T>` ✓
- `simthing-feeder` has no `capability.rs` yet — PR 4 creates it ✓
- **212 tests passing**, 1 ignored, zero warnings ✓

### Performance and spec layer

- [x] **Priority 1 — activated overlay GPU integration test.** Landed
      2026-05-22. `activated_suspended_overlay_appears_in_gpu_delta_and_affects_values`
      in `crates/simthing-sim/tests/boundary_integration.rs`. Proves the full
      Suspended → Permanent transition: suspended overlay is GPU-inert (Pass 3
      filter), `BoundaryRequest::ActivateOverlay` flips lifecycle, boundary
      gpu_sync rebuilds Pass 3 deltas, next tick's Pass 3 applies the overlay
      to `values` (0.5 → 0.75 via Multiply(1.5)).
- [x] **Priority 2 — capability fission replay test.** Landed 2026-05-22.
      `replay_fission_with_cloned_capability_subtree_reconstructs_full_payload`
      in `crates/simthing-sim/tests/boundary_integration.rs`. Drives a faction
      fission with `clone_capability_children: true` and
      `capability_container_kinds: ["tech_tree"]`; verifies the
      `FissionOccurred { node }` payload carries the full cloned tech_tree
      subtree (2 nested levels), and `ReplayDriver` reconstructs the spawned
      faction, its cloned tech_tree, and the tech_tree's child, with slots
      allocated for every node and lineage round-tripped.
- [x] **Priority 3 — serde default for `clone_capability_children`.** Landed
      2026-05-22. `fission_template_deserializes_without_clone_capability_children`
      in `crates/simthing-core/src/property.rs`. Pre-V6 JSON/RON without the
      field deserializes as `false` (safe default — no capability cloning
      runs without explicit spec-layer opt-in).

### Performance and spec layer (V6 guardrails complete — B2 done)

- [x] **Priority 4 — B2 fission-growth Approach A (targeted value upload).**
      Landed 2026-05-22. `WorldGpuState::rebuild_for_slots` now preserves
      existing GPU contents via `copy_buffer_to_buffer` (values,
      previous_values, output_vectors, previous_output_vectors). Fission /
      AddChild / final-capacity pre-grow no longer force a full shadow
      flush. New `DispatchCoordinator::upload_row_range` coalesces
      contiguous dirty slots into single `queue.write_buffer` calls in
      `gpu_sync`. Regression guard:
      `fission_beyond_initial_headroom_grows_gpu_state` now asserts
      `!full_value_upload && value_rows_uploaded == 1` for a single
      fission across a growth boundary.
- [x] **Priority 4 — B2 Approach B (append-only threshold registry,
      2026-05-22).** `ThresholdBuilder::append_subtree` /
      `append_lineage` and `WorldGpuState::append_thresholds` push new
      registrations at the tail of the existing GPU buffer (preserving
      event_kind indices) when boundary mutations are limited to pure
      fission spawning. `boundary.rs` detects the eligible case (no
      fusions, no expiry, no add/remove, no dimension/config change)
      and skips the full tree walk. `fission_stress` `boundary_gpu_sync_ms`:
      ~7 → ~3.8 ms (~3 ms saved); upload bytes ~2.5 MB → ~1.0 MB;
      ms_per_sim_day unchanged (within noise on this machine).
      Regression guard:
      `fission_beyond_initial_headroom_grows_gpu_state` now asserts
      `threshold_regs_uploaded == 2` for a single fission (1 new
      FissionTrigger + 1 new FusionTrigger), proving the append path
      writes only deltas instead of rebuilding the registry.
- [x] **Priority 4 — B2 Approach C (incremental reduction topology,
      2026-05-22).** New `simthing-gpu::TopologyState` is the canonical
      source for the CSR `Topology`; `gpu_sync.rs` takes it by `&mut`
      so the full-rebuild path refreshes it and the append path
      (mirroring Approach B's eligibility predicate) patches it
      in-place via `add_child(parent_slot, child_slot)`. The
      `SlotAllocator`'s monotonically-increasing index guarantee
      makes the new child the highest slot in the world, so appending
      to the parent's child list preserves the ascending-slot
      invariant without re-sorting. Determinism safety verified by
      two new unit tests in `simthing-gpu::reduction`
      (`topology_state_flatten_matches_build_topology` and
      `topology_state_incremental_add_child_matches_full_rebuild`)
      that prove byte-identical CSR output AND bit-identical CPU
      oracle reduction. Integration test adds
      `reduction_edges == 3` and `reduction_depths == 4` assertions.
      `fission_stress` `boundary_gpu_sync_ms`: ~3.8 → ~2.0 ms.
- [ ] **Scenario format expansion.** Full RON tree/registry/shadow seeds —
      behind the GPU performance path.
- [ ] **Map-scale representation doc spike.** Evaluate sidecars only if
      benchmarks show tree-representation pressure.
- [ ] **`simthing-studio` designer GUI** — tabled; depends on `simthing-spec`.

---

## Notes

### Architecture boundaries (unchanged)

- Suspended overlays are CPU-visible and GPU-free until activated.
- Capability cloning is opt-in per `FissionTemplate` and defaults to `false`.
- Cohort/location fission is unaffected unless a template opts in.
- No WGSL shader changes were required for V6 or PR #38.

### `capability_container_kinds` contract (PR #38)

| Field | Role |
|---|---|
| `clone_capability_children: bool` | Gates whether fission runs the clone path at all. |
| `capability_container_kinds: Vec<String>` | Opaque `Custom(name)` labels to match against parent children. |

Studio/RON authors own the strings via **`simthing-spec`** (planned). Simulation
never interprets "tech tree" vs "national ideas" — it only compares `SimThingKind::Custom(name)`
to the template list. Modders add `"racial_abilities"` (or any label) in RON;
no simulation recompile.

**Faction fission RON example:**

```ron
FissionTemplate(
    child_kind:                 Faction,
    fusion_intensity_threshold: 0.8,
    fusion_scar_coefficient:    0.05,
    resolution_label:           "separatism",
    clone_capability_children:  true,
    capability_container_kinds: [
        "tech_tree",
        "national_ideas",
        "talent_tree",
        "racial_abilities",
    ],
)
```

### Doc references

- Simulation spec: `docs/design_v6.md` (incl. implementation addenda)
- Capability trees: `docs/capability_tree_v1.md` (incl. addendum §11)
- **Spec-layer handoff (canonical):** `docs/workshop/simthing_spec_progress_log.md`
- Workshop index: `docs/workshop/README.md`
- Historical worksheet: superseded; see `docs/workshop/simthing_spec_progress_log.md`
- Source workshop Q&A: `docs/workshop/capability_tree_studio_workshop.md`
- Historical workshop: `docs/workshop/tech_tree_decisions.md`
- Agent map: `docs/agents.md`

### Spec-layer dependency graph (PR 11 complete)

```text
simthing-core
    ↑
simthing-feeder   ← CapabilityUnlockRegistration, CapabilityUnlockEvent,
                    ScriptedEventTriggerRegistration, ScriptedEventTriggerEvent
    ↑         ↑
simthing-spec     simthing-sim   ← ThresholdSemantic, extract_*,
(production:      (production)     BoundaryHookContext, external threshold regs
 core + feeder
 only)
    ↑
simthing-driver   ← SpecSessionState, install_spec_state (wired)

simthing-studio   ← deferred GUI
```

### Recommended session order

1. ~~Priority 1 (activated overlay GPU proof)~~ — Done 2026-05-22, PR #39.
2. ~~Priority 2 (capability fission replay)~~ — Done 2026-05-22, PR #39.
3. ~~Priority 3 (`clone_capability_children` serde default)~~ — Done 2026-05-22, PR #39.
4. ~~Priority 4 — B2 Approach A (targeted value upload)~~ — Done 2026-05-22, PR #40.
5. ~~Priority 4 — B2 Approach B (append-only threshold registry)~~ — Done 2026-05-22, PR #41.
6. ~~Priority 4 — B2 Approach C (incremental reduction topology)~~ — Done 2026-05-22, PR #43.
7. ~~**PR 11 Track B** — mechanical prep~~ — Done PR #47, `392992f`.
8. ~~**PR 11 Track A** — session/driver assembly~~ — Done `01fb572`, parked `9e63718`.
9. ~~Composer Phase 0 + Phase 1 ADRs + O3~~ — Done through `c3f3556` (PRs #49–51).
10. ~~Composer S3 + S4~~ — topology full-rebuild guard; capability instance reverse map (PR #52, `7914528`).
11. ~~**O1** — RON-driven session installation~~ (PR #53, `6ba4e0d`). 320 tests.
12. ~~Post-O1 doc parking sync~~ (PR #54, `7eb015a`).
13. ~~Codex evaluation doc sync~~ (PR #55, `04867b1`).
14. ~~O1b E2E test (Cursor)~~ — landed **ignored/RED**; exposes overlay-id remapping gap.
15. **Next — Codex P0:** fix handler to resolve `ActivateOverlay` ids via `instance.by_overlay`; un-ignore O1b test.
16. **Next — Codex P1:** **S5/O5** — conservative Approach C eligibility fix +
     append-only external thresholds.
17. **Next — Codex P2:** **O4** (per-owner scripted events) then **O2** (replay v3).
18. **Next — Opus P3:** **EffectTarget ADR** — Owner vs CapabilityTree effect scope
     before Studio/modder docs.
19. ~~**Cursor:** InstallTargetSpec examples/docs PR~~ — `docs/examples/` + parse smoke test.
20. **Next — Cursor:** Built-in kind string + v0 effect-target docs PR; optional S5 regression test PR.
21. Scenario format expansion / map-scale representation — tabled.
