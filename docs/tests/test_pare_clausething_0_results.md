# TEST-PARE-CLAUSETHING-0 Results

## Status

**PROBATION** - first per-crate Track D paring rung complete; four same-family clausething admission tests deleted after source review.

Prerequisite satisfied: PR #1085 (`TEST-PARE-AUDIT-1`) merged to `master` at `54c498633756f1e801ceba0926b62b1db3eca2c7` before this branch was opened.

## What Changed

- Deleted four `simthing-clausething` integration tests that had same-family representatives kept.
- Removed the four deleted rows from `scripts/ci/test_inventory.tsv`, reducing the live inventory from 6,300 to 6,296 rows.
- Updated the 49 D1 Wave 2 COLLAPSE-plan rows in `scripts/ci/test_pare_audit.tsv` to final dispositions: `PARED`, `COLLAPSED-REPRESENTATIVE`, or `BLOCKED`.
- Extended `scripts/ci/test_inventory_check.sh` so historical `PARED` audit rows are valid while live inventory coverage remains exact.

## Source Review Method

Each D1 Wave 2 row was reviewed against the source body of the proposed test and representative. Rows were deleted only when the representative kept the same rejection family or a stronger admission boundary. When D1 grouped distinct rejection families only because they shared a file/parser surface, the row was retained and marked `BLOCKED`.

## Rows Considered

D1 Wave 2 COLLAPSE-plan rows considered: **49**.

Final dispositions:

- `PARED`: 4
- `COLLAPSED-REPRESENTATIVE`: 4
- `BLOCKED`: 41

D1 Wave 2 rows that were already `AUDIT-BLOCKED`: 8, unchanged.

## Rows Deleted

- `ct_scenario_container.rs::scenario_commitment_non_finite_weight_is_rejected`
  - Representative kept: `ct_scenario_container.rs::scenario_commitment_non_finite_threshold_is_rejected`
  - Rejection family: commitment non-finite numeric admission.
  - Boundary: finite-number admission hard-error remains covered by the threshold representative.
- `ct_scenario_container.rs::scenario_palma_feedstock_missing_d_output_col_is_rejected`
  - Representative kept: `ct_scenario_container.rs::scenario_palma_feedstock_missing_w_output_col_is_rejected`
  - Rejection family: `palma_feedstock` missing required output-column admission.
  - Boundary: required-output-column hard-error remains covered by the W output-column representative.
- `mapgenerator_cli_pr6_generated_hyperlanes_lower.rs::generated_hyperlane_scenario_rejects_unknown_endpoint_without_widening`
  - Representative kept: `mapgen_links.rs::unknown_endpoint_is_rejected`
  - Rejection family: hyperlane unknown endpoint.
  - Boundary: generic mapgen topology admission hard-error is the stronger boundary.
- `mapgenerator_cli_pr6_generated_hyperlanes_lower.rs::generated_hyperlane_scenario_rejects_self_link_without_widening`
  - Representative kept: `mapgen_links.rs::self_link_is_rejected`
  - Rejection family: hyperlane self-link.
  - Boundary: generic mapgen topology admission hard-error is the stronger boundary.

## Rows Retained Despite D1 COLLAPSE Plan

Rows blocked after source review: **41**.

Reason: D1's same-file/same-parser grouping was too coarse. The retained rows cover distinct rejection families such as missing vs invalid field-operator parameters, recursive inline-script rejection, economic-key ambiguity vs rejected key forms, owner duplicate ids vs unsupported owner fields, payload unsupported fields vs invalid counts, topology cap families, and scenario-container field/source/vocabulary/cardinality checks.

## Representatives Kept

- `ct_scenario_container.rs::scenario_commitment_non_finite_threshold_is_rejected`
- `ct_scenario_container.rs::scenario_palma_feedstock_missing_w_output_col_is_rejected`
- `mapgen_links.rs::unknown_endpoint_is_rejected`
- `mapgen_links.rs::self_link_is_rejected`

## ct_scenario_container Partition

D1 proposed `28 -> 1`; source review rejected that broad collapse.

Partition result:

- Deleted: 2
- Representatives kept: 2
- Blocked as distinct families: 24

Families retained include child-kind rejection, duplicate location ids, link topology, non-N4/nested links, commitment missing threshold, commitment bad/unknown source/overlay/column, field-operator missing/CFL/non-finite/unknown-output/forbidden vocabulary, `palma_feedstock` missing/unknown/invalid source or column, route/movement vocabulary, and per-block cardinality checks.

## Targeted Tests Run

Only edited integration test binaries were run:

```bash
cargo test -p simthing-clausething --test ct_scenario_container -- --nocapture
cargo test -p simthing-clausething --test mapgenerator_cli_pr6_generated_hyperlanes_lower -- --nocapture
```

Result:

- `ct_scenario_container`: PASS, 43 passed / 0 failed.
- `mapgenerator_cli_pr6_generated_hyperlanes_lower`: PASS, 6 passed / 0 failed.

## Inventory / Check Results

```text
TEST-INVENTORY-CHECK REPORT
  rows: 6296
  discovered: 6296
  missing: 0
  extra: 0
  inspect: none
TEST-PARE-AUDIT REPORT
  audit rows: 1226
  candidate rows: 1222
  missing audit rows: 0
  extra audit rows: 0
TEST-INVENTORY-CHECK-VERDICT: PASS
```

## Doctrine Scan / Digest

Commands:

```bash
bash scripts/ci/doctrine_scan.sh
bash scripts/ci/gen_digest.sh --check
```

Result:

- `doctrine_scan`: PASS, failures=0 inspect=0.
- `gen_digest --check`: PASS.

## Scope Ledger

- No compile_fail, trybuild, seal-proof, oracle-parity, golden-byte, invariant-required, or STEAD-required tests removed.
- No crate source files edited.
- No kernel/sim/gpu/driver tests edited.
- No workflow, scanner, allowlist, runtime, or GPU edits.
- No all-crates cargo test and no bare full-crate cargo test run.
- Owner-deep full batteries remain quarantined artillery.
- Smoke PASS remains mechanics-only, not seal-proof.

## Graduation Routing

Recommended status: **PROBATION**.

Why: this is the first actual deletion/collapse rung. DA should verify every deletion has a same-family representative, `ct_scenario_container.rs` was partitioned rather than mechanically collapsed, never-pare classes remain untouched, and the edited test binaries plus inventory/check/doctrine/digest validation pass.

## Known Gaps / Next

- DA review of this first deletion rung.
- If accepted, continue Track D with the next narrowly scoped per-crate collapse, likely remaining clausething blocked families only after stronger source-level grouping or `TEST-PARE-SPEC-0` for Wave 3.
