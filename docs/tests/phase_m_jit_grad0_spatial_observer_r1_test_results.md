# Phase M-JIT-GRAD-0 R1 — Observer `mag2` Determinism Classification — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `a155faf643c09473d118c80bf41faffa0582c1ca` (M-JIT-GRAD-0 merge on `master`)  
**Final commit SHA:** `ebdf9aa` (branch `phase-m-jit-grad-0-r1-mag2-classification`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT remedial evidence pass (V7.7 §5)  
**Decision:** **REMEDIAL — observer output classification explicit; `mag2` reclassified approximate/diagnostic on batch corpus**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected:

- `crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs`
- `docs/tests/phase_m_jit_grad0_spatial_observer_test_results.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md`
- `crates/simthing-gpu/src/shaders/accumulator_op.wgsl`

Confirmed before edits:

1. Observer shader computes `mag2 = dx * dx + dy * dy`.
2. Tests compare `dx`/`dy`/`descent_*` bit-exactly against CPU oracle.
3. Prior test did not require `mag2` bit-exact CPU/GPU parity (allowed ≤1 ULP GPU self-consistency).
4. Prior report stated `mag2` within ≤1 ULP without explicit classification.
5. Production plan contained a general exact CPU/GPU parity stop condition that could overclaim observer `mag2` exactness.

---

## Found Issue

M-JIT-GRAD-0 correctly proved the batched spatial observer performance path (10,000 observers, one dispatch, semantic-free WGSL, no `sqrt`, clamp boundary, default-off posture). However, `mag2` was checked only for ≤1 ULP GPU self-consistency while active docs implied broader exact CPU/GPU oracle parity. This could cause future agents to treat `mag2` as exact-authoritative observer state.

---

## Output Classification Table

| Output | Role | Small grid (11 observers) | 10k batch sample (63 observers) |
|--------|------|---------------------------|----------------------------------|
| `dx` | exact-authoritative | bit-exact (max ULP=0) | bit-exact (max ULP=0) |
| `dy` | exact-authoritative | bit-exact (max ULP=0) | bit-exact (max ULP=0) |
| `descent_x` | exact-authoritative | bit-exact (max ULP=0) | bit-exact (max ULP=0) |
| `descent_y` | exact-authoritative | bit-exact (max ULP=0) | bit-exact (max ULP=0) |
| `mag2` | shader-order CPU oracle: `cpu.dx*cpu.dx + cpu.dy*cpu.dy` | **exact-authoritative** (11/11, max ULP=0) | **approximate/diagnostic** (57/63 exact, max ULP=1, `ApproximateJitOnly`) |

**Final `mag2` classification (batch corpus, primary):** `ApproximateJitOnly` — bounded diagnostic/ranking hint; not admitted as deterministic authoritative state at production admission surfaces.

Authoritative exact observer outputs for the performance path: **`dx`, `dy`, `descent_x`, `descent_y`**.

---

## Oracle Policy (R1)

| Oracle | Definition | Role |
|--------|------------|------|
| **Primary shader-order CPU `mag2`** | `cpu.dx * cpu.dx + cpu.dy * cpu.dy` from independent field-sampling CPU oracle | Primary comparator for GPU `mag2` classification |
| **Prior ≤1 ULP self-consistency** | GPU `mag2` vs readback `dx`/`dy` | Withdrawn as implicit exactness claim |

---

## 10,000 Observer Proof (unchanged)

- 10,000 observers, **1 dispatch**, semantic-free WGSL, no `sqrt`, clamp boundary — remains green.
- Sampled batch oracle subset (63 indices): exact outputs bit-exact; `mag2` max ULP=1.

---

## Tests Run and Results

```
cargo test -p simthing-driver --test phase_m_jit_grad0_spatial_observer -- --nocapture
```

**Result:** 8 passed (3 new R1 classification tests).

Key stdout:

```
small_grid: mag2_shader_order exact=11/11 max_ulp=0 class=ExactDeterministicCandidate
batch_10000_sample: mag2_shader_order exact=57/63 max_ulp=1 class=ApproximateJitOnly
mag2_r1_classification=ApproximateJitOnly
```

```
cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
```

**Result:** 8 passed.

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
rg "sqrt\(" crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs crates/simthing-gpu/src/shaders crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs
```

**Result:** no `sqrt(` in observer WGSL or baseline runtime shaders; `sqrt(` only in M-JIT-SQRT candidate test context (assertions + generated candidate WGSL).

```
rg "mag2|ExactDeterministicCandidate|ApproximateJitOnly|RejectedDeferred|exact-authoritative|diagnostic|shader-order|shader order" crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs docs/tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md docs/accumulator_op_v2_production_plan.md docs/workshop/mapping_current_guidance.md
```

**Result:** explicit `mag2` classification in tests and R1 report; production plan/guidance amended to state `mag2` approximate/diagnostic on batch corpus.

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|FIELD_POLICY|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs crates/simthing-gpu/src/shaders docs/tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md
```

**Result:** observer WGSL semantic-free; forbidden terms only in guardrail context.

```
rg "production JIT|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|simthing-sim.*Personality|simthing-sim.*Memory|chained OrderBand|automatic snapshot|atlas|M-4A|ActiveOnlyExperimentalNoHalo|source_mask|source identity" crates docs
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

- `crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs`
- `docs/tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no sqrt in the exact observer path, no production JIT wiring, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-GRAD-0 R1 classifies spatial observer outputs honestly and prevents mag2 exactness overclaim; V7.7 / Mapping ADR / FIELD_POLICY GPU-resident default-off posture intact.
