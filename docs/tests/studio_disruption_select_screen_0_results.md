# STUDIO-DISRUPTION-SELECT-SCREEN-0 Results

## Status
**PROBATION / DRAFT** — PR [#1420](https://github.com/khorum08/SimThing/pull/1420); dispatch `5022654370`; Remand 1 `5023235060`; Remand 2 `5024193617`. Owner OVL remains OPEN. No freeze.

## Identity SHAs
| role | full SHA |
|---|---|
| base / post-12.10 master | `ced8ce2d80a24213676c4f606edd2126eb2bda6f` |
| handoff_head | `9f1949d9cd54c997414ea7a87004741e3c423500` |
| implementation_code_sha | `4b2db7e8f8d450c718c8006a60bc7182598be906` |
| tested_code_sha | `3a5a2bb89796ebf4bb9a3ffe90a8c22a29f04fd6` |
| evidence_head_sha / final_head_sha | PR tip after this evidence commit (bound in PR body) |

## Remand 2 correction
- Reverted production `DisruptionReadoutRecord::new` completely (`simthing-spec` disruption readout matches pre-PR form).
- Integration falsifier builds admitted values through test-local `DisruptionAuthorityReadback` → `disruption_readout_snapshot_with_readback` → `studio_disruption_readout_map_from_snapshot`.
- Fixture star systems carry `GALAXY_GRIDCELL_ROLE_STAR_SYSTEM` so the landed 12.2 snapshot path enumerates them.

## Remand 1 corrections (retained)
1. Owned/hostile/neutral selection consumes distinct admitted values; eligibility is `star_id == selected_id`; co-owned identity + absent-id fail-soft bite.
2. Live selected disruption invalidates global + selected per-star dirty keys.
3. Duplicate TEST-BUDGET triage/justification rows removed (one each).

## What changed (implementation retained)
- Presentation mapper + `sync_star_visuals_system` compose + dirty keys + Studio_ops Telemetry
- No sealed Spec API added for tests

## Proof matrix
| test | catches |
|---|---|
| breakpoints_and_above_100_clamp | wrong piecewise / missing clamp |
| deselect_restores_identity | sticky screen after clear |
| applies_to_owned_neutral_and_hostile_selection | wrong system / ignore 12.2 path / ownership gate |
| coexists_with_11_6_owned_brighten | co-owned set inherits disruption |
| live_disruption_invalidates_visual_dirty_gate | live accretion does not refresh visuals |
| no_wgsl_or_scenario_mutation_surface | kernel/Spec creep |

## Scope Ledger
| | |
|---|---|
| Specified | Selected-star disruption blur/tint; deselect; 11.6 coexist; telemetry |
| Implemented | mapeditor presentation + Remand-1/2 falsifiers via 12.2 readback path |
| Proxied | admitted 12.2 disruption snapshot (fail-soft 0 when absent) |
| Deferred | Owner OVL; freeze (post orch accept) |
| Out of scope | Spec mutation; WGSL; CPU planner; 12.5; Spec test constructors |

## Conformance
piecewise YES · clamp YES · deselect YES · 12.2-readback selection YES · 11.6 coexist YES · dirty-gate YES · no Spec API leak YES · OVL OPEN

## Sticky disposition
Expected Clearance without class-envelope-violation from Spec API leak. Keep draft/open/unmerged. Owner alone closes OVL.
