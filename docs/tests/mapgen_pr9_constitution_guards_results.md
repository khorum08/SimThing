# MapGen PR9 Constitutional Guard Hardening Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (PR9 guard-hardening report; DA-approved 2026-06-14).

## Verdict

**PASS / DA-APPROVED (2026-06-14, Opus / Design Authority)** — DA performed a genuine pre-merge audit (not
the PR body) and reran the battery green (`mapgen_constitution_guards` 21; clausething 8/10/16/19/23/19/45;
`mapgen_pr8_scheduled_concurrency` 6; `ct_bh3_closeout_sample_driver` 2; fmt/`git diff --check` clean).
Confirmed: this is **guard-hardening only** — the sole new source is `validate_one_system_per_gridcell` (a
grid-metadata `(row,col)` uniqueness check wired into `generate_mapgen_lattice_hierarchy`, no render coords,
no new kind). The 21-test battery is **meaningful, not theater**: it `include_str!`-scans the **actual
source** of all six active generators (PR1–PR7) + the PR8 GPU/driver helpers for Euclidean-authority /
forbidden-vocabulary / forbidden-kind tokens, and pairs every scan with a **behavioral** test
(horizon-cap-rejected, allow_extended-rejected, dense/global-rejected, duplicate-cell-rejected,
links-use-lattice-not-render-coords, positions-inert, PALMA-not-route, L2-doesn't-widen-L1, default-off, and
a scan asserting the PR8 harness keeps `field_values.is_none()`). No new generator feature, GPU kernel,
semantic WGSL, runtime engine, simthing-sim change, CPU planner, full-field readback, or FIELD-MOVIE-DATASET-0
export. PR9 consolidates constitutional guards before PR10 end-to-end sample. No new
generator capabilities, no new GPU kernel, no semantic WGSL, no runtime engine, no simthing-sim changes,
no PR10 install exercise, no FIELD-MOVIE-DATASET-0 export.

## Track scope

0.0.8.2.5 MapGen PR9: Candidate F / Euclidean, P1 bounded-horizon locality, one-system-per-cell, inert
render positions, and forbidden route/path/predecessor/movement/border/frontline guard hardening. **Do not
merge until DA review.**

PR9 is guard hardening only. PR9 does not add new generator capabilities beyond hardening. PR9 does not add
new runtime/GPU kernels. PR9 does not execute the PR10 end-to-end sample. PR9 hardens Candidate F, P1/horizon,
one-system-per-cell, inert-position, and no-pathfinding boundaries. PR9 does not implement FIELD-MOVIE-DATASET-0
export. PR9 does not reopen 0.0.8.2 closeout.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/design_0_0_8_1.md` §0.7
4. `docs/adr/mapping_sparse_regioncell.md`
5. `docs/adr/resource_flow_substrate.md`
6. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR9, §8, §9
7. `docs/design_0_0_8_1_border_hack_track.md`
8. `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md`
9. `docs/clausething/mapgen_corpus_manifest.md`
10. `docs/clausething/MapGenThing.md`
11. `docs/tests/mapgen_pr1`–`mapgen_pr8` results
12. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| Admission helper | `crates/simthing-clausething/src/mapgen_lattice.rs` (`validate_one_system_per_gridcell`) |
| PR9 guard battery | `crates/simthing-clausething/tests/mapgen_constitution_guards.rs` |
| Crate exports | `crates/simthing-clausething/src/lib.rs` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| PALMA guide | `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| PR9 report | `docs/tests/mapgen_pr9_constitution_guards_results.md` |

## Guard categories added

| Category | Coverage |
|---|---|
| Candidate F / Euclidean | Source scans on PR1–PR7 generators + PR8 GPU scheduling modules for forbidden `sqrt(` / `length(` / `normalize(` / `hypot(` / `norm(` / `euclidean` patterns |
| P1 / horizon | Bounded default horizon; cap rejection; `allow_extended_horizon` rejection; dense/global diffusion rejection; L2 reduction without L1 widening; PR8 feedstock preserves bounded horizon |
| One-system-per-cell | `validate_one_system_per_gridcell`; duplicate placement rejection; ordinary `Location` gridcells; forbidden kind tokens absent |
| Inert render positions | All gridcells carry `inert=` render metadata |
| Lattice topology | N4 links derived from `(row,col)` placements, not render coordinates |
| Forbidden semantics | Generated pack scan for route/path/predecessor/movement/border/frontline/cpu_planner/graph_engine vocabulary |
| PALMA posture | Field feedstock only; zero route/predecessor surfaces in expansion report |
| PR8 posture | Harness documents compact probe-only readback; no CPU planner vocabulary |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_pr1`–`mapgen_pr7` reports/guardrails | CURRENT_EVIDENCE / LIVE_GUARDRAIL / PROBATION (PR7) | Unchanged |
| `mapgen_pr8_scheduled_concurrency_results.md` | CURRENT_EVIDENCE | Unchanged |
| `mapgen_pr8_scheduled_concurrency.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_constitution_guards.rs` | LIVE_GUARDRAIL | New PR9 consolidated battery |
| `docs/tests/mapgen_pr9_constitution_guards_results.md` | PROBATION | This report |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

## Forbidden-boundary summary

- No authoritative Euclidean adjacency in active MapGen or PR8 scheduling source (pattern scan).
- L1 horizon remains bounded; horizon widening and dense/global diffusion rejected at admission.
- One system per gridcell enforced; gridcells remain ordinary `Location` SimThings.
- Render positions remain inert metadata; hyperlane links use lattice placements only.
- No route/path/predecessor/movement/border/frontline/cpu_planner surfaces in generated PR7 pack.
- No new GPU kernel, no simthing-sim changes, no PR10 end-to-end execution in this rung.

## Validation battery

```text
cargo fmt --all -- --check          PASS
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse    8 passed
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy   10 passed
cargo test -p simthing-clausething --test mapgen_resource_flow       16 passed
cargo test -p simthing-clausething --test mapgen_links               19 passed
cargo test -p simthing-clausething --test mapgen_movement_front      23 passed
cargo test -p simthing-clausething --test mapgen_palma               19 passed
cargo test -p simthing-clausething --test mapgen_constitution_guards 21 passed
cargo test -p simthing-clausething --test ct_scenario_container      45 passed
cargo test -p simthing-driver --test mapgen_pr8_scheduled_concurrency  6 passed
cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver   2 passed
git diff --check                    PASS
```

## DA sign-off status

**DA-APPROVED (2026-06-14, Opus / Design Authority)** after a genuine pre-merge audit + battery rerun. The
guard battery `mapgen_constitution_guards` is durable and is confirmed **LIVE_GUARDRAIL**. Only the Design
Authority writes a DA sign-off.

## PR10 readiness

**PR10 may now proceed** — end-to-end canonical sample (ingest → generate → admit/install → GPU compact
evidence) — under its own DA-review gate.
