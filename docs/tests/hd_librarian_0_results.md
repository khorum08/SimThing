# HD-LIBRARIAN-0 Results

Status: PROBATION
## PR / Branch / Tested Head

- PR: #1337
- Branch: `codex/hd-librarian-0`
- Handoff: `handoffs/HD-LIBRARIAN-0.hd.md`
- `HD-RECEIPT: 561129af1c70`
- Tested code SHA: see PR body `tested_code_sha` for the current head-bound value
- Merge: NOT MERGED
## What Changed

- Added `scripts/ci/librarian.sh` with `--staleness`, `--cull [--confirm]`, `--catalog [--role ...]`, and `--selftest`.
- Added owner previews/reports: `anchor_check.sh --resync --dry-run`, `anchor_query.sh --dead-listeners`, dry-run `anchor_query.sh --prune`, and `doc_budget_check.sh --headroom`.
- `--staleness` composes those owner reports plus `track_closeout.sh --artifact-expiry/--discover` into a complete capped report.
- `--cull` dry-runs by default, delegates reaping to `track_closeout.sh --discover/--decommission`, delegates reach-log pruning to `anchor_query.sh --prune`, and labels owner-reported items as `REAP`, `KEEP`, `DA-ROUTE`, or `ERROR`.
- `--catalog` derives role output from `handoff_dispatch.sh --render`, `orient.sh --role=...`, and `anchor_query.sh --paths`.
- Added `/librarian staleness|cull|catalog` parsing and doctrine-exec workflow plumbing; confirmed culls are owner-only, PR-branch-bound, and non-Owner confirms route to Owner review.
- Confirmed cull is two-phase: dry owner plan + cap/owner-state preflight first, then mutating owner commands only after admission.
- Compact the handoff ingress wrapper so the unchanged HD-LIBRARIAN projection fits the existing 60-line ingress cap without changing the authoritative handoff receipt.
## Load-Bearing Proofs

- `LIBRARIAN-SELFTEST-VERDICT: PASS`.
- `ANCHOR-CHECK-SELFTEST: PASS (7 fixtures)`.
- `ANCHOR-QUERY-SELFTEST: PASS (12 fixtures)`.
- `LIBRARIAN-STALENESS-VERDICT: INSPECT` locally from owner `artifact-expiry` cruft; report is 45 lines and under the reply cap.
- `LIBRARIAN-CULL-VERDICT: DRY` locally; report is 10 lines and no writes occur without `--confirm`.
- `LIBRARIAN-CATALOG-VERDICT: PASS` for coding/orchestrator/da; role reports are 6 lines, 18 lines together, and leave the live reach log byte-identical.
- Parser smoke PASS for `/librarian staleness`, `/librarian cull --confirm`, `/librarian catalog --role da`, and malformed action rejection.
- `HANDOFF-DISPATCH-SELFTEST: PASS`; `--render-ingress 1337 handoffs/HD-LIBRARIAN-0.hd.md` is 59 lines.
- Workflow `/librarian` replies have marker + one-line header plus explicit `reply-line-cap`, and update by event-unique originating command marker.

## Falsifiers

- `cull-dry-run-default`: isolated fixture state remains byte-unchanged.
- `src-path-routes-to-DA`: confirmed cull over fixture owner output containing `crates/**/src/**` emits per-item `DA-ROUTE` and leaves guarded bytes unchanged.
- `staleness-report-cap`: oversized real fixture dead-listener state drives the actual `--staleness` command to nonzero `FAIL(report-line-cap...)`.
- `catalog-per-role`: each role is generated through fixture dispatcher/orientation/anchor inputs, and changing an input changes catalog output.
- Remand-2/3 falsifiers: confirm cap and anchor harness failure leave guarded bytes unchanged; failed/non-Owner confirms cannot commit or run cull; prune/catalog read paths do not create or mutate reach logs; catalog >10 payload markers are not sliced; owner orphan/expired staleness cannot claim aggregate PASS; distinct accepted commands have distinct report markers.

## Scope Ledger

- Classification: gate-wiring.
- Touched: librarian wrapper, anchor preview/query/doc-budget owner reports, doctrine-exec command parser/workflow, handoff ingress wrapper, generated orientation, HD board/evidence docs.
- Untouched: `crates/**`, Studio/UI, scenarios, class TSVs, binding TSVs, anchor TSV contents.
- Deferred: DA deep audit, final graduation stamp, merge, post-merge live `/librarian` comment proof from default branch.

## Known Gaps

- Live same-PR `/librarian` proof is bootstrap-blocked until this workflow change lands on the default branch, because GitHub evaluates `issue_comment` workflows from default branch.
