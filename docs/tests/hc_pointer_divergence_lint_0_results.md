# HC-POINTER-DIVERGENCE-LINT-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Gate-wiring rung; do not self-graduate, undraft, merge, or advance progression.

Rung: HC-POINTER-DIVERGENCE-LINT-0
HD-RECEIPT: a5046f2dc11c
ORIENT-RECEIPT: 5e5bc265ae7b
Classification: gate-wiring / DA-RESERVE(gate-wiring)
ANCHOR-ACK: orientation-harness-core

## What changed

- `gen_orientation.sh --check` / generate: **FAIL(pointer-divergence)** when the authoritative
  `Active open rung` row names a rung whose Exit-proof is already graduation/finished-stamped, or
  names a rung absent from the ladder. Names the row + remedy.
- `next_rung_pointer` completion is **Exit-proof only**. A Scope/deliverable cell that merely
  *describes* completion words (e.g. `DA-GRADUATED / merged [#N]` narrative) no longer false-completes
  the rung (HC-3 source fix).
- `--park` **REFUSES** a divergent authoritative pointer (`FAIL(divergent-pointer)`, zero writes) so
  `--unpark` can never restore divergence (§3a; same family as open-PR refusal; HD-6 transactional).
- `docs/owner_authoring_guide.md`: two-source pointer rule — stamping Exit-proof does not move
  Active open rung; both must be updated at graduation.
- HC-4 Exit-proof cell stamped **PROBATION** leading the cell; orientation regenerated.

## Falsifier (bites)

| control | pre-fix | fixed |
|---|---|---|
| graduated-rung-named-as-pointer | generate/`--check` green (no agreement lint) | `FAIL(pointer-divergence)` |
| unknown-rung (absent from ladder) | generate/`--check` green | `FAIL(pointer-divergence)` + absent remedy |
| scope-cell with `DA-GRADUATED / merged [#N]` narrative, Exit-proof open | `next_rung_pointer` skips to next rung | selects the open narrative rung |
| legitimate not-yet-dispatched next rung | PASS | PASS |
| `none`-form Active open rung | PASS | PASS |
| `--park` with divergent pointer | would store/restore divergence | `FAIL(divergent-pointer)`, tree byte-identical |

Green-both-ways avoided: each FAIL control is proven distinct on the pre-fix rule set inside selftest.

## Validation

- PASS: `bash scripts/ci/gen_orientation.sh --selftest`
- PASS: `bash scripts/ci/agent_scan.sh`
- PASS: `bash scripts/ci/gen_orientation.sh --check`
- PASS: `bash scripts/ci/doc_budget_check.sh --check`
- PASS: live 0.0.8.6 `--unpark` re-proved in disposable sandbox —
  `LIVE-0086-UNPARK-PROOF: PASS receipt=19e0e85c8d3f restored_rows=1 restored_handoffs=1 active_pointer=docs/design_0_0_8_6_studio_live_ops.md`

## Scope Ledger

- `scripts/ci/gen_orientation.sh` — exit-only completion; pointer-divergence lint; park refusal; selftests
- `docs/owner_authoring_guide.md` — two-source rule (under DOC-BUDGET)
- `docs/design_0_0_8_4_8_4_1_harness_corrections.md` — PROBATION leads HC-4 Exit-proof cell
- `docs/orchestrator_orientation.md` — regenerated
- this results doc

Forbidden surfaces not touched: `crates/**`, Studio/UI, `scans.tsv`, clearance router, new tables.
0.0.8.6 park block byte-exact (re-proved).

## Graduation routing

CI verdict: local required-check battery green at committed head.
Risk class: DA-reserve / gate-wiring.
Recommended posture: PROBATION / proof-present / DA-review-pending.
DA stamps graduation at merge; do not self-graduate.
