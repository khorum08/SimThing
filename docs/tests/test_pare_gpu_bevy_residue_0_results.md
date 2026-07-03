# TEST-PARE-GPU-BEVY-RESIDUE-0 Results

## Status

**DONE — merged #1100**. Merge commit `a71700f3fb`. Live inventory after merge: **5332**.

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

Driver/sim/workshop surviving integration binaries are **not** executed on GHA (cold-compile timeout risk); covered by `cargo check` plus local proof below.

**Remedial closeout (orchestration):** `cargo test -p simthing-driver --test atlas_0080_0` was **removed** from the GitHub profile after run `28636114868` timed out at 300s on cold CI compile (desktop/Bevy transitive deps). Do **not** re-add atlas to GHA targeted proof. **`GHA-PROOF-SEAL-0`** now enforces this mechanically via `doctrine_exec_gha_proof_seal.sh` / `doctrine_exec_profile_lint.sh`.

### Class 1 survivor — `atlas_0080_0` (local / owner-deep only)

The `atlas_0080_0` binary survives Wave 3 with Class 1 rows (golden-byte, oracle-parity, behavior-regression). It is **not** a GitHub targeted-proof command. Proof posture:

| Surface | Command | Where |
|---|---|---|
| GHA targeted profile | *(excluded)* | `cargo check -p simthing-driver` floor only |
| Local owner-deep | `cargo test -p simthing-driver --test atlas_0080_0 -- --nocapture` | owner machine — PASS on Windows head `811c7c0e0a` |

Deleted atlas admission duplicates (e.g. `atlas_0080_0_rejects_other_stop_lines`) are covered by wave review disposition and `e10r_rejects_slot_mismatch` representative; the surviving binary itself is Class 1 retention, not deleted-residue proof.

### Local GPU/Bevy/desktop compile proof

- `cargo check -p simthing-mapeditor` — PASS
- `cargo check -p simthing-tools` — PASS
- `cargo test -p simthing-driver --test atlas_0080_0 -- --nocapture` — PASS
- `cargo test -p simthing-mapeditor --test studio_ingestion_admission_report --no-run` — PASS
- `cargo test -p simthing-tools --test typeface_lr4 --no-run` — PASS
- `cargo test -p simthing-gpu --test bh1_choke_readout --no-run` — PASS

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

Profile: `test-pare-gpu-bevy-residue` (CPU-only on GHA; no Bevy/mapeditor/tools/atlas legs)

Commands: five `cargo check -p` floors + four clausething CPU representative tests only. No workspace test sweep. No `atlas_0080_0`.

Trigger: `/seal-proof profile=test-pare-gpu-bevy-residue`

| Run | Head | Verdict | Notes |
|---|---|---|---|
| `28636990813` | `a30bf54f3e` | **PASS** failures=0 inspect=0 | authoritative master closeout (docs + merged #1100) |
| `28636487452` | `811c7c0e0a` | PASS failures=0 inspect=0 | pre-merge CPU-only profile proof |
| `28636114868` | `6bf8502276` | FAIL | atlas_0080_0 timed out — remediated by profile removal |

Workflow-dispatch runs without a PR report `DOCTRINE-EXEC-VERDICT: INSPECT` when `merge_ref_status=UNAVAILABLE`; executable outcome is still failures=0 inspect=0.

## Graduation routing

- CI verdict: **PASS** — run `28636990813` on master `a30bf54f3e`, failures 0, inspect 0 (remediated profile; atlas excluded from GHA)
- Risk class: integration fossil deletion under DA-approved temporary tests scope; GPU/Bevy/desktop legs local-only
- Falsification check: 251 deleted rows are Class 2 collapse targets with named representatives; Class 1 rows retained; no product code edited outside `#[test]` surfaces
- Recommended posture: deep — largest inventory drop in Track D owner-deep residue wave

## Known gaps / next

Track D residue cadence (`TEST-PARE-CADENCE-DF`) after material reduction lands.
