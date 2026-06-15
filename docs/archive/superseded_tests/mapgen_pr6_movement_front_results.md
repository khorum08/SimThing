# MapGen PR6 Movement-Front Authoring Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (PR6 Movement-Front report; DA-approved 2026-06-13).

## Verdict

**PASS / DA-APPROVED (2026-06-13, Opus / Design Authority)** — DA performed a genuine pre-merge audit
against the branch source (not the PR body), reran the battery green (`mapgen_movement_front` 23,
`mapgen_neutral_ast_parse` 8, `mapgen_lattice_hierarchy` 10, `mapgen_resource_flow` 16, `mapgen_links`
19, `ct_scenario_container` 45; fmt/`git diff --check` clean), and confirmed: L1 is bounded local
`SaturatingFlux`/`RegionFieldSpec` only (horizon 4, cap 8, `allow_extended_horizon: false`; rejects
Normalized/Gradient/dense-global profiles); L2 is `RegionFieldReductionSpec` slot-range hierarchy feedstock
that does not widen L1 horizon; L3 is `FirstSliceCommitmentSpec`/`HydratedScenarioCommitment` threshold
feedstock, not CPU planning; PR4 suppression-arena `ArenaPressureBindingSpec` binds `mapgen_suppression::flow`
onto PR3 gridcell placements (not Stellaris positions); pack explicitly clears PALMA/W/stress compose;
forbidden route/path/predecessor/movement/border/frontline/cpu_planner vocabulary guarded; no sqrt/distance/
Euclidean authority in generator source; no runtime/GPU/driver/simthing-sim file changes; no new
`SimThingKind`; no semantic WGSL. **PR6 lowers the PR5-enrolled tiny MapGen slice into existing
Movement-Front authoring feedstock only.**

## Track scope

0.0.8.2.5 MapGen PR6: Movement-Front L1/L2/L3 authoring/lowering (core §7). **Merged after genuine DA
sign-off (Opus, 2026-06-13).**

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
| `mapgen_neutral_ast_parse.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_lattice_hierarchy.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_resource_flow.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_links.rs` | LIVE_GUARDRAIL | Unchanged |
| `ct_scenario_container.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_movement_front.rs` | CURRENT_EVIDENCE | New PR6 generator (DA-approved) |
| `mapgen_movement_front.rs` (tests) | LIVE_GUARDRAIL | Promoted at DA approval |
| `docs/tests/mapgen_pr6_movement_front_results.md` | CURRENT_EVIDENCE | This report; DA-approved |
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

**Genuine DA sign-off (Opus / Design Authority, 2026-06-13): APPROVE — no fix needed.** PR7 may proceed
(subject to its own DA-review gate; only the Design Authority writes a DA sign-off).
