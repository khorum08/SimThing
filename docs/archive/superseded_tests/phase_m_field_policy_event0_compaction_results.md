# FIELD_POLICY-EVENT-0 — GPU-Resident Event Compaction Results

## Base HEAD

`9d86d87` (post-FIELD_POLICY-OBS-4 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_field_policy_event0_compaction.rs` | **New** — 7 tests: semantic-free WGSL, edge compaction, dense corpus, OBS-4 smoke, 34k density perf, warm 32× dispatch, no wiring |
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_field_policy_event0_compaction` descriptor + compaction contracts |
| `crates/simthing-spec/tests/field_policy_obs0_overlay_score_admission.rs` | FIELD_POLICY-EVENT-0 admission tests (+2) |
| `docs/workshop/mapping_current_guidance.md` | FIELD_POLICY-EVENT-0 row |
| `docs/accumulator_op_v2_production_plan.md` | FIELD_POLICY-EVENT-0 section |
| `docs/invariants.md` | Compacted GPU event invariant |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **FIELD_POLICY-OBS-4 event authority:** `state_u32` / `event_code_u32` ExactDeterministicEventFlag under Q16.16 score + threshold.
2. **Row-local today:** one `event_code_u32` per observer row; no compact stream.
3. **Compact record format:** `source_index`, `event_code`, `state`, `score_fixed`, `reserved` (5×u32, numeric only).
4. **Compaction strategy:** atomic counter slot assignment; nonzero `event_code` rows emit records when `slot < capacity`.
5. **Overflow/capacity:** `event_count` = total nonzero rows attempted; `overflow_flag=1` when `slot >= capacity`; records written = `min(event_count, capacity)`.
6. **No CPU planner:** compaction is GPU-resident numeric filtering; no urgency/commitment/bridge semantics.

## Event record layout (5×u32)

| Offset | Field |
|---:|---|
| 0 | source_index_u32 |
| 1 | event_code_u32 |
| 2 | state_u32 |
| 3 | score_fixed (i32 bits) |
| 4 | reserved_u32 |

## Compaction strategy

Atomic `atomicAdd(event_count, 1)` per nonzero row; write record at returned slot if `< capacity`, else set overflow flag. Cross-workgroup order is **not** specified.

## Ordering authority classification

| Aspect | Authority |
|---|---|
| event_count | ExactAuthoritative |
| overflow_flag | ExactAuthoritative |
| event membership | ExactAuthoritativeUnordered (when capacity sufficient) |
| event order | UnspecifiedAtomicOrder |

## Descriptor/admission status

**Landed:** `m_jit_field_policy_event0_compaction` — default_off, reads OBS-4-style event inputs, writes `event_count`, `overflow_flag`, `event_record`.

## Correctness

| Case | Result |
|---|---|
| edge (8 scenarios) | event_count/overflow exact; membership exact when no overflow |
| dense (4096 rows) | 2340/2340 membership exact unordered |
| OBS-4 smoke (34k) | 17,142/17,142 membership exact |
| zero capacity | event_count=1, overflow=1, written=0 |

## Integrated OBS-4 smoke

| Metric | Value |
|---|---|
| input rows | 34,000 |
| nonzero events | 17,142 |
| compact count | 17,142 |
| overflow | 0 |

## 34k density benchmarks (single dispatch + readback)

| density % | event_count | elapsed_ms (approx) | per_row_us (approx) |
|---:|---:|---:|---:|
| 0 | 0 | ~4.1 | ~0.12 |
| 1 | 337 | ~1.8 | ~0.05 |
| 10 | 3,317 | ~1.7 | ~0.05 |
| 50 | 16,805 | ~1.1 | ~0.03 |
| 100 | 34,000 | ~1.4 | ~0.04 |

## 34k warm repeated-dispatch (32×, 50% density)

| Metric | Value |
|---|---|
| total_ms | ~6.5 |
| per_dispatch_ms | ~0.202 |
| per_row_us | ~0.0059 |
| event_count | 16,805 |
| overflow | 0 |

## Tests/scans run

```bash
cargo test -p simthing-driver --test phase_m_field_policy_event0_compaction -- --nocapture  # 7 passed
cargo test -p simthing-driver --test phase_m_field_policy_obs4_threshold_event -- --nocapture  # 7 passed
cargo test -p simthing-driver --test phase_m_field_policy_obs3_fixed_point_score -- --nocapture  # 6 passed
cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture  # 14 passed
cargo check --workspace  # ok
```

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — FIELD_POLICY-EVENT-0 landed a default-off/test-only GPU-resident event compaction probe consuming exact OBS-4 threshold event codes; event count, membership, and overflow behavior are exact under the declared capacity contract, event ordering is explicitly classified unordered, 34k density and warm-dispatch benchmarks were recorded, no CPU planner/urgency/commitment bridge, scheduler/cache/default wiring, semantic WGSL, or production economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
