# FIELD_POLICY-EVENT-1 — GPU-Resident Event Code Bucketing Results

## Base HEAD

`6b42d04` (post-FIELD_POLICY-PIPE-0 merge, pre-EVENT-1)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_field_policy_event1_code_bucketing.rs` | **New** — 7 tests: semantic-free WGSL, bucket edge, dense corpus, PIPE-0→bucket smoke, 34k distributions, warm 32×, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_field_policy_event1_code_bucketing` descriptor + bucket authority enums |
| `crates/simthing-spec/src/compile/mod.rs`, `lib.rs`, `jit_kernel_descriptor_admission.rs` | exports + registry |
| `crates/simthing-spec/tests/field_policy_obs0_overlay_score_admission.rs` | FIELD_POLICY-EVENT-1 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | FIELD_POLICY-EVENT-1 row |
| `docs/accumulator_op_v2_production_plan.md` | FIELD_POLICY-EVENT-1 section |
| `docs/invariants.md` | GPU event-code bucket invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **FIELD_POLICY-PIPE-0 authority:** score/state/event exact on intermediate rows; compaction `event_count`/`overflow_flag` ExactAuthoritative; membership ExactAuthoritativeUnordered under capacity; order UnspecifiedAtomicOrder.
2. **Unordered today:** compact record order and (when overflow) which records land in buckets remain UnspecifiedAtomicOrder / capacity-bound only.
3. **Bucket layout:** `CODE_COUNT=4`; codes 1–3 bucket; code 0 ignored; flat `bucket_records[code][slot][5×u32]`.
4. **Capacity/overflow:** per-code `atomicAdd` slot; `bucket_overflow[code]=1` when slot ≥ `capacity_per_code`; `invalid_code_count` for code ≥ CODE_COUNT.
5. **Bucket authority:** counts/overflow/invalid ExactAuthoritative; membership ExactAuthoritativeUnordered when per-code capacity sufficient; order UnspecifiedAtomicOrder.
6. **No CPU filtering:** bucketing pass reads GPU compact records directly; integrated smoke runs threshold + compact + bucket in one encoder without intermediate CPU filtering.

## Bucket layout

| Buffer | Shape | Role |
|---|---|---|
| compact records (input) | 5×u32 × N | source_index, event_code, state, score_fixed, reserved |
| bucket_counts | 4×atomic u32 | per-code attempt totals |
| bucket_overflow | 4×atomic u32 | per-code overflow flags |
| bucket_records | 4 × capacity × 5×u32 | per-code bounded records |
| bucket_meta | 1×atomic u32 | invalid_code_count |

## Bucketing strategy

Per record: skip code 0; count invalid when code ≥ 4; `atomicAdd(bucket_counts[code])`; write record when slot < capacity_per_code else set overflow.

## Code range and invalid-code contract

Valid bucket indices: 1..3 (code 0 ignored, bucket 0 unused). Codes ≥ 4 increment `invalid_code_count` exactly.

## Capacity/overflow contract

Written per code = `min(bucket_counts[code], capacity_per_code)`. Overflow flagged without silent drop of count accounting.

## Ordering authority classification

| Aspect | Authority |
|---|---|
| bucket_counts | ExactAuthoritative |
| bucket_overflow | ExactAuthoritative |
| invalid_code_count | ExactAuthoritative |
| bucket membership | ExactAuthoritativeUnordered (capacity sufficient) |
| bucket order | UnspecifiedAtomicOrder |

## Descriptor/admission status

**Landed:** `m_jit_field_policy_event1_code_bucketing` — default_off, test-only, reads compact event record fields, writes bucket outputs.

## Correctness results

| Case | Result |
|---|---|
| edge (9 scenarios) | counts/overflow/invalid exact; membership exact when no overflow |
| dense (8192 records) | counts/invalid exact; membership exact unordered per code |
| integrated smoke (512 rows, 3 dispatches) | event_count=272; per-code counts exact; membership exact |

## Integrated PIPE-0 smoke

| Metric | Value |
|---|---|
| observer rows | 512 |
| dispatches | 3 |
| event_count | 272 |
| compact overflow | 0 |
| bucket counts | [0, 152, 120, 0] |
| invalid | 0 |
| membership | exact |

## 34k distribution benchmarks (capacity=20,000 per code)

| distribution | elapsed_ms (approx) | per_record_us | counts (1/2/3) | overflow | invalid |
|---|---:|---:|---|---:|---:|
| all_code_1 | ~5.2 | ~0.15 | 34000/0/0 | code1 | 0 |
| balanced_12 | ~2.4 | ~0.07 | 17000/17000/0 | none | 0 |
| balanced_123 | ~2.1 | ~0.06 | 11334/11333/11333 | none | 0 |
| skewed_90_10 | ~2.1 | ~0.06 | 30529/3471/0 | code1 | 0 |
| invalid_mix | ~2.0 | ~0.06 | 10667/10667/10666 | none | 2000 |

## 34k warm repeated-dispatch

| Metric | Value |
|---|---|
| repeats | 32 |
| distribution | balanced_12 |
| total_ms | ~9.07 |
| per_dispatch_ms | ~0.284 |
| per_record_us | ~0.0083 |
| counts | [0, 17000, 17000, 0] |
| overflow | none |
| membership | exact unordered |

## Tests/scans run

```text
cargo test -p simthing-driver --test phase_m_field_policy_event1_code_bucketing -- --nocapture  → 7/7 PASS
cargo test -p simthing-driver --test phase_m_field_policy_pipe0_observer_event_pipeline -- --nocapture  → 7/7 PASS
cargo test -p simthing-driver --test phase_m_field_policy_event0_compaction -- --nocapture  → 7/7 PASS
cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture  → 18/18 PASS
cargo check --workspace  → PASS
```

## Transient cleanup result

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — FIELD_POLICY-EVENT-1 landed a default-off/test-only GPU-resident event-code bucketing probe consuming compact unordered FIELD_POLICY event records; per-code counts, unordered membership, invalid-code accounting, and overflow behavior are exact under the declared capacity contract, ordering remains explicitly unordered, 34k distribution and warm-dispatch benchmarks were recorded, integrated PIPE-0→bucket smoke passed with three GPU passes and no CPU filtering, no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, or production economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
