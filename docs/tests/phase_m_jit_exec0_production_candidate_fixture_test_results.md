# Phase M-JIT-EXEC-0 — Production-Candidate Execution Fixture — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `e2244f4` (M-JIT-REG-1 merge on `master`)  
**Final commit SHA:** _(set at commit; authoritative post-merge SHA is the GitHub squash-merge commit)_  
**Lane classification:** Tier-2 GPU/JIT production-candidate execution fixture (V7.7 §5)  
**Decision:** **IMPLEMENTED — ProductionCandidatePreview-gated execution fixture**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

| Question | Answer |
|----------|--------|
| Rebuild and admit exact GRAD-0→scorer via REG-1 in fixture? | Yes — `build_registry_entry` → `preview_production_candidate_registry_entry`. |
| Reuse GRAD-1 shader/execution without production wiring? | Yes — same fused WGSL and single-dispatch GPU path, test-local only. |
| Stay test-only/default-off? | Yes — `MappingExecutionProfile::default() == Disabled`; candidate `default_off=true`. |
| Bit-exact score parity? | Yes — aligned `f32::mul_add` CPU oracle vs GPU `fma` nesting. |
| Exclude mag2/sqrt? | Yes — exact graph omits mag2; WGSL has no `sqrt(` or `mag2`. |
| Production gates closed? | Production registry, scheduler, runtime cache, observer scheduling/caching, JIT cohort dispatch, default mapping. |

---

## Candidate Admission Flow

1. Build exact GRAD-0→scorer `KernelGraphSpec` (no mag2).
2. Build `KernelGraphRequestSpec`.
3. `preview_kernel_registry_manifest`.
4. Extract single `KernelRegistryEntryPreview`.
5. `preview_production_candidate_registry_entry`.
6. Assert `ProductionCandidatePreview`, `default_off`, `production_wiring=false`.
7. Execute GRAD-1-style fused GPU path.
8. Compare GPU output to CPU oracle.

Execution helper is not invoked if REG-1 admission fails (verified via atomic gate).

---

## Execution Design

- Score: `fma(w0, descent_x, fma(w1, descent_y, bias))` with bitcast literals.
- Clamp-boundary finite differences for dx/dy/descent.
- 10,000 structured observers on 128×128 field.
- One compute dispatch (`dispatch_count=1`, 157 workgroups @ 64).

---

## Dispatch Count

**1** dispatch for 10,000-observer batch.

---

## 10,000-Observer Result

`jit_exec0_production_candidate_grad1_executes_with_oracle_parity`: 10,000 outputs, 1 dispatch, sampled oracle parity bit-exact.

---

## CPU/GPU Oracle Parity Result

Bit-exact on sampled corpus (first 16, last 16, 32 structured indices) for dx, dy, descent_x, descent_y, score.

---

## Approximate Candidate Rejection Result

| Case | Result |
|------|--------|
| mag2 in canonical text | **Rejected** before GPU (`EXECUTION_HELPER_INVOKED` false) |
| sqrt descriptor entry | **Rejected** (`m_jit_sqrt_0_candidate`) before GPU |

---

## sqrt / mag2 Exclusion Result

Exact path WGSL: no `sqrt(`, no `mag2`. Candidate canonical text verified free of approximate markers before execution.

---

## Proof: No Production Registry/Cache/Scheduler/Default Wiring

EXEC-0 fixture uses explicit test GPU dispatch only after REG-1 admission. No `FirstSliceMappingSession`, `KernelCache`, `AccumulatorOpSession`, `simthing_sim` import, or production session wiring.

---

## Tests Run and Results

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

Production-candidate admission + EXEC-0 fixture symbols present in driver tests and spec; no production registry/cache/scheduler wiring added.

Approximate/sqrt/mag2 terms appear only in rejection tests and guardrail context; exact EXEC-0 score path excludes them.

Dispatch occurs only in explicit test fixture after `ProductionCandidatePreview` admission.

---

## Transient-Log Cleanup Result

Historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-driver/tests/phase_m_jit_exec0_production_candidate_fixture.rs`
- `docs/tests/phase_m_jit_exec0_production_candidate_fixture_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no production registry, no runtime kernel cache, no production scheduler, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate mag2/sqrt candidate executed, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-EXEC-0 is an explicit test-only/default-off ProductionCandidatePreview-gated execution fixture with CPU/GPU oracle parity; V7.7 / Mapping ADR / SEAD GPU-resident posture intact.
