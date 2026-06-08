# FIELD_POLICY-EVENT-2 — GPU-Resident Per-Bucket Reductions Results

## Base HEAD

`cb6b293` (post-FIELD_POLICY-EVENT-1 merge, pre-EVENT-2)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_field_policy_event2_bucket_reductions.rs` | **New** — 8 tests: semantic-free WGSL, reduction edge, dense corpus, EVENT-1→reduce smoke, PIPE-0→bucket→reduce smoke, 34k perf, warm 32×, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_field_policy_event2_bucket_reductions` descriptor + reduction authority enums |
| `crates/simthing-spec/src/compile/mod.rs`, `lib.rs`, `jit_kernel_descriptor_admission.rs` | exports + registry |
| `crates/simthing-spec/tests/field_policy_obs0_overlay_score_admission.rs` | FIELD_POLICY-EVENT-2 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | FIELD_POLICY-EVENT-2 row |
| `docs/accumulator_op_v2_production_plan.md` | FIELD_POLICY-EVENT-2 section |
| `docs/invariants.md` | GPU bucket reduction invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **EVENT-1 bucket authority:** per-code counts/overflow/invalid ExactAuthoritative; membership ExactAuthoritativeUnordered under capacity; order UnspecifiedAtomicOrder.
2. **Unordered:** bucket record slot order remains unspecified; reductions are order-invariant over scanned records.
3. **Exact reductions:** count, min, max over Q16.16 `score_fixed` are exact when count > 0; sum uses i64 two-limb accumulation.
4. **Sum overflow:** `FLAG_SUM_OVERFLOW` set on i64 checked-add overflow; no silent wrap.
5. **Output authority:** reduction_count/sum/min/max/overflow ExactAuthoritative under declared contracts; empty buckets use `FLAG_EMPTY`.
6. **No CPU filtering:** reduction pass reads GPU bucket buffers directly; integrated smokes chain bucket+reduce (and compact→bucket→reduce) without CPU filtering between GPU passes.

## Reduction layout

Per code (6×u32):

| Offset | Field |
|---:|---|
| 0 | count (scanned records = min(bucket_count, capacity)) |
| 1 | sum_lo |
| 2 | sum_hi |
| 3 | min_score |
| 4 | max_score |
| 5 | flags (bit0 empty, bit1 sum_overflow) |

## Reduction strategy

One workgroup per code (`dispatch CODE_COUNT`); serial scan of `0..min(bucket_counts[code], capacity_per_code)` within each workgroup.

## Sum overflow contract

i64 two-limb signed accumulation with explicit overflow detection; `FLAG_SUM_OVERFLOW` when addition would exceed i64 range.

## Empty-bucket contract

count=0, sum/min/max=0, `FLAG_EMPTY=1`.

## Ordering authority classification

| Aspect | Authority |
|---|---|
| bucket record order | UnspecifiedAtomicOrder |
| reductions (count/sum/min/max) | Order-invariant; ExactAuthoritative under overflow contract |

## Descriptor/admission status

**Landed:** `m_jit_field_policy_event2_bucket_reductions` — default_off, reads bucket_counts + bucket_record, writes reduction outputs.

## Correctness results

| Case | Result |
|---|---|
| edge (9 scenarios) | count/sum/min/max/overflow/empty exact |
| dense (4096×3 codes) | all codes exact |
| EVENT-1 smoke (256 records) | bucket+reduce GPU chain exact |
| PIPE-0 smoke (341 compact records) | counts [0,170,171,0] reductions exact |

## Integrated EVENT-1 smoke

| Metric | Value |
|---|---|
| compact records | 256 |
| GPU passes | bucket + reduce |
| membership | exact |

## Integrated PIPE-0 → EVENT-1 → reductions smoke

| Metric | Value |
|---|---|
| compact events | 341 |
| per-code counts | [0, 170, 171, 0] |
| reductions | exact |

## 34k benchmark (balanced_12, capacity=20k)

| Metric | Value |
|---|---|
| elapsed_ms | ~21.3 (single + readback) |
| per_record_us | ~0.63 |
| counts | [0, 17000, 17000, 0] |

## 34k warm repeated-dispatch

| Metric | Value |
|---|---|
| repeats | 32 |
| total_ms | ~16.8 |
| per_dispatch_ms | ~0.52 |
| per_record_us | ~0.015 |
| counts | [0, 17000, 17000, 0] |

## Tests/scans run

```text
cargo test -p simthing-driver --test phase_m_field_policy_event2_bucket_reductions -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_field_policy_event1_code_bucketing -- --nocapture  → 7/7 PASS
cargo test -p simthing-driver --test phase_m_field_policy_pipe0_observer_event_pipeline -- --nocapture  → 7/7 PASS
cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture  → 20/20 PASS
cargo check --workspace  → PASS
```

No silent sum wrap in fixture (overflow flagged); no deterministic bucket ordering claim.

## Transient cleanup result

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — FIELD_POLICY-EVENT-2 landed a default-off/test-only GPU-resident per-bucket reduction probe consuming unordered event-code buckets; count/sum/min/max/overflow behavior is exact under declared order-invariant and overflow/empty-bucket contracts, integrated EVENT-1 and PIPE-0 pipeline smokes were recorded, 34k timing was recorded, no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, or production economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
