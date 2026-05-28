# SEAD tensor/stencil WGSL prototype probe — test results

**Date/time:** 2026-05-19  
**Base HEAD (before sandbox branch):** `6338789` — SEAD operator toolkit revert HEAD doc update  
**Sandbox merge SHA:** `cd99ff6` (PR #206)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — 10/10 sandbox tests PASS (run with `--test-threads=1`).

**Note:** Horizon tests use NSEW stencil with cluster seed → one propagation hop → source zero (mirrors prior AccumulatorOp setup bands 0–3). `directed_stencil` variant uses south/east neighbors only and requires directed setup — not equivalent to NSEW setup hop.

---

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test sead_tensor_stencil_wgsl_sandbox -- --nocapture --test-threads=1` | **PASS** — 10/10 |

**Full log:** [`sead_tensor_stencil_wgsl_sandbox_full.log`](sead_tensor_stencil_wgsl_sandbox_full.log)

---

## Decision Gate Summary

| Test | Area | PASS / PARTIAL / DEFERRED / FAIL | Key result |
|---|---|---|---|
| 0 | WGSL capability | PASS | compile=YES dispatch_10x10=YES dispatch_1024=YES readback=YES |
| 1 | 3×3 correctness | PASS | gpu_cpu_max_error=0.0 north=south=east=west=80 |
| 2 | 10×10 horizon | PARTIAL | normalized first_directional_H=8 t44=3.94; raw blowup=YES at H≥12; directed=FAIL |
| 3 | Cost scaling | PASS | projected_30k=284.7ms speedup=11.4× vs AccumulatorOp 3236.6ms |
| 4 | Stability | PARTIAL | recommended=normalized_stencil; raw=blowup; decayed=too weak |
| 5 | Hierarchy hybrid | PARTIAL | local_gradient=YES stencil_ms=4.9 reduction_ms=1.9; urgency=0 (parent cols) |
| 6 | Dirty mask | PARTIAL | active_ratio=0.22 speedup=2.35× max_active_err=0.000646 |
| 7 | Generality review | PASS | general_tensor=YES mapping_semantics=NO simthing_sim=NO |

### Overall verdict

```text
PARTIAL
```

Reason:

```text
The prototype WGSL is a general structured field primitive (flat buffers + dimensions + columns + kernel weights; no map/faction/AI semantics). Cost curve materially improves vs per-edge AccumulatorOp: projected 30k ~285 ms (normalized, 10×10 measure) vs 3236.6 ms dirty-adjusted baseline (~11×), scaling to 80–1200× on larger grids (rough projection). normalized_stencil reaches [4][4] with correct gradient at H=8 without blowup. Not YES because: (1) production StructuredFieldStencilOp API/harness integration remains future work; (2) raw/clamped operators unstable or saturate; decayed_normalized too weak for tactical H≤16; directed_stencil fails with NSEW setup; (3) long-horizon normalized still amplifies at H=24 (marginal); (4) hybrid urgency EML needs parent personality columns beyond threat sum; (5) active mask speedup is size/sparsity dependent. Not NO because generality and cost hypotheses are substantively confirmed.
```

### Recommended ADR action

```text
1. Add generic StructuredFieldStencilOp to future mapping ADR — as a candidate production primitive for dense local 2D field propagation, alongside hierarchy-first strategic awareness and AccumulatorOp for non-grid reductions/EML.
```

### Recommended operator

```text
recommended_stencil_operator = normalized_stencil
recommended_kernel_size = 3×3 NSEW (directed SE as optional variant mode with directed setup)
recommended_boundary_mode = zero
recommended_decay = pair alpha/gamma so total gain ≤ 1 for long horizons (decayed_normalized stable but too weak for H=8 tactical; tune gamma upward or use normalized with horizon cap)
recommended_normalization = per-cell neighbor_count normalization (variant 1)
```

### Cost comparison

```text
per_edge_accumulator_projected_30k_dirty_adjusted = 3236.6ms
stencil_projected_30k (normalized, 10×10 H=8 measure) = 284.7ms
estimated_speedup = 11.4×
budget_label = MARGINAL — single-tick projected 30k ~285ms is materially better than AccumulatorOp but still above ideal sub-100ms tactical budget; large-grid projection suggests amortized stencil dispatch is the right cost shape
```

Scaling reference (normalized, H=8 unless noted):

| cells | projected_30k | speedup vs accum |
|---|---|---|
| 100 | 611ms | 5.3× |
| 196 | 241ms | 13.4× |
| 1024 | 40ms | 81× |
| 4096 | 10ms | 321× |
| 16384 (H=4) | 2.6ms | 1230× |

### Generality / constitutionality

```text
general_tensor_primitive = YES
mapping_semantics_embedded = NO
simthing_sim_awareness = NO
new_runtime_api_needed = PARTIAL (StructuredFieldStencilOp harness + ping-pong buffers + cadence integration)
```

---

## Test 2 — Horizon detail (selected)

```text
normalized_stencil H=8:  t44=3.94 grad=(-7.56,-7.56) dir=correct max=235 blowup=NO ms=0.92 dispatches=8
normalized_stencil H=16: t44=2039.61 dir=correct max=13225 blowup=NO
normalized_stencil H=24: t44=301822 dir=correct max=928102 blowup=NO (marginal amplification)

raw_stencil H=8:  t44=55385 dir=correct blowup=NO
raw_stencil H=12: blowup=YES

directed_stencil H=8: t44=0 dir=none (NSEW setup incompatible)

decayed_normalized H=8: t44=0.0004 dir=none (too weak for tactical horizon)
```

Pass criteria assessment: **PARTIAL** — normalized reaches [4][4] by H=8 with correct direction and no blowup at H=8/16.

---

## Test 5 — Hybrid detail

```text
Hierarchy+stencil hybrid:
  local_gradient_correct=YES
  faction_threat=2805.88
  faction_urgency=0.00 (faction slot lacks aggression/resource/risk columns for urgency EML)
  stencil_ms=4.90
  reduction_ms=1.94
  total_ms=6.83
  baseline lateral_H8_ms=21.09 hierarchy_reduction_ms=1.45
```

Stencil + hierarchy reduction is ~3× faster than lateral AccumulatorOp H=8 on 10×10 for this hybrid path.

---

## Test 6 — Dirty mask detail

```text
active_ratio=0.220
unmasked_ms=1.638
masked_ms=0.696
speedup=2.35×
max_active_err=0.000646
```

Mask is expressible in prototype WGSL; measurable speedup at ~22% active cells on 10×10.

---

## Preserved artifacts

- Rust: [`docs/workshop/sead_tensor_stencil_wgsl_sandbox_code_preserve.rs`](../workshop/sead_tensor_stencil_wgsl_sandbox_code_preserve.rs)
- WGSL: [`docs/workshop/sead_tensor_stencil_prototype.wgsl`](../workshop/sead_tensor_stencil_prototype.wgsl)
- Variant copies: `sead_tensor_stencil_*_prototype.wgsl` under `docs/workshop/`
- Notes: [`docs/workshop/sead_tensor_stencil_prototype_notes.md`](../workshop/sead_tensor_stencil_prototype_notes.md)
- This report + full log

Production shader/test/runtime files removed on revert to parked state.
