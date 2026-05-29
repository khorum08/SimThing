# Phase M-JIT-DOC-CLOSEOUT — Documentation/Evidence Surface Cleanup Results

**Lane:** Phase M-JIT documentation/evidence surface closeout (not architecture, not new JIT feature, not SHA hygiene loop).

**Base HEAD:** `d62b09df9c81279e5346f432f86db01ab423965d` (M-JIT-PROD-0)

**Branch:** `phase-m-jit-doc-closeout-cleanup`

---

## Summary

Stale closed/confirmed JIT reports, docs-cleanup R-series hygiene loops, parking packets, and superseded `docs/tests/archive/` material were **deleted** (not archived). Active guidance was compacted for Opus review. E-phase / E11 / Resource Flow stalled evidence was **preserved**. Guardrails remain binding at the designer/spec-admission layer (`docs/invariants.md`, active workshop guidance). No SHA-mismatch remediation loop was started.

---

## Files deleted (`git rm`)

### Superseded M-JIT intermediate reports (conclusions summarized in PROD-0 + active docs)

- `docs/tests/phase_m_jit_exec0_production_candidate_fixture_test_results.md`
- `docs/tests/phase_m_jit_reg1_production_candidate_registry_admission_test_results.md`
- `docs/tests/phase_m_jit_cohort0_r1_collision_helper_fence_test_results.md`
- `docs/tests/phase_m_jit_key0_kernel_graph_identity_test_results.md`
- `docs/tests/phase_m_jit_desc2_kernel_graph_admission_test_results.md`

### Docs-cleanup R-series / pre-Phase-M hygiene loops

- `docs/tests/phase_m_docs_cleanup_archive_test_results.md`
- `docs/tests/phase_m_docs_cleanup_archive_r1_test_results.md` … `r7`
- `docs/tests/docs_cleanup_pre_phase_m_test_results.md`

### Superseded parking/review packets (conclusions in active guidance + `docs/reviews/`)

- `docs/tests/phase_m_boundary_resolution_review_packet_test_results.md`
- `docs/tests/phase_m_eml_gadget_2abc_parking_packet_test_results.md`
- `docs/tests/phase_m_eml_gadget_2de_parking_packet_test_results.md`

### Entire `docs/tests/archive/` tree (superseded sandbox logs/revert reports; workshop preserves remain under `docs/workshop/archive/`)

26 files including mapping/SEAD revert logs, parked sandbox full logs, and v7_6 promotion artifacts.

**No new archive was created.**

---

## Files retained for Opus review (JIT evidence minimum)

| Report | Role |
|---|---|
| `phase_m_jit_prod0_registry_shell_test_results.md` | Closure authority — default-off production registry shell |
| `phase_m_jit_exec1_cohort_execution_fixture_test_results.md` | Cohort GPU execution proof |
| `phase_m_jit_sqrt_candidate_battery_r1_test_results.md` | Native sqrt `ApproximateJitOnly` classification |
| `phase_m_jit_grad0_spatial_observer_r1_test_results.md` | Observer `mag2` classification |
| `phase_m_jit_grad1_observer_formula_fusion_test_results.md` | Fused exact-subset score path |
| `phase_m_jit_doc_closeout_cleanup_results.md` | This cleanup pass |

**Post-cleanup `docs/tests/` JIT file count:** 6 (including this report).

---

## Active docs updated

| File | Change |
|---|---|
| `docs/workshop/mapping_current_guidance.md` | Single compact M-JIT closed block; retained report links only; E-phase retention note; removed stale parking-test link |
| `docs/workshop/workshop_current_state.md` | Next action → JIT track closed; Opus surface compacted; archive ref updated |
| `docs/accumulator_op_v2_production_plan.md` | JIT ladder → compact closeout section; dead report links removed; parking ref → review packet |
| `docs/invariants.md` | Added **JIT Kernel Registry (Phase M-JIT, closed at PROD-0)** binding table |
| `docs/worklog.md` | One append-only closeout line |

**Must-remain active docs verified present:** `mapping_current_guidance.md`, `workshop_current_state.md`, `phase_m_gating_and_doc_policy.md`, `accumulator_op_v2_production_plan.md`, `invariants.md`, `worklog.md`.

---

## E-phase / E11 Protection Result

E-phase, E11, Resource Flow, ResourceEconomySpec, economy boundary, treasury, and economy→mapping files were separately scanned. Reports documenting stalled/review-blocked E-phase work were retained. The cleanup did not delete E-phase evidence merely because it was old. Only files classified as duplicate/superseded and not needed for review were deleted.

| File / pattern | Classification |
|---|---|
| `docs/tests/phase_m_daily_economy_fixture_test_results.md` | **KEEP** — active/stalled evidence |
| `docs/tests/phase_m_economy_sead_product_fixture_test_results.md` | **KEEP** — active/stalled evidence |
| `docs/tests/phase_m_resource_economy_authoring_ergonomics*.md` | **KEEP** — active/stalled evidence |
| `docs/tests/phase_m_boundary_resolution_doctrine_r1/r2_*` | **KEEP** — referenced by active docs |
| `docs/tests/phase_m_boundary_cadence_doctrine_audit.md` | **KEEP** — active/stalled evidence |
| `docs/workshop/e11_hierarchical_allocation_design.md` | **KEEP** — active/stalled evidence |
| `docs/workshop/e11_implementation_handoff.md` (untracked) | **KEEP** — ambiguous / stalled handoff |
| `docs/workshop/e11_readiness_review.md` (untracked) | **KEEP** — ambiguous / stalled review |
| `docs/reviews/e11b_*`, `resource_flow_*`, `phase_m_boundary_resolution_and_example_economy_*` | **KEEP** — active/stalled evidence |
| `docs/tests/sead_*_sandbox_test_results.md` | **KEEP** — stalled SEAD probe evidence |
| `docs/adr/resource_flow_substrate.md` | **KEEP** — binding ADR |
| E-phase files in deleted JIT/hygiene/parking targets | **DELETE** — duplicate/superseded (none matched E-phase retention criteria) |

---

## Broken-reference scan

Scanned active docs for links to deleted reports:

```
docs/workshop, docs/accumulator_op_v2_production_plan.md, docs/invariants.md, docs/worklog.md
```

**Result:** No active-doc references remain to deleted JIT intermediate reports, deleted parking-packet test results, or `docs/tests/archive/`. One stale link to `phase_m_first_slice_vertical_proof_parking_test_results.md` in `accumulator_op_v2_production_plan.md` was replaced with the accepted review packet. Historical references in `docs/reviews/` and `docs/workshop/archive/` were not edited (non-authoritative).

---

## SHA scan (report-only; intentionally not chased)

Active docs may cite historical SHAs (e.g. PROD-0 `d62b09d`). No SHA-mismatch remediation loop was started. No active doc was found misleading about current closure state after this pass.

---

## Guardrail scan

Required guardrails present in `docs/invariants.md` (JIT Kernel Registry section), `docs/workshop/mapping_current_guidance.md`, and `docs/accumulator_op_v2_production_plan.md`:

- no semantic WGSL
- no default SimSession mapping wiring
- no production economy→mapping bridge
- no CPU-side AI planner / urgency / commitment emission
- `simthing-sim` map/Gadget/Personality/Memory-semantic-free
- native sqrt not exact-authoritative; approximate `mag2` cannot feed exact score inputs
- `ProductionCandidatePreview` default-off / `production_wiring=false`
- production registry shell admits only validated production candidates
- scheduler/cache/default wiring are separate follow-on tracks
- ClauseThing proposal-only

---

## Scratch/tmp scan

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \)
```

Full `.log` files under `docs/tests/` are intentional test-run artifacts for non-JIT slices (EML-GADGET, boundary, economy fixtures). No `*tmp*` or `*scratch*` artifacts found. None deleted.

---

## Tests / scans run

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test phase_m_jit_prod0_registry_shell -- --nocapture` | **7/7 PASS** |
| `cargo test -p simthing-driver --test phase_m_jit_exec1_cohort_execution_fixture -- --nocapture --test-threads=1` | **5/5 PASS** (1 flaky under parallel default; passes single-threaded — pre-existing race, not introduced by doc pass) |
| `cargo test -p simthing-spec --test jit_kernel_registry_admission -- --nocapture` | **8/8 PASS** |
| `cargo test -p simthing-spec --test jit_kernel_registry_preview -- --nocapture` | **7/7 PASS** |
| `cargo test -p simthing-spec --test jit_kernel_cohort_preview -- --nocapture` | **7/7 PASS** |
| `cargo test -p simthing-spec --test jit_kernel_graph_identity -- --nocapture` | **7/7 PASS** |
| `cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture` | **11/11 PASS** |
| `cargo check --workspace` | **PASS** (pre-existing warnings only) |

---

## Workshop surface (`docs/workshop/` depth ≤ 2)

Active workshop files remain; stale E11 handoffs on disk (`e11_implementation_handoff.md`, `e11_readiness_review.md`) intentionally kept as untracked stalled evidence. `docs/workshop/archive/` historical material untouched (not expanded).

---

## Final verdict

**PASS** — Phase M-JIT documentation closeout cleanup landed; stale docs/tests reports and stale hygiene/parking artifacts were deleted rather than archived, only Opus-needed JIT evidence remains (6 files including this report), active docs no longer reference deleted reports, guardrails remain binding at the designer/spec-admission layer, no SHA-mismatch hygiene loop was started, no production/default wiring or exact/approx authority discipline was weakened, required tests and `cargo check --workspace` are green, E-phase stalled evidence preserved, and V7.7 / Mapping ADR / SEAD posture remains intact.
