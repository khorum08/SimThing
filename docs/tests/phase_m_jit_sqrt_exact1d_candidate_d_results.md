# SQRT-EXACT-1D — Candidate D Bitmask-Split Sqrt Probe Results

**Lane:** Tier-2 test-only shader/software deterministic `sqrt` implementation probe (not production admission, not JIT reopening).

**Base HEAD:** `fa09c477538be1e2d1ed2a85bd9e831ffc7996c3`

**Branch:** `phase-sqrt-exact-1d-candidate-d`

**Test file:** `crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`

---

## Summary

SQRT-EXACT-1D landed a test-only **Candidate D** (`CorrectlyRoundedHwBitmask`) slice in the existing sqrt exact candidate battery. D implements hardware-sqrt seed + bitmask split + hardened Dekker residual + directional Markstein snap + integer subnormal normalization path. **Candidate C / f64 / `F64RoundDown` was not implemented.** Candidate A remains legacy evidence only (not a live candidate). Candidate B remains fallback/contingency. On DX12/naga, D **fires residual corrections** (117 on dense normal corpus) where A never fired (0), and normal-range edge rows are bit-exact; however **subnormal outputs still flush to 0.0** (FTZ on final f32 store/scale), and dense normal max ULP remains **1**. No production exact sqrt admission was added; M-JIT remains closed at PROD-0.

---

## Candidate D implementation summary

| Component | Implementation |
|---|---|
| Enum variant | `CorrectlyRoundedHwBitmask` |
| WGSL entry | `sqrt_cr_d(x: f32) -> f32` |
| Normal path | `sqrt(s)` seed → bitmask split (`y_hi = bitcast<f32>(bitcast<u32>(y) & 0xFFFFF000u)`) → `dekker_residual_hardened` → `snap_directional` |
| Subnormal path | `sqrt_cr_d_subnormal_integer(x_bits)` — integer mantissa shift normalization, normal f32 bit pattern construction, scale via `pow2_i32` |
| Residual hardening | Split helper functions with intermediate storage (`yhi_yhi`, `term1`, `two_yhi_ylo`, `ylo_ylo`, `e_part1`, `sp`) to force sequencing against naga/DXC contraction |
| Probe shader | `emit_d_probe_wgsl` — records native vs D output, correction/up/down snap flags |

---

## Confirmations

| Check | Result |
|---|---|
| A not revived as live candidate | **Yes** — A tests remain SQRT-EXACT-0 legacy evidence only |
| C / f64 not implemented | **Yes** |
| B remains fallback | **Yes** — `sqrt_exact1d_candidate_b_fallback_still_classified` passes |
| Production sqrt admission | **No** |
| Native sqrt `ApproximateJitOnly` | **Yes** |
| `mag2` blocked as exact input | **Yes** |
| M-JIT closed at PROD-0 | **Yes** |
| Ignored exhaustive sweep added | **No** — D did not achieve zero ULP on edge + dense normal + subnormal |

---

## Residual-hardening strategy

**Strategy 1 (implemented):** Split Dekker residual into named helper `dekker_residual_hardened(y, s)` with separate intermediate variables and helper functions (`snap_directional`, `sqrt_positive_finite_normal`) to inhibit FP reassociation/contraction.

**Behavioral proof:** Contraction-barrier probe reports **117 corrections** and **117 D-vs-native output changes** on normal-range corpus; Candidate A reported **0 corrections** on the same backend (#305 / SQRT-EXACT-0).

**Not claimed:** Generated HLSL was not manually inspected in this pass; exactness is not claimed until exhaustive `max_ulp == 0`.

---

## Integer subnormal normalization strategy

**Implemented:** `sqrt_cr_d_subnormal_integer` detects `exp == 0 && mant != 0`, left-shifts mantissa with leading-zero loop, constructs normal `s_bits`, runs normal sqrt path, applies integer-derived scale (`pow2_i32(-half_k)` with odd-k `1/sqrt(2)` factor).

**Result on DX12/naga:** Integer path is exercised (`integer_path_used = 2572/2572` on subnormal corpus), but **all subnormal outputs flush to 0.0** (`flush_count = 2572`). Final `y * scale` produces subnormal results that FTZ eliminates before readback — same class of failure as A/B, unresolved without integer-only output bit construction (Candidate E contingency).

---

## WGSL compile result

**PASS** — `sqrt_exact1d_candidate_d_wgsl_compiles_semantic_free` compiles D through wgpu/naga on test host; `sqrt(4.0) → 2.0` bit-exact.

---

## Semantic scan result

**PASS** — no forbidden semantic terms; no `f64`, `SHADER_F64`, `F64RoundDown`, `sqrt_cr_c`, or `fma(` in D WGSL.

---

## Edge-row result (21 named rows)

| Metric | Value |
|---|---:|
| Total rows | 21 |
| Exact (non-NaN) | 16 |
| Normal exact | 16 |
| Normal max ULP | **0** |
| Subnormal exact | **0/2** |
| NaN classified | 3 |

**Subnormal failures:** `smallest_subnormal`, `largest_subnormal` → GPU `0.0` vs CPU subnormal sqrt.

**Normal highlights:** All normal finite edge rows including A/B failure boundaries (`rounding_boundary_a/b`, neighbors) bit-exact.

---

## Dense normal sweep result

Corpus: 1044 inputs (dense stratified + A/B failure neighborhoods, normal+zero+inf only).

| Metric | Value |
|---|---:|
| Tested | 1044 |
| Exact bits | 1032 |
| Max ULP | **1** |
| Classification | **ApproximateJitOnly** |
| Total corrections | 117 |
| Up corrections | 116 |
| Down corrections | 1 |

**Worst-case rows:** Near-min-normal values (`~1e-38` range) at ULP=1 (e.g. `x=1.7632415e-38`).

---

## Subnormal sweep result

Corpus: 2572 inputs (first/last 1024 positive subnormals + pow2 mantissas + LCG mantissas).

| Metric | Value |
|---|---:|
| Tested | 2572 |
| Exact bits | **0** |
| Max ULP | 536870911 |
| Integer path used | 2572 |
| Output flush count | **2572** |

---

## Contraction-barrier probe result

Corpus: 1043 positive-finite normal inputs (A/B neighborhoods + dense stratified normals).

| Metric | Value |
|---|---:|
| Native sqrt mismatch vs CPU | 129 |
| D mismatch vs CPU | 12 |
| Residual correction count | **117** |
| Up snap count | 116 |
| Down snap count | 1 |
| D changes output vs native sqrt | **117** |

**Interpretation:** D is **not** returning native `sqrt` unchanged; residual path fires where A did not. Remaining D mismatches (12) and 1-ULP dense failures indicate incomplete exactness on this backend.

---

## Classification

| Candidate | Verdict |
|---|---|
| D (`CorrectlyRoundedHwBitmask`) | **ApproximateJitOnly** on normal-range dense corpus (`max_ulp=1`); normal edge rows bit-exact; **subnormal path unresolved (FTZ flush)**; **not** `ExactCandidatePendingExhaustiveSweep` (subnormal + dense not zero ULP) |
| A | Legacy / empirically dead as live candidate (correction_count=0) |
| B | Fallback / **RejectedDeferred** on this host |
| C / f64 | **Not implemented** |

**Candidate E escalation:** Subnormal output FTZ remains a blocker; integer-only output bit construction (E) is the documented contingency if D cannot survive subnormal store/scale on DX12.

---

## Guardrail confirmations

Same as SQRT-EXACT-0: no exact sqrt kernel in `landed_jit_kernel_descriptors()`; `validate_exact_kernel_inputs` rejects `sqrt_out` and `mag2`; baseline `accumulator_op.wgsl` remains sqrt-free.

---

## Required scans (recorded)

### D presence

```
rg "CorrectlyRoundedHwBitmask|sqrt_cr_d|SQRT-EXACT-1D|candidate_d" crates/simthing-driver/tests docs/tests/phase_m_jit_sqrt_exact1d_candidate_d_results.md docs/workshop/sqrt_candidates.md
```

**Result:** D present in test battery, this report, and updated design note §11.

### C/f64 rejection

```
rg "F64RoundDown|SHADER_F64|f64\\(|sqrt_cr_c|Candidate C" crates docs
```

**Result:** No D implementation references; rejection/guardrail context only.

### A not revived

```
rg "fma\\(|sqrt_cr_a|CorrectlyRoundedHwFma" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs docs/tests/phase_m_jit_sqrt_exact1d_candidate_d_results.md docs/workshop/sqrt_candidates.md
```

**Result:** A remains SQRT-EXACT-0 legacy; `fma` only in A probe/WGSL and diagnostic text.

### Exact authority blocked

```
rg "ApproximateJitOnly|ExactDeterministic|native sqrt|mag2|validate_exact_inputs|landed_jit_kernel_descriptors" docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/invariants.md docs/tests/phase_m_jit_sqrt_exact1d_candidate_d_results.md crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs
```

**Result:** Native sqrt and mag2 remain blocked from exact authority.

### Production wiring guardrails

```
rg "default SimSession|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|semantic WGSL|scheduler|kernel cache" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs docs/tests/phase_m_jit_sqrt_exact1d_candidate_d_results.md docs/workshop/sqrt_candidates.md
```

**Result:** Guardrails only; no authorization.

---

## Tests / commands

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture` | **PASS** (13 passed, 1 ignored) |
| `cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture` | **PASS** (8 passed) |
| `cargo check --workspace` | **PASS** |

Ignored exhaustive sweep for D: **not added** (zero ULP not achieved on subnormal + dense normal).

---

## Transient cleanup

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

**Result:** No scratch/tmp artifacts deleted.

---

## Final verdict

**PASS — SQRT-EXACT-1D landed** as a test-only Candidate D bitmask-split sqrt probe. D implements residual hardening and integer subnormal input normalization; residual corrections fire on DX12 where A did not; normal edge rows are bit-exact; subnormal output FTZ remains unresolved. Candidate C/f64 not implemented; A not revived; B remains fallback; no production exact sqrt admission; native sqrt remains `ApproximateJitOnly`; M-JIT remains closed at PROD-0; active docs updated; V7.7 / Mapping ADR / SEAD guardrails intact.
