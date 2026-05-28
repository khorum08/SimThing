# Mapping Optimization Toolkit Probe — Test Results

**Date/time:** 2026-05-19  
**Base HEAD (before sandbox branch):** `b9e7466446218d1f700bb7a4c8ec0267f41ee18e`  
**Sandbox merge SHA:** (set at merge)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — 11/11 sandbox tests PASS (`--test-threads=1`).

**Note:** Uses live V7.6 `StructuredFieldStencilOp` only. No mapping runtime. Atlas uses 1-cell gutter per 10×10 tile. Source-capped operator (cap=500) on H=8 tactical horizon.

---

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test mapping_optimization_toolkit_sandbox -- --nocapture --test-threads=1` | **PASS** — 11/11 |

**Full log:** [`mapping_optimization_toolkit_sandbox_full.log`](mapping_optimization_toolkit_sandbox_full.log)

---

## Decision Gate Summary

| Test | Area | PASS / PARTIAL / DEFERRED / FAIL | Key result |
|---|---|---|---|
| 0 | Baseline | PASS | 10×10=4.9ms, 32×32=2.0ms, 64×64=1.9ms; t44=3.94 |
| 1 | Atlas correctness | PARTIAL | max_error=3.46–5.27; cross_tile_leak=YES (gutter=1 insufficient at H=8) |
| 2 | Atlas cost | PASS | speedup_N4=7.3×, N16=15.4×, N64=59.6× |
| 3 | Cadence determinism | PASS | dispatches_avoided=1580; deterministic_replay=YES |
| 4 | Cadence quality | PARTIAL | all models hit source_cap=500; quality equivalent on this fixture |
| 5 | Dirty skip correctness | PASS | skip_ratio=0.625; false_skips=0 |
| 6 | Dirty + atlas | PARTIAL | speedup=1.76–2.69×; max_error=0.0 on scheduled tiles |
| 7 | Active halo | PARTIAL | best_halo=H8; max_error=0.0026; active_only FAIL; speedup~1.9× |
| 8 | Combined stack | PARTIAL | speedup_vs_standalone=31×; max_error=3.46 (atlas coupling) |
| 9 | Cost projection | PARTIAL | combined_stack_30k≈8.2ms vs accumulator 3236.6ms (rough) |
| 10 | ADR classification | PASS | see table below |

### Overall verdict

```text
PARTIAL
```

Reason:

```text
The optimization toolkit is promising for Mapping ADR adoption but not YES-ready as a production stack.
Atlas batching materially reduces dispatch overhead (up to ~60× for N=64 regions). Dirty macro-region
skipping is correct with zero false skips. Cadence tiers are deterministic and reduce dispatch count.
H-hop halo (H=8) matches full-grid oracle on 10×10. Not YES because: (1) gutter=1 atlas allows
cross-tile coupling at H=8 — ADR must specify isolation/gutter policy before atlas batching is
production-safe; (2) active-only / 1-cell halo fail correctness; (3) combined stack inherits atlas
coupling error; (4) cadence quality tradeoff masked by source cap on this fixture; (5) halo speedup
modest at 64% mask coverage on small grid. Not NO because all four optimizations are expressible on
current V7.6 generic toolkit without mapping runtime, new semantic WGSL, or production pass wiring.
```

---

## ADR Adoption Classification

| Optimization | Verdict | Layer | Evidence | Remaining risk | Recommended ADR wording |
|---|---|---|---|---|---|
| Atlas batching | Adopt provisionally | driver/scheduler + runtime substrate | N=64 speedup 59.6×; dispatch 576→9 | gutter=1 cross-tile leak at H=8 | Pack scheduled 10×10 tiles into atlas with ADR-defined gutter/isolation; batch one `StructuredFieldStencilOp` dispatch per atlas |
| Cadence tiers | Adopt now | RON/Designer policy | 1580 dispatches avoided / 120 ticks; deterministic | Field-type quality depends on cap/decay authoring | Author cadence tier per field class (EveryTick/4/10/60/Event); scheduler skips non-due fields |
| Dirty macro-region skipping | Adopt now | driver/scheduler | 62.5% skip ratio; false_skips=0 | Conservative false schedules acceptable | Primary sparse-map optimization: skip clean macro-regions before command-buffer construction |
| Active frontier + halo | Adopt provisionally | driver/scheduler + runtime substrate | H8 halo max_error=0.0026 vs full grid | active-only unusable; H-hop erases speedup on small grids | Do not authorize `ActiveOnlyExperimentalNoHalo` without H-hop or per-hop frontier expansion contract |

---

## Test 0 — Baseline (detail)

| grid | wall_ms | mean_ms/dispatch | t44 | max_value |
|---|---|---|---|---|
| 10×10 | 4.858 | 0.540 | 3.94 | 235.18 |
| 16×16 | 2.459 | 0.273 | 3.94 | 235.18 |
| 32×32 | 2.015 | 0.224 | 3.94 | 235.18 |
| 64×64 | 1.886 | 0.210 | 3.94 | 235.18 |

---

## Test 2 — Atlas cost (detail)

| N | standalone_ms | atlas_ms | speedup | gutter_overhead% |
|---|---|---|---|---|
| 4 | 18.1 | 2.5 | 7.3× | 30.6 |
| 16 | 31.6 | 2.1 | 15.4× | 30.6 |
| 64 | 116.6 | 2.0 | 59.6× | 30.6 |

---

## Test 7 — Active halo (detail)

| strategy | mask_ratio | max_error | t44_error | speedup |
|---|---|---|---|---|
| active_only | 0.040 | 235.18 | 3.94 | 1.45× |
| halo_1 | 0.080 | 195.88 | 3.94 | 1.56× |
| halo_H8 | 0.640 | 0.0026 | 0.00 | 1.92× |
| halo_per_hop_equiv | 0.640 | 0.0026 | 0.00 | 1.89× |

---

## Projection Summary (rough)

```text
baseline_per_edge_accumulator_projected_30k_dirty_adjusted = 3236.6ms
previous_stencil_projected_30k = 124.5ms
atlas_projected_30k = 59.8ms
dirty_atlas_10_percent_projected_30k = 6.0ms
cadence_adjusted_projected_30k = 31.1ms
combined_stack_projected_30k = 8.2ms
```

All projections are rough linear extrapolations from local GPU timings.

---

## Preserved artifacts

- [`mapping_optimization_toolkit_sandbox_code_preserve.rs`](../workshop/mapping_optimization_toolkit_sandbox_code_preserve.rs)
- [`mapping_optimization_toolkit_candidate_notes.md`](../workshop/mapping_optimization_toolkit_candidate_notes.md)
- [`mapping_atlas_batching_candidate.rs`](../workshop/mapping_atlas_batching_candidate.rs)
- [`mapping_cadence_tiers_candidate.rs`](../workshop/mapping_cadence_tiers_candidate.rs)
- [`mapping_dirty_macro_region_candidate.rs`](../workshop/mapping_dirty_macro_region_candidate.rs)
- [`mapping_active_frontier_halo_candidate.rs`](../workshop/mapping_active_frontier_halo_candidate.rs)

---

## Posture unchanged

- V7.6 `StructuredFieldStencilOp` remains live, opt-in, hardened, inert by default.
- No mapping runtime. No production pass graph wiring.
- Resource Flow defaults unchanged. `simthing-sim` remains semantic-free.
