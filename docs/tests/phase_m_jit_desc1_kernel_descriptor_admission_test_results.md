# Phase M-JIT-DESC-1 — Spec-Layer Kernel Descriptor Admission Preview — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `7a711316d3c7b91e8147eca631aeb051031b1723` (M-JIT-DESC-0 merge on `master`)  
**Final commit SHA:** `3992f02` (branch `phase-m-jit-desc-1-spec-admission`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT descriptor admission implementation (V7.7 §5)  
**Decision:** **IMPLEMENTED — spec-layer kernel descriptor admission preview**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected DESC-0 driver test, DESC-0 report, GRAD-0/R1, GRAD-1, SQRT R1 reports, mapping guidance, accumulator plan, invariants, and `simthing-spec` admission conventions (`region_field_admission.rs`, `resource_economy_admission.rs`).

| Question | Answer |
|----------|--------|
| Where should descriptor admission preview live? | `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs` — matches existing `compile/*_admission` pattern; exported from `compile/mod.rs` and `lib.rs`. |
| Can DESC-0 model move/mirror without runtime scheduling? | Yes — pure spec types + validation; no GPU dispatch, cache, or session wiring. |
| Can exact-input validation remain pure spec logic? | Yes — `validate_exact_kernel_inputs` inspects producer output authority only. |
| Can descriptor IDs/read/write names remain semantic-free? | Yes — forbidden-term scan on id/reads/writes at admission time. |
| Can this be tested without GPU runtime code? | Yes — `simthing-spec/tests/jit_kernel_descriptor_admission.rs` (8 unit tests). |
| Which production gates remain closed? | Production descriptor registry/scheduler, observer scheduling/caching, JIT cohort dispatch, default SimSession mapping, production economy→mapping bridge, ProductionCandidate lane, production sqrt admission. |

---

## Spec-Layer Descriptor / Admission Model

Module: `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs`

```rust
KernelLane { TestOnly, ProductionCandidate }
OutputAuthority { ExactAuthoritative, ApproximateDiagnostic, RejectedDeferred }
NativeMathClass { None, ApproximateJitOnly }

KernelDescriptorSpec { id, lane, reads, writes, native_math, semantic_free, default_off, production_wiring }
KernelOutputSpec { name, authority }

validate_kernel_descriptor_admission(&KernelDescriptorSpec) -> Result<(), SpecError>
validate_exact_kernel_inputs(producer, &[&str]) -> Result<(), SpecError>
landed_jit_kernel_descriptors() -> Vec<KernelDescriptorSpec>
```

Errors use `SpecError::JitKernelDescriptorAdmission { kernel, reason }`.

---

## Admission Rules

`validate_kernel_descriptor_admission` rejects:

1. `production_wiring == true`
2. `default_off == false`
3. `semantic_free == false`
4. forbidden semantic terms in id/read/write names
5. `NativeMathClass::ApproximateJitOnly` with any `ExactAuthoritative` output
6. empty outputs
7. duplicate output names
8. `KernelLane::ProductionCandidate` (no production registry gate yet)

`validate_exact_kernel_inputs` rejects missing, `ApproximateDiagnostic`, and `RejectedDeferred` outputs; admits only `ExactAuthoritative`.

---

## Landed Descriptors (TestOnly)

| ID | Outputs | Native math |
|----|---------|-------------|
| `m_jit_0_weighted_accumulator` | `out_col` exact | None |
| `m_jit_0_ema` | `out_col` exact | None |
| `m_jit_sqrt_0_candidate` | `sqrt_out`, `magnitude_out` approximate | ApproximateJitOnly |
| `m_jit_grad_0_observer` | `dx`/`dy`/descent exact; `mag2` approximate | None |
| `m_jit_grad_1_observer_score` | `dx`/`dy`/descent/`score` exact; no `mag2` | None |

All five pass `validate_kernel_descriptor_admission`.

---

## Exact-Input Validation Results

| Producer | Input | Result |
|----------|-------|--------|
| GRAD-0 | `mag2` | **Rejected** (approximate/diagnostic) |
| SQRT-0 | `sqrt_out` | **Rejected** (approximate/diagnostic) |
| GRAD-0 | `dx`, `dy`, `descent_x`, `descent_y` | **Admitted** |
| GRAD-1 | `score` | **Admitted** |

---

## Rejection Cases (spec tests)

| Test | Rejection trigger |
|------|-------------------|
| `jit_desc1_rejects_production_wiring` | `production_wiring = true` |
| `jit_desc1_rejects_default_on` | `default_off = false` |
| `jit_desc1_rejects_semantic_descriptor_names` | forbidden term in id/read/write |
| `jit_desc1_rejects_approximate_native_math_exact_output` | ApproximateJitOnly + ExactAuthoritative output |
| `jit_desc1_rejects_production_candidate_lane` | `ProductionCandidate` lane |
| `jit_desc1_rejects_duplicate_outputs` | duplicate output name |

---

## Tests Run and Results

```
cargo test -p simthing-spec --test jit_kernel_descriptor_admission -- --nocapture
```

**Result:** 8 passed.

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
cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
```

**Result:** 8 passed.

```
cargo check --workspace
```

**Result:** PASS.

M-JIT-0 not touched; `phase_m_jit_evaleml_wgsl_prototype` not re-run.

---

## Scans Run and Results

```
rg "KernelDescriptorSpec|validate_kernel_descriptor_admission|validate_exact_kernel_inputs|OutputAuthority|NativeMathClass|KernelLane" crates/simthing-spec crates/simthing-driver/tests docs/tests/phase_m_jit_desc1_kernel_descriptor_admission_test_results.md
```

**Result:** spec-layer descriptor/admission in `jit_kernel_descriptor_admission.rs`, tests, exports; DESC-0 driver test unchanged as test proof.

```
rg "ApproximateDiagnostic|ExactAuthoritative|RejectedDeferred|mag2|sqrt_out|score|ProductionCandidate|TestOnly" crates/simthing-spec crates/simthing-driver/tests docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_jit_desc1_kernel_descriptor_admission_test_results.md
```

**Result:** `mag2` and `sqrt_out` remain approximate/diagnostic; GRAD-1 `score` exact-authoritative; ProductionCandidate gated at admission.

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-spec crates/simthing-driver/tests/phase_m_jit_desc0_kernel_descriptor.rs docs/tests/phase_m_jit_desc1_kernel_descriptor_admission_test_results.md
```

**Result:** forbidden terms only in guardrail lists/tests/report context; descriptor IDs/read/write names remain semantic-free.

```
rg "production JIT|observer scheduling|observer caching|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|simthing-sim.*Personality|simthing-sim.*Memory|chained OrderBand|automatic snapshot|atlas|M-4A|ActiveOnlyExperimentalNoHalo|source_mask|source identity" crates docs
```

**Result:** guardrail/deferred context only in existing tests/docs; no new production/default wiring from DESC-1.

---

## Transient-Log Cleanup Result

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

(PowerShell equivalent: `Get-ChildItem docs/tests -File` filtered for `.log` / `tmp` / `scratch`.)

**Result:** historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/error.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/jit_kernel_descriptor_admission.rs`
- `docs/tests/phase_m_jit_desc1_kernel_descriptor_admission_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no production JIT wiring, no production observer scheduling/caching, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate output admitted as exact-authoritative input, no ProductionCandidate descriptor admitted without a separate gate, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-DESC-1 is a spec-layer descriptor admission preview preserving exact vs approximate output authority; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
