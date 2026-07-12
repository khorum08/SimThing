# HD-TRUTH-GUARDS-0 Results

Status: PROBATION

## PR / Branch / Tested Head

- PR: #1328
- Branch: `codex/hd-truth-guards-0`
- Base: `6f2d2acc627e195376be01071112dac4fdc331d2`
- Tested code head: `ef261c319f5246474b9d0980bd2cb29ca9d52a67`
- Merge: NOT MERGED

## What Changed

- `relay_lint.sh` now fails graduation/exit-stamp merge claims unless the claimed commit resolves and is an ancestor of `origin/master`.
- `clearance_check.sh` now emits exactly one `body_sha: fresh|evidence-tail|STALE` line from the existing tested-SHA and evidence-tail machinery.
- `relay_lint.sh` now fails self-graduating rung claims unless the rung's own ladder row is stamped in the PR diff.

## Load-Bearing Falsifiers

- Mislanded merge: `RELAY-LINT-VERDICT: FAIL(claimed-merge-not-on-master)`; catches #1316-style non-master graduation claims.
- Unresolvable merge: `RELAY-LINT-VERDICT: FAIL(unresolvable-claimed-merge)`; catches skip-on-resolution-failure.
- Stampless rung: `RELAY-LINT-VERDICT: FAIL(self-rung-stamp-missing)`; catches silent closeout drift.

## Passing Controls

- Ancestor + own row stamp: `RELAY-LINT-VERDICT: PASS`.
- Transport/remedial different-rung record: `RELAY-LINT-VERDICT: PASS`.
- Exact-head body: `CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE`; `body_sha: fresh`.
- Evidence-tail follow-up: `CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE`; `body_sha: evidence-tail`.
- Code delta after tested SHA: `CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE`; `body_sha: STALE`.
- Aggregate relay-lint selftest: `RELAY-LINT-SELFTEST: PASS (34 fixtures)`.
- Aggregate clearance selftest: `CLEARANCE-SELFTEST: PASS (99 fixtures)`.

## Scope Ledger

- Specified: master-ancestry guard, clearance freshness line, self-rung stamp-in-diff guard.
- Implemented: existing relay-lint and clearance-router paths plus focused fixtures.
- Reused: single evidence-tail classifier, existing PR-head/tested-SHA resolution, existing sticky renderer.
- Deferred: DA deep audit, final graduation stamp, merge.
- Untouched: `crates/**`, workflows, HD-2+, Studio/UI, new TSV/classes/verdicts.
- Classification: gate-wiring.

## Sticky Body SHA

- Local fixture proof covers `fresh`, `evidence-tail`, and `STALE`.
- Live PR #1328 sticky is expected to route `DA-RESERVE(gate-wiring)` and show `body_sha: evidence-tail` after this evidence-tail commit.

## Known Gaps

- DA deep audit and final graduation/merge only.
