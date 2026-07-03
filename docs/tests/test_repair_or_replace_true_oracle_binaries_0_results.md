# TEST-REPAIR-OR-REPLACE-TRUE-ORACLE-BINARIES-0 Results

## Status

PROBATION - pending DA review.

This rung retired non-compiling fossil test binaries surfaced by the D2o major finding and by the required iterative `cargo check -p <crate> --tests` floor. It does not self-mark COMPLETE and is not merge-cleared without DA review.

## PR / branch / merge

- Branch: `test-repair-or-replace-true-oracle-binaries-0`
- PR: pending
- Base SHA: `6c637fcd89f2293408656825186e515618737c57`
- Merge SHA, if merged: none

## Pre-flight verification

- `master` contains #1105 merge `6c637fcd89f2293408656825186e515618737c57`.
- Opening Track D inventory: 5,147 rows.
- The six closed D2o `td-csd-*` edit-scope rows were present before this rung and were removed as the first edit.
- `docs/tests/test_pare_conservative_survivor_delete_0_results.md` records the D2o major finding and names this follow-on.

## Discovery sweep

The required compile sweep was run with exact `cargo check -p <crate> --tests` commands over `simthing-core`, `simthing-kernel`, `simthing-sim`, `simthing-driver`, `simthing-spec`, `simthing-workshop`, `simthing-clausething`, `simthing-mapgenerator`, and `simthing-gpu`.

The first sweep found broken binaries in `simthing-sim`, `simthing-driver`, `simthing-spec`, `simthing-clausething`, and `simthing-gpu`. Iterative driver sweeps then exposed dependent fossil callers until the full floor passed.

Durable review table: `docs/tests/test_repair_or_replace_true_oracle_binaries_0_review.tsv`.

## What changed

- Removed the closed D2o temporary scope rows.
- Added temporary scope rows for this rung's touched test/source-test surfaces.
- Deleted 124 non-compiling fossil test binaries: 92 `simthing-driver`, 20 `simthing-sim`, 6 `simthing-clausething`, 3 `simthing-gpu`, and 3 `simthing-spec`.
- Removed 925 matching live inventory rows and recorded historical `PARED` audit rows under this deletion wave.
- Removed protected coverage rows that pointed at deleted/non-compiling owners.
- Added the targeted Doctrine Exec profile `test-repair-or-replace-true-oracle-binaries`.
- Added/updated bounded `sentinel-core` to carry the same tests-compile floor.
- Made narrow `#[cfg(test)]` driver source fixes for current slot newtypes.

## Deleted / replaced / repaired

- Deleted: 925 inventory rows / 124 files.
- Replaced: none.
- Repaired: test-only driver source modules for `SlotIndex` API adoption.

Review-table candidate sources: `dead-binary` 71 rows, `dress-rehearsal-lineage` 9 rows, `discovery-sweep` 845 rows.

## Coverage re-key

Coverage rows whose surviving owner was a deleted/non-compiling binary were removed from `docs/tests/protected_class_oracle_parity_coverage.tsv`. Existing compiling owners remain for live retained surfaces; fossil-only surfaces are recorded in the review table as retired rather than kept as false coverage claims.

## Tests-compile floor

Added GHA-safe compile-only commands to profile `test-repair-or-replace-true-oracle-binaries` and `sentinel-core`:

```bash
cargo check -p simthing-core --tests
cargo check -p simthing-kernel --tests
cargo check -p simthing-sim --tests
cargo check -p simthing-driver --tests
cargo check -p simthing-spec --tests
cargo check -p simthing-workshop --tests
cargo check -p simthing-clausething --tests
cargo check -p simthing-mapgenerator --tests
cargo check -p simthing-gpu --tests
```

## Load-bearing proofs

- Discovery sweep: review table records every retired row and catches hidden dead binaries outside the D2o seed set.
- `cargo check --tests` floor: PASS for all nine listed crates; catches this dead-binary class reforming.
- Replacement tests: none; no replacement proofs were added.
- Coverage map check: stale coverage rows pointing at deleted files removed; catches false coverage owners.
- Scope gate: PASS; catches lingering closed-rung scope authority and unauthorized edit surfaces.
- Profile lint + GHA proof-seal: PASS; catches accidental Atlas/Bevy/GPU/desktop proof in non-owner-deep GHA profiles.
- Doctrine Scan: PASS, failures=0 inspect=0.
- Targeted Doctrine Exec: pending PR `/seal-proof profile=test-repair-or-replace-true-oracle-binaries`.

## Scope Ledger

- Specified: delete/replace/repair dead TRUE_MEMBER oracle/parity fossil binaries; re-key coverage; add compile floors; keep PR PROBATION.
- Implemented: deletion-dominant cleanup, coverage pruning, historical audit rows, inventory update, profile floor, source-test API repair.
- Proxied: none.
- Deferred: DA review of retired fossil-only coverage surfaces; live targeted Doctrine Exec proof after PR creation.
- Out of scope: product logic changes, scanner/allowlist edits, mapeditor/tools GHA runtime proof, Atlas/Bevy/GPU runtime proof.

## Known gaps / next

- PR remains PROBATION pending DA review.
- Targeted Doctrine Exec must be run against the PR merge ref before clearance.
- Temporary `td-torb-*` scope rows should be re-sealed after this rung closes.

## Graduation routing

- CI verdict: pending final live CI.
- Triage entries: none locally.
- Risk class: test deletion + coverage re-key + gate-wiring + temporary scope rows.
- Falsification check: DA runs the cargo-check `--tests` floor, verifies every live oracle coverage row points at a compiling owner, perturbs one dead binary reference to prove the compile floor fails, and checks the review table against the inventory/boundary diff.
- Recommended posture: deep - deletion against formerly TRUE_MEMBER / oracle-parity claims plus gate-wiring requires DA inspection, not light review.
