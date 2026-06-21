# 0.8.3 SimThing Studio Production

> Lifecycle: production synthesis. The **Scenario Runtime + Save/Load Closing Track is DA-APPROVED and closed** (PR #852/#853). This document is the authoritative Studio production design track and closing PR ladder — not a per-PR worklog.
>
> Detailed PR/worklog history: [`docs/workshop/studio_production_log.md`](workshop/studio_production_log.md)
>
> Per-rung validation evidence: `docs/tests/*_results.md` and [`docs/tests/current_evidence_index.md`](tests/current_evidence_index.md)

## Executive Production Status

SimThing Studio is an editor and presentation shell over **SimThing-Spec scenario authority**. MapGenerator and future ClauseThing imports produce or hydrate a `SimThingScenarioSpec`. Studio projects that authority into flat indexes, view models, render anchors, Bevy entities, dialogs, and camera state.

**Current baseline (2026-06-20):** the **Scenario Runtime + Save/Load Closing Track is DA-APPROVED and closed** (PR #852, provenance PR #853). The surface-gridcell HOLD from final DA review was resolved by **SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0** (PR #851). **The next production track is selected by the project owner** — future agents must not reopen the closed ladder unless the owner explicitly requests regression review or a new defect is found.

**Anti-hygiene rule:** Reject docs-only or comparison-only rungs unless they directly enable one of: Scenario load, validation, STEAD mapping, Studio projection rebuild, recursive spatial-tree RF runtime, candidate ScenarioSpec mutation, candidate save/reopen, Studio UI closure, or DA precheck readiness.

## SimThing Doctrine

- Everything is a **SimThing**; behavior is expressed as SimThings, properties, overlays, and admitted accumulator/resource-flow surfaces.
- After generation or import, the sole authoritative substrate is **`SimThingScenarioSpec`**: recursive `SimThing` root tree plus structural grid metadata, placements, links, and provenance.
- Studio projections, Bevy entities, GPU buffers, runtime reports, and editor config are **not** model authority.
- Model edits apply to scenario authority first; Studio rebuilds projection layers from authority.
- Terran Pirate is a **lower-layer golden fixture** only — not the canonical save-game tree shape.

## RF / Spatial-Tree Runtime Doctrine

```text
ScenarioSpec / serializable Scenario SimThing container
  -> GameSession SimThing
     -> Owner gridcells / Owner SimThings as GameSession children
     -> GalaxyGridcell SimThing
        -> child grid:
           -> StarMap gridcell SimThing
           -> inert 1×1 gridcell SimThings
              -> child grid:
                 -> Planet gridcell SimThing
                 -> inert 1×1 gridcell SimThings
                    -> child grid:
                       -> 1×1 surface gridcell SimThing
                          -> pop cohorts / fleets / buildings / infrastructure / leaders
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
- Each planet gridcell admits exactly one default 1×1 surface gridcell Location at (0,0) with role `surface`.
- Cohorts, fleets, infrastructure, leaders, and other admitted gameplay SimThings are children of the surface gridcell, not direct children of the planet gridcell.

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
| Whole `SimThingScenarioSpec` save/load authority model | **DA-APPROVED / closed** | SAVELOAD-AUTHORITY-PIN-0R (CAPABILITY-TREE-CLOSEOUT-0) |
| Backend `.simthing-scenario.json` file IO | **DA-APPROVED / closed** | SCENARIO-SAVELOAD-IO-0 (CAPABILITY-TREE-CLOSEOUT-0) |
| Studio Save Scenario / Load Scenario controls | **DA-APPROVED / closed** | SCENARIO-SAVELOAD-UI-0 (CAPABILITY-TREE-CLOSEOUT-0) |
| Scenario-native loaded `StudioSession` source | **DA-APPROVED / closed** | SCENARIO-NATIVE-SESSION-0 (CAPABILITY-TREE-CLOSEOUT-0) |
| Studio projections derived from ScenarioSpec authority | **DA-APPROVED / closed** | STUDIO-SIMTHING-SPEC-BOUNDARY-1 (CAPABILITY-TREE-CLOSEOUT-0) |
| General scenario ingestion / admission | **DA-APPROVED / closed** | GENERAL-SCENARIO-INGESTION-ADMISSION-0 (CAPABILITY-TREE-CLOSEOUT-0) |
| Owner silo reduce-up / disburse-down first slice | **DA-APPROVED / closed** | SESSION-RESOURCE-FLOW-SILOS-0 (CAPABILITY-TREE-CLOSEOUT-0) |
| Recursive spatial-tree RF proof ladder through runtime property-view boundary | **DA-APPROVED / closed** | #795–#826 ladder (CAPABILITY-TREE-CLOSEOUT-0) |
| **SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0** | **DA-APPROVED / closed (#828)** | Headless canonical JSON load → serialize → reload → stable authority digest |
| **SCENARIO-STEAD-MAP-ROUNDTRIP-0** | **DA-APPROVED / closed (#834)** | STEAD IDs, links, RF metadata, spatial tree survive canonical roundtrip; owner metadata distinct from spatial parentage |
| **LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0** | **DA-APPROVED / closed (#836)** | Loaded ScenarioSpec authority envelope for Studio; composes #828 + #834 readiness |
| **LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0** | **DA-APPROVED / closed (#838)** | Recursive Accumulator RF runtime surface for loaded ScenarioSpec spatial trees |
| **LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0** | **DA-APPROVED / closed (#840)** | Full recursive runtime report chain for loaded ScenarioSpec sessions |
| **SCENARIO-CANDIDATE-FROM-RUNTIME-0** | **DA-APPROVED / closed (#842)** | Candidate ScenarioSpec from loaded runtime property-view rows |
| **SCENARIO-CANDIDATE-SAVE-REOPEN-0** | **DA-APPROVED / closed (#844)** | Candidate ScenarioSpec canonical JSON save/reopen with stable authority digest |
| **SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0** | **DA-APPROVED / closed (#845)** | Pre-UI hardening of candidate ScenarioSpec canonical JSON writer |
| **STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0** | **DA-APPROVED / closed (#846)** | Studio UI workflow for loaded scenario runtime candidate save/reopen |
| **STUDIO-CANDIDATE-REOPEN-ADOPT-0** | **DA-APPROVED / closed (#847)** | Successful Reopen Candidate adopts candidate into Studio session |
| **SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0** | **DA-APPROVED / closed (#848)** | DA precheck consolidation for Scenario Runtime + Save/Load closing track |
| **SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0** | **DA-APPROVED / closed** | Remedial restoration of planet 1×1 surface gridcell tier (PR #851) |
| **SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0** | **DA-APPROVED / closed** | Final DA rerun; closing track graduated (PR #852/#853) |

## SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0 — planet surface gridcell tier remediation

This remedial pass restores the constitutionally required 1×1 surface gridcell tier below planet gridcells. Gameplay SimThings are no longer homed directly on the planet gridcell; cohorts, fleets, buildings, infrastructure, leaders, and other admitted gameplay children are children of the surface gridcell. Recursive Accumulator Flow settles first at the surface arena and then bubbles surface → planet → starmap/star-system → galaxy. Owner identity remains metadata/channel scope, not spatial parentage. No new savefile format, persistent history, GPU dispatch, pathfinding, combat, economy execution, or fleet movement is introduced.

This rung resolved the FINAL-DA-REVIEW-0 HOLD. The closing track graduated to DA-APPROVED via `SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0` (PR #852).

## SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 — DA precheck for Scenario Runtime + Save/Load closing track

This rung consolidates the Scenario Runtime + Save/Load Closing Track for human DA review. It verifies that canonical ScenarioSpec load/save/reopen, STEAD preservation, loaded session envelope, recursive RF runtime, full runtime report chain, candidate ScenarioSpec mutation, candidate save/reopen, hardened create-new candidate writing, Studio UI exposure, and reopened-candidate adoption are all present and validated. It does not promote the feature to DA by itself. Persistent history, replace-existing overwrite flow, GPU dispatch, combat, pathfinding, economy execution, and fleet movement remain deferred.

## STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 — Studio UI workflow for loaded scenario runtime candidate save/reopen

This rung exposes the loaded Scenario Runtime + Save/Load workflow in Studio UI. The UI shows loaded ScenarioSpec digest, STEAD/link/tree validation, recursive RF runtime readiness, report-chain readiness, candidate mutation availability, candidate digest, Save Candidate, and Reopen Candidate. Save Candidate uses the hardened create-new candidate writer and refuses to overwrite existing targets without a future explicit replace flow. Reopen Candidate loads canonical ScenarioSpec JSON and rebuilds validation/projection readiness. UI state, Bevy state, runtime reports, and GPU buffers remain non-authoritative. Persistent history and GPU dispatch remain deferred.

### STUDIO-CANDIDATE-REOPEN-ADOPT-0 — successful Reopen Candidate adopts candidate session

This remedial pre-DA pass completes the Reopen Candidate UI workflow by adopting the reopened candidate ScenarioSpec into the active Studio session after canonical load, STEAD validation, and projection-readiness checks pass. Failed reopen preserves the current session. UI state, Bevy state, runtime reports, and GPU buffers remain non-authoritative. Persistent history and GPU dispatch remain deferred.

### STUDIO-RUNTIME-SAVELOAD-STATUS-CACHE-0 — cached status refresh performance remediation

Runtime/candidate save-load status is a cached presentation surface. The DA-approved proof/report chain remains valid but must not be recomputed every Studio frame. Studio refreshes status on scenario load, candidate save, candidate reopen/adoption, explicit manual refresh, or authority change. ScenarioSpec remains authority; cached UI status remains non-authoritative.

### STUDIO-SETTINGS-PERFORMANCE-TELEMETRY-0 — live FPS and allocated VRAM estimate

The Settings window now includes a Performance section with live FPS and an allocated VRAM estimate in MB. The VRAM value is an in-app estimate of Studio-created GPU allocations rather than a driver-wide total unless otherwise stated. The telemetry is a presentation diagnostic surface only and does not affect ScenarioSpec authority, runtime RF semantics, save/load behavior, GPU dispatch, or DA-approved evidence lifecycle.

### STUDIO-RENDER-LOOP-DIRTY-GATE-0 — render-loop performance telemetry and dirty gating

The Studio render loop no longer rebuilds hyperlane meshes every frame when the camera/view/settings/session key is unchanged. Star visual material/depth updates are dirty-gated where safe. The Settings Performance section now exposes render-loop diagnostics for hyperlane rebuilds, star visual sync, billboard sync, picking projection, and VRAM scan timing. These are presentation diagnostics and render-cache behavior only; ScenarioSpec authority, save/load behavior, RF/Accumulator semantics, DA-approved evidence lifecycle, and GPU dispatch boundaries are unchanged.

### STUDIO-FRAME-PHASE-GPU-TELEMETRY-0 — frame-phase and GPU-context telemetry

The Settings Performance section now distinguishes FPS, tracked-asset VRAM estimate, build profile, GPU adapter/backend/present information, egui/UI timing, frame-phase timing, and render-loop diagnostics. The existing FPS collapse is not explained by currently instrumented hyperlane/star/picking systems alone; those account for less than 1 ms while the total frame time remains over 200 ms. This telemetry is diagnostic only and does not change ScenarioSpec authority, save/load behavior, RF/Accumulator semantics, GPU dispatch boundaries, or DA-approved lifecycle state.

## SCENARIO-CANDIDATE-SAVE-REOPEN-0 — save and reopen candidate ScenarioSpec

This rung saves the cloned candidate ScenarioSpec as canonical ScenarioSpec JSON, reopens it, validates STEAD IDs, links, RF metadata, and spatial tree shape, rebuilds projection readiness, and proves the candidate authority digest is stable after reopen. It does not introduce a distinct savefile format or persistent history. Studio UI wiring and GPU dispatch remain deferred.

### SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0 — pre-UI write hardening

This remedial hardening pass preserves Rung 6 while making the candidate ScenarioSpec canonical JSON writer safe for the upcoming Studio UI rung. The helper writes to a same-directory temp file and no longer removes an existing target before a successful write strategy is guaranteed. Candidate artifacts remain canonical ScenarioSpec JSON only; no distinct savefile format or persistent history is introduced.

## SCENARIO-CANDIDATE-FROM-RUNTIME-0 — candidate ScenarioSpec from loaded runtime property-view rows

This rung generates a cloned candidate ScenarioSpec from loaded-scenario runtime property-view rows. The loaded original ScenarioSpec remains unchanged and its authority digest is preserved. Runtime property-view rows provide GPU-compatible source data; candidate materialization is an authority-boundary serialization step, not a CPU production simulation engine. Candidate save, savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 — full recursive runtime report chain for loaded ScenarioSpec sessions

This rung attaches the existing recursive runtime report chain to loaded ScenarioSpec sessions. It composes the loaded Scenario session envelope, recursive Accumulator RF runtime surface, owner-silo/disburse-down reports, local allocation, local effects, semantic projection, semantic execution records, semantic participant delta previews, runtime participant state rows, and runtime participant property-view rows. The chain remains explicit runtime/report mode only. ScenarioSpec remains authority. CPU work is oracle/reference/shadow/report formatting only. Candidate ScenarioSpec mutation, savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 — recursive Accumulator RF runtime surface for loaded ScenarioSpec trees

This rung attaches recursive Accumulator RF runtime reporting to loaded ScenarioSpec spatial trees. It walks the loaded ScenarioSpec Location hierarchy, resolves RF locally at each parent Location gridcell node, settles sibling surplus/deficit before upward bubbling, and emits owner/resource/scope keyed GPU-compatible row/table surfaces. ScenarioSpec remains authority. CPU work is oracle/reference/shadow/report formatting only. Runtime mutation, semantic execution, savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 — loaded ScenarioSpec authority envelope for Studio

This rung defines the loaded Scenario Studio session envelope around ScenarioSpec authority. It composes canonical IO and STEAD map roundtrip readiness, reports scenario import/export eligibility, Studio projection rebuild readiness, recursive RF prerequisites, and runtime sidecar availability. Studio config, Bevy ECS state, GPU buffers, and runtime reports are explicitly non-authoritative. Runtime tick execution, runtime mutation, semantic execution, savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred.

## SCENARIO-STEAD-MAP-ROUNDTRIP-0 — STEAD IDs, links, RF metadata, and spatial tree survive ScenarioSpec roundtrip

This rung proves that ScenarioSpec canonical load/save/reload preserves stable SimThing IDs, link integrity, ownership metadata, RF metadata, and spatial tree shape. Owner metadata remains distinct from spatial parentage. Recursive RF prerequisites are preserved: parent Location arenas are discoverable, spatial gridcell Locations retain interior grids, and RF channel metadata remains available for owner/resource/scope keyed resolution. Runtime mutation, savefile persistence, semantic execution, Studio UI wiring, and GPU dispatch remain deferred.

## Post-DA status — Scenario Runtime + Save/Load closed

The Scenario Runtime + Save/Load Closing Track is **DA-APPROVED and closed** by `SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0` / PR #852, with provenance fill in PR #853. The prior final-review HOLD was resolved by `SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0` / PR #851, which restored the required planet 1×1 surface gridcell tier. Future agents must not reopen this ladder unless the project owner explicitly requests regression review or a new defect is found. **The next production track is selected by the project owner.**

### Current Studio capability baseline

The Studio can:

- load canonical ScenarioSpec JSON
- validate STEAD IDs, links, RF metadata, and spatial tree shape
- preserve the corrected spatial gridcell hierarchy through load/save/reopen
- expose loaded ScenarioSpec digest and readiness status
- build a loaded scenario session envelope
- report recursive Accumulator/RF runtime readiness
- produce runtime report-chain rows
- clone a candidate ScenarioSpec from runtime property-view rows
- save candidate ScenarioSpec as canonical ScenarioSpec JSON using hardened create-new writer
- reopen candidate ScenarioSpec
- adopt reopened candidate ScenarioSpec into active Studio session

**Constitutional baseline:** ScenarioSpec remains the only serialized scenario authority. Studio UI state, Bevy ECS state, runtime reports, property-view rows, and GPU buffers remain non-authoritative. Accumulator Flow remains GPU-resident in shape through row/table surfaces. CPU work remains oracle/reference/serialization/file-IO/validation/proof/report formatting only.

**Approved deferrals:** replace-existing candidate save / overwrite confirmation; persistent history / timeline; GPU dispatch / WGSL implementation; pathfinding; combat; economy execution; fleet movement / supply; non-canonical savefile format.

## STUDIO-CAPABILITY-TREE-PROBATION-CLOSEOUT-0 — parent capability rows closed after DA-approved save/load track

This closeout closes the parent capability-tree rows that are now covered by the DA-approved Scenario Runtime + Save/Load closing track. The closeout does not implement new runtime behavior. It records that ScenarioSpec save/load authority, backend scenario file IO, Studio save/load controls, scenario-native loaded sessions, Studio projections from ScenarioSpec authority, general scenario ingestion/admission, owner-silo RF, and the recursive spatial-tree RF/property-view ladder are current evidence under the DA-approved child ladder. ScenarioSpec remains authority; Studio UI, Bevy state, runtime reports, property-view rows, and GPU buffers remain non-authoritative. The corrected planet surface gridcell tier and Accumulator Flow GPU-residency shape remain the baseline. Replace-existing save, persistent history, GPU dispatch, pathfinding, combat, economy execution, and fleet movement remain deferred.

The recursive RF / property-view ladder is closed as evidence for GPU-compatible Accumulator Flow row/table shape and authority boundaries, not as evidence of final GPU dispatch.

## Scenario Runtime + Save/Load Closing Track

**Status: DA-APPROVED and closed.** This closing track completed Studio Scenario import/load/save/runtime closure. It is not a shutdown track and it is not a hygiene loop. All rungs 0–8 plus remediation and final DA rerun are DONE. Do not reopen unless the project owner requests regression review.

| Rung | PR / Track ID | Purpose | Completed? | Evidence / Notes | Next dependency |
|---:|---|---|---|---|---|
| 0 | SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 | Headless canonical ScenarioSpec load/save/reload with stable authority digest. | DONE | Landed in PR #828. Canonical JSON, load, save, reload, digest proof. | STEAD map roundtrip |
| 1 | SCENARIO-STEAD-MAP-ROUNDTRIP-0 | Prove STEAD IDs, links, ownership metadata, RF metadata, and spatial tree shape survive load/save/reload. | DONE | Reuses #828 canonical IO. STEAD/tree/RF metadata roundtrip report; obsolete `docs/0.8.3 Simthing Studio Production.md` alias retired. | Studio session envelope |
| 2 | LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 | Define loaded ScenarioSpec authority envelope for Studio: digest, validation, projection readiness, RF readiness, save/export eligibility. | DONE | Composes #828 canonical IO + #834 STEAD roundtrip; authority/runtime sidecar envelopes; non-authority surfaces explicit. | Recursive RF runtime |
| 3 | LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 | Attach recursive Accumulator RF runtime to loaded ScenarioSpec spatial trees. | DONE | Composes #836 session envelope; parent-arena/participant/channel GPU-compatible rows; local settlement before upward bubbling. | Runtime report chain |
| 4 | LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 | Attach recursive RF → owner-silo → allocation → effects → semantic → execution → delta → runtime state → property view chain to loaded scenarios. | DONE | Composes #838 recursive RF runtime + prior landed runtime ladder into loaded-scenario chain report. | Candidate ScenarioSpec |
| 5 | SCENARIO-CANDIDATE-FROM-RUNTIME-0 | Generate mutated candidate ScenarioSpec from runtime property-view rows while original loaded ScenarioSpec remains unchanged. | DONE | Composes #840 report chain + landed property-mutation boundary; clone applies preview rows to candidate only. | Candidate save/reopen |
| 6 | SCENARIO-CANDIDATE-SAVE-REOPEN-0 | Save candidate ScenarioSpec, reopen it, validate STEAD/tree/projection, and prove digest stability. | DONE | Composes #842 candidate path + #828 canonical IO; #845 pre-UI write hardening. | Studio UI |
| 7 | STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 | Expose load, validation, recursive runtime readiness, candidate mutation, save candidate, and reopen candidate in Studio UI. | DONE | Studio UI adapter + #847 adoption fix; create-new Save Candidate; reopened-candidate adoption. | DA precheck |
| 8 | SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 | Consolidate evidence for DA review of the full Scenario Runtime + Save/Load feature. | DONE | Evidence matrix; regression validation; READY FOR HUMAN DA REVIEW recommendation. | Human DA review |

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

Scenario Runtime + Save/Load closure is complete. The following remain deferred for future tracks:

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

## SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-0 — final DA review and lifecycle graduation

**Decision: HOLD / NOT READY FOR DA PROMOTION.** The final maximum-reasoning DA review pass
(`docs/tests/scenario_runtime_saveload_final_da_review_0_results.md`) reviewed the Scenario Runtime +
Save/Load Closing Track across code, tests, evidence, Studio workflow, STEAD preservation, the corrected
spatial gridcell hierarchy, Accumulator Flow / GPU-residency doctrine, and authority boundaries. ScenarioSpec
authority, owner-as-metadata (non-spatial), the RF settle-then-bubble shape, canonical-JSON candidate
save/reopen, and non-authority surfaces are all sound. **One track-wide constitutional blocker prevents
promotion.**

**Blocking issue — spatial gridcell hierarchy collapse.** The mandated `Planet gridcell → child grid → 1×1
surface gridcell → {cohort / fleet / infrastructure / leader}` is not implemented: gameplay SimThings are
homed **directly on the planet gridcell**, and the planet's Location child (the surface gridcell) is actively
deferred (`DeepPlanetChildDeferred`). The 1×1 surface gridcell tier is absent from code
(`planet_child_location.rs`), this production doc's hierarchy block (above), the closing-track tests, and the
corpus fixture. No DA-approved Deviation Record exists; `invariants.md` §"No silent tier collapse" forbids
promotion.

**Lifecycle:** All closing-track rows remain **PROBATION**. No row advances to `CURRENT_EVIDENCE` /
`DA-APPROVED`. Rungs 0–8 prior status is unchanged; SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-0 is recorded as
**HELD**.

**Remedial track landed:** `SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0` restores the surface gridcell tier in
code, tests, fixture, and this doc. **HOLD remains** until `SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-0` is
re-run and evaluates the remediated hierarchy. *(Superseded — see the rerun below; HOLD is now lifted.)*

## SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0 — final DA review rerun after surface gridcell remediation

**Decision: DA-APPROVED.** This final rerun reviewed the Scenario Runtime + Save/Load Closing Track after
SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0 (PR #851) restored the required planet 1×1 surface gridcell tier.
Code, tests, fixtures, evidence, Studio workflow, STEAD preservation, the corrected spatial gridcell
hierarchy, Accumulator Flow / GPU-residency doctrine, and authority boundaries were reviewed. The track is
DA-approved. ScenarioSpec remains authority; Studio UI, Bevy state, runtime reports, property-view rows, and
GPU buffers remain non-authoritative. Candidate save/reopen uses canonical ScenarioSpec JSON only.
Replace-existing candidate save, persistent history, GPU dispatch, pathfinding, combat, economy execution, and
fleet movement remain deferred.

**Blocker resolution:** the planet 1×1 surface gridcell tier is restored and **required** — planets admit
exactly one surface gridcell at (0,0); gameplay SimThings are children of the surface gridcell; direct
gameplay under a planet is rejected (`PlanetDirectGameplayChildRequiresSurfaceGridcell`); missing/duplicate/
off-(0,0) surface is rejected; Accumulator Flow settles at the surface arena and bubbles surface→planet→star→
galaxy (proven non-vacuously); STEAD roundtrip, candidate-from-runtime, candidate-save-reopen, Studio UI
status, and reopen-adopt each preserve the surface tier under test. No silent tier collapse remains.

**Ladder:** Rungs 0–8 remain DONE. SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0 remains DONE.
SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0 is DONE after merge. **The Scenario Runtime + Save/Load
Closing Track lifecycle is DA-APPROVED (closed).** Next track is selected by the project owner.

Full review: `docs/tests/scenario_runtime_saveload_final_da_review_rerun_0_results.md`.