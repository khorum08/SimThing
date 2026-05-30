# SEAD-ACT-1 — Phase E-Style Numeric Proposal Consumer Results

## Base HEAD

`4df11a4` (post-SEAD-ACT-0 merge, pre-ACT-1)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_sead_act1_phase_e_proposal_consumer.rs` | **New** — 8 tests: semantic-free WGSL, consumer edge, dense corpus, ACT-0→consumer smoke, full-chain smoke, 34k perf, warm 32×, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_sead_act1_phase_e_proposal_consumer` descriptor + Phase E consumer authority enums |
| `crates/simthing-spec/src/compile/mod.rs`, `lib.rs`, `jit_kernel_descriptor_admission.rs` | exports + registry |
| `crates/simthing-spec/tests/sead_obs0_overlay_score_admission.rs` | SEAD-ACT-1 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | SEAD-ACT-1 row |
| `docs/accumulator_op_v2_production_plan.md` | SEAD-ACT-1 section + actionability policy note |
| `docs/invariants.md` | Phase E consumer invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **ACT-0 proposal authority:** proposal_count/overflow ExactAuthoritative; membership ExactAuthoritativeUnordered under capacity; order UnspecifiedAtomicOrder; fields exact under fixed integer rules.
2. **Unordered:** proposal record slot order remains unspecified; consumer summary (accepted/invalid counts, sum, max) is order-invariant over scanned records.
3. **Phase E needs:** numeric candidate inputs — admitted proposal codes, aggregate counts, score sum/max — without semantic runtime interpretation.
4. **Fixture-safe consumption:** fixed admitted-code table lookup; accepted/ignored/invalid accounting; i64 summary with explicit overflow flag; no resource/order/route semantics.
5. **Not a CPU planner:** GPU `consume_pass` only; CPU oracle for verification; no urgency traversal, commitment emission, or bridge wiring.
6. **M/E closure:** bridges ACT-0 actionability substrate toward Phase E/Economic V1 fixtures while keeping interpretation in future spec layer.

## Consumer input layout

| Field | Source |
|---|---|
| `proposal_count` | ACT-0 meta[0] |
| `proposal_overflow` | ACT-0 meta[1] |
| `proposal_records` | ACT-0 buffer (5×u32 per record) |

## Consumer output layout (`proposal_summary`, 7×u32)

| Offset | Field |
|---:|---|
| 0 | accepted_count |
| 1 | ignored_count (proposal_count − scanned) |
| 2 | invalid_count |
| 3 | summary_score_lo |
| 4 | summary_score_hi |
| 5 | max_score |
| 6 | flags (bit0 sum_overflow, bit1 input_overflow) |

## Fixed integer admission rule contract

Per proposal: if `proposal_code` ∈ admitted numeric table → accepted (increment count, add score to i64 sum, update max); else invalid. Scanned records = min(proposal_count, proposal_capacity). Ignored = proposal_count − scanned.

## Overflow contract

i64 checked-add for summary; `FLAG_SUM_OVERFLOW` when exceeded. `FLAG_INPUT_OVERFLOW` when ACT-0 proposal overflow flag set. No silent wrap or drop.

## Ordering authority classification

| Aspect | Authority |
|---|---|
| proposal input order | UnspecifiedAtomicOrder (from ACT-0) |
| summary outputs | OrderInvariantExact over scanned multiset |

## Descriptor/admission status

**Landed:** `m_jit_sead_act1_phase_e_proposal_consumer` — default_off, TestOnly lane, reads proposal_count/overflow/record, writes accepted/ignored/invalid/summary_score/max_score/proposal_summary.

## Correctness results

| Case | Result |
|---|---|
| edge (10 scenarios) | accepted/ignored/invalid/summary/max/flags exact |
| dense (128 proposals) | accepted=102 invalid=26 exact |
| ordering | OrderInvariantExact |

## ACT-0 → consumer smoke

| Metric | Value |
|---|---|
| GPU passes | bucket + reduce + propose + consume (4) |
| proposal_count | 4 |
| accepted_count | 4 |
| invalid_count | 0 |
| overflow | 0 |

## Full PIPE/EVENT/ACT → consumer smoke

| Metric | Value |
|---|---|
| compact events | 341 |
| bucket_counts | [0, 170, 171, 0] |
| proposal_count | 3 |
| accepted_count | 3 |
| overflow | 0 |

## 34k benchmark

| Metric | Value |
|---|---|
| event_count | 34,000 |
| dispatches | 4 |
| elapsed_ms | 11.099 |
| readback | yes |
| proposal_count | 3 |
| accepted_count | 3 |
| per_record_us | 0.3264 |

## 34k warm repeated-dispatch benchmark

| Metric | Value |
|---|---|
| repeats | 32 |
| total_ms | 43.226 |
| per_pipeline_ms | 1.3508 |
| per_record_us | 0.0397 |
| accepted_count | 3 |
| overflow | 0 |

## Tests/scans run

```text
cargo test -p simthing-driver --test phase_m_sead_act1_phase_e_proposal_consumer -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_sead_act0_numeric_proposals -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_sead_event2_bucket_reductions -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_sead_pipe0_observer_event_pipeline -- --nocapture  → 7/7 PASS
cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture  → 24/24 PASS
cargo check --workspace  → PASS
```

## Transient cleanup result

No scratch/tmp artifacts removed under `docs/tests`.

## M/E Closure Relevance

1. **Beyond ACT-0:** ACT-1 proves proposal records can be consumed as numeric candidate inputs with exact summaries — first Phase E-style bridge slice.
2. **Phase E / Economic V1:** future fixtures can read accepted_count/summary/max as generic counters without semantic runtime wiring.
3. **Not wired to production:** no SimSession default wiring, scheduler/cache, semantic WGSL, CPU planner, simthing-sim awareness, or economy→mapping bridge.
4. **Not a CPU planner:** fixed integer table lookup and order-invariant aggregation only; no urgency, commitment, or semantic code interpretation.
5. **Next M/E step:** default-off Economic V1 fixture that maps numeric summaries to admitted counter/proposal admission records (still fixture-only, no production bridge).

## Final verdict

**PASS** — SEAD-ACT-1 landed a default-off/test-only Phase E-style numeric proposal consumer; ACT-0 records feed numeric summaries without CPU filtering between GPU passes; accepted/invalid counts and summary outputs are exact under fixed integer/overflow contracts; full-chain and 34k timing recorded; M/E closure relevance documented; no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, simthing-sim awareness, or production economy bridge added; active docs and production plan updated; tests and cargo check green; V7.7 / Mapping ADR / SEAD posture intact.
