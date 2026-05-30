# SEAD-OBS-3 — Fixed-Point Aggregate Score for Multi-Layer Observer Overlay Results

## Base HEAD

`8d5d3e0` (post-SEAD-OBS-2 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_sead_obs3_fixed_point_score.rs` | **New** — 6 tests: edge rows, dense corpus, 34k perf, warm 32× dispatch, score authority, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `ExactQ16WeightedSum`, `m_jit_sead_obs3_multilayer_fixed_score` descriptor + admission |
| `crates/simthing-spec/tests/sead_obs0_overlay_score_admission.rs` | SEAD-OBS-3 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | SEAD-OBS-3 row |
| `docs/accumulator_op_v2_production_plan.md` | SEAD-OBS-3 section |
| `docs/invariants.md` | Exact aggregate score invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **Per-layer magnitude authority (SEAD-OBS-2):** Q16.16 gx/gy → exact integer mag2 → artifact-backed Candidate F → exact `layerN_mag_bits` (`max_ulp = 0`).
2. **Aggregate score approximate boundary:** OBS-2 used f32 `bias + Σ weight × mag`; `score_bits` was `ApproximateDiagnosticF32` (dense max_ulp = 15 vs f32 oracle).
3. **Feasible fixed-point format:** Q16.16 signed weights/bias/score; `mag_fixed = round_ties_even(mag_f32 × 65536)` from exact `mag_bits`; accumulation via trunc-toward-zero `(weight × mag_fixed) / 65536`.
4. **f32 score leak prevention:** Distinct descriptor `m_jit_sead_obs3_multilayer_fixed_score` with `ExactQ16WeightedSum`; OBS-2 `m_jit_sead_obs2_multilayer_overlay_score` unchanged (`ApproximateDiagnosticF32`); admission rejects exact f32 score and score_bits under fixed contract.
5. **Overflow bounds (4 layers):** Corpus |grad| ≤ 16, |weight| ≤ 2, |bias| ≤ 2 → max per-layer term ≈ 2.97×10⁶; 4-layer sum + bias fits i32; edge cases at |grad| = 16, |weight| = 2 verified overflow = 0.
6. **34k benchmark shape:** 34,000 rows, 4 layers, single dispatch + readback; warm 32× dispatch with one final readback.

## Fixed-point score format

| Field | Format |
|---|---|
| weights, bias | Q16.16 signed (`i32` storage) |
| mag input | `mag_fixed = round_ties_even(bitcast<f32>(mag_bits) × 65536)` |
| accumulation | `score += (weight × mag_fixed) / 65536` (trunc toward zero, i32 wrap) |
| score output | Q16.16 signed `score_fixed` (`ExactAuthoritative`) |

## Conversion contract

`mag_bits` remains exact f32 magnitude under Q16.16 + F. Conversion to score input uses banker's rounding at Q16.16 scale (WGSL `round(mag_f32 * 65536.0)`; CPU `round_ties_even`). GPU/CPU parity verified on dense + edge corpora.

## Descriptor/admission status

**Landed:** `m_jit_sead_obs3_multilayer_fixed_score` — default_off, `ExactQ16WeightedSum`, per-layer mag exact, `score_fixed` exact.  
**Preserved:** `m_jit_sead_obs2_multilayer_overlay_score` — `ApproximateDiagnosticF32` unchanged.

## Output authority matrix

| Output | OBS-3 authority | OBS-2 authority |
|---|---|---|
| layer0..3_mag_bits | ExactAuthoritative | ExactAuthoritative |
| score_fixed | ExactAuthoritative | — |
| score_bits | — | ApproximateDiagnostic |
| flags | ApproximateDiagnostic | ApproximateDiagnostic |

## Correctness

| Metric | Result |
|---|---|
| edge rows | 6/6 score exact, 24/24 mag exact, mag max_ulp = 0 |
| dense rows | 6,272/6,272 score exact, 25,088/25,088 mag exact |
| mag max_ulp | 0 |
| overflow (declared range) | 0 |

## 34k single-dispatch benchmark

| Metric | Value |
|---|---|
| rows | 34,000 |
| layers | 4 |
| dispatches | 1 |
| includes_readback | true |
| elapsed_ms | ~7.8 |
| per_row_us | ~0.23 |
| spot_mag_max_ulp | 0 |
| spot_score_exact | 512/512 |

## 34k warm repeated-dispatch (32×)

| Metric | Value |
|---|---|
| total_ms | ~8.9 |
| per_dispatch_ms | ~0.278 |
| per_row_us | ~0.0082 |
| per_layer_mag_us | ~0.0020 |
| spot_mag_max_ulp | 0 |
| score authority | ExactQ16WeightedSum |

## Comparison to SEAD-OBS-2

| Path | per_dispatch_ms (warm) | score authority |
|---|---:|---|
| SEAD-OBS-2 f32 | ~0.238 | ApproximateDiagnosticF32 |
| SEAD-OBS-3 fixed | ~0.278 | ExactQ16WeightedSum |

Fixed-point accumulation adds ~17% warm dispatch cost vs OBS-2 f32 score.

## Tests/scans run

```bash
cargo test -p simthing-driver --test phase_m_sead_obs3_fixed_point_score -- --nocapture  # 6 passed
cargo test -p simthing-driver --test phase_m_sead_obs2_multilayer_overlay_score -- --nocapture  # 6 passed
cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture  # 10 passed
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture  # 10 passed
cargo check --workspace  # ok
rg "sead_obs3|ExactQ16WeightedSum|m_jit_sead_obs3" crates docs  # OBS-3 present
rg "score_bits.*ExactAuthoritative" crates docs  # rejects only; no authorized f32 exact score
rg "ApproximateDiagnosticF32|m_jit_sead_obs2" crates docs  # OBS-2 f32 remains diagnostic
```

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — SEAD-OBS-3 landed a distinct fixed-point aggregate score variant for multi-layer GPU-resident observer overlays; existing f32 score descriptors remain ApproximateDiagnostic, exact score authority is limited to the pinned Q16.16 weighted-sum contract, per-layer Q16.16 + F magnitudes remain exact, 34k timing and correctness results were recorded, no scheduler/cache/default wiring/semantic WGSL/economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / SEAD posture remains intact.
