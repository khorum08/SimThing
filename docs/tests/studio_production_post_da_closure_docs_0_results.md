# STUDIO-PRODUCTION-POST-DA-CLOSURE-DOCS-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `studio-production-post-da-closure-docs-0`
- PR: #854
- Merge SHA: `7a049827b943071fc1abc22113415608478ed5b8`

## Mission

Post-DA documentation closure for the Scenario Runtime + Save/Load Closing Track. Record DA approval (#852/#853), surface-gridcell remediation (#851), current Studio capability baseline, corrected spatial hierarchy, approved deferrals, and next-track selection state so future agents do not reopen the closed ladder.

## Documents updated

- `docs/design_0_0_8_3_studio_production.md` — Post-DA closure section, executive status, capability baseline
- `docs/workshop/studio_production_log.md` — concise closure entry
- `docs/tests/current_evidence_index.md` — docs-only closure row
- `docs/tests/studio_production_post_da_closure_docs_0_results.md` — this report

## Scenario Runtime + Save/Load closure summary

The Scenario Runtime + Save/Load Closing Track is **DA-APPROVED and closed** by `SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0` / PR #852, with provenance fill in PR #853. The prior final-review HOLD was resolved by `SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0` / PR #851, which restored the required planet 1×1 surface gridcell tier. Future agents must not reopen this ladder unless the project owner explicitly requests regression review or a new defect is found.

## Current Studio capability baseline

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

## Corrected spatial hierarchy baseline

```text
Serializable Scenario SimThing container
  -> GameSession SimThing
     -> Owner gridcells / Owner SimThings as GameSession children
     -> GalaxyGridcell SimThing
        -> child grid containing:
           -> StarMap gridcell SimThing
           -> inert 1×1 gridcell SimThings in the same galaxy grid
              -> child grid containing:
                 -> Planet gridcell SimThing
                 -> inert 1×1 gridcell SimThings in the same starmap/system grid
                    -> child grid containing:
                       -> 1×1 surface gridcell SimThing
                          -> pop cohorts / fleets / buildings / infrastructure / leaders
                          -> other non-grid gameplay child SimThings
```

## Approved deferrals

Still deferred:

- replace-existing candidate save / overwrite confirmation
- persistent history / timeline
- GPU dispatch / WGSL implementation
- pathfinding
- combat
- economy execution
- fleet movement / supply
- non-canonical savefile format

## Evidence lifecycle and cleanup

During this PR, no live evidence rows were deleted. No prior result reports were deleted. No DA-approved lifecycle rows from #852/#853 were demoted. This docs-only row uses **PROBATION-DOC-REPAIR**. New result evidence lives in this file.

## Boundary / non-goals

Docs-only navigation update. No code, ScenarioSpec schema, runtime behavior, DA rerun, unrelated PROBATION row promotion/demotion, or ladder reopening.

## Validation

| Command | Result |
|---|---|
| `cargo fmt -p simthing-spec -p simthing-driver -p simthing-mapeditor -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `git diff --check` | PASS |
| Placeholder guard | PASS |
| Alias guard | PASS |
| Lifecycle isolation check | PASS |
| Doc guards (DA-APPROVED / rerun / surface tier) | PASS |

## Files changed

- `docs/design_0_0_8_3_studio_production.md`
- `docs/workshop/studio_production_log.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/studio_production_post_da_closure_docs_0_results.md`

## Known gaps

- Next production track not yet selected by project owner.
- Approved deferrals (replace-existing save, history, GPU dispatch, etc.) remain open tracks.

## Next recommended action

Project owner selects the next production track. Do not reopen Scenario Runtime + Save/Load unless regression review is explicitly requested.