# SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-0 Results

## Status
**HELD — NOT READY FOR DA PROMOTION.** One track-wide constitutional blocker: the spatial gridcell
hierarchy collapses the mandated **1×1 surface gridcell tier**. All closing-track rows remain in **PROBATION**.

## PR / branch / merge
- Branch: `scenario-runtime-saveload-final-da-review-0`
- This PR documents the final DA review and the HOLD. It promotes **nothing**.

## Mission
Final maximum-reasoning DA review of the Scenario Runtime + Save/Load Closing Track (code, tests, evidence,
constitutional audit, Studio production-track audit, corrected spatial gridcell hierarchy audit, Accumulator
Flow / GPU-residency audit) and lifecycle graduation out of PROBATION **iff clean**. It is not clean.

## Reviewed documents
`docs/agents.md`; `docs/design_0_0_8_3_studio_production.md`; `docs/workshop/studio_production_log.md`
(context); `docs/tests/current_evidence_index.md`; all eleven closing-track `*_results.md`; constitution
`docs/design_0_0_8_3.md` (§0/§0.7/§0.8) + antecedents; `docs/simthing_core_design.md` (§0/§2/§3/§5/§7);
`docs/stead_spatial_contract.md`; ADRs `resource_flow_substrate.md`, `mapping_sparse_regioncell.md`,
`game_mode_session_installation.md`, `spec_session_state_replay.md`; `docs/invariants.md`.

## Reviewed code paths
`crates/simthing-spec/src/spec/planet_child_location.rs` (decisive), `scenario.rs`,
`loaded_scenario_recursive_rf_runtime.rs`, `owner_silo_disburse_down.rs`, `spatial_local_grid.rs`,
`scenario_canonical_io.rs`; driver compile plans; `crates/simthing-mapeditor/src/*`.

## Reviewed tests
`loaded_scenario_recursive_rf_runtime` (spec, 15 passed); spec/driver canonical IO + STEAD roundtrip +
envelope + report-chain + candidate suites (encode the model under review); mapeditor UI/adopt suites.

## Studio production track state
The production doc claims the closing track complete through DA precheck. ScenarioSpec authority, owner-as-
metadata (not spatial parent), save/load mechanism, RF "settle-siblings-then-bubble" shape, and non-authority
surface discipline are all present and correctly framed. The **spatial containment hierarchy it certifies is
non-conformant** (see audit below).

## Production log context
The closing ladder was introduced to stop hygiene-only loops and consolidate scenario runtime save/load for
human DA review; Studio UI is command/presentation-only; persistent history / GPU dispatch / combat / economy
execution / fleet movement are deferred. Context only — does not override the current production doc or this
review.

## Closing track evidence matrix

| Track ID | PR | Result report | Code reviewed | Tests reviewed | Validation | Authority boundary | Spatial hierarchy | Lifecycle before | Lifecycle after | DA decision |
|---|---|---|---|---|---|---|---|---|---|---|
| SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 | #828 | yes | yes | yes | build✓ | OK (authority preserved) | n/a (mechanism); roundtrips non-conformant fixture | PROBATION | PROBATION | HOLD |
| SCENARIO-STEAD-MAP-ROUNDTRIP-0 | #834 | yes | yes | yes | build✓ | OK | **NON-CONFORMANT** (certifies collapsed shape) | PROBATION | PROBATION | HOLD |
| LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 | #836 | yes | yes | yes | build✓ | OK | carries collapsed model | PROBATION | PROBATION | HOLD |
| LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 | #838 | yes | yes | 15✓ | build✓ | OK | **NON-CONFORMANT** (settles at planet w/ gameplay as direct siblings) | PROBATION | PROBATION | HOLD |
| LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 | #840 | yes | yes | yes | build✓ | OK | carries collapsed model | PROBATION | PROBATION | HOLD |
| SCENARIO-CANDIDATE-FROM-RUNTIME-0 | #842 | yes | yes | yes | build✓ | OK | clones collapsed model | PROBATION | PROBATION | HOLD |
| SCENARIO-CANDIDATE-SAVE-REOPEN-0 | #844 | yes | yes | yes | build✓ | OK | mechanism sound; non-conformant payload | PROBATION | PROBATION | HOLD |
| SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0 | #845 | yes | yes | yes | build✓ | OK | mechanism sound; non-conformant payload | PROBATION | PROBATION | HOLD |
| STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 | #846 | yes | yes | yes | build✓ | OK (presentation only) | presents collapsed model | PROBATION | PROBATION | HOLD |
| STUDIO-CANDIDATE-REOPEN-ADOPT-0 | #847 | yes | yes | yes | build✓ | OK | adopts collapsed model | PROBATION | PROBATION | HOLD |
| SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 | #848 | yes | yes | yes | build✓ | OK | precheck did not catch collapse | PROBATION | PROBATION | HOLD |
| SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-0 | (this) | this doc | — | — | — | — | — | — | PROBATION | HELD |

## Constitution / ADR alignment
Aligned: everything-is-a-SimThing; ScenarioSpec/Session authority with projections/GPU buffers as cache (core
§1/§4, invariants State Authority); owner-entities are GameSession siblings, never spatial parents, capture =
metadata/column, never reparenting (core §2 Law 2 — **explicitly enforced**, `planet_child_location.rs:4`);
RF = generic accumulation, reduce-up/disburse-down, owner/resource/scope channels are metadata not containment
(core §5, RF ADR); semantic-free `simthing-sim` posture preserved (CPU oracle/serialization/reporting only).
**Not aligned:** the spatial containment hierarchy (below).

## Corrected spatial gridcell hierarchy audit — **BLOCKER**
Mandated (base truth): `Planet gridcell → child grid → 1×1 surface gridcell → {pop cohorts, fleets, buildings,
leaders, …}`.

Implemented: gameplay SimThings are admitted as **direct children of the planet gridcell**
(`planet_child_location.rs::evaluate_planet_non_grid_child`, walked from `child.children` at
`evaluate_planet_local_gridcell` lines ~814-824); any **Location child of a planet** (which *is* the surface
gridcell) is **deferred/blocked** as `DeepPlanetChildDeferred` (`has_deeper_location_nesting`). The string
"surface gridcell" appears in **no** source, doc, test, or fixture.

Drift classification: **code model blocker + documentation blocker + test-coverage blocker + DA-promotion
blocker**. Present in code (`planet_child_location.rs`), the authoritative production doc
(`design_0_0_8_3_studio_production.md` lines 30-39, 56-57), the tests (encode it), and the corpus fixture (4
cohorts / 1 fleet, no surface tier). No DA-approved Deviation Record exists; per invariants §"No silent tier
collapse," a collapsed tier — even "parked/not-yet-wired" — cannot pass without one, and as DA I decline to
approve a Deviation that erases the very tier the corrected hierarchy emphasizes.

Upper levels are conformant: `GalaxyMap → galaxy gridcell(role=star_system = StarMap) → local gridcell(role=
planet = Planet)` carries no extra node (galaxy correction satisfied). Owner correction satisfied. The
production-doc prose at lines 35-37 ("galactic gridcell → interior local grid → star-system local gridcell")
is loose vs the corrected wording and should be tightened in remediation, but the decisive defect is the
planet→surface collapse.

## Accumulator Flow / GPU residency audit
Doctrine satisfied in shape: RF rows are GPU-compatible owner/resource/scope row/table surfaces; parent arena
rows settle local gridcell siblings before upward bubbling; owner/resource/scope are metadata channels, not
spatial parents; candidate mutation consumes runtime property-view rows; candidate save/reopen is canonical
ScenarioSpec JSON only; no CPU production engine, no GPU dispatch introduced. **No forbidden CPU-authority
phrases** present (handoff grep false-positives were pipe/exit-code artifacts; clean on exit code). Caveat:
the settlement is performed at the **planet** arena with gameplay children as direct siblings; once the surface
tier is restored, settlement must occur at the **surface gridcell** arena, then bubble surface→planet→star→
galaxy.

## ScenarioSpec authority audit
PASS. `SimThingScenarioSpec` is the sole serializable authority; UI/Bevy/runtime reports/property-view rows/GPU
buffers are non-authoritative; save/load = canonical JSON.

## STEAD contract audit
PASS in mechanism (stable IDs, links, RF metadata, owner-vs-spatial distinction survive roundtrip) — but the
preserved **spatial tree shape is the non-conformant one**.

## Runtime report-chain audit
PASS in composition; carries the collapsed model.

## Candidate mutation / save-reopen / Studio UI / Reopen-adopt audits
Mechanisms PASS (clone-from-runtime, canonical-JSON create-new save, reopen, UI command/presentation surface,
adopt-on-success). All operate over the non-conformant model.

## Non-authority surfaces audit
PASS. UI/Bevy/reports/property-view rows/GPU buffers remain non-authoritative.

## Deferred boundaries
Persistent history, replace-existing candidate overwrite, GPU dispatch, pathfinding, combat, economy execution,
fleet movement, savefile format distinct from canonical ScenarioSpec JSON — all remain deferred (correctly).

## Regression validation
`cargo check -p simthing-spec` (build OK, 5 warnings); `cargo check -p simthing-driver` (build OK, 39
warnings); `cargo test -p simthing-spec --test loaded_scenario_recursive_rf_runtime` (15 passed). Preflight
guards: alias file absent; **no live alias links** (only deletion-context prose); **no placeholders**; **no
forbidden CPU-authority phrases**. Fixture `owner_silo_disburse_down_scoped.simthing-scenario.json` is
**unmodified** by tests. The full per-test matrix was not exhaustively re-run: the review terminated at a
model-level HOLD blocker that supersedes test-pass (the suites validate the model under review). No unavailable
command is claimed as passed.

## Evidence lifecycle and cleanup
No live evidence rows or prior result reports deleted. This review's evidence lives only in this file. No
scratch files created.

## Lifecycle graduation decision
**No promotion.** All eleven closing-track rows + this review row stay **PROBATION**. The canonical lifecycle
labels in `current_evidence_index.md` are `PROBATION` / `CURRENT_EVIDENCE — DA-APPROVED`; no row advances.

## Remaining risks / known gaps
The collapsed surface tier means RF arena settlement, candidate cloning, and STEAD shape-preservation all
certify a hierarchy that gameplay SimThings cannot correctly inhabit; promoting would lock the wrong spatial
contract into the save/load format and the Studio workflow.

## DA decision
Decision: HOLD / NOT READY FOR DA PROMOTION.

Blocking issues:
- The mandated **1×1 surface gridcell tier is collapsed**: gameplay SimThings (cohort/fleet/infrastructure/
  leader) are homed directly on the planet gridcell, and the surface-gridcell Location is actively deferred
  (`DeepPlanetChildDeferred`), across code (`planet_child_location.rs`), the production doc
  (`design_0_0_8_3_studio_production.md` lines 30-57), the closing-track tests, and the corpus fixture. No
  DA-approved Deviation Record exists; invariants §"No silent tier collapse" forbids promotion.

Required remedial handoff:
- SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0

## Boundary / non-goals
This review does not implement the remediation, does not promote any row, and does not authorize replace-
existing overwrite, persistent history, GPU dispatch, pathfinding, combat, economy execution, or fleet
movement.

## Files changed
- `docs/tests/scenario_runtime_saveload_final_da_review_0_results.md` (new)
- `docs/design_0_0_8_3_studio_production.md` (HOLD section)
- `docs/tests/current_evidence_index.md` (FINAL-DA-REVIEW row at PROBATION/HELD; no other row changed)

## Next recommended action
Execute **SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0** (scope below), then re-run this final DA review.

---

## SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0 — narrowest remedial handoff

**Goal:** restore the one missing tier so the canonical hierarchy matches the mandated truth:
`Planet gridcell → child grid (default 1×1) → 1×1 surface gridcell (Location) → {cohort/fleet/infrastructure/
leader}`.

**Scope (minimal):**
1. `crates/simthing-spec/src/spec/planet_child_location.rs`: **admit** a 1×1 surface gridcell Location as the
   planet gridcell's interior child (un-defer it from `DeepPlanetChildDeferred`); re-home
   `evaluate_planet_non_grid_child` / `collect_planet_non_grid_children` to read gameplay children from the
   **surface gridcell**, not directly from the planet. Reject gameplay (non-grid) SimThings placed directly
   under a planet gridcell. Keep deeper-than-surface nesting bounded/deferred.
2. RF runtime (`loaded_scenario_recursive_rf_runtime.rs` + driver compile): settle the leaf arena at the
   **surface gridcell** (siblings = its gameplay children), then bubble surface→planet→star-system→galaxy.
3. Corpus fixture `scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json`: insert the 1×1
   surface gridcell between each planet gridcell and its cohorts/fleet. Confirm normal tests do not mutate it.
4. Production doc `design_0_0_8_3_studio_production.md` lines 30-57: replace the collapsed hierarchy block and
   spatial invariants with the corrected hierarchy (explicit 1×1 surface gridcell tier; gameplay children are
   children of the surface gridcell); tighten the loose "galactic gridcell → interior grid → star-system
   gridcell" wording to the corrected galaxy/star levels.
5. Update affected spec + driver tests to assert the surface-gridcell tier and surface-level RF settlement.
6. Re-run the closing-track validation matrix, then re-run SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-0.

**Out of scope:** replace-existing overwrite, persistent history, GPU dispatch, pathfinding, combat, economy
execution, fleet movement, any savefile format other than canonical ScenarioSpec JSON.
