# 0.8.3 SimThing Studio Production

> Lifecycle: **PROBATION** production synthesis until DA approval. This document is the authoritative Studio production design track and closing PR ladder — not a per-PR worklog.
>
> Detailed PR/worklog history: [`docs/workshop/studio_production_log.md`](workshop/studio_production_log.md)
>
> Per-rung validation evidence: `docs/tests/*_results.md` and [`docs/tests/current_evidence_index.md`](tests/current_evidence_index.md)

## Executive Production Status

SimThing Studio is an editor and presentation shell over **SimThing-Spec scenario authority**. MapGenerator and future ClauseThing imports produce or hydrate a `SimThingScenarioSpec`. Studio projects that authority into flat indexes, view models, render anchors, Bevy entities, dialogs, and camera state.

**Current baseline (2026-06-20):** headless canonical ScenarioSpec load/save roundtrip is **landed** (PR #828, PROBATION). Prior doc-cleanup **STUDIO-PRODUCTION-LADDER-CLOSURE-0R** (PR #830) trimmed verbose worklog material from the legacy synthesis doc. The active product track is the finite **Scenario Runtime + Save/Load Closing Track** below — not another hygiene or comparison loop.

**Anti-hygiene rule:** Reject docs-only or comparison-only rungs unless they directly enable one of: Scenario load, validation, STEAD mapping, Studio projection rebuild, recursive spatial-tree RF runtime, candidate ScenarioSpec mutation, candidate save/reopen, Studio UI closure, or DA precheck readiness.

## SimThing Doctrine

- Everything is a **SimThing**; behavior is expressed as SimThings, properties, overlays, and admitted accumulator/resource-flow surfaces.
- After generation or import, the sole authoritative substrate is **`SimThingScenarioSpec`**: recursive `SimThing` root tree plus structural grid metadata, placements, links, and provenance.
- Studio projections, Bevy entities, GPU buffers, runtime reports, and editor config are **not** model authority.
- Model edits apply to scenario authority first; Studio rebuilds projection layers from authority.
- Terran Pirate is a **lower-layer golden fixture** only — not the canonical save-game tree shape.

## RF / Spatial-Tree Runtime Doctrine

```text
ScenarioSpec is the serializable authority.

ScenarioSpec
  -> Scenario root
     -> GameSession
        -> Owner SimThings
        -> GalaxyMap / WorldStateMap Location
           -> galactic gridcell Location SimThings
              -> interior local grid
                 -> star-system local gridcell Location SimThings
                    -> planet/starbase/orbital Location gridcell SimThings
                       -> cohorts / fleets / infrastructure / leaders as child SimThings
```

**Spatial invariants:**

- Everything is a SimThing.
- ScenarioSpec is the save/load authority.
- GameSession contains Owner SimThings and the root spatial Location.
- Owners are GameSession children, not spatial parents.
- Owner/RF identity is metadata/channel scope, not containment.
- Ownership changes update metadata/properties/columns, never spatial parentage.
- GalaxyMap / WorldStateMap is the root spatial Location.
- Every spatial gridcell is a Location SimThing.
- Every spatial gridcell has an interior local grid.
- Default interior grid is 1×1 unless expanded.
- Star-system galactic gridcells use 10×10 local grids.
- Inert gridcells still use receiver grids for falloff, emanation, and heatmap values.
- Planet/starbase/orbital bodies are local-grid Location cells inside star-system local grids.
- Cohorts, fleets, infrastructure, and leaders are child SimThings under spatial Location cells unless later promoted to spatial containers.

**RF / Accumulator doctrine:**

- RF is generic flow accumulation over SimThings.
- RF resolves locally at each parent Location gridcell node.
- sibling surplus/deficit settles within the parent Location arena before net surplus/deficit bubbles upward.
- Upward RF reduction is by owner/resource/scope.
- Owner/RF channels are metadata arenas/scopes, not spatial containment.
- AccumulatorOp / GPU rows are execution/proof/cache surfaces.
- ScenarioSpec remains the authority.
- CPU may perform oracle/reference/shadow/bookkeeping/report formatting.
- CPU must not become production simulation authority.

## STEAD Mapping Doctrine

- Scenario load must validate stable IDs and links.
- ID reservation must prevent collisions.
- Studio projection rows must derive from ScenarioSpec authority.
- Studio view models and Bevy state are not authority.
- Save/load must preserve stable Scenario identity, SimThing identity, link integrity, ownership metadata, RF metadata, and spatial tree shape.

## Current Landed Capability Summary

| Capability | Status | Notes |
|---|---|---|
| Whole `SimThingScenarioSpec` save/load authority model | Landed / PROBATION | SAVELOAD-AUTHORITY-PIN-0R |
| Backend `.simthing-scenario.json` file IO | Landed / PROBATION | SCENARIO-SAVELOAD-IO-0 |
| Studio Save Scenario / Load Scenario controls | Landed / PROBATION | SCENARIO-SAVELOAD-UI-0 |
| Scenario-native loaded `StudioSession` source | Landed / PROBATION | SCENARIO-NATIVE-SESSION-0 |
| Studio projections derived from ScenarioSpec authority | Landed / PROBATION | STUDIO-SIMTHING-SPEC-BOUNDARY-1 |
| General scenario ingestion / admission | Landed / PROBATION | GENERAL-SCENARIO-INGESTION-ADMISSION-0 |
| Owner silo reduce-up / disburse-down first slice | Landed / PROBATION | SESSION-RESOURCE-FLOW-SILOS-0 |
| Recursive spatial-tree RF proof ladder through runtime property-view boundary | Landed / PROBATION | #795–#826 ladder |
| **SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0** | **Landed / PROBATION (#828)** | Headless canonical JSON load → serialize → reload → stable authority digest |
| **SCENARIO-STEAD-MAP-ROUNDTRIP-0** | **Landed / PROBATION (#834)** | STEAD IDs, links, RF metadata, spatial tree survive canonical roundtrip; owner metadata distinct from spatial parentage |
| **LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0** | **Landed / PROBATION** | Loaded ScenarioSpec authority envelope for Studio; composes #828 + #834 readiness |

## LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 — loaded ScenarioSpec authority envelope for Studio

This rung defines the loaded Scenario Studio session envelope around ScenarioSpec authority. It composes canonical IO and STEAD map roundtrip readiness, reports scenario import/export eligibility, Studio projection rebuild readiness, recursive RF prerequisites, and runtime sidecar availability. Studio config, Bevy ECS state, GPU buffers, and runtime reports are explicitly non-authoritative. Runtime tick execution, runtime mutation, semantic execution, savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## SCENARIO-STEAD-MAP-ROUNDTRIP-0 — STEAD IDs, links, RF metadata, and spatial tree survive ScenarioSpec roundtrip

This rung proves that ScenarioSpec canonical load/save/reload preserves stable SimThing IDs, link integrity, ownership metadata, RF metadata, and spatial tree shape. Owner metadata remains distinct from spatial parentage. Recursive RF prerequisites are preserved: parent Location arenas are discoverable, spatial gridcell Locations retain interior grids, and RF channel metadata remains available for owner/resource/scope keyed resolution. Runtime mutation, savefile persistence, semantic execution, Studio UI wiring, and GPU dispatch remain deferred.

## Scenario Runtime + Save/Load Closing Track

This closing track completes Studio Scenario import/load/save/runtime closure. It is not a shutdown track and it is not a hygiene loop. Each rung must move one of the following forward: Scenario load, Scenario validation, STEAD mapping, Studio projection rebuild, recursive spatial-tree RF runtime, candidate ScenarioSpec mutation, candidate save/reopen, or Studio-visible save/load workflow.

| Rung | PR / Track ID | Purpose | Completed? | Evidence / Notes | Next dependency |
|---:|---|---|---|---|---|
| 0 | SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 | Headless canonical ScenarioSpec load/save/reload with stable authority digest. | DONE | Landed in PR #828. Canonical JSON, load, save, reload, digest proof. | STEAD map roundtrip |
| 1 | SCENARIO-STEAD-MAP-ROUNDTRIP-0 | Prove STEAD IDs, links, ownership metadata, RF metadata, and spatial tree shape survive load/save/reload. | DONE | Reuses #828 canonical IO. STEAD/tree/RF metadata roundtrip report; obsolete `docs/0.8.3 Simthing Studio Production.md` alias retired. | Studio session envelope |
| 2 | LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 | Define loaded ScenarioSpec authority envelope for Studio: digest, validation, projection readiness, RF readiness, save/export eligibility. | DONE | Composes #828 canonical IO + #834 STEAD roundtrip; authority/runtime sidecar envelopes; non-authority surfaces explicit. | Recursive RF runtime |
| 3 | LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 | Attach recursive Accumulator RF runtime to loaded ScenarioSpec spatial trees. | NEXT | RF resolves locally at each parent Location node; sibling settlement before upward bubbling. | Runtime report chain |
| 4 | LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 | Attach recursive RF → owner-silo → allocation → effects → semantic → execution → delta → runtime state → property view chain to loaded scenarios. | PENDING | Explicit runtime/report mode only. No hidden mutation on load. | Candidate ScenarioSpec |
| 5 | SCENARIO-CANDIDATE-FROM-RUNTIME-0 | Generate mutated candidate ScenarioSpec from runtime property-view rows while original loaded ScenarioSpec remains unchanged. | PENDING | Candidate digest changes; original digest stable. No save write yet. | Candidate save/reopen |
| 6 | SCENARIO-CANDIDATE-SAVE-REOPEN-0 | Save candidate ScenarioSpec, reopen it, validate STEAD/tree/projection, and prove digest stability. | PENDING | Atomic write path. Runtime reports rebuild from reopened ScenarioSpec, not persisted as authority. | Studio UI |
| 7 | STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 | Expose load, validation, recursive runtime readiness, candidate mutation, save candidate, and reopen candidate in Studio UI. | PENDING | UI state is not authority. Failed load/save preserves current session. | DA precheck |
| 8 | SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 | Consolidate evidence for DA review of the full Scenario Runtime + Save/Load feature. | DEFERRED | Review-only consolidation after rungs 1–7. Not another implementation rung. | Human DA decision |

### Rung 1 — SCENARIO-STEAD-MAP-ROUNDTRIP-0

Loaded and re-saved ScenarioSpec must preserve stable SimThing IDs, link integrity, ID reservation, parent/child tree shape, spatial parentage, ownership metadata, RF metadata, and Studio projection inputs. Owner metadata must remain distinct from spatial parentage. This rung must expose recursive RF prerequisites: parent Location arenas, spatial gridcell interior grids, and owner/resource/scope channel metadata.

### Rung 2 — LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0

Studio needs a loaded Scenario session envelope around ScenarioSpec authority: authority digest, validation status, projection readiness, recursive RF readiness, save/export eligibility, and runtime sidecars. The envelope must not make Studio config, Bevy entities, GPU buffers, or runtime reports authoritative.

### Rung 3 — LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0

Loaded scenarios must compile recursive spatial-tree RF reports from the actual Location hierarchy. RF resolves locally at each parent Location gridcell node. Sibling surplus/deficit settles within that parent arena before unresolved net surplus/deficit bubbles upward by owner/resource/scope.

### Rung 4 — LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0

Loaded ScenarioSpec must feed the recursive runtime report chain: recursive RF, owner-silo, local allocation, local effects, semantic projection, execution records, delta previews, runtime state rows, and runtime property view. This remains explicit report/runtime mode only, not hidden mutation on load.

### Rung 5 — SCENARIO-CANDIDATE-FROM-RUNTIME-0

Runtime property-view rows must produce a mutated candidate ScenarioSpec. The loaded original ScenarioSpec must remain unchanged. Candidate mutation records must preserve participant id, owner_ref, resource_key, scope, property id, and before/runtime/after values.

### Rung 6 — SCENARIO-CANDIDATE-SAVE-REOPEN-0

The candidate ScenarioSpec must save through canonical serialization and atomic temp-to-rename write, reopen, validate STEAD IDs/links/tree shape, rebuild Studio projections, and prove candidate digest stability. Runtime reports must rebuild from reopened ScenarioSpec rather than being persisted as authority.

### Rung 7 — STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0

Studio must visibly expose loaded ScenarioSpec digest, STEAD/link/tree validation status, recursive RF runtime readiness, candidate mutation availability, Save Candidate, and Reopen Candidate. UI state, Bevy entities, and runtime caches remain non-authoritative.

## Deferred Post-Closure Tracks

Deferred until after Scenario Runtime + Save/Load closure:

- typed semantic mutation channels
- persistent timeline/history model beyond candidate save/reopen
- combat
- movement/pathfinding/route predecessor state
- full economy execution
- fleet supply
- Studio GPU dispatch
- DA promotion itself

## DA Promotion Criteria

Promotion from PROBATION to CURRENT_EVIDENCE requires:

1. Closing track Rungs 1–7 complete (or explicit waiver with owner sign-off).
2. DA precheck consolidation submitted if requested.
3. No open authority-boundary regressions on ScenarioSpec load/save, STEAD mapping, or spatial-tree RF doctrine.
4. Owner sign-off on evidence index rows and synthesis doc.

## Known Risks (Concise)

- Scenario path defaults to CWD-relative file; native Save Scenario dialog and platform app-data dirs remain deferred.
- `map_container_id` is a SimThing raw-id string binding.
- Structural mirrors remain defensive f32 vectors.
- Dense galaxy-scale Movement-Front execution still requires atlas work.