# Phase M-JIT-REG-1 — Production-Candidate Registry Admission Gate — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `ee0d2e33dcbe87cb2e029b3b25aab5837e6051f1` (M-JIT-REG-0 merge on `master`)  
**Final commit SHA:** `9545a4d` (branch `phase-m-jit-reg-1-production-candidate-admission`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT production-candidate registry admission preview (V7.7 §5)  
**Decision:** **IMPLEMENTED — ProductionCandidatePreview admission gate**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

| Question | Answer |
|----------|--------|
| Sufficient manifest fields? | Yes — `stable_key`, `canonical_text`, sorted `request_ids`, lane, `default_off`, `production_wiring`. |
| Purely spec-layer? | Yes — admission inspects REG-0 entry + canonical text markers only. |
| Non-dispatching/default-off? | Yes — promoted entries remain `default_off=true`, `production_wiring=false`. |
| Exact-only authority required? | Yes — canonical text must not contain `ApproximateDiagnostic` or approximate markers. |
| Semantic-free canonical text? | Yes — forbidden semantic term scan on canonical text. |
| Reject approximate native math/outputs? | Yes — rejects `ApproximateJitOnly`, `mag2`, `sqrt_out`, sqrt descriptor id. |
| Production gates closed? | Production registry, scheduler, runtime cache, observer scheduling/caching, JIT dispatch, default mapping, economy→mapping bridge. |

---

## Production-Candidate Preview Model

Extended `KernelRegistryLane` with `ProductionCandidatePreview`.

```rust
preview_production_candidate_registry_entry(&KernelRegistryEntryPreview)
    -> Result<KernelRegistryEntryPreview, SpecError>
```

Returns copy with `lane = ProductionCandidatePreview`, `default_off = true`, `production_wiring = false`.

---

## Admission Rules

Promote from `TestOnlyPreview` only when:

1. REG-0 manifest validation passes for single-entry manifest.
2. `stable_key` non-empty and `jit-graph-v1:` prefix.
3. Canonical text non-empty, semantic-free.
4. No forbidden markers: `m_jit_sqrt_0_candidate`, `sqrt_out`, `magnitude_out`, `mag2`, `ApproximateJitOnly`, `ApproximateDiagnostic`, `RejectedDeferred`.
5. `request_ids` non-empty and sorted.
6. `default_off == true`, `production_wiring == false`.

---

## Exact Candidate Admission Result

Exact GRAD-0→scorer graph (grad0 without `mag2`, scorer with exact `score`): **Admits** as `ProductionCandidatePreview`.

---

## Approximate / mag2 / sqrt Rejection Results

| Case | Result |
|------|--------|
| GRAD-0 `mag2` edge (upstream) | **Rejected** before manifest |
| Manual canonical with `mag2` | **Rejected** at candidate admission |
| SQRT descriptor canonical | **Rejected** (`m_jit_sqrt_0_candidate`) |

---

## Semantic / Default-On / Production-Wired Rejection Results

| Case | Result |
|------|--------|
| Forbidden semantic term in canonical | **Rejected** |
| `default_off = false` | **Rejected** |
| `production_wiring = true` | **Rejected** |
| Bad/missing `jit-graph-v1:` key | **Rejected** |
| Unsorted `request_ids` | **Rejected** |

---

## Proof: No Scheduler / Cache / GPU Dispatch

Registry admission module contains no GPU imports, dispatch APIs, cache structures, or scheduler wiring.

---

## Tests Run and Results

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
cargo test -p simthing-spec --test jit_kernel_descriptor_admission -- --nocapture
```

**Result:** 8 passed.

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

Documented in report sections above; no production registry/cache/scheduler added.

---

## Transient-Log Cleanup Result

Historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-spec/src/compile/jit_kernel_registry_preview.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/jit_kernel_registry_admission.rs`
- `docs/tests/phase_m_jit_reg1_production_candidate_registry_admission_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no GPU dispatch/runtime scheduler/cache, no production JIT wiring, no production observer scheduling/caching, no production registry, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate output admitted as exact-authoritative input, no exact deterministic native sqrt admission, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-REG-1 is a spec-layer ProductionCandidatePreview admission gate only, with no production execution/caching/scheduling; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
