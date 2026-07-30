# TP-PURGE-0 results — Remand 5 same-input mutants

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **PROBATION / proof-present / DA-review-pending** — Remand 5 `5137044659` discharged locally; Stage B arithmetic preserved `145+73=218`
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `df08db5ebb2d4f8af874cfe151c1aa157100af36` (#1521 FRESHNESS-NO-CARGO)
- Board: `5133929991` · Remands `5134500978` / `5135691949` / `5136003644` / `5136696481` / **`5137044659`** · Continuation `5136490881`
- DA: `5135942768` · remainder `5136311181`
- expected_route: `DA-RESERVE(gate-wiring)`
- Rung: `TP-PURGE-0`

## Stage package

| stage | disposition | headline |
|---|---|---|
| A REAP | ACCEPTED | 441/441; authorized deletions **22**; deletion-guard PASS |
| B REPLACE | **CLOSED + Remand 5 same-input honesty** | `145 + 73 = 218`; `inline_unique_cases = 10`; two harnesses |
| C DETACH/DE-NAME/DELETE | ACCEPTED | ceiling **0/0/0**; admission preserved |

## Remand 5 repairs (determinism same-semantic-input)

1. **`replay`** — same snapshot+frame to both; red = mutant executor that reverse-applies entries internally
2. **`ordering` / overlay** — presentation order is meaningful; same deltas; red = value-sort mutant planner (not swap input)
3. **`canonical-serialization`** — same inline packet; red = mutant compact RON serializer (not reverse `owner_columns`)
4. **`jit-artifact`** — same SoftStep+opts; red = mutant reverse-encode compiler (not post-return `nodes.reverse()`)
5. **Tautological / changed-input / post-return reds repaired** for the four Remand 5 postures
6. Preserved: Remand 4 accumulator subpaths, owner-silo presentation-order-independent green, mobility-dispatch same-input mutant, arithmetic `145+73=218`

Details: `docs/tests/tp_purge_0_stage_b_results.md`

## Re-prove (Remand 5)

| check | result |
|---|---|
| orient coding | ORIENT-RECEIPT `56fe3e6032b0` |
| Stage A pair-audit | PASS 441/441 |
| deletion-guard | PASS removed=686 authorized=22 |
| detachability + selftest | PASS 0/0/0 |
| inventory drift | PASS |
| lifecycle `--scheduled` | PASS expired=0 |
| closeout `--prove` | PASS |
| `--rungclose TP-PURGE-0` | FAIL diagnostic (expected PROBATION/pointer) |
| workspace build | PASS |
| adapter-pinned driver | **45/0/1 / 13 harnesses** |
| corpus-absent engine | **181/0/1 / 39 harnesses** (restored) |
| `tp_full_transpile_0` | PASS |
| matrix harnesses (adapter-pinned) | **4/0** (`cpu_gpu_parity_matrix_0` + `determinism_matrix_0`) |
| agent_scan | 0 hard FAIL; WORKSHOP-HOMING INSPECT dispositioned |
