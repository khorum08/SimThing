# TP-PURGE-0 results — Remand 4 live-mechanism honesty

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **PROBATION / proof-present / DA-review-pending** — Remand 4 `5136696481` discharged locally; Stage B arithmetic preserved `145+73=218`
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `df08db5ebb2d4f8af874cfe151c1aa157100af36` (#1521 FRESHNESS-NO-CARGO)
- Board: `5133929991` · Remands `5134500978` / `5135691949` / `5136003644` / **`5136696481`** · Continuation `5136490881`
- DA: `5135942768` · remainder `5136311181`
- expected_route: `DA-RESERVE(gate-wiring)`
- Rung: `TP-PURGE-0`

## Stage package

| stage | disposition | headline |
|---|---|---|
| A REAP | ACCEPTED | 441/441; authorized deletions **22**; deletion-guard PASS |
| B REPLACE | **CLOSED + Remand 4 honesty** | `145 + 73 = 218`; `inline_unique_cases = 10`; two harnesses |
| C DETACH/DE-NAME/DELETE | ACCEPTED | ceiling **0/0/0**; admission preserved |

## Remand 4 repairs

1. **`accumulator`** — subpath table: transfer, emission, intent, velocity, weighted-mean, owner-silo Sum, bh2 W-compose; each with mechanism-local planted defect
2. **`replay`** — live `ReplayDriver::apply_frame`; defect reverses entry application order
3. **`jit-artifact`** — live `compile_eml_gadget`; defect reverses node encoding order
4. **Tautological reds repaired** — no second-input / post-ser byte-flip / post-return reverse
5. **Owner-silo ordering green** — live disburse on original **and reversed** presentation
6. **Canonical-serialization** — inline packet (no `mobility_scenario0_packet()` factory)

Details: `docs/tests/tp_purge_0_stage_b_results.md`

## Re-prove (Remand 4)

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
