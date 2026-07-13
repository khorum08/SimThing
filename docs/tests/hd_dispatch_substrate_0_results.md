# HD-DISPATCH-SUBSTRATE-0 Results

Status: PROBATION

## PR / Branch / Tested Head

- PR: #1331
- Branch: `codex/hd-dispatch-substrate-0`
- Base: `c46a6e8cda35f956516895bf2482f0abe009dd2f`
- Required handoff base ancestor: `fd022256b82c30c42da7d51e041128494bf3dd0a`
- Tested code head: `a7d5edc6c8b122c69cfbe4f2bf225fa2ab5d1215`
- Merge: NOT MERGED

## What Changed

- Added strict `.hd.md` handoff object lint/render/receipt/board JSON substrate.
- Added `owner_directives.tsv`; active directives render into every projection, retired rows do not.
- Extended `relay_lint.sh` to enforce `HD-RECEIPT` drift checks and first-handoff bootstrap.
- Extended clearance workflow with handoff ingress sticky and exact-title `SimThing Board` issue sync.
- Remedial pass: workflow resolves handoff from explicit `Rung:` identity, fails duplicate boards,
  normalizes open PR branch/draft/route, and renders each open PR route.
- Remand-2 pass: `handoff_dispatch.sh` reuses `anchor_query.sh --paths`; board and
  handoff ingress render complete-or-fail under the 60-line cap; board issue lookup is paginated.

## Load-Bearing Proofs

- `HD-RECEIPT: 990cb20dee0b` for `handoffs/HD-DISPATCH-SUBSTRATE-0.hd.md`.
- `HANDOFF-DISPATCH-SELFTEST: PASS`.
- `RELAY-LINT-SELFTEST: PASS (36 fixtures)`.
- `TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS`.
- `DOC-BUDGET-VERDICT: PASS`.
- `YAML-OK` for `.github/workflows/clearance.yml`.
- `git diff --check` PASS.
- `AGENT-SCAN-VERDICT: PASS delta_inspect=0`.
- Shared anchor resolver proof: `REQUIRED-ANCHORS: orientation-harness-core` comes from
  `anchor_query.sh --paths`, with no separate trigger matcher in `handoff_dispatch.sh`.
- Complete board proof: live board digest is under 60 lines; oversize board fails `board-line-cap`.
- Complete ingress proof: live ingress sticky is under 60 lines; oversize wrapper fails `ingress-line-cap`.
- Paginated board lookup proof: slurped issue pages still resolve single board issue #1332.
- Live clearance sticky on #1331 corrected earlier to `DA-RESERVE(gate-wiring)`, `body_sha: evidence-tail`.
- Live handoff sticky on #1331: `HD-LINT-VERDICT: PASS`, coding projection receipt matches.
- Live board issue: #1332; remedial helper targets it for update, not duplicate creation.

## Falsifiers

- Draft handoff blocks coding projection with `owner-approval-required`.
- Missing/unknown frontmatter keys and 81-line body fixtures fail.
- Relay bootstrap fixture passes only when the added `.hd.md` receipt matches.
- Relay drift fixture fails when a claimed receipt differs from the base object.
- Explicit-rung resolver fails when changed `.hd.md` does not match the PR body rung.
- Duplicate `SimThing Board` issue fixture fails instead of selecting one with `head -n 1`.
- Board render fixture proves `head`, `draft`, and `route` appear on open PR lines.
- Oversize board and oversize handoff ingress fixtures fail instead of truncating required content.
- Paginated issue-list fixture resolves the existing board issue instead of creating a duplicate.

## Scope Ledger

- Classification: gate-wiring.
- Touched: handoff substrate, relay lint, clearance workflow, fixture inventory, HD docs.
- Untouched: `crates/**`, Studio/UI, scenarios, allowlists, class TSVs, binding/anchor TSVs.
- Deferred: DA deep audit, final graduation stamp, merge.

## Known Gaps

- DA deep audit, final graduation stamp, and merge remain blocked.
