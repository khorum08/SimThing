# SimThing Todo Log

Current parking state after **PR #38** (`a8aab5b`) — parameterized
`FissionTemplate::capability_container_kinds` — merged to `master` on
2026-05-22. Prior context: V6 simulation core (`f39fe6d`), doc parking
(`95516b9`, `07076b4`), capability-tree concept docs (PR #37).

**Tests:** `cargo test --workspace` → **199** passed, **1** ignored timing
diagnostic, zero warnings.

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

- [ ] **Priority 1 — activated overlay GPU integration test.** Prove
      `ActivateOverlay` puts a formerly suspended overlay into the next Pass 3
      delta upload and affects `values` on the following tick (not just CPU
      tree / observability state).
- [ ] **Priority 2 — capability fission replay test.** End-to-end replay for
      fission with `clone_capability_children: true` and a populated
      `capability_container_kinds` list; verify `FissionOccurred { node }`
      reconstructs the full spawned subtree including cloned capability children.
- [ ] **Priority 3 — serde default for `clone_capability_children`.** Add test
      proving old `FissionTemplate` JSON/RON without `clone_capability_children`
      deserializes as `false`. *(Partial: `capability_container_kinds` default
      test landed in PR #38; `clone_capability_children` default test still open.)*

### Performance and studio (after guardrails)

- [ ] **Priority 4 — B2 fission-growth topology batching.** Retain or
      append-patch GPU topology/threshold buffers on growth boundaries only when
      slot ordering and `event_kind` semantics remain provably correct.
      `fission_stress` ~60 ms/sim-day locally after B1/B2 partial work.
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

1. Priority 1 (activated overlay GPU proof)
2. Priority 2 (capability fission replay)
3. Finish Priority 3 (`clone_capability_children` serde default)
4. Priority 4 (B2 growth batching)
5. Studio capability-tree builder
