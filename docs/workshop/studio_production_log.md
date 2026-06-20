# SimThing Studio Production Log

> Workshop worklog for PR-by-PR history, validation tables, changelog blocks, and verbose PROBATION notes.
> The authoritative production ladder lives in [`docs/design_0_0_8_3_studio_production.md`](../design_0_0_8_3_studio_production.md).
> Per-rung validation evidence remains in docs/tests/*_results.md.

Moved from production doc by **STUDIO-PRODUCTION-LADDER-CLOSURE-0R** on 2026-06-20.

---

## SAVELOAD-AUTHORITY-PIN-0R — Authority hardening

**Date / PR:** 2026-06-17 — SAVELOAD-AUTHORITY-PIN-0R

### What changed

- Pinned whole `SimThingScenarioSpec` as the serialized save/load authority with explicit serde helpers and post-deserialize validation.
- `structural_grid.map_container_id` now resolves to an actual `Location` SimThing raw id in the recursive tree; first-Location fallback removed.
- Structural integer mirrors (`system_id`, `col`, `row`) are range-checked to `<= 16_777_216` exact f32 integers; primary authority remains `structural_grid.placements`.
- Loaded scenarios reserve ids through `reserve_simthing_ids_from_scenario` with explicit `DuplicateId` / `IdSpaceExhausted` errors.
- Invalid STEAD mapping defers RF and heatmap readiness (`InvalidSteadMapping`) instead of classifying bounded/atlas execution.
- Added `apply_gridcell_property_edit` model-first mutation helper and projection-rebuild proof tests.

### Authority / substrate implications

`SimThingScenarioSpec` is the sole model substrate after generation. `StudioHydrationBoundary`, `StudioHydratedGrid`, `StudioGalaxyViewModel`, render anchors, and Bevy entities remain projections refreshed from scenario authority.

### STEAD / Mapping implications

`resolve_map_container` binds `map_container_id` to a direct `World` child `Location`. Gridcell `Location` SimThings must be children of that exact container. Validator rejects dangling ids, non-`Location` containers, and gridcells under the wrong parent.

### RF / Accumulator implications

`StudioRfAccumulatorReadiness::from_scenario` and the deferred wrapper derive participant counts from gridcells under the declared map container. Invalid STEAD mapping yields `ready_for_spatial_rf_over_locations = false` with a deferred reason.

### Heatmap / Movement-Front implications

`StudioHeatmapReadinessKind::InvalidSteadMapping` is distinct from `AtlasRequired`. Valid small grids classify as `BoundedTheaterEligible`; valid oversized dense layouts classify as `AtlasRequired` (execution deferral, not layout failure). Invalid STEAD never classifies as bounded or atlas-required.

### Save/load implications

Authority roundtrip preserves root tree, structural grid, map container binding, links, provenance, and children. `deserialize_scenario_authority` validates and reserves ids on load. Save/load UI remains deferred.

### Evidence lifecycle status

SAVELOAD-AUTHORITY-PIN-0R evidence is **PROBATION** pending DA approval. Prior SAVELOAD-AUTHORITY-PIN-0 evidence remains branch context; this rung hardens rather than replaces it.

### Deferred work

Save/load file IO and UI, ClauseThing import wiring, full editor mutation command surface, heatmap/RF visualization, atlas execution, live simulation, and exact typed `PropertyValue` enum expansion (f32 mirror range-check chosen for compatibility).

## Current Evidence And Lifecycle Status

The current evidence ledger remains the compact live guardrail index. Studio PR1 through PR2R12, STUDIO-HYDRATION-BOUNDARY-0, STUDIO-SIMTHING-SPEC-BOUNDARY-1, SAVELOAD-AUTHORITY-PIN-0, SAVELOAD-AUTHORITY-PIN-0R, and SIMTHING-STUDIO-CONFIG-0 are PROBATION unless and until DA-promoted. STEAD contract rows that were DA-approved remain CURRENT_EVIDENCE.

## SIMTHING-STUDIO-CONFIG-0 — Studio config persistence

**Date / PR:** 2026-06-17 — SIMTHING-STUDIO-CONFIG-0

### What changed

- Added presentation-only `simthing-studio-config.json` in the working directory (`schema_version = 1`).
- Studio loads and validates config at startup; missing file uses defaults; invalid file fails open to defaults with a status warning.
- Config saves atomically on Settings window close (X or Close) and on app exit.
- Settings window gained a Reset button that restores star/hyperlane render settings and dialog-controlled values to defaults without touching scenario authority.

### Presentation vs authority

`simthing-studio-config.json` stores editor UI/render preferences only: settings dialog state, star/hyperlane render controls, view toggles, and optional camera snapshot. It does **not** serialize `SimThingScenarioSpec`, root `SimThing`, structural grid, generated galaxy data, RF/Accumulator state, or runtime vertical-test state.

### Future runtime vertical-test compatibility

Future runtime vertical-test loading inside Studio must load or synthesize `SimThingScenarioSpec` (or equivalent scenario authority) separately. Studio config may control view/render preferences around that authority but cannot become the runtime/scenario source of truth.

### Evidence lifecycle status

SIMTHING-STUDIO-CONFIG-0 evidence is **PROBATION** pending DA approval.

## SCENARIO-SAVELOAD-IO-0 — SimThing-Spec scenario file IO

**Date / PR:** 2026-06-17 — SCENARIO-SAVELOAD-IO-0

### What changed

- Added backend scenario/model file IO in `crates/simthing-mapeditor/src/scenario_io.rs` using `.simthing-scenario.json` files (separate from `simthing-studio-config.json`).
- `save_scenario_authority_to_path` / `load_scenario_authority_from_path` wrap `serialize_scenario_authority` / `deserialize_scenario_authority` with atomic tmp→rename writes.
- `save_current_session_scenario_to_path` persists `session.scenario_authority` only.
- `load_studio_session_from_scenario_path` validates STEAD mapping and links, reserves SimThing IDs, and rebuilds `StudioHydrationBoundary` + `StudioGalaxyViewModel` from authority.
- `validate_scenario_links` added to `simthing-spec` and wired into `deserialize_scenario_authority`.
- `StudioSession::from_scenario_authority` supports projection rebuild without regenerating from MapGenerator.

### Scenario vs config separation

Scenario/model save/load is separate from `simthing-studio-config.json`. Scenario files serialize the whole `SimThingScenarioSpec` (recursive `root`, `structural_grid`, `map_container_id` binding, `placements`, `links`, `provenance`). Naked `root: SimThing` alone remains insufficient save authority. Studio config remains presentation-only.

### Load path

Loading validates STEAD mapping (`validate_stead_mapping_consistency`), link endpoints (`validate_scenario_links`), and reserves existing SimThing IDs before future spawns. Projections rebuild through `studio_projection_from_simthing_spec` and `StudioGalaxyViewModel::from_hydration` using structural `(col,row)` — not render anchors or Bevy state.

### Future runtime vertical-test horizon

Future runtime vertical-test loading should enter through:

```text
runtime vertical-test source
  -> SimThing-Spec scenario/runtime authority
  -> Studio projection
  -> GPU-resident execution/readiness surfaces
```

This pass does not implement vertical-test load. Scenario state must not live in Bevy entities, Studio config, view models, or GPU-resident render buffers as authority.

### Evidence lifecycle status

SCENARIO-SAVELOAD-IO-0 evidence is **PROBATION** pending DA approval.

## SCENARIO-SAVELOAD-UI-0 — Scenario authority UI wiring

**Date / PR:** 2026-06-17 — SCENARIO-SAVELOAD-UI-0

### What changed

- Left Studio panel now exposes **Save Scenario** / **Load Scenario** controls with an editable scenario path field (default `simthing-current.simthing-scenario.json`).
- UI actions call existing `save_current_session_scenario_to_path` / `load_studio_session_from_scenario_path` helpers via `app/scenario_io.rs` action functions.
- Save requires an active `StudioSession`; load replaces the session only on success and rebuilds Bevy galaxy scene from the loaded view model.
- `scenario_path_text` and `last_scenario_io_status` are presentation-only Studio UI state (not persisted in scenario authority or `simthing-studio-config.json`).

### Scenario vs config separation

Save Scenario / Load Scenario operate on `.simthing-scenario.json` files containing whole `SimThingScenarioSpec`. `simthing-studio-config.json` remains separate presentation-only config with independent save-on-settings-close and save-on-app-exit behavior.

### Load failure behavior

Failed scenario load preserves the current session and reports `Scenario load failed: <reason>` in the status panel.

### Loaded-session compatibility bridge

The Save Scenario / Load Scenario UI loads SimThing-Spec authority and rebuilds Studio projections. Future runtime vertical-test loading must target this authority layer, then attach GPU-resident execution/readiness surfaces, not bypass through Bevy state or Studio config.

Loaded sessions may still carry a synthetic `GenerationRunOutput` for report/view projection compatibility. That bridge is temporary and is not model authority; `session.scenario_authority` remains the loaded `SimThingScenarioSpec`.

### Evidence lifecycle status

SCENARIO-SAVELOAD-UI-0 evidence is **PROBATION** pending DA approval.

## SCENARIO-NATIVE-SESSION-0 — Scenario-native loaded sessions and GPU-resident projection readiness

**Date / PR:** 2026-06-17 — SCENARIO-NATIVE-SESSION-0

### What changed

- Loaded scenarios are no longer treated as generated sessions. `StudioSessionSource::LoadedScenario` vs `Generated` records authority provenance explicitly.
- The synthetic `GenerationRunOutput` bridge is **removed** for loaded scenarios (`generated_output: None`). Generated sessions retain `generated_output: Some(output)` for MapGenerator report evidence.
- `StudioScenarioSummary` derives from `SimThingScenarioSpec` (counts, STEAD/link validity, RF/heatmap readiness). Loaded status text reads `Loaded scenario: N systems, M links, STEAD valid` — not "Generated".
- Dense structural projection tables (`StudioStructuralProjection` with location/link index rows) derive from scenario authority and are future GPU-upload feedstock.
- `StudioGpuResidencyReadiness` is a projection/cache (not authority) describing whether dense structural tables and RF/MF/runtime surfaces can be prepared without allocating GPU buffers.
- RF/Accumulator and heatmap readiness remain scenario-derived; invalid STEAD blocks RF/heatmap; valid oversized frames report `AtlasRequired`.

### Architecture path

```text
Loaded .simthing-scenario.json
  -> SimThingScenarioSpec authority
  -> scenario-native StudioSession (source=LoadedScenario, generated_output=None)
  -> scenario_summary + structural_projection + gpu_residency_readiness
  -> StudioHydrationBoundary
  -> StudioGalaxyViewModel
  -> Bevy projection
```

Future runtime vertical-test loading should enter through SimThing-Spec scenario/runtime authority and attach GPU-resident execution/readiness surfaces — not through synthetic MapGenerator output or Bevy entity state.

### Save/load behavior preserved

Save Scenario / Load Scenario still operate on whole `SimThingScenarioSpec`. Load failure preserves the current session. `simthing-studio-config.json` remains presentation-only.

### Evidence lifecycle status

SCENARIO-NATIVE-SESSION-0 evidence is **PROBATION** pending DA approval.

## STUDIO-PANEL-GAP-AND-SCENARIO-LINK-CANON-0 — Panel inset fix and canonical structural links

**Date / PR:** 2026-06-17 — STUDIO-PANEL-GAP-AND-SCENARIO-LINK-CANON-0

### What changed

- Main left floating panel now preserves a bottom inset equal to its left inset (`left_panel_rect` in `panel_layout.rs`). Panel body scrolls inside the inset bounds.
- Panel layout remains presentation-only and is not model authority.
- Scenario links are structural adjacency edges, not routes. Validation rejects unknown endpoints, self-links, direct duplicates, and reversed duplicates.
- `canonical_scenario_link_pair` / `canonical_scenario_link_key` provide deterministic undirected edge keys.
- `StudioStructuralProjection` emits sorted canonical dense link rows `(min_dense_index, max_dense_index)`.
- GPU-resident readiness cannot be built from invalid graph links.

### Future runtime path

Future runtime vertical-test loading must enter through canonical `SimThingScenarioSpec` authority and structural graph projection before attaching GPU-resident execution/readiness surfaces.

### Evidence lifecycle status

STUDIO-PANEL-GAP-AND-SCENARIO-LINK-CANON-0 evidence is **PROBATION** pending DA approval.

## GPU-STRUCTURAL-UPLOAD-PACKET-0 — Structural upload packet for GPU-resident horizon

**Date / PR:** 2026-06-17 — GPU-STRUCTURAL-UPLOAD-PACKET-0

### What changed

- Added `StudioGpuStructuralUploadPacket` with `repr(C)` frame, location, and link rows derived from `SimThingScenarioSpec` through `StudioStructuralProjection`.
- Upload packet is projection/cache, not authority. No render metadata, Bevy entity IDs, route/predecessor state, or semantic movement fields.
- `build_gpu_structural_upload_packet_from_scenario()` rejects invalid STEAD mapping, invalid graph links, and u32 count overflow (no silent truncation).
- `StudioGpuResidencyReadiness` now reports `structural_upload_packet_ready` and row counts. `AtlasRequired` remains valid-structure execution deferral, not packet invalidity.
- Table-setting for future GPU-resident runtime vertical-test work. No GPU kernels or runtime execution introduced.

### Evidence lifecycle status

GPU-STRUCTURAL-UPLOAD-PACKET-0 evidence is **PROBATION** pending DA approval.

## GPU-STRUCTURAL-RESIDENCY-0 — Structural packet GPU buffer residency

**Date / PR:** 2026-06-17 — GPU-STRUCTURAL-RESIDENCY-0

### What changed

- Added `simthing-gpu` structural upload module: `repr(C)` frame/location/link GPU rows, `COPY_DST | COPY_SRC` buffer allocation, and blocking readback proof.
- Mapeditor bridges `StudioGpuStructuralUploadPacket` to GPU rows via `to_structural_gpu_rows()`; `prove_gpu_buffer_residency_blocking()` uploads and verifies byte-stable readback.
- GPU buffers are projection/cache, not authority. Save/load remains `SimThingScenarioSpec`. Studio config remains presentation-only.
- `StudioGpuResidencyReadiness` now reports `gpu_buffer_residency_ready` separately from CPU packet readiness; defaults false until device upload proof runs.
- No WGSL, no runtime simulation, no RF/MF execution. Table-setting for future runtime vertical-test GPU execution.

### Evidence lifecycle status

GPU-STRUCTURAL-RESIDENCY-0 evidence is **PROBATION** pending DA approval.

## GPU-STRUCTURAL-VALIDATION-WGSL-0 — GPU validation over resident structural packet

**Date / PR:** 2026-06-17 — GPU-STRUCTURAL-VALIDATION-WGSL-0

### What changed

- Hardened structural upload readback to fallible `Result` with explicit `MapAsyncFailed` / `ReadbackFailed` errors.
- Added `structural_validation.wgsl` compute pass reading resident frame and link buffers, writing compact `StructuralValidationReportGpu`.
- Validates dense link endpoint bounds (`< location_count`) and self-links in parallel on GPU.
- Mapeditor exposes `prove_gpu_structural_validation_blocking()` for test/proof use only — no runtime UI wiring.
- GPU validation is projection/cache over scenario-derived packets, not model authority. Save/load remains `SimThingScenarioSpec`.
- No RF/MF execution, pathfinding, route/predecessor semantics, or runtime sim loop.
- GPU adapter evidence: **REAL_ADAPTER_OBSERVED** in validation environment.

### Evidence lifecycle status

GPU-STRUCTURAL-VALIDATION-WGSL-0 evidence is **PROBATION** pending DA approval.

## VERTICAL-TEST-SCENARIO-SEED-0 — Runtime vertical-test seed through Studio authority

**Date / PR:** 2026-06-17 — VERTICAL-TEST-SCENARIO-SEED-0

### What changed

- First concrete bridge toward loading/recreating the prior runtime vertical test in Studio.
- Minimal vertical-test-shaped seed encoded as `SimThingScenarioSpec` authority (`runtime_vertical_seed`): World root, map-container Location, two gridcell Locations with cohort payload children, canonical adjacency link `1→2`, structural placements on an 8×8 frame, provenance `VERTICAL-TEST-SCENARIO-SEED-0`.
- Fixture: `crates/simthing-mapeditor/tests/fixtures/runtime_vertical_seed.simthing-scenario.json`; builder: `runtime_vertical_seed_scenario_spec()` in `crates/simthing-mapeditor/src/runtime_vertical_seed.rs`.
- Studio loads the seed through existing scenario save/load IO (`load_scenario_authority_from_path`, `load_studio_session_from_scenario_path`) — not Studio config or Bevy state.
- Loaded seed rebuilds `StudioHydrationBoundary`, `StudioGalaxyViewModel`, `StudioStructuralProjection`, and `StudioGpuStructuralUploadPacket` from authority.
- Existing GPU structural validation WGSL validates the loaded seed on a real adapter (`invalid_link_endpoint_count = 0`, `self_link_count = 0`, `location_count = 2`, `link_count = 1`).
- Runtime execution, RF/MF execution, heatmap rendering, pathfinding, route/predecessor semantics, and live sim loop remain deferred.
- **GPU-LINK-ACCUMULATOR-SMOKE-0** now pulled by this seed (see below).

### Evidence lifecycle status

VERTICAL-TEST-SCENARIO-SEED-0 evidence is **PROBATION** pending DA approval. GPU adapter evidence: **REAL_ADAPTER_OBSERVED**.

## GPU-LINK-ACCUMULATOR-SMOKE-0 — First vertical-seed-pulled GPU accumulation over structural links

**Date / PR:** 2026-06-17 — GPU-LINK-ACCUMULATOR-SMOKE-0

### What changed

- First generic GPU neighbor accumulation over canonical structural links, pulled by `runtime_vertical_seed` — not abstract scaffolding.
- Bit-exact `i32` contract: for each link `(a,b)`, `output[a] += input[b]` and `output[b] += input[a]`; CPU oracle uses `checked_add`; overflow is rejected before dispatch; GPU `atomicAdd` runs only on inputs proven non-overflowing.
- CPU oracle `cpu_structural_link_accumulate_i32` compares values and little-endian bytes against GPU readback; vertical seed `input=[10,20]` yields `output=[20,10]`.
- Uses structural dense indices from `SimThingScenarioSpec`-derived `StudioGpuStructuralUploadPacket`; runs structural validation WGSL before accumulation.
- Mapeditor proof helpers: `prove_gpu_link_accumulator_smoke_blocking`, `prove_runtime_vertical_seed_gpu_link_accumulator_blocking` (test/proof only).
- Not pathfinding, Movement-Front, RF execution, heatmap rendering, or runtime simulation. No route/predecessor semantics introduced.
- GPU outputs are projection/cache — not persisted as save authority.
- Table-setting for future RF/Accumulator and runtime vertical-test GPU surfaces.

### Evidence lifecycle status

GPU-LINK-ACCUMULATOR-SMOKE-0 evidence is **PROBATION** pending DA approval. GPU adapter evidence: **REAL_ADAPTER_OBSERVED**.

## STUDIO-SCENARIO-NATIVE-FILEDIALOG-0 — Native scenario file picker

**Date / PR:** 2026-06-18 — STUDIO-SCENARIO-NATIVE-FILEDIALOG-0

### What changed

- Primary **Load Scenario...** button opens a native OS file dialog (`rfd`) filtered to `*.simthing-scenario.json`.
- Selected path populates the Studio scenario path text field as an absolute/canonical presentation path.
- **Manual Load Path** retains explicit load from the text field for diagnostics.
- `set_programmatic_scenario_path` allows agents/tests to populate the path field without opening the dialog.
- Scenario loading still uses existing `SimThingScenarioSpec` authority IO (`load_studio_session_from_scenario_path`).
- Cancel, invalid selection, and load failure preserve the current session; successful load rebuilds hydration/view projections.
- Studio config remains presentation-only; picker/path state is not model authority.
- Does not change GPU, runtime, RF/MF, or simulation execution.

### Evidence lifecycle status

STUDIO-SCENARIO-NATIVE-FILEDIALOG-0 evidence is **PROBATION** pending DA approval.

## STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0 — Canonical Scenario tree load/save/display

**Date / PR:** 2026-06-19 — #782 STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0 (`b4037fc9`)

### What changed

- Studio/mapeditor can now load canonical Scenario-root files (`scenarios/corpus/minimal_scenario_root.simthing-scenario.json`, `scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json`).
- Studio builds a `StudioScenarioDocument` view model for `Scenario → GameSession → Owner(s) + GalaxyMap → gridcell Locations`.
- Studio rebuilds the galaxy map projection from the canonical GalaxyMap child via existing spec helpers (`game_session_galaxy_map`, `resolve_map_container`, `spatial_authority_root`) — not legacy World-root assumptions.
- Inert and star-system gridcells are distinguishable in the Studio document (`StudioGridcellRole`).
- Studio can save/reload canonical Scenario authority after safe metadata edits (owner display name, GalaxyMap display name).
- Terran Pirate remains a **legacy lower-layer golden fixture** loaded through explicit `LegacyWorldRoot` compatibility routing.
- Studio remains editor/presentation only — no runtime tick ownership, no GPU dispatch, no StudioEngine/GalaxyMapEngine/OwnerEngine/FactionEngine/WorldEngine/ScenarioEngine.
- **SESSION-RESOURCE-FLOW-SILOS-0** remains deferred.

### Evidence lifecycle status

STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0 evidence is **PROBATION** pending DA approval.

## GENERAL-SCENARIO-INGESTION-ADMISSION-0 — arbitrary Scenario ingestion and typed admission

**Date / PR:** 2026-06-19 — #783 GENERAL-SCENARIO-INGESTION-ADMISSION-0 (`397a048d`)

### What changed

- The canonical scenario ontology is now loadable, displayable, and **ingestible** beyond minimal fixtures via `ingest_scenario` / `ingest_scenario_from_str` in **simthing-spec**.
- Ingestion classifies arbitrary Scenario files as **Admitted**, **PartiallyAdmitted**, **Rejected**, or **Unsupported** with typed validation errors and deferrals across Scenario → GameSession → Owner(s) → GalaxyMap → structural readiness.
- **simthing-driver** exposes `evaluate_scenario_compile_readiness` reusing existing structural N4 theater compile surfaces — no new GPU primitives.
- Legacy World-root Terran Pirate remains **compatibility-only** and **lower-layer golden fixture** (`LegacyWorldRootCompatibility` deferral).
- Driver/GPU-resident lower-layer compile assets are reused only after canonical authority admission.
- Studio remains editor/presentation; ingestion authority is spec/driver, not Studio.
- **SESSION-RESOURCE-FLOW-SILOS-0** and GPU-resident execution remain deferred unless existing driver compile readiness is proven without new GPU primitives.

### Evidence lifecycle status

GENERAL-SCENARIO-INGESTION-ADMISSION-0 evidence is **PROBATION** pending DA approval.

## SESSION-RESOURCE-FLOW-SILOS-0 — Owner stockpile reduce-up/disburse-down first slice

**Date / PR:** 2026-06-19 — #784 SESSION-RESOURCE-FLOW-SILOS-0 (`873692bb`)

### What changed

- Owner silos are generic properties on Owner SimThings (`owner_silo_marker`, `owner_silo_current`, `owner_silo_capacity`).
- Spatial participants reference owners by `owner_flow_owner_ref` properties/columns on gridcell children — not spatial parenting.
- Owners remain direct GameSession children and are not spatial parents.
- First slice admits deterministic reduce-up/disburse-down reporting via `evaluate_owner_silo_flow` (non-mutating oracle) and `OwnerSiloAdmissionReport` integrated into scenario ingestion (`ScenarioIngestionResult.owner_silo`).
- Admitted owner-silo scenarios with flow participants no longer receive blanket `OwnerResourceFlowNotYetExecuted` deferrals; silo-only placeholders without participants still defer.
- **simthing-driver** reuses existing `compile_resource_flow_admission`, `compile_and_materialize_resource_flow`, and `materialize_arena_registry` through `session_resource_flow_silos` with explicit participants only.
- No economy engine, faction engine, owner engine, or new GPU primitive exists.
- GPU-resident owner-silo tick execution is **PARTIAL/deferred** — generic admission/materialization over existing ResourceFlow surfaces is **PASS**; CPU oracle reports flow math.
- General scenario ingestion now classifies owner-resource-flow support rather than blanket deferring admitted flows.
- Canonical corpus fixtures: `scenarios/corpus/owner_silo_*.simthing-scenario.json`.
- Prerequisites: PRs #776–#783.

### Evidence lifecycle status

SESSION-RESOURCE-FLOW-SILOS-0 evidence is **PROBATION** pending DA approval.

### SESSION-RESOURCE-FLOW-SILOS-HARDEN-0 — malformed silo metadata rejection

**Date / PR:** 2026-06-19 — #785 SESSION-RESOURCE-FLOW-SILOS-HARDEN-0 (`faa84a67`)

- Malformed `owner_silo_current` / `owner_silo_capacity` values are rejected with `InvalidSiloAmount` — no silent `unwrap_or` defaults.
- Active admitted silo flow requires a valid `owner_silo_current` when `owner_silo_marker` is present; marker-only placeholders defer until participants exist.
- `current > capacity` is rejected as `InvalidSiloAmount`.
- Owner references remain property/column based; Owners are not spatial parents.
- Driver ResourceFlow materialization refuses rejected owner-silo admission (`build_owner_silo_resource_flow_spec` returns `None`).
- GPU owner-silo tick remains **PARTIAL/deferred**; no new GPU primitive/shader.
- Full `cargo test -p simthing-spec` re-run **PASS** on hardening validation (prior PR #784 link crash did not reproduce); see `docs/tests/session_resource_flow_silos_harden_0_results.md`.

## SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0 — resident GPU proof for admitted Owner silo flow

**Date / PR:** 2026-06-19 — SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0

### What changed

- Owner-silo flow remains generic **ResourceFlow/Accumulator** machinery — not an owner/faction/economy/silo engine.
- **simthing-driver** lowers admitted owner-silo participants into existing `CompiledAccumulatorOpPlan` surfaces via `compile_owner_silo_gpu_tick_plan` / `owner_silo_accumulator_compile`.
- **simthing-sim** owns resident GPU tick state (`SimGpuAccumulatorTickState`); **simthing-gpu** reuses existing `AccumulatorOpSession` / existing WGSL — no new shader files.
- Proof readback is scoped (`SimGpuReadbackPolicy::ProofReadback`); production `None` policy does not require readback.
- Scenario authority is **not** mutated by GPU proof outputs (projection/cache/evidence only).
- **GPU participant accumulation: PASS** — explicit participant surplus/deficit sums match CPU oracle on admitted corpus fixtures.
- **Full owner-silo state mutation: PARTIAL/deferred** — `evaluate_owner_silo_flow` remains semantic truth for reduce-up/disburse-down totals.
- Invalid silo amount rejects before GPU lowering (`compile_owner_silo_gpu_tick_plan` refuses rejected admission).
- Prerequisites: PRs #784, #785.

### Evidence lifecycle status

SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0 evidence is **PROBATION** pending DA approval.

## STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0 — Studio displays canonical ingestion/admission status

**Date / PR:** 2026-06-19 — STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0

### What changed

- Studio now displays ingestion/admission reports generated by **simthing-spec** authority via `StudioScenarioAdmissionSummary`.
- Studio calls `ingest_scenario` / `ingest_scenario_from_str` with the canonical profile — it does **not** own ingestion authority or duplicate validation traversal.
- Canonical valid, invalid, unsupported, and legacy World-root scenarios are visibly distinguished in admission summaries.
- Owner-silo admission totals, GPU participant accumulation readiness, and full state mutation deferral are visible presentation fields.
- Studio does **not** dispatch GPU, does **not** call `SimGpuAccumulatorTickState` or `compile_owner_silo_gpu_tick_plan`, and does **not** own sim tick lifecycle.
- Scenario authority is **not** mutated by admission report display.
- Terran Pirate remains **lower-layer golden fixture / legacy compatibility only**.
- Prerequisite: PR #786.

### Evidence lifecycle status

STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0 evidence is **PROBATION** pending DA approval.

## STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0 — canonical gridcell placement edits

**Date / PR:** 2026-06-19 — STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0

### What changed

- Studio can apply explicit structural placement edit commands to canonical Scenario authority via `studio_apply_structural_placement_command`.
- **simthing-spec** owns `StructuralPlacementCommand` / `apply_structural_placement_command`; Studio calls spec APIs rather than duplicating authority logic.
- Structural placement edits update GalaxyMap gridcell Location children and `structural_grid` placement mirrors consistently.
- Successful edits preserve STEAD mapping consistency (`validate_stead_mapping_consistency`) and canonical validation.
- Invalid edits are rejected without partial mutation (draft validate-then-swap).
- Studio rebuilds `StudioScenarioDocument`, structural projection, and admission summaries after successful edits.
- Save/reload roundtrip preserves edited structural authority.
- Driver/GPU-resident readiness is preserved through existing structural N4 compile surfaces (`evaluate_scenario_compile_readiness`, `compile_structural_n4_theater`).
- Studio does **not** dispatch GPU, does **not** call sim tick lifecycle, and does **not** mutate driver or GPU caches directly.
- GPU execution remains through existing driver/sim/gpu paths only.
- Terran Pirate remains **legacy lower-layer fixture**, not canonical edit authority.
- Prerequisite: PR #787.

### Evidence lifecycle status

STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0 evidence is **PROBATION** pending DA approval.

## PLANET-CHILD-LOCATION-ADMISSION-0 — planets as child Locations under star-system gridcells

**Date / PR:** 2026-06-19 — PLANET-CHILD-LOCATION-ADMISSION-0

### What changed

- Planets are admitted as `SimThingKind::Location` child nodes under star-system gridcell Locations (`GALAXY_CHILD_LOCATION_ROLE_PLANET` + `PLANET_ID_PROPERTY_ID`).
- Inert gridcells do not evaluate and cannot own planets; planets under inert gridcells are rejected.
- Planets are **not** structural grid placements and do not alter `structural_grid`.
- **simthing-spec** owns `evaluate_planet_child_locations` / `PlanetChildLocationCommand`; scenario ingestion reports planet child-location admission instead of blanket `PlanetsNotYetAdmitted` for valid planet nodes.
- Planet simulation/economy/population/cohort behavior remains deferred (`PlanetSimulationDeferred`).
- Studio displays planet child Locations in `StudioScenarioDocument.planets` as presentation over scenario authority.
- `studio_apply_planet_child_location_command` rebuilds document/projection/admission after edits.
- Driver structural N4 readiness remains gridcell-only; planet Locations are not structural placement participants.
- GPU-resident execution surfaces are unchanged; no new primitive/shader/WGSL.
- Terran Pirate remains **lower-layer golden fixture** only.
- Prerequisite: PR #788.

### Evidence lifecycle status

PLANET-CHILD-LOCATION-ADMISSION-0 evidence is **PROBATION** pending DA approval. Ontology remediated by PLANET-LOCAL-GRID-REMEDIATION-0 below.

## PLANET-LOCAL-GRID-REMEDIATION-0 — planets are star-system-local gridcell SimThings

**Date / PR:** 2026-06-19 — PLANET-LOCAL-GRID-REMEDIATION-0 (remediates PR #789)

### What changed

- PR #789’s planet child-location admission is remediated to preserve recursive SimThing grid doctrine.
- **GalaxyMap** arranges galactic gridcell Location SimThings via `structural_grid`.
- **Star-system gridcells** arrange their own child gridcell Location SimThings in a **local grid** (default **10×10**).
- **Planets** are non-inert **local gridcell** Location SimThings (`LOCAL_GRIDCELL_ROLE_PLANET` + `PLANET_ID_PROPERTY_ID` + local `col`/`row`).
- Planet gridcells may later own cohorts, ownership children, resource children, or other recursive SimThings.
- Planet gridcells are **not** GalaxyMap `structural_grid` placements.
- `validate_planet_child_locations` is fail-closed (`Err` when admission report has errors).
- **simthing-spec** owns `PlanetLocalGridCommand` / `evaluate_planet_child_locations`; ingestion reports `planet_gridcell_count` and typed deferrals (`PlanetSimulationDeferred`, not blanket `PlanetsNotYetAdmitted`).
- Studio displays planets as local gridcells under star-system gridcells (`local_col`/`local_row`, local frame size).
- Driver GalaxyMap structural N4 readiness remains gridcell-only; star-system local grid GPU operator deferred.
- No GPU primitive/shader/WGSL; no sim runtime tick ownership; Terran Pirate remains lower-layer golden fixture only.
- Prerequisite: PR #789 (ontology-remediated).

### Evidence lifecycle status

PLANET-LOCAL-GRID-REMEDIATION-0 evidence is **PROBATION** pending DA approval.

## RECURSIVE-SPATIAL-GRID-DEFAULTS-0 — universal 1×1 interior-grid defaults

**Date / PR:** 2026-06-19 — PR #791 — RECURSIVE-SPATIAL-GRID-DEFAULTS-0 (merge `1807faa9`; finalizes PR #790)

### What changed

- Every spatial gridcell Location SimThing has an interior child grid; default is **1×1** unless explicitly expanded.
- **Inert** galactic gridcells admit an implicit **1×1 receiver** local cell at (0,0) for falloff/heatmap/RF-readiness reporting.
- **Star-system** galactic gridcells default to **10×10** local grids (unchanged from #790).
- **Planet** local gridcells are non-inert `Location` SimThings under star-system grids with local col/row; interior default **1×1**.
- Planet gridcells may own non-grid children (cohort, fleet, infrastructure, leader) without local coordinates.
- Owner SimThings remain GameSession children / RF channel scopes — not spatial parents.
- Driver GalaxyMap structural N4 remains gridcell-only; star-system local-grid GPU operator deferred.
- No GPU primitive/shader/WGSL; no sim runtime changes.

### Evidence lifecycle status

RECURSIVE-SPATIAL-GRID-DEFAULTS-0 evidence is **PROBATION** pending DA approval.

## PLANET-NON-GRID-CHILD-ADMISSION-0 — non-grid children under planet gridcells

**Date / PR:** 2026-06-19 — PR #792 — PLANET-NON-GRID-CHILD-ADMISSION-0 (merge `6790907b`; builds on PR #791)

### What changed

- Planet gridcells admit non-grid children: `Cohort`, `Fleet`, `Station`, `Custom(Infrastructure|Leader)`.
- Non-grid children do not carry local gridcell col/row; coordinate metadata on non-grid children rejects.
- Non-grid children are not GalaxyMap `structural_grid` placements and may carry owner/channel metadata.
- `evaluate_planet_child_locations` reports `planet_non_grid_child_count` with typed simulation deferrals.
- Studio displays `planet_non_grid_children` on `StudioScenarioDocument`.
- Planet simulation/economy/population/combat execution remains deferred.

### Evidence lifecycle status

PLANET-NON-GRID-CHILD-ADMISSION-0 evidence is **PROBATION** pending DA approval.

## RF Proof Ladder — Production Synthesis Index (#795–#800)

The planet-child RF proof ladder is landed through six discoverable synthesis sections below. Each rung is proof/report-only; Scenario authority is not mutated; economy execution and local participant effect application remain deferred.

| Rung | PR | Synthesis section |
|------|-----|-------------------|
| PLANET-CHILD-RF-GPU-PARTICIPANT-0 | #795 | § PLANET-CHILD-RF-GPU-PARTICIPANT-0 |
| PLANET-CHILD-RF-REDUCE-UP-0 | #796 | § PLANET-CHILD-RF-REDUCE-UP-0 |
| OWNER-SILO-RUNTIME-WRITEBACK-0 | #797 | § OWNER-SILO-RUNTIME-WRITEBACK-0 |
| OWNER-SILO-DISBURSE-DOWN-0 | #798 | § OWNER-SILO-DISBURSE-DOWN-0 |
| RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 | #799 | § RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 |
| RUNTIME-RF-TICK-INTEGRATION-0 | #800 | § RUNTIME-RF-TICK-INTEGRATION-0 |

GPU proof across the ladder reuses existing AccumulatorOp surfaces stage-by-stage; no new GPU primitive or WGSL was introduced in #795–#800.

## PLANET-CHILD-RF-GPU-PARTICIPANT-0 — planet/non-grid child RF participants over existing AccumulatorOp

**Date / PR:** 2026-06-20 — PR #795 — PLANET-CHILD-RF-GPU-PARTICIPANT-0 (merge `6e00ca89`; builds on PR #792)

### What changed

- Planet gridcells and admitted non-grid child SimThings can contribute owner/channel-scoped RF participant inputs using metadata/properties/columns, not spatial owner-parenting.
- The participant model depends on the recursive local-grid doctrine: star-system gridcells own local grids, planets are local gridcell Location SimThings, and cohorts/fleets/infrastructure/leaders are non-grid child SimThings under planet gridcells.
- The driver lowers admitted participant surplus/deficit rows into existing AccumulatorOp plans.
- **simthing-spec** exports `evaluate_planet_child_rf_admission` / `planet_child_rf_participant_inputs` with typed classifications, deferrals, and fail-closed malformed-amount rejection.
- **simthing-driver** lowers admitted participant surplus/deficit rows into existing `CompiledAccumulatorOpPlan` surfaces via `compile_planet_child_rf_gpu_tick_plan` / `planet_child_rf_accumulator_compile`.
- **simthing-sim** / **simthing-gpu** reuse existing AccumulatorOp Sum-over-INPUT_LIST proof path — no new GPU primitive, WGSL, or shader.
- GPU output is proof/cache only and does not mutate scenario authority.
- Full owner-silo state mutation and economic disburse-down remain deferred.
- No new planet/economy/combat/orbit engine, route/pathfinding state, MapGenerator/ClauseThing changes, Terran Pirate fixture edits, or Studio GPU dispatch.

### Evidence lifecycle status

PLANET-CHILD-RF-GPU-PARTICIPANT-0 evidence is **PROBATION** pending DA approval.

## PLANET-CHILD-RF-REDUCE-UP-0 — scoped local RF reduce-up over planet participants

**Date / PR:** 2026-06-20 — PR #796 — PLANET-CHILD-RF-REDUCE-UP-0 (merge `c1eb325c`; builds on PR #795)

### What changed

- Planet gridcell and non-grid child RF participants reduce into scoped owner/resource/planet/star-system buckets.
- Owner/channel scope remains metadata/properties/columns, not spatial parentage.
- Participants in the same star system but different owners remain separate RF buckets.
- The CPU oracle computes `surplus_total`, `deficit_total`, `net_surplus`, and `net_deficit` per bucket.
- **simthing-spec** exports `evaluate_planet_child_rf_reduce_up` with per-bucket totals.
- **simthing-driver** / GPU proof reuses existing AccumulatorOp participant-sum surfaces via `compile_planet_child_rf_reduce_up_gpu_proof_plan`; no new GPU primitive or WGSL is introduced.
- Scenario authority is not mutated; full owner-silo state mutation and disburse-down writes remain deferred.
- Studio presentation of reduce-up buckets is deferred; spec/driver report exists.
- No planet/economy/combat/orbit engine, route/pathfinding state, new GPU primitive/WGSL, or Studio GPU dispatch.

### Evidence lifecycle status

PLANET-CHILD-RF-REDUCE-UP-0 evidence is **PROBATION** pending DA approval.

## OWNER-SILO-RUNTIME-WRITEBACK-0 — runtime owner-silo writeback from scoped reduce-up

**Date / PR:** 2026-06-20 — PR #797 — OWNER-SILO-RUNTIME-WRITEBACK-0 (merge `3508d578`; builds on PR #796)

### What changed

- Scoped planet child RF reduce-up buckets now feed a runtime owner-silo writeback preview/application path.
- Owner/channel scope remains metadata/properties/columns, not spatial parentage.
- Runtime writeback updates runtime-resident owner-silo current values only; Scenario authority is not mutated.
- **simthing-spec** exports `owner_silo_writeback_inputs_from_planet_child_reduce_up` and `apply_owner_silo_runtime_writeback_cpu` with checked arithmetic, no underflow, capacity clamp, unmet deficit recording, and deterministic owner/resource ordering.
- **simthing-driver** compiles `OwnerSiloRuntimeWritebackPlan` with per-owner/resource GPU aggregate AccumulatorOp proof plans.
- GPU proof covers owner/resource aggregate net sums; CPU oracle applies runtime writeback semantics.
- Disburse-down remains deferred. Studio presentation remains deferred.
- No new GPU primitive/WGSL, planet/economy/combat/orbit engine, route/pathfinding state, or Studio GPU dispatch.

### Evidence lifecycle status

OWNER-SILO-RUNTIME-WRITEBACK-0 evidence is **PROBATION** pending DA approval.

## OWNER-SILO-DISBURSE-DOWN-0 — runtime owner-silo local allocation preview

**Date / PR:** 2026-06-20 — PR #798 — OWNER-SILO-DISBURSE-DOWN-0 (merge `98395fbe`; builds on PR #797)

### What changed

- Runtime owner-silo writeback results now feed a disburse-down allocation oracle.
- Demand buckets are derived from owner/resource metadata on planet gridcells and admitted non-grid child SimThings.
- Owner/channel scope remains metadata/properties/columns, not spatial parentage.
- CPU oracle allocates available owner/resource current to scoped local demand buckets using deterministic priority ordering.
- Unmet demand and remaining owner-silo availability are recorded.
- Scenario authority is not mutated.
- Allocation application to planet/cohort/fleet state remains deferred.
- GPU proof, if present, reuses existing AccumulatorOp aggregate surfaces for demand totals and does not introduce new GPU primitives or WGSL.
- Full economic execution, cohort consumption, fleet supply, combat, movement, and Studio GPU dispatch remain deferred.

### Evidence lifecycle status

OWNER-SILO-DISBURSE-DOWN-0 evidence is **PROBATION** pending DA approval.

## RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 — runtime allocation state for local participants

**Date / PR:** 2026-06-20 — PR #799 — RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 (merge `1c01df47`; builds on PR #798)

### What changed

- Disburse-down allocation results now produce runtime-local participant allocation state.
- Each allocation records owner/resource/scope/source SimThing id, requested, allocated, unmet, and priority.
- Scenario authority is not mutated.
- No planet/cohort/fleet/infrastructure SimThing properties are changed.
- Economy execution, consumption, fleet supply, combat, movement, and savefile mutation remain deferred.
- GPU proof, if present, reuses existing AccumulatorOp aggregate surfaces for allocated totals and does not introduce new GPU primitives or WGSL.
- Studio presentation remains deferred unless implemented without GPU dispatch or sim tick.

### Evidence lifecycle status

RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 evidence is **PROBATION** pending DA approval.

## RUNTIME-RF-TICK-INTEGRATION-0 — composed runtime RF tick boundary report

**Date / PR:** 2026-06-20 — PR #800 — RUNTIME-RF-TICK-INTEGRATION-0 (merge `b497497b`; builds on PR #799)

### What changed

- The RF chain now composes participant admission, scoped reduce-up, owner-silo runtime writeback, owner-silo disburse-down, and runtime-local allocation application into a single deterministic runtime tick report.
- Scenario authority is not mutated.
- Economy execution, consumption, supply effects, combat, movement, savefile mutation, and local participant property changes remain deferred.
- GPU proof remains stage-local over existing AccumulatorOp surfaces; no new GPU primitive or WGSL is introduced.
- Studio presentation remains deferred unless implemented without GPU dispatch or sim tick.

### Evidence lifecycle status

RUNTIME-RF-TICK-INTEGRATION-0 evidence is **PROBATION** pending DA approval.

## RUNTIME-TICK-EXECUTION-SHELL-0 — deterministic runtime tick shell over composed RF reports

**Date / PR:** 2026-06-20 — PR #802 — RUNTIME-TICK-EXECUTION-SHELL-0 (merge `02550f2991b1fe9f1bf5082fd0802f99face5e9e`; builds on PR #800)

### What changed

- The runtime tick execution shell drives the composed `RuntimeRfTickPlan` through a deterministic scheduler/report boundary.
- The shell records tick id, stage ordering, readiness, stage-local GPU proof availability, RF totals, and deferred-effect flags.
- Scenario authority is not mutated.
- Economy execution, consumption, supply effects, combat, movement, savefile mutation, and local participant property changes remain deferred.
- GPU proof remains stage-local over existing AccumulatorOp surfaces; no new GPU primitive, WGSL, or fused tick kernel is introduced.
- Studio presentation remains deferred unless implemented without GPU dispatch or sim tick.

### Evidence lifecycle status

RUNTIME-TICK-EXECUTION-SHELL-0 evidence is **PROBATION** pending DA approval.

## LOCAL-PARTICIPANT-EFFECTS-0 — runtime participant effect previews under tick shell

**Date / PR:** 2026-06-20 — PR #803 — LOCAL-PARTICIPANT-EFFECTS-0 (merge `0b5b1791b00525ab07e2158af5c888e193d18b25`; builds on PR #802)

### What changed

- Local participant effects are now computed as runtime/proof-only previews from runtime-local allocation state under the tick shell.
- Each effect preview records source SimThing id, owner/resource/scope, requested, allocated, unmet, and satisfied/unsatisfied status.
- Scenario authority is not mutated.
- No participant SimThing properties are changed.
- Economy execution, consumption, fleet supply, combat, movement, savefile mutation, and Studio GPU dispatch remain deferred.
- GPU proof, if present, reuses existing AccumulatorOp aggregate surfaces for allocated/unmet totals and does not introduce new GPU primitives or WGSL.
- Studio presentation remains deferred unless implemented without GPU dispatch or sim tick.

### Evidence lifecycle status

LOCAL-PARTICIPANT-EFFECTS-0 evidence is **PROBATION** pending DA approval.

## RUNTIME-TICK-HISTORY-REPLAY-0 — deterministic tick history and replay evidence

**Date / PR:** 2026-06-20 — PR #804 — RUNTIME-TICK-HISTORY-REPLAY-0 (merge `d787a1c5782b47332f9c020c23b77fbb6982d047`; builds on PR #803)

### What changed

- Runtime tick history/replay evidence now records deterministic proof entries over the runtime tick shell and local participant effects.
- Each entry records Scenario authority digest, tick id, stage order, RF totals, local effect totals, deferred-effect flags, and deterministic entry digest.
- Replay evaluates the same Scenario authority and tick id repeatedly and verifies matching entry digests.
- Scenario authority is not mutated.
- No savefile, persistent timeline, participant SimThing property, economy, consumption, fleet supply, combat, movement, or Studio GPU dispatch is introduced.
- GPU proof remains stage-local over existing AccumulatorOp surfaces; no fused replay kernel, new GPU primitive, or WGSL is introduced.
- Studio presentation remains deferred unless implemented without GPU dispatch or sim tick.

### Evidence lifecycle status

RUNTIME-TICK-HISTORY-REPLAY-0 evidence is **PROBATION** pending DA approval.

## LOCAL-EFFECT-APPLICATION-AUTHORITY-0 — runtime local effect application authority boundary

**Date / PR:** 2026-06-20 — PR #805 — LOCAL-EFFECT-APPLICATION-AUTHORITY-0 (merge `c4d7273f0d1a008805da1df473db78287ea75715`; builds on PR #804)

### What changed

- Local effect application is now represented as a runtime/proof-only authority boundary.
- The application report converts local participant effect previews into deterministic runtime application records.
- Each record preserves source SimThing id, owner/resource/scope, requested, allocated, unmet, satisfied status, and runtime_applied_amount.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Savefiles and persistent timelines are not written.
- Semantic economy execution, consumption, fleet supply, combat, movement, and Studio GPU dispatch remain deferred.
- GPU proof, if present, reuses existing AccumulatorOp aggregate surfaces for runtime_applied/unmet totals and does not introduce new GPU primitives or WGSL.
- Studio presentation remains deferred unless implemented without GPU dispatch or sim tick.

### Evidence lifecycle status

LOCAL-EFFECT-APPLICATION-AUTHORITY-0 evidence is **PROBATION** pending DA approval.

## SEMANTIC-LOCAL-EFFECT-TYPES-0 — typed runtime semantic local effect outputs

**Date / PR:** 2026-06-20 — PR #806 — SEMANTIC-LOCAL-EFFECT-TYPES-0 (merge `51c9080de837553649915643be29807808e27b4c`; builds on PR #805)

### What changed

- Typed semantic local effect outputs are now defined as a runtime/proof-only boundary derived from local effect application records.
- Effect kinds include ResourceSatisfied, ResourceShortfall, and RuntimeAppliedAmount with deterministic amounts and ordering.
- RuntimeAppliedAmount records quantity; ResourceSatisfied records semantic satisfaction status; ResourceShortfall records unmet demand.
- Each output preserves source SimThing id, owner/resource/scope, effect kind, amount, and deferral flags.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Savefiles and persistent timelines are not written.
- Semantic effect execution, consumption, fleet supply, combat, movement, and Studio GPU dispatch remain deferred.
- GPU proof, if present, reuses existing AccumulatorOp aggregate surfaces for runtime_applied/shortfall totals and does not introduce new GPU primitives or WGSL.
- Studio presentation remains deferred unless implemented without GPU dispatch or sim tick.

### Evidence lifecycle status

SEMANTIC-LOCAL-EFFECT-TYPES-0 evidence is **PROBATION** pending DA approval.

## RECURSIVE-LOCAL-RF-EVALUATOR-0 — local gridcell evaluator nexus proof

**Date / PR:** 2026-06-20 — PR #807 — RECURSIVE-LOCAL-RF-EVALUATOR-0 (merge `b5e0b611487016f89400f091eb8a56af1eced4d2`; builds on PR #806)

### What changed

- The recursive local RF evaluator proves the core SimThing location-gridcell model: every spatial gridcell Location SimThing may act as a local RF evaluator nexus.
- Each Location arena gathers direct participant rows and child Location RF outputs, resolves local surplus against local demand by owner/resource, and emits only net surplus/deficit upward to its parent Location.
- Sibling redistribution is demonstrated: a parent Location uses one child Location's net surplus to satisfy another child Location's net deficit before bubbling only the remainder upward.
- The previous planet-child RF, owner-silo, runtime tick, local effect, and semantic effect proof ladder remains valid as a compatibility slice.
- Explicit `OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID` metadata is supported; missing resource key falls back to `"generic"`.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Semantic execution, savefile mutation, persistent timeline mutation, and Studio GPU dispatch remain deferred.
- GPU proof, if present, reuses existing AccumulatorOp aggregate surfaces for arena totals and does not introduce new GPU primitives or WGSL.

### Evidence lifecycle status

RECURSIVE-LOCAL-RF-EVALUATOR-0 evidence is **PROBATION** pending DA approval.

## RECURSIVE-LOCAL-RF-GPU-RESIDENCY-REMEDIATION-0R — aggregate proof includes child Location outputs and preserves GPU residency

**Date / PR:** 2026-06-20 — PR #808 — RECURSIVE-LOCAL-RF-GPU-RESIDENCY-REMEDIATION-0R (merge `b68bd43456df16d6ba64993ad71f94c05e821103`; builds on PR #807)

### What changed

- The recursive local RF evaluator remains a proof/oracle layer until integrated into a GPU-resident runtime path.
- This remediation reasserts the maximal GPU-residency doctrine: runtime RF aggregation should lower to flat GPU-compatible rows/tables, while CPU space remains limited to deterministic oracle validation, semantic-side bookkeeping, compile-plan construction, and owner/user-facing reports.
- The recursive RF aggregate proof now includes both direct participant rows and child Location outputs.
- This closes the proof gap between recursive settlement semantics and the AccumulatorOp aggregate proof surface: per-arena owner/resource surplus and demand totals now include direct participant surplus/demand plus child Location net surplus/deficit.
- The previous planet-child RF / owner-silo / tick / effect / semantic proof ladder remains preserved as a compatibility slice and is not replaced by this remediation.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Semantic execution, savefile mutation, persistent timeline mutation, tick-shell RF source replacement, and Studio GPU dispatch remain deferred.
- No new GPU primitive or WGSL is introduced.

### Deferred next rungs

1. Reconcile planet-child RF ladder with recursive local RF evaluator outputs.
2. Integrate recursive local RF evaluator into runtime tick shell as optional GPU-resident RF source.
3. Semantic effect execution authority remains deferred until recursive RF evaluator is integrated into tick shell.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive RF proof reports remains deferred.

### Evidence lifecycle status

RECURSIVE-LOCAL-RF-GPU-RESIDENCY-REMEDIATION-0R evidence is **PROBATION** pending DA approval.

## PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 — compatibility projection between legacy RF ladder and recursive RF evaluator

**Date / PR:** 2026-06-20 — PR #809 — PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 (merge `7ccd571f1dbab9f507037f55ad955420cddc1020`; builds on PR #808)

### What changed

- The planet-child RF ladder and recursive Location RF evaluator now have an explicit reconciliation/projection report.
- The previous planet-child RF / owner-silo / runtime tick / local effect / semantic effect ladder remains preserved as the current compatibility slice.
- The recursive evaluator remains a GPU-resident row/table target, with CPU limited to oracle/reference/shadow reconciliation, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- The reconciliation distinguishes participant-row compatibility from parent-level recursive net bubbling. Sibling redistribution may change parent-level net outputs while preserving the participant rows that feed the previous ladder.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Tick-shell RF source replacement, semantic execution, savefile mutation, persistent timeline mutation, and Studio GPU dispatch remain deferred.
- No new GPU primitive, WGSL, or fused recursive RF kernel is introduced.

### Deferred next rungs

1. Integrate recursive local RF evaluator into runtime tick shell as optional GPU-resident RF source.
2. Add side-by-side tick-shell reports comparing legacy RF source and recursive RF source.
3. Semantic effect execution authority remains deferred until recursive RF tick-shell source is proven.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive RF/reconciliation proof reports remains deferred.

### Evidence lifecycle status

PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 evidence is **PROBATION** pending DA approval.

## RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 — optional recursive RF source preview for tick shell

**Date / PR:** 2026-06-20 — PR #810 — RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 (merge `5d283140a104dd8955da2e3b3a379ef418b28c11`; builds on PR #809)

### What changed

- The runtime tick shell now has an optional side-by-side RF source comparison report.
- The legacy planet-child/owner-silo RF source remains the default runtime RF tick source.
- The recursive Location RF evaluator is available as a preview source and GPU-compatible row/table target, but it does not yet drive owner-silo disburse-down, local allocation, local effects, or semantic effects.
- The side-by-side report composes the legacy runtime RF tick plan, recursive local RF plan, and reconciliation report.
- CPU work remains oracle/reference/shadow comparison, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Tick-shell RF source replacement remains deferred.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Semantic execution, savefile mutation, persistent timeline mutation, and Studio GPU dispatch remain deferred.
- No new GPU primitive, WGSL, or fused recursive RF kernel is introduced.

### Deferred next rungs

1. Promote recursive RF source from preview to selectable tick-shell source behind explicit mode flag.
2. Add legacy-vs-recursive tick-shell equivalence gates for fixtures that should match.
3. Integrate recursive RF source into owner-silo/disburse-down only after selectable-source proof.
4. Semantic effect execution authority remains deferred until recursive RF tick-shell source is proven.
5. Runtime tick persistent history/replay storage remains deferred.
6. Star-system local-grid GPU operators remain deferred.
7. Fleet movement/combat remains deferred.
8. Studio presentation of recursive RF source comparison reports remains deferred.

### Evidence lifecycle status

RUNTIME-TICK-RECURSIVE-RF-SOURCE-0 evidence is **PROBATION** pending DA approval.

## RUNTIME-TICK-RECURSIVE-RF-SELECTABLE-SOURCE-0 — explicit selectable recursive RF report source for tick shell

**Date / PR:** 2026-06-20 — PR #811 — RUNTIME-TICK-RECURSIVE-RF-SELECTABLE-SOURCE-0 (merge `7deabc9daaa07db1abf728d21b0b6d42f9ef521e`; builds on PR #810)

### What changed

- Recursive RF is now explicitly selectable as the tick-shell RF report source behind `RuntimeRfTickSourceMode::RecursiveSelectable`.
- The legacy planet-child/owner-silo RF source remains the default runtime RF tick source.
- Reconciliation/equivalence selection gates control when recursive selection is allowed.
- Recursive selection is RF-report-only; owner-silo disburse-down, local allocation, local effects, and semantic effects remain deferred.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-compatible row/table target; no new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Semantic execution, savefile mutation, persistent timeline mutation, and Studio GPU dispatch remain deferred.

### Deferred next rungs

1. Integrate recursive RF selectable source into owner-silo/disburse-down behind explicit source mode.
2. Only after owner-silo integration, evaluate recursive RF local allocation/effect path.
3. Semantic effect execution authority remains deferred until recursive RF source is proven through owner-silo/allocation path.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of selectable RF source reports remains deferred.

### Evidence lifecycle status

RUNTIME-TICK-RECURSIVE-RF-SELECTABLE-SOURCE-0 evidence is **PROBATION** pending DA approval.

## OWNER-SILO-RECURSIVE-RF-SOURCE-0 — recursive RF drives owner-silo disburse-down behind explicit source mode

**Date / PR:** 2026-06-20 — PR #812 — OWNER-SILO-RECURSIVE-RF-SOURCE-0 (merge `807965d8f94e5a54085c9373f5802d9154850448`; builds on PR #811)

### What changed

- Recursive RF can now drive owner-silo/disburse-down proof reports behind an explicit RF source mode.
- The legacy planet-child/owner-silo RF source remains the default.
- This rung is not another comparison-only hygiene layer: it produces recursive-source owner-silo demand buckets and a recursive-source owner-silo disburse-down report.
- Recursive-source disburse-down does not yet feed local allocation, local effects, semantic effects, savefiles, or persistent history.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target; no new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.

### Deferred next rungs

1. Integrate recursive owner-silo disburse-down into local allocation behind explicit source mode.
2. Only after allocation integration, evaluate recursive RF local effect path.
3. Semantic effect execution authority remains deferred until recursive RF source is proven through allocation path.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive owner-silo source reports remains deferred.

### Evidence lifecycle status

OWNER-SILO-RECURSIVE-RF-SOURCE-0 evidence is **PROBATION** pending DA approval.

## LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 — recursive owner-silo disburse-down feeds local allocation behind explicit source mode

**Date / PR:** 2026-06-20 — PR #813 — LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 (merge `2a32494f747dd50becfea586eb7ba4d5f2335fbc`; builds on PR #812)

### What changed

- Recursive-source owner-silo/disburse-down can now feed runtime local allocation proof reports behind an explicit RF source mode.
- The legacy planet-child/owner-silo/local-allocation path remains the default.
- This rung is not another comparison-only hygiene layer: it produces a recursive-source local allocation report.
- Recursive-source local allocation does not yet feed local effects, semantic effects, savefiles, or persistent history.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target; no new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Semantic execution, savefile mutation, persistent timeline mutation, local effect integration, and Studio GPU dispatch remain deferred.

### Resource-key note

#812 normalizes recursive owner-silo buckets to `generic` for current writeback channel alignment. Typed recursive RF resource metadata remains present in recursive source rows, but typed owner-silo/local-allocation channels remain deferred until a later multi-resource channel rung.

### Deferred next rungs

1. Integrate recursive local allocation into local effect application behind explicit source mode.
2. Semantic effect execution authority remains deferred until recursive allocation path is proven through local effects.
3. Typed owner-silo resource channels must be restored before multi-resource economy semantics are authoritative.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive local allocation reports remains deferred.

### Evidence lifecycle status

LOCAL-ALLOCATION-RECURSIVE-RF-SOURCE-0 evidence is **PROBATION** pending DA approval.

## LOCAL-EFFECT-APPLICATION-RECURSIVE-RF-SOURCE-0 — recursive local allocation feeds local effects behind explicit source mode

**Date / PR:** 2026-06-20 — PR #814 — LOCAL-EFFECT-APPLICATION-RECURSIVE-RF-SOURCE-0 (merge `51fb53f6fe01bb3ecff187708b881f5aedc8423e`; builds on PR #813)

### What changed

- Recursive-source local allocation can now feed local participant effect previews and local effect application proof reports behind an explicit RF source mode.
- The legacy planet-child/owner-silo/local-allocation/local-effect path remains the default.
- This rung is not another comparison-only hygiene layer: it produces recursive-source local participant effect previews and recursive-source local effect application reports.
- Recursive-source local effect application does not yet feed semantic local effects, semantic execution, savefiles, or persistent history.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target; no new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Semantic execution, savefile mutation, persistent timeline mutation, semantic local effect integration, and Studio GPU dispatch remain deferred.

### Resource-key note

Current owner-silo/local-allocation writeback alignment still uses `generic`; typed recursive RF resource metadata remains present in recursive source rows, but typed local-effect/semantic resource channels remain deferred until a later multi-resource channel rung.

### Deferred next rungs

1. Feed recursive-source local effect application report into semantic local effects behind explicit source mode.
2. Only after semantic local effect projection, evaluate semantic effect execution authority.
3. Typed local-effect/semantic resource channels remain deferred.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive-source local effect reports remains deferred.

### Evidence lifecycle status

LOCAL-EFFECT-APPLICATION-RECURSIVE-RF-SOURCE-0 evidence is **PROBATION** pending DA approval.

## SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 — recursive local effects feed semantic local effects behind explicit source mode

**Date / PR:** 2026-06-20 — PR #816 — SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 (merge `148b3b03dbac65967d91551dd69295302ce63093`; builds on PR #814)

### What changed

- Recursive-source local effect application can now feed semantic local effect projection proof reports behind an explicit RF source mode.
- The legacy planet-child/owner-silo/local-allocation/local-effect/semantic projection path remains the default.
- This rung is not another comparison-only hygiene layer: it produces recursive-source semantic local effect projection reports.
- Recursive-source semantic local effect projection does not yet drive semantic execution, participant property mutation, savefiles, or persistent history.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target; no new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Scenario authority is not mutated.
- Participant SimThing properties are not mutated.
- Semantic execution, savefile mutation, persistent timeline mutation, and Studio GPU dispatch remain deferred.

### Resource-key note

Current owner-silo/local-allocation/local-effect writeback alignment still uses `generic`; typed recursive RF resource metadata remains present in recursive source rows, but typed semantic resource channels remain deferred until a later multi-resource channel rung.

### Deferred next rungs

1. Evaluate semantic effect execution authority behind explicit source mode.
2. Only after semantic execution gates pass, consider participant property mutation authority.
3. Typed semantic resource channels remain deferred.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive-source semantic local effect reports remains deferred.

### Evidence lifecycle status

SEMANTIC-LOCAL-EFFECTS-RECURSIVE-RF-SOURCE-0 evidence is **PROBATION** pending DA approval.

## SEMANTIC-EFFECT-EXECUTION-BOUNDARY-0 — recursive semantic effects produce runtime execution records without mutation

**Date / PR:** 2026-06-20 — PR #818 — SEMANTIC-EFFECT-EXECUTION-BOUNDARY-0 (merge `4ddb7b4a918c9e4502de4d12b5fe50784cb19bc3`; builds on PR #816)

### What changed

- Recursive-source semantic local effects can now be converted into deterministic runtime semantic execution records behind explicit RF source mode.
- The legacy semantic path remains default.
- This rung is not semantic state mutation. It proves execution boundary only.
- Execution records do not mutate participant SimThing properties, Scenario authority, savefiles, or persistent history.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target.
- No new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Typed semantic resource channels remain deferred.

### Resource-key note

Current owner-silo/local-allocation/local-effect writeback alignment still uses `generic`; typed recursive RF resource metadata remains present in recursive source rows, but typed semantic execution channels remain deferred until a later multi-resource channel rung.

### Deferred next rungs

1. Add runtime-only participant property delta previews for semantic execution records.
2. Prove property-delta previews without mutating participant SimThings.
3. Only after delta-preview authority, evaluate controlled runtime state mutation.
4. ScenarioSpec/savefile/persistent history remain deferred.
5. Typed semantic resource channels remain deferred.
6. Studio presentation remains deferred.

### Evidence lifecycle status

SEMANTIC-EFFECT-EXECUTION-BOUNDARY-0 evidence is **PROBATION** pending DA approval.

## SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 — semantic execution records produce runtime-only participant delta previews

**Date / PR:** 2026-06-20 — PR #820 — SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 (merge `d55775e19bec5a9b783b3469b39d36708098ae77`; builds on PR #818)

### What changed

- Recursive-source semantic execution records can now be converted into deterministic runtime-only participant property delta previews behind explicit source mode.
- The legacy semantic path remains default.
- This rung is not participant mutation. It previews the property deltas that would be applied by a later mutation authority rung.
- Delta preview records do not mutate participant SimThing properties, Scenario authority, savefiles, or persistent history.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target.
- No new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Typed semantic mutation channels remain deferred.

### Resource-key note

Current owner-silo/local-allocation/local-effect writeback alignment still uses `generic`; typed recursive RF resource metadata remains present in recursive source rows, but typed property mutation channels remain deferred until a later multi-resource channel rung.

### Deferred next rungs

1. Add controlled runtime-only participant state mutation from delta preview records.
2. Prove mutation is runtime-state-only and still does not mutate ScenarioSpec/savefile.
3. Only after runtime-state mutation proof, evaluate persistence/savefile boundary.
4. Typed semantic mutation channels remain deferred.
5. Studio presentation remains deferred.

### Evidence lifecycle status

SEMANTIC-PARTICIPANT-DELTA-PREVIEW-0 evidence is **PROBATION** pending DA approval.

## RUNTIME-PARTICIPANT-STATE-MUTATION-0 — semantic delta previews apply to runtime-only participant state rows

**Date / PR:** 2026-06-20 — PR #822 — RUNTIME-PARTICIPANT-STATE-MUTATION-0 (merge `4d0b1ed2722402ea138f980149b038cd4a9bb8b3`; builds on PR #820)

### What changed

- Recursive-source semantic delta preview records can now be applied to deterministic runtime-only participant state rows behind explicit source mode.
- The legacy semantic path remains default.
- This rung is runtime state mutation only. It does not mutate participant SimThing properties, Scenario authority, savefiles, or persistent history.
- Runtime state rows are report/table data and remain separate from ScenarioSpec authority.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target.
- No new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Typed semantic mutation channels remain deferred.

### Resource-key note

Current owner-silo/local-allocation/local-effect writeback alignment still uses `generic`; typed recursive RF resource metadata remains present in recursive source rows, but typed property mutation channels remain deferred until a later multi-resource channel rung.

### Deferred next rungs

1. Prove controlled participant SimThing property mutation boundary from runtime state rows, without ScenarioSpec/savefile persistence.
2. Only after property mutation boundary, evaluate savefile/persistent history boundary.
3. Typed semantic mutation channels remain deferred.
4. Studio presentation remains deferred.

### Evidence lifecycle status

RUNTIME-PARTICIPANT-STATE-MUTATION-0 evidence is **PROBATION** pending DA approval.

## RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 — runtime participant state rows apply to runtime property view

**Date / PR:** 2026-06-20 — PR #824 — RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 (merge `633fb0c06a6b4ed877d4dbb4b030c6ce9c17ade7`; builds on PR #822)

### What changed

- Recursive-source runtime participant state rows can now be applied to a deterministic runtime-only participant property view behind explicit source mode.
- The legacy semantic path remains default.
- This rung is a runtime property mutation boundary only. It does not mutate ScenarioSpec SimThing.properties, Scenario authority, savefiles, or persistent history.
- Runtime property view rows are report/table data and remain separate from ScenarioSpec authority.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target.
- No new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Typed semantic mutation channels remain deferred.

### Resource-key note

Current owner-silo/local-allocation/local-effect writeback alignment still uses `generic`; typed recursive RF resource metadata remains present in recursive source rows, but typed property mutation channels remain deferred until a later multi-resource channel rung.

### Deferred next rungs

1. Evaluate controlled ScenarioSpec property mutation authority separately.
2. Only after ScenarioSpec property mutation boundary, evaluate savefile/persistent history boundary.
3. Typed semantic mutation channels remain deferred.
4. Studio presentation remains deferred.

### Evidence lifecycle status

RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 evidence is **PROBATION** pending DA approval.

## SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 — runtime property view applies to cloned ScenarioSpec candidate

**Date / PR:** 2026-06-20 — PR #826 — SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 (merge `90de96070376cb86a12d78c80bb3c5857b2bae8f`; builds on PR #824)

### What changed

- Recursive-source runtime participant property-view rows can now be applied to a cloned ScenarioSpec candidate behind explicit source mode.
- The legacy semantic path remains default.
- This rung proves the Scenario property mutation authority boundary without mutating the input ScenarioSpec.
- Candidate ScenarioSpec properties may change inside the boundary report, but the input ScenarioSpec, savefiles, fixtures, and persistent history remain unchanged.
- CPU work remains oracle/reference/shadow selection, semantic-side bookkeeping, compile-plan construction, and owner/user-facing report formatting.
- Recursive RF remains a GPU-resident row/table target.
- No new GPU primitive, WGSL, or fused recursive RF kernel is introduced.
- Typed semantic mutation channels remain deferred.

### Resource-key note

Current owner-silo/local-allocation/local-effect writeback alignment still uses `generic`; typed recursive RF resource metadata remains present in recursive source rows, but typed property mutation channels remain deferred until a later multi-resource channel rung.

### Deferred next rungs

1. Produce comprehensive new-chat digest after this rung lands.
2. Then evaluate savefile/persistent history boundary for mutated candidate ScenarioSpec.
3. Typed semantic mutation channels remain deferred.
4. Studio presentation remains deferred.

### Evidence lifecycle status

SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 evidence is **PROBATION** pending DA approval.

## SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 — ScenarioSpec canonical import/export base

**Date / PR:** 2026-06-20 — PR #828 — SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 (merge `ee651acdf4b4a0e94b707300974d285ea2664754`)

### What changed

- This rung proves the headless ScenarioSpec load/save roundtrip required for Studio scenario import/export. It does not implement savefile persistence, runtime mutation, semantic execution, Studio UI, or Studio GPU dispatch. Canonical ScenarioSpec JSON is loaded, serialized, reloaded, and checked for stable Scenario authority digest.
- Added `load_scenario_spec_from_json_str`, `save_scenario_spec_to_canonical_json`, and `prove_scenario_canonical_load_save_roundtrip`.
- Driver `compile_scenario_canonical_io_plan_from_json_str` reports Studio import/export readiness with savefile and runtime mutation deferrals.

### Deferred next rungs

1. Wire Studio import/export UI to canonical I/O plan.
2. Evaluate savefile/persistent history boundary separately from ScenarioSpec canonical JSON.
3. Runtime mutation and semantic execution remain deferred.

### Evidence lifecycle status

SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 evidence is **PROBATION** pending DA approval.

## PRODUCTION-SYNTHESIS-RF-LADDER-0R — production doc coverage repair for RF ladder #795–#800

**Date / PR:** 2026-06-20 — PR #801 — PRODUCTION-SYNTHESIS-RF-LADDER-0R (merge `9a070aa6`; repairs synthesis for #795–#800)

### What changed

- Added RF Proof Ladder production synthesis index (#795–#800) for discoverability.
- Reaffirmed RF/location/owner-channel doctrine in the constitutional spine.
- Repaired stale deferred-work wording that broadly implied RF execution arenas were still absent.
- Aligned production doc section content with landed #795–#800 result reports.
- Documentation/evidence repair only; no product/runtime code changes.

### Evidence lifecycle status

PRODUCTION-SYNTHESIS-RF-LADDER-0R evidence is **PROBATION-DOC-REPAIR**; not DA-promoted.

## ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0 — Converging bit-exact GPU link smoke toward canonical execution

**Date / PR:** 2026-06-18 — ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0

### What changed

- Earlier `structural_link_accumulator` work was valid **PROBATION proof scaffolding only** — GPU residency and bit-exact i32 neighbor accumulation over canonical structural links.
- PR #756 fixed the bit-exact arithmetic contract: CPU `checked_add` oracle, overflow-as-error, and byte-for-byte GPU readback proof.
- The bespoke `structural_link_accumulator` is now explicitly fenced as **proof_only / smoke_only / not_runtime**; it must not mature into a second simulation/resource-flow engine under Studio.
- Direct migration to canonical **AccumulatorOp / AO-WGSL-0** is **not feasible in this PR**; an exact generic capability gap is documented in `docs/tests/accumulator_driver_sim_convergence_0_results.md`.
- Production runtime accumulation must converge on canonical AccumulatorOp / AO-WGSL-0 or an explicitly approved generic successor.
- Future “play it out” execution must route through **simthing-driver** (scenario/runtime compile or assembly into generic operations) and **simthing-sim** (tick, boundary lifecycle, simulation progression).
- **simthing-gpu** executes semantic-free generic ops requested by driver/sim — not Studio proof helpers per frame.
- Studio remains projection/presentation/proof harness: loads `SimThingScenarioSpec`, runs explicit proof helpers in tests only; no Bevy `Update` system calls accumulator smoke.
- The terran-pirate scenario is deferred until driver/sim assembly and canonical accumulation are ready.
- Big-endian/portable byte-proof hardening is deferred until after structural execution convergence.

### AccumulatorOp capability gap (neutral terms)

AccumulatorOp needs scenario-derived structural coupling rows:

- dense source index
- dense target index
- input scalar channel
- output scalar channel
- combine mode: checked exact sum
- optional mask/threshold fields deferred

Additional blockers for this PR: AccumulatorOp value grid is f32; PROBATION smoke is i32 bit-exact; no driver compile path from scenario links; no sim tick ownership.

### Evidence lifecycle status

ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0 evidence is **PARTIAL** pending DA approval. PR #756 bit-exact smoke tests preserved. GPU adapter evidence unchanged from prior PROBATION smoke runs.

## ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 — Sum-over-INPUT_LIST through canonical execution

**Date / PR:** 2026-06-18 — ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1

### What changed

- Implements the bounded **AccumulatorOp Sum-over-INPUT_LIST** extension in AO-WGSL-0 / CPU oracle — no new GPU primitive.
- Replaces documentation-as-code with behavioral evidence: deleted `accumulator_convergence.rs` and driver stub test.
- **simthing-driver** compiles `SimThingScenarioSpec` structural links into canonical AccumulatorOp input-list plans via `compile_structural_link_neighbor_sum_plan`.
- **simthing-sim** owns tick/boundary execution seam via `execute_accumulator_plan_tick_cpu`.
- Studio remains scenario loader / projection / proof harness — proof helpers for bespoke accumulator removed; no Bevy runtime dispatch.
- Bespoke `structural_link_accumulator` **deleted** after canonical proof (`runtime_vertical_seed` `[10,20]→[20,10]`).
- Exact **f32 integer range** contract (`|v| <= 2^24`) documented and tested for vertical-seed values.
- Future Gu-Yang and PALMA surfaces remain governed by STEAD §10 and must route to existing sanctioned operators — not implemented here.
- Big-endian portable byte-proof hardening remains deferred until after structural convergence.

### Canonical execution path (vertical seed)

```text
runtime_vertical_seed.simthing-scenario.json
  -> SimThingScenarioSpec authority
  -> simthing-driver compile_structural_link_neighbor_sum_plan
  -> AccumulatorOp Sum-over-INPUT_LIST plan
  -> simthing-sim execute_accumulator_plan_tick_cpu
  -> simthing-gpu AO-WGSL-0 / CPU oracle
  -> [10,20] -> [20,10]
```

### Evidence lifecycle status

ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 evidence is **PROBATION** pending DA approval. GPU adapter evidence: **REAL_ADAPTER_OBSERVED** for AccumulatorOp Sum-over-INPUT_LIST vertical seed. See `docs/tests/accumulator_driver_sim_convergence_1_results.md`. DA SPLIT ruling preserved in `docs/tests/accumulator_driver_sim_convergence_0_results.md`.

## SIM-GPU-ACCUMULATOR-BACKEND-0 — Sim-owned GPU backend for AccumulatorOp plans

**Date / PR:** 2026-06-18 — SIM-GPU-ACCUMULATOR-BACKEND-0

### What changed

- **simthing-sim** now owns both CPU oracle and GPU AccumulatorOp tick backends for driver-compiled plans.
- GPU backend reuses **simthing-gpu** `AccumulatorOpSession` / AO-WGSL-0 — no new GPU primitive or shader.
- `execute_accumulator_plan_tick_gpu` executes `CompiledAccumulatorOpPlan` with exact f32 integer input validation.
- `execute_accumulator_plan_tick_cpu` preserved as oracle/fallback; `AccumulatorTickBackend` selects backend.
- `runtime_vertical_seed` `[10,20]→[20,10]` proven through driver compile + **sim GPU tick**; CPU and GPU ticks match.
- Studio remains scenario loader / projection / proof harness — app/load paths do not construct `AccumulatorOpSession` or call sim GPU tick directly.
- Gu-Yang and PALMA remain deferred under STEAD §10.
- Big-endian portable byte-proof hardening remains deferred.

### Canonical GPU execution path (vertical seed)

```text
runtime_vertical_seed.simthing-scenario.json
  -> SimThingScenarioSpec authority
  -> simthing-driver compile_structural_link_neighbor_sum_plan
  -> CompiledAccumulatorOpPlan
  -> simthing-sim execute_accumulator_plan_tick_gpu
  -> simthing-gpu AccumulatorOpSession / AO-WGSL-0
  -> [10,20] -> [20,10]
```

### Evidence lifecycle status

SIM-GPU-ACCUMULATOR-BACKEND-0 evidence is **PROBATION** pending DA approval. GPU adapter evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/sim_gpu_accumulator_backend_0_results.md`.

## SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0 — Resident sim-owned GPU AccumulatorOp tick state

**Date / PR:** 2026-06-18 — SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0

### What changed

- **simthing-sim** now owns `SimGpuAccumulatorTickState` — resident GPU tick state for driver-compiled plans.
- `AccumulatorOpSession` / AO-WGSL-0 reused across ticks; ops uploaded once at initialization.
- `SimGpuReadbackPolicy` makes proof readback explicit; production tick (`None`) does not silently enable debug readback.
- Resident state ticks twice with different inputs (`[10,20]→[20,10]`, then `[30,40]→[40,30]`).
- CPU and GPU tick outputs match on exact f32 integer inputs.
- `execute_accumulator_plan_tick_gpu` retained as one-shot proof helper delegating to resident state.
- Studio app/load paths remain projection-only — no session construction or proof readback.
- Gu-Yang and PALMA remain deferred under STEAD §10.
- Big-endian portable byte-proof hardening remains deferred.
- Full validation sweep recorded; BACKEND-0 evidence amended for partial sweep correction.

### Resident GPU execution path (vertical seed)

```text
SimThingScenarioSpec authority
  -> simthing-driver compile_structural_link_neighbor_sum_plan
  -> CompiledAccumulatorOpPlan
  -> simthing-sim SimGpuAccumulatorTickState (session retained)
  -> tick with SimGpuReadbackPolicy
  -> simthing-gpu AccumulatorOpSession / AO-WGSL-0
  -> [10,20] -> [20,10]
```

### Evidence lifecycle status

SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0 evidence is **PROBATION** pending DA approval. GPU adapter evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/sim_gpu_resident_accumulator_tick_0_results.md`.

## SIM-GPU-READBACK-SCOPE-0 — Scoped proof readback for resident sim GPU ticks

**Date / PR:** 2026-06-18 — SIM-GPU-READBACK-SCOPE-0

### What changed

- Proof readback now scopes debug readback gating via `scoped_debug_readback_allowed` / `DebugReadbackGuard` and restores prior state on success, error, and panic (RAII).
- `debug_readback_allowed()` queries current gate state.
- Production resident GPU tick with `SimGpuReadbackPolicy::None` does not enable readback or call `readback_full`.
- One-shot proof helper remains proof/presentation-only with scoped readback.
- Resident sim GPU tick state remains the production-horizon execution shape.
- Studio app/load paths do not own dispatch or readback gating.
- Gu-Yang and PALMA remain deferred under STEAD §10.
- Big-endian portable byte-proof hardening remains deferred.

### Evidence lifecycle status

SIM-GPU-READBACK-SCOPE-0 evidence is **PROBATION** pending DA approval. See `docs/tests/sim_gpu_readback_scope_0_results.md`.

## TERRAN-PIRATE-SCENARIO-SKELETON-0 — Horizon scenario skeleton through sim-owned GPU tick

**Date / PR:** 2026-06-18 — TERRAN-PIRATE-SCENARIO-SKELETON-0

### What changed

- First terran-pirate **horizon skeleton** represented as `SimThingScenarioSpec` authority (`terran_pirate_skeleton`).
- Four gridcell Locations, three canonical hyperlane links with forked adjacency.
- Skeleton loads through Studio scenario IO and rebuilds hydration/view projection.
- **simthing-driver** compiles skeleton links into AccumulatorOp Sum-over-INPUT_LIST plan.
- **simthing-sim** resident GPU tick executes compiled plan; CPU and GPU outputs match on exact f32 inputs.
- Proof readback scoped via `DebugReadbackGuard`; panic unwind restoration tested.
- Studio remains loader/projection/proof harness — no runtime dispatch.
- Gu-Yang and PALMA remain deferred under STEAD §10.

### Canonical horizon execution path

```text
terran_pirate_skeleton.simthing-scenario.json
  -> SimThingScenarioSpec authority
  -> compile_structural_link_neighbor_sum_plan
  -> SimGpuAccumulatorTickState
  -> AccumulatorOp Sum-over-INPUT_LIST / AO-WGSL-0
  -> scoped ProofReadback
```

### Evidence lifecycle status

TERRAN-PIRATE-SCENARIO-SKELETON-0 evidence is **PROBATION** pending DA approval. GPU evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/terran_pirate_scenario_skeleton_0_results.md`.

## TERRAN-PIRATE-SCENARIO-SKELETON-0R — Authority/evidence hardening

**Date / PR:** 2026-06-18 — TERRAN-PIRATE-SCENARIO-SKELETON-0R

### What changed

- Terran Pirate skeleton authority is now a canonical `SimThingScenarioSpec` artifact at `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` outside Studio/editor test fixtures.
- Driver and sim proofs load scenario authority directly through `simthing-spec`, not `simthing-mapeditor` builders.
- Studio remains a scenario IO / projection consumer of the same artifact.
- **simthing-driver** compiles the skeleton into AccumulatorOp Sum-over-INPUT_LIST without Studio/Bevy dependency.
- **simthing-sim** ticks the compiled plan through resident GPU state without Studio/Bevy dependency.
- Mapeditor builder retained as convenience generator; equivalence proof against canonical artifact.
- PR #764 evidence repaired with exact validation command recording (see 0R results report).
- Gu-Yang and PALMA remain deferred under STEAD §10.
- No new GPU primitive, no new shader, no route/predecessor/pathfinding semantics.

### Canonical horizon execution path

```text
scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json
  -> SimThingScenarioSpec authority (simthing-spec deserialize)
  -> compile_structural_link_neighbor_sum_plan (simthing-driver)
  -> SimGpuAccumulatorTickState (simthing-sim)
  -> AccumulatorOp Sum-over-INPUT_LIST / AO-WGSL-0 (simthing-gpu)
  -> scoped ProofReadback
```

Studio load/projection reads the same artifact via scenario IO.

### Evidence lifecycle status

TERRAN-PIRATE-SCENARIO-SKELETON-0R evidence is **PROBATION** pending DA approval. GPU evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/terran_pirate_scenario_skeleton_0r_results.md`.

## SIMTHING-SIM-DEVDEP-SEAM-0 — sim-owned tick seam dependency cleanup

**Date / PR:** 2026-06-18 — SIMTHING-SIM-DEVDEP-SEAM-0

### What changed

- **simthing-sim** no longer dev-depends on `simthing-driver`, `simthing-mapeditor`, or `simthing-spec`.
- Sim-local tests use semantic-free hand-built CompiledAccumulatorOpPlan fixtures (`two_slot_vertical_input_list_plan`, `forked_four_slot_input_list_plan`).
- Terran Pirate scenario→driver→sim resident GPU proof moved to `simthing-driver/tests/terran_pirate_skeleton_resident_tick.rs`.
- `cargo test -p simthing-spec` passes fully, including `e10_does_not_import_arena_registry_into_simthing_sim`.
- Canonical scenario authority remains at `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json`.
- Gu-Yang and PALMA remain deferred under STEAD §10.
- No new GPU primitive, no new shader, no route/predecessor/pathfinding semantics.

### Restored execution seam

```text
SimThingScenarioSpec authority (integration proofs only, in driver)
  -> compile_structural_link_neighbor_sum_plan (simthing-driver)
  -> CompiledAccumulatorOpPlan (generic, semantic-free)
  -> SimGpuAccumulatorTickState tick (simthing-sim — no upward imports)
  -> AccumulatorOp / AO-WGSL-0 (simthing-gpu)
```

### Evidence lifecycle status

SIMTHING-SIM-DEVDEP-SEAM-0 evidence is **PROBATION** pending DA approval. See `docs/tests/simthing_sim_devdep_seam_0_results.md`.

## DRIVER-TEST-HARNESS-GREEN-0 — simthing-driver package test hygiene

**Date / PR:** 2026-06-18 — DRIVER-TEST-HARNESS-GREEN-0

### What changed

- The `bh3_authoring_installs_existing_operator` Cargo integration-test binary blocked `cargo test -p simthing-driver terran_pirate` on Windows (`os error 740` — UAC installer-name heuristic when `install` appears in the binary stem; not a Rust compile error).
- BH-3 install-bridge guard merged into `ct_bh3_closeout_sample_driver.rs`; standalone `install`-named binary removed.
- Pre-existing blocker `palma_path_5_install_session_property` renamed to `palma_path_5_session_property` (same UAC class).
- `cargo test -p simthing-driver terran_pirate` now runs honestly (PASS).
- Terran Pirate scenario→driver→sim resident GPU proof remains in `simthing-driver`.
- **simthing-sim** remains free of `simthing-driver` / `simthing-mapeditor` / `simthing-spec` upward dev-dependencies.
- `cargo test -p simthing-spec` passes fully, including e10.
- Gu-Yang and PALMA remain deferred under STEAD §10.
- No new GPU primitive, no new shader, no route/predecessor/pathfinding semantics.

### Evidence lifecycle status

DRIVER-TEST-HARNESS-GREEN-0 evidence is **PROBATION** pending DA approval. See `docs/tests/driver_test_harness_green_0_results.md`.

## TERRAN-PIRATE-MAPPING-FIRST-SLICE-0 — structural N4 Gu-Yang/PALMA GPU proof

**Date / PR:** 2026-06-18 — TERRAN-PIRATE-MAPPING-FIRST-SLICE-0

### What changed

- Canonical Terran Pirate `SimThingScenarioSpec` at `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` is deserialized in `simthing-driver` to derive structural grid placements and grid N4 adjacency from authoritative `(col,row)` — not hyperlane links, render coordinates, emission order, or row-major fill.
- Bounded first-slice theater uses the scenario `StructuralGridFrame` (`8×8`, four occupied cells).
- Hyperlane link adjacency remains separate: `AccumulatorOp` Sum-over-INPUT_LIST link-gather proof unchanged.
- Existing Gu-Yang/SaturatingFlux surface (`StructuredFieldStencilOp`) exercised with CPU/GPU parity on structural theater seeds.
- Existing PALMA surfaces (`WImpedanceComposeOp` + `MinPlusStencilOp`) exercised with CPU/GPU D-field parity; choke feedstock from SaturatingFlux projection column.
- Outputs are projection/cache only; scenario authority is not mutated.
- Studio remains loader/projection consumer; no runtime dispatch added.
- **simthing-sim** seam remains clean (no upward dev-dependencies).
- No new GPU primitive, shader, route, predecessor, pathfinding, or border/frontline service semantics.

### Restored / extended execution seam

```text
SimThingScenarioSpec authority
  -> structural N4 theater derivation (driver test)
  -> SaturatingFlux structured_field_stencil (Gu-Yang class)
  -> W-impedance compose + min-plus stencil (PALMA class)
  -> projection/cache outputs only

Hyperlane links (parallel, not conflated)
  -> compile_structural_link_neighbor_sum_plan
  -> AccumulatorOp Sum-over-INPUT_LIST
```

### Evidence lifecycle status

TERRAN-PIRATE-MAPPING-FIRST-SLICE-0 evidence is **PROBATION** pending DA approval. GPU evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/terran_pirate_mapping_first_slice_0_results.md`.

## STRUCTURAL-N4-THEATER-COMPILE-0 — driver structural theater admission surface

**Date / PR:** 2026-06-18 — STRUCTURAL-N4-THEATER-COMPILE-0

### What changed

- Driver now owns `compile_structural_n4_theater` — reusable structural N4 theater compile/admission from `SimThingScenarioSpec` authority.
- N4 adjacency derived from `structural_grid.placements (col,row)` only; hyperlane `scenario.links` ignored for theater geometry.
- Terran Pirate canonical scenario admits into 8×8 bounded theater (4 occupied cells, 3 N4 edges).
- Oversize frames (e.g. 11×11) return typed `AtlasDeferred` (`FrameExceedsStandardMaxGrid`); structural layout is not shrunk and dense-global fallback is not attempted.
- PR #768 mapping first-slice Gu-Yang/PALMA GPU parity tests refactored to use the driver compile surface.
- Hyperlane link gather remains separate via `compile_structural_link_neighbor_sum_plan` → AccumulatorOp Sum-over-INPUT_LIST.
- Outputs remain projection/cache only; scenario authority not mutated.
- Studio remains loader/projection consumer; simthing-sim seam remains clean.
- No new GPU primitive, shader, route, predecessor, pathfinding, or border/frontline service semantics.

### Evidence lifecycle status

STRUCTURAL-N4-THEATER-COMPILE-0 evidence is **PROBATION** pending DA approval. See `docs/tests/structural_n4_theater_compile_0_results.md`.

## SIM-MAPPING-PLAN-TICK-SEAM-0 — sim-owned resident mapping tick seam

**Date / PR:** 2026-06-18 — SIM-MAPPING-PLAN-TICK-SEAM-0

### What changed

- `simthing-sim` now owns resident tick lifecycle for already-compiled generic mapping plans via `SimGpuMappingTickState`.
- Mapping plan descriptors (`CompiledMappingPlan` / `CompiledMappingStep`) contain generic GPU operator configs only — no scenario/spec/driver/Studio semantics.
- Driver continues to compile canonical scenario authority into structural N4 theater admission and assemble generic mapping plans.
- Terran Pirate mapping first-slice proof now runs through driver→sim→gpu ownership (`terran_pirate_mapping_plan_tick.rs`).
- `SimGpuMappingReadbackPolicy::None` is readback-free; `ProofReadback` is explicit and scoped.
- Existing Gu-Yang/SaturatingFlux and PALMA/W-impedance/min-plus GPU parity remains green through the sim tick seam.
- Outputs remain projection/cache only; scenario authority not mutated.
- Studio remains loader/projection consumer; simthing-sim has no upward dev-dependencies.
- No new GPU primitive, shader, route, predecessor, pathfinding, or border/frontline service semantics.

### Evidence lifecycle status

SIM-MAPPING-PLAN-TICK-SEAM-0 evidence is **PROBATION** pending DA approval. GPU evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/sim_mapping_plan_tick_seam_0_results.md`.

## DRIVER-MAPPING-PLAN-COMPILE-0 — driver generic mapping-plan assembly

**Date / PR:** 2026-06-18 — DRIVER-MAPPING-PLAN-COMPILE-0

### What changed

- Driver now owns `compile_mapping_plan_from_admitted_theater` — reusable assembly from admitted structural N4 theater plus admitted mapping specs into generic `CompiledMappingPlan`.
- Structural theater admission (`compile_structural_n4_theater`) remains separate from mapping plan compile.
- `simthing-sim` continues to own resident tick lifecycle via `SimGpuMappingTickState`.
- Terran Pirate proof chain: canonical scenario → structural N4 theater → driver mapping plan compile → sim resident tick → existing GPU operators.
- Existing Gu-Yang/SaturatingFlux and PALMA/W-impedance/min-plus CPU/GPU parity remains green.
- `SimGpuMappingReadbackPolicy::None` remains readback-free; `ProofReadback` is explicit.
- Outputs remain projection/cache only; scenario authority not mutated.
- Studio remains loader/projection consumer; simthing-sim has no upward dev-dependencies.
- No new GPU primitive, shader, route, predecessor, pathfinding, or border/frontline service semantics.

### Evidence lifecycle status

DRIVER-MAPPING-PLAN-COMPILE-0 evidence is **PROBATION** pending DA approval. See `docs/tests/driver_mapping_plan_compile_0_results.md`.

## SIM-MAPPING-READBACK-POLICY-HARDEN-0 — mapping None/ProofReadback discipline

**Date / PR:** 2026-06-18 — SIM-MAPPING-READBACK-POLICY-HARDEN-0

### What changed

- Mapping `SimGpuMappingReadbackPolicy::None` is formally proven readback-free at the sim API boundary for structured-field, W-compose, and min-plus paths.
- Structured-field proof readback uses existing `scoped_debug_readback_allowed` RAII guard (accumulator precedent); W-compose has no readback API; min-plus uses explicit `GpuResident` vs `DiagnosticReadback` modes.
- Added ProofReadback→None and None→ProofReadback→None non-leak tests in sim and driver integration proofs.
- Resident mapping tick state reuse and readback policy discipline proven across mixed-policy tick sequences.
- Driver compile surface (`compile_mapping_plan_from_admitted_theater`) and sim resident tick seam (`SimGpuMappingTickState`) remain intact.
- Terran Pirate driver→sim→gpu proof remains green with extended readback sequencing.
- Studio remains loader/projection consumer; simthing-sim has no upward dev-dependencies.
- No new GPU primitive, shader, route, predecessor, pathfinding, or border/frontline service semantics.
- Atlas scheduling remains deferred.

### Evidence lifecycle status

SIM-MAPPING-READBACK-POLICY-HARDEN-0 evidence is **PROBATION** pending DA approval. GPU evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/sim_mapping_readback_policy_harden_0_results.md`.

## SIM-MAPPING-ATLAS-SCHEDULER-0 — sim-owned multi-theater mapping scheduler

**Date / PR:** 2026-06-18 — SIM-MAPPING-ATLAS-SCHEDULER-0

### What changed

- `simthing-sim` now owns `SimGpuMappingAtlasScheduler` — resident multi-theater scheduling over already-compiled generic `CompiledMappingPlan` batches.
- Scheduler constructs one `SimGpuMappingTickState` per theater slot at init and reuses resident operator state across ticks.
- Inputs are generic compiled plans and generic tick buffers only (`CompiledMappingAtlas`, `MappingAtlasTickInputs`); outputs are per-theater `SimGpuMappingTickOutput` in stable slot order.
- Driver continues to own scenario authority, structural theater admission, and mapping plan compile; sim schedules resident ticks only.
- Terran Pirate driver integration proof: canonical scenario → driver compile → sim atlas scheduler → existing GPU operators, plus synthetic second generic theater.
- `SimGpuMappingReadbackPolicy::None` remains readback-free across all scheduled theaters; `ProofReadback` is explicit and scoped per theater.
- Scenario authority is not mutated; Studio remains loader/projection consumer with no runtime dispatch.
- `simthing-sim` remains free of driver/spec/mapeditor upward dependencies.
- Atlas partition/admission for oversize scenarios remains deferred; scheduler accepts already-compiled plans only.
- No new GPU primitive, shader, route, predecessor, pathfinding, or border/frontline service semantics.

### Evidence lifecycle status

SIM-MAPPING-ATLAS-SCHEDULER-0 evidence is **PROBATION** pending DA approval. GPU evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/sim_mapping_atlas_scheduler_0_results.md`.

## DRIVER-STRUCTURAL-ATLAS-PARTITION-0 — driver partition admission for oversized structural theaters

**Date / PR:** 2026-06-18 — DRIVER-STRUCTURAL-ATLAS-PARTITION-0

### What changed

- Driver now owns `compile_structural_n4_atlas` — structural N4 atlas partition/admission for oversized frames.
- Terran Pirate 8×8 remains a single bounded theater via `StructuralAtlasAdmission::Single`.
- Synthetic oversize layouts (e.g. 11×11) partition into multiple bounded theaters without shrinking scenario authority.
- Original frame dimensions preserved in `CompiledStructuralN4Atlas` metadata; each partition carries `StructuralTheaterOrigin` and local coordinates.
- Cross-partition N4 edges are explicitly recorded as `DeferredCrossPartitionN4Edge` — halo exchange deferred for first slice.
- Partitioned theaters compile into generic mapping plans and feed the existing `SimGpuMappingAtlasScheduler`.
- `compile_structural_n4_theater` legacy `AtlasDeferred` behavior unchanged for callers not using atlas partition API.
- None/ProofReadback discipline preserved in partition scheduler integration proof.
- Studio remains loader/projection consumer; simthing-sim remains free of upward dependencies.
- No new GPU primitive, shader, route, predecessor, pathfinding, or border/frontline service semantics.

### Evidence lifecycle status

DRIVER-STRUCTURAL-ATLAS-PARTITION-0 evidence is **PROBATION** pending DA approval. GPU evidence: **REAL_ADAPTER_OBSERVED**. Cross-partition N4 halo exchange explicitly deferred. See `docs/tests/driver_structural_atlas_partition_0_results.md`.

## DRIVER-STRUCTURAL-ATLAS-HALO-0 — one-cell structural halo admission

**Date / PR:** 2026-06-19 — DRIVER-STRUCTURAL-ATLAS-HALO-0

### What changed

- Driver now admits one-cell structural N4 halo cells for partitioned atlas theaters when `include_overlap_halo: true`.
- Halo admission preserves original frame metadata and scenario authority; owned vs halo cells are explicitly distinguished via `StructuralTheaterHaloCell` and `StructuralTheaterCellRole`.
- Global coordinates remain recoverable from partition origin, coord padding, and local coordinates.
- Halo-disabled mode preserves PR #774 `deferred_cross_partition_edges` behavior.
- Halo-enabled mode retains deferred edge records as provenance and adds `halo_coverage` metadata.
- Halo-augmented theaters that exceed configured caps return `HaloExceedsTheaterCap` without silent shrink/drop.
- Driver compiles halo-augmented theaters into generic structured-field mapping plans for the sim atlas scheduler.
- `simthing-sim` scheduler remains generic; None/ProofReadback discipline preserved across scheduled halo theaters.
- Studio remains loader/projection consumer; no new GPU primitive, shader, route, pathfinding, or border/frontline semantics.

### Evidence lifecycle status

DRIVER-STRUCTURAL-ATLAS-HALO-0 evidence is **PROBATION** pending DA approval. GPU evidence: **REAL_ADAPTER_OBSERVED**. See `docs/tests/driver_structural_atlas_halo_0_results.md`.

## SCENARIO-SESSION-OWNER-ROOT-REVISED-0 — owner doctrine and proof-ladder demotion

**Date / PR:** 2026-06-19 — SCENARIO-SESSION-OWNER-ROOT-REVISED-0 (#776)

### What changed

- Owner entities documented as GameSession sibling children (not overlays); `SimThingKind::Owner` added with deprecated `Faction` legacy alias.
- Active constitution §0 uses Owner/owner-entity terminology; Terran Pirate / mapping / atlas PRs #764–#775 reclassified as lower-layer golden fixtures.
- Targeted e10 guards prevent owner-doctrine and hygiene-loop relapse.

## SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0 — Scenario SimThing file root

**Date / PR:** 2026-06-19 — SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0

### What changed

- `SimThingKind::Scenario` is the canonical serializable file root kind; scenario id/version/provenance metadata lives on Scenario-root properties.
- `scenario_id` / `provenance` sidecar fields remain transitional serde mirrors only.
- `validate_scenario_root_authority` requires Scenario root for canonical files; `validate_legacy_world_root_compatibility` admits legacy World-root fixtures (Terran Pirate) through a named compatibility path.
- Canonical corpus fixture: `scenarios/corpus/minimal_scenario_root.simthing-scenario.json`.
- GameSession child enforcement deferred to SCENARIO-GAMESESSION-CHILD-0; Studio IO preserves legacy Terran Pirate load path.
- SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0 remains PROBATION but is qualified by SCENARIO-METADATA-LOSSLESS-0 (see below).

## SCENARIO-METADATA-LOSSLESS-0 — lossless Scenario-root seed metadata

**Date / PR:** 2026-06-19 — SCENARIO-METADATA-LOSSLESS-0

### What changed

- SCENARIO-METADATA-LOSSLESS-0 fixed the canonical Scenario-root generator seed encoding so Scenario metadata roundtrips arbitrary u64 values exactly.
- Replaced lossy two-f32 32-bit-half encoding with four u16 chunks stored as exact f32 integers (0..=65535).
- Added roundtrip tests for `0`, `u64::MAX`, `0x8000_0000_0000_0001`, and `0x1234_5678_9ABC_DEF0`; malformed seed metadata is rejected.
- Sidecar `provenance.generator_seed` sync from root metadata remains exact; corpus fixture updated.
- SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0 remains PROBATION but is now qualified by this precision fix.
- GameSession child enforcement deferred to SCENARIO-GAMESESSION-CHILD-0 (now landed — see below).

## SCENARIO-GAMESESSION-CHILD-0 — GameSession under Scenario root

**Date / PR:** 2026-06-19 — SCENARIO-GAMESESSION-CHILD-0

### What changed

- Added `SimThingKind::GameSession` / `SimThingKindTag::GameSession` as the canonical running session root marker (not a runtime engine).
- Canonical Scenario roots now require exactly one direct `GameSession` child via `validate_scenario_game_session_child`.
- Legacy World-root fixtures (Terran Pirate) remain explicit compatibility only; they do not satisfy canonical GameSession validation.
- Minimal corpus fixture updated to `Scenario -> GameSession`; lossless Scenario-root seed metadata preserved.
- Prerequisites: SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0 (PR #777), SCENARIO-METADATA-LOSSLESS-0 (PR #778).
- Owners and GalaxyMap/WorldStateMap enforcement deferred to SESSION-OWNER-ENTITIES-0 (now landed — see below) and SESSION-GALAXYMAP-WORLDSTATE-0.

## SESSION-OWNER-ENTITIES-0 — Owner entities under GameSession

**Date / PR:** 2026-06-19 — SESSION-OWNER-ENTITIES-0

### What changed

- Canonical GameSession now requires at least one direct `Owner` child with unique non-empty `owner_id` metadata on Owner SimThing properties.
- Owner identity properties: `owner_id`, `owner_display_name`, `owner_archetype`; inert `owner_silo_marker` placeholder for future stockpile work.
- Optional `Custom("CapabilityTree")` child placeholder under Owner — no capability gameplay logic.
- Legacy `SimThingKind::Faction` accepted as deprecated Owner alias for serialized compatibility only.
- Minimal corpus fixture advanced to `Scenario -> GameSession -> Owner`.
- Prerequisites: PRs #776–#779. GalaxyMap deferred to SESSION-GALAXYMAP-WORLDSTATE-0; resource-flow silos deferred to SESSION-RESOURCE-FLOW-SILOS-0.

## DA / PROBATION / CURRENT_EVIDENCE Status

This synthesis is PROBATION. It summarizes the active track and branch evidence but does not claim DA approval. Promotion requires owner sign-off.

## Known Risks

- Scenario path field defaults to CWD-relative `simthing-current.simthing-scenario.json`; **Load Scenario...** now opens a native picker. Save Scenario dialog and platform app-data dirs remain deferred.
- Studio config path is CWD-only (`simthing-studio-config.json`); platform app-data dirs are deferred.
- Legacy RON `settings.ron` under AppData still persists window/generation path metadata separately from JSON presentation config.
- `map_container_id` is a SimThing raw-id string; logical display aliases like `studio_galaxy_map` are no longer accepted as authority bindings.
- Structural mirrors remain defensive f32 vectors, not exact typed properties.
- ClauseThing import into Studio authority is not wired yet.
- Dense galaxy-scale Movement-Front execution still requires atlas work.
- Runtime tick execution shell scheduler/report boundary is landed (RUNTIME-TICK-EXECUTION-SHELL-0); local participant effect previews are landed (LOCAL-PARTICIPANT-EFFECTS-0); deterministic tick history/replay evidence is landed (RUNTIME-TICK-HISTORY-REPLAY-0); local effect application authority boundary is landed (LOCAL-EFFECT-APPLICATION-AUTHORITY-0); typed semantic local effect outputs are landed (SEMANTIC-LOCAL-EFFECT-TYPES-0); recursive local RF evaluator nexus proof is landed (RECURSIVE-LOCAL-RF-EVALUATOR-0); semantic effect execution and economy execution remain deferred.
- Bespoke `structural_link_accumulator` deleted; RF/link coupling now routes through AccumulatorOp Sum-over-INPUT_LIST via driver compile + sim tick (PROBATION — see CONVERGENCE-1).
- Big-endian/portable byte-proof hardening deferred until after structural execution convergence.

## Deferred Work

The RF proof ladder now has participant admission, scoped reduce-up, runtime owner-silo writeback, disburse-down allocation, runtime-local allocation state, composed runtime RF tick reporting (#795–#800), a deterministic runtime tick execution shell (RUNTIME-TICK-EXECUTION-SHELL-0), runtime local participant effect previews (LOCAL-PARTICIPANT-EFFECTS-0), deterministic tick history/replay evidence (RUNTIME-TICK-HISTORY-REPLAY-0), a local effect application authority boundary (LOCAL-EFFECT-APPLICATION-AUTHORITY-0), typed semantic local effect outputs (SEMANTIC-LOCAL-EFFECT-TYPES-0), and a recursive local RF evaluator nexus proof (RECURSIVE-LOCAL-RF-EVALUATOR-0). Semantic effect execution, economy execution, consumption, fleet supply, combat, movement, savefile mutation, and Studio GPU dispatch remain deferred.

Other deferred work: native Save Scenario dialog, full runtime vertical-test execution, platform-specific config/scenario directories, new-map flow, full editor mutation command surface beyond the proof helper, heatmap rendering, pathfinding, route/predecessor semantics, movement-order logic, semantic WGSL, simulation GPU kernels, Clausewitz UI importer, CSS/WebView, exact typed property representation, new SimThing kinds, Gu-Yang falloff borders (STEAD §10 surface 2), PALMA reach field (STEAD §10 surface 3), big-endian canonical byte-proof helpers (`to_le_bytes` / `from_le_bytes`), and terran-pirate play-out.

## Next Production Rungs

1. Reconcile planet-child RF ladder with recursive local RF evaluator outputs.
2. Integrate recursive local RF evaluator into runtime tick shell as optional source of RF truth.
3. Semantic effect execution authority remains deferred until recursive RF evaluator is integrated into tick shell.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive RF proof reports remains deferred.
6. Broader scenario corpus ingestion UX / batch reports.
7. UI affordances for planet/structural editing if command layers remain headless.
8. Cohort/population/resource overlays under admitted planets.
