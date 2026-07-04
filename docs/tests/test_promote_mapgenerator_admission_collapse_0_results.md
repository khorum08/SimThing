# TEST-PROMOTE-MAPGENERATOR-ADMISSION-COLLAPSE-0 Results

## Status

**PROBATION / DA REVIEW**. Merge not authorized for Grok.

## Mission

Promote six `simthing-mapgenerator` admission promotion targets into stronger lower-boundary proof and retire redundant integration representatives in the same PR. This rung resolves only the six simthing-mapgenerator promotion targets. It does not process mapeditor, tools/typeface, spec, clausething, owner-deep, protected-corpus, or workflow surfaces.

## Scope

In scope: `crates/simthing-mapgenerator/**`, six promotion targets, inventory/boundary/audit/promotion ledgers for those rows, design/evidence/results docs.

Out of scope: mapeditor, tools, spec, clausething, driver, gpu, Admission Substrate, SimThing-Kernel, CI profiles/scanners/allowlists/workflows, `.github/**`.

## Target rows

Six mapgenerator promotion targets from `docs/tests/test_promotion_wave_plan.md` (pre-rung backlog 31 → post-rung 25).

## Promotion map

Review table: `docs/tests/test_promote_mapgenerator_admission_collapse_0_review.tsv`

| Boundary family | Decision |
|---|---|
| admission-hard-error | PROMOTED_AND_RETIRED |
| finite-number-admission | PROMOTED_AND_RETIRED |
| missing-or-unknown-reference-admission | PROMOTED_AND_RETIRED |
| parser-span-admission | PROMOTED_AND_RETIRED |
| topology-admission | PROMOTED_AND_RETIRED |
| duplicate-id-admission | PROMOTED_AND_RETIRED |

All six replaced by `crates/simthing-mapgenerator/tests/admission_boundary.rs` lower-boundary representatives.

## Retired rows

- `editor_prep.rs::shape_param_rejects_inf`
- `editor_prep.rs::shape_param_rejects_nan`
- `params.rs::unknown_shape_param_rejects`
- `params.rs::invalid_num_stars_rejects`
- `params.rs::invalid_hyperlane_min_max_rejects`
- `special_routes.rs::special_routes_reject_duplicate_pairs`

## Kept rows

None of the six old integration representatives kept. Six new promoted representatives in `admission_boundary.rs` are the boundary KEEP rows (no `promotion-target:` residue).

## Proof

Recorded on branch `grok/test-promote-mapgenerator-admission-collapse-0` (base `dbb1cf158c`):

- `bash scripts/ci/doctrine_scan.sh`: PASS `failures=0 inspect=0`
- `bash scripts/ci/gen_digest.sh --check`: PASS
- `bash scripts/ci/test_inventory_check.sh`: PASS (`rows=4070`)
- `bash scripts/ci/test_pare_boundary_check.sh`: PASS
- `bash scripts/ci/test_inventory_drift_check.sh`: PASS (`promotion-target rows=25`)
- `git diff --check origin/master...HEAD`: PASS
- Targeted mapgenerator tests: `admission_boundary` 6/6 PASS; `editor_prep` 17/17; `params` 12/12; `special_routes` 10/10
- Non-Bevy survivor compile floor: PASS (core, kernel, sim, workshop, mapgenerator `--tests`)

Forbidden proof avoided: no workspace tests, owner-deep profiles, tools/mapeditor, workflow_dispatch, desktop/Bevy/GPU.

## Inventory delta

- before: 4070
- after: 4070 (6 retired + 6 promoted replacements)

## Promotion backlog delta

- before: 31 promotion-target rows
- after: 25 promotion-target rows (six mapgenerator targets resolved)

## Graduation routing

- Risk class: first promotion/retirement wave / DA-held
- DA question: Does Opus accept the promoted mapgenerator admission representatives and retired rows?
- Not orchestrator-clearable