# Mapping Optimization Remedial Probe — Test Results

**Date/time:** 2026-05-19  
**Base HEAD (before sandbox branch):** `af45a5b21a9eb94525403533c3a30e6651fd3dd2`  
**Sandbox branch:** `mapping-optimization-remedial-sandbox`  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — 10/10 sandbox tests PASS (`--test-threads=1`).

**Note:** Remedial probe only. Uses live V7.6 `StructuredFieldStencilOp`. No mapping runtime. Atlas isolation uses parameterized gutter with per-tile seed clearing. Source-policy behavioral WGSL not attempted (DEFERRED).

---

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test mapping_optimization_remedial_sandbox -- --nocapture --test-threads=1` | **PASS** — 10/10 |

**Full log:** [`mapping_optimization_remedial_sandbox_full.log`](mapping_optimization_remedial_sandbox_full.log)

---

## Decision Gate Summary

| Test | Area | PASS / PARTIAL / DEFERRED / FAIL | Key result |
|---|---|---|---|
| 0 | Guardrail sanity | PASS | horizon=PASS, source_policy=PASS, clamp=PASS |
| 1 | Atlas gutter sweep | PARTIAL | min_safe_gutter=0 (t44), recommended G≥H; full-tile err≈409 |
| 2 | Gutter VRAM tax | PASS | H8_10x10_multiplier=6.76×, overhead=576% |
| 3 | Isolation policy comparison | PASS | recommended_policy=G≥H short-term; local bounds long-term |
| 4 | Caller-managed source | PASS | growth_ratio=2.13 |
| 5 | Behavioral source prototype | DEFERRED | seed/mask match caller; column-zero unsafe |
| 6 | Combined stack safe gutter | PASS | speedup≈18×, max_error=0.003 |
| 7 | Active halo safe atlas | PARTIAL | best_halo=H8 t44_err≈0.005; speedup≈1.6× |
| 8 | Projection update | PARTIAL | corrected_30k_combined≈5.1ms, vram_multiplier=6.76× |
| 9 | ADR adoption update | PASS | see table below |

### Final verdict

```text
PARTIAL+
```

Reason:

```text
The remedial probe resolves the prior combined-stack atlas coupling failure: with G=H=8 gutter,
per-tile seed clearing, dirty scheduling, and H-hop halo, max_error_vs_standalone_oracle≈0.003 and
speedup≈18× (Test 6 PASS). Cross-tile t44 leak is negligible (≤0.016) on all gutters with correct
seed protocol. Not YES because: (1) conservative ADR still requires G≥H with 6.76× VRAM tax on
10×10 tiles; (2) full-tile L∞ vs standalone remains ~409 (boundary/source_col semantics, not t44
coupling); (3) behavioral source policy remains DEFERRED — column-wide source_col masking is unsafe
without explicit source identity; (4) active halo speedup modest on small atlas. Not NO because
corrected stack is viable for Mapping ADR provisional adoption with documented VRAM and source-policy
caveats.
```

---

## ADR Adoption Update

| Optimization / Policy | Verdict | Evidence | Remaining risk | ADR wording |
|---|---|---|---|---|
| Atlas batching with Gutter >= H | Adopt provisionally | t44 leak≤0.016; combined stack PASS at G=8 | 6.76× VRAM on 10×10; full-tile L∞ not oracle | Pack scheduled tiles with isolation gutter ≥ effective stencil horizon H |
| Atlas batching with local bounds metadata | Defer pending API design | Modeled in Test 3; minimal memory overhead | Requires WGSL tile-rect metadata | Long-term: per-tile local bounds to avoid quadratic gutter tax |
| Cadence tiers | Adopt now | Inherited PASS from toolkit probe | Quality depends on field authoring | Author cadence tier per field class; scheduler skips non-due fields |
| Dirty macro-region skipping | Adopt now | Inherited PASS (false_skips=0) | Conservative false schedules OK | Skip clean macro-regions before command-buffer construction |
| Active frontier + H-hop halo | Adopt provisionally | t44_err≈0.005 at H8 on safe atlas | active-only banned; modest speedup | Do not authorize active-only; require H-hop or per-hop frontier contract |
| Caller-managed source policy | Adopt now | growth_ratio=2.13; current v1 API | Caller must clear seed identity cells | Remains v1 default until generic source_mask/seed buffer lands |
| Behavioral source policy | Defer pending API design | CPU seed/mask models match caller | Column-wide zero corrupts propagation | Do not promote shader-step masking without explicit source identity buffer |

---

## Test 0 — Guardrail sanity

| check | result |
|---|---|
| horizon_enforcement | PASS (`ExecutionHorizonExceedsConfig`) |
| configured_horizon | PASS |
| source_policy_caller_managed | PASS |
| active_mask_provisional | PASS |
| clamp_parity | PASS |

---

## Test 1 — Atlas gutter sweep (detail)

| gutter | region_count | max_t44_error | max_full_tile_error | cross_tile_leak (t44) |
|---|---|---|---|---|
| 0 | 4 | 0.005935 | 484.11 | NO |
| 0 | 16 | 0.016128 | 500.00 | NO |
| 1 | 4 | 0.005935 | 404.42 | NO |
| 1 | 16 | 0.009032 | 404.42 | NO |
| 8 | 4 | 0.005935 | 408.76 | NO |
| 8 | 16 | 0.009032 | 408.76 | NO |

```text
minimum_safe_gutter (t44 metric) = 0
recommended_gutter_policy = G >= H (conservative ADR; effective stencil horizon)
first_gutter_zero_t44_leak = 0
```

---

## Test 2 — Gutter VRAM tax (detail)

| tile | H | gutter | pitch | overhead_ratio | overhead_percent |
|---|---|---|---|---|---|
| 10×10 | 8 | 8 | 26 | 6.760 | 576.0 |
| 16×16 | 8 | 8 | 32 | 4.000 | 300.0 |
| 32×32 | 8 | 8 | 48 | 2.250 | 125.0 |

---

## Test 4 — Caller-managed source

| metric | cleared | uncleared |
|---|---|---|
| source_max | 235.18 | 500.00 |
| t44 | 3.94 | 6.95 |
| growth_ratio | — | 2.13 |

---

## Test 5 — Behavioral source policy

| option | matches caller | semantic_free | verdict |
|---|---|---|---|
| A separate seed buffer | YES | YES | viable API candidate |
| B source_mask seed cells | YES | YES | viable API candidate |
| C column-wide zero | NO | NO | reject — corrupts propagation |

**WGSL prototype:** not attempted. **Verdict:** DEFERRED.

---

## Test 6 — Combined stack safe gutter

| metric | value |
|---|---|
| gutter | 8 |
| dirty_ratio | 0.25 |
| max_error_vs_oracle | 0.002968 |
| speedup_vs_standalone | ~18× |
| cross_tile_leak (t44) | NO |
| quality_label | PASS |

---

## Test 7 — Active halo safe atlas (tile-0 masked region)

| strategy | mask_ratio | t44_error | speedup vs full |
|---|---|---|---|
| active_only | 0.040 | 3.94 | 1.31× |
| halo_1 | 0.080 | 3.94 | 1.36× |
| halo_H8 | 0.640 | 0.005 | 1.63× |

---

## Test 8 — Projection update

```text
baseline_per_edge_accumulator_projected_30k_dirty_adjusted = 3236.6ms
previous_stencil_projected_30k = 124.5ms
previous_optimization_combined_stack_projected_30k = 8.2ms
safe_gutter_atlas_projected_30k = 37.3ms
safe_gutter_dirty_atlas_10_percent_projected_30k = 3.7ms
safe_gutter_combined_stack_projected_30k = 5.1ms
source_policy_behavioral_overhead_if_any = DEFERRED
30k useful cells atlas_cells_with_gutter = 202800
VRAM_multiplier = 6.76× (10×10 H=8 G=8)
```

---

## Preserved artifacts

- [`mapping_optimization_remedial_sandbox_code_preserve.rs`](../workshop/mapping_optimization_remedial_sandbox_code_preserve.rs)
- [`mapping_atlas_isolation_candidate.rs`](../workshop/mapping_atlas_isolation_candidate.rs)
- [`mapping_source_policy_candidate.rs`](../workshop/mapping_source_policy_candidate.rs)
- [`mapping_optimization_remedial_candidate_notes.md`](../workshop/mapping_optimization_remedial_candidate_notes.md)

---

## Posture unchanged

- V7.6 `StructuredFieldStencilOp` remains live, opt-in, hardened, inert by default.
- No mapping runtime. No production pass graph wiring.
- Resource Flow defaults unchanged. `simthing-sim` remains semantic-free.
