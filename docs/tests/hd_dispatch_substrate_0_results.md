# HD-DISPATCH-SUBSTRATE-0 Results

Status: PROBATION

## PR / Branch / Tested Head

- PR: pending draft
- Branch: `codex/hd-dispatch-substrate-0`
- Base: `c46a6e8cda35f956516895bf2482f0abe009dd2f`
- Required handoff base ancestor: `fd022256b82c30c42da7d51e041128494bf3dd0a`
- Tested code head: pending commit
- Merge: NOT MERGED

## What Changed

- Added strict `.hd.md` handoff object lint/render/receipt/board JSON substrate.
- Added `owner_directives.tsv`; active directives render into every projection, retired rows do not.
- Extended `relay_lint.sh` to enforce `HD-RECEIPT` drift checks and first-handoff bootstrap.
- Extended clearance workflow with handoff ingress sticky and exact-title `SimThing Board` issue sync.

## Load-Bearing Proofs

- `HD-RECEIPT: 990cb20dee0b` for `handoffs/HD-DISPATCH-SUBSTRATE-0.hd.md`.
- `HANDOFF-DISPATCH-SELFTEST: PASS`.
- `RELAY-LINT-SELFTEST: PASS (36 fixtures)`.
- `TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS`.
- `DOC-BUDGET-VERDICT: PASS`.
- `YAML-OK` for `.github/workflows/clearance.yml`.
- `git diff --check` PASS.

## Falsifiers

- Draft handoff blocks coding projection with `owner-approval-required`.
- Missing/unknown frontmatter keys and 81-line body fixtures fail.
- Relay bootstrap fixture passes only when the added `.hd.md` receipt matches.
- Relay drift fixture fails when a claimed receipt differs from the base object.

## Scope Ledger

- Classification: gate-wiring.
- Touched: handoff substrate, relay lint, clearance workflow, fixture inventory, HD docs.
- Untouched: `crates/**`, Studio/UI, scenarios, allowlists, class TSVs, binding/anchor TSVs.
- Deferred: DA deep audit, final graduation stamp, merge.

## Known Gaps

- Live GitHub sticky and board issue must be observed after the draft PR is opened.
