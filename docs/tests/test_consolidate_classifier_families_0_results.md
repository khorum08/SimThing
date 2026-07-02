# TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0 Results

## Status

**PROBATION** — implementation complete; awaiting GitHub-side Doctrine Exec proof and DA/orchestrator review. Not self-cleared for merge.

## Fable/DA consolidation exit applied

Applied Fable's consolidation-exit rule across all live `TIER4_CLASSIFIER_CONSOLIDATION` rows with `CONSOLIDATE_TO_TABLE`: independent hygiene-theater classifier-input tests collapsed into one metadata table per crate family while preserving distinct input case labels. No never-pare or active TP rows were touched. No crate `src/**` edits.

## Boundary families processed

| consolidation_target | initial rows | integration consolidated | source-level blocked |
|---|---:|---:|---:|
| `table::simthing-clausething::hygiene_theater_cases` | 1 | 1 | 0 |
| `table::simthing-driver::hygiene_theater_cases` | 93 | 91 | 2 |
| `table::simthing-gpu::hygiene_theater_cases` | 3 | 3 | 0 |
| `table::simthing-mapeditor::hygiene_theater_cases` | 18 | 18 | 0 |
| `table::simthing-sim::hygiene_theater_cases` | 2 | 2 | 0 |
| `table::simthing-tools::hygiene_theater_cases` | 8 | 8 | 0 |
| `table::simthing-workshop::hygiene_theater_cases` | 9 | 9 | 0 |

All rows were `B-T4-HYGIENE-THEATER-CONSOLIDATION`. No other `CONSOLIDATE_TO_TABLE` families remain in the live ledger (`Tier 4 CONSOLIDATE candidates: 0` after merge).

## Rows considered

Source review table: [`test_consolidate_classifier_families_0_review.tsv`](test_consolidate_classifier_families_0_review.tsv)

- Ledger `CONSOLIDATE_TO_TABLE` rows: 134
- Terminal review rows (including 7 new table representatives): 141
- Initial disposition: 134 `CONSOLIDATE_TO_TABLE`

## Independent tests removed

- Integration tests retired from live inventory: **132**
- Full test files deleted: 3 (`resource_flow_flat_star_continued_soak.rs`, `c1_threshold_perf.rs`, `c2_intent_perf.rs`)
- Mixed files stripped (#[test] functions removed, non-hygiene tests retained): 32
- Audit ledger updated: 132 rows marked `PARED` under wave `TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0`

## Table-driven tests added

| crate | file | cases preserved |
|---|---|---:|
| simthing-clausething | `test_consolidate_classifier_families_0_hygiene_consolidation.rs` | 1 |
| simthing-driver | `test_consolidate_classifier_families_0_hygiene_consolidation.rs` | 91 |
| simthing-gpu | `test_consolidate_classifier_families_0_hygiene_consolidation.rs` | 3 |
| simthing-mapeditor | `test_consolidate_classifier_families_0_hygiene_consolidation.rs` | 18 |
| simthing-sim | `test_consolidate_classifier_families_0_hygiene_consolidation.rs` | 2 |
| simthing-tools | `test_consolidate_classifier_families_0_hygiene_consolidation.rs` | 8 |
| simthing-workshop | `test_consolidate_classifier_families_0_hygiene_consolidation.rs` | 9 |

Each table test: `hygiene_theater_cases_table_preserves_inputs` — metadata preservation with readable per-case failure labels (same pattern as `TEST-PARE-SPEC-0`).

## Input cases preserved

- **132** distinct classifier-input cases preserved as table rows across 7 consolidation targets
- **2** source-level driver rows (`intent_stress_queues_tick_patches`, `stress_builtins_load_at_small_scale`) recorded as `KEPT_PROMOTION_REQUIRED`; boundary ledger rekeyed to `PROMOTION_REQUIRED` / `source-level-rung-required`

## Rows retained and why

- **7** new table representatives (`CONSOLIDATED_TEST`) — one per crate family
- **2** driver `src/scenario.rs` unit tests — `KEPT_PROMOTION_REQUIRED` (crate source edits forbidden)
- **81** active TP live-rung rows — untouched (active-track protection)
- **904** never-pare rows — untouched

## Never-pare / active-rung protection

- Actionable never-pare rows in consolidate set: 0
- Actionable active TP rows in consolidate set: 0
- Ledger defect check: no never-pare row was assigned `CONSOLIDATE_TO_TABLE`

## Inventory / boundary / drift checks

Local (Git Bash):

- `bash scripts/ci/test_inventory_check.sh`: **PASS** (5745 rows; discovered=5745; audit reconciled)
- `bash scripts/ci/test_pare_boundary_check.sh`: **PASS** (Tier 4 CONSOLIDATE candidates: 0)
- `bash scripts/ci/test_inventory_drift_check.sh`: **PASS** (promotion-target rows: 15)
- Live inventory after wave: **5745 rows** (was 5870 after #1091; net −125)

## Targeted local tests

Exact edited binaries only (`cargo test -p <crate> --test test_consolidate_classifier_families_0_hygiene_consolidation -- --nocapture`):

| crate | result |
|---|---|
| simthing-clausething | PASS |
| simthing-driver | PASS |
| simthing-gpu | PASS |
| simthing-mapeditor | PASS |
| simthing-sim | PASS |
| simthing-tools | PASS |
| simthing-workshop | PASS |

No `cargo test -p <crate>`, no `cargo test --workspace`, no owner-deep battery.

## GitHub-side Doctrine Exec proof

Pending after push:

```text
/seal-proof profile=test-consolidate-classifier-families
```

Required acceptance:

- `profile: test-consolidate-classifier-families`
- `tested_ref: refs/pull/<PR>/merge`
- `merge_ref_status: PASS`
- `DOCTRINE-EXEC-VERDICT: PASS failures=0 inspect=0`
- no `doctrine_surface_truth.sh` command (`risk_class=test-deletion-classifier-consolidation`)

## Doctrine scan / digest

- `bash scripts/ci/doctrine_scan.sh`: **PASS** (`failures=0 inspect=0`; TEST-INVENTORY-DRIFT stock gate PASS)
- `bash scripts/ci/gen_digest.sh --check`: **PASS**

## No-full-battery proof

- Doctrine Exec profile `test-consolidate-classifier-families` lists exactly seven `cargo test -p <crate> --test test_consolidate_classifier_families_0_hygiene_consolidation -- --nocapture` commands
- `risk_class=test-deletion-classifier-consolidation` — surface-truth skipped per generalized `test-deletion-*` rule from #1091 0R2
- No full-crate or workspace cargo test invoked locally

## Scope Ledger

**Allowed edits:**

- `crates/*/tests/**` — 7 new table files; 32 mixed files stripped; 3 soak files deleted
- `scripts/ci/test_inventory.tsv`, `test_pare_boundary_rows.tsv`, `test_pare_audit.tsv`, `doctrine_exec_profiles.tsv`
- `docs/tests/**`, `docs/design_0_0_8_4_6_ci_scaffolding.md`

**Not edited:** crate `src/**`, workflows, scanner allowlists, kernel/sim never-pare suites

## Graduation routing

```text
Graduation routing (for DA/orchestrator — why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE (local stock gates + targeted tests); GitHub proof pending
  Triage entries:      none (this PR)
  Risk class:          classifier consolidation + test deletion + boundary-ledger execution + owner-edict full-battery reduction
  Falsification check: Verify every processed TIER4 classifier row has terminal disposition; verify 132 distinct classifier-path inputs preserved in table rows; verify independent per-input tests removed from live inventory; verify no never-pare or active-track row touched; verify inventory/boundary/drift checks, doctrine_scan, gen_digest, and targeted Doctrine Exec pass; verify no full-crate cargo test run.
  Recommended posture: deep — this exercises Fable's consolidation-exit doctrine directly across seven crates.
```

## Known gaps / next

- **CT-2c longest-match modifier-key decoder** (`ct_2c_category_economy.rs`): ledger rows are `KEEP` / `COLLAPSE_TO_REPRESENTATIVE` / `NEVER_PARE`, not `CONSOLIDATE_TO_TABLE`; `rejected_key_forms_hard_error_with_spans` already uses an inline case table. No CT-2c consolidation in this wave — correct per live ledger authority.
- **2** driver source-level hygiene soak rows remain `PROMOTION_REQUIRED` until a source-level rung lands.
- **simthing-sim** table representative is provisional `AUDIT` under kernel/sim strict tier (not `promotion-target:`); six other crate tables carry promotion targets.
- Next Track D wave: Tier 2 collapse candidates (378 rows) or promotion rungs for the 15 `promotion-target:` representatives.