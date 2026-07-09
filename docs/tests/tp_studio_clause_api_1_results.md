# TP-STUDIO-CLAUSE-API-1 Results

## Status

**PROOF-PRESENT / limited production ClauseScript composition API.**

Implements admitted Option A surface: caller-supplied clause path/bytes + source resolver →
clausething parse/hydrate → StructuralRebindReady rebind → STEAD validate → authority serde →
Studio session hydrate. **No** UI picker, TP production defaults, GameMode/RF attach, or closeout.

## Admitted scope

| Allowed | Status |
|---|---|
| Generic composition under StructuralRebindReady | **done** |
| Caller-supplied path/bytes + resolver | **done** |
| clausething parse/hydrate | **done** |
| Generic rebind (elevated to clausething) | **done** |
| simthing-spec authority serde + STEAD validate | **done** |
| StudioSession hydrate exit proof | **done** |

## Implemented API

Home: `crates/simthing-mapeditor/src/clause_scenario_ingest.rs`

| Symbol | Role |
|---|---|
| `ClauseScenarioSourceResolver` | Caller placeholder → path map |
| `ClauseScenarioIngestOptions` | Mode + resolver (default mode StructuralRebindReady; **empty** resolver) |
| `ingest_clause_scenario_path` / `ingest_clause_scenario_bytes` | Compose parse/hydrate/rebind |
| `save_clause_scenario_authority_to_path` | Existing scenario_io authority save |
| `load_clause_studio_session_from_path` | Ingest → save JSON → `load_studio_session_from_scenario_path` |
| `load_studio_session_from_clause_ingest_result` | Direct `StudioSession::from_loaded_scenario` |

Generic projection/rebind spine: `crates/simthing-clausething/src/clause_scenario_projection.rs`

## Resolver policy

- No silent TP fixture paths in production.
- Unresolved `{{...}}` placeholders → structured `SourceResolution` error.
- Tests supply `{{FIXTURE_JSON}}` → base-disc path **only as caller data**.

## Projection mode

```text
ClauseScenarioProjectionMode::StructuralRebindReady
```

`AuthorityTreeCandidate` is internal (clausething helper); not a production open mode.

## StructuralRebindReady proof

Test: `api_1_clause_path_to_structural_rebind_ready_spec` — non-empty map_container_id + placements.

## STEAD validation proof

`validate_stead_mapping_consistency` PASS on API output.

## Scenario links proof

When non-empty, `validate_scenario_links` PASS.

## Authority serde roundtrip proof

`api_1_authority_serde_roundtrip_preserves_structural_rebind_ready` — save/load via scenario_io; STEAD still PASS.

## Studio session hydrate proof

```text
session_hydrate: PASS
```

- `load_clause_studio_session_from_path` → `load_studio_session_from_scenario_path` PASS
- `load_studio_session_from_clause_ingest_result` → `StudioSession::from_loaded_scenario` PASS

## No-defaults proof

`api_1_no_production_tp_defaults` — empty resolver fails; production source has no
`tp_base_disc_1500` / `terran_pirate_galaxy` / `TP-FULL-TRANSPILE` strings.

## No UI picker proof

`api_1_no_ui_picker_surface` — API module has no rfd/dialog/picker hooks; no clause picker export.

## Non-goals (honored)

- UI `.clause` picker
- GameMode / RF / combat / palma / commitment attach
- live-run theater
- closeout
- claiming “Studio ClauseScript ingest done” beyond StructuralRebindReady composition

## Commands

```bash
cargo test -p simthing-mapeditor --test tp_studio_clause_api_1 -- --nocapture
cargo check -p simthing-mapeditor
cargo check -p simthing-clausething
cargo check -p simthing-workshop
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
bash scripts/ci/doc_budget_check.sh --check
bash scripts/ci/doctrine_scan.sh
git diff --check
```

## Clearance routing

Production mapeditor + clausething under explicit DA admission. Expect
`ORCHESTRATOR-CLEARABLE` only if a class covers this shape; else
`DA-RESERVE(unclassified-scope)` — **not novelty**.

## Known gaps

- UI picker still requires separate admission (`TP-STUDIO-CLAUSE-PICKER-ADMISSION-0`)
- Lowerer owner-key / TP property-id residual debt remains for future widenings
- GameMode/RF attach not in this API

## Recommended next rung

```text
TP-STUDIO-CLAUSE-PICKER-ADMISSION-0
```
