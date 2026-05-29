# Phase M-JIT-DESC-0 — Kernel Descriptor / Admission Manifest — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `263184ca8b890e92d8b580e93f36c7b70070cec7` (M-JIT-GRAD-1 merge on `master`)  
**Final commit SHA:** `9a0feaf` (branch `phase-m-jit-desc-0-kernel-descriptor`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT descriptor/admission proof (V7.7 §5)  
**Decision:** **IMPLEMENTED — test-only kernel descriptor/admission manifest**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected landed M-JIT proof reports and test files for M-JIT-0, M-JIT-SQRT-0/R1, M-JIT-GRAD-0/R1, and M-JIT-GRAD-1.

Confirmed before edits:

1. **Exact-authoritative outputs:** M-JIT-0 `out_col`; GRAD-0 `dx`/`dy`/descent; GRAD-1 `dx`/`dy`/descent/`score`.
2. **Approximate/diagnostic outputs:** M-JIT-SQRT-0 overall `ApproximateJitOnly`; GRAD-0 `mag2` (`ApproximateJitOnly` on batch corpus).
3. **Test-only kernels:** all landed JIT/observer proofs are test-only, no production wiring.
4. **Native approximate math:** M-JIT-SQRT-0 candidate only (`sqrt`).
5. **Semantic-free:** all proof WGSL is semantic-free.
6. **Production gates closed:** observer scheduling/caching, JIT cohort dispatch, exact `sqrt` admission, default mapping, economy→mapping bridge.

---

## Descriptor Model

```rust
KernelLane { TestOnly, ProductionCandidate }
OutputAuthority { ExactAuthoritative, ApproximateDiagnostic, RejectedDeferred }
NativeMathClass { None, ApproximateJitOnly }

KernelDescriptor {
    id, lane, reads, writes, native_math,
    semantic_free, default_off, production_wiring
}
```

Validation helper `validate_exact_inputs(producer, required_exact_inputs)` rejects approximate/diagnostic or missing outputs when used as exact-authoritative inputs.

---

## Kernel Descriptors Added

| ID | Lane | Native math | Production wiring |
|----|------|-------------|-------------------|
| `m_jit_0_weighted_accumulator` | TestOnly | None | false |
| `m_jit_0_ema` | TestOnly | None | false |
| `m_jit_sqrt_0_candidate` | TestOnly | ApproximateJitOnly | false |
| `m_jit_grad_0_observer` | TestOnly | None | false |
| `m_jit_grad_1_observer_score` | TestOnly | None | false |

---

## Authority Classification Table

| Kernel | Output | Authority |
|--------|--------|-----------|
| `m_jit_0_weighted_accumulator` | `out_col` | ExactAuthoritative |
| `m_jit_0_ema` | `out_col` | ExactAuthoritative |
| `m_jit_sqrt_0_candidate` | `sqrt_out`, `magnitude_out` | ApproximateDiagnostic |
| `m_jit_grad_0_observer` | `dx`, `dy`, `descent_x`, `descent_y` | ExactAuthoritative |
| `m_jit_grad_0_observer` | `mag2` | ApproximateDiagnostic |
| `m_jit_grad_1_observer_score` | `dx`, `dy`, `descent_x`, `descent_y`, `score` | ExactAuthoritative |
| `m_jit_grad_1_observer_score` | _(no `mag2` write)_ | — |

---

## Approximate-Output-as-Exact-Input Rejection

| Input source | Input | Result |
|--------------|-------|--------|
| GRAD-0 | `mag2` | **Rejected** (approximate/diagnostic) |
| SQRT-0 | `sqrt_out` | **Rejected** (approximate/diagnostic) |
| GRAD-0 | `dx`, `dy`, `descent_x`, `descent_y` | **Admitted** (exact-authoritative) |
| GRAD-1 | `score` | **Admitted** (exact-authoritative) |

---

## Default-Off / No-Production-Wiring Result

All five descriptors: `lane == TestOnly`, `default_off == true`, `production_wiring == false`.  
`MappingExecutionProfile::default() == Disabled` asserted in tests.

---

## Semantic-Free Descriptor Result

All descriptors: `semantic_free == true`.  
Descriptor IDs and read/write buffer names contain no forbidden semantic terms.

---

## Tests Run and Results

```
cargo test -p simthing-driver --test phase_m_jit_desc0_kernel_descriptor -- --nocapture
```

**Result:** 5 passed.

```
cargo test -p simthing-driver --test phase_m_jit_grad1_observer_formula_fusion -- --nocapture
```

**Result:** 5 passed.

```
cargo test -p simthing-driver --test phase_m_jit_grad0_spatial_observer -- --nocapture
```

**Result:** 8 passed.

```
cargo test -p simthing-driver --test phase_m_jit_evaleml_wgsl_prototype -- --nocapture
```

**Result:** 6 passed.

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

```
rg "ExactAuthoritative|ApproximateDiagnostic|RejectedDeferred|KernelDescriptor|OutputAuthority|NativeMathClass|TestOnly|ProductionCandidate" crates/simthing-driver/tests docs/tests/phase_m_jit_desc0_kernel_descriptor_test_results.md
```

**Result:** descriptor test and report record output authority explicitly.

```
rg "mag2|ApproximateJitOnly|diagnostic|exact-authoritative|score|observer formula|fused observer" crates/simthing-driver/tests docs/tests/phase_m_jit_desc0_kernel_descriptor_test_results.md docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md
```

**Result:** `mag2` diagnostic-only; GRAD-1 score exact-authoritative preserved in descriptors and docs.

```
rg "sqrt\(" crates/simthing-driver/tests crates/simthing-gpu/src/shaders
```

**Result:** no `sqrt(` in GRAD-0/GRAD-1/baseline runtime shaders; `sqrt(` only in M-JIT-SQRT candidate test context.

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-driver/tests/phase_m_jit_desc0_kernel_descriptor.rs docs/tests/phase_m_jit_desc0_kernel_descriptor_test_results.md
```

**Result:** descriptor names semantic-free; forbidden terms only in guardrail context in report.

```
rg "production JIT|observer scheduling|observer caching|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|simthing-sim.*Personality|simthing-sim.*Memory|chained OrderBand|automatic snapshot|atlas|M-4A|ActiveOnlyExperimentalNoHalo|source_mask|source identity" crates docs
```

**Result:** guardrail/deferred context only; no new production/default wiring.

---

## Transient-Log Cleanup Result

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

**Result:** historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-driver/tests/phase_m_jit_desc0_kernel_descriptor.rs`
- `docs/tests/phase_m_jit_desc0_kernel_descriptor_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no production JIT wiring, no production observer scheduling/caching, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate output admitted as exact-authoritative input, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-DESC-0 is a test-only kernel descriptor/admission manifest proof preserving exact vs approximate output authority; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
