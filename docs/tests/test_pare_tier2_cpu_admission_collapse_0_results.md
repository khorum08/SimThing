# TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-0 Results

## Status

**PROBATION** - implementation proof in progress for DA/orchestrator review. This PR is not self-cleared for merge.

## Orientation / Predecessor Rungs

This rung follows the completed Track D material waves:

- `TEST-PARE-SPEC-0`: 425 spec fossil rows deleted; 7 hygiene inputs consolidated to 1 table; merged as #1091.
- `TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0`: 132 hygiene-theater rows retired behind one CPU-only `simthing-spec` table; merged as #1092 and closed out by #1093.

## Fable/DA Tier 2 Rule Applied

Applied `DA-RULING: ADMISSION-BOUNDARY-COLLAPSE`: Tier 2 admission hard-error families keep one representative negative test per boundary. Variants that differ only by field name, fixture source, scenario shape, unsupported vocabulary spelling, or malformed-example enumeration collapse to that representative.

## CPU-Safe Scope

Processed the CPU-safe Tier 2 rows admitted by the current checker guard in:

- `simthing-clausething`

`simthing-mapgenerator` rows were reviewed but blocked because the current inventory checker rejects mapgenerator test-file edits on this Track D path.

Excluded from this rung: `simthing-driver`, `simthing-gpu`, `simthing-mapeditor`, `simthing-tools`, `simthing-workshop`, `simthing-kernel`, and `simthing-sim`.

## Boundary Families Processed

| boundary | rows |
|---|---:|
| `B-T2-SIMTHING_CLAUSETHING-ADMISSION_HARD_ERROR` | 6 |
| `B-T2-SIMTHING_CLAUSETHING-DUPLICATE_ID_ADMISSION` | 2 |
| `B-T2-SIMTHING_CLAUSETHING-FIELD_PAYLOAD_ADMISSION` | 2 |
| `B-T2-SIMTHING_CLAUSETHING-FINITE_NUMBER_ADMISSION` | 3 |
| `B-T2-SIMTHING_CLAUSETHING-MISSING_OR_UNKNOWN_REFERENCE_ADMISSION` | 11 |
| `B-T2-SIMTHING_CLAUSETHING-PARSER_SPAN_ADMISSION` | 4 |
| `B-T2-SIMTHING_CLAUSETHING-TOPOLOGY_ADMISSION` | 16 |
| `B-T2-SIMTHING_CLAUSETHING-UNSUPPORTED_VOCABULARY_ADMISSION` | 5 |
| `B-T2-SIMTHING_MAPGENERATOR-ADMISSION_HARD_ERROR` | 14 |
| `B-T2-SIMTHING_MAPGENERATOR-DUPLICATE_ID_ADMISSION` | 5 |
| `B-T2-SIMTHING_MAPGENERATOR-FINITE_NUMBER_ADMISSION` | 1 |
| `B-T2-SIMTHING_MAPGENERATOR-MISSING_OR_UNKNOWN_REFERENCE_ADMISSION` | 4 |
| `B-T2-SIMTHING_MAPGENERATOR-PARSER_SPAN_ADMISSION` | 2 |
| `B-T2-SIMTHING_MAPGENERATOR-TOPOLOGY_ADMISSION` | 11 |

## Rows Considered

Review table: [`test_pare_tier2_cpu_admission_collapse_0_review.tsv`](test_pare_tier2_cpu_admission_collapse_0_review.tsv)

- Boundary mappings considered: 86
- Live inventory rows considered: 82
- Historical PARED rows kept mapped: 4

## Rows Deleted

- Deleted in this PR: 33 live integration tests
- Historical already-deleted rows recorded: 4
- Inventory rows: 5,739 -> 5,706

## Representatives Kept

Eight representative rows were retained and marked `KEEP` with legal `promotion-target:` entries:

- `bh3_authoring_parse::bh3_authoring_rejects_missing_u_sat`
- `ct_0c_expansion::recursive_inline_script_is_rejected`
- `ct_1a_entity::unsupported_entity_field_is_hard_error`
- `bh3_authoring_parse::bh3_authoring_rejects_invalid_chi_literal`
- `ct_scenario_container::duplicate_location_ids_are_rejected`
- `ct_scenario_container::scenario_commitment_non_finite_threshold_is_rejected`
- `ct_scenario_container::scenario_second_field_operator_is_rejected`
- `mapgen_links::self_link_is_rejected`

## Rows Blocked And Why

Forty-one otherwise collapsible rows were blocked by proof boundaries: one `mapgenerator_cli_pr11_scale_envelope` row because its exact binary currently fails an unrelated 1000-star scale proof, three `ct_2a`/`ct_2c` rows because their exact binaries require a real GPU adapter on GitHub, and all 37 reviewed `simthing-mapgenerator` rows because the current inventory checker rejects mapgenerator test-file edits on this Track D path. These candidates remain live rather than broadening, masking, or checker-law editing in this deletion wave.

Rows in driver/gpu/mapeditor/tools/workshop/kernel/sim families were intentionally not considered by this PR. They remain for later proof-boundary or strict-tier rungs.

## Never-Pare / Active-Rung Protection

No never-pare row and no active Terran-Pirate live-rung row was deleted, collapsed, or promotion-targeted by this wave.

## Kernel/Sim Strict-Tier Protection

`simthing-kernel` and `simthing-sim` were not touched. No kernel/sim KEEP row was widened by this PR.

## Bevy/GPU Proof-Boundary Protection

No Bevy, WGPU, desktop, driver, mapeditor, tools, workshop, gpu, kernel, or sim proof binary was added to the targeted profile. The GitHub proof is constrained to exact admitted CPU-side `simthing-clausething` integration binaries.

## Inventory / Boundary / Drift Checks

- `bash scripts/ci/test_inventory_check.sh`: PASS (`rows=5706`, `discovered=5706`, `missing=0`, `extra=0`)
- `bash scripts/ci/test_pare_boundary_check.sh`: PASS (`live inventory rows=5706`, `historical PARED rows mapped=37`)
- `bash scripts/ci/test_inventory_drift_check.sh`: PASS (`unledgered=0`, `stale=0`, `promotion-target rows=18`)

## Targeted Local Tests

- `DOCTRINE_EXEC_PROFILE=test-pare-tier2-cpu-admission-collapse bash scripts/ci/doctrine_exec.sh`: command suite PASS (`failures=0`, `inspect=0`)

The local verdict label is `INSPECT` because owner-deep/head metadata is unavailable locally; failures and inspect ledgers are both empty. The profile contains only exact `cargo test -p <cpu-safe-crate> --test <binary> -- --nocapture` commands.

## GitHub-Side Doctrine Exec Proof

First GitHub dispatch failed on adapter-bound `ct_2a_intrinsic_flow`; the profile was then narrowed to exact admitted `simthing-clausething` CPU-safe binaries only, with mapgenerator rows blocked by the current checker guard.

Live Doctrine Exec after narrowing:

- profile: `test-pare-tier2-cpu-admission-collapse`
- head_sha: `e81f656ee643da7543c9be8acf1ac617f2404bd4`
- tested_ref: `refs/pull/1094/merge`
- merge_ref_status: PASS
- run: 28628411511
- job: 84899910586 / doctrine-exec
- verdict: `DOCTRINE-EXEC-VERDICT: PASS failures=0 inspect=0`
- forbidden proof markers: no `doctrine_surface_truth.sh`, no `apt-get`, no workspace/full-crate cargo command, no driver/gpu/mapeditor/tools/workshop/kernel/sim test binary, no adapter-bound `ct_2a`/`ct_2c` binary, no mapgenerator test binary.

This docs-only evidence update must be followed by a final live Doctrine Exec rerun on the updated PR head before merge consideration.

## Doctrine Scan / Digest

- `bash scripts/ci/doctrine_scan.sh`: PASS (`failures=0`, `inspect=0`, `selftest=SKIPPED`)
- `bash scripts/ci/gen_digest.sh --check`: PASS

## No-Full-Battery Proof

No `cargo test --workspace`, no bare `cargo test -p <crate>`, no owner-deep profile, and no full-crate battery is part of this proof path.

## Scope Ledger

Touched allowed surfaces only:

- CPU-safe `crates/simthing-clausething/tests/**`
- `simthing-mapgenerator` review rows only; mapgenerator test edits were blocked and restored
- Track D ledgers and review table
- Targeted Doctrine Exec profile table
- Track D docs

No crate `src/**`, workflows, scanner allowlists, `doctrine_exec.sh`, `Cargo.toml`, kernel/sim/gpu/driver/mapeditor/tools/workshop tests, or desktop dependency bootstrap was touched.

## Graduation Routing

Graduation routing (for DA/orchestrator - why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE | INSPECT(n) | FAIL
  Triage entries:      this PR's rows in scripts/ci/triage_log.tsv, or "none"
  Risk class:          Tier 2 admission collapse + test deletion + boundary-ledger execution + CPU-only proof-boundary discipline
  Falsification check: Verify every processed Tier 2 row has terminal disposition; verify one representative per processed admission boundary; verify deleted rows differ only by enumeration/fixture/vocabulary/shape under same boundary; verify no never-pare, active TP, kernel/sim strict-tier, source-level, or Bevy/GPU-linked row was improperly deleted; verify inventory/boundary/drift, doctrine_scan, gen_digest, and targeted CPU-only Doctrine Exec pass; verify no full-crate/workspace/owner-deep cargo test was run.
  Recommended posture: deep - this is the first Tier 2 collapse wave after the proof-boundary correction.

## Known Gaps / Next

Future rungs must handle non-CPU-safe Tier 2 families with their own proof-boundary discipline, especially driver/gpu/mapeditor/tools/workshop rows and kernel/sim strict-tier rows.
