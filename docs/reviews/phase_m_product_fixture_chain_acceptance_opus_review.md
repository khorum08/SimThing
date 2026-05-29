# Opus/Product Acceptance Review — Phase M Product-Fixture Chain

**Date:** 2026-05-29
**Authority:** Opus 4.8, mapping/SEAD design authority under human delegation. Guardrail authority
for this review extends up to the **designer-facing studio/importer layer and the scenario
definition stage** — **not** the sim or boundary layer.
**Decision type:** Acceptance review — **not** an implementation handoff. No code changed.
**Reviews:** packet `docs/reviews/phase_m_product_fixture_chain_review_packet.md`; parking report
`docs/tests/phase_m_product_fixture_chain_parking_test_results.md` and the cited chain reports.
**Builds on:** the boundary/economy acceptance
(`phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md`) and the first-slice
vertical proof acceptance (`phase_m_first_slice_vertical_proof_acceptance_opus_review.md`).

---

## 1. Executive verdict

**PASS WITH CONDITIONS.** The Phase M product-fixture chain is **accepted as a fixture-level
product proof**:

```text
Abstract tick/boundary doctrine
  → Daily Economy Fixture V1 (discrete ResourceEconomySpec)
  → Resource Economy Authoring Ergonomics V1 (preview/diagnostics)
  → Economy + SEAD Product Fixture V1 (Option A orchestration)
```

A discrete `ResourceEconomySpec` boundary result can **influence** an opt-in first-slice SEAD
commitment fixture by selecting **authored, pre-proven** EML weight profiles, while the SEAD urgency
and the commitment event still **emerge through the existing GPU-resident
field → reduction → `field_urgency` EvalEML → Threshold + EmitEvent path.** This is **fixture
orchestration only**. It authorizes no production economy→mapping runtime bridge, no generic
boundary-output packet, no `DailyResolutionBoundary`, no general scenario engine, no atlas, no
default SimSession mapping wiring, no CPU planner, no semantic WGSL, and no Resource Flow default-on.

---

## 2. Evidence reviewed

Packet + parking report + the nine cited chain reports read. **Verified in code, not taken on the
reports' word:**

- **The economy→SEAD link is fixture-only and disciplined.**
  `crates/simthing-driver/tests/support/economy_sead_product_fixture.rs` (`#![allow(dead_code)]`,
  `tests/support`, **not exported** from `simthing-driver`): the sole CPU "decision" is
  `eml_weights_from_treasury_stress(treasury)` → it picks between **two pre-authored, pre-proven
  weight profiles** `(0.2,0.1)` / `(0.9,0.1)` by comparing resolved treasury to a threshold. It does
  **not** compute urgency and does **not** emit the commitment.
- **Urgency and commitment stay GPU-resident.** `run_sead_commitment_with_economy_weights` passes the
  selected weights into `tick_with_scenario_commitment` and asserts `reduction_executed`,
  `eml_executed`, and `reduction_stencil_readbacks == 0`. Threat/urgency are obtained via
  `diagnostic_readback_reduction_eml` (GPU readback for verification), not CPU computation; the SEAD
  event (`0x53454144`) comes from the GPU Threshold + EmitEvent scan.
- **Designer/importer-layer guardrail is real (the layer my authority covers).**
  `crates/simthing-spec/src/compile/resource_economy_admission.rs` compiles economy specs through
  `compile_resource_economy` (conservation-exact transfers, non-zero throttle, property/role
  resolution) and rejects malformed input as `SpecError::ResourceEconomyAdmission` at the
  spec/importer stage — before any runtime. Resource-Flow-enabled visibility is surfaced in the
  preview; E-11 stays default-off.
- **Boundary doctrine intact.** Legible `tick`/`boundary`/`day`/`day_index`/`ticks_per_day` names;
  no `DailyResolutionBoundary` (regression-guarded); discrete `ResourceEconomySpec` distinct from
  continuous Resource Flow E-11.

**Independent verification run (this review, real GPU, this machine):**

| Suite | Result |
|---|---|
| `phase_m_economy_sead_product_fixture` | **6/6** (incl. `economy_sead_product_fixture_is_deterministic`) |
| `phase_m_resource_economy_authoring_ergonomics` | **4/4** |
| `resource_economy_authoring_preview` (spec) | **8/8** |

The parking report records the broader chain green (daily economy 7/7, boundary cadence 7/7,
first-slice runtime 28/28, admission 11/11, GPU bridge 3/3, `cargo check --workspace`).

---

## 3. Acceptance decision (answers to the five questions)

1. **Product-fixture chain — ACCEPT WITH CONDITIONS.** The chain is a sound fixture-level product
   proof: resolved economy boundary → authored fixture weight selection → GPU-resident
   field/reduction/`field_urgency` EvalEML → Threshold + EmitEvent commitment.
2. **Fixture orchestration boundary — ACCEPT.** It is acceptable that the economy→SEAD connection
   exists only in `tests/support` orchestration. The CPU reads resolved treasury at the boundary and
   selects an authored weight profile; it does **not** compute SEAD urgency and does **not** emit the
   commitment. The production economy→mapping bridge remains unauthorized.
3. **SEAD discipline — PASS.** No CPU planner; no CPU urgency computation; no CPU commitment
   emission. The commitment is a GPU Threshold + EmitEvent crossing over the parent urgency column;
   the GPU-resident first-slice path remains the source of urgency.
4. **Boundary/economy doctrine — PASS.** Legible names retained; day/calendar meaning stays
   host/spec interpretation; `ResourceEconomySpec` is the discrete boundary-banking example;
   Resource Flow E-11 stays continuous/high-frequency, separately opt-in, default-off.
5. **Binding non-authorizations — ACCEPT (kept binding).** See §5.

---

## 4. Conditions

- **C-1 (fixture-only link).** The economy→SEAD connection must remain in `tests/support` fixture
  orchestration. A production economy→mapping runtime bridge is a **separate, explicitly-gated
  decision** and is **not** authorized by this acceptance.
- **C-2 (CPU role is select-not-compute).** At the boundary the CPU may read resolved storage and
  **select among authored, admitted weight profiles** only. The moment a fixture's CPU step begins
  *computing* the urgency signal (rather than choosing a pre-authored input), or *emitting* the
  commitment, the SEAD line has been crossed — stop and escalate.
- **C-3 (guardrail placement).** Guardrails for this chain live at the **designer/importer/
  scenario-admission layer** (economy spec admission, scenario RON admission, authored weight
  profiles) — **not** the sim or boundary layer. Do not push economy/SEAD coupling into
  `simthing-sim` or the boundary protocol.

---

## 5. Binding guardrails / non-authorizations (kept binding)

```text
No production economy→mapping runtime bridge.
No generic boundary-output packet.
No DailyResolutionBoundary.
No default SimSession mapping pass-graph wiring.
No general scenario engine (FirstSliceScenarioSpec stays narrow).
No atlas / M-4A implementation.
No Resource Flow E-11 default economy.
No CPU urgency computation or CPU planner event emission.
No dense RegionCell grid readback at the boundary.
No cached commitment scan.
No calendar/pause semantics in simthing-sim (legible tick/boundary/day naming stays preferred).
```

Authoritative home: `docs/invariants.md` — "Boundary resolution (tick / boundary / day)" and
"Mapping (Sparse RegionCell)" rows, plus the new economy→mapping fixture-only row added in this pass.

---

## 6. Recommended next implementation handoff

**A is accepted/parked now** (this memo). Next *implementation* handoff should be **B — Authoring
Ergonomics R2** (better preview UX/diagnostics for designers; no substrate-semantic expansion) **or
C — another tiny product fixture combining a second non-map substrate with SEAD** (opt-in,
fixture-scoped). Either is acceptable; both stay at the designer/importer/fixture layer.

- **D (tightly bounded generic boundary-output packet):** **not yet** — only with explicit product
  authorization and a proof it cannot become `DailyResolutionBoundary` by another name (abstract,
  read-only carrier of already-resolved values; no calendar fields, no CPU recomputation, no day
  arithmetic).
- **E (mapping scale / M-4 atlas):** **not yet** — only after a named multi-theater scenario, an
  approved VRAM budget, and a §11-gate-passing M-4 PR.

---

## 7. Stop conditions for the next handoff (escalate; do not land)

Whichever of B/C is taken next must not introduce any of the §5 non-authorizations, and in
particular must not:
- create a production economy→mapping bridge or move the economy→SEAD link out of `tests/support`;
- have the CPU compute SEAD urgency or emit the commitment (selection of authored profiles only);
- add day/calendar/pause **semantics** to `simthing-sim`, or semantic WGSL;
- wire mapping into the default `SimSession` pass graph, or flip Resource Flow E-11 default-on;
- read dense RegionCell grids at the boundary, or add a cached commitment scan;
- grow `FirstSliceScenarioSpec` / the preview structs into a general scenario engine or
  runtime-command object.

For **D specifically:** a boundary-output packet must stay an abstract, read-only carrier; the moment
it gains a calendar field, a CPU compute step, or day arithmetic it is the forbidden primitive.

---

## 8. Doc / ADR updates made alongside this memo

- **New:** this memo.
- **`docs/reviews/phase_m_product_fixture_chain_review_packet.md`** — status flipped to
  **ACCEPTED (PASS WITH CONDITIONS)**.
- **`docs/invariants.md`** — one binding row added to the Mapping section: economy→mapping influence
  is fixture-orchestration-only (CPU selects authored weight profiles, never computes urgency or
  emits commitments); no production economy→mapping runtime bridge without a separate gated decision.
- **`docs/workshop/mapping_current_guidance.md`**, **`docs/workshop/workshop_current_state.md`**,
  **`docs/accumulator_op_v2_production_plan.md`**, **`docs/todo.md`** — status flipped to accepted;
  next step = B or C; not D, not E.
- **`docs/worklog.md`** — dated 2026-05-29 acceptance entry appended.

All updates are decision/classification only. No production code changed; `MappingExecutionProfile`
default remains `Disabled`; `simthing-sim` remains map-free; Resource Flow E-11 remains default-off;
`request_atlas_batching` stays rejected at admission.
