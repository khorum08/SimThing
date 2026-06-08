# FIELD_POLICY tensor/stencil WGSL refinement probe — test results

**Date/time:** 2026-05-19  
**Base HEAD (before sandbox branch):** `aa001b1` — FIELD_POLICY tensor/stencil WGSL revert doc update  
**Sandbox merge SHA:** `be564a3` (PR #208)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — 12/12 sandbox tests PASS (run with `--test-threads=1`).

**Note:** Horizon tests use cluster seed → one propagation hop → source zero unless source policy overrides. Directed setups use orientation-compatible `directed_mode` (NW for top-left → SE travel; SE for bottom-right → NW travel). Parent EML uses order band 0 (Sum) then band 1 (EvalEML) with aggression/risk populated on parent slot columns.

---

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test field_policy_tensor_stencil_refinement_sandbox -- --nocapture --test-threads=1` | **PASS** — 12/12 |

**Full log:** [`archive/field_policy/field_policy_tensor_stencil_refinement_sandbox_full.log`](archive/field_policy/field_policy_tensor_stencil_refinement_sandbox_full.log)

---

## Test 0 — Capability sanity

```text
wgsl_compile=YES
pingpong_buffers=YES
dispatch_10x10=YES
dispatch_1024=YES
column_aware_reduction_fixture=YES
parent_eval_eml=YES
```

---

## Test 1 — Long-horizon stability (selected)

| Operator | H | t44 | max | l1 | blowup | dir |
|---|---:|---:|---:|---:|:---:|---|
| normalized_stencil | 8 | 3.94 | 235 | 2806 | NO | correct |
| normalized_stencil | 24 | 301822 | 928102 | 18869124 | NO | correct |
| normalized_stencil | 32 | 33659036 | 68018800 | 1681799936 | YES | correct |
| source_capped_normalized | 8 | 3.94 | 235 | 2806 | NO | correct |
| source_capped_normalized | 16 | 500 | 500 | 25227 | NO | none |
| source_capped_normalized | 24 | 500 | 500 | 47422 | NO | none |
| normalized_horizon_cap_H8 | 16–32 | 3.94 | 235 | 2806 | NO | correct |
| decayed_normalized_mid | 8 | 0.0001 | 0.6 | 5 | NO | none |

**Pass:** PARTIAL — `source_capped_normalized` and `normalized_horizon_cap_H8` bound late-horizon growth; plain normalized still amplifies at H=24+.

---

## Test 2 — Ping-pong correctness

| Grid | H | gpu_cpu_max_error |
|---|---:|---:|
| 3×3 | 1–8 | 0.000000 |
| 10×10 | 1–8 | 0.000000 |

```text
data_race_detected=NO
stable_readwrite=YES
```

**Pass:** PASS

---

## Test 3 — Directed compatible setup

| Setup | H | t44 | dir | first_directional_H |
|---|---:|---:|---|---:|
| NW_to_SE_top_left | 8 | 12777.5 | correct | 8 |
| NW_to_SE_top_left | 16 | 403270 | partial | — |
| SE_to_NW_bottom_right | 8 | 3908.4 | correct | 8 |

Prior NSEW+directed failure was harness mismatch. Directed works with compatible source/kernel orientation; H=16 still amplifies.

**Pass:** PARTIAL

---

## Test 4 — Source injection policy (source_capped_normalized)

| Policy | H=8 t44 | H=24 max | amplification |
|---|---:|---:|:---:|
| one_shot_zero | 3.94 | 500 | NO |
| persistent | 0 | 138 | NO |
| every_4 | 0 | 255 | NO |

**Pass:** PARTIAL — one_shot_zero preserves H=8 gradient; cap bounds H=24.

---

## Test 5 — Column-aware parent EML

```text
parent_threat_total=2805.88
parent_resource_pressure=100.0 (grid cells @ 1.0)
aggression_A=0.2  urgency_A=571.18
aggression_B=0.9  urgency_B=2535.29
urgency_ratio=4.44
reduction_ms=1.38
stencil_ms=7.69
total_hybrid_ms=9.07
```

**Pass:** PASS — urgency nonzero and personality-sensitive when parent columns populated and EvalEML runs on band 1 after Sum.

---

## Test 6 — EML admission

| Class | legacy | C8 | finding |
|---|---|---|---|
| field_pressure | NO | YES | A (designer policy) |
| field_urgency | NO | YES | A |
| field_decay | NO | YES | A |
| bounded_field_update | NO | YES | A |
| conversion_rate | YES | YES | E (alias sufficient) |

**Pass:** PASS — runtime C-8 path sufficient; legacy whitelist rejection is admission policy only.

---

## Test 7 — Active mask + ping-pong (source_capped, H=8)

| active_ratio | speedup | max_active_err |
|---:|---:|---:|
| 0.05 | 1.54× | 131.4 |
| 0.10 | 1.51× | 75.5 |
| 0.25 | 1.61× | 3.73 |
| 0.50 | 1.35× | 0.002 |
| 1.00 | 8.86× | 0.000 |

**Pass:** PARTIAL — meaningful speedup at ≥25% active; edge artifacts at very sparse ratios on this micro-benchmark.

---

## Test 8 — Refined cost scaling (source_capped_normalized, one_shot_zero)

| cells | H | wall_ms | projected_30k |
|---:|---:|---:|---:|
| 100 | 8 | 3.98 | 1192.8 |
| 1024 | 8 | 3.10 | 90.8 |
| 4096 | 8 | 3.56 | 26.1 |
| 16384 | 8 | 7.10 | 13.0 |

**Pass:** PARTIAL — large-grid projection strong; 10×10 H=8 measure higher than prior normalized probe due to ping-pong + cap path overhead.

---

## Test 9 — Generality review

```text
Does WGSL know about maps? NO
Does WGSL know about factions? NO
Does WGSL know about AI? NO
Does WGSL operate on flat buffers + dimensions + columns + kernel weights? YES
Does WGSL require simthing-sim awareness? NO
Does WGSL require Resource Flow default-on? NO
Does production adoption require a new generic runtime API? PARTIAL
Does production adoption require designer/RON admission rules? PARTIAL
general_tensor=YES
```

**Pass:** PASS

---

## Decision Gate Summary

| Test | Area | PASS / PARTIAL / DEFERRED / FAIL | Key result |
|---|---|---|---|
| 0 | Capability sanity | PASS | pingpong=YES parent_eml=YES |
| 1 | Long-horizon stability | PARTIAL | best=source_capped; H8 t44=3.94; H24 max=500 |
| 2 | Ping-pong correctness | PASS | max_error_H8=0.0 |
| 3 | Directed setup | PARTIAL | first_directional_H=8 both orientations at H=8 |
| 4 | Source policy | PARTIAL | best=one_shot_zero + source_cap |
| 5 | Column-aware parent EML | PASS | urgency_A=571 urgency_B=2535 ratio=4.44 |
| 6 | EML admission | PASS | finding=A designer policy; C8=YES |
| 7 | Active mask + ping-pong | PARTIAL | speedup@10%=1.51× speedup@25%=1.61× |
| 8 | Refined cost scaling | PARTIAL | projected_30k=1192.8 (10×10 H=8) |
| 9 | Generality review | PASS | general_tensor=YES |

### Overall verdict

```text
PARTIAL
```

Reason:

```text
Refinement resolves three prior artifact-class objections: (1) ping-pong correctness for H>1 is proven (GPU=CPU oracle); (2) directed stencil works when source orientation matches directed_mode (prior failure was NSEW harness mismatch); (3) parent EvalEML urgency is nonzero and personality-sensitive when threat/resource are reduced into parent columns and aggression/risk are bound before EvalEML on a later order band. Long-horizon amplification is controllable via source_cap or authored H≤8 horizon cap, not via decayed-normalized alone (too weak tactically). Remaining gaps for production ADR: StructuredFieldStencilOp API/ping-pong binding, explicit source injection policy authoring, directed_mode metadata, designer admission for field_* classes, and 10×10 micro-benchmark cost regression vs prior normalized probe (1193ms vs 285ms projected 30k) from ping-pong dispatch overhead — large-grid projections remain favorable (13ms @ 16k cells H=8).
```

### Recommended ADR action

```text
1. Promote generic StructuredFieldStencilOp as a candidate production primitive in the mapping ADR — with explicit stability contracts (source cap and/or horizon cap), ping-pong buffers, optional active mask, and column-aware parent reduction into EML input columns.
```

### Recommended production constraints

```text
recommended_operator = source_capped_normalized (variant 5) or normalized with authored H<=8 horizon cap
recommended_horizon_cap = H<=8 tactical default; H<=16 only with source_cap or decay contract
recommended_source_policy = one_shot_seed_then_zero (no implicit persistent pumping)
recommended_pingpong = YES
recommended_active_mask = PARTIAL (benefit depends on active_ratio; use when >=25% expected)
recommended_parent_binding = column-aware SlotRange Sum into parent threat/resource columns; personality cols on parent; EvalEML on subsequent order band
recommended_eml_admission = C-8 register_formula at runtime; extend designer/RON whitelist for field_* classes (policy-only)
recommended_guardrail_location = designer-RON / scenario authoring for formula classes; runtime for ping-pong + source policy + horizon cap
```

### Cost comparison

```text
per_edge_accumulator_projected_30k_dirty_adjusted = 3236.6ms
previous_stencil_projected_30k = 284.7ms
refined_stencil_projected_30k = 1192.8ms
estimated_speedup_vs_accumulator = 2.7× (10×10 H=8 measure)
estimated_speedup_vs_previous_stencil = 0.24× (10×10 — ping-pong overhead; large-grid 16k cells H=8 projected_30k=13.0ms ≈ 22× vs previous 10×10 normalized measure)
budget_label = MARGINAL
```

---

## Preserve locations

- `docs/workshop/archive/field_policy/field_policy_tensor_stencil_refinement_sandbox_code_preserve.rs`
- `docs/workshop/archive/field_policy/field_policy_tensor_stencil_refinement_prototype.wgsl`
- `docs/workshop/archive/field_policy/field_policy_tensor_stencil_refinement_notes.md`
- Variant WGSL copies (parametric kernel; variant selected by uniform)
- This file and full log under `docs/tests/`
