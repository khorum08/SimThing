# TP-STUDIO-CLAUSE-INGEST-0 / 0R Results

## Status

**DA-GRADUATED / COMPLETE (0R)** — merged [#1222](https://github.com/khorum08/SimThing/pull/1222) @ `bcbc2f4389417a0a4aa12b9e0e71a13414d94981` (head `ee0480c438f7215e49dca8309a920a0f0f957a00`). DA acceptance 2026-07-09: workshop-homed Studio-ingest **candidate** proof; no production mapeditor ClauseScript API.

The ingest helper is **workshop-homed candidate code**.
The proof uses existing Studio/mapeditor ScenarioSpec **authority** IO (via production
`simthing-spec` serialize/deserialize — the same layer `simthing-mapeditor::scenario_io` wraps).
**No production Studio API is admitted by this rung.**
Production-crate elevation is a future DA/Owner admission decision (`TP-STUDIO-CLAUSE-API-ADMISSION-0`).

## Identity

| Field | Value |
|---|---|
| Rung | `TP-STUDIO-CLAUSE-INGEST-0R` (homing boundary) |
| PR | [#1222](https://github.com/khorum08/SimThing/pull/1222) |
| Merge SHA | `bcbc2f4389417a0a4aa12b9e0e71a13414d94981` |
| tested_code_sha | `ee0480c438f7215e49dca8309a920a0f0f957a00` |
| Kind | workshop-homed scenario-candidate service + proof |
| Home | `crates/simthing-workshop/src/tp_studio_clause_ingest.rs` |
| Approved source | `crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause` |
| Embedded base | `crates/simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json` (`{{FIXTURE_JSON}}`) — **caller/test default, not production Studio default** |
| Prior proofs | FULL-TRANSPILE-0, LIVE-RUN-0R2 (cited only), readiness report |
| Homing test | Would this code exist if this scenario didn't? → **no** → workshop |

## Implemented path

```text
approved terran_pirate_galaxy.clause
  → workshop-homed ingest_tp_clause_scenario_path
  → parse_raw_document + hydrate_scenario (simthing-clausething production)
  → project_tp_pack_to_scenario_spec (candidate authority_root shape)
  → save via production ScenarioSpec authority serde (simthing-spec;
      same authority layer as mapeditor scenario_io)
  → reload via production ScenarioSpec authority serde
  → canonical digest / byte parity
```

## Dependency note (mapeditor)

`simthing-workshop` does **not** depend on `simthing-mapeditor` (Bevy shell). Pulling mapeditor
into workshop would force a heavy GUI/runtime dependency for a candidate proof.

Instead the proof calls production `serialize_scenario_authority` /
`deserialize_scenario_authority` / `save_scenario_spec_to_canonical_json` from
`simthing-spec`, which is exactly what mapeditor `scenario_io` wraps. The integration
shape is proven without elevating a production Studio API.

## Approved source file

```text
crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause
```

## Existing proof reuse

| Surface | Reuse |
|---|---|
| Parse + hydrate full TP clause | production `simthing-clausething` |
| Authority projection shape | FULL-TRANSPILE candidate shape (empty placements until rebind) |
| Canonical JSON save/load | production `simthing-spec` authority serde |
| LIVE-RUN | cited only; not re-proven |

## Workshop-facing candidate API

| API | Role |
|---|---|
| `ingest_tp_clause_scenario_path` | Workshop-homed open path |
| `project_tp_pack_to_scenario_spec` | Pack → ScenarioSpec candidate |
| `save_scenario_authority_json_to_path` | Authority save via production serde |
| `load_scenario_authority_json_from_path` | Authority load via production serde |
| `TpStudioClauseIngestError::status_message` | Status/Display string |

## Load/save roundtrip proof

Test: `tp_studio_clause_ingest_0_load_save_roundtrip` (workshop)

- Opens approved `.clause` via workshop-homed helper.
- Scenario id: `terran_pirate_galaxy`.
- Saves/reloads via production ScenarioSpec authority serde.
- Canonical JSON bytes and `authority_digest` match.

## Error-path proof

Test: `tp_studio_clause_ingest_0_malformed_clause_error_context` (workshop)

- Malformed clause fails ingest.
- `status_message()` non-empty with ClauseThing/TP context.

## Non-goals

- production `simthing-mapeditor` ClauseScript ingest API
- UI file picker
- track closeout
- full Studio session STEAD rebind
- live run / GPU / kernel / new grammar
- self-granted substrate widening

## Boundary / future work

| Item | Notes |
|---|---|
| Homing | Workshop until DA/Owner admits generic Studio API |
| Production elevation | Requires explicit DA/Owner authorization |
| STEAD session hydrate | Needs placement/map_container rebind (FULL-TRANSPILE residue) |
| UI Open ClauseScript | Deferred |

## Commands

```bash
cargo test -p simthing-workshop --test tp_studio_clause_ingest_0 -- --nocapture
cargo check -p simthing-workshop
cargo check -p simthing-mapeditor
cargo check -p simthing-clausething
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
bash scripts/ci/doc_budget_check.sh --check
bash scripts/ci/doctrine_scan.sh
git diff --check
```

## Clearance routing

Workshop-homed TP candidate + inventory/lifecycle + report.
Expect `ORCHESTRATOR-CLEARABLE` if a workshop class matches, else precise reserve
(`unclassified-scope` / envelope) — **not novelty**.
