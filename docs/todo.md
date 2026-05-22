# SimThing Todo Log

Current parking state after **B2 fission-growth A/B/C complete** and
**`simthing-spec` PR 1 lane stabilized** — PR #46 (`7eb48dc`) reverted the
exploratory PR #45 vertical slice; simulation crates unchanged except the
revert itself.

**Tests:** `cargo test --workspace` → **212** passed, **1** ignored timing
diagnostic, zero warnings. `fission_stress` ~55 ms/sim-day with
`boundary_gpu_sync_ms` ~2.0 ms (B2 A/B/C complete).

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
- [ ] **PR 3** — `CapabilityTreeBuilder` (`compile/capability.rs`,
      `runtime/capability_definition.rs`, `runtime/capability_state.rs`).
      Stub `CapabilityUnlockRegistration` locally; PR 4 replaces it.
- [ ] **PR 4** — capability unlock registration bridge: add
      `CapabilityUnlockRegistration` to `simthing-feeder/src/capability.rs`;
      add `ThresholdSemantic::CapabilityUnlock` + `build_with_capability_unlocks`
      to `simthing-sim`; add `simthing-feeder` dep to `simthing-spec`.
- [ ] **PR 5** — capability runtime state + boundary handler
      (`boundary/capability_handler.rs`). Called by session coordinator,
      not embedded in `BoundaryProtocol`.
- [ ] **PR 6** — preview + mutual exclusivity completion (`preview/capability_preview.rs`).

**Known divergences between handoff doc and PR 1 code (Opus must resolve):**

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
- **Spec-layer handoff (canonical):** `docs/workshop/simthing_spec_workshop.md`
- Source workshop Q&A: `docs/workshop/capability_tree_studio_workshop.md`
- Historical workshop: `docs/workshop/tech_tree_decisions.md`
- Agent map: `docs/agents.md`

### Spec-layer dependency graph (approved; PR 1 actual vs planned)

```text
simthing-core
    ↑
simthing-spec   ← PR 1: simthing-core only (authoring structs + RON load)
    ↑
simthing-driver (may depend on simthing-spec for session assembly — later)

Planned later:
  simthing-feeder   ← CapabilityUnlockRegistration (PR 4)
  simthing-sim      ← ThresholdSemantic::CapabilityUnlock (PR 4)

simthing-studio   ← deferred GUI; depends on simthing-spec
```

### Recommended session order

1. ~~Priority 1 (activated overlay GPU proof)~~ — Done 2026-05-22, PR #39.
2. ~~Priority 2 (capability fission replay)~~ — Done 2026-05-22, PR #39.
3. ~~Priority 3 (`clone_capability_children` serde default)~~ — Done 2026-05-22, PR #39.
4. ~~Priority 4 — B2 Approach A (targeted value upload)~~ — Done 2026-05-22, PR #40.
5. ~~Priority 4 — B2 Approach B (append-only threshold registry)~~ — Done 2026-05-22, PR #41.
6. ~~Priority 4 — B2 Approach C (incremental reduction topology)~~ — Done 2026-05-22, PR #43.
7. **Next session — primary track:** **`simthing-spec` PR 2** (property + overlay
     spec compiler only). PR 1 lane is stable (authoring structs + RON load only).
     Do not implement PRs 3–6 or driver session wiring until PR 2 lands.
   - **Alternate tracks** (parallel, not blocking spec work):
   - **`tick_event_readback_ms` deep dive** — Opus for architecture; Sonnet for impl.
   - **Cache-integrity hardening for `cached_topology_state`** — Sonnet.
8. Scenario format expansion / map-scale representation doc — tabled until
   the above land.
