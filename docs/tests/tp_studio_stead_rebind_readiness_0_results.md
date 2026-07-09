# TP-STUDIO-STEAD-REBIND-READINESS-0 Results

## Status

**OPEN / report-only production evaluation.** Gap ledger from the accepted workshop-homed
candidate `SimThingScenarioSpec` projection to full Studio session hydrate.

**No implementation** in this rung: no production mapeditor `.clause` API, no UI picker,
no rebind code, no closeout.

## Decision context

| Decision | Citation |
|---|---|
| Workshop-homed clause ingest candidate | #1222 merged @ `bcbc2f4389` — `docs/tests/tp_studio_clause_ingest_0_results.md` |
| Production mapeditor ClauseScript API **denied** | #1224 / `TP-STUDIO-CLAUSE-API-ADMISSION-0` Option B — `docs/tests/tp_studio_clause_api_admission_0_results.md` |
| Binding language | Use: workshop-homed Studio-ingest candidate proof. Do **not** use: production Studio ClauseScript ingest API. |
| Active pointer (pre-report) | `TP-STUDIO-STEAD-REBIND-READINESS-0` in `docs/design_0_0_8_5_clausescript_terran_pirate_galaxy.md` § Phase 8.5 |

## Current accepted stack

```text
.clause bytes
  → parse_raw_document / hydrate_scenario   [simthing-clausething production]
  → workshop project_tp_pack_to_scenario_spec
       crates/simthing-workshop/src/tp_studio_clause_ingest.rs
  → SimThingScenarioSpec candidate
  → serialize/deserialize_scenario_authority  [simthing-spec production]
  → mapeditor JSON scenario_io only           [simthing-mapeditor/src/scenario_io.rs]
```

**Not in stack:** production mapeditor `.clause` path; full `StudioSession::from_loaded_scenario` for candidate Spec.

## Candidate projection shape

Symbol: `project_tp_pack_to_scenario_spec` —
`crates/simthing-workshop/src/tp_studio_clause_ingest.rs`.

| Field | Candidate value | Source |
|---|---|---|
| `scenario_id` | pack.scenario_id (`terran_pirate_galaxy`) | pack |
| `root` | `pack.authority_root` (Scenario → GameSession → Owners + GalaxyMap) | hydrate |
| `structural_grid.frame` | embedded base disc frame | `HydratedEmbeddedStaticGalaxyScenario.source_structural_grid.frame` |
| `structural_grid.map_container_id` | **empty `String`** | hard-coded |
| `structural_grid.placements` | **empty `Vec`** | hard-coded |
| `links` | **empty `Vec`** | hard-coded |
| `provenance` | embedded base provenance | embedded |

Matches FULL-TRANSPILE `authority_spec` test helper residual
(`crates/simthing-clausething/tests/tp_full_transpile_0.rs`): authority tree + frame/provenance,
**no** placement/link rebind onto authority node ids.

## Studio session hydrate gap ledger

Studio load path:

```text
load_scenario_authority_from_path          [scenario_io.rs]
  → StudioSession::from_loaded_scenario    [session.rs]
    → studio_projection_from_scenario_authority [hydration.rs]
      → validate_stead_mapping_consistency [simthing-spec scenario.rs]
```

| Gap | Blocking symbol | Symbol / path |
|---|---|---|
| Empty `map_container_id` | `SteadMappingError::MissingMapContainerId` | `resolve_map_container` in `crates/simthing-spec/src/spec/scenario.rs` |
| Empty `placements` | STEAD / view-model system count zero; RF readiness false | `StudioRfAccumulatorReadiness` / `StudioHydrationBoundary` in `hydration.rs` |
| Empty `links` | Hyperlane projection empty | `build_structural_projection` in `scenario_projection.rs` |
| Candidate Spec can authority-serde | **not blocked** | `serialize_scenario_authority` / `deserialize_scenario_authority` |
| Candidate Spec fails session hydrate | **blocked** | observed at CLAUSE-INGEST 0: `SteadMappingInconsistent("…map_container_id is missing")` |

**Answer:** Moving to full Studio session hydrate requires a **StructuralRebindReady** Spec:
non-empty `map_container_id` resolving to a Location under the spatial root, placements whose
`simthing_id_raw` / location ids bind into the authority tree, and links consistent with those
locations — then `StudioSession::from_loaded_scenario` can rebuild view model + STEAD.

## Spec vs pack metadata boundary

### Belongs on `SimThingScenarioSpec` (Studio authority)

Type: `crates/simthing-spec/src/spec/scenario.rs` — `SimThingScenarioSpec`.

| Surface | Role |
|---|---|
| `root` | Scenario / GameSession / Owner / GalaxyMap / Location tree |
| `structural_grid.frame` | Lattice size / occupied_cells |
| `structural_grid.map_container_id` | STEAD map container (raw id string of GalaxyMap/Location) |
| `structural_grid.placements` | Per-system row/col + `simthing_id_raw` for STEAD |
| `links` | Hyperlane topology in Spec authority |
| `scenario_id` / `provenance` | Identity + generator metadata (sidecar + root properties) |

### Pack-only / not required for Studio hydrate

Type: `HydratedScenarioPack` — `crates/simthing-clausething/src/hydrate_scenario.rs`.

| Surface | Classification for Studio hydrate |
|---|---|
| `game_mode: GameModeSpec` | **Live-run / driver**, not required for mapeditor STEAD view hydrate |
| `grid_metadata` | Authoring feedstock; frame already mirrored via embedded Spec grid |
| `owners` / `ownership_volumes` | Partially mirrored into authority tree; pack rows remain workshop |
| `fleet_ship_payloads` | Pack + tree ships; Studio display optional |
| `combat_arena_payload` | **Live-run / RF workshop** (Mechanism B) |
| `palma_feedstock` / `commitment` / `w_impedance_compose` | **Live-run workshop** |
| `install_targets` | Driver install; LIVE-RUN rebind uses theater targets |

## Placement/link rebind requirements

### What LIVE-RUN already proves (workshop, not Studio Spec writeback)

`apply_live_run_post_hydration` — `crates/simthing-workshop/src/live_run_post_hydration.rs`:

1. Contested border systems selected from ownership volumes on `authority_root`.
2. Theater cells get `(theater_row, theater_col)` on a **7×7** grid; namespaced
   `theater_target_id`.
3. Runtime install clones authority-system shells; `install_targets` map embedded → shell id.
4. Embedded lattice remains STEAD feedstock for **workshop live session**, not full Spec placements.

This is **LiveRunWorkshopOnly**: bounded theater + RF/STEAD execution. It does **not** fill
`SimThingScenarioSpec.structural_grid.placements` for full-galaxy Studio inspect.

### What full Studio rebind must do (future implementation)

| Step | Requirement |
|---|---|
| A | Resolve GalaxyMap / map container raw id under `authority_root` → set `map_container_id` |
| B | For each star-system Location in authority tree (or embedded placement set), emit |
|   | `SimThingStructuralGridPlacement { location_id, target_id, system_id, row, col, simthing_id_raw }` |
|   | with **authority** `simthing_id_raw` (not producer-local only) |
| C | Emit `links` from pack `grid_metadata.links` or embedded source links, id-remapped to authority |
| D | `occupied_cells` / frame consistent with placement count |
| E | `validate_stead_mapping_consistency(spec)` PASS before `StudioSession::from_loaded_scenario` |

Embedded base disc JSON already carries placements under producer ids
(`tp_base_disc_1500.simthing-scenario.json`); rebind is the **join** of embedded lattice coordinates
to authority-tree SimThing ids built during hydrate.

## map_container_id / structural grid policy

| Rule | Source |
|---|---|
| Empty map_container + empty placements | Treated specially in some paths via `is_empty_structural_grid` — still fails session STEAD resolve |
| Non-empty `map_container_id` | Must parse as `u32`, resolve under spatial root, kind `Location`, GalaxyMap-eligible |
| Studio generation path | Sets `map_container_id` from map container raw id (`hydration.rs` generate path) |
| Candidate path | Deliberately empty — FULL-TRANSPILE residual |

**Policy recommendation:** Production Studio-hydratable projection **must not** emit empty grid.
Candidate mode may, for authority-tree-only save/reload proofs.

## GameModeSpec / RF feedstock boundary

| Surface | Studio inspect hydrate | Live-run / driver |
|---|---|---|
| STEAD grid + placements + links | **Required** | Required |
| Galaxy view model / stars / hyperlanes | **Required** | Optional for headless |
| `GameModeSpec` (RF columns, economy, region fields) | **Not required** for mapeditor session hydrate | **Required** for driver SimSession RF |
| combat_arena / fleet emission / commitments | Out of Studio 0 scope | Workshop LIVE-RUN |
| Full-galaxy dense MF | Out of scope (atlas Deviation) | Out of scope |

Studio session hydrate ≠ live RF run. Do not couple rebind implementation to GameMode attach.

## Lowerer heuristic audit

| Surface | Path / symbol | Classification |
|---|---|---|
| `parse_raw_document` | `simthing-clausething` | **safe production spine** |
| Authority tree build / owners as children | `hydrate_scenario` | **safe production spine** (generic scenario grammar) |
| Embedded `static_galaxy_scenario` / source_json | hydrate | **scenario-shaped but tolerated** (generic embed mechanism) |
| Owner-key posture special cases (`terran`/`pirate` in combat finalize) | `hydrate_combat_arena.rs` requires owners `terran`+`pirate`; weapon defaults | **must retire before API admission** or **must become authored data** |
| `TP_FLEET_*` / `TP_SHIP_*` property id constants | `hydrate_scenario.rs` `TP_FLEET_POSTURE_PROPERTY_ID` etc. | **must become authored data** or remain **workshop-only** until registry-generic |
| `fleet_ship_payloads` | pack field | **scenario-shaped but tolerated** as grammar; production API must not hardcode TP fleet semantics |
| `combat_arena_payload` | pack + tree enroll | **must remain workshop-only** for RF/combat until generic combat authoring admitted |
| `project_tp_pack_to_scenario_spec` empty placements | workshop | **must remain workshop-only** as candidate mode; production API must expose only hydratable modes |
| Pack fields born as TP rungs (PALMA, fronts, commitments) | hydrate + workshop post-hydration | **must remain workshop-only** for live-run; not Studio Spec spine |

## Projection mode names

Precise modes for future API / admission language:

### `AuthorityTreeCandidate`

- **Definition:** Spec carries `authority_root` as `root`, frame/provenance from embed or defaults;
  `map_container_id` empty; `placements` empty; `links` empty.
- **Proof today:** workshop `project_tp_pack_to_scenario_spec` + authority serde roundtrip.
- **Studio session hydrate:** **FAIL** (`MissingMapContainerId` / STEAD).
- **Production mapeditor API:** must **not** claim this mode as “open scenario for edit.”

### `StructuralRebindReady`

- **Definition:** Spec has non-empty `map_container_id`, placements bound to authority
  `simthing_id_raw`, links consistent, `validate_stead_mapping_consistency` PASS.
- **GameModeSpec / RF:** not required.
- **Studio session hydrate:** **PASS** (`StudioSession::from_loaded_scenario`).
- **Production mapeditor JSON open:** this is the **minimum** mode for “load for inspect.”

### `StudioSessionHydratable`

- **Definition:** `StructuralRebindReady` **plus** any Studio document/admission invariants
  required by `build_studio_scenario_document_with_admission` (canonical Scenario root,
  GameSession/Owners/GalaxyMap as already enforced for authority_root shapes).
- **Equals:** StructuralRebindReady for current TP authority_root shapes once STEAD fields filled.
- **Production mapeditor API:** may expose **only** this mode for load-from-clause *if* API admission reopens.

### `LiveRunWorkshopOnly`

- **Definition:** Workshop post-hydration theater rebind + RF/combat/commitment attach;
  may use `install_targets` and pack fields without writing full Spec placements.
- **Home:** `simthing-workshop` (`live_run_post_hydration`, fronts, etc.).
- **Production mapeditor API:** must **not** expose this mode.

**Admission sentence template:**

```text
The production mapeditor API only exposes StudioSessionHydratable (StructuralRebindReady Spec),
not AuthorityTreeCandidate or LiveRunWorkshopOnly.
```

## Future tests required

| # | Test intent | Home suggestion |
|---|---|---|
| 1 | Candidate Spec fails `validate_stead_mapping_consistency` | workshop (document status quo) |
| 2 | Rebind helper: authority_root + embedded grid → StructuralRebindReady Spec | workshop until DA admits |
| 3 | Rebind Spec → `StudioSession::from_loaded_scenario` PASS | mapeditor test **or** workshop if deps allow; prefer not pulling Bevy into workshop — may use `validate_stead` + thin projection unit test |
| 4 | Roundtrip: rebind Spec save/load digest stable | workshop + simthing-spec |
| 5 | Negative: dangling map_container_id / placement id | unit |
| 6 | Mode classifier: assert projection mode enum | future small pure module (not TP-named) |

## Non-goals (this rung)

- production mapeditor `.clause` API
- UI file picker
- actual placement/link rebind implementation
- GameModeSpec attach
- new lowerer behavior / ScenarioSpec fields
- live-run / GPU / kernel / closeout

## Recommended next rung

**Primary (bounded implementation path is named):**

```text
TP-STUDIO-STEAD-REBIND-0
```

Workshop-homed (default) helper: `AuthorityTreeCandidate` → `StructuralRebindReady` Spec using
embedded lattice + authority ids; prove `validate_stead_mapping_consistency` + optional Studio
session path without admitting mapeditor `.clause` API.

**If DA wants modes as first-class substrate before rebind code:**

```text
TP-STUDIO-PROJECTION-MODES-0
```

Pure naming + classifier + tests; no rebind algorithm yet.

**Do not reopen** `TP-STUDIO-CLAUSE-API-ADMISSION-0` until at least `StructuralRebindReady`
exists and is proven, and lowerer owner-key / TP property-id debt is retired or scoped.

## Commands

```bash
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/doc_budget_check.sh --check
bash scripts/ci/doctrine_scan.sh
git diff --check
```

No cargo test (report-only).

## Clearance routing

Docs ladder / readiness report shape (`docs-ladder-pointer-correction` if design + orientation +
`*_readiness_0_results.md`). Expect `ORCHESTRATOR-CLEARABLE` or precise
`DA-RESERVE(unclassified-scope)` — **not novelty**.
