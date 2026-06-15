# BH-1R choke consumption — test results

**Rung:** BH-1R GPU-resident choke reduce/threshold consumer  
**Date:** 2026-06-11  
**PR:** BH-1R: add GPU-resident choke threshold consumer

## Remedial context

BH-1 correctly landed GPU-resident choke readout (`1 − C/χ`) but overclaimed consumption: the prior “consumption proof” was CPU-side sum/threshold only. BH-1R closes that gap with a compact GPU consumer.

## Candidate-F sqrt audit (BH-0 + BH-1 + BH-1R hot paths)

Searched `saturating_flux_choke_threshold.wgsl` and `saturating_flux_choke_threshold.rs` for: `sqrt`, `length`, `distance`, `normalize`, `hypot`, `magnitude`, `norm(`.

**Result:** no forbidden tokens. Reduce uses linear sum, max, count, and threshold compare. No `m_jit_sqrt_f_exact` routing required.

Guard test: `bh1r_no_native_sqrt_in_hot_path`.

## Implementation summary

| Surface | Change |
|---|---|
| `SaturatingFluxChokeThresholdOp` | Staged parallel GPU reducer: pass 1 (`256`-thread workgroups × `ceil(cells/256)`), pass 2 fold to compact 4-float output |
| Compact readback | 4 floats only (`sum`, `max`, `count_above`, `crossed`) |
| `cpu_choke_threshold_oracle` | test-only parity oracle |

**BH-1R-SCALE:** single-lane reducer remediated — see [`bh1r_scale_parallel_reduction_results.md`](bh1r_scale_parallel_reduction_results.md).

No full-field CPU readback for threshold decision. No border service, pathfinding, movement, PALMA, or stored C field.

## Targeted gates

```text
cargo fmt --all -- --check
cargo test -p simthing-gpu --test bh0_saturating_flux
cargo test -p simthing-gpu --test bh1_choke_readout
cargo test -p simthing-gpu --test bh1r_choke_threshold
cargo test -p simthing-spec --test bh0_saturating_flux_admission
cargo test -p simthing-spec --test bh1_choke_readout_admission
```

## Test matrix

| Test | Result |
|---|---|
| `bh1r_no_native_sqrt_in_hot_path` | PASS |
| `bh1r_choke_threshold_gpu_matches_cpu_oracle` | PASS |
| `bh1r_choke_threshold_stays_gpu_resident` | PASS |
| `bh1r_crowded_field_crosses_threshold` | PASS |
| `bh1r_clear_field_does_not_cross_threshold` | PASS |
| BH-0 regression | PASS |
| BH-1 regression | PASS |

## Status split

| Item | Status |
|---|---|
| BH-1 choke readout | IMPLEMENTED / PASS |
| BH-1R GPU consumption | IMPLEMENTED / PASS |
| Same-frame Layer-2 admission wiring | still deferred (strict sink; not BH-1R scope) |

`cargo test --workspace` not run.
