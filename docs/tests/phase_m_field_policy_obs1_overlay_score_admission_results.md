# FIELD_POLICY-OBS-1 — Descriptor/Admission for Mobile Observer Overlay Score Results

## Base HEAD

`8736adb` (post-FIELD_POLICY-OBS-0 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_field_policy_obs0_overlay_score` descriptor, `ScoreAuthorityContract`, `InlineFixedPointMag2Sqrt`, admission validators |
| `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs` | `score_authority_contract` field, registry entry |
| `crates/simthing-spec/tests/field_policy_obs0_overlay_score_admission.rs` | **New** — 6 admission tests |
| `crates/simthing-driver/tests/phase_m_field_policy_obs0_mobile_overlay_score.rs` | Warm repeated-dispatch benchmark, descriptor wiring tests |
| `docs/workshop/mapping_current_guidance.md` | FIELD_POLICY-OBS-1 row |
| `docs/accumulator_op_v2_production_plan.md` | FIELD_POLICY-OBS-1 section |
| `docs/invariants.md` | Score descriptor authority invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **FIELD_POLICY-OBS-0 exact outputs:** `mag2_bits`, `mag_bits` (Q16.16 integer mag2 + F sqrt, max_ulp=0).
2. **Approximate/diagnostic:** `score_bits` (f32 bias + weight×mag), `flags`.
3. **Descriptor deferred because:** inline exact mag admission required extending `ExactPreSqrtInputContract` beyond `ExactMag2Bits`/`RawDxDyProbe`.
4. **Descriptor metadata needed:** gx/gy/w_mag/bias reads; mag2_bits/mag_bits exact; score_bits/flags diagnostic; Q16.16 mag2 source; F artifact; score authority contract.
5. **Admission prevents score overclaiming:** `ScoreAuthorityContract::ApproximateDiagnosticF32` rejects `score_bits` as `ExactAuthoritative`.
6. **Warm benchmark shape:** 32 repeated dispatches, one final readback (measurement proxy only).

## Descriptor metadata

| Field | Value |
|---|---|
| id | `m_jit_field_policy_obs0_overlay_score` |
| label | `FieldPolicyObserverOverlayScore` |
| reads | gx_fixed, gy_fixed, w_mag_fixed, bias_fixed |
| mag2 contract | ExactFixedPointDxDy Q16.16 (±16.0) |
| pre-sqrt | InlineFixedPointMag2Sqrt |
| sqrt | Candidate F `e2e9e27601ee2e13` / `sqrt_cr_f_bits` |
| score contract | ApproximateDiagnosticF32 |
| default_off | true |
| production_wiring | false |

## Output authority matrix

| Output | Authority |
|---|---|
| mag2_bits | ExactAuthoritative |
| mag_bits | ExactAuthoritative |
| score_bits | ApproximateDiagnostic |
| flags | ApproximateDiagnostic |

## Admission matrix

| Case | Result |
|---|---|
| Valid landed descriptor | Accepted |
| score_bits ExactAuthoritative + f32 contract | Rejected |
| Q12 fraction_bits (wrong Q) | Rejected |
| Missing Mag2SourceContract | Rejected |
| Missing F artifact | Rejected |
| Wrong F hash | Rejected |
| Native ApproximateJitOnly math | Rejected |
| production_wiring=true / default_off=false | Rejected |

## Warm 34k repeated-dispatch benchmark

| Metric | Value |
|---|---|
| rows | 34,000 |
| dispatches | 32 |
| includes_readback | true (one final) |
| total_ms | ~4.14 |
| per_dispatch_ms | ~0.129 |
| per_row_us | ~0.0038 |
| spot_mag_max_ulp | 0 |
| score_authority | ApproximateDiagnosticF32 |

## Comparison

| Path | Timing |
|---|---|
| FIELD_POLICY-OBS-0 cold 34k (1 dispatch + readback) | ~13–15.6 ms |
| FIELD_POLICY-OBS-1 warm 34k (32 dispatches + readback) | ~0.129 ms/dispatch |
| SQRT-MAG2-PERF-0 combined Q16 warm | ~1.7 ms |

## Tests/scans run

```bash
cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture  # 6 passed
cargo test -p simthing-driver --test phase_m_field_policy_obs0_mobile_overlay_score -- --nocapture  # 9 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag2_perf0_fixed_hotpath -- --nocapture  # 10 passed
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture  # 10 passed
cargo check --workspace  # ok
```

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — FIELD_POLICY-OBS-1 landed default-off descriptor/admission for the GPU-resident mobile observer overlay score row; exact authority for mag2/mag remains bounded to Q16.16 + artifact-backed Candidate F, score_bits remains ApproximateDiagnostic under f32 weighting, warm 34k repeated-dispatch timing was recorded, no scheduler/cache/default wiring/semantic WGSL/economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
