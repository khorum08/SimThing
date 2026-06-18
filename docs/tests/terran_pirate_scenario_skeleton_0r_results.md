# TERRAN-PIRATE-SCENARIO-SKELETON-0R — Authority/evidence hardening before Gu-Yang/PALMA

> **Lifecycle: PROBATION** — canonical scenario authority promoted outside Studio/editor fixtures; driver/sim proofs consume `simthing-spec` directly; Studio remains load/projection consumer; exact validation commands recorded. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** TERRAN-PIRATE-SCENARIO-SKELETON-0R  
**Base:** `master` after PR #764 / TERRAN-PIRATE-SCENARIO-SKELETON-0

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | CANONICAL_AUTHORITY | Created (promoted from editor fixture) |
| `crates/simthing-mapeditor/tests/fixtures/terran_pirate_skeleton.simthing-scenario.json` | LEGACY_EDITOR_FIXTURE | Retained for regeneration convenience; not authority |
| `docs/tests/terran_pirate_scenario_skeleton_0r_results.md` | PROBATION | Created (this file) |
| `docs/tests/terran_pirate_scenario_skeleton_0_results.md` | PROBATION | Superseded for authority boundary; validation gap noted |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated § TERRAN-PIRATE-SCENARIO-SKELETON-0R |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated |

## Why this is not hygiene

PR #764 proved the constitutional chain but left two soft spots blocking Gu-Yang/PALMA horizon work:

1. The result report claimed a full validation sweep without recording exact command outcomes.
2. Driver/sim skeleton tests imported scenario authority through `simthing-mapeditor` builders, letting Studio/editor convenience become the source of truth for non-Studio proofs.

Gu-Yang falloff borders and PALMA reach must run over structural grid authority, not Studio helper state or test-only editor builders.

## Current authority-boundary issue (remediated)

**Before 0R:** `terran_pirate_skeleton_scenario_spec()` in `simthing-mapeditor` was the de facto authority for driver/sim horizon proofs.

**After 0R:** `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` is canonical `SimThingScenarioSpec` authority. Driver and sim deserialize through `simthing-spec`. Mapeditor builder proves semantic equivalence only.

## Orientation answers

| Question | Answer |
|---|---|
| Current source of Terran Pirate skeleton scenario? | `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` (canonical artifact) |
| Which tests imported through simthing-mapeditor (before)? | `crates/simthing-driver/tests/terran_pirate_skeleton_compile.rs`, `crates/simthing-sim/tests/terran_pirate_skeleton_tick.rs` |
| Which tests load scenario authority directly (after)? | Same driver/sim test files via `include_str!` + `deserialize_scenario_authority` |
| Where does canonical scenario artifact live? | `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` |
| Driver compile without Studio/editor dependency? | Yes — `compile_structural_link_neighbor_sum_plan` on deserialized `SimThingScenarioSpec` |
| Sim tick without Studio/editor dependency? | Yes — driver compile + `SimGpuAccumulatorTickState` on canonical authority |
| Studio still loads/projects same scenario? | Yes — `load_scenario_authority_from_path` / `load_studio_session_from_scenario_path` on canonical path |
| Exact validation commands run? | Listed below with PASS/FAIL/PARTIAL |
| Any unrun command? | `cargo test -p simthing-spec` full suite: PARTIAL (1 pre-existing failure unrelated to 0R) |

## Canonical scenario artifact

- **Path:** `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json`
- **scenario_id:** `terran_pirate_skeleton`
- **provenance:** `TERRAN-PIRATE-SCENARIO-SKELETON-0`
- **topology:** hub(1) — corridor(2) — choke(4); corridor forks to branch(3)
- **links:** 1↔2, 2↔3, 2↔4
- **dense inputs (test oracle):** `[10, 20, 40, 30]` → expected `[20, 80, 20, 20]`
- No render anchors, Bevy IDs, Studio config, or GPU readback values in authority.

## Driver compile proof (no Studio)

5/5 tests PASS (`--test terran_pirate_skeleton_compile`). Loads canonical JSON via `simthing-spec`. Corridor dense slot gathers neighbors [0,2,3]. AO-WGSL-0 compatible. No mapeditor import.

## Sim CPU/GPU tick proof (no Studio)

7/7 tests PASS (`--test terran_pirate_skeleton_tick`). CPU == GPU == `[20, 80, 20, 20]`. Scenario authority not mutated. Scoped proof readback does not leak into None tick. REAL_ADAPTER_OBSERVED.

## Studio load/projection proof

10/10 tests PASS (`--test terran_pirate_skeleton`). Builder semantically equivalent to canonical artifact. Hydration (4 cells) and view model (4 stars, 3 hyperlanes) rebuild from canonical path.

## Scoped readback preservation

5/5 tests PASS (`--test debug_readback_scope`). `scoped_debug_readback_allowed` panic restoration intact.

## Production None-tick preservation

Not touched in 0R. Existing Studio bypass guards PASS (`accumulator_convergence_1_guards`).

## Gu-Yang / PALMA horizon goals (deferred)

**Gu-Yang falloff borders:**
- adjacency = grid N4 over structural (col,row)
- target operator path = `StructuredFieldStencilOp::SaturatingFlux`
- target readout = saturating flux choke column / bounded field values
- execution = bounded theater first slice
- oversize behavior = typed atlas deferral

**PALMA reach:**
- adjacency = grid N4 over structural (col,row)
- target operator path = `WImpedanceComposeOp` → `MinPlusStencilOp`
- D is a field; W is impedance/feedstock
- execution = bounded theater first slice
- oversize behavior = typed atlas deferral

**Forbidden:** border service, frontline service, route object, predecessor/came_from map, pathfinding engine, movement-order semantics, CPU map planner, semantic WGSL, new bespoke GPU kernel, dense-global diffusion, shrinking structural layout to fit theater cap.

## Big-endian / portable byte-proof backlog

Deferred:
- explicit little-endian byte helpers
- cross-platform byte-order evidence
- replacing host-endian bytemuck casts in canonical artifact byte proofs

## Forbidden-token scan

No forbidden route/predecessor/pathfinding/border-service semantics in changed sources. Human-readable terran/pirate in provenance and scenario names only.

## Tests added/changed

| Crate | Change |
|---|---|
| `simthing-driver` | `terran_pirate_skeleton_compile.rs` — canonical `include_str!` + `simthing-spec` deserialize |
| `simthing-sim` | `terran_pirate_skeleton_tick.rs` — same; local dense-input helper |
| `simthing-mapeditor` | `terran_pirate_skeleton.rs` — canonical path; builder equivalence proof |
| `simthing-mapeditor` | `accumulator_convergence_1_guards.rs` — production doc names 0R |

## Commands run (exact outcomes)

| Command | Status | Notes |
|---|---|---|
| `cargo fmt --all -- --check` | PASS | After `cargo fmt --all` |
| `cargo check -p simthing-spec` | PASS | |
| `cargo test -p simthing-spec` | PARTIAL | 128+ tests pass; `e10_does_not_import_arena_registry_into_simthing_sim` FAIL — **pre-existing on master** (simthing-sim dev-dep on simthing-driver from SKELETON-0); unrelated to 0R |
| `cargo check -p simthing-driver` | PASS | |
| `cargo test -p simthing-driver terran_pirate` | PASS | 5/5 via `--test terran_pirate_skeleton_compile` (serial `CARGO_BUILD_JOBS=1`) |
| `cargo check -p simthing-sim` | PASS | |
| `cargo test -p simthing-sim terran_pirate` | PASS | 7/7 via `--test terran_pirate_skeleton_tick` |
| `cargo check -p simthing-mapeditor` | PASS | |
| `cargo test -p simthing-mapeditor terran_pirate` | PASS | 10/10 via `--test terran_pirate_skeleton` |
| `cargo test -p simthing-mapeditor accumulator_convergence_1_guards` | PASS | 18/18 |
| `cargo test -p simthing-gpu debug_readback_scope` | PASS | 5/5 via `--test debug_readback_scope` |
| `cargo test -p simthing-clausething --test stead_spatial_contract_guards` | PASS | 12/12 |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS | 10/10 |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | PASS | 7/7 |
| `cargo test -p simthing-clausething --test mapgen_movement_front` | PASS | 23/23 |
| `git diff --check` | PASS | |
| `git diff --name-only master...HEAD` | PASS | Recorded at commit time |

## Windows / resource-limit notes

Initial parallel `cargo test -p simthing-spec` hit linker `LNK1102: out of memory`. Reran with `CARGO_BUILD_JOBS=1`; all terran-pirate-relevant test binaries executed successfully.

## Files changed

- `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` (new)
- `crates/simthing-driver/tests/terran_pirate_skeleton_compile.rs`
- `crates/simthing-sim/tests/terran_pirate_skeleton_tick.rs`
- `crates/simthing-sim/Cargo.toml` (added `simthing-spec` dev-dep)
- `crates/simthing-mapeditor/tests/terran_pirate_skeleton.rs`
- `crates/simthing-mapeditor/tests/accumulator_convergence_1_guards.rs`
- `docs/tests/terran_pirate_scenario_skeleton_0r_results.md`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `Cargo.lock`

## Deleted/archived artifacts

None. Legacy editor fixture retained as convenience generator target.

## Deferred work

- Gu-Yang falloff borders (STEAD §10 surface 2)
- PALMA reach field (STEAD §10 surface 3)
- Big-endian portable byte-proof helpers
- Full terran-pirate play-out
- Remediate `e10_does_not_import_arena_registry_into_simthing_sim` vs simthing-sim dev-dep on simthing-driver (pre-existing from SKELETON-0)

## DA status

**PROBATION** — authority boundary hardened; exact validation recorded; awaiting owner sign-off.