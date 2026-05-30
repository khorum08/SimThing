# SEAD-OBS-4 — GPU-Resident Threshold Event Emission Results

## Base HEAD

`cb96202` (post-SEAD-OBS-3 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_sead_obs4_threshold_event.rs` | **New** — 7 tests: semantic-free WGSL, edge rows, dense corpus, 34k perf, warm 32× dispatch, event authority, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_sead_obs4_threshold_event` descriptor + threshold/event contracts |
| `crates/simthing-spec/tests/sead_obs0_overlay_score_admission.rs` | SEAD-OBS-4 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | SEAD-OBS-4 row |
| `docs/accumulator_op_v2_production_plan.md` | SEAD-OBS-4 section |
| `docs/invariants.md` | GPU threshold event invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **SEAD-OBS-3 exact score:** `ExactQ16WeightedSum`; `score_fixed` ExactAuthoritative; per-layer mag exact under Q16.16 + F.
2. **Diagnostic boundary:** OBS-2 `score_bits` remains ApproximateDiagnosticF32; OBS-4 `flags` remains ApproximateDiagnostic.
3. **Threshold/event row shape:** 4×(gx,gy,w) + bias + threshold + hysteresis + prior_state inputs; mag_bits + score_fixed + state + event_code + flags outputs (24×u32 stride).
4. **No CPU planner:** crossing logic is integer compare on GPU; event rows are numeric codes only; no urgency/commitment bridge.
5. **Event authority:** `state_u32` and `event_code_u32` ExactAuthoritative under `ExactQ16Threshold` + `ExactDeterministicEventFlag`.
6. **34k benchmark:** single dispatch + readback; warm 32× dispatch with one final readback.

## Row layout (24×u32)

| Offset | Field |
|---:|---|
| 0–11 | layer0..3 (gx, gy, w) Q16.16 |
| 12 | bias_fixed |
| 13 | threshold_fixed |
| 14 | hysteresis_fixed |
| 15 | prior_state_u32 |
| 16–19 | layer0..3_mag_bits |
| 20 | score_fixed |
| 21 | state_u32 |
| 22 | event_code_u32 |
| 23 | flags |

## Threshold/hysteresis contract

Q16.16 signed integer comparisons only:

```text
if prior_state == 0 && score_fixed >= threshold + hysteresis → state=1, event_code=1
elif prior_state == 1 && score_fixed <= threshold - hysteresis → state=0, event_code=2
else → state=prior_state, event_code=0
```

Score path reuses OBS-3 exact fixed-point accumulation.

## Descriptor/admission status

**Landed:** `m_jit_sead_obs4_threshold_event` — default_off, `ExactQ16WeightedSum` score, `ExactQ16Threshold` operands, `ExactDeterministicEventFlag` outputs.

## Output authority matrix

| Output | Authority |
|---|---|
| layer0..3_mag_bits | ExactAuthoritative |
| score_fixed | ExactAuthoritative |
| state_u32 | ExactAuthoritative |
| event_code_u32 | ExactAuthoritative |
| flags | ApproximateDiagnostic |

## Correctness

| Metric | Result |
|---|---|
| edge rows | 8/8 score, state, event exact |
| dense rows | 47,040/47,040 score, state, event exact |
| overflow | 0 |
| 34k spot (512) | 512/512 score, state, event exact |

## Event count summary (34k mobile corpus)

| Metric | Value |
|---|---|
| total non-zero events | 17,429 / 34,000 |
| spot sample events_up (512) | 152 |

## 34k single-dispatch benchmark

| Metric | Value |
|---|---|
| rows | 34,000 |
| layers | 4 |
| dispatches | 1 |
| includes_readback | true |
| elapsed_ms | ~64.5 (includes shader/pipeline creation on cold run) |
| per_row_us | ~1.90 |
| spot exactness | 512/512 |

## 34k warm repeated-dispatch (32×)

| Metric | Value |
|---|---|
| total_ms | ~7.4 |
| per_dispatch_ms | ~0.231 |
| per_row_us | ~0.0068 |
| spot exactness | 512/512 |
| event authority | ExactDeterministicEventFlag |

## Comparison to SEAD-OBS-3

| Path | warm per_dispatch_ms |
|---|---:|
| SEAD-OBS-3 fixed score | ~0.278 |
| SEAD-OBS-4 threshold event | ~0.231 |

Threshold/event logic adds negligible cost vs OBS-3 score-only warm path on repeated dispatch.

## Tests/scans run

```bash
cargo test -p simthing-driver --test phase_m_sead_obs4_threshold_event -- --nocapture  # 7 passed
cargo test -p simthing-driver --test phase_m_sead_obs3_fixed_point_score -- --nocapture  # 6 passed
cargo test -p simthing-driver --test phase_m_sead_obs2_multilayer_overlay_score -- --nocapture  # 6 passed
cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture  # 12 passed
cargo check --workspace  # ok
```

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — SEAD-OBS-4 landed a default-off/test-only GPU-resident threshold event emission probe from exact Q16.16 observer scores; threshold/hysteresis comparisons are fixed-point/pinned, state/event outputs are exact deterministic under the declared contract, 34k timing and correctness results were recorded, no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, or production economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / SEAD posture remains intact.
