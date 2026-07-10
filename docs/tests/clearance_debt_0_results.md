# CLEARANCE-DEBT-0 Results

## Status

**PROBATION / proof-present / DA-review-pending** — gate-wiring remedial debt. Expected clearance: `DA-RESERVE(gate-wiring)`. Do not self-merge.

## Changed files

| Path | Role |
|---|---|
| `scripts/ci/precedented_classes.tsv` | Retire 7 TP classes (`status=retired`) |
| `scripts/ci/class_predicates.tsv` | Drop TP picker + admitted-clause-api predicate rows |
| `scripts/ci/clearance_check.sh` | Triage ancestor evidence-tail; accurate missing-field FAIL; register 3 fixtures |
| `scripts/ci/fixtures/clearance/**` | TP→unclassified expectations; triage pair; missing-field strings; binding/engine repoints |
| `scripts/ci/test_inventory.tsv` | New fixture ledger rows |
| `docs/tests/clearance_debt_0_results.md` | This evidence |
| `docs/orchestrator_orientation.md` | Regenerated (retired class statuses) |

No product implementation files changed. No 0.0.8.6 product rungs implemented.

## TP class retirement

- Before: 7 active TP production/API/picker classes (+ `tp-suspended-demo` suspended)
- After: **−7** retired; `tp-suspended-demo` **kept suspended** for `clearance_selftest_suspended_class`
- Predicates removed: `tp-studio-clause-picker`, `tp-admitted-clause-api-composition`
- Fixture families: TP clearable expectations → `DA-RESERVE(unclassified-scope)`; new `tp_closed_track_no_longer_clearable`

## Triage head-binding (#1169)

Ancestor triage SHA + evidence-tail-only (`docs/tests/**`, `scripts/ci/triage_log.tsv`, `*_results.md`) → clearable. Ancestor + code delta → `DA-RESERVE(triage-missing)`. Immutability Law #1169 / `OH-IMMUTABLE-EVIDENCE-0`: live head equality is not the invariant.

## Missing-field string

`missing coverage_basis only` → `FAIL(missing-coverage-basis: …)` (not `missing-tested-code-sha`). Underscores hyphenated to match harness token style.

## Selftest

`bash scripts/ci/clearance_check.sh --selftest` → **PASS (75 fixtures)**

## agent_scan

`AGENT-SCAN-VERDICT: PASS delta_inspect=0` — `DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0`

## Expected clearance for this PR

`DA-RESERVE(gate-wiring)`
