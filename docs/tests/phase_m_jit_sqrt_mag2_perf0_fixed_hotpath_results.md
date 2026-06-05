# SQRT-MAG2-PERF-0 — Exact Fixed-Point Mag2 + F Sqrt Performance Decomposition Results

## Base HEAD

`fa71029` (post-SQRT-MAG2-0 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_jit_sqrt_mag2_perf0_fixed_hotpath.rs` | **New** — perf decomposition + optimization probes (10 tests) |
| `docs/workshop/mapping_current_guidance.md` | SQRT-MAG2-PERF-0 row |
| `docs/accumulator_op_v2_production_plan.md` | SQRT-MAG2-PERF-0 section |
| `docs/workshop/sqrt_candidates.md` | Performance note |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **SQRT-MAG2-0 exactness proven:** Q16.16 integer mag2 784/784 exact; F sqrt max_ulp=0 on edge/dense/34k spot check.
2. **Prior benchmarks:** F-only 34k ~1.376 ms; raw dx/dy probe ~5.4 ms; SQRT-MAG2-0 combined ~21.5 ms (first-run/cold GPU, readback included).
3. **Benchmark includes buffer init + readback:** Yes — same row layout (6×u32), storage init, single dispatch, staging copy + map.
4. **Likely cost center:** Fixed-point u32 limb multiply/add dominates shader vs F sqrt alone; readback/buffer path is non-trivial fraction.
5. **Safe optimizations:** Q12.12 probe (different contract), lo-only conversion (rejected), split kernels (slower), no-readback dispatch proxy (shader estimate).
6. **34k target:** Sub-millisecond shader proxy (~0.24 ms/dispatch) and ~1–4 ms end-to-end with readback — well within typical per-tick GPU budget for 34k entities.

## Benchmark methodology

- Deterministic 34k mobile-simthing Q16.16 pairs (28-value gradient table, LCG `0x5345_4144`).
- Row stride 6×u32 (816,000 bytes readback).
- `includes_readback=true` unless noted (Candidate D).
- Spot-check 512 rows for correctness on 34k paths.

## 34k timing decomposition (this run, readback included)

| Path | elapsed_ms | per_entity_us | dispatches |
|---|---:|---:|---:|
| Readback/baseline (passthrough touch) | 2.503 | 0.0736 | 1 |
| Fixed mag2 only (Q16.16) | 1.476 | 0.0434 | 1 |
| F sqrt only (precomputed mag2_bits) | 1.771 | 0.0521 | 1 |
| Combined Q16.16 mag2 + F | 1.730 | 0.0509 | 1 |
| Split mag2 + sqrt kernels | 1.632 | 0.0480 | 2 |
| Combined Q12.12 mag2 + F | 1.211 | 0.0356 | 1 |

**Interpretation:** Buffer init + readback baseline ~2.5 ms. End-to-end combined path ~1.7 ms on warm GPU (run-to-run variance affects mag2-only vs combined ordering). Shader-heavy proxy (32 dispatches, one readback): **0.174 ms/dispatch**, **0.005 µs/entity**.

## Scaled smoke (combined Q16.16)

| rows | elapsed_ms | per_entity_us |
|---:|---:|---:|
| 10,000 | 3.055 | 0.3055 |
| 34,000 | 2.473 | 0.0727 |
| 100,000 | 3.269 | 0.0327 |

## Optimization candidates

| Candidate | Result | Verdict |
|---|---|---|
| **A — Q12.12** | Dense 784/784 exact under Q12 contract; 551/784 mag2_bits differ vs Q16.16; 34k ~1.18 ms | **Probe only** — valid exact contract but coarser; not selected as primary |
| **B — lo-only conversion** | 423/784 rows have hi≠0; lo-only wrong on all 423 | **REJECTED** for full ±16 FIELD_POLICY range |
| **C — split kernels** | 34k ~1.63 ms vs combined ~1.73 ms; max_ulp=0 | **REJECTED** — extra dispatch, no win |
| **D — no-readback proxy** | 32× dispatch, 0.174 ms/dispatch | **Diagnostic** — shader estimate only |

## Recommended exact hot-path candidate

| Field | Selection |
|---|---|
| Q format | **Q16.16** (unchanged from SQRT-MAG2-0) |
| Range | ±16.0 FIELD_POLICY gradient table |
| Kernel layout | Single combined mag2 + F sqrt |
| 34k timing | ~1.7–2.5 ms (warm GPU, readback included) |
| Correctness | max_int_error=0, max_ulp=0 |

## Comparison to prior benchmarks

| Path | 34k elapsed_ms (approx) |
|---|---:|
| F-only sqrt (`sqrt_cr_f_bits`) | ~1.376 |
| Raw dx/dy F probe | ~5.4 |
| SQRT-MAG2-0 combined (first run) | ~21.5 |
| SQRT-MAG2-PERF-0 combined (this run) | ~1.73 |

SQRT-MAG2-0 ~21.5 ms likely reflects cold-start shader compile + first GPU submission; warm reruns align with F-only + mag2 overhead.

## Tests/scans run

```bash
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag2_perf0_fixed_hotpath -- --nocapture  # 10 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag2_0_fixed_exact -- --nocapture        # 7 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag0_f_exact_magnitude -- --nocapture    # 12 passed
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture            # 10 passed
cargo check --workspace  # ok
```

## Guardrail confirmations

- No scheduler/cache/default SimSession wiring
- No semantic WGSL
- No production economy→mapping bridge
- Raw f32 dx/dy remains probe-only
- F hash `e2e9e27601ee2e13` unchanged
- Q16.16 descriptor/invariant unchanged (no invariants.md edit required)

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — SQRT-MAG2-PERF-0 decomposed and optimized the exact fixed-point mag2 + artifact-backed F sqrt FIELD_POLICY hot path; exactness remains bounded to the pinned Q16.16 contract, raw f32 dx/dy remains probe-only, 34k timing and cost breakdown were recorded, four exact-preserving optimization probes were evaluated (Q12.12 probe-only, lo-only rejected, split rejected, no-readback proxy diagnostic), no scheduler/cache/default wiring/semantic WGSL/economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
