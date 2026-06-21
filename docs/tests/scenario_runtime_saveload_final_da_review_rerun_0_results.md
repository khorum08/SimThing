# SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0 Results

## Status
**APPROVED FOR DA PROMOTION / LIFECYCLE GRADUATION.** The planet 1×1 surface gridcell tier blocker from the
prior HOLD is fully resolved. The Scenario Runtime + Save/Load closing track graduates from PROBATION to
**CURRENT_EVIDENCE — DA-APPROVED**.

## PR / branch / merge
- Branch: `scenario-runtime-saveload-final-da-review-rerun-0`
- Remediation under review: `SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0` / **PR #851** (merge `704f6084`).

## Mission
Re-run the final DA review after PR #851's surface-gridcell remediation; promote the closing track iff the
blocker is genuinely resolved and all checks pass. No new features, mutation semantics, GPU dispatch, history,
combat, pathfinding, economy, or fleet movement introduced.

## Reviewed documents
`docs/agents.md`; `docs/tests/agent_completion_discipline_0.md`; `docs/design_0_0_8_3_studio_production.md`;
`docs/workshop/studio_production_log.md`; `docs/tests/current_evidence_index.md`; prior review
`scenario_runtime_saveload_final_da_review_0_results.md`; remediation
`scenario_planet_surface_gridcell_tier_0_results.md`; the DA precheck + all eleven closing-track result
reports; constitution `design_0_0_8_3.md` (§0/§0.7/§0.8) + antecedents; `simthing_core_design.md`;
`stead_spatial_contract.md`; ADRs `resource_flow_substrate.md`, `mapping_sparse_regioncell.md`,
`invariants.md`.

## Reviewed code paths
`crates/simthing-spec/src/spec/planet_child_location.rs` (decisive), `scenario.rs` (role const + exports),
`loaded_scenario_recursive_rf_runtime.rs` (surface arenas + bubbling proofs), `recursive_local_rf.rs`,
`lib.rs` (exports); driver compile plans; `crates/simthing-mapeditor/src/*`.

## Reviewed tests
spec: `planet_child_location` (9), `planet_child_location_admission` (25), `scenario_canonical_io` (7),
`scenario_stead_map_roundtrip` (11), `loaded_scenario_studio_session_envelope` (11),
`loaded_scenario_recursive_rf_runtime` (15), `loaded_scenario_runtime_report_chain` (18),
`scenario_candidate_from_runtime` (18), `scenario_candidate_save_reopen` (24). driver:
`loaded_scenario_recursive_rf_runtime` (10), `loaded_scenario_runtime_report_chain` (9),
`scenario_candidate_from_runtime` (9), `scenario_candidate_save_reopen` (8). mapeditor:
`studio_scenario_runtime_saveload_ui` (14), `studio_candidate_reopen_adopt` (11). **All pass.**

## PR #851 remediation review
Verified against the prior HOLD blocker, line by line:
- `LOCAL_GRIDCELL_ROLE_SURFACE = "surface"` exists (`scenario.rs:112`) and is exported (`lib.rs:620`).
- `make_planet_gridcell` seeds a `make_surface_gridcell()` child at (0,0) (`planet_child_location.rs:365`).
- Admission (`evaluate_planet_local_gridcell` / `evaluate_planet_interior_location_child`): the surface
  gridcell is **admitted** at (0,0); a **second** surface → `PlanetSurfaceGridcellDuplicate`; surface off
  (0,0) → `PlanetLocalGridCoordinateOutOfFrame`; **missing** surface → `PlanetSurfaceGridcellMissing`;
  **direct** gameplay child under a planet → `PlanetDirectGameplayChildRequiresSurfaceGridcell` (+
  `direct_gameplay_child_under_planet_count`); gameplay **under surface** → admitted (+
  `gameplay_child_under_surface_count`); only deeper-than-surface Location nesting stays
  `DeepPlanetChildDeferred`.
- Hiding concern (handoff): resolved — missing surface and direct gameplay each raise their **own** errors;
  `planet_gameplay_children()` returning empty cannot mask an invalid placement because admission errors
  independently.
- `is_surface_gridcell`, `planet_surface_gridcell`, `planet_gameplay_children`, `make_surface_gridcell`, and
  report fields `surface_gridcell_count` / `planet_surface_gridcell_count` / `gameplay_child_under_surface_count`
  / `direct_gameplay_child_under_planet_count` / `surface_gridcell_tier_present` /
  `surface_gridcell_tier_required` all present and exercised.
- RF: `surface_arena_count`, `gameplay_rows_parented_to_surface`, `surface_to_planet_bubbling_present` present;
  the `prove_*` functions are **non-tautological** (cross-reference tree structure vs. participant rows /
  arena reports) and are exercised **non-vacuously** on the owner-silo fixture (real cohorts/fleet + surfaces).

## Studio production track state
Complete and conformant. ScenarioSpec authority, owner-as-metadata (non-spatial), the corrected spatial
hierarchy, RF surface settle-then-bubble, canonical-JSON candidate save/reopen, and non-authority surfaces are
all present, correct, and tested.

## Production log context
Closing ladder consolidated scenario runtime save/load for human DA review; Studio UI command/presentation
only; persistent history / GPU dispatch / combat / economy / fleet movement deferred. Context only.

## Closing track evidence matrix

| Track ID | PR | Merge / evidence SHA | Result report | Code reviewed | Tests reviewed | Validation | Authority boundary | Spatial hierarchy | Lifecycle before | Lifecycle after | DA decision |
|---|---|---|---|---|---|---|---|---|---|---|---|
| SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 | #828 | ledger | yes | 7✓ | PASS | OK | conformant (carries surface tier) | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| SCENARIO-STEAD-MAP-ROUNDTRIP-0 | #834 | ledger | yes | 11✓ | PASS | OK | **CONFORMANT** (asserts gameplay-under-surface) | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 | #836 | ledger | yes | 11✓ | PASS | OK | conformant | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 | #838 | ledger | yes | 15+10✓ | PASS | OK | **CONFORMANT** (settles at surface; bubbles surface→planet) | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 | #840 | ledger | yes | 18+9✓ | PASS | OK | conformant | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| SCENARIO-CANDIDATE-FROM-RUNTIME-0 | #842 | ledger | yes | 18+9✓ | PASS | OK | **CONFORMANT** (preserves surface) | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| SCENARIO-CANDIDATE-SAVE-REOPEN-0 | #844 | `aecb4421` | yes | 24+8✓ | PASS | OK | **CONFORMANT** (preserves surface) | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0 | #845 | `8aa72e6c` | yes | (24)✓ | PASS | OK | conformant | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 | #846 | ledger | yes | 14✓ | PASS | OK (presentation) | **CONFORMANT** (status preserves surface) | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| STUDIO-CANDIDATE-REOPEN-ADOPT-0 | #847 | `8cefd9c8` | yes | 11✓ | PASS | OK | **CONFORMANT** (adopted scenario preserves surface) | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 | #848 | `46c2c4b2` | yes | n/a | PASS | OK | conformant | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-0 | #849 | `0b6306c5` | yes | n/a | n/a | OK | (superseded by rerun) | PROBATION (HELD) | CURRENT_EVIDENCE — DA-APPROVED (closed by rerun) | APPROVE |
| SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0 | #851 | `704f6084` | **yes (decisive)** | 9+25✓ | PASS | OK | **CONFORMANT** (restores surface tier) | PROBATION | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |
| SCENARIO-RUNTIME-SAVELOAD-FINAL-DA-REVIEW-RERUN-0 | (this) | (this PR) | this doc | — | — | — | — | — | CURRENT_EVIDENCE — DA-APPROVED | APPROVE |

## Constitution / ADR alignment
Aligned. Everything-is-a-SimThing; ScenarioSpec/Session authority with projections/GPU buffers as cache;
owner-entities are GameSession siblings, never spatial parents, capture = metadata/column, never reparenting
(enforced); RF = generic Accumulator Flow accumulation, reduce-up/disburse-down, owner/resource/scope channels
are metadata not containment; semantic-free posture preserved (CPU oracle/serialization/reporting only). The
spatial containment hierarchy now matches the mandated truth.

## Corrected spatial gridcell hierarchy audit
**PASS.** `GameSession → GalaxyGridcell → {StarMap gridcell + inert} → {Planet gridcell + inert} → {1×1
surface gridcell} → gameplay children`. No extra galaxy or planet map node; planet admits exactly one surface
gridcell at (0,0); gameplay children live under the surface gridcell; owner identity is metadata, not spatial
parentage. Code, tests, fixtures, and the production-doc hierarchy block all agree.

## Planet surface gridcell tier audit
**PASS** (see PR #851 review above). Required, admitted (not deferred), uniqueness/coordinate/missing/direct
all guarded with tests.

## Accumulator Flow / GPU residency audit
**PASS.** Runtime RF rows are GPU-compatible owner/resource/scope row/table surfaces
(`gpu_compatible_row_table_surface`); the **surface gridcell is now the leaf parent arena**; sibling settlement
occurs at the surface arena before bubbling surface→planet→star→galaxy (proven non-vacuously); owner/resource/
scope are metadata channels, not parents; candidate mutation consumes runtime property-view rows; candidate
save/reopen is canonical ScenarioSpec JSON only; **no CPU production engine, no GPU dispatch** introduced. No
forbidden CPU-authority phrases.

## ScenarioSpec authority audit
PASS. Sole serializable authority; UI/Bevy/runtime reports/property-view rows/GPU buffers non-authoritative.

## STEAD contract audit
PASS. Stable IDs, links, RF metadata, owner-vs-spatial distinction, and the **corrected** spatial tree shape
(now including the surface tier) survive canonical roundtrip.

## Runtime report-chain audit
PASS over the corrected hierarchy.

## Candidate mutation audit
PASS. Candidate cloned from runtime property-view rows; surface tier preserved (test).

## Candidate save/reopen audit
PASS. Canonical-JSON create-new save + reopen; surface tier preserved (test); no distinct savefile format.

## Studio UI workflow audit
PASS. Command/presentation surface; candidate status / Save Candidate / Reopen Candidate; surface tier
preserved in status (test).

## Reopen Candidate adoption audit
PASS. Successful reopen adopts the candidate into the active session; **adopted scenario preserves the surface
tier** (test); failed reopen preserves the current session.

## Non-authority surfaces audit
PASS. UI/Bevy/reports/property-view rows/GPU buffers remain non-authoritative.

## Deferred boundaries
Replace-existing candidate overwrite, persistent history, GPU dispatch, pathfinding, combat, economy
execution, fleet movement, non-canonical savefile format — all remain deferred.

## Regression validation
`cargo check -p simthing-spec` / `-p simthing-driver` build (tests link & run). Full per-test matrix run (all
PASS — counts above). Mapeditor (Bevy) tests run and pass. Preflight: alias file absent, no live alias links,
no placeholders, no forbidden CPU-authority phrases. `git diff --check` clean. Fixtures `owner_silo_disburse_
down_scoped` and `planet_child_location_admitted` unmodified by tests; both confirmed to contain the surface
tier via the admission test (`surface_gridcell_tier_present`), since roles are byte-encoded (not literal
strings) in the canonical JSON.

## Evidence lifecycle and cleanup
No live evidence rows or prior reports deleted. This rerun's evidence lives only in this file. No scratch files.

## Lifecycle graduation decision
Promote all fourteen closing-track rows from PROBATION to **CURRENT_EVIDENCE — DA-APPROVED** (the evidence
index's canonical approved label; `DA-APPROVED` is the preferred handoff label and is recorded inline).

## Remaining risks / known gaps
None blocking. Deeper-than-surface Location nesting and replace-existing/history/GPU dispatch remain deferred
by design. Structural mirrors remain defensive f32 vectors (pre-existing, out of scope).

## DA decision
Decision: APPROVED FOR DA PROMOTION / LIFECYCLE GRADUATION.

The Scenario Runtime + Save/Load Closing Track is DA-approved after planet surface gridcell remediation.
ScenarioSpec remains authority; STEAD IDs, links, RF metadata, and the corrected spatial gridcell hierarchy
are preserved; the planet 1×1 surface gridcell tier is restored; gameplay SimThings are homed under surface
gridcells; Accumulator Flow settles first at surface arenas and remains GPU-resident in shape through
row/table surfaces; CPU remains oracle/reference/serialization/reporting only; Studio UI, Bevy state, runtime
reports, property-view rows, and GPU buffers remain non-authoritative. Replace-existing candidate save,
persistent history, GPU dispatch, pathfinding, combat, economy execution, and fleet movement remain deferred.

## Boundary / non-goals
This review adds no features, no mutation semantics, no GPU dispatch, no history, and does not broaden into
combat, pathfinding, economy, or fleet movement.

## Files changed
- `docs/tests/scenario_runtime_saveload_final_da_review_rerun_0_results.md` (new)
- `docs/design_0_0_8_3_studio_production.md` (rerun section + ladder; lifecycle DA-APPROVED)
- `docs/tests/current_evidence_index.md` (closing-track rows promoted PROBATION → CURRENT_EVIDENCE — DA-APPROVED)

## Next recommended action
Closing track is DA-approved (closed). The project owner selects the next track.
