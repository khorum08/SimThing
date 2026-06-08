# SQRT-EXACT-0 — Shader/Software Deterministic Sqrt Candidate Battery Results

**Lane:** Tier-2 test-only shader/software deterministic `sqrt` implementation probe (not production admission, not JIT reopening).

**Base HEAD:** `8f537d9a5b65f16906b2461a4dd6438553a39e41`

**Branch:** `phase-sqrt-exact-0-candidate-battery`

**Test file:** `crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`

---

## Summary

SQRT-EXACT-0 landed a test-only GPU candidate battery for **Candidate A** (`CorrectlyRoundedHwFma`) and **Candidate B** (`CorrectlyRoundedNewtonTwoProduct`) only. **Candidate C / f64 / `F64RoundDown` was not implemented or tested.** WGSL compiles; semantic scans are clean. Edge rows and dense stratified sweeps were recorded on DX12/naga. No production exact sqrt admission was added; M-JIT remains closed at PROD-0; native sqrt and `mag2` remain blocked from exact authority.

---

## Candidates

| ID | Name | Status |
|---|---|---|
| A | `CorrectlyRoundedHwFma` | Implemented + probed |
| B | `CorrectlyRoundedNewtonTwoProduct` | Implemented + probed |
| C | `F64RoundDown` / f64 | **Explicitly rejected — not implemented or tested** |

---

## WGSL compile result

**PASS** — both candidate shader modules compile through `wgpu`/naga on the test host.

---

## Semantic-free scan

**PASS** — generated WGSL contains no forbidden semantic terms. Forbidden exact-0 terms absent from implementation WGSL: `f64`, `F64RoundDown`, `SHADER_F64`, `sqrt_cr_c`.

---

## Edge-row results (15 named rows × 2 candidates)

| Candidate | All exact | Normal exact | Normal max ULP | Subnormal exact |
|---|---:|---:|---:|---:|
| A (`CorrectlyRoundedHwFma`) | 10/15 | 10/13 | 0 | 0/2 |
| B (`CorrectlyRoundedNewtonTwoProduct`) | 8/15 | 8/13 | 1 (worst-case row) | 0/2 |

**Notes:**
- Subnormal rows (`smallest_subnormal`, `largest_subnormal`) return `0.0` on GPU for both candidates on this DX12 backend (subnormal intermediate flush); recorded, not treated as promotion evidence.
- A passes all non-subnormal edge rows bit-exact on this host.
- B mismatches on some normal edges (e.g. `rounding_boundary_b`: GPU `1.0` vs CPU `0.99999994`).

NaN rows: both sides NaN; payload bits may differ — classified as NaN/NaN without overclaiming payload parity.

---

## Dense stratified sweep

Corpus: 1065 deterministic bit patterns (subnormal-heavy + exponent/mantissa sweep + SQRT-0 neighborhoods).

| Candidate | Scope | Tested | Exact bits | Max ULP | Classification |
|---|---|---:|---:|---:|---|
| A | all | 1065 | 897 | 536870911 | RejectedDeferred (subnormal flush dominates) |
| A | normal+zero+inf | 1026 | 897 | **1** | **ApproximateJitOnly** |
| B | all | 1065 | 873 | 1598029824 | RejectedDeferred |
| B | normal+zero+inf | 1026 | 873 | 1598029824 | RejectedDeferred |

---

## Candidate A FMA-fusion probe

- Positive-finite corpus tested: 1064
- Correction count (residual path fired): **0**
- Max ULP (positive-finite corpus incl. subnormals): 536870911
- **Interpretation:** On normal-range inputs A is within **1 ULP** on dense stratified sweep (`max_ulp=1`), consistent with **non-fused `fma` or residual miscorrection** on this backend; subnormal flush dominates all-corpus max ULP. **Not promotable** without exhaustive `max_ulp == 0` proof.

---

## Candidate B portability / no-fma-dependency

**PASS (static)** — B WGSL contains no `fma(` calls; native `sqrt` appears only in special-value passthrough guard (`is_non_finite_positive_or_nonpositive`). Finite-positive path is shader/software (Newton + TwoProduct).

**Runtime:** B does not yet achieve bit-exact normal-range parity on this DX12 host (edge + dense failures above). **Not promotable.**

---

## Full exhaustive sweep

| Property | Value |
|---|---|
| Test name | `sqrt_exact0_full_exhaustive_sweep_is_ignored_by_default` |
| Default CI | **Ignored** (`#[ignore]`) |
| Promotion criterion | `max_ulp == 0` over `0x0000_0000..=0x7F7F_FFFF` |
| Status | **Not run in this landing pass** |

**Explicit command:**

```bash
cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery sqrt_exact0_full_exhaustive_sweep_is_ignored_by_default -- --ignored --nocapture
```

---

## Classification (this landing pass)

| Candidate | Verdict |
|---|---|
| A | **ApproximateJitOnly** on normal-range dense corpus (`max_ulp=1`); edge normal rows bit-exact; **exact-candidate pending exhaustive sweep** (not promoted) |
| B | **RejectedDeferred / not promotable** on this host (normal-range edge and dense failures); requires algorithm/backend remediation before exhaustive sweep |

---

## Guardrail confirmations

| Check | Result |
|---|---|
| Production sqrt admission added | **No** |
| M-JIT closed at PROD-0 | **Yes** |
| Native sqrt remains `ApproximateJitOnly` in landed descriptors | **Yes** |
| `mag2` blocked as exact input | **Yes** (`validate_exact_kernel_inputs` rejects) |
| No exact sqrt kernel in `landed_jit_kernel_descriptors()` | **Yes** |
| Implementation code beyond test battery | **No** |
| Candidate C / f64 implemented | **No** |
| E-phase evidence touched | **No** |

---

## Required scans (recorded)

### C/f64 rejection scan

```
rg "F64RoundDown|SHADER_F64|f64\\(|sqrt_cr_c|Candidate C" crates docs
```

**Result:** Implementation references only in test file forbidden-term guards and comments. Design note retains f64 rejection text only.

### A/B presence scan

**Result:** `CorrectlyRoundedHwFma`, `CorrectlyRoundedNewtonTwoProduct`, `sqrt_cr_a`, `sqrt_cr_b` present in test file and this report.

### Guardrail scan

**Result:** Active docs and invariants preserve `ApproximateJitOnly` native sqrt, exhaustive-proof requirement, and admission firewall.

---

## Tests / commands

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture` | **6 passed; 1 ignored** |
| `cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture` | **8/8 PASS** |
| `cargo check --workspace` | **PASS** |

---

## Final verdict

**PASS** — SQRT-EXACT-0 landed as a test-only shader/software deterministic sqrt candidate battery for Candidate A and Candidate B only; Candidate C/f64 was not implemented or tested, WGSL compiled, semantic scans were clean, edge rows and dense stratified sweep results were recorded, full exhaustive sweep is available as an ignored/default-off proof gate, no production exact sqrt admission or `mag2` exact authority was added, M-JIT remains closed at PROD-0, and V7.7 / Mapping ADR / FIELD_POLICY guardrails remain intact.
