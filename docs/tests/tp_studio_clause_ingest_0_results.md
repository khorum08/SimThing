# TP-STUDIO-CLAUSE-INGEST-0 Results

## Status

**PROOF-PRESENT / Studio wiring.** Studio-facing ClauseScript scenario ingest opens the approved
`terran_pirate_galaxy.clause`, routes through `simthing-clausething` parse/hydrate, projects to
canonical `SimThingScenarioSpec`, and proves save/reload via existing Studio ScenarioSpec authority
IO without semantic drift.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-STUDIO-CLAUSE-INGEST-0` |
| Kind | Studio / mapeditor wiring (minimal service path) |
| Approved source | `crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause` |
| Embedded base | `crates/simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json` (`{{FIXTURE_JSON}}`) |
| Prior proofs | `TP-FULL-TRANSPILE-0` (#1215), `TP-LIVE-RUN-0R2` (#1217), readiness report `tp_studio_ingest_readiness_0_results.md` |
| Closeout | **Not** this rung — Owner-triggered only |

## Implemented path

```text
.clause file path
  → Studio-facing ingest_clause_scenario_path
  → resolve {{FIXTURE_JSON}} → embedded base-disc JSON
  → parse_raw_document (simthing-clausething)
  → hydrate_scenario (simthing-clausething)
  → project_hydrated_pack_to_scenario_spec (authority_root + embedded frame/provenance)
  → SimThingScenarioSpec
  → save_scenario_authority_to_path (.simthing-scenario.json)
  → load_scenario_authority_from_path
  → canonical digest / byte parity
```

## Approved source file

```text
crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause
```

## Existing proof reuse

| Surface | Reuse |
|---|---|
| Parse + hydrate full TP clause | `simthing-clausething` (same path as FULL-TRANSPILE) |
| Authority projection shape | FULL-TRANSPILE `authority_spec` (empty placements until install rebind) |
| Canonical JSON save/load | Studio `scenario_io` + `save_scenario_spec_to_canonical_json` |
| LIVE-RUN | **Cited only** for runtime liveness boundary; not re-proven |

## Studio-facing ingest API

| API | Role |
|---|---|
| `ingest_clause_scenario_path` | Primary Studio-facing open path |
| `project_hydrated_pack_to_scenario_spec` | Pack → ScenarioSpec (no second authority) |
| `load_studio_session_from_clause_path` | Optional session adopt when STEAD allows |
| `StudioClauseIngestError::status_message` | UI/status Display string |
| `StudioClauseIngestError::hydrate_token_index` | Spanned hydrate token when present |

Module: `crates/simthing-mapeditor/src/studio_clause_ingest.rs`

UI file picker: **DEFERRED** — service path and save/load parity are proven; native `.clause` picker remains future work.

## Load/save roundtrip proof

Test: `tp_studio_clause_ingest_0_load_save_roundtrip`

- Opens approved `.clause` via Studio-facing ingest (not a test-only clausething helper).
- Scenario id: `terran_pirate_galaxy`.
- Saves via `save_scenario_authority_to_path`.
- Reloads via `load_scenario_authority_from_path`.
- Canonical JSON bytes and `authority_digest` match (no semantic drift).

## Error-path proof

Test: `tp_studio_clause_ingest_0_malformed_clause_error_context`

- Malformed clause snippet fails ingest.
- `status_message()` is non-empty and carries ClauseThing parse/hydrate context for UI/status display.
- Full span-to-UI panel mapping is **follow-on** if richer diagnostics are required.

## Non-goals

- track closeout / `TP-DA-CLOSEOUT-0`
- full Studio UI panels for RF/combat/front metadata
- full-galaxy Movement-Front atlas / 1000-tick soak
- generic GPU `destroyed_ships` emission
- casualty → next STEAD `ArenaPressureBinding` coupling
- new parser semantics / grammar
- new runtime/GPU/kernel code
- UI `.clause` file picker (deferred)

## Boundary / future work

| Item | Notes |
|---|---|
| Placement/link rebind onto authority nodes | FULL-TRANSPILE residue; empty placements intentional |
| Studio session STEAD hydrate for TP pack | Fails until map_container/placements rebind; authority IO path is the 0 proof |
| UI Open ClauseScript menu / extension filter | Deferred |
| Richer spanned error UI | Token index available on hydrate errors; panel wiring deferred |

## Commands

```bash
cargo test -p simthing-mapeditor --test tp_studio_clause_ingest_0 -- --nocapture
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

Studio/mapeditor service + integration test + inventory/lifecycle + results report.
Expect router verdict under hardened clearance (not novelty). Gate-wiring only if harness surfaces touched.
