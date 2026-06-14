# MapGen PR6 Movement-Front Authoring Results

> **Artifact lifecycle: PROBATION** (PR6 Movement-Front report; pending DA review before merge promotion).

## Verdict

**PASS pending DA review** — PR6 lowers the PR5-enrolled tiny MapGen slice into existing Movement-Front
authoring feedstock only: L1 bounded `SaturatingFlux`/`RegionFieldSpec` with suppression-arena
`ArenaPressureBindingSpec`; L2 `RegionFieldReductionSpec` hierarchy feedstock; L3
`FirstSliceCommitmentSpec`/`HydratedScenarioCommitment` threshold feedstock. No PALMA, no driver/GPU
runtime execution, no pathfinding/movement/route/predecessor/border/frontline semantics, no Euclidean
authority, no new `SimThingKind`, no runtime/GPU/driver/simthing-sim file changes.

## Track scope

0.0.8.2.5 MapGen PR6: Movement-Front L1/L2/L3 authoring/lowering (core §7). **Do not merge until DA
review.**

PR6 generates Movement-Front authoring/lowering feedstock only. PR6 uses existing GPU-resident
field/reduction/threshold surfaces. PR6 does not add new GPU kernels. PR6 does not execute driver/GPU
path. PR6 does not generate PALMA. PR6 does not implement pathfinding/movement/routes/predecessors. PR6
does not implement border/frontline services. PR6 does not touch runtime/GPU/driver/simthing-sim. PR6 does
not import the whole Stellaris corpus.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR6, §9
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
19. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| Movement-Front generator | `crates/simthing-clausething/src/mapgen_movement_front.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| PR6 tests | `crates/simthing-clausething/tests/mapgen_movement_front.rs` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| PR6 report | `docs/tests/mapgen_pr6_movement_front_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_pr1`–`mapgen_pr5` reports/guardrails | CURRENT_EVIDENCE / LIVE_GUARDRAIL | Unchanged |
| `mapgen_movement_front.rs` | PROBATION | New PR6 generator |
| `mapgen_movement_front.rs` (tests) | PROBATION | New PR6 guardrail battery |
| `docs/tests/mapgen_pr6_movement_front_results.md` | PROBATION | This report |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

## L1/L2/L3 lowering summary (tiny pentad fixture)

| Layer | Surface | Binding |
|---|---|---|
| L1 | `RegionFieldSpec` + `SaturatingFlux` on 3×3 lattice, horizon 4 (≤ 8 cap) | `ArenaPressureBindingSpec` from `mapgen_suppression::flow` onto five gridcell placements |
| L2 | `RegionFieldReductionSpec` (`gridcell_lattice_to_sector_parent`) | Slot-range sum feedstock; does not widen L1 horizon |
| L3 | `HydratedScenarioCommitment` + embedded `FirstSliceCommitmentSpec` | Threshold 0.75 / event_kind 7; default-off mapping profile preserved |

## Expansion report (default options)

| Field | Value |
|---|---|
| `l1_field_operator_count` | 1 |
| `l1_horizon` | 4 |
| `l1_locality_bound` | 4 |
| `l2_reduction_count` | 1 |
| `l2_reduction_scope` | `gridcell_lattice_to_sector_parent` |
| `l3_commitment_count` | 1 |
| `l3_thresholds` | `[0.75]` |
| `generated_columns` | source 0, choke 2, urgency 4 |
| `rf_source_bindings` | `mapgen_suppression::flow` |
| `forbidden_surface_count` | 0 |
| `unsafe_expansion_flags` | `l2_reduction_spans_full_lattice` (advisory; expected for 3×3 lattice) |

## Validation battery

```text
cargo fmt --all -- --check          PASS
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse    8 passed
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy   10 passed
cargo test -p simthing-clausething --test mapgen_resource_flow       16 passed
cargo test -p simthing-clausething --test mapgen_links               19 passed
cargo test -p simthing-clausething --test mapgen_movement_front      23 passed
cargo test -p simthing-clausething --test ct_scenario_container      45 passed
git diff --check                    PASS
```

Driver/spec tests not required — no spec-facing admission API changes outside `simthing-clausething`.

## Governance

Only the Design Authority writes a DA sign-off. PR7 may proceed only after DA approval of PR6.
