# TP-STUDIO-INGEST-READINESS-0 Results

## Status

**DA-GRADUATED / COMPLETE** — merged [#1219](https://github.com/khorum08/SimThing/pull/1219) @ `54a5a5e445468b134aee9b124658a6cdb02999aa` (head `d149e6cce1384e703b83552a9b32b076aa470f4f`). DA exit-proof stamp 2026-07-09.

Report-only evaluation. **No Studio implementation** in this PR. Implementation of Studio ingest is a follow-on production rung, not closeout.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-STUDIO-INGEST-READINESS-0` |
| PR | [#1219](https://github.com/khorum08/SimThing/pull/1219) |
| Merge SHA | `54a5a5e445468b134aee9b124658a6cdb02999aa` |
| Kind | evaluation / report only |
| Approved source | `crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause` |
| Prior proofs | `TP-FULL-TRANSPILE-0` (#1215), `TP-LIVE-RUN-0R2` (#1217) |
| Closeout | **Not** next — `TP-DA-CLOSEOUT-0` is Owner-triggered Workplan Closure Track only |

## Approved source file

```text
crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause
```

Composes: embedded 1500-star disc (`source_json` → `tp_base_disc_1500.simthing-scenario.json`), Terran/Pirate owners, ownership volumes, planet/surface payloads, fleets/ships, combat arena, field_operator / palma_feedstock / commitment authoring metadata.

## Existing proof reuse map

| Proven surface | Where | Studio reuse |
|---|---|---|
| Parse + `hydrate_scenario` of full `.clause` | `simthing-clausething` `tp_full_transpile_0` | **available but not wired** to Studio UI/session |
| Authority root + ownership/fleets/combat hydrate | same | **available** as `HydratedScenarioPack` |
| Canonical `SimThingScenarioSpec` JSON roundtrip | `save_scenario_spec_to_canonical_json` / deserialize | **available**; Studio already uses this for `.simthing-scenario.json` |
| Studio load/save ScenarioSpec authority | `simthing-mapeditor` `scenario_io`, app `load_scenario_*` / `save_scenario_*` | **proven** for JSON ScenarioSpec only |
| Studio hydrate projection from ScenarioSpec | `hydration.rs` `from_scenario` | **proven** for Spec authority → Studio view |
| Bounded theater multi-tick RF/STEAD | workshop `tp_live_run_0` | **proven** in workshop tests; **not** Studio-integrated full session |
| Clausething dependency in mapeditor | Cargo.toml + star-name catalog only | **partial** — no `hydrate_scenario` path |

## Studio ingest target

What “ready” means:

```text
User selects terran_pirate_galaxy.clause (or equivalent path)
  → Studio invokes parse_raw_document + hydrate_scenario
  → projects to SimThingScenarioSpec (authority_root + structural grid policy as DA-accepted)
  → displays/hydrates session for inspection
  → save as .simthing-scenario.json (or accepted format)
  → reload without semantic drift vs transpile proof
  → optional: open SimSession / bounded live path with explicit theater vs full-galaxy boundary
```

## ClauseScript ingest gaps

| Gap | Status |
|---|---|
| Native `.clause` file picker / extension filter | **missing** (Studio pickers target `.simthing-scenario.json`) |
| `parse_raw_document` call from Studio | **not-wired** (API exists in clausething; mapeditor only uses star-name catalog parse) |
| `hydrate_scenario` call from Studio | **not-wired** |
| Spanned hard-error display to UI | **missing** (errors exist on hydrate path; no Studio status mapping for clause spans) |
| `{{FIXTURE_JSON}}` / external base path resolution | **partially-proven** in tests via path substitute; Studio needs stable path policy for embedded `source_json` |

## Transpile/load/save gaps

| Gap | Status |
|---|---|
| Transpile pack → `SimThingScenarioSpec` | **proven** in `tp_full_transpile_0` (test helper `authority_spec`); **not-wired** as a Studio service |
| Save canonical JSON after transpile | **proven** via `save_scenario_spec_to_canonical_json`; **not-wired** from clause ingest |
| Reload canonical JSON | **proven** Studio path for ScenarioSpec files |
| authority_root vs legacy scenario-container `root` dual representation | **deferred** (DA note on FULL-TRANSPILE: converge or Deviation by Studio load rung) |
| Placement/link rebind onto authority nodes | **partially-proven** as LIVE-RUN residue; full Studio install policy **not-wired** |

## Canonical/RON/save-format gap analysis

| Format | Status |
|---|---|
| `.simthing-scenario.json` canonical ScenarioSpec | **proven** Studio default (`SCENARIO_FILE_SUFFIX`) |
| RON Studio document path | **partially-proven** for other Studio document flows; **not required** for clause path if JSON remains authority |
| Persist original `.clause` beside JSON | **not-required** for first Studio ingest (optional provenance); **not-wired** |
| Roundtrip `.clause` → JSON → reload vs re-hydrate clause | **missing** explicit Studio test; **proven** only clause→JSON in clausething test |

## Studio UI / driver boundary gaps

| Surface | Status |
|---|---|
| Menu: Open Scenario (JSON) | **proven** |
| Menu: Open ClauseScript scenario | **missing** |
| Display scenario_id / metadata | **partially-proven** for Spec loads |
| Display ownership volumes / fleet counts | **not-wired** for pack-derived TP metadata (view model is Spec/projection oriented) |
| Display RF combat / diplomacy / front feedstock | **not-wired** (workshop Mechanism B; Studio has no TP pack panel) |
| Driver `SimSession::open_from_spec` from Studio | **partially-proven** via session paths for Spec; full TP game_mode/RF not Studio-driven |

## Runtime/session-load gaps

| Gap | Status |
|---|---|
| Load ScenarioSpec into Studio session | **proven** |
| Attach full TP `GameModeSpec` (RF/economy/region_fields) from hydrate pack | **not-wired** |
| Bounded theater vs full-galaxy live execution boundary UX | **missing** (LIVE-RUN proves theater in workshop; Studio must not imply full-galaxy dense MF) |
| GPU adapter live path from Studio UI | **deferred** / environment-dependent |

## RF/STEAD/live-run reuse boundaries

| Item | Boundary |
|---|---|
| FULL-TRANSPILE | Hydrate/transpile only; placement rebind residual for install/live |
| LIVE-RUN 0R2 | Workshop focused sessions; RF-shaped combat; **not** Studio-integrated |
| Casualty → next ArenaPressureBinding | **deferred** (named LIVE-RUN residue / closeout Deviation) |
| Generic GPU destroyed_ships emission | **deferred** substrate opportunity |
| Dense full-galaxy MF | **deferred** atlas Deviation |

## Studio-readiness gap ledger

| Row | Status | Notes |
|---|---|---|
| ClauseScript file selection / file picker / source input | **missing** | JSON picker only |
| ClauseScript parse invocation from Studio | **not-wired** | `parse_raw_document` exists off Studio |
| hydrate_scenario invocation from Studio | **not-wired** | clausething dep present but unused for scenarios |
| transpile to canonical SimThingScenarioSpec | **partially-proven** | proven in test; no Studio service |
| save canonical transpile artifact | **partially-proven** | JSON save proven for Spec; not after clause ingest |
| reload canonical transpile artifact | **proven** | Studio JSON load path |
| RON or non-JSON save format, if required by Studio | **not-required** | JSON is current authority; RON optional elsewhere |
| authority_root vs legacy scenario-container root convergence | **deferred** | DA residual from FULL-TRANSPILE |
| embedded base lattice / placement/link rebind preservation | **partially-proven** | embedded + LIVE-RUN rebind; Studio install policy open |
| Studio display of scenario metadata | **partially-proven** | Spec path |
| Studio display of ownership volumes | **not-wired** | pack fields not projected |
| Studio display of fleets/ships | **partially-proven** | if present in Spec tree projection; TP pack metadata not dedicated |
| Studio display of RF combat/diplomacy/front/feedstock surfaces | **not-wired** | Mechanism B / pack surfaces |
| Studio load into driver SimSession | **partially-proven** | Spec session paths; full TP game_mode open |
| bounded theater vs full-galaxy live execution boundary | **missing** (UX) / **proven** (workshop) | must stay explicit |
| error display for spanned ClauseScript hard errors | **missing** | |
| semantic-free runtime boundary | **proven** | doctrine-scan + transpile/live-run proofs; must not regress |

## Required future tests

1. Studio unit/integration: open `.clause` path → hydrate → Spec → save JSON → reload digest match vs `tp_full_transpile_0` baseline.
2. Spanned error: corrupt clause surfaces UI status with span.
3. Embedded base path: resolve `source_json` relative to Studio project / fixture root.
4. Optional: Studio-bounded theater live smoke (reuse LIVE-RUN surfaces; GPU optional gate).

## Required future production rungs

Recommended (not closeout):

| Order | Candidate rung | Intent |
|---|---|---|
| 1 | `TP-STUDIO-CLAUSE-INGEST-0` | Wire parse+hydrate+transpile service into mapeditor; clause open path |
| 2 | `TP-STUDIO-CLAUSE-SAVELOAD-0` | Save/reload JSON after clause ingest; digest parity |
| 3 | `TP-STUDIO-TP-PACK-VIEW-0` (optional) | Display ownership/fleet/RF feedstock metadata |
| 4 | `TP-STUDIO-BOUNDED-LIVE-0` (optional) | Explicit theater live, not full-galaxy MF |

`TP-DA-CLOSEOUT-0` remains **Owner-triggered only** after Owner declares the workplan complete.

## Non-goals

No implementation of: Studio UI, file picker, RON serializer, new save/load format, new runtime bridge, new GPU/RF primitive, generic destroyed_ships emission, casualty→ArenaPressureBinding coupling, full-galaxy atlas scheduler, track closeout, lifecycle disposition.

## Load-bearing commands

```text
# Ladder correction / orientation freshness (this docs PR)
bash scripts/ci/gen_orientation.sh
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/doc_budget_check.sh --check
bash scripts/ci/doctrine_scan.sh

# Existing proofs (optional re-run; LIVE-RUN needs GPU)
cargo test -p simthing-clausething --test tp_full_transpile_0 -- --nocapture
cargo test -p simthing-workshop --test tp_live_run_0 -- --nocapture
```

## Graduation routing

| Field | Value |
|---|---|
| Risk class | data-deliverable / docs ladder |
| This report | evaluation complete when DA accepts gap ledger |
| Next production | implement Studio clause ingest (not closeout) |
| Closeout | Owner-triggered Workplan Closure Track only |
