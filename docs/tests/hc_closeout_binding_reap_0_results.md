# HC-CLOSEOUT-BINDING-REAP-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Gate-wiring rung; do not self-graduate, undraft, merge, or advance progression.

Rung: HC-CLOSEOUT-BINDING-REAP-0
HD-RECEIPT: ff0ed4f7be50
ORIENT-RECEIPT: 5e5bc265ae7b
Classification: gate-wiring / DA-RESERVE(gate-wiring)
ANCHOR-ACK: orientation-harness-core

## What changed

- `scripts/ci/track_closeout.sh --apply` reaps the **closing track's** `discharged` rows from `binding_conditions.tsv`.
  Membership uses the same identity surface as `--park`: ladder rung IDs plus the track's Binding
  conditions markdown table. Active rows stay. Other open/parked tracks' rows stay.
- Reap is HD-6 transactional (preflight plan → stage → write → rollback on fault). Fault injection
  env `TRACK_CLOSEOUT_FAULT_AFTER_BINDING_WRITE=1` proves rollback.
- CLOSEOUT-RECEIPT / apply report now report `binding_reaped=<n>` and `rungs=...`.
- One-time retirement of the 10 dead closed-track rows (TP×4, HU×2, OC×2, HD×2):
  `binding_conditions.tsv` **12 → 2** (keep `HC-TRACK-OPEN-0` discharged + `HC-CLOSEOUT-0` active).
- Protocol doc updated; HC-3 exit-proof cell stamped PROBATION; orientation regenerated.

## Falsifier (bites)

`track_closeout.sh --prove` fixture `binding-reap-*`:

| control | pre-apply | post-apply (fixed) | pre-fix leave-it |
|---|---|---|---|
| closing-track discharged `CLOSE-ME-0` | PRESENT | REMOVED (`binding_reaped=1`) | PRESENT (`binding_reaped=0`) |
| open-track discharged `OPEN-STAY-0` | PRESENT | PRESENT (negative control) | PRESENT |
| closing-track active `CLOSE-ME-ACTIVE-0` | PRESENT | PRESENT | PRESENT |
| rollback fault after binding write | — | table byte-identical to pre-apply | — |

Green-both-ways is avoided: the pre-fix path leaves the closed-track discharged row.

## Validation

- PASS: `bash scripts/ci/track_closeout.sh --prove`
- PASS: `bash scripts/ci/doctrine_selftest.sh`
- PASS: `bash scripts/ci/agent_scan.sh`
- PASS: `bash scripts/ci/gen_orientation.sh --check`
- PASS: `bash scripts/ci/doc_budget_check.sh --check`
- PASS: live 0.0.8.6 `--unpark` re-proved in disposable sandbox (`receipt=19e0e85c8d3f`)

## Scope Ledger

- `scripts/ci/track_closeout.sh` — reap plan + HD-6 write + prove fixtures
- `scripts/ci/binding_conditions.tsv` — net −10 rows (retirement)
- `docs/track_closeout_protocol.md` — apply stage documents binding reaping
- `docs/design_0_0_8_4_8_4_1_harness_corrections.md` — PROBATION leads HC-3 Exit-proof cell
- `docs/orchestrator_orientation.md` — regenerated
- this results doc

Forbidden surfaces not touched: `crates/**`, clearance router, `gen_orientation` logic, new tables,
park block of 0.0.8.6 (byte-exact; reaper does not read/write park blocks).

## Graduation routing

CI verdict: local required-check battery green at committed head.
Risk class: DA-reserve / gate-wiring.
Recommended posture: PROBATION / proof-present / DA-review-pending.
DA stamps graduation at merge; do not self-graduate.
