# Phase M First-Slice Vertical Proof — Review / Parking Packet

> **Audience:** Opus / product review  
> **Status:** **ACCEPTED — PASS WITH CONDITIONS** (Opus 2026-05-28). Acceptance memo:
> [`phase_m_first_slice_vertical_proof_acceptance_opus_review.md`](phase_m_first_slice_vertical_proof_acceptance_opus_review.md).
> Accepted as complete for the single-grid opt-in FIELD_POLICY path; atlas / default wiring / perception /
> `source_mask` / map residency remain separately gated. Queue-write scale caveat must be resolved
> before scaling.  
> **Date:** 2026-05-28  
> **Master baseline at parking:** `29e95246865cea4c8672c19f2aa5c72c3b18e7b7`

---

## 1. Executive verdict

**ACCEPTED — PASS WITH CONDITIONS (Opus 2026-05-28).** Phase M first-slice vertical proof is
accepted as complete for the single-grid, opt-in, GPU-resident mapping + FIELD_POLICY commitment path. See
the acceptance memo for verdicts, conditions, and watchlist:
[`phase_m_first_slice_vertical_proof_acceptance_opus_review.md`](phase_m_first_slice_vertical_proof_acceptance_opus_review.md).

It proves a single-grid, opt-in, GPU-resident mapping + FIELD_POLICY commitment path.

It does **not** authorize atlas, perception, map residency, source identity, active masks, or default SimSession wiring.

The landed chain now covers scenario-level RON authoring with explicit `MappingExecutionProfile`, `RegionFieldSpec`, `FirstSliceCommitmentSpec`, GPU-resident field propagation, parent reduction, `field_urgency` EvalEML, and Threshold + EmitEvent commitment. No additional runtime behavior is required for this parking pass.

**Related prior review:** [`m4_m4a_first_slice_oversight_opus_review.md`](m4_m4a_first_slice_oversight_opus_review.md) (M-4A isolation policy ratification; first-slice R1/R2/R3 accepted as stable base).

---

## 2. Landed chain

```text
FirstSliceScenarioSpec RON
  mapping_execution_profile: Disabled | SparseRegionFieldV1
  region_field: RegionFieldSpec
    commitment: FirstSliceCommitmentSpec

compile_first_slice_scenario_preview
  → CompiledFirstSliceScenarioPreview

FirstSliceMappingSession::open_from_scenario_preview
  → GPU-resident first-slice runtime

hot path:
  StructuredFieldStencilOp
  → AccumulatorOpSession values buffer bridge
  → SlotRange Sum
  → field_urgency EvalEML
  → Threshold + EmitEvent
```

Earlier layers in the same vertical proof (still active, not superseded):

```text
RegionFieldSpec RON
  → compile_region_field_preview
  → explicit MappingExecutionProfile in tests / scenario wrapper
  → FirstSliceMappingSession
```

---

## 3. Evidence table

| Test report | Purpose | Core result | Status |
|---|---|---|---|
| [`phase_m_first_slice_runtime_test_results.md`](../tests/phase_m_first_slice_runtime_test_results.md) | Initial opt-in first-slice runtime wiring (M-first-slice) | 10×10 SourceCappedNormalized H≤8; scheduler dirty skip; Sum + field_urgency; default Disabled; atlas rejected | **PASS** |
| [`phase_m_first_slice_runtime_r1_no_readback_correctness_test_results.md`](../tests/phase_m_first_slice_runtime_r1_no_readback_correctness_test_results.md) | GPU-state ownership / caller-managed seed protocol on no-readback hot path | Hot path honest reports; seed-only clear; no hidden CPU readback in stencil path | **PASS** |
| [`phase_m_first_slice_runtime_r2_gpu_bridge_test_results.md`](../tests/phase_m_first_slice_runtime_r2_gpu_bridge_test_results.md) | GPU-resident Layer 1→2→3 bridge | Stencil → AccumulatorOp values bridge without GPU→CPU→GPU staging; `reduction_stencil_readbacks=0` | **PASS** |
| [`phase_m_first_slice_runtime_r3_readiness_test_results.md`](../tests/phase_m_first_slice_runtime_r3_readiness_test_results.md) | Readiness / observability parking (R3) | `FirstSliceReadinessReport`; bridge cost counters; hot-path invariant locked | **PASS** |
| [`phase_m_first_slice_product_fixture_test_results.md`](../tests/phase_m_first_slice_product_fixture_test_results.md) | Product-facing RegionFieldSpec/RON fixture | Explicit SparseRegionFieldV1 opt-in; finite propagation; weight-sensitive urgency; hot path no readback | **PASS** |
| [`phase_m_first_slice_product_commitment_fixture_test_results.md`](../tests/phase_m_first_slice_product_commitment_fixture_test_results.md) | Threshold + EmitEvent over parent urgency | Low urgency no event; high urgency one event; GPU threshold crossing (not CPU planner) | **PASS** |
| [`phase_m_first_slice_commitment_spec_test_results.md`](../tests/phase_m_first_slice_commitment_spec_test_results.md) | Designer-facing `FirstSliceCommitmentSpec` RON admission | Commitment binding admitted on RegionFieldSpec; same GPU-resident FIELD_POLICY path | **PASS** |
| [`phase_m_first_slice_scenario_spec_test_results.md`](../tests/phase_m_first_slice_scenario_spec_test_results.md) | Scenario-level RON wrapper with explicit execution profile | Disabled admits without execute; SparseRegionFieldV1 full hot path + commitment from same preview | **PASS** |
| [`phase_m_first_slice_scenario_spec_r1_hygiene_test_results.md`](../tests/phase_m_first_slice_scenario_spec_r1_hygiene_test_results.md) | Post-landing hygiene | Test-only helper moved out of production API; budget estimate errors propagate; crash history documented | **PASS** |
| [`phase_m_first_slice_vertical_proof_parking_test_results.md`](../tests/phase_m_first_slice_vertical_proof_parking_test_results.md) | This parking pass verification | Targeted first-slice suite + workspace check green; review packet created | **PASS** |

---

## 4. Code surfaces

### Spec / admission (production)

| Path | Role |
|---|---|
| [`crates/simthing-spec/src/spec/region_field.rs`](../../crates/simthing-spec/src/spec/region_field.rs) | `RegionFieldSpec`, `FirstSliceCommitmentSpec`, `MappingExecutionProfile` |
| [`crates/simthing-spec/src/spec/first_slice_scenario.rs`](../../crates/simthing-spec/src/spec/first_slice_scenario.rs) | `FirstSliceScenarioSpec` scenario wrapper |
| [`crates/simthing-spec/src/compile/region_field_admission.rs`](../../crates/simthing-spec/src/compile/region_field_admission.rs) | Region field admission + compile preview |
| [`crates/simthing-spec/src/compile/first_slice_scenario_admission.rs`](../../crates/simthing-spec/src/compile/first_slice_scenario_admission.rs) | Scenario compile preview + budget estimate |

### Runtime (production, opt-in)

| Path | Role |
|---|---|
| [`crates/simthing-driver/src/first_slice_mapping_runtime.rs`](../../crates/simthing-driver/src/first_slice_mapping_runtime.rs) | `FirstSliceMappingSession`, GPU-resident hot path, commitment fixture helpers |

**Not production API:** [`crates/simthing-driver/tests/support/first_slice_scenario_fixture.rs`](../../crates/simthing-driver/tests/support/first_slice_scenario_fixture.rs) — acceptance-test helper (`FirstSliceScenarioFixtureSession`) only; opens session + commitment from the same admitted scenario preview.

### Acceptance tests

| Path | Role |
|---|---|
| [`crates/simthing-driver/tests/phase_m_first_slice_scenario_spec.rs`](../../crates/simthing-driver/tests/phase_m_first_slice_scenario_spec.rs) | End-to-end scenario RON → GPU path → commitment event |
| Plus: `phase_m_first_slice_runtime.rs`, product/commitment fixture tests, `region_field_spec_admission.rs` | Layered evidence for runtime, product, commitment, spec admission |

---

## 5. GPU-resident execution path

1. **Opt-in gate:** `MappingExecutionProfile::SparseRegionFieldV1` required; default remains `Disabled`.
2. **Layer 1 — field propagation:** `StructuredFieldStencilOp` executes `source_capped_normalized` on a single 10×10 grid (H≤8), caller-managed one-shot seed then zero, dirty scheduling via `FieldScheduler`.
3. **Bridge:** GPU copy from stencil values into `AccumulatorOpSession` values buffer; queue writes for child resource columns and parent EML weights (known scale caveat).
4. **Layer 2 — reduction:** `SlotRange Sum` over child slots into parent threat column.
5. **Layer 3 — formula:** `field_urgency` EvalEML on parent slot.
6. **Commitment:** Existing AccumulatorOp Threshold + EmitEvent scan over parent urgency column; event readback for tests only.

**Hot-path invariant:** `reduction_stencil_readbacks == 0`; tick report returns `field_values`, `reduction_parent_value`, and `eml_output` as `None` on the production-shaped hot path.

---

## 6. Designer / spec authoring path

1. **RegionFieldSpec RON** — grid, operator, horizon, reduction binding, optional `parent_formula`, optional `FirstSliceCommitmentSpec`, optional VRAM budget cap.
2. **FirstSliceScenarioSpec RON** — scenario name, explicit `mapping_execution_profile`, nested `region_field`.
3. **Admission** — `compile_region_field_preview` / `compile_first_slice_scenario_preview`; `request_atlas_batching: true` rejected.
4. **Runtime open** — `FirstSliceMappingSession::open_from_scenario_preview` (production) or test helper from same preview.
5. **Execution** — seed queue → tick → optional commitment threshold scan using commitment binding from the **same** admitted preview (no orphan external threshold).

---

## 7. FIELD_POLICY commitment path

Measured product signals (10×10 fixture, weights `(0.2, 0.1)` vs `(0.9, 0.1)`):

| Signal | Low profile | High profile | Threshold |
|---|---:|---:|---:|
| Parent threat (reduction) | ~9965 | ~9965 | — |
| Parent urgency (EML) | ~2003 | ~8979 | 5490.87 |
| Threshold events | 0 | 1 | event_kind `0x53454144` (FIELD_POLICY) |

The commitment decision is the GPU threshold crossing over parent urgency, not CPU-side `if urgency > threshold` planner logic.

---

## 8. Default-off / opt-in safety posture

| Guard | Status |
|---|---|
| `MappingExecutionProfile::default()` | `Disabled` |
| Spec / scenario presence alone executes mapping | **No** |
| Disabled profile: admit structure, zero dispatches | **Yes** |
| Default `SimSession` pass-graph wiring | **None** |
| `simthing-sim` map awareness | **None** |
| `PipelineFlags::default().use_accumulator_resource_flow` | **false** (unchanged) |
| `request_atlas_batching` at admission | **Rejected** |
| CPU-side AI planner | **None** |
| New EML opcode for commitment | **None** |
| Semantic / map / faction WGSL | **None** |

---

## 9. M-4 atlas boundary

- **M-4A isolation policy:** Ratified (algebraic G=0 preferred; physical gutter fallback). Policy only — **not** implementation authorization.
- **Atlas batching:** Provisional and **unimplemented**. `request_atlas_batching` remains rejected at admission until a §11-gate-passing M-4 PR with named multi-theater scenario and approved VRAM budget.
- **This vertical proof:** Single grid, no atlas, no M-4A production masking, no active mask / halo.

Design reference: [`workshop/mapping_atlas_batching_isolation_design_note.md`](../workshop/mapping_atlas_batching_isolation_design_note.md).

---

## 10. What this proves

- Spec presence alone does not execute mapping.
- Explicit `SparseRegionFieldV1` profile executes the first-slice mapping path.
- Disabled profile admits as structure but dispatches zero work.
- Field propagation is GPU-resident.
- Stencil → reduction → EML bridge is GPU-resident.
- Commitment event is emitted by existing Threshold + EmitEvent substrate.
- Low-weight profile emits no event.
- High-weight profile emits one event.
- Hot path preserves `reduction_stencil_readbacks == 0`.
- Diagnostic readback is test/reporting only.
- `request_atlas_batching` remains rejected.
- `simthing-sim` remains map-free.

---

## 11. What this does not prove

- Does not prove atlas batching runtime.
- Does not prove M-4A algebraic atlas masking in production.
- Does not prove multi-map / multi-theater scaling.
- Does not prove perception / fog / deception fields.
- Does not prove map residency or dirty-summary caching.
- Does not prove source identity / behavioral source policy.
- Does not prove active mask / halo.
- Does not prove default SimSession integration.
- Does not solve the queue-write scale caveat.

---

## 12. Known caveat

First-slice bridge uses queue writes for child resource values and parent weights. This is acceptable for the 10×10 first-slice scenario fixture. Future multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized resource column, fill helper, or GPU fill kernel after a separate measured design step.

---

## 13. Recommended next decision options

| Option | Description | Recommendation |
|---|---|---|
| **A — Opus/product acceptance** | Review and accept Phase M first-slice vertical proof as complete | **Do first** |
| **B — Product scenario expansion** | One tiny gameplay-facing scenario around existing proof; still single-grid and opt-in | After A, if product wants narrative wrapper |
| **C — Map residency / summary validity** | Cached summary, dirty/residual/cadence skip policy so skipped maps remain strategically visible | After A; candidate next substrate slice |
| **D — Queue-write scale hardening** | Replace per-slot child resource writes with preinitialized resource column, fill helper, or GPU fill kernel | After A; measured design step before multi-field/atlas |
| **E — M-4 atlas packer** | Deferred until named multi-theater scenario, approved VRAM budget, §11-gate-passing implementation PR | **Do not start yet** |

**Recommended order:** A first. Then either C or D. Do not start E yet.

**Decision (Opus 2026-05-28):** **A done** — accepted (PASS WITH CONDITIONS). Next implementation
handoff is **C (map residency / summary validity)** as the next substrate slice, with **D
(queue-write scale hardening)** as a hard prerequisite before any multi-field/atlas scaling. **E
(atlas packer) is not next.** (Note: this packet's option letters differ from the acceptance memo's
A–E list, where queue-write hardening is option B; the substance is identical — map residency or
queue-write hardening next, not atlas.)

---

## 14. Parking posture statement

Phase M first-slice vertical proof parked for Opus/product review.

The landed chain now covers scenario-level RON authoring with explicit MappingExecutionProfile, RegionFieldSpec, CommitmentSpec, GPU-resident field propagation, parent reduction, field_urgency EvalEML, and Threshold + EmitEvent commitment.

No additional runtime behavior landed in this parking pass.

No default SimSession wiring was introduced.

No CPU-side AI planner was introduced.

No atlas batching landed.

No M-4A atlas masking landed.

No active mask, perception, map residency, behavioral source policy, or source_mask landed.

No semantic WGSL landed.

simthing-sim remains map-free.

Defaults unchanged.
