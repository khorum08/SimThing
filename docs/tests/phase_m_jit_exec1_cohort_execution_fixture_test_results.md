# Phase M-JIT-EXEC-1 — Production-Candidate Cohort Execution Fixture — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `c5d8f31` (M-JIT-EXEC-0 + ClauseThing workshop strategy on `master`)  
**Final commit SHA:** `761ff2f` (branch `phase-m-jit-exec-1-cohort-execution-fixture`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT production-candidate cohort execution fixture (V7.7 §5)  
**Decision:** **IMPLEMENTED — ProductionCandidatePreview-gated cohort execution fixture**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

| Question | Answer |
|----------|--------|
| Identical requests group into one entry? | Yes — `preview_kernel_registry_manifest` cohorts by stable graph identity. |
| Pass ProductionCandidatePreview admission? | Yes — exact-only graph (no mag2 in canonical). |
| Combined batch without scheduler? | Yes — fixture concatenates observer buffers per request segment. |
| Test-only/default-off? | Yes — `MappingExecutionProfile::Disabled`; candidate `default_off=true`. |
| Bit-exact sampled parity? | Yes — per-segment CPU oracle vs GPU `fma` nesting. |
| Exclude mag2/sqrt? | Yes — exact graph + WGSL guards. |
| Production gates closed? | Production registry, scheduler, runtime cache, observer scheduling/caching, JIT cohort dispatch, default mapping. |

---

## Cohort Grouping Flow

1. Two identical exact GRAD-0→scorer requests (`exec1_req_alpha`, `exec1_req_beta`) with reordered node order.
2. `preview_kernel_registry_manifest` → **one** entry with sorted `request_ids`.
3. Distinct graph variant (bias read) → **two** manifest entries; single-cohort execution helper refuses.

---

## Candidate Admission Flow

REG-1 `preview_production_candidate_registry_entry` on the single cohort entry before GPU dispatch. Approximate mag2/sqrt candidates reject before execution helper.

---

## Execution Design

- Score: `fma(w0, descent_x, fma(w1, descent_y, bias))`.
- Combined batch: 10,000 observers × 2 requests = **20,000** observers.
- One compute dispatch over concatenated observer buffer.
- Sampled oracle parity validated per request segment.

---

## Dispatch Count

**1** dispatch for 20,000-observer combined cohort batch.

---

## Request Count and Observer Count

**2** request IDs; **20,000** combined observers.

---

## CPU/GPU Oracle Parity Result

Bit-exact on sampled indices from each request segment (first 16, last 16, 32 structured indices per segment).

---

## Distinct-Graph Refusal Result

Mixed exact + distinct graphs → manifest **2 entries**; `try_execute_admitted_cohort` returns error before GPU (`one manifest entry`).

---

## Approximate Candidate Rejection Result

| Case | Result |
|------|--------|
| mag2 in canonical | **Rejected** before GPU |
| sqrt descriptor entry | **Rejected** before GPU |

---

## sqrt / mag2 Exclusion Result

Exact fused WGSL path: no `sqrt(`, no `mag2` in score.

---

## Proof: No Production Registry/Cache/Scheduler/Default Wiring

EXEC-1 fixture is test-local only; no driver lib wiring; no ClauseThing parser/front-end touched.

---

## ClauseThing Non-Interference Note

No ClauseThing parser/front-end code modified. ClauseThing remains proposal-only per workshop strategy.

---

## Tests Run and Results

```
cargo test -p simthing-driver --test phase_m_jit_exec1_cohort_execution_fixture -- --nocapture
```

**Result:** 5 passed.

```
cargo test -p simthing-driver --test phase_m_jit_exec0_production_candidate_fixture -- --nocapture
```

**Result:** 4 passed.

```
cargo test -p simthing-spec --test jit_kernel_registry_admission -- --nocapture
```

**Result:** 8 passed.

```
cargo test -p simthing-spec --test jit_kernel_registry_preview -- --nocapture
```

**Result:** 7 passed.

```
cargo test -p simthing-spec --test jit_kernel_cohort_preview -- --nocapture
```

**Result:** 7 passed.

```
cargo test -p simthing-spec --test jit_kernel_graph_identity -- --nocapture
```

**Result:** 7 passed.

```
cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture
```

**Result:** 11 passed.

```
cargo test -p simthing-driver --test phase_m_jit_grad1_observer_formula_fusion -- --nocapture
```

**Result:** 5 passed.

```
cargo test -p simthing-driver --test phase_m_jit_grad0_spatial_observer -- --nocapture
```

**Result:** 8 passed.

```
cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
```

**Result:** 8 passed.

```
cargo check --workspace
```

**Result:** PASS.

---

## Scans Run and Results

Cohort admission-gated execution symbols present in EXEC-1 fixture; no production registry/cache/scheduler wiring added.

---

## Transient-Log Cleanup Result

Historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-driver/tests/phase_m_jit_exec1_cohort_execution_fixture.rs`
- `docs/tests/phase_m_jit_exec1_cohort_execution_fixture_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no production registry, no runtime kernel cache, no production scheduler, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no ClauseThing runtime/front-end implementation, no new production EML opcode, no production sqrt admission, no approximate mag2/sqrt candidate executed, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-EXEC-1 is an explicit test-only/default-off ProductionCandidatePreview-gated cohort execution fixture with CPU/GPU oracle parity; V7.7 / Mapping ADR / SEAD GPU-resident posture intact.
