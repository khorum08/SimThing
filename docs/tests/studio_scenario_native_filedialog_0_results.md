# STUDIO-SCENARIO-NATIVE-FILEDIALOG-0 — Native scenario file picker for Load Scenario

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added STUDIO-SCENARIO-NATIVE-FILEDIALOG-0 PROBATION row |
| `docs/tests/studio_scenario_native_filedialog_0_results.md` | PROBATION | This report |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

This pass fixes a concrete Studio UX failure: **Load Scenario** depended on a CWD-relative text path. The primary button now opens a native OS file dialog, stores an absolute/canonical path in the presentation path field, and loads through existing `SimThingScenarioSpec` authority IO.

## Pre-edit orientation answers

| Question | Answer |
|---|---|
| Where is Load Scenario handled? | `crates/simthing-mapeditor/src/app/ui.rs` `draw_scenario_io_controls`; load actions in `app/scenario_io.rs` |
| Existing file-dialog dependency? | None — added `rfd` to `simthing-mapeditor` only |
| Why limited to mapeditor? | Native picker is Studio desktop presentation; spec/driver/sim crates must not depend on UI dialogs |
| Path canonicalization? | `canonicalize_scenario_display_path` uses `std::fs::canonicalize`, else absolute/CWD-joined fallback |
| Cancel behavior? | Session and `scenario_path_text` preserved |
| Invalid selection? | Session preserved; path restored to pre-picker value on validation failure |
| Path is presentation only? | Yes — not serialized into scenario authority or studio config content |

## Native dialog dependency

`rfd` 0.15 added to `crates/simthing-mapeditor/Cargo.toml` only. `NativeScenarioFilePicker` wraps `rfd::FileDialog` with title **Load SimThing Scenario** and filter **SimThing Scenario (*.simthing-scenario.json)**. Tests use `FakeScenarioFilePicker` — no dialog in CI.

## Programmatic path population (agent/testing caveat)

`set_programmatic_scenario_path(state, path)` populates the scenario path text field with a canonicalized absolute path without opening the dialog. Exported from `simthing-mapeditor` on Windows for agents/tests. Pair with **Manual Load Path** or `load_scenario_manual_path_action`.

## Load behavior summary

| Event | Session | Path field |
|---|---|---|
| Cancel | Preserved | Preserved |
| Invalid extension/config | Preserved | Restored to pre-picker value |
| Load failure | Preserved | Updated to selected absolute path (diagnostic) |
| Load success | Replaced with loaded session | Updated to selected absolute path |

## Authority boundary proof

Load still uses `load_studio_session_from_scenario_path` → `load_scenario_authority_from_path` → `deserialize_scenario_authority` with STEAD/link/ID validation. Studio config and path text are not model authority.

## Tests added

**simthing-mapeditor** (`app/scenario_io.rs`): 13 picker/programmatic tests plus existing scenario save/load UI tests.

**simthing-mapeditor** (`studio_scenario_load.rs`): 2 path helper tests.

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo check -p simthing-spec
cargo test -p simthing-spec --lib
cargo check -p simthing-core
cargo test -p simthing-core
cargo check -p simthing-gpu
cargo test -p simthing-gpu
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
git diff --check
```

## Windows/resource-limit notes

None observed for this PR's validation commands.

## Files changed

- `crates/simthing-mapeditor/Cargo.toml`
- `crates/simthing-mapeditor/src/studio_scenario_load.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/src/app/scenario_io.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/studio_scenario_native_filedialog_0_results.md`
- `Cargo.lock`

## Deleted/archived artifacts

None.

## Deferred work

ACCUMULATOR-CONVERGENCE-0, native Save Scenario dialog, platform app-data scenario directories, full runtime vertical-test execution.

## DA status

**PROBATION** — pending owner design-authority approval. No GPU adapter evidence required for this UX pass.