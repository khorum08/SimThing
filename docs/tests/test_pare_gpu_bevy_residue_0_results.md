# TEST-PARE-GPU-BEVY-RESIDUE-0 Results

## Status

**PROBATION** — GitHub CPU-only Doctrine Exec proof pending; local GPU/Bevy/desktop compile proof recorded below.

## Ruling applied

`DA-RULING: TRACK-D-OWNER-DEEP-RESIDUE-PARE` — Wave 3: delete Class 2 GPU/Bevy/desktop integration capability-proof relic tests with live boundary representatives.

## Predecessor closeout (#1099)

- **#1099 `TEST-PARE-SRC-UNIT-FOSSIL-RESIDUE-0`**: **DONE** — merged; inventory was **5583** before this wave.

## Mission

Delete 251 Class 2 integration-test fossil rows (admission-adjacent, hygiene-theater, usecase-superseded) across driver/sim/gpu/mapeditor/tools/workshop/clausething. Keep 23 Class 1 enforcement rows. No never-pare, oracle-parity, seal-proof, golden-byte, STEAD, or active-track proof rows touched.

## Disposition

Review table: `docs/tests/test_pare_gpu_bevy_residue_0_review.tsv`

| Metric | Count |
|---|---:|
| rows considered | 274 |
| rows deleted | 251 |
| Class 1 kept | 23 |
| representatives kept | per boundary ledger |

By crate: simthing-driver 191, simthing-gpu 8, simthing-mapeditor 22, simthing-sim 21, simthing-tools 5, simthing-workshop 4.

By class: admission-adjacent 244, hygiene-theater 6, usecase-superseded 1.

By disposition: COLLAPSE_TO_REPRESENTATIVE 229, DELETE 22.

Inventory: 5583 → **5332** (−251).

## Scope gate

Temporary `tests/**` scope rows added per touched protected crate under profile `test-pare-gpu-bevy-residue`.

## Execution split (owner-ratified §3B)

**GitHub Actions (CPU-only):** inventory/boundary/edit-scope gates plus `cargo check -p` for non-Bevy touched crates and representative **CPU** integration binaries only. No `mapeditor_linux_cargo_check.sh`, no `simthing-mapeditor` compile, no `simthing-tools` Bevy tests, no GPU-session execution, no desktop Studio legs on GHA.

**Local (GPU/Bevy/desktop):** surviving mapeditor/tools/gpu integration binaries compiled locally (`cargo test --no-run` / owner machine). GHA absence of these legs is **expected**, not a failure.

## Targeted proof

### GitHub (profile `test-pare-gpu-bevy-residue`)

- `cargo check -p simthing-driver`
- `cargo check -p simthing-sim`
- `cargo check -p simthing-gpu`
- `cargo check -p simthing-workshop`
- `cargo check -p simthing-clausething`
- `cargo test -p simthing-clausething --test ct_1a_entity -- --nocapture`
- `cargo test -p simthing-clausething --test ct_0c_expansion -- --nocapture`
- `cargo test -p simthing-clausething --test ct_2a_intrinsic_flow -- --nocapture clause_hydrated_game_mode_matches_ron_baseline`
- `cargo test -p simthing-clausething --test ct_2c_category_economy -- --nocapture category_hydrated_game_mode_matches_ron_baseline`
- `cargo test -p simthing-driver --test atlas_0080_0 -- --nocapture`
- `cargo test -p simthing-driver --test mapping_plan_compile -- --nocapture`
- `cargo test -p simthing-sim --test mapping_plan_tick -- --nocapture`
- `cargo test -p simthing-workshop --test overlay_order_semantics -- --nocapture`

### Local GPU/Bevy/desktop compile proof

- `cargo check -p simthing-mapeditor`
- `cargo check -p simthing-tools`
- `cargo test -p simthing-mapeditor --test studio_ingestion_admission_report --no-run`
- `cargo test -p simthing-tools --test typeface_lr4 --no-run`
- `cargo test -p simthing-gpu --test bh1_choke_readout --no-run`

## Required gates

Local gate proof:

- `scripts/ci/test_edit_scope_check.sh --prove` — PASS
- `scripts/ci/test_inventory_check.sh` — PASS; rows 5332
- `scripts/ci/test_pare_boundary_check.sh` — PASS
- `scripts/ci/test_inventory_drift_check.sh` — PASS
- `scripts/ci/doctrine_scan.sh` — PASS; failures 0, inspect 0
- `scripts/ci/gen_digest.sh --check` — PASS
- `scripts/ci/doctrine_exec_profile_lint.sh` — PASS

Local GPU/Bevy/desktop compile proof:

- `cargo check -p simthing-mapeditor` — PASS
- `cargo check -p simthing-tools` — PASS
- `cargo test -p simthing-mapeditor --test studio_ingestion_admission_report --no-run` — PASS
- `cargo test -p simthing-tools --test typeface_lr4 --no-run` — PASS
- `cargo test -p simthing-gpu --test bh1_choke_readout --no-run` — PASS

## GitHub Doctrine Exec

Profile: `test-pare-gpu-bevy-residue` (CPU-only on GHA)

Trigger: `/seal-proof profile=test-pare-gpu-bevy-residue`

## Graduation routing

- CI verdict: pending GitHub CPU-only proof
- Risk class: integration fossil deletion under DA-approved temporary tests scope; GPU/Bevy/desktop legs local-only
- Falsification check: 251 deleted rows are Class 2 collapse targets with named representatives; Class 1 rows retained; no product code edited outside `#[test]` surfaces
- Recommended posture: deep — largest inventory drop in Track D owner-deep residue wave

## Known gaps / next

Track D residue cadence (`TEST-PARE-CADENCE-DF`) after material reduction lands.
