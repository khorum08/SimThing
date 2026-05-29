# Phase M-JIT-GRAD-1 — Observer + Exact Formula Fusion — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `151c8b0f24d93800645708231287dc03bf5e8f67` (M-JIT-GRAD-0 R1 merge on `master`)  
**Final commit SHA:** `34a3252` (branch `phase-m-jit-grad-1-observer-formula-fusion`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT observer+formula fusion proof (V7.7 §5)  
**Decision:** **IMPLEMENTED — test-only GPU-resident observer+exact-formula fusion prototype**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected:

- `docs/tests/phase_m_jit_grad0_spatial_observer_test_results.md`
- `docs/tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md`
- `docs/tests/phase_m_jit_evaleml_wgsl_prototype_test_results.md`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md`
- `crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs`
- `crates/simthing-driver/tests/phase_m_jit_evaleml_wgsl_prototype.rs`
- `docs/workshop/mapping_current_guidance.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/invariants.md`

Confirmed before edits:

1. `dx`, `dy`, `descent_x`, `descent_y` are exact-authoritative (M-JIT-GRAD-0 R1).
2. `mag2` is approximate/diagnostic only and must not be used as exact score input.
3. Observer shader remains semantic-free.
4. M-JIT-0 exact subset (`LITERAL_F32`/`MUL`/`ADD`/`SUB`/`RETURN_TOP`) supports a small scoring formula.
5. Fused observer+score proof can remain test-only.
6. No production scheduler/caching/default wiring required.

---

## Fused Shader Design

**Option B lite:** test-local emitter injects exact-subset score expression into static observer WGSL.

One GPU compute pass per run:

1. Read `fields` + `observers`.
2. Compute clamp-boundary finite-difference `dx`, `dy`, `descent_x`, `descent_y`.
3. Compute exact-subset score via `fma(w0, descent_x, fma(w1, descent_y, bias))`.
4. Write `ObserverScoreOutput` (no `mag2` field).

Bindings: uniform `FusionParams`, storage `fields`, `observers`, `outputs`.

---

## Exact Score Formula

```text
score = fma(w0, descent_x, fma(w1, descent_y, bias))
```

Weights (bitcast literals, M-JIT-0 style):

| Param | Value |
|-------|-------|
| `w0` | `0.65` |
| `w1` | `0.35` |
| `bias` | `0.125` |

CPU oracle uses matching `f32::mul_add` nesting for bit-exact parity with WGSL `fma()`.

**Approximate `mag2` excluded:** fused output struct has no `mag2`; score expression references only `descent_x`/`descent_y`.

---

## 10,000 Observer Fused Batch Result

| Metric | Value |
|--------|-------|
| Observers | 10,000 |
| Grid | 128×128, `n_dims=4` |
| **Dispatch count** | **1** |
| Workgroups | 157 (`workgroup_size=64`) |
| Elapsed (local GPU, incl. readback) | ~3.5 ms |
| Sampled oracle check | 63 indices; bit-exact for `dx`/`dy`/descent/score |

---

## CPU/GPU Oracle Parity Results

### Small grid (8×8, 11 observers)

Bit-exact CPU/GPU match for `dx`, `dy`, `descent_x`, `descent_y`, `score` (all 11 observers).

### 10,000 observer batch (sampled subset)

Bit-exact for all exact-authoritative fields and `score` on sampled indices.

---

## `sqrt` Exclusion

- Fused WGSL contains **no `sqrt(`**.
- Score uses only exact-authoritative descent outputs; no native `sqrt`, no `mag2`.

---

## Tests Run and Results

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
rg "sqrt\(" crates/simthing-driver/tests/phase_m_jit_grad1_observer_formula_fusion.rs crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs crates/simthing-gpu/src/shaders crates/simthing-driver/tests/phase_m_jit_sqrt_candidate_battery.rs
```

**Result:** no `sqrt(` in GRAD-1 fused path, GRAD-0 observer path, or baseline runtime shaders; `sqrt(` only in M-JIT-SQRT candidate test context.

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-driver/tests/phase_m_jit_grad1_observer_formula_fusion.rs crates/simthing-gpu/src/shaders docs/tests/phase_m_jit_grad1_observer_formula_fusion_test_results.md
```

**Result:** fused WGSL semantic-free; forbidden terms only in guardrail context.

```
rg "mag2|ApproximateJitOnly|diagnostic|exact-authoritative|score|observer formula|fused observer" crates/simthing-driver/tests docs/tests/phase_m_jit_grad1_observer_formula_fusion_test_results.md docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md
```

**Result:** GRAD-1 score excludes `mag2`; docs preserve GRAD-0 R1 `mag2` diagnostic-only status.

```
rg "production JIT|observer scheduling|observer caching|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|simthing-sim.*Personality|simthing-sim.*Memory|chained OrderBand|automatic snapshot|atlas|M-4A|ActiveOnlyExperimentalNoHalo|source_mask|source identity" crates docs
```

**Result:** guardrail/deferred context only; no new production/default wiring.

```
rg "10000|10,000|observer score|formula fusion|dispatch_workgroups|workgroup_size|dx|dy|descent_x|descent_y|score" crates/simthing-driver/tests docs/tests/phase_m_jit_grad1_observer_formula_fusion_test_results.md
```

**Result:** test and report document fused observer+score path, 10,000 scale, dispatch count=1.

---

## Transient-Log Cleanup Result

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

**Result:** historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-driver/tests/phase_m_jit_grad1_observer_formula_fusion.rs`
- `docs/tests/phase_m_jit_grad1_observer_formula_fusion_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no sqrt in the fused exact observer path, no approximate mag2 used as exact score input, no production JIT wiring, no production observer scheduling/caching, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-GRAD-1 is a test-only GPU-resident observer+exact-formula fusion proof; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
