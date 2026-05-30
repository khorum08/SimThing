# SEAD-OBS-2 — Multi-Layer GPU-Resident Observer Overlay Score Results

## Base HEAD

`d6019c8` (post-SEAD-OBS-1 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_sead_obs2_multilayer_overlay_score.rs` | **New** — 6 tests: semantic-free WGSL, dense correctness, 34k perf, warm 32× dispatch, score authority, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_sead_obs2_multilayer_overlay_score` descriptor + multilayer admission |
| `crates/simthing-spec/tests/sead_obs0_overlay_score_admission.rs` | SEAD-OBS-2 admission tests |
| `docs/workshop/mapping_current_guidance.md` | SEAD-OBS-2 row |
| `docs/accumulator_op_v2_production_plan.md` | SEAD-OBS-2 section |
| `docs/invariants.md` | Multi-layer score invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **SEAD-OBS-1 exactness:** mag2_bits/mag_bits ExactAuthoritative under Q16.16 + F; score ApproximateDiagnosticF32.
2. **Approximate/diagnostic:** f32 `score = bias + weight * mag`; flags diagnostic.
3. **Row expansion:** 4×(gx, gy, weight) + bias inputs; 4×layer mag_bits + score + flags outputs (19×u32 stride).
4. **Layer count:** 4 layers (fixed `SEAD_OBS2_LAYER_COUNT`).
5. **Output authority:** layer0..3_mag_bits Exact; score_bits/flags ApproximateDiagnostic.
6. **34k benchmark:** single dispatch + readback; warm 32× dispatch proxy with one readback.

## Layer count

4 (`SEAD_OBS2_LAYER_COUNT`)

## Row layout (19×u32)

| Offset | Field |
|---:|---|
| 0–11 | layer0..3 (gx, gy, w) each Q16.16 |
| 12 | bias_fixed |
| 13–16 | layer0..3_mag_bits |
| 17 | score_bits |
| 18 | flags |

## Math path

Per layer: Q16.16 gx/gy → integer mag2 → mag2_bits → `sqrt_cr_f_bits` → exact mag_bits.  
Score: `bias + Σ weight_i × mag_i` via f32 (ApproximateDiagnostic).

## Descriptor/admission status

**Landed:** `m_jit_sead_obs2_multilayer_overlay_score` — default_off, InlineFixedPointMag2Sqrt + Q16.16 mag2 contract, ApproximateDiagnosticF32 score.

## Output authority matrix

| Output | Authority |
|---|---|
| layer0..3_mag_bits | ExactAuthoritative |
| score_bits | ApproximateDiagnostic |
| flags | ApproximateDiagnostic |

## Correctness (dense corpus)

| Metric | Result |
|---|---|
| rows | 6,272 |
| layer mag slots | 25,088 |
| mag2/mag exact | 25,088/25,088 |
| mag max_ulp | 0 |
| score max_ulp (f32 oracle) | 15 (diagnostic; not exact contract) |
| overflow | 0 |

## 34k single-dispatch benchmark

| Metric | Value |
|---|---|
| rows | 34,000 |
| layers | 4 |
| dispatches | 1 |
| includes_readback | true |
| elapsed_ms | ~7.0 |
| per_row_us | ~0.21 |
| layer_mags | 136,000 |
| spot_mag_max_ulp | 0 |

## 34k warm repeated-dispatch (32×)

| Metric | Value |
|---|---|
| total_ms | ~7.6 |
| per_dispatch_ms | ~0.238 |
| per_row_us | ~0.0070 |
| per_layer_mag_us | ~0.0017 |
| spot_mag_max_ulp | 0 |

## Comparison to SEAD-OBS-1

| Path | per_dispatch_ms (approx) |
|---|---:|
| SEAD-OBS-1 warm single-layer | ~0.129 |
| SEAD-OBS-2 warm 4-layer | ~0.238 |

4-layer cost scales sub-linearly vs 4× single-layer (~0.52 ms naive).

## Tests/scans run

```bash
cargo test -p simthing-driver --test phase_m_sead_obs2_multilayer_overlay_score -- --nocapture  # 6 passed
cargo test -p simthing-driver --test phase_m_sead_obs0_mobile_overlay_score -- --nocapture  # 9 passed
cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture  # 8 passed
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture  # 10 passed
cargo check --workspace  # ok
```

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — SEAD-OBS-2 landed a default-off/test-only multi-layer GPU-resident observer overlay score probe; four Q16.16 + artifact-backed F magnitude layers are evaluated per row, per-layer magnitudes remain exact under the pinned fixed-point contract, aggregate score remains ApproximateDiagnostic under f32 weighting, 34k timing and correctness results were recorded, no scheduler/cache/default wiring/semantic WGSL/economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / SEAD posture remains intact.
