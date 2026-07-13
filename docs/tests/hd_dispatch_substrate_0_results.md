# HD-DISPATCH-SUBSTRATE-0 Results

Status: PROBATION

## PR / Branch / Tested Head

- PR: #1331
- Branch: `codex/hd-dispatch-substrate-0`
- Base: `c46a6e8cda35f956516895bf2482f0abe009dd2f`
- Required handoff base ancestor: `fd022256b82c30c42da7d51e041128494bf3dd0a`
- Tested code head: `1045d0982139d8264afb3d8d6dfb41df715bc59b`
- Merge: NOT MERGED

## What Changed

- Added strict `.hd.md` handoff object lint/render/receipt/board JSON substrate.
- Added `owner_directives.tsv`; active directives render into every projection, retired rows do not.
- Extended `relay_lint.sh` to enforce `HD-RECEIPT` drift checks and first-handoff bootstrap.
- Extended clearance workflow with handoff ingress sticky and exact-title `SimThing Board` issue sync.
- Remedial pass: workflow resolves handoff from explicit `Rung:` identity, fails duplicate boards,
  normalizes open PR branch/draft/route, and renders each open PR route.

## Load-Bearing Proofs

- `HD-RECEIPT: 990cb20dee0b` for `handoffs/HD-DISPATCH-SUBSTRATE-0.hd.md`.
- `HANDOFF-DISPATCH-SELFTEST: PASS`.
- `RELAY-LINT-SELFTEST: PASS (36 fixtures)`.
- `TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS`.
- `DOC-BUDGET-VERDICT: PASS`.
- `YAML-OK` for `.github/workflows/clearance.yml`.
- `git diff --check` PASS.
- Live clearance sticky on #1331: `DA-RESERVE(gate-wiring)`, `body_sha: evidence-tail`.
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

## Scope Ledger

- Classification: gate-wiring.
- Touched: handoff substrate, relay lint, clearance workflow, fixture inventory, HD docs.
- Untouched: `crates/**`, Studio/UI, scenarios, allowlists, class TSVs, binding/anchor TSVs.
- Deferred: DA deep audit, final graduation stamp, merge.

## Known Gaps

- Current-head GitHub workflow refresh and live `/relay-lint` must be rerun after this remedial push.
