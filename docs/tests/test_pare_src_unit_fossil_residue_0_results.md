# TEST-PARE-SRC-UNIT-FOSSIL-RESIDUE-0 Results

## Status

**PROBATION** — local gates and `cargo check` complete; GitHub Doctrine Exec pending on PR merge ref.

## Mission

Delete 88 `src/**` unit-test fossil rows (82 admission-adjacent, 6 usecase-superseded) classified as boundary COLLAPSE/DELETE candidates across seven protected crates. No never-pare, oracle-parity, seal-proof, or compile_fail rows touched.

## Predecessor closeout

- **Wave 1 #1098 `TEST-PARE-BROKEN-CLAUSETHING-ADMISSION-RESIDUE-0`**: **DONE** — merged; inventory was 5671 before this wave.

## Disposition

Review table: `docs/tests/test_pare_src_unit_fossil_residue_0_review.tsv`

| Metric | Count |
|---|---:|
| rows considered | 88 |
| rows deleted | 88 |
| representatives kept (elsewhere) | per boundary ledger |
| rows blocked | 0 |

By crate: mapeditor 42, kernel 15, core 9, sim 9, driver 6, gpu 6, feeder 1.

By class: admission-adjacent 82, usecase-superseded 6.

By disposition: COLLAPSE_TO_REPRESENTATIVE 77, DELETE 11.

Inventory: 5671 → **5583** (−88).

## Scope gate

`scripts/ci/test_edit_scope_check.sh` now allows temporary protected-crate `src/**` scope rows when rationale contains `Owner/DA-approved` and the glob is src-only.

Temporary scope rows added for each touched crate under profile `test-pare-src-unit-fossil-residue` / risk class `test-deletion-src-unit-fossil-residue`. Rows remain in this PR so `test_inventory_check.sh` can authorize the branch `src/**` diff against `origin/master`; retirement_condition marks wave consumption.

No production logic outside `#[cfg(test)]`, no `Cargo.toml`, no `.github/**`, no product repairs.

## Targeted proof

Profile `test-pare-src-unit-fossil-residue` uses `cargo check -p` only for touched crates:

- `simthing-core`
- `simthing-feeder`
- `simthing-kernel`
- `simthing-sim`
- `simthing-driver`
- `simthing-gpu`
- `simthing-mapeditor`

No full-crate test sweep, workspace test, owner-deep, GPU session, Bevy, or desktop proof.

## Required gates

Local gate proof: see commit message / PR CI.

## GitHub Doctrine Exec

Profile: `test-pare-src-unit-fossil-residue`

Trigger: `/seal-proof profile=test-pare-src-unit-fossil-residue`

## Graduation routing

- CI verdict: pending GitHub proof
- Risk class: src unit fossil deletion under DA-approved temporary src scope rows
- Falsification check: every processed row has terminal disposition; only admission/usecase-superseded src unit tests removed; inventory/boundary/drift, doctrine_scan, gen_digest, profile lint, and targeted `cargo check` pass
- Recommended posture: deep — first protected-crate `src/**` deletion wave after edit-scope gate extension

## Known gaps / next

Follow-on waves per boundary ledger for remaining non-src residue and GPU-residue classifications.
