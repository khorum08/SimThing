# TEST-PARE-SPEC-0 Results

## Status

**PROBATION** - implementation proof in progress for DA/orchestrator review. This PR is not self-cleared for merge.

## Fable/DA Standard Applied

Applied the standing `ADMISSION-BOUNDARY-COLLAPSE` regime from Track D graduation. Retention bears the burden: admission enumeration collapses to one representative per owned boundary, type/seal and doctrine-scan duplicates delete, and classifier-input families consolidate to tables instead of surviving as independent soak tests.

## What Changed

- Processed every `simthing-spec` row that entered the wave with `DELETE`, `COLLAPSE_TO_REPRESENTATIVE`, or `CONSOLIDATE_TO_TABLE`.
- Removed 425 runtime integration-test rows from the live corpus.
- Consolidated seven 34k hygiene-theater soak inputs into `hygiene_theater_cases_table_preserves_inputs`.
- Retained owned Tier 2 representatives with legal `promotion-target:` state.
- Added `scripts/ci/test_residue_classes.tsv` and wired the touched Track D stock checkers to read the shared permanent-residue token set.
- Added targeted Doctrine Exec profile `test-pare-spec`, listing exact edited `simthing-spec` integration test binaries only.

## Boundary Families Processed

| boundary | initial rows |
|---|---:|
| `B-T1-AS-NEWTYPES-SEALS` | 4 |
| `B-T2-SIMTHING_SPEC-ADMISSION_HARD_ERROR` | 192 |
| `B-T2-SIMTHING_SPEC-DUPLICATE_ID_ADMISSION` | 18 |
| `B-T2-SIMTHING_SPEC-FIELD_PAYLOAD_ADMISSION` | 60 |
| `B-T2-SIMTHING_SPEC-FINITE_NUMBER_ADMISSION` | 6 |
| `B-T2-SIMTHING_SPEC-MISSING_OR_UNKNOWN_REFERENCE_ADMISSION` | 41 |
| `B-T2-SIMTHING_SPEC-PARSER_SPAN_ADMISSION` | 49 |
| `B-T2-SIMTHING_SPEC-TOPOLOGY_ADMISSION` | 35 |
| `B-T3-FORGE-UNSAFE-SCAN` | 1 |
| `B-T3-SEMANTIC-FREE-SCAN` | 17 |
| `B-T3-SURFACE-SCAN` | 6 |
| `B-T4-HYGIENE-THEATER-CONSOLIDATION` | 7 |
| `B-USECASE-SUPERSEDED-LEGACY-DEFAULT` | 15 |

## Rows Considered

Source review table: [`test_pare_spec_0_review.tsv`](test_pare_spec_0_review.tsv)

- Rows considered from the live boundary ledger: 451
- Terminal review rows including the new consolidated representative: 452
- Initial dispositions: 401 collapse, 43 delete, 7 consolidate

## Rows Deleted

- `DELETED`: 425
- Deleted rows were removed from `scripts/ci/test_inventory.tsv` and `scripts/ci/test_pare_boundary_rows.tsv`.
- Type/seal and scan-owned deletes cite their owning boundary in the review table.

## Rows Collapsed

- Tier 2 admission enumeration rows were collapsed to owned representatives.
- `COLLAPSED_REPRESENTATIVE`: 1 row in the original actionable set, the field-payload integration representative.
- Existing representative rows outside the actionable set were retained and moved from vague `AUDIT` to legal `KEEP` plus `promotion-target:`.

## Rows Consolidated

- `CONSOLIDATED_INPUT`: 7
- `CONSOLIDATED_TEST`: 1
- New table-driven test: `crates/simthing-spec/tests/test_pare_spec_0_hygiene_consolidation.rs::hygiene_theater_cases_table_preserves_inputs`

## Representatives Kept

- `jit_kernel_graph_admission::jit_desc2_rejects_cycles`
- `scenario_ingestion_admission::rejects_duplicate_owner_ids`
- `eml_gadget_tier2_acceleration::rejects_non_finite_dt`
- `scenario_ingestion_admission::rejects_missing_owner`
- `eml_gadget_tier1::invalid_params_reject`
- `planet_child_location_admission::duplicate_planet_id_rejected`
- `bh2s_stress_compose_admission::bh2s_admission_rejects_input_field_budget_exceeded`
- Existing source-unit field-payload representative `region_field_budget::over_budget_rejects` remains live because crate source edits are out of scope.

## Never-Pare / Active-Rung Protection

- Actionable never-pare rows found: 0
- Actionable active TP live-rung rows found: 0
- Never-pare and active-track rows were not deleted, collapsed, or promotion-targeted by this wave.

## Promotion-Required Rows Left Untouched

- 18 source-level rows in `crates/simthing-spec/src/**` were not edited because crate source edits are explicitly out of scope.
- Those rows are terminally recorded in the review table as `BLOCKED_LEDGER_DEFECT` and rekeyed in the live boundary ledger as `PROMOTION_REQUIRED` with `source-level-rung-required`.

## Targeted Tests Run

Local targeted Doctrine Exec profile:

- profile: `test-pare-spec`
- runner: Git Bash (`C:\Program Files\Git\bin\bash.exe`)
- command: `bash scripts/ci/doctrine_exec.sh` with `DOCTRINE_EXEC_PROFILE=test-pare-spec`
- command shape: exact `cargo test -p simthing-spec --test <binary> -- --nocapture` commands only
- result: `DOCTRINE-EXEC-VERDICT: INSPECT failures=0 inspect=0 profile=test-pare-spec profile_class=targeted owner_deep=false head_sha=unknown`
- note: local profile reports `INSPECT` because owner-deep/head metadata is unavailable locally; the failure and inspect ledgers are both empty.

Additional targeted repair slice:

- previously failing exact binaries rerun after fixture/API drift repair: PASS
- remaining owner-silo/surface-scope exact slice: PASS (`recursive_local_rf`, `recursive_rf_reconciliation`, `runtime_participant_property_mutation_boundary`, `runtime_participant_state_mutation`, `runtime_rf_tick_source`)

## GitHub-Side Doctrine Exec Proof

Live targeted Doctrine Exec proof completed on the PR merge ref.

- profile: test-pare-spec
- head_sha: 560bacee19627171774f14ae491d0a6359c6ed6a
- tested_ref: refs/pull/1091/merge
- merge_ref_status: PASS
- run: 28614694073
- job: 84855509020 / doctrine-exec
- verdict: DOCTRINE-EXEC-VERDICT: PASS failures=0 inspect=0

## Inventory / Boundary / Drift Checks

- `bash scripts/ci/test_inventory_check.sh`: PASS (`rows=5870`, `discovered=5870`, `missing=0`, `extra=0`, `inspect=none`)
- `bash scripts/ci/test_pare_boundary_check.sh`: PASS (`boundary rows=76`, `live inventory rows=5870`, `historical PARED rows mapped=4`)
- `bash scripts/ci/test_inventory_drift_check.sh`: PASS (`unledgered=0`, `stale=0`, `promotion-target rows=9`)

## Doctrine Scan / Digest

- `bash scripts/ci/doctrine_scan.sh`: PASS (`DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0 selftest=SKIPPED`)
- `bash scripts/ci/gen_digest.sh --check`: PASS

## No-Full-Battery Proof

No bare `cargo test -p simthing-spec`, `cargo test -p <crate>`, `cargo test --workspace`, owner-deep, or all-crates battery is part of this proof. The new `test-pare-spec` profile lists exact `cargo test -p simthing-spec --test <binary> -- --nocapture` commands.

## Scope Ledger

Touched allowed Track D surfaces: `crates/simthing-spec/tests/**`, Track D ledgers, the targeted Doctrine Exec profile table, Track D docs, and the checker token-set consolidation required by the handoff after touching a stock checker.

No runtime crate source, kernel/sim/gpu/driver tests, workflows, scanner allowlists, or owner-deep batteries were touched.

## Graduation Routing

CI verdict: PASS-RELIABLE for PR Doctrine Scan, default smoke, and targeted test-pare-spec GitHub proof on head 560bacee19627171774f14ae491d0a6359c6ed6a.

Triage entries: none locally (`failures=0 inspect=0`).

Risk class: test deletion + boundary-ledger execution + owner-edict full-battery reduction

Falsification check: verify all considered rows have terminal review dispositions; no never-pare or active TP row was removed; type/seal, admission, scan, classifier, and usecase-owned rows follow their owning boundary; inventory/boundary/drift checks, doctrine scan, digest, local exact binaries, and GitHub exact-binary Doctrine Exec pass; no full-crate or all-crates cargo test was run.

Recommended posture: deep.

## Known Gaps / Next

The 18 source-level rows require a follow-up source-authorized rung or boundary correction. This PR keeps them live and promotion-required rather than violating the `src/**` edit ban.
