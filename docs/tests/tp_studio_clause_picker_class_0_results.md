# TP-STUDIO-CLAUSE-PICKER-CLASS-0 Results

## Status

**DONE** — registered precedented class `tp-studio-clause-picker` so #1239-shaped admitted picker UI diffs are `ORCHESTRATOR-CLEARABLE` when proof fields pass and the envelope holds.

## Class row

| Field | Value |
|---|---|
| class_id | `tp-studio-clause-picker` |
| envelope | `0.0.8.5-terran-pirate` |
| status | `active` |
| promotion_blocker | `TP-STUDIO-CLAUSE-PICKER-CLASS-0` |
| table | `scripts/ci/precedented_classes.tsv` |

Primary selection: picker shape (`clause_scenario_picker.rs` or `tests/tp_studio_clause_picker_*.rs`) beats API composition and workshop-candidate.

## Class envelope

Allowed surfaces:

```text
crates/simthing-mapeditor/src/clause_scenario_picker.rs
crates/simthing-mapeditor/src/app/scenario_io.rs
crates/simthing-mapeditor/src/app/ui.rs
crates/simthing-mapeditor/src/app/mod.rs
crates/simthing-mapeditor/src/lib.rs
crates/simthing-mapeditor/tests/tp_studio_clause_picker_*.rs
crates/simthing-mapeditor/tests/tp_studio_clause_api_*.rs
docs/tests/tp_studio_clause_picker_*_results.md
scripts/ci/test_inventory.tsv
scripts/ci/test_lifecycle_boundary_rows.tsv
docs/design_0_0_8_5_*.md
docs/orchestrator_orientation.md
```

Path gate: requirement `picker_only` → out-of-envelope (runtime/GPU/kernel/sim/workshop/closeout) → `DA-RESERVE(class-envelope-violation)`.

## Required proof fields

Requirements token list:

```text
tested_code_sha|coverage_basis|ci_green|studio_clause_picker|production_api_only|ui_file_picker_yes|no_tp_defaults|session_hydrate|no_duplicate_parse_rebind|no_gamemode_rf_live_closeout|picker_only
```

Expected body values:

```text
studio_clause_picker: ADMITTED_NARROW_UI
production_api_only: YES
ui_file_picker: YES
tp_defaults_in_production: NO
session_hydrate: PASS
duplicate_parse_rebind_path: NO
gamemode_rf_attach: NO
live_run_state: NO
closeout_run: NO
track_closeout_apply: NO
tested_code_sha: <8+ hex>
coverage_basis: PASS
ci_green: PASS (fixture ci_status / live checks)
```

## #1239-shaped clearable proof

Fixture `clearance_selftest_picker_class_clearable` → `ORCHESTRATOR-CLEARABLE`.

## Missing fields proof

Fixture `clearance_selftest_picker_class_missing_fields` (no `production_api_only`) →
`FAIL(missing-picker-proof-fields: production_api_only: YES required)`.

## Production API-only rejection proof

`production_api_only: NO` → `DA-RESERVE(class-envelope-violation)`.

## TP/default rejection proof

`tp_defaults_in_production: YES` → `DA-RESERVE(class-envelope-violation)`.

## Duplicate parse/rebind rejection proof

`duplicate_parse_rebind_path: YES` → `DA-RESERVE(class-envelope-violation)`.

## GameMode/RF/live-run/closeout rejection proof

`closeout_run: YES` (representative) → `DA-RESERVE(class-envelope-violation)`.
Same path for `gamemode_rf_attach: YES`, `live_run_state: YES`, `track_closeout_apply: YES`.

## Runtime/GPU/kernel rejection proof

#1239 shape + `crates/simthing-sim/src/engine_source_change.rs` → `DA-RESERVE(class-envelope-violation)`.

## Admitted-scope-router-gap non-regression

Picker-adjacent path outside class with valid admitted-envelope body →
`DA-RESERVE(admitted-scope-router-gap)` (fixture `clearance_selftest_picker_class_admitted_scope_gap`).

## API class non-regression

#1230-shaped composition still `ORCHESTRATOR-CLEARABLE` under `tp-admitted-clause-api-composition`
(fixture `clearance_selftest_picker_class_api_nonregression`); not stolen by picker class.

## Gate-wiring preservation

Fixture touching `clearance_check.sh` + `precedented_classes.tsv` → `DA-RESERVE(gate-wiring)`.

## Fixture lifecycle posture

Committed under `scripts/ci/fixtures/clearance/clearance_selftest_picker_class_*` (10 fixtures).
Ledgered in `scripts/ci/test_inventory.tsv` as seal-proof harness fixtures for
`TP-STUDIO-CLAUSE-PICKER-CLASS-0`. Not scenario artifacts. `fixture_accretion: LEDGERED`.

## Commands

```bash
bash scripts/ci/clearance_check.sh --selftest
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_picker_class_clearable
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
bash scripts/ci/doc_budget_check.sh --check
bash scripts/ci/doctrine_scan.sh
git diff --check
bash scripts/ci/clearance_check.sh --pr <PR_NUMBER>
```

## Clearance routing

This PR changes router/class surfaces → expected:

```text
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
```

Do **not** self-merge.

## Known gaps

- Forbidden-surface body fields are declarative (claim-checked); deep semantic code analysis is out of scope.
- Option C admitted-envelope registry remains later harness work.
- No product picker behavior changed.

## Recommended next action

Owner/DA **Phase 8 completeness / readiness decision**.

**Not next:** product picker work, GameMode/RF attach, live-run, runtime/GPU/kernel, closeout
(unless Owner explicitly declares workplan complete).
