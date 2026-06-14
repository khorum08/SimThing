# MapGen PR7 PALMA W/D Reach Feedstock Results

> **Artifact lifecycle: PROBATION** (PR7 PALMA report; pending MapGen closeout promotion).

## Verdict

**PASS** — PR7 lowers the PR6-enrolled tiny MapGen slice into existing PALMA W/D authoring feedstock only:
`HydratedScenarioPalmaFeedstock` with W bound to PR6 `mapgen_suppression_front` SaturatingFlux choke column
(col 2), W output col 3, D output col 4; generic `WImpedanceComposeSpec` admits through existing preview
compiler. No routes/paths/predecessors/movement orders/destination plans, no driver/GPU runtime execution,
no Euclidean authority, no new `SimThingKind`, no runtime/GPU/driver/simthing-sim file changes. **Stayed inside
pre-adjudicated M7 boundary — no DA escalation required.**

## Track scope

0.0.8.2.5 MapGen PR7: PALMA W/D reach feedstock (M7).

PR7 generates PALMA W/D reach feedstock only. PR7 uses existing PALMA/min-plus authoring surfaces. PR7 does
not execute driver/GPU path. PR7 does not generate routes, paths, predecessors, or movement orders. PR7 does
not implement pathfinding. PR7 does not implement border/frontline services. PR7 does not touch
runtime/GPU/driver/simthing-sim. PR7 does not add new GPU kernels. PR7 does not import the whole Stellaris
corpus.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR7, §8, §9
6. `docs/design_0_0_8_1_border_hack_track.md`
7. `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md`
8. `docs/clausething/mapgen_corpus_manifest.md`
9. `docs/clausething/MapGenThing.md`
10. `docs/clausething/ct_vertical_consumer_contract.md`
11. `docs/clausething/ct_2c_economic_category_memo.md`
12. `docs/clausething/ct_3b_4a_movement_front_heatmap_memo.md`
13. `docs/tests/mapgen_pr1_corpus_manifest_results.md`
14. `docs/tests/mapgen_pr2_neutral_ast_results.md`
15. `docs/tests/mapgen_pr3_lattice_hierarchy_results.md`
16. `docs/tests/mapgen_pr3_da_audit_results.md`
17. `docs/tests/mapgen_pr4_resource_flow_results.md`
18. `docs/tests/mapgen_pr5_links_results.md`
19. `docs/tests/mapgen_pr6_movement_front_results.md`
20. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| PALMA feedstock helper | `crates/simthing-clausething/src/hydrate_palma_feedstock.rs` |
| PALMA generator | `crates/simthing-clausething/src/mapgen_palma.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| PR7 tests | `crates/simthing-clausething/tests/mapgen_palma.rs` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| PALMA guide | `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` |
| PR7 report | `docs/tests/mapgen_pr7_palma_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_pr1`–`mapgen_pr6` reports/guardrails | CURRENT_EVIDENCE / LIVE_GUARDRAIL | Unchanged |
| `mapgen_palma.rs` | PROBATION | New PR7 generator |
| `mapgen_palma.rs` (tests) | PROBATION | New PR7 guardrail battery |
| `docs/tests/mapgen_pr7_palma_results.md` | PROBATION | This report |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

## PALMA lowering summary (tiny pentad fixture)

| Surface | Binding |
|---|---|
| `HydratedScenarioPalmaFeedstock` (`mapgen_pentad_wd`) | `w_source` = `mapgen_suppression_front`; choke col 2; W col 3; D col 4 |
| `WImpedanceComposeSpec` | Composes W from PR6 source col 0 + choke col 2 → W col 3 |
| Mapping profile | `Disabled` (default-off) |

## Expansion report (default options)

| Field | Value |
|---|---|
| `palma_feedstock_count` | 1 |
| `w_source_field_operator_id` | `mapgen_suppression_front` |
| `w_source_column` | 2 (choke) |
| `w_output_column` | 3 |
| `d_output_column` | 4 |
| `grid_size` | 3 |
| `n_dims` | 6 |
| `source_col` | 0 |
| `choke_output_col` | 2 |
| `default_off_status` | true |
| `route_surface_count` | 0 |
| `predecessor_surface_count` | 0 |
| `unsafe_expansion_flags` | empty |

## Validation battery

```text
cargo fmt --all -- --check          PASS
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse    8 passed
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy   10 passed
cargo test -p simthing-clausething --test mapgen_resource_flow       16 passed
cargo test -p simthing-clausething --test mapgen_links               19 passed
cargo test -p simthing-clausething --test mapgen_movement_front      23 passed
cargo test -p simthing-clausething --test mapgen_palma             19 passed
cargo test -p simthing-clausething --test ct_scenario_container      45 passed
git diff --check                    PASS
```

Driver/spec tests not required — no spec-facing admission API changes outside `simthing-clausething`.

## DA escalation

None required. PR7 stayed inside the pre-adjudicated M7 PALMA feedstock boundary: W/D field metadata only,
no route/predecessor/path/movement semantics, no Euclidean authority, no driver/GPU/runtime changes, no new
spec/runtime types, no new `SimThingKind`.

## Governance

PR8 (Gu-Yang ∥ PALMA scheduled-concurrency spike) may proceed under its DA-review gate.
