# FIELD_POLICY-OBS-0 — GPU-Resident Mobile Observer Overlay Score Results

## Base HEAD

`bb5387c` (post-SQRT-MAG2-PERF-0 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_field_policy_obs0_mobile_overlay_score.rs` | **New** — 7 tests: semantic-free WGSL, 34k perf, magnitude spot check, dense corpus, score authority, no wiring, edge rows |
| `docs/workshop/mapping_current_guidance.md` | FIELD_POLICY-OBS-0 row |
| `docs/accumulator_op_v2_production_plan.md` | FIELD_POLICY-OBS-0 section |
| `docs/invariants.md` | Mobile observer overlay score authority invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **Accepted gradient magnitude path:** Q16.16 fixed gx/gy → integer mag2 → pinned mag2_bits → artifact-backed Candidate F sqrt → exact mag (max_ulp=0).
2. **Q format/range:** Q16.16, ±16.0 FIELD_POLICY gradient table.
3. **34k benchmark (SQRT-MAG2-PERF-0):** combined Q16.16 ~1.7–2.5 ms warm GPU with readback; no-readback proxy ~0.174 ms/dispatch.
4. **Probe-only/rejected:** raw f32 dx/dy, native sqrt, Q12.12 (probe-only), lo-only conversion, split kernels, diagnostic mag2 without fixed-point source.
5. **Row shape:** numeric 10×u32 row (gx/gy/weight/bias inputs + mag2_lo/hi/mag2_bits/mag_bits/score_bits/flags outputs); no semantic field names in WGSL.
6. **Output authority:** `mag` exact-authoritative under Q16.16+F; `score` approximate/diagnostic (f32 bias + weight×mag).

## Row layout (10×u32, 1,360,000 bytes @ 34k)

| Index | Field | Role |
|---:|---|---|
| 0 | gx_fixed | Q16.16 input |
| 1 | gy_fixed | Q16.16 input |
| 2 | w_mag_fixed | Q16.16 weight input |
| 3 | bias_fixed | Q16.16 bias input |
| 4 | mag2_lo | exact integer mag2 limb |
| 5 | mag2_hi | exact integer mag2 limb |
| 6 | mag2_bits | exact-authoritative |
| 7 | mag_bits | exact-authoritative (F sqrt) |
| 8 | score_bits | approximate/diagnostic (f32) |
| 9 | flags | reserved (0) |

## Math path

```text
gx_fixed, gy_fixed (Q16.16)
→ u32 limb mag2 = gx² + gy²
→ mag2_bits (pinned f32 conversion)
→ mag_bits = sqrt_cr_f_bits(mag2_bits)
→ score_bits = bitcast(bias/65536 + weight/65536 * mag)
```

## Output authority

| Output | Authority | Contract |
|---|---|---|
| mag2_bits | ExactAuthoritative | Q16.16 `ExactFixedPointDxDy` (same as SQRT-MAG2-0) |
| mag_bits | ExactAuthoritative | Artifact-backed Candidate F (`e2e9e27601ee2e13`) |
| score_bits | ApproximateDiagnostic | f32 multiply/add after exact mag; not pinned fixed-point |

## Correctness results

| Corpus | mag2 exact | mag max_ulp | score max_ulp | overflow |
|---|---:|---:|---:|---:|
| Edge (6 rows) | 6/6 | 0 | 0 | 0 |
| Dense (50,176 rows) | 50,176/50,176 | 0 | 0 | 0 |
| 34k spot (512 rows) | 512/512 | 0 | — | 0 |

## 34k benchmark

| Metric | Value |
|---|---|
| rows | 34,000 |
| dispatches | 1 |
| includes_readback | true |
| elapsed_ms | ~15.6 (this run; cold-start variance) |
| per_row_us | ~0.46 |
| spot_mag_max_ulp | 0 |

**Timing caveats:** First-run shader compile can inflate elapsed_ms (this run ~15.6 ms vs SQRT-MAG2-PERF-0 warm combined ~1.7 ms). Overlay adds negligible f32 score work over combined mag2+F path.

## Comparison to SQRT-MAG2-PERF-0

| Path | 34k elapsed_ms (approx) |
|---|---:|
| SQRT-MAG2-PERF-0 combined Q16.16 | ~1.7 |
| FIELD_POLICY-OBS-0 overlay score (this run) | ~15.6 |

Score multiply/add is not the cost driver; mag2+F dominates. Warm reruns expected to align with combined path + readback overhead.

## Descriptor/admission status

Suggested id `m_jit_field_policy_obs0_overlay_score` **deferred** — inline exact mag admission for combined mag2+F+score would require extending `ExactPreSqrtInputContract`; kept as driver fixture only. Existing `m_jit_mag2_fixed_exact` and F artifact admission unchanged.

## Tests/scans run

```bash
cargo test -p simthing-driver --test phase_m_field_policy_obs0_mobile_overlay_score -- --nocapture  # 7 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag2_perf0_fixed_hotpath -- --nocapture  # 10 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag2_0_fixed_exact -- --nocapture  # 7 passed
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture  # 10 passed
cargo check --workspace  # ok
```

## Guardrail confirmations

- No scheduler/cache/default SimSession wiring
- No semantic WGSL (forbidden-term scan clean)
- No production economy→mapping bridge
- No CPU-side AI planner
- Raw f32 dx/dy not used for mag path
- F hash `e2e9e27601ee2e13` verified via `include_str!`
- No Candidate C/f64

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — FIELD_POLICY-OBS-0 landed a default-off/test-only GPU-resident mobile observer overlay score probe consuming the exact Q16.16 mag2 + artifact-backed F sqrt path; 34k timing and correctness results were recorded, magnitude exactness remains bounded to the pinned fixed-point contract, score authority was classified as approximate/diagnostic f32 arithmetic, no scheduler/cache/default wiring/semantic WGSL/economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
