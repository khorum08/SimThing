# SEAD-PIPE-0 — Integrated GPU Observer-Event Pipeline Results

## Base HEAD

`3fc88dd` (post-SEAD-EVENT-0 merge, pre-PIPE-0)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_sead_pipe0_observer_event_pipeline.rs` | **New** — 7 tests: semantic-free WGSL, edge pipeline, dense corpus, 34k integrated, 34k capacity variants, warm 32× two-pass, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_sead_pipe0_observer_event_pipeline` descriptor + validation |
| `crates/simthing-spec/src/compile/mod.rs`, `lib.rs`, `jit_kernel_descriptor_admission.rs` | exports + registry |
| `crates/simthing-spec/tests/sead_obs0_overlay_score_admission.rs` | SEAD-PIPE-0 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | SEAD-PIPE-0 row |
| `docs/accumulator_op_v2_production_plan.md` | SEAD-PIPE-0 section |
| `docs/invariants.md` | Integrated pipeline invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **OBS-4 event authority:** `score_fixed` ExactQ16WeightedSum; `state_u32` / `event_code_u32` ExactDeterministicEventFlag under Q16.16 threshold/hysteresis.
2. **EVENT-0 compaction authority:** `event_count` / `overflow_flag` ExactAuthoritative; membership ExactAuthoritativeUnordered when capacity sufficient; order UnspecifiedAtomicOrder.
3. **Integrated pipeline authority:** threshold pass inherits OBS-4 score/state/event exactness on intermediate rows; compaction pass inherits EVENT-0 count/membership/overflow; ordering remains UnspecifiedAtomicOrder.
4. **Buffer layout:** observer input 16×u32/row → intermediate event row 5×u32/row (OBS-4 output form) → compact record 5×u32 + atomic counters; no CPU interpretation between passes.
5. **Ordering/capacity:** same as EVENT-0 — overflow flagged when slot ≥ capacity; membership exact unordered only when capacity ≥ event_count.
6. **34k benchmark shape:** `mobile_observer_rows(34000)` (same corpus family as OBS-4), capacity=34000, two dispatches per cycle, one final readback.

## Pipeline layout

```text
Pass A (threshold_pass): observer_inputs[16×N] → event_rows[5×N]
Pass B (compact_pass):   event_rows[5×N] → counters + records[5×capacity]
```

Single command encoder submits both passes; no CPU readback/filtering between passes.

## Buffer contracts

| Buffer | Stride | Fields |
|---|---:|---|
| observer input | 16×u32 | 4×(gx_fixed, gy_fixed, w_fixed), bias, threshold, hysteresis, prior_state |
| intermediate event row | 5×u32 | source_index, event_code, state, score_fixed, flags |
| compact record | 5×u32 | source_index, event_code, state, score_fixed, reserved |
| counters | 2×atomic u32 | event_count, overflow_flag |

## Dispatch sequence

1. `threshold_pass` — workgroups `ceil(N/64)`
2. `compact_pass` — workgroups `ceil(N/64)`
3. (test readback only after both passes complete)

## Descriptor/admission status

**Landed:** `m_jit_sead_pipe0_observer_event_pipeline` — default_off, test-only, production_wiring=false.

Contracts: Q16.16 observer input, ExactQ16WeightedSum score, ExactDeterministicEventFlag (pass A), ExactAuthoritativeUnordered compaction membership, UnspecifiedAtomicOrder, Candidate F artifact binding for pass A sqrt.

## Output authority matrix

| Output | Authority |
|---|---|
| score (intermediate) | ExactQ16WeightedSum |
| state / event (intermediate) | ExactDeterministicEventFlag |
| event_count | ExactAuthoritative |
| overflow_flag | ExactAuthoritative |
| membership (compact records) | ExactAuthoritativeUnordered (capacity sufficient) |
| order | UnspecifiedAtomicOrder |

## Correctness results

| Case | Result |
|---|---|
| edge (7 scenarios) | score/state/event exact per row; event_count/overflow exact; membership exact when capacity sufficient |
| dense (47,040 rows) | 47,040/47,040 score/state/event; 25,508/25,508 membership exact unordered |
| 34k integrated sample | 512/512 score/state/event spot check; 17,429/17,429 membership |

## Capacity behavior

| capacity | event_count | written | overflow | membership |
|---:|---:|---:|---:|---|
| 0 | 17,429 | 0 | 1 | N/A (overflow) |
| 8,714 (half) | 17,429 | 8,714 | 1 | N/A (overflow) |
| 17,429 (exact) | 17,429 | 17,429 | 0 | exact unordered |
| 34,000 | 17,429 | 17,429 | 0 | exact unordered |

## 34k integrated benchmark

| Metric | Value |
|---|---|
| rows | 34,000 |
| dispatches | 2 |
| readback | yes (final) |
| elapsed_ms | ~0.855 |
| per_row_us | ~0.025 |
| event_count | 17,429 |
| overflow | 0 |
| membership | 17,429/17,429 exact unordered |

## 34k warm repeated-dispatch benchmark

| Metric | Value |
|---|---|
| repeats | 32 (each = 2 dispatches) |
| total_ms | ~10.427 |
| per_pipeline_ms | ~0.326 |
| per_row_us | ~0.0096 |
| event_count | 17,429 |
| overflow | 0 |
| membership | 17,429/17,429 |

## Comparison to OBS-4 + EVENT-0 standalone

| Stage | Warm per-dispatch (approx) |
|---|---|
| OBS-4 threshold (standalone) | ~0.233 ms/dispatch |
| EVENT-0 compaction (standalone, 50% density) | ~0.187 ms/dispatch |
| PIPE-0 integrated (both passes) | ~0.326 ms/pipeline |

Integrated warm cost is below the sum of standalone warm dispatches (~0.42 ms), indicating shared setup amortization across the two-pass encoder.

## Tests/scans run

```text
cargo test -p simthing-driver --test phase_m_sead_pipe0_observer_event_pipeline -- --nocapture  → 7/7 PASS
cargo test -p simthing-driver --test phase_m_sead_event0_compaction -- --nocapture            → 7/7 PASS
cargo test -p simthing-driver --test phase_m_sead_obs4_threshold_event -- --nocapture           → 7/7 PASS
cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture             → 16/16 PASS
cargo check --workspace                                                                         → PASS
```

Guardrail scans: no CPU planner/bridge authorization; no deterministic ordering claim; no Candidate C/f64; F artifact hash `e2e9e27601ee2e13` verified in WGSL test.

## Transient cleanup result

No scratch/tmp artifacts deleted under `docs/tests/` (retained exhaustive proof logs).

## Final verdict

**PASS** — SEAD-PIPE-0 landed a default-off/test-only integrated GPU observer-event pipeline; OBS-4 exact threshold event rows feed EVENT-0 compaction without CPU filtering, event count/membership/overflow are exact under the capacity contract, ordering remains explicitly unordered, 34k integrated and warm-dispatch benchmarks were recorded, no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, or production economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / SEAD posture remains intact.
