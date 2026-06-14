# MapGen PR5 Hyperlane Link / Lane-Coupling Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (PR5 link/coupling report; DA-approved 2026-06-14).

## Verdict

**PASS / DA-APPROVED (2026-06-14, Opus / Design Authority)** — DA performed a genuine pre-merge audit
against the branch source (not the PR body), reran the battery green (`mapgen_links` 19, `mapgen_neutral_ast_parse`
8, `mapgen_lattice_hierarchy` 10, `mapgen_resource_flow` 16, `ct_scenario_container` 45; fmt/`git diff --check`
clean), and confirmed: N4 classification uses integer `(row,col)` `abs_diff` (no Euclidean / sqrt / render-position
authority; the test self-guards this); `mapgen::lane_coupling` is **inert authoring metadata only** (a `{from,to}`
record + an empty-`sub_fields` property, consumed by no driver/GPU/runtime, no route/path/predecessor, cannot
widen L1 horizon — field-propagation consumption deferred to later rungs); all four caps (links / lane-couplings /
per-node fanout / lane-coupling fanout) enforced; self-links + unknown endpoints hard-rejected; duplicates
canonicalized deterministically; no Movement-Front/SaturatingFlux/PALMA/FIELD_POLICY/runtime/GPU/driver/simthing-sim
output; no new `SimThingKind`; no Candidate F. **PR5 lowers five `add_hyperlane` declarations** from the tiny neutral-AST
fixture into three bounded N4 lattice links plus two bounded lane-coupling authoring properties on the
PR4-enrolled pack. Endpoints are validated; self-links and unknown endpoints are rejected; duplicates
canonicalize deterministically; link, lane-coupling, and per-node fanout caps are enforced; authored
Stellaris positions remain inert render metadata; no Euclidean adjacency authority; no
route/path/predecessor/movement/border/frontline vocabulary; no Movement-Front/SaturatingFlux/PALMA/
FIELD_POLICY/runtime/GPU/driver/simthing-sim output; no new `SimThingKind`.

## Track scope

0.0.8.2.5 MapGen PR5: bounded hyperlane-to-link and lane-coupling authoring (M6). **Merged after
genuine DA sign-off (Opus, 2026-06-14).**

PR5 lowers hyperlane-like declarations to bounded link/coupling metadata only. PR5 does not implement
pathfinding. PR5 does not implement movement. PR5 does not implement routes or predecessors. PR5 does
not implement border or frontline services. PR5 does not implement Movement-Front. PR5 does not
implement PALMA W/D. PR5 does not implement FIELD_POLICY commitments. PR5 does not touch
runtime/GPU/driver/simthing-sim. PR5 does not import the whole Stellaris corpus.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR5, §9
6. `docs/clausething/mapgen_corpus_manifest.md`
7. `docs/clausething/MapGenThing.md`
8. `docs/clausething/ct_vertical_consumer_contract.md`
9. `docs/clausething/ct_2c_economic_category_memo.md`
10. `docs/clausething/ct_3b_4a_movement_front_heatmap_memo.md`
11. `docs/tests/mapgen_pr1_corpus_manifest_results.md`
12. `docs/tests/mapgen_pr2_neutral_ast_results.md`
13. `docs/tests/mapgen_pr3_lattice_hierarchy_results.md`
14. `docs/tests/mapgen_pr3_da_audit_results.md`
15. `docs/tests/mapgen_pr4_resource_flow_results.md`
16. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| Link/coupling generator | `crates/simthing-clausething/src/mapgen_links.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| PR5 tests | `crates/simthing-clausething/tests/mapgen_links.rs` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| PR5 report | `docs/tests/mapgen_pr5_links_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_pr1_corpus_manifest_results.md` | CURRENT_EVIDENCE | Unchanged |
| `mapgen_pr2_neutral_ast_results.md` | CURRENT_EVIDENCE | Unchanged |
| `mapgen_pr3_lattice_hierarchy_results.md` | CURRENT_EVIDENCE | Unchanged |
| `mapgen_pr3_da_audit_results.md` | CURRENT_EVIDENCE | Unchanged |
| `mapgen_pr4_resource_flow_results.md` | CURRENT_EVIDENCE | Unchanged |
| `mapgen_neutral_ast_parse.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_lattice_hierarchy.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_resource_flow.rs` | LIVE_GUARDRAIL | Unchanged |
| `ct_scenario_container.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_links.rs` | CURRENT_EVIDENCE | New PR5 generator (DA-approved) |
| `mapgen_links.rs` (tests) | LIVE_GUARDRAIL | Promoted at DA approval |
| `docs/tests/mapgen_pr5_links_results.md` | CURRENT_EVIDENCE | This report; DA-approved |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

## Lowering summary (tiny pentad fixture)

| Hyperlane (authored) | Lattice classification | Output surface |
|---|---|---|
| `0` ↔ `9` | N4-adjacent | `HydratedScenarioGridMetadata.links` |
| `0` ↔ `2` | N4-adjacent | `HydratedScenarioGridMetadata.links` |
| `9` ↔ `15` | N4-adjacent | `HydratedScenarioGridMetadata.links` |
| `0` ↔ `31` | long-range | `mapgen::lane_coupling` inert property |
| `31` ↔ `15` | long-range | `mapgen::lane_coupling` inert property |

Adjacency authority uses PR3 row/col lattice placements only — not Stellaris `position` coordinates.

## Expansion report (default options)

| Field | Value |
|---|---|
| `link_count` | 3 |
| `max_links` | 8 |
| `max_per_node_fanout` (cap) | 4 |
| observed max per-node fanout | 3 (`0`) |
| `lane_coupling_count` | 2 |
| `max_lane_coupling_count` | 8 |
| `max_lane_coupling_fanout` (cap) | 4 |
| rejection counts | all zero on happy path |
| `unsafe_expansion_flags` | empty |

## Validation battery

```text
cargo fmt --all -- --check          PASS
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse   8 passed
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy  10 passed
cargo test -p simthing-clausething --test mapgen_resource_flow      16 passed
cargo test -p simthing-clausething --test mapgen_links              19 passed
cargo test -p simthing-clausething --test ct_scenario_container     45 passed
git diff --check                    PASS
```

Driver/spec tests not required — no spec-facing admission API changes outside `simthing-clausething`.

## Deferred rejection cases

None for PR5 scope. All required rejection tests are reachable via `lower_hyperlane_topology` with
injected hyperlane lists and tightened caps.

## Governance

**Genuine DA sign-off (Opus / Design Authority, 2026-06-14): APPROVE — no fix needed.** PR6 may proceed
(subject to its own DA-review gate; only the Design Authority writes a DA sign-off).
