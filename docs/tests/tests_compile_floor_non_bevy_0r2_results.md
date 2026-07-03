# TESTS-COMPILE-FLOOR-NON-BEVY-0R2 — crate_checks column guard + stale-profile scrub

## Status
DONE — DA-executed (Fable, executive DA, 2026-07-03). Follows the DA-approved D2s rung (#1112, merge
`4c6c07f8`). Gate-state; DA-held.

## Defect found by the owner (the load-bearing finding)
The 0R forbidden-desktop-dependency lint scanned only the `tests` and `doc_tests` command columns. But the
Doctrine Exec **engine also executes the `crate_checks` column** (`doctrine_exec.sh` runs
`cargo check -p <crate>` for every comma-separated entry). Four already-merged closed-wave profiles carried
blocked crates in that unscanned column:

| Profile | Blocked crates in `crate_checks` (now scrubbed) |
|---|---|
| `test-pare-src-unit-fossil-residue` | `simthing-driver`, `simthing-gpu`, `simthing-mapeditor` |
| `test-pare-gpu-bevy-residue` | `simthing-driver`, `simthing-gpu` |
| `test-pare-protected-pare-delete` | `simthing-driver` |
| `test-pare-conservative-survivor-delete` | `simthing-driver` |

These are the residue of the owner's original observation: a runner reaching `simthing-mapeditor` /
`simthing-driver` `cargo check --tests` would pull the Bevy/ALSA/desktop dependency graph — the exact
Linux-desktop-binary path that must never execute in non-owner-deep GHA. The 0R guard would not have caught
them because it looked at the wrong columns. (They did not fire ALSA installs in practice because those
profiles' *executable* legs were already scrubbed or the crates only appeared in `crate_checks`; but the
column was an open smuggling lane.)

## Fix
1. `doctrine_exec_profile_lint.sh` now scans the **`crate_checks` column** for blocked crates on every
   `owner_deep=false` profile, with a distinct remedy message naming the column.
2. The four stale profiles are scrubbed of blocked crates (closed-wave residue; no live proof lost — those
   waves are merged and their targeted proof already ran).
3. A negative-control prove case (`prove-bad-crate-checks-mapeditor`) is added to the lint's `--prove`
   battery so the column can never silently stop being scanned (rot guard).

## Proof
- Lint clean on the scrubbed tree; `--prove-gha-proof-seal` PASS (includes the new crate_checks case).
- DA perturbation: injected `crate_checks: simthing-core,simthing-tools` → lint **FAIL** with the
  column-named remedy; tree restored clean.
- Full battery green: doctrine scan PASS 0/0; digest PASS; inventory/drift/boundary PASS.
- No `crates/**`, `.github/**`, `scans.tsv`, `allow/**`, or `test_edit_scope.tsv` changes.

## Scope Ledger
`doctrine_exec_profile_lint.sh` (column scan + prove case) + `doctrine_exec_profiles.tsv` (scrub only) +
this doc. No product logic, no workflow, no scanner/allowlist edits.
