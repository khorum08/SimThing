# SimThing Todo Log

Current parking state after **B2 Approach A (targeted value upload, PR #40)
and B2 Approach B (append-only threshold registry)** — both landed on
2026-05-22 atop V6 guardrails Priorities 1–3 (PR #39).

**Tests:** `cargo test --workspace` → **202** passed, **1** ignored timing
diagnostic, zero warnings. `fission_stress` ~55 ms/sim-day with
`boundary_gpu_sync_ms` ~3.8 ms (down from ~7 ms pre-B); upload bytes
~1.0 MB (down from ~2.5 MB pre-B, ~2.7 MB pre-A).

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

---

## Next

### V6 guardrails (do before B2)

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
      runs without explicit studio opt-in).

### Performance and studio (V6 guardrails complete — clear path to B2)

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
- [ ] **Capability-tree studio layer.** `CapabilityTreeSpec` / builder /
      session init per `capability_tree_v1.md` and
      `workshop/tech_tree_decisions.md`. Studio populates
      `capability_container_kinds` on faction fission templates; simulation
      crates stay agnostic.
- [ ] **Scenario format expansion.** Full RON tree/registry/shadow seeds —
      behind the GPU performance path.
- [ ] **Map-scale representation doc spike.** Evaluate sidecars only if
      benchmarks show tree-representation pressure.
- [ ] **`simthing-studio` designer UI** — tabled.

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

Studio/RON authors own the strings. Simulation never interprets
"tech tree" vs "national ideas" — it only compares `SimThingKind::Custom(name)`
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
- Studio capability trees: `docs/capability_tree_v1.md` (incl. addendum §11)
- Workshop handoff: `docs/workshop/tech_tree_decisions.md`
- Agent map: `docs/agents.md`

### Recommended session order

1. ~~Priority 1 (activated overlay GPU proof)~~ — Done 2026-05-22, PR #39.
2. ~~Priority 2 (capability fission replay)~~ — Done 2026-05-22, PR #39.
3. ~~Priority 3 (`clone_capability_children` serde default)~~ — Done 2026-05-22, PR #39.
4. ~~Priority 4 — B2 Approach A (targeted value upload)~~ — Done 2026-05-22, PR #40.
5. ~~Priority 4 — B2 Approach B (append-only threshold registry)~~ — Done 2026-05-22, PR #41.
6. ~~Priority 4 — B2 Approach C (incremental reduction topology)~~ — Done 2026-05-22.
7. **Next session — pick one:**
   - Studio capability-tree builder — most gameplay-visible; exercises V6
     suspended-overlay path end-to-end. Per `docs/capability_tree_v1.md`.
   - `tick_event_readback_ms` deep dive — the single largest cost remaining
     in `fission_stress` (~21 ms / ~40% of total). GPU → CPU bandwidth-bound;
     async readback or ring-buffer schemes could be substantial.
   - Cache-invalidation hardening for `cached_topology_state` — current
     correctness relies on always taking the full-rebuild path on any
     non-fission-only mutation. A defensive integrity check (e.g.
     `debug_assert!` reflattening matches `build_topology` on every
     non-eligible boundary) would catch any future regressions early.
8. Scenario format expansion / map-scale representation doc — tabled until
   the above land.
