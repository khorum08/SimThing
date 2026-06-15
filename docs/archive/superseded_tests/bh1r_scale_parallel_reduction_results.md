# BH-1R-SCALE parallel reduction — test results

**Rung:** BH-1R-SCALE production-scale GPU parallel choke reduction  
**Date:** 2026-06-11  
**PR:** BH-1R-SCALE: parallelize GPU choke threshold reduction

## Remedial context

BH-1R landed a compact GPU consumer but used a single `@workgroup_size(1,1,1)` invocation looping all cells — GPU-resident but not production-scale. BH-1R-SCALE replaces it with a staged parallel reducer.

## Candidate-F sqrt audit (BH-0 + BH-1 + BH-1R + BH-1R-SCALE)

Searched parallel reducer WGSL/Rust for: `sqrt`, `length`, `distance`, `normalize`, `hypot`, `magnitude`, `norm(`.

**Result:** no forbidden tokens in authoritative paths. Staged reduce uses sum, max, count, linear compare. No `m_jit_sqrt_f_exact` routing required.

Guard test: `bh1r_no_native_sqrt_in_hot_path` in `bh1r_scale_parallel_reduction.rs`.

## Implementation summary

| Surface | Change |
|---|---|
| Pass 1 | `reduce_choke_partials_pass1` — `@workgroup_size(256)` × `n_partials` workgroups, grid-stride cell gather + workgroup tree reduce |
| Pass 2 | `reduce_choke_final_pass2` — fold partials (supports `n_partials > 256`) to compact 4-float output |
| Validation | overflow guard on `width * height * n_dims`; finite threshold; column bounds |

Compact readback unchanged (4 floats). CPU oracle test-only.

## Targeted gates

```text
cargo fmt --all -- --check
cargo test -p simthing-gpu --test bh0_saturating_flux
cargo test -p simthing-gpu --test bh1_choke_readout
cargo test -p simthing-gpu --test bh1r_choke_threshold
cargo test -p simthing-gpu --test bh1r_scale_parallel_reduction
cargo test -p simthing-spec --test bh0_saturating_flux_admission
cargo test -p simthing-spec --test bh1_choke_readout_admission
```

## Test matrix

| Test | Result |
|---|---|
| `bh1r_parallel_reduction_not_single_lane` | PASS |
| `bh1r_parallel_choke_threshold_gpu_matches_cpu_oracle` | PASS |
| `bh1r_compact_readback_only` | PASS |
| `bh1r_crowded_field_crosses_threshold` | PASS |
| `bh1r_clear_field_does_not_cross_threshold` | PASS |
| `bh1r_config_rejects_invalid_or_overflow_shape` | PASS |
| `bh1r_no_native_sqrt_in_hot_path` | PASS |
| BH-0 / BH-1 / BH-1R regression | PASS |

`cargo test --workspace` not run.
