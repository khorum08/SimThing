# FIELD_POLICY-ACT-3 — Economic V1-Style Fixture Substrate Records Results

## Base HEAD

`f5e27ca` (post-FIELD_POLICY-ACT-2 merge, pre-ACT-3)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_field_policy_act3_economic_fixture_records.rs` | **New** — 8 tests: semantic-free WGSL, fixture edge, dense corpus, ACT-2→fixture smoke, full-chain smoke, 34k perf, warm 32×, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_field_policy_act3_economic_fixture_records` descriptor + Phase E Economic V1 fixture authority |
| `crates/simthing-spec/src/compile/mod.rs`, `lib.rs`, `jit_kernel_descriptor_admission.rs` | exports + registry |
| `crates/simthing-spec/tests/field_policy_obs0_overlay_score_admission.rs` | FIELD_POLICY-ACT-3 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | FIELD_POLICY-ACT-3 row |
| `docs/accumulator_op_v2_production_plan.md` | FIELD_POLICY-ACT-3 section + actionability policy note |
| `docs/invariants.md` | Economic V1 fixture record invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **ACT-2 admission authority:** admission_code, accepted/invalid counts, summary score lo/hi, max_score, flags ExactAuthoritative under fixed integer threshold/overflow contracts; admission reads OrderInvariantExact summary from ACT-1.
2. **Non-semantic and fixture-local:** admission and fixture layers use numeric codes, counts, scores, priority, tier, and flags only — no resource/order/route/buy/sell semantics; GPU-resident passes with CPU oracle for verification only.
3. **Economic V1 substrate record needs:** record_code, source_admission_code, echoed accepted/invalid counts and score fields, priority, tier, flags (emitted + rejection reason + overflow propagation).
4. **Fixed integer mapping sufficient for ACT-3:** admission_code → record_code/priority/tier lookup table; emit when admitted and known code with no overflow; rejection flags otherwise.
5. **Distinct from production bridge:** no scheduler/cache/default SimSession wiring, no economy→mapping bridge, no simthing-sim semantic awareness, no CPU planner between GPU passes.
6. **M/E closure:** converts admitted-candidate artifacts into reusable Economic V1-style numeric fixture substrate for future Phase E authoring without runtime planner behavior.

## Fixture record input layout

| Field | Source |
|---|---|
| `admission_record[7×u32]` | ACT-2 admit output |

| Offset | Field |
|---:|---|
| 0 | admission_code |
| 1 | accepted_count |
| 2 | invalid_count |
| 3 | summary_score_lo |
| 4 | summary_score_hi |
| 5 | max_score |
| 6 | flags (bit0 admitted, bits 1–5 rejection/overflow) |

## Lookup table (uniform + storage)

| Field | Type |
|---|---|
| admission_code | u32 |
| record_code | u32 |
| priority | i32 |
| tier | u32 |

Default table: 5001→9001/100/1, 5002→9002/200/2, 5003→9003/300/3.

## Fixture record output layout (`fixture_record`, 10×u32)

| Offset | Field |
|---:|---|
| 0 | record_code |
| 1 | source_admission_code |
| 2 | accepted_count |
| 3 | invalid_count |
| 4 | summary_score_lo |
| 5 | summary_score_hi |
| 6 | max_score |
| 7 | priority |
| 8 | tier |
| 9 | flags |

## Fixed integer mapping rule contract

Emit record when ALL hold:

- admission flags bit0 (admitted) set
- admission_code found in lookup table
- no input_overflow (propagated from admission bit4)
- no summary_overflow (propagated from admission bit5)

Otherwise emit rejected fixture record with reason flags.

## Fixture output flags

| Bit | Name |
|---:|---|
| 0 | record_emitted |
| 1 | rejected_not_admitted |
| 2 | rejected_unknown_admission_code |
| 3 | input_overflow |
| 4 | summary_overflow |
| 5 | reserved |

## Overflow contract

ACT-2 overflow flags propagate to fixture flags (bits 3–4). Fixture never silently emits when overflow is set. No silent wrap or drop.

## Ordering authority classification

| Aspect | Authority |
|---|---|
| admission_record input | ExactAuthoritative (from ACT-2) |
| fixture record | ExactAuthoritative under fixed integer lookup contract |

## Descriptor/admission status

**Landed:** `m_jit_field_policy_act3_economic_fixture_records` — default_off, TestOnly lane, reads admission_record, writes fixture_record/record_code under fixed integer lookup/overflow contracts.

## Correctness results

| Case | Result |
|---|---|
| edge (10 scenarios) | record_code, priority, tier, flags, rejection reason exact |
| dense (64 admission rows) | fixture records + flags exact |
| overflow propagation | input/summary overflow flags exact |

## ACT-2 → fixture record smoke

| Metric | Value |
|---|---|
| GPU passes | bucket + reduce + propose + consume + admit + fixture (6) |
| admission_code | 5001 |
| admission_flags | 1 |
| record_code | 9001 |
| record_flags | 1 |
| priority | 100 |
| tier | 1 |
| CPU filtering between passes | none |

## Full PIPE/EVENT/ACT → fixture record smoke

| Metric | Value |
|---|---|
| compact events | 341 |
| event_count | 341 |
| bucket_counts | [0, 170, 171, 0] |
| proposal_count | 3 |
| accepted_count | 3 |
| admission_code | 5001 |
| record_code | 9001 |
| record_flags | 1 |
| overflow | 0 |

## 34k benchmark

| Metric | Value |
|---|---|
| event_count | 34,000 |
| dispatches | 6 |
| elapsed_ms | 1.511 |
| readback | yes |
| proposal_count | 3 |
| accepted_count | 3 |
| admission_code | 5001 |
| record_code | 9001 |
| overflow | 0 |
| per_record_us | 0.0444 |

## 34k warm repeated-dispatch benchmark

| Metric | Value |
|---|---|
| repeats | 32 |
| total_ms | 38.108 |
| per_pipeline_ms | 1.1909 |
| per_record_us | 0.0350 |
| record_code | 9001 |
| overflow | 0 |

## Tests/scans run

```text
cargo test -p simthing-driver --test phase_m_field_policy_act3_economic_fixture_records -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_field_policy_act2_proposal_admission_records -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_field_policy_act1_phase_e_proposal_consumer -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_field_policy_act0_numeric_proposals -- --nocapture  → 8/8 PASS
cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture  → 28/28 PASS
cargo check --workspace  → PASS
```

Scans (Grep tool; `rg` unavailable in shell):

| Scan | Result |
|---|---|
| `field_policy_act3\|economic_fixture\|fixture_record\|m_jit_field_policy_act3` in crates/docs | ACT-3 test/spec/docs present |
| forbidden semantic/planner/bridge terms in ACT-3 report + production plan + invariants | guardrail-only references |
| `proposal_order.*Exact\|ordered proposal.*Exact\|atomic.*ordered` | no unauthorized deterministic ordering claims |
| production wiring / scheduler / bridge authorization | guardrail-only; no authorization |
| Candidate C / f64 / SHADER_F64 | no implementation |
| `docs/tests` scratch/tmp | no obvious scratch/tmp deleted (E-phase evidence logs retained) |

## Transient cleanup result

No scratch/tmp artifacts removed under `docs/tests` (E-phase/E11 evidence logs retained).

## M/E Closure Relevance

1. **Beyond ACT-2:** ACT-3 proves fixture-local admission records can become Economic V1-style numeric substrate records with record_code, priority, tier, and emission flags — a reusable fixture artifact shape ACT-2 did not emit.
2. **Phase E / Economic V1:** future fixtures can consume record_code, priority, tier, and echoed score/count fields as authorable numeric substrate without semantic runtime wiring or CPU planner behavior.
3. **Not wired to production:** no SimSession default wiring, scheduler/cache, semantic WGSL, CPU planner, simthing-sim awareness, or economy→mapping bridge.
4. **Not a CPU planner:** fixed integer lookup table only; no urgency traversal, commitment emission, or semantic code interpretation.
5. **Next M/E step:** default-off Economic V1 fixture that chains substrate records into authorable counter/proposal validation corpus (still fixture-only, no production bridge).

FIELD_POLICY-ACT-3 proves that fixture-local admission records can be converted into Economic V1-style numeric substrate records under fixed integer mapping contracts. This gives Phase E a reusable numeric fixture artifact shape without authorizing runtime planner behavior, semantic interpretation, scheduler/cache/default wiring, simthing-sim awareness, or economy bridge wiring.

## Final verdict

**PASS** — FIELD_POLICY-ACT-3 landed a default-off/test-only Economic V1-style fixture substrate record layer; ACT-2 admission records feed exact fixture records under fixed integer mapping/overflow contracts without CPU filtering; full PIPE/EVENT/ACT-to-fixture smoke and 34k timing were recorded; M/E closure relevance was documented; no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, simthing-sim awareness, or production economy bridge was added; active docs and production plan were updated; tests and cargo check are green; V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
