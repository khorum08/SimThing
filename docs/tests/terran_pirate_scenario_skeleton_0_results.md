# TERRAN-PIRATE-SCENARIO-SKELETON-0 — Horizon scenario skeleton through sim-owned GPU tick

> **Lifecycle: PROBATION** — skeleton authority, driver compile, resident GPU tick, CPU/GPU parity, scoped readback, panic guard. REAL_ADAPTER_OBSERVED. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** TERRAN-PIRATE-SCENARIO-SKELETON-0  
**Base:** `master` after PR #763 / SIM-GPU-READBACK-SCOPE-0

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated |
| `docs/tests/terran_pirate_scenario_skeleton_0_results.md` | PROBATION | Created (this file) |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated § TERRAN-PIRATE-SCENARIO-SKELETON-0 |
| `crates/simthing-mapeditor/tests/fixtures/terran_pirate_skeleton.simthing-scenario.json` | Fixture | Created from builder |

## Why this is not hygiene

First scenario-shaped horizon skeleton proving the constitutional chain beyond two-cell vertical seed: scenario authority → driver compile → sim resident GPU tick → scoped proof readback.

## Orientation answers

| Question | Answer |
|---|---|
| Minimal skeleton today? | 4 gridcell Locations, 3 canonical links (fork at corridor), cohort payloads, STEAD placements |
| Deferred | Fleets, factions, borders, combat, economy, Gu-Yang, PALMA, pathfinding, Studio runtime UI |
| First link-coupling tick | Sum-over-INPUT_LIST neighbor gather on forked hyperlane graph |
| Input vector | `[10, 20, 40, 30]` dense order (hub, corridor, choke, branch) |
| Expected output | `[20, 80, 20, 20]` (CPU oracle from compiled adjacency) |
| Driver compile without Studio? | Yes — `compile_structural_link_neighbor_sum_plan` |
| Sim resident GPU tick? | Yes — `SimGpuAccumulatorTickState` + `ProofReadback` |
| Readback scoped? | Yes — `scoped_debug_readback_allowed`; panic test added |
| Gu-Yang/PALMA | Deferred STEAD §10 |

## Scenario skeleton authority summary

- **scenario_id:** `terran_pirate_skeleton`
- **provenance:** `TERRAN-PIRATE-SCENARIO-SKELETON-0`
- **topology:** hub(1) — corridor(2) — choke(4); corridor forks to branch(3)
- **links:** 1↔2, 2↔3, 2↔4

## Driver compile proof

5/5 driver tests PASS. Corridor dense slot gathers neighbors [0,2,3]. AO-WGSL-0 compatible.

## Sim CPU/GPU tick proof

7/7 sim tests PASS. CPU == GPU == `[20, 80, 20, 20]`. Scenario authority not mutated.

## Readback panic/scope proof

`scoped_debug_readback_guard_restores_after_panic` — PASS (catch_unwind + RAII Drop).

## Studio load/projection proof

Skeleton loads via scenario IO; hydration (4 cells) and view model (4 stars, 3 hyperlanes) rebuild.

## Studio runtime bypass

Existing guards PASS; no Studio dispatch of GPU tick state.

## Gu-Yang / PALMA

Deferred per STEAD §10.

## Big-endian backlog

Deferred.

## Forbidden-token scan

No forbidden terms in GPU/runtime identifiers. Human-readable terran/pirate in provenance/fixture names only.

## Tests added

- mapeditor: 10 scenario/Studio tests + fixture
- driver: 5 compile tests
- sim: 7 tick tests
- gpu: 1 panic readback test

## Commands run

**Note (0R remediation):** This report did not record exact command outcomes. See `docs/tests/terran_pirate_scenario_skeleton_0r_results.md` for the repaired validation sweep.

## Windows / resource-limit notes

No paging-file failures observed.

## DA status

**PROBATION** — horizon skeleton canonical execution proven; awaiting owner sign-off.