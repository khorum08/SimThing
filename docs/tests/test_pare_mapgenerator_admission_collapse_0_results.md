# TEST-PARE-MAPGENERATOR-ADMISSION-COLLAPSE-0 Results

## Status

**DONE — merged #1096** (merge commit `5dc4c0d499ac7e87c60e02f1385f3161b4de2451`). Live inventory after merge: **5675**.

## Mission

Collapse the remaining `simthing-mapgenerator` Tier 2 admission fossil tests after `TEST-EDIT-SCOPE-GATE-0` made the mapgenerator test-edit surface data-driven and authorized.

## Boundary Families Processed

- `B-T2-SIMTHING_MAPGENERATOR-ADMISSION_HARD_ERROR`
- `B-T2-SIMTHING_MAPGENERATOR-DUPLICATE_ID_ADMISSION`
- `B-T2-SIMTHING_MAPGENERATOR-FINITE_NUMBER_ADMISSION`
- `B-T2-SIMTHING_MAPGENERATOR-MISSING_OR_UNKNOWN_REFERENCE_ADMISSION`
- `B-T2-SIMTHING_MAPGENERATOR-PARSER_SPAN_ADMISSION`
- `B-T2-SIMTHING_MAPGENERATOR-TOPOLOGY_ADMISSION`

## Disposition

Review table: `docs/tests/test_pare_mapgenerator_admission_collapse_0_review.tsv`

- rows considered: 37
- rows deleted: 31
- representatives kept: 6
- rows blocked: 0

Representatives kept:

- `crates/simthing-mapgenerator/tests/editor_prep.rs::shape_param_rejects_inf`
- `crates/simthing-mapgenerator/tests/special_routes.rs::special_routes_reject_duplicate_pairs`
- `crates/simthing-mapgenerator/tests/editor_prep.rs::shape_param_rejects_nan`
- `crates/simthing-mapgenerator/tests/params.rs::unknown_shape_param_rejects`
- `crates/simthing-mapgenerator/tests/params.rs::invalid_num_stars_rejects`
- `crates/simthing-mapgenerator/tests/params.rs::invalid_hyperlane_min_max_rejects`

Deleted rows differ only by malformed spelling, fixture source, shape-param variant, duplicate topology variant, or hyperlane/topology enumeration under the same Tier 2 admission boundary.

## Scope Gate

`scripts/ci/test_edit_scope.tsv` authorizes `crates/simthing-mapgenerator/tests/*.rs` for `risk_class=test-deletion-tier2-admission-collapse` under profile `test-pare-mapgenerator-admission-collapse`.

No `src/**`, kernel, sim, GPU, driver, mapeditor, tools, workshop, `Cargo.toml`, workflow, scanner allowlist, or doctrine executor files are changed.

## Targeted Proof

No full-crate, workspace, owner-deep, GPU, Bevy, or desktop proof was used.

Local targeted binaries:

- `cargo test -p simthing-mapgenerator --test editor_prep -- --nocapture` - PASS, 19 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test params -- --nocapture` - PASS, 15 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test special_routes -- --nocapture` - PASS, 11 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test connectivity -- --nocapture` - PASS, 6 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test lattice -- --nocapture` - PASS, 7 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test occupancy -- --nocapture` - PASS, 6 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test partition -- --nocapture` - PASS, 11 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test pr8_shapes -- --nocapture` - PASS, 20 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test scale_envelope -- --nocapture` - PASS, 14 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test strategy -- --nocapture` - PASS, 11 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test topology -- --nocapture` - PASS, 10 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test topology_stead -- --nocapture` - PASS, 8 passed / 0 failed
- `cargo test -p simthing-mapgenerator --test visual_preview -- --nocapture` - PASS, 19 passed / 0 failed

## Required Gates

Local gate proof:

- `bash scripts/ci/test_inventory_check.sh` - PASS; rows 5675, discovered 5675, missing 0, extra 0, inspect none
- `bash scripts/ci/test_pare_boundary_check.sh` - PASS; live inventory rows 5675, historical PARED rows mapped 68
- `bash scripts/ci/test_inventory_drift_check.sh` - PASS; unledgered 0, stale 0, promotion-target rows 24
- `bash scripts/ci/doctrine_scan.sh` - PASS; failures 0, inspect 0
- `bash scripts/ci/gen_digest.sh --check` - PASS

## GitHub Doctrine Exec

Profile added: `test-pare-mapgenerator-admission-collapse`

Live proof:

- profile: `test-pare-mapgenerator-admission-collapse`
- head_sha: `88b793165a503642eb2dde8ec0d1a40e9d8fa12f`
- tested_ref: `refs/pull/1096/merge`
- merge_ref_status: PASS
- run: `28630985641`
- job: `84907631106 / doctrine-exec`
- verdict: `DOCTRINE-EXEC-VERDICT: PASS failures=0 inspect=0`
- proof commands: exact edited `simthing-mapgenerator --test ...` binaries only
- no `doctrine_surface_truth.sh`
- no `apt-get`
- no full-crate/workspace/owner-deep/GPU/Bevy/desktop proof

## Graduation Routing

Graduation routing (for DA/orchestrator - why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE | INSPECT(n) | FAIL
  Triage entries:      none
  Risk class:          Tier 2 mapgenerator admission collapse + test deletion + boundary-ledger execution + newly data-driven edit-scope gate
  Falsification check: every processed mapgenerator Tier 2 row has terminal disposition; exactly one representative per processed boundary remains; deleted rows are enumeration/fixture/vocabulary/shape variants under the same boundary; inventory/boundary/drift, doctrine_scan, gen_digest, and targeted Doctrine Exec pass; no full-crate/workspace/owner-deep/GPU/Bevy proof was used.
  Recommended posture: deep - first deletion wave after edit-scope gate conversion.

## Known Gaps / Next

Graduated with merge #1096. Follow-on CPU-safe residue sweep: `TEST-PARE-CPU-SAFE-BOUNDARY-SWEEP-0`.
