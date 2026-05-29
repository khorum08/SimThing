# Phase M-JIT-GRAD-0 — GPU-Resident Batched Spatial Field Observer — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `13c39c66d2be9adc226d2b9db63e234b58b0177c` (M-JIT-SQRT-0 R1 merge on `master`)  
**Final commit SHA:** `2264426` (branch `phase-m-jit-grad-0-spatial-observer`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT spatial-observer proof (V7.7 §5)  
**Decision:** **IMPLEMENTED — test-only GPU-resident batched spatial field observer prototype**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected:

- `docs/tests/phase_m_jit_evaleml_wgsl_prototype_test_results.md`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_test_results.md`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md`
- `docs/tests/phase_m_m5e_gradient_scarcity_opportunity_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/invariants.md`
- `crates/simthing-gpu/src/structured_field_stencil.rs`
- `crates/simthing-gpu/src/shaders/structured_field_stencil.wgsl`
- `crates/simthing-driver/tests/phase_m_jit_evaleml_wgsl_prototype.rs`
- `crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs`

Confirmed before edits:

1. Observers can sample a dense flat field buffer with generic `(x, y, source_col)` coordinates (same flat layout as structured field stencil).
2. One dispatch can cover ≥10,000 observers via `@compute @workgroup_size(64)` and `dispatch_workgroups(ceil(n/64), 1, 1)`.
3. Central-difference gradients `dx = 0.5*(east-west)`, `dy = 0.5*(south-north)` require no `sqrt`.
4. Output buffer can store `dx`, `dy`, `mag2`, `descent_x`, `descent_y`.
5. CPU oracle parity is feasible for a deterministic subset with clamp boundary policy.
6. Shader can remain semantic-free (generic `fields`/`observers`/`outputs` names only).
7. Prototype can remain test-only with no production mapping or `simthing-sim` wiring.

---

## Chosen Boundary Policy

**Clamp boundary** (matches existing `StructuredFieldStencilBoundaryMode::Clamp` in `structured_field_stencil.wgsl`): out-of-bounds neighbor samples clamp to the nearest valid cell. Same policy in CPU oracle and WGSL.

---

## Observer Buffer Design

| Buffer | Layout |
|--------|--------|
| `fields` | Flat `width × height × n_dims` f32 array (row-major cells) |
| `observers` | `ObserverInput { x, y, source_col, _pad }` per observer |
| `outputs` | `ObserverOutput { dx, dy, mag2, descent_x, descent_y, _pad }` per observer |
| `params` (uniform) | `width`, `height`, `n_dims`, `n_observers`, `boundary_mode` |

Gradient/descent contract:

```text
dx = 0.5 * (east - west)
dy = 0.5 * (south - north)
descent_x = -dx
descent_y = -dy
mag2 = dx*dx + dy*dy   // no sqrt
```

---

## Dispatch Design

- Static semantic-free WGSL fixture (Option A) embedded in test file.
- `@compute @workgroup_size(64, 1, 1)` — one thread per observer.
- **Dispatch count: 1** compute pass per run.
- 10,000 observers → **157 workgroups** (`ceil(10000/64)`).

---

## 10,000 Observer Batch Result

| Metric | Value |
|--------|-------|
| Observers | 10,000 |
| Grid | 128×128, `n_dims=4`, `source_col=0` |
| Dispatch count | **1** |
| Workgroups | 157 |
| Workgroup size | 64 |
| Elapsed (local GPU, incl. readback) | ~2.8 ms |
| Oracle sample check | 64 sampled indices (first/last/spaced); `dx`/`dy`/`descent_*` bit-exact; `mag2` within ≤1 ULP of `dx*dx+dy*dy` GPU self-consistency |

No CPU-side per-observer dispatch or planning loop beyond test oracle verification.

---

## CPU/GPU Oracle Parity Results

### Small grid (8×8, 11 observers: center/corners/edges)

| Field | Result |
|-------|--------|
| `dx`, `dy`, `descent_x`, `descent_y` | Bit-exact CPU/GPU match (all 11 observers) |
| `mag2` | GPU self-consistent within ≤1 ULP of `dx*dx+dy*dy` |

### 10,000 observer batch (sampled subset)

Same parity policy: gradient/descent fields bit-exact; `mag2` ≤1 ULP GPU self-consistency (GPU mul/add rounding for `mag2` can differ from CPU recomputation from read-back `dx`/`dy` by 1 ULP on some inputs).

---

## `sqrt` Exclusion / Squared-Magnitude Rationale

- Observer WGSL contains **no `sqrt(`**.
- `mag2 = dx*dx + dy*dy` supports descent direction and relative ranking without Euclidean magnitude.
- Native WGSL `sqrt` remains `ApproximateJitOnly` (M-JIT-SQRT-0) and is not used in this exact path.

---

## Tests Run and Results

```
cargo test -p simthing-driver --test phase_m_jit_grad0_spatial_observer -- --nocapture
```

**Result:** 5 passed.

```
cargo test -p simthing-driver --test phase_m_jit_evaleml_wgsl_prototype -- --nocapture
```

**Result:** 6 passed.

```
cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
```

**Result:** 8 passed.

```
cargo test -p simthing-driver --test phase_m_eml_gadget_runtime_execution_gate -- --nocapture
```

**Result:** 6 passed.

```
cargo check --workspace
```

**Result:** PASS.

M-5E fixture not re-run (mapping fixture helpers untouched).

---

## Scans Run and Results

```
rg "sqrt\(" crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs crates/simthing-gpu/src/shaders crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs
```

**Result:** no `sqrt(` in M-JIT-GRAD-0 observer WGSL; no `sqrt(` in baseline runtime shaders; `sqrt(` appears only in M-JIT-SQRT candidate test (assertions + generated candidate WGSL).

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs crates/simthing-gpu/src/shaders docs/tests/phase_m_jit_grad0_spatial_observer_test_results.md
```

**Result:** test file lists terms only in `FORBIDDEN_SEMANTIC_TERMS` guardrail; observer WGSL and baseline shaders semantic-free; report mentions terms only as guardrail context.

```
rg "production JIT|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|simthing-sim.*Personality|simthing-sim.*Memory|chained OrderBand|automatic snapshot|atlas|M-4A|ActiveOnlyExperimentalNoHalo|source_mask|source identity" crates docs
```

**Result:** guardrail/deferred context only; no new production/default wiring from this pass.

```
rg "10000|10,000|observer|spatial observer|descent|mag2|dispatch_workgroups|workgroup_size|dx|dy" crates/simthing-driver/tests docs/tests/phase_m_jit_grad0_spatial_observer_test_results.md
```

**Result:** test and report document batched observer path, 10,000 scale, `mag2`, `dx`/`dy`, dispatch count=1, workgroup_size=64.

(Scans executed with ripgrep-backed grep; shell `rg` binary unavailable in this environment PATH.)

---

## Transient-Log Cleanup Result

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

**Result:** historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs`
- `docs/tests/phase_m_jit_grad0_spatial_observer_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no sqrt in the exact observer path, no production JIT wiring, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-GRAD-0 is a test-only GPU-resident spatial observer proof for batched field-gradient/descent sampling; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
