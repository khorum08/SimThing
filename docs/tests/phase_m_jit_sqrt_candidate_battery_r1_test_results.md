# Phase M-JIT-SQRT-0 R1 — Magnitude Oracle-Order Correction — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `72aab044626b236cd571c8253793a58536b73e95` (M-JIT-SQRT-0 merge on `master`)  
**Final commit SHA:** `678c114` (branch `phase-m-jit-sqrt-0-r1-oracle-order`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 JIT sqrt remedial evidence pass (V7.7 §5)  
**Decision:** **REMEDIAL — vector magnitude CPU oracle aligned to generated WGSL shader-text order**  
**Verdict:** **PASS — `ApproximateJitOnly` overall classification preserved**

---

## Pre-Edit Evaluation Summary

Inspected:

- `crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_test_results.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/workshop/mapping_current_guidance.md`
- `crates/simthing-gpu/src/shaders/accumulator_op.wgsl`
- `crates/simthing-driver/tests/phase_m_jit_evaleml_wgsl_prototype.rs`

Confirmed before edits:

1. Generated vector WGSL uses separate multiply/add before `sqrt` (`tmp_0 = x*x; tmp_1 = y*y; tmp_2 = tmp_0 + tmp_1; sqrt(tmp_2)`).
2. Prior vector CPU oracle used `x.mul_add(x, y * y).sqrt()` — not the same operation order as generated WGSL.
3. Direct scalar classification was already `ApproximateJitOnly` (max ULP=1).
4. No production `sqrt` admission exists; baseline `accumulator_op.wgsl` remains `sqrt`-free.

---

## Found Issue

M-JIT-SQRT-0 correctly classified native WGSL `sqrt` as `ApproximateJitOnly` overall because the direct scalar corpus shows max ULP=1. However, the vector magnitude battery compared generated WGSL (separate `*` and `+`) against a CPU oracle using `x.mul_add(x, y * y).sqrt()`. The prior report stated the CPU oracle was "aligned to candidate test using `x.mul_add(x, y*y)`", which was misleading and could cause future agents to over-trust vector magnitude exactness under an unproven FMA alignment claim.

---

## Corrected Oracle Policy

| Oracle | Definition | Role |
|--------|------------|------|
| **Oracle A (shader-order, primary)** | `sum_shader_order = (x * x) + (y * y); cpu_shader_order = sum_shader_order.sqrt()` | Primary comparator for GPU vs CPU; drives vector magnitude classification |
| **Oracle B (FMA diagnostic)** | `sum_fma = x.mul_add(x, y * y); cpu_fma = sum_fma.sqrt()` | Diagnostic only; recorded separately; must not be described as aligned unless WGSL intentionally emits FMA-equivalent behavior (it does not) |

Generated WGSL does not emit `mul_add`. WGSL compiler fusion on this backend is not guaranteed and is not assumed.

---

## Direct Scalar Results (unchanged)

| Metric | Value |
|--------|-------|
| Tested cases | 14 |
| Exact cases | 12 |
| Max ULP (GPU vs `x.sqrt()`) | 1 |
| Classification | `ApproximateJitOnly` |

---

## Euclidean Magnitude Results

| Metric | Shader-order (primary) | FMA diagnostic |
|--------|------------------------|----------------|
| Tested cases | 8 | 8 |
| Exact cases | 8 | 8 |
| Max ULP | 0 | 0 |
| Classification (primary only) | `ExactDeterministicCandidate` | _(diagnostic, not used for overall class)_ |

---

## Gradient Magnitude Results

| Metric | Shader-order (primary) | FMA diagnostic |
|--------|------------------------|----------------|
| Tested cases | 8 | 8 |
| Exact cases | 8 | 8 |
| Max ULP | 0 | 0 |
| Classification (primary only) | `ExactDeterministicCandidate` | _(diagnostic, not used for overall class)_ |

On this backend/corpus, shader-order and FMA diagnostic oracles agree (max ULP=0 for both). This does not prove FMA alignment; it only shows the tested inputs did not expose order divergence.

---

## Final Classification

| Surface | Classification |
|---------|----------------|
| Direct scalar | `ApproximateJitOnly` |
| Euclidean magnitude (shader-order primary) | `ExactDeterministicCandidate` |
| Gradient magnitude (shader-order primary) | `ExactDeterministicCandidate` |
| **Overall native sqrt candidate** | **`ApproximateJitOnly`** |

Conservative rule: overall classification is the weakest of scalar and vector (shader-order primary) results.

---

## Prior Exact-Vector Claim Assessment

The prior report's claim that vector magnitudes were "exact" **still holds under the corrected shader-order primary oracle** on this platform (max ULP=0, 8/8 exact for both Euclidean and gradient corpora). What changed is **evidence honesty**: exactness is now proven against the same operation order the generated WGSL expresses, not against an FMA oracle that was never emitted. The misleading "aligned via `mul_add`" wording is withdrawn. Overall classification remains `ApproximateJitOnly` because direct scalar `sqrt` is not bit-exact (max ULP=1).

---

## Tests Run and Results

```
cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
```

**Result:** 8 passed (including new `jit_sqrt_vector_oracle_order_is_explicit`).

Key stdout:

```
direct_scalar: tested=14, exact=12, max_ulp=1, classification=ApproximateJitOnly
euclidean_magnitude: tested=8, shader_order exact=8/max_ulp=0, fma_diagnostic exact=8/max_ulp=0, primary_classification=ExactDeterministicCandidate
gradient_magnitude: tested=8, shader_order exact=8/max_ulp=0, fma_diagnostic exact=8/max_ulp=0, primary_classification=ExactDeterministicCandidate
sqrt_candidate_final_classification=ApproximateJitOnly (scalar max_ulp=1, magnitude shader_order max_ulp=0, magnitude fma_diagnostic max_ulp=0)
oracle_policy: primary=shader_order max_ulp=0 class=ExactDeterministicCandidate; diagnostic=fma max_ulp=0
```

```
cargo test -p simthing-driver --test phase_m_jit_evaleml_wgsl_prototype -- --nocapture
```

**Result:** 6 passed.

```
cargo test -p simthing-driver --test phase_m_eml_gadget_runtime_execution_gate -- --nocapture
```

**Result:** 6 passed.

```
cargo check --workspace
```

**Result:** PASS.

---

## Scans Run and Results

```
rg "sqrt\(" crates/simthing-gpu/src/shaders crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs crates/simthing-driver/tests/phase_m_jit_evaleml_wgsl_prototype.rs
```

**Result:** no `sqrt(` in baseline runtime shaders; `sqrt(` appears only in M-JIT-SQRT-0 candidate test context.

```
rg "mul_add|shader-order|shader order|fma|fused|ApproximateJitOnly|ExactDeterministicCandidate|RejectedDeferred" crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs docs/tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md docs/tests/phase_m_jit_sqrt_candidate_battery_test_results.md
```

**Result:** R1 test file and report clearly distinguish shader-order primary oracle from FMA diagnostic; classification enums present.

```
rg "production sqrt admission|new production EML opcode|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|simthing-sim.*Personality|simthing-sim.*Memory|chained OrderBand|automatic snapshot" crates docs
```

**Result:** guardrail/deferred context only; no new production/default wiring from this pass.

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD" crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs crates/simthing-gpu/src/shaders docs/tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md
```

**Result:** test file lists terms only in `FORBIDDEN_SEMANTIC_TERMS` guardrail; generated WGSL remains semantic-free; no matches in baseline shaders.

(Scans executed with ripgrep-backed grep; shell `rg` binary unavailable in this environment PATH.)

---

## Transient-Log Cleanup Result

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

**Result:** historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No production sqrt admission, no sqrt in baseline accumulator_op.wgsl, no semantic WGSL, no production JIT wiring, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-SQRT-0 R1 corrects vector oracle order evidence while keeping native sqrt classified explicitly; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
