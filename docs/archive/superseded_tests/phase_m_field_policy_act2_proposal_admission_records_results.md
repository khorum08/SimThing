# FIELD_POLICY-ACT-2 — Fixture-Local Proposal Admission Records Results

## Base HEAD

`3391b6f` (post-FIELD_POLICY-ACT-1 merge, pre-ACT-2)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_field_policy_act2_proposal_admission_records.rs` | **New** — 8 tests: semantic-free WGSL, admission edge, dense corpus, ACT-1→admission smoke, full-chain smoke, 34k perf, warm 32×, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_field_policy_act2_proposal_admission_records` descriptor + Phase E fixture admission authority |
| `crates/simthing-spec/src/compile/mod.rs`, `lib.rs`, `jit_kernel_descriptor_admission.rs` | exports + registry |
| `crates/simthing-spec/tests/field_policy_obs0_overlay_score_admission.rs` | FIELD_POLICY-ACT-2 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | FIELD_POLICY-ACT-2 row |
| `docs/accumulator_op_v2_production_plan.md` | FIELD_POLICY-ACT-2 section + actionability policy note |
| `docs/invariants.md` | fixture-local admission invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **ACT-1 numeric summary authority:** accepted/ignored/invalid counts, i64 summary score, max_score, and flags ExactAuthoritative under fixed integer admitted-code table and explicit overflow contracts; summary outputs OrderInvariantExact over scanned proposal multiset; proposal input order remains UnspecifiedAtomicOrder from ACT-0.
2. **Unordered:** proposal record slot order; summary field aggregation order over scanned records. Admission reads summary fields only — no ordering claim on proposals.
3. **Fixture-local admission record needs:** admission_code (from rules), echoed accepted/invalid counts, echoed summary score lo/hi and max_score, flags (admitted + rejection reason bits + overflow propagation).
4. **Fixed integer admission rules sufficient for ACT-2:** `min_accepted`, `min_max_score`, `max_invalid` thresholds; reject when ACT-1 `input_overflow` or `summary_overflow` flags set; no f32 arithmetic; no semantic interpretation.
5. **Distinct from production bridge:** GPU `admit_pass` only between consume and readback; CPU oracle for verification; no scheduler/cache/default SimSession wiring, no economy→mapping bridge, no simthing-sim semantic awareness.
6. **M/E closure:** converts Phase E-style numeric summaries into a stable, bounded, replayable admitted-candidate artifact shape for future Economic V1 fixtures without authorizing planner or semantic action behavior.

## Admission input layout

| Field | Source |
|---|---|
| `proposal_summary[7×u32]` | ACT-1 consume output |

| Offset | Field |
|---:|---|
| 0 | accepted_count |
| 1 | ignored_count |
| 2 | invalid_count |
| 3 | summary_score_lo |
| 4 | summary_score_hi |
| 5 | max_score |
| 6 | flags (bit0 sum_overflow, bit1 input_overflow) |

## Admission rules (uniform)

| Field | Type |
|---|---|
| admission_code | u32 |
| min_accepted | u32 |
| min_max_score | i32 |
| max_invalid | u32 |

Default edge rules: code 5001, min_accepted=1, min_max_score=0, max_invalid=10. Smoke/benchmark rules: max_invalid=100.

## Admission output layout (`admission_record`, 7×u32)

| Offset | Field |
|---:|---|
| 0 | admission_code |
| 1 | accepted_count |
| 2 | invalid_count |
| 3 | summary_score_lo |
| 4 | summary_score_hi |
| 5 | max_score |
| 6 | flags |

## Fixed integer admission rule contract

Admitted when ALL hold:

- `accepted_count >= min_accepted`
- `max_score >= min_max_score`
- `invalid_count <= max_invalid`
- ACT-1 input_overflow flag == 0
- ACT-1 summary_overflow flag == 0

Otherwise admitted=0 with rejection reason bits set.

## Admission output flags

| Bit | Name |
|---:|---|
| 0 | admitted |
| 1 | rejected_below_count |
| 2 | rejected_below_score |
| 3 | rejected_invalid_nonzero |
| 4 | input_overflow |
| 5 | summary_overflow |

## Overflow contract

ACT-1 overflow flags propagate to admission flags (bits 4–5). Admission never silently admits when overflow is set. No silent wrap or drop.

## Ordering authority classification

| Aspect | Authority |
|---|---|
| proposal_summary input | OrderInvariantExact (from ACT-1) |
| admission record | ExactAuthoritative under fixed integer rule contract |

## Descriptor/admission status

**Landed:** `m_jit_field_policy_act2_proposal_admission_records` — default_off, TestOnly lane, reads proposal_summary, writes admission_record under fixed integer threshold/overflow contracts.

## Correctness results

| Case | Result |
|---|---|
| edge (12 scenarios) | admission_code, flags, rejection reason exact |
| dense (64 summary rows) | admission + flags exact; order-invariant summary input |
| overflow propagation | input_overflow and summary_overflow reject with correct flags |

## ACT-1 → admission smoke

| Metric | Value |
|---|---|
| GPU passes | bucket + reduce + propose + consume + admit (5) |
| proposal_count | 4 |
| accepted_count | 4 |
| admission_code | 5001 |
| flags | 1 (admitted) |
| CPU filtering between passes | none |

## Full PIPE/EVENT/ACT → admission smoke

| Metric | Value |
|---|---|
| compact events | 341 |
| event_count | 341 |
| bucket_counts | [0, 170, 171, 0] |
| proposal_count | 3 |
| accepted_count | 3 |
| admission_code | 5001 |
| flags | 1 |
| overflow | 0 |

## 34k benchmark

| Metric | Value |
|---|---|
| event_count | 34,000 |
| dispatches | 5 |
| elapsed_ms | 48.995 |
| readback | yes |
| proposal_count | 3 |
| accepted_count | 3 |
| admission_code | 5001 |
| overflow | 0 |
| per_record_us | 1.4410 |

## 34k warm repeated-dispatch benchmark

| Metric | Value |
|---|---|
| repeats | 32 |
| total_ms | 51.031 |
| per_pipeline_ms | 1.5947 |
| per_record_us | 0.0469 |
| admission_code | 5001 |
| flags | 1 |
| overflow | 0 |

## Tests/scans run

```text
cargo test -p simthing-driver --test phase_m_field_policy_act2_proposal_admission_records -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_field_policy_act1_phase_e_proposal_consumer -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_field_policy_act0_numeric_proposals -- --nocapture  → 8/8 PASS
cargo test -p simthing-driver --test phase_m_field_policy_event2_bucket_reductions -- --nocapture  → 8/8 PASS
cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture  → 26/26 PASS
cargo check --workspace  → PASS
```

Scans (Grep tool; `rg` unavailable in shell):

| Scan | Result |
|---|---|
| `field_policy_act2\|proposal_admission\|admission_record\|m_jit_field_policy_act2` in crates/docs | ACT-2 test/spec present; docs updated |
| forbidden semantic/planner/bridge terms in ACT-2 report + production plan + invariants | guardrail-only references |
| `proposal_order.*Exact\|ordered proposal.*Exact\|atomic.*ordered` | no unauthorized deterministic ordering claims |
| production wiring / scheduler / bridge authorization | guardrail-only; no authorization |
| Candidate C / f64 / SHADER_F64 | no implementation |
| `docs/tests` scratch/tmp | no obvious scratch/tmp deleted (E-phase evidence logs retained) |

## Transient cleanup result

No scratch/tmp artifacts removed under `docs/tests` (E-phase/E11 evidence logs retained).

## M/E Closure Relevance

1. **Beyond ACT-1:** ACT-2 proves Phase E-style numeric proposal summaries can become fixture-local admission records under fixed integer threshold/overflow contracts — a stable admitted-candidate artifact shape ACT-1 did not emit.
2. **Phase E / Economic V1:** future fixtures can read admission_code, echoed counts/scores, and admitted/rejection flags as generic substrate without semantic runtime wiring or CPU planner behavior.
3. **Not wired to production:** no SimSession default wiring, scheduler/cache, semantic WGSL, CPU planner, simthing-sim awareness, or economy→mapping bridge.
4. **Not a CPU planner:** fixed integer threshold comparison only; no urgency traversal, commitment emission, or semantic code interpretation.
5. **Next M/E step:** default-off Economic V1 fixture that chains admission records into authorable counter/proposal substrate records (still fixture-only, no production bridge).

FIELD_POLICY-ACT-2 proves that Phase E-style numeric proposal summaries can become fixture-local admission records under fixed integer contracts. This gives Phase E a reusable admitted-candidate artifact shape without authorizing runtime planner behavior, semantic interpretation, scheduler/cache/default wiring, simthing-sim awareness, or economy bridge wiring.

## Final verdict

**PASS** — FIELD_POLICY-ACT-2 landed a default-off/test-only fixture-local proposal admission record layer; ACT-1 numeric summaries feed exact admission records under fixed integer threshold/overflow contracts without CPU filtering; full PIPE/EVENT/ACT-to-admission smoke and 34k timing were recorded; M/E closure relevance was documented; no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, simthing-sim awareness, or production economy bridge was added; active docs and production plan were updated; tests and cargo check are green; V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
