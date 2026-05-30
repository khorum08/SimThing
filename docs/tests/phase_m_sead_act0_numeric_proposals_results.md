# SEAD-ACT-0 — GPU-Resident Numeric Action Proposals Results

## Base HEAD

`2a3250c` (pre-ACT-0 branch base; ACT-0 lands on `phase-sead-act0-numeric-proposals`)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_sead_act0_numeric_proposals.rs` | **New** — 8 tests: semantic-free WGSL, proposal edge, dense corpus, EVENT-2→proposal smoke, PIPE→proposal smoke, 34k perf, warm 32×, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_sead_act0_numeric_proposals` descriptor + proposal authority enums |
| `crates/simthing-spec/src/compile/mod.rs`, `lib.rs`, `jit_kernel_descriptor_admission.rs` | exports + registry |
| `crates/simthing-spec/tests/sead_obs0_overlay_score_admission.rs` | SEAD-ACT-0 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | SEAD-ACT-0 row |
| `docs/accumulator_op_v2_production_plan.md` | SEAD-ACT-0 section + M/E closure policy note |
| `docs/invariants.md` | SEAD numeric proposal invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **EVENT-2 bucket/reduction authority:** per-code count, i64 sum (lo/hi), min, max, empty-bucket flag, sum-overflow flag — all ExactAuthoritative under order-invariant scan contract; input bucket membership ExactAuthoritativeUnordered under capacity.
2. **Unordered:** bucket record slot order and proposal record slot order remain UnspecifiedAtomicOrder; reductions and proposal membership are multiset-exact when capacity sufficient.
3. **Minimal semantic-free proposal record:** `source_code_u32`, `proposal_code_u32`, `count_u32`, `score_i32`, `flags_u32` — numeric codes only; no resource/order/route semantics.
4. **Numeric rule:** per-code fixed rules — primary `count >= min_count && max_score >= threshold_max`; optional `count >= min_count && sum_score >= threshold_sum` (i64 compare, skipped when sum-overflow flag set).
5. **Output authority:** `proposal_count` and `proposal_overflow` ExactAuthoritative; proposal membership ExactAuthoritativeUnordered under capacity; proposal order UnspecifiedAtomicOrder; individual proposal fields exact under fixed integer rule contract.
6. **M/E closure without production planner:** ACT-0 proves GPU-resident bounded numeric proposals from observer/event stack outputs; Phase E may later interpret proposal codes; no runtime planner, bridge, or default wiring added.

## Proposal record layout

Per proposal (5×u32):

| Offset | Field |
|---:|---|
| 0 | `source_code_u32` (event bucket code) |
| 1 | `proposal_code_u32` (rule output code) |
| 2 | `count_u32` (bucket count at emit) |
| 3 | `score_i32` (representative max_score) |
| 4 | `flags_u32` (rule bit + reduction overflow passthrough) |

Rule flags: bit0 `FLAG_RULE_MAX`, bit1 `FLAG_RULE_SUM`.

## Proposal rule contract

Per-code uniform `ProposalRuleGpu` (8×u32): `min_count`, `threshold_max`, `threshold_sum_lo/hi`, `proposal_code_max/sum`, `enable_sum_rule`.

WGSL `propose_pass`: one workgroup per code; evaluates rules against EVENT-2 reduction row; `atomicAdd` proposal slot; overflow flag when slot ≥ capacity.

## Capacity/overflow contract

`proposal_meta[0]` = attempted proposal count (includes slots beyond capacity). `proposal_meta[1]` = overflow flag when any emit exceeds capacity. Records written only for slots `< proposal_capacity`. Overflow never silent.

## Ordering authority classification

| Aspect | Authority |
|---|---|
| proposal record order | UnspecifiedAtomicOrder |
| proposal membership | ExactAuthoritativeUnordered when capacity sufficient |
| proposal_count / overflow | ExactAuthoritative |
| reduction inputs | ExactAuthoritative under EVENT-2 contract |

## Descriptor/admission status

**Landed:** `m_jit_sead_act0_numeric_proposals` — default_off, production_wiring=false, reads `reduction_count/sum_score/min_score/max_score/reduction_flags`, writes `proposal_count/proposal_overflow_flag/proposal_record`, no sqrt artifact.

## Correctness results

| Case | Result |
|---|---|
| edge (12 scenarios) | proposal_count, overflow, rule flags exact; membership exact when capacity sufficient |
| dense (4096×3 codes) | count=4 overflow=0 membership exact |
| ordering | UnspecifiedAtomicOrder (no deterministic order claim) |

## EVENT-2 → proposal smoke

| Metric | Value |
|---|---|
| compact records | 256 |
| GPU passes | bucket + reduce + propose (3) |
| proposal_count | 4 |
| overflow | 0 |
| membership | exact |

## PIPE-0 → EVENT-1 → EVENT-2 → ACT-0 smoke

Compact corpus mirrors PIPE-0 mobile pattern (341 events). GPU chain: bucket → reduce → propose without CPU filtering between passes. CPU oracle for verification only.

| Metric | Value |
|---|---|
| event_count | 341 |
| bucket_counts | [0, 170, 171, 0] |
| proposal_count | 3 |
| overflow | 0 |
| membership | exact |

## 34k benchmark

| Metric | Value |
|---|---|
| event_count | 34,000 |
| dispatches | 3 (bucket + reduce + propose) |
| elapsed_ms | 13.643 |
| readback | yes |
| proposal_count | 3 |
| overflow | 0 |
| per_record_us | 0.4013 |

## 34k warm repeated-dispatch benchmark

| Metric | Value |
|---|---|
| repeats | 32 |
| total_ms | 39.696 |
| per_pipeline_ms | 1.2405 |
| per_record_us | 0.0365 |
| proposal_count | 3 |
| overflow | 0 |

## Tests/scans run

```text
cargo test -p simthing-driver --test phase_m_sead_act0_numeric_proposals -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_sead_event2_bucket_reductions -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_sead_event1_code_bucketing -- --nocapture  → (green)
cargo test -p simthing-driver --test phase_m_sead_pipe0_observer_event_pipeline -- --nocapture  → (green)
cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture  → 22/22 PASS
cargo check --workspace  → PASS
```

Guardrail scans: ACT-0 symbols present; no deterministic proposal-order claims; guardrail-only planner/bridge references in active docs.

## Transient cleanup result

No scratch/tmp artifacts removed under `docs/tests` (none matched obvious transient pattern).

## M/E Closure Relevance

1. **Substrate added:** SEAD-ACT-0 turns EVENT-2 order-invariant reductions into bounded, semantic-free numeric proposal records on GPU — first actionability probe without CPU planning.
2. **Phase E / Economic V1 consumption:** future economic counters/proposals may read numeric `proposal_code_u32` + scores as local candidate inputs; interpretation stays in spec/Phase E layer.
3. **Not wired to production:** no SimSession default wiring, scheduler/cache, semantic WGSL, CPU urgency traversal, commitment emission, or economy→mapping bridge.
4. **Not a new open-ended SEAD chain:** ACT-0 closes observability→proposal gap on existing EVENT/PIPE stack; further SEAD slices require M/E toolset or closure justification per production plan.
5. **Next M/E-facing step:** wire admitted numeric proposal buffers into a default-off Phase E fixture consumer (counter/proposal admission) without production runtime integration.

## Final verdict

**PASS** — SEAD-ACT-0 landed a default-off/test-only GPU-resident numeric action proposal probe consuming EVENT-2 bucket reductions; proposal count, unordered membership, and overflow behavior are exact under fixed integer rule/capacity contracts; EVENT-2 and PIPE-pattern smokes plus 34k timing were recorded; M/E closure relevance documented; no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, or production economy bridge added; active docs and production plan updated; tests and cargo check green; V7.7 / Mapping ADR / SEAD posture intact.
