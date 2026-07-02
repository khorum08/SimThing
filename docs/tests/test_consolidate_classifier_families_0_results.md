# TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0 Results

## Status

**0R remedial complete** — proof boundary narrowed to CPU-only representative. Awaiting fresh GitHub Doctrine Exec on current head; prior green proof on seven Bevy/GPU-linked binaries is **not accepted**.

## 0R proof boundary correction

The initial #1092 implementation preserved the 132-row reduction correctly but widened GitHub proof by:

- installing ALSA/X11/Wayland/Vulkan packages in `doctrine_exec.sh`
- widening `test-deletion-*` command timeout to 900s
- adding `x11` to `simthing-tools` dev-dependencies
- running seven Bevy/GPU/driver/mapeditor/tools/workshop exact test binaries on GitHub Linux

**0R corrected the proof boundary:** the accepted proof uses a single CPU-only consolidation representative in `simthing-spec` and does not compile Bevy/GPU/driver/mapeditor/tools/workshop test binaries on GitHub.

## Fable/DA consolidation exit applied

Applied Fable's consolidation-exit rule: independent hygiene-theater classifier-input tests collapsed into metadata table rows while preserving distinct case labels. No never-pare or active TP rows touched. No crate `src/**` edits.

## Boundary families processed

All 134 live `CONSOLIDATE_TO_TABLE` rows (`B-T4-HYGIENE-THEATER-CONSOLIDATION`). Consolidation target unified to `table::track-d::hygiene_theater_cases`.

## Rows considered

Source review table: [`test_consolidate_classifier_families_0_review.tsv`](test_consolidate_classifier_families_0_review.tsv)

- Terminal review rows: 135 (132 `CONSOLIDATED_INPUT`, 2 `KEPT_PROMOTION_REQUIRED`, 1 `CONSOLIDATED_TEST`)

## Independent tests removed

- Integration tests retired: **132** (unchanged from initial #1092)
- Live inventory: **5739** (was 5870 after #1091; net −131)

## Table-driven tests added

**0R:** one CPU-side representative replaces seven per-crate binaries:

| file | cases |
|---|---:|
| `crates/simthing-spec/tests/test_consolidate_classifier_families_0_hygiene_consolidation.rs` | 132 |

Test function: `hygiene_theater_cases_table_preserves_inputs` — metadata preservation only (crate, file, test_name, classifier input, wave, disposition).

## Input cases preserved

All **132** consolidated input cases preserved as table rows with readable per-case failure labels.

## Rows retained and why

- **2** driver `src/scenario.rs` unit tests → `KEPT_PROMOTION_REQUIRED`
- **904** never-pare + **81** active TP rows untouched
- **No** `simthing-sim` hygiene-table row (kernel/sim strict-tier correction)

## Never-pare / active-rung protection

Verified: 0 never-pare or active TP rows in consolidate set.

## Reverted doctrine_exec Bevy bootstrap

Removed from `scripts/ci/doctrine_exec.sh`:

- `install_linux_bevy_test_deps_if_needed`
- all `apt-get install` of desktop/audio/GPU packages
- automatic 900s timeout widening for `test-deletion-*`

Kept: `risk_class=test-deletion-*` skips `doctrine_surface_truth.sh` (#1091 rule).

## Reverted simthing-tools x11 feature

Removed proof-only `x11` feature from `simthing-tools` dev-dependencies.

## Inventory / boundary / drift checks

Local:

- `test_inventory_check.sh`: PASS (5739 rows)
- `test_pare_boundary_check.sh`: PASS (Tier 4 CONSOLIDATE candidates: 0)
- `test_inventory_drift_check.sh`: PASS (promotion-target rows: 10)
- `doctrine_scan.sh`: PASS
- `gen_digest.sh --check`: PASS

## Targeted local tests

```bash
cargo test -p simthing-spec --test test_consolidate_classifier_families_0_hygiene_consolidation -- --nocapture
```

Result: PASS

No Bevy/GPU/driver/mapeditor/sim/tools/workshop test binaries run.

## GitHub-side Doctrine Exec proof

Profile: `test-consolidate-classifier-families`

```bash
cargo test -p simthing-spec --test test_consolidate_classifier_families_0_hygiene_consolidation -- --nocapture
```

Accepted 0R proof on `bfcc986c56d725e64a5a8a0bef34bb19c4b992a2`:

- profile: `test-consolidate-classifier-families`
- tested_ref: `refs/pull/1092/merge`
- merge_ref_status: PASS
- run: 28626085144
- verdict: `DOCTRINE-EXEC-VERDICT: PASS failures=0 inspect=0`
- command: `cargo test -p simthing-spec --test test_consolidate_classifier_families_0_hygiene_consolidation -- --nocapture` only
- no `apt-get`, no `doctrine_surface_truth.sh`, no Bevy/GPU-linked binaries

**Rejected prior proof:** run `28623116615` (seven Bevy-linked binaries + desktop bootstrap).

## No-full-battery / no Bevy GitHub proof

- Profile lists exactly one CPU-side `simthing-spec` test binary
- No `apt-get` desktop package install in doctrine exec
- No `doctrine_surface_truth.sh` for `test-deletion-classifier-consolidation`

## Scope Ledger

**0R edits:** CPU-side table in `simthing-spec/tests/**`; removed six Bevy/GPU-linked table files; reverted `doctrine_exec.sh` bootstrap; reverted `simthing-tools/Cargo.toml`; ledgers/docs/profile updated.

**Preserved from initial #1092:** 132 test deletions, stripped mixed files, 3 deleted soak files, audit PARED rows.

## Graduation routing

```text
Graduation routing:
  CI verdict:          PASS-RELIABLE (local stock gates + CPU-only targeted test); GitHub proof pending on 0R head
  Triage entries:      none
  Risk class:          classifier consolidation + test deletion + 0R proof-boundary correction
  Falsification check: 132 rows preserved in CPU table; no Bevy bootstrap; no simthing-sim table; profile CPU-only
  Recommended posture: standard — proof boundary now matches Track D doctrine
```

## Known gaps / next

- CT-2c not in `CONSOLIDATE_TO_TABLE` ledger (out of scope)
- 2 driver source-level rows await promotion rung
- Runtime-specific Bevy-linked proof deferred to future `TEST-CONSOLIDATE-BEVY-LINKED-CLASSIFIER-FAMILIES-0` if needed