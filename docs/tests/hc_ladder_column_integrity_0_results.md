# HC-LADDER-COLUMN-INTEGRITY-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Gate-wiring rung; do not self-graduate, undraft, merge, or advance progression.

Rung: HC-LADDER-COLUMN-INTEGRITY-0
HD-RECEIPT: e797620f327f
ORIENT-RECEIPT: 5e5bc265ae7b
Classification: gate-wiring / DA-RESERVE(gate-wiring)
ANCHOR-ACK: orientation-harness-core

## What changed

- `parse_rungs()` now asserts every ladder **data** row has exactly the column count its own
  table header declares. Mismatch → `FAIL(ladder-column-count)` naming the row + remedy
  ("say it without a pipe — backticks do not help; a bare pipe splits too").
- Assert is **only** at the existing `parse_rungs` choke point (active gen/`--check`, `--park`,
  `--unpark`). No repo-wide doc/ladder walk — closed tracks with escaped-pipe ladder rows
  (e.g. `design_0_0_8_4_6_ci_scaffolding.md`) stay green.
- HC-5 Scope cell rewritten without any pipe character (it was itself column-shifted by a
  backtick-wrapped bare pipe while documenting the defect). Exit-proof stamped **PROBATION** leading.
- Orientation regenerated.

## Falsifier (bites)

| control | pre-fix | fixed |
|---|---|---|
| fixture row with escaped pipe in Scope | naive split assigns WRONG exit-proof (`parts[3]` shifted); `--check` would PASS | `FAIL(ladder-column-count)` names `PIPE-ROW` + remedy |
| bare pipe in 5-col Scope | same silent shift | `FAIL(ladder-column-count)` names `BARE-PIPE-ROW` |
| clean 4-col / 5-col row | PASS | PASS (exit-proof cell correct) |

Green-both-ways avoided: pre-fix misread of Exit-proof is proven inside selftest before the fixed path fails.

## Validation

- PASS: `bash scripts/ci/gen_orientation.sh --selftest`
- PASS: `bash scripts/ci/agent_scan.sh`
- PASS: `bash scripts/ci/gen_orientation.sh --check`
- PASS: `bash scripts/ci/doc_budget_check.sh --check`
- PASS: live 0.0.8.6 `--unpark` re-proved in disposable sandbox —
  `LIVE-0086-UNPARK-PROOF: PASS receipt=19e0e85c8d3f restored_rows=1 restored_handoffs=1 active_pointer=docs/design_0_0_8_6_studio_live_ops.md`
- PASS: active HC workplan ladder parses clean (5 columns exactly after Scope rewrite)

## Scope Ledger

- `scripts/ci/gen_orientation.sh` — column-count invariant at `parse_rungs`; selftest falsifier
- `docs/design_0_0_8_4_8_4_1_harness_corrections.md` — PROBATION leads HC-5 Exit-proof; Scope pipe-free
- `docs/orchestrator_orientation.md` — regenerated
- this results doc

Forbidden surfaces not touched: `crates/**`, Studio/UI, `scans.tsv`, clearance router, new tables,
repo-wide ladder scan. 0.0.8.6 park block byte-exact (re-proved).

## Graduation routing

CI verdict: local required-check battery green at committed head.
Risk class: DA-reserve / gate-wiring.
Recommended posture: PROBATION / proof-present / DA-review-pending.
DA stamps graduation at merge; do not self-graduate.
