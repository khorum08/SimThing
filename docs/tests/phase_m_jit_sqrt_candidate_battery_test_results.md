# Phase M-JIT-SQRT-0 — Native WGSL `sqrt` Candidate Battery — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `b6447f5f15e5499e0f22688fe5d29922d3f3f946`  
**Final commit SHA:** `1a1c39c` (implementation commit on `phase-m-jit-sqrt-0-candidate-battery`; squash/merge SHA recorded by PR merge)  
**Lane classification:** Tier-2 JIT sqrt candidate battery (V7.7 §5)  
**Decision:** **IMPLEMENTED — test-only native WGSL `sqrt` candidate battery**  
**Verdict:** **PASS — `ApproximateJitOnly` classification**

---

## Pre-Edit Evaluation Summary

Inspected:

- `docs/tests/phase_m_jit_evaleml_wgsl_prototype_test_results.md`
- `crates/simthing-driver/tests/phase_m_jit_evaleml_wgsl_prototype.rs`
- `docs/invariants.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/workshop/mapping_current_guidance.md`
- `crates/simthing-spec/src/compile/eml_gadget.rs`
- `crates/simthing-gpu/src/accumulator_op/session.rs`
- `crates/simthing-gpu/src/shaders/accumulator_op.wgsl`
- `crates/simthing-core/src/eml_registry.rs` (used because requested path
  `crates/simthing-spec/src/compile/eml_registry.rs` does not exist in this repo)

Confirmed before edits:

1. M-JIT-0 emits exact-bit float literals with `bitcast<f32>(<u32 bits>)`.
2. Current JIT subset rejects unsupported ops (structured rejection).
3. `sqrt` is absent from baseline runtime shaders (`accumulator_op.wgsl`).
4. No production opcode/admission path currently authorizes `sqrt`.

---

## Candidate Formulas Tested

- **Candidate A (direct scalar):** `sqrt(x)`
- **Candidate B (Euclidean 2D):** `sqrt((x * x) + (y * y))`
- **Candidate C (gradient magnitude shape):** generic-column
  `sqrt((col_x * col_x) + (col_y * col_y))`
- **Candidate D (classifier):** explicit exact/approximate/reject classification driven by
  measured ULP distance (no silent relaxation)

---

## Input Corpus

### Direct scalar corpus (`x`)

Admissible tested values:

- `0.0`, `1.0`, `2.0`, `4.0`
- `f32::MIN_POSITIVE`, `1e-20`, `1e-10`
- non-square representatives: `0.2`, `0.3`, `3.1415927`, `10.75`, `12345.678`
- large finite positives: `1e8`, `1e20`

Policy in this battery:

- Negative / NaN / infinity are non-authoritative for exact deterministic admission.
- Subnormals are not part of authoritative exact corpus; this battery uses normal-domain + zero.

### Vector corpus (`x`, `y`) for Euclidean/gradient magnitude

- `(3,4)`, `(0,0)`
- mixed small: `(0.125, 0.0625)`, `(1.25, 2.5)`
- mixed large finite: `(1234, 4321)`, `(1e10, 2e10)`
- non-integer/rounding-edge: `(0.30000004, 0.70000005)`, `(31.5, 0.125)`

---

## Generated WGSL Safety Summary

- Candidate emitter is test-local to
  `crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs`.
- Generated WGSL is deterministic and semantic-free; identifiers are generic
  (`values`, `slot`, `n_dims`, `base`, `col_*`, `tmp_*`, `out_col`).
- `sqrt(` appears only in generated candidate shader text in this test slice.
- No RON gadget IDs or gameplay semantics are emitted.
- No production shader/runtime surfaces were modified to include `sqrt`.

---

## Battery Results

### Direct scalar `sqrt(x)`

- Tested cases: **14**
- Bit-exact matches: **12**
- Max ULP distance: **1**
- Local-platform classification: **`ApproximateJitOnly`**

### Euclidean magnitude `sqrt(x*x + y*y)`

- CPU oracle order aligned to candidate test using `x.mul_add(x, y*y)` then `sqrt`.
- Tested cases: **8**
- Bit-exact matches: **8**
- Max ULP distance: **0**
- Local-platform classification: **`ExactDeterministicCandidate`** (for this corpus/platform only)

### Gradient magnitude candidate (generic column form)

- Tested cases: **8**
- Bit-exact matches: **8**
- Max ULP distance: **0**
- Local-platform classification: **`ExactDeterministicCandidate`** (for this corpus/platform only)

### Negative-domain behavior

- Negative inputs are explicitly treated as non-authoritative for exact deterministic admission.
- Test enforces they do not promote exact classification.

---

## Bit-Exact and ULP Verdict

- **Bit-exact parity overall:** **not global** for the tested battery due to direct scalar corpus
  non-zero ULP observations.
- **Recorded bound (this platform/corpus):** max ULP = **1**.

**Explicit classification:** **`Approximate JIT-only candidate`**.

Interpretation:

`Native WGSL sqrt is not bit-exact under this battery; it may be useful only for approximate visualization or non-authoritative JIT formulas unless a deterministic software sqrt lowering is implemented.`

---

## Baseline Runtime/Admission Guardrail Status

- `sqrt` remains absent from baseline runtime shader:
  `crates/simthing-gpu/src/shaders/accumulator_op.wgsl`.
- No production EML opcode/admission was added.
- No production JIT/default session wiring was added.

---

## Tests Run and Results

```
cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
```
**Result:** **7 passed; 0 failed**

```
cargo test -p simthing-driver --test phase_m_jit_evaleml_wgsl_prototype -- --nocapture
```
**Result:** **6 passed; 0 failed**

```
cargo test -p simthing-driver --test phase_m_eml_gadget_runtime_execution_gate -- --nocapture
```
**Result:** **6 passed; 0 failed**

```
cargo test -p simthing-spec --test eml_gadget_tier1 -- --nocapture
```
**Result:** **14 passed; 0 failed**

```
cargo test -p simthing-spec --test eml_gadget_tier2_temporal -- --nocapture
```
**Result:** **10 passed; 0 failed**

```
cargo check --workspace
```
**Result:** **PASS** (pre-existing unrelated warnings only)

Broader confidence (optional, unchanged surfaces):

```
cargo test -p simthing-spec --test eml_gadget_tier2_hysteresis -- --nocapture   # 16 passed
cargo test -p simthing-spec --test eml_gadget_tier2_acceleration -- --nocapture # 11 passed
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture # 28 passed
```

---

## Scans Run and Results

Required scan:

```
rg "sqrt\(" crates/simthing-gpu/src/shaders crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs crates/simthing-driver/tests/phase_m_jit_evaleml_wgsl_prototype.rs
```

Result: `sqrt(` appears in the new M-JIT-SQRT-0 candidate test file only; no `sqrt(` in baseline
runtime shaders (including `accumulator_op.wgsl`) and no `sqrt(` in
`phase_m_jit_evaleml_wgsl_prototype.rs`.

Required scan:

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD" crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs crates/simthing-gpu/src/shaders docs/tests/phase_m_jit_sqrt_candidate_battery_test_results.md
```

Result: test/report contain terms only as forbidden/guardrail context; no matches in
`crates/simthing-gpu/src/shaders`.

Required scan:

```
rg "production JIT|default SimSession mapping|production economy→mapping bridge|ResourceEconomySpec.*mapping|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|simthing-sim.*Personality|simthing-sim.*Memory|chained OrderBand|automatic snapshot" crates docs
```

Result: guardrail/deferred context only; no new production/default wiring.

Required scan:

```
rg "NativeSqrt|ApproximateJitOnly|ExactDeterministic|sqrt|new EML opcode|GradientXY|source_mask|source identity|atlas|M-4A|ActiveOnlyExperimentalNoHalo" crates docs
```

Result: `NativeSqrt`/`ApproximateJitOnly` appear only in this test/report context; no production
opcode/admission additions. Existing deferred items remain deferred.

(Scans executed with the ripgrep-backed `rg` tool interface; shell `rg` binary is unavailable in
this environment PATH.)

---

## Transient-Log Cleanup Result

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

**Result:** historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No production sqrt admission, no sqrt in baseline accumulator_op.wgsl, no semantic WGSL, no
production JIT wiring, no default mapping wiring, no simthing-sim Gadget/Personality/Memory
semantics, no new production EML opcode, no chained scheduling, no automatic snapshot/copy
scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge;
M-JIT-SQRT-0 is a test-only native WGSL sqrt candidate battery with explicit exact/approximate/reject
classification; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
