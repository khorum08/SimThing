# Opus/Product Acceptance Review — Phase M First-Slice Vertical Proof

**Date:** 2026-05-28
**Authority:** Opus 4.8, mapping/SEAD design authority under human delegation (authority to raise
guardrails up to the Designer-facing / RON / Scenario layer).
**Decision type:** Acceptance review — **not** an implementation handoff. No code changed.
**Reviews:** PR #240 parking (`docs/tests/phase_m_first_slice_vertical_proof_parking_test_results.md`)
and packet `docs/reviews/phase_m_first_slice_vertical_proof_review_packet.md`.
**Builds on:** `docs/reviews/m4_m4a_first_slice_oversight_opus_review.md` (M-4A ratified; R1/R2/R3 accepted).

---

## 1. Executive verdict

**PASS WITH CONDITIONS.** The Phase M first-slice vertical proof is **accepted as complete for the
single-grid, opt-in SEAD path.** It demonstrates the full intended vertical slice end-to-end:

```text
RON scenario/spec authoring (FirstSliceScenarioSpec / RegionFieldSpec / FirstSliceCommitmentSpec)
→ explicit MappingExecutionProfile opt-in
→ GPU-resident field propagation (StructuredFieldStencilOp)
→ GPU-resident parent reduction (SlotRange Sum)
→ field_urgency EvalEML
→ Threshold + EmitEvent commitment
```

Acceptance is conditioned on the prohibitions below remaining enforced and the queue-write scale
caveat being resolved before any scaling. Atlas, default SimSession wiring, perception,
`source_mask`, active masks, and map residency remain **separately gated and not authorized** by
this acceptance.

---

## 2. Evidence reviewed

**Packet + parking report** (read in full). **Spot-checked** R2 GPU-bridge, scenario-spec R1
hygiene, product commitment fixture, and commitment-spec results.

**Code surfaces verified directly (not taken on the reports' word):**
- `crates/simthing-spec/src/spec/first_slice_scenario.rs` — `FirstSliceScenarioSpec` is narrow:
  `{ name, mapping_execution_profile, region_field }`. Not a general scenario engine.
- `crates/simthing-spec/src/compile/first_slice_scenario_admission.rs` — compiles to
  `CompiledFirstSliceScenarioPreview` (a preview, not a runtime command object); budget estimated
  with `SingleGridNoAtlas`; budget failure now propagates as `SpecError` (R1 hygiene), not `.ok()`.
- `crates/simthing-spec/src/spec/region_field.rs` — `MappingExecutionProfile::default() = Disabled`;
  `enables_execution()` true only for `SparseRegionFieldV1`. `FirstSliceCommitmentSpec` is bounded
  (Upward-only direction; explicit threshold/event_kind/cols).
- `crates/simthing-spec/src/compile/region_field_admission.rs` `compile_commitment()` — **rejects
  non-finite threshold, zero event_kind, non-Upward direction, missing reduction, missing/wrong
  parent_formula class, mismatched parent_slot, and out-of-range/non-canonical urgency_col.** This
  is the designer/RON-layer guardrail that stops bad commitment data before the GPU.
- `crates/simthing-driver/src/first_slice_mapping_runtime.rs` — `open_from_scenario_preview`
  delegates to `open_preview_with_budget` with the preview's profile (Disabled ⇒ inert tick).
  `scan_commitment_threshold` uses the **existing** `AccumulatorOpSession` Threshold + EmitEvent
  substrate (`ThresholdRegistration`, `DIR_UPWARD`, `event_kind`) — no new EML opcode, no CPU
  planner; CPU readback only inspects the emitted event.
- `request_atlas_batching` remains rejected at `compile_region_field_preview` (re-verified).

**Independent verification run (this review, real GPU, this machine):**

| Suite | Result |
|---|---|
| `phase_m_first_slice_scenario_spec` | **9/9** (incl. `disabled_scenario_admits_but_does_not_execute`) |
| `phase_m_first_slice_product_commitment_fixture` | **7/7** (incl. deterministic high-urgency event) |
| `region_field_spec_admission` | **11/11** (incl. `test_j_first_slice_compile_preview_only`) |

The parking report additionally records full-workspace green (`cargo test --workspace -j 1`, ~280s)
and the layered runtime/product/bridge suites (28/28, 7/7, 2/2).

---

## 3. Acceptance decision (answers to the five acceptance questions)

1. **Vertical proof acceptance — ACCEPT WITH CONDITIONS.** The landed chain proves the intended
   slice end-to-end. Conditions: keep the prohibitions in §5/§7 enforced; resolve the queue-write
   scale caveat before scaling (Q4). Diagnostic readback stays test-only; the hot path keeps
   `reduction_stencil_readbacks == 0`.

2. **SEAD discipline — PASS.** There is no CPU planner. The commitment is a GPU `Threshold`
   crossing over a parent SimThing column; `field_urgency` EvalEML interprets parent pressure; the
   commitment signal is an `EmitEvent` from the existing substrate. Measured: low-weight profile
   emits 0 events, high-weight profile emits 1 (event_kind `0x53454144`). The decision is the GPU
   threshold crossing, not `if urgency > t` CPU logic.

3. **Boundary discipline — ACCEPT WITH WATCHLIST.** Spec/driver/gpu/sim separation is clean and
   `simthing-sim` is map-free. Watchlist (carried forward + one addition) in §5.

4. **Known caveat — YES (acceptable for the parked 10×10 proof) WITH CONDITION.** Per-slot queue
   writes for child resource values and parent weights are fine at 10×10 and are now *reported*
   (R3 counters). **Condition:** before multi-field, multi-map, atlas, or broader production
   scaling, replace per-slot queue writes with a measured GPU-resident mechanism (preinitialized
   resource column / generic fill helper / GPU fill kernel / other measured approach).

5. **Next step — A first, then C or B; not E.** Accept and park the vertical proof as complete
   (A). The next *implementation* handoff should be **map residency / summary validity (C)** as the
   next substrate slice, with **queue-write scale hardening (B)** as a hard prerequisite that must
   precede any multi-field/atlas scaling regardless of ordering. **Do not start the M-4 atlas
   packer (E).**

---

## 4. Conditions

- **C-1 (scaling gate):** The queue-write caveat (Q4) must be resolved by a measured GPU-resident
  mechanism **before** any multi-field/multi-map/atlas/scaling work. It is not a blocker for the
  parked proof or for a single-grid gameplay wrapper.
- **C-2 (prohibitions hold):** All §5/§7 prohibitions remain enforced; none may be relaxed except
  through its own separately-gated decision.
- **C-3 (admission stays authoritative):** The commitment/region/scenario admission checks remain
  the first line; `request_atlas_batching` stays rejected until a §11-gate-passing M-4 PR.

---

## 5. Boundary / watchlist items

Carried forward from the packet (all still valid):
- `FirstSliceScenarioSpec` must remain narrow — **not** a general scenario engine.
- `CompiledFirstSliceScenarioPreview` must **not** become a general runtime command object.
- `MappingExecutionProfile::enables_execution` must **not** be used to wire default `SimSession`.
- Test-only helpers must stay in `tests/support` (R1 hygiene already moved
  `FirstSliceScenarioFixtureSession` there — resolved).

**Added by this review:**
- **W-1 (commitment fixture surface):** `tick_with_commitment_threshold_fixture` /
  `tick_with_commitment_spec_fixture` / `scan_commitment_threshold` live on the production
  `FirstSliceMappingSession` (named `*_fixture`, opt-in, gated on `enabled && scheduled &&
  eml_executed`). Acceptable for the proof. Before the commitment path is used by any non-test
  caller, either promote it to a deliberate bounded commitment API or move the fixture-shaped
  scan behind a clearer test/production boundary. Do not let `*_fixture` methods accrete
  general-purpose runtime responsibilities.

---

## 6. Recommended next implementation handoff

**Primary: Option C — Map residency / summary validity.** Define how a skipped/dirty/cadenced
RegionField still exposes a valid strategic summary (fresh / cached / decayed / stale-with-
confidence / zero-if-empty) so hierarchy + parent EML stay informed without running every dense
local map every tick. This is the natural next substrate slice and complements (does not duplicate)
dirty/cadence skipping. Generic, bounded, opt-in, designer/RON-governed, semantic-free,
oracle-testable — same bar as everything else.

**Alternative / prerequisite: Option B — Queue-write scale hardening.** A measured design step to
replace per-slot child resource writes. Sequence this first if product intends to scale (multi-
field/atlas) sooner than it needs residency; it is a hard gate before that scaling either way.

**Optional: Option D — single-grid gameplay wrapper.** Only if product wants a narrative wrapper;
must stay single-grid and opt-in, reusing the landed proof.

**Not now: Option E — M-4 atlas packer.** Blocked until a named multi-theater scenario, an approved
VRAM budget, and a §11-gate-passing M-4 implementation PR all exist.

---

## 7. Stop conditions for the next handoff (escalate; do not land)

Whichever of C/B/D is taken next must not introduce any of:
- semantic / map / faction / AI WGSL, or `simthing-sim` map awareness;
- default-on mapping execution or default `SimSession` pass-graph wiring;
- atlas batching, M-4A atlas masking in production, or relaxing the `request_atlas_batching`
  admission rejection;
- active mask / halo, perception / fog, or `source_mask` / behavioral source policy (pre-M-5);
- a new EML opcode, a CPU-side AI planner, or column-wide `source_col` zeroing;
- Resource Flow default changes.

For **C (map residency)** specifically: a skipped map's summary must be a bounded, deterministic,
designer-governed value; it must **not** become a CPU-side cache that re-derives gameplay state, and
it must not write back into authoritative true fields except via the existing event path.

For **B (queue-write hardening)**: the replacement must be a generic GPU substrate mechanism with
CPU-oracle parity; it must not embed mapping semantics in the fill path.

---

## 8. Doc / ADR updates made alongside this memo

- **New:** this memo (`docs/reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md`).
- **`docs/reviews/phase_m_first_slice_vertical_proof_review_packet.md`** — status flipped from
  "Parked for acceptance review" to **Accepted (PASS WITH CONDITIONS)**; recommended-next updated
  to "C or B, not E."
- **`docs/adr/mapping_sparse_regioncell.md`** — first-slice section gains a 2026-05-28 acceptance
  note recording the full vertical SEAD slice (RON → GPU → Threshold+EmitEvent) accepted for the
  single-grid opt-in path.
- **`docs/workshop/mapping_current_guidance.md`**, **`docs/workshop/workshop_current_state.md`**,
  **`docs/accumulator_op_v2_production_plan.md`**, **`docs/todo.md`** — next-step updated to
  accepted + "C or B next, not atlas."
- **`docs/worklog.md`** — dated 2026-05-28 acceptance entry appended.

All updates are decision/classification only. No production code changed; `MappingExecutionProfile`
default remains `Disabled`; `simthing-sim` remains map-free; `request_atlas_batching` stays rejected.
