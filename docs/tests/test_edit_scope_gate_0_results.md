# TEST-EDIT-SCOPE-GATE-0 Results

## Status

**PROBATION** - implementation proof in progress for DA/orchestrator review. No test deletion is included in this PR.

## Mission

Implement `DA-RULING: TRACK-D-TEST-EDIT-SCOPE-GATE` Option A by replacing hardcoded Track D crate-test edit exceptions in `scripts/ci/test_inventory_check.sh` with an auditable data table.

## Scope Table

Table: `scripts/ci/test_edit_scope.tsv`

Schema:

- `scope_id`
- `path_glob`
- `authorized_risk_class`
- `authorized_profile`
- `rationale`
- `retirement_condition`

Seeded scopes:

- `track-d-clausething-tests` -> `crates/simthing-clausething/tests/*.rs`
- `track-d-spec-tests` -> `crates/simthing-spec/tests/*.rs`
- `track-d-mapgenerator-tests` -> `crates/simthing-mapgenerator/tests/*.rs`

## Guard Behavior

`scripts/ci/test_edit_scope_check.sh` validates the table and proves that:

- authorized mapgenerator test edits pass;
- authorized historical clausething/spec surfaces pass;
- unauthorized driver test edits fail;
- `src/**` fails;
- kernel/sim test edits fail;
- GPU/Bevy-family test edits fail.

`scripts/ci/test_inventory_check.sh` delegates crate `src`/`tests`/`benches` path authorization to the data-driven checker and prints matching scope rows for authorized crate test edits.

## #1094 Closeout

`TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-0` is marked **DONE - merged #1094**.

- merge commit: `68f6b87de52964580fa6b11020e7d9ce64d13077`
- live inventory after merge: 5706

## Validation

Local proof from `test-edit-scope-gate-0`:

- `bash scripts/ci/test_edit_scope_check.sh --prove` - PASS
- `bash scripts/ci/test_inventory_check.sh` - PASS; rows 5706, discovered 5706, missing 0, extra 0, inspect none
- `bash scripts/ci/test_pare_boundary_check.sh` - PASS; live inventory rows 5706, missing owning boundary 0
- `bash scripts/ci/test_inventory_drift_check.sh` - PASS; unledgered 0, stale 0
- `bash scripts/ci/doctrine_scan.sh` - PASS; failures 0, inspect 0
- `bash scripts/ci/gen_digest.sh --check` - PASS

No cargo tests are required or run for this gate PR.

## Scope Ledger

Allowed surfaces only:

- `scripts/ci/test_inventory_check.sh`
- `scripts/ci/test_edit_scope.tsv`
- `scripts/ci/test_edit_scope_check.sh`
- Track D docs and evidence files

No `crates/**`, `Cargo.toml`, `.github/**`, scanner allowlist, inventory, audit, boundary-row, or workflow files are changed.

## Known Gaps

Mapgenerator Tier 2 admission deletion is intentionally deferred to `TEST-PARE-MAPGENERATOR-ADMISSION-COLLAPSE-0` after this gate lands.
