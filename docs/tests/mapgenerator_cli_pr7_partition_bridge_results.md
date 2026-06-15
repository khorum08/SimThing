# MapGeneratorCLI PR7 — Partition / Bridge Structural Producer + Clustering Results

> **Artifact lifecycle: PROBATION** (pending DA approval after independent branch-source audit).

## Verdict

**PROBATION — pending DA review.** Adds bounded producer-side partition/cluster assignment and cross-group bridge
endpoint selection represented **only** as `static_galaxy_scenario` `add_hyperlane` declarations. Generated output
parses and lowers through existing closed MapGen lattice/link surfaces without front-end widening. **Zero**
`crates/simthing-clausething/src/` changes. No new grammar, no route/path/predecessor/movement/border/frontline
semantics, field operators, RF, Movement-Front, PALMA, driver/GPU, simthing-sim, new `SimThingKind`, Euclidean
authority, or FIELD-MOVIE-DATASET-0 work.

## Track scope

0.0.8.6 MapGeneratorCLI PR7 adds partition/bridge structural producer + clustering over generated placements.
**0.0.8.2.5 MapGen remains closed and is not reopened.** PR8 (remaining vanilla shape registry completion) is **not**
started in this PR.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgenerator_cli_pr1_params_results.md` through `pr6b` | CURRENT_EVIDENCE | Unchanged — preserved |
| `docs/tests/mapgenerator_cli_pr7_partition_bridge_results.md` | PROBATION | New (this report) |
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| Partition assignment + bridge selection | `crates/simthing-mapgenerator/src/partition.rs` |
| Cluster assignment + cluster bridges | `crates/simthing-mapgenerator/src/cluster.rs` |
| Structure pipeline wiring | `crates/simthing-mapgenerator/src/lib.rs` |
| Partition tests | `crates/simthing-mapgenerator/tests/partition.rs` |
| Cluster tests | `crates/simthing-mapgenerator/tests/cluster.rs` |
| Integration lowering proof | `crates/simthing-clausething/tests/mapgenerator_cli_partition_bridge_lower.rs` |
| Ladder + production track | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md`, `docs/design_0_0_8_1_clausething_production_track.md` |
| PR7 results (this report) | `docs/tests/mapgenerator_cli_pr7_partition_bridge_results.md` |

## Partition model summary

Producer types: `PartitionKind` (`HomeSystemPartition`, `OpenSpacePartition`, `ClusterPartition` reserved in enum),
`PartitionId`, `PartitionAssignment`, `PartitionOptions`, `PartitionReport`, `PartitionError`, `BridgeEdge`.

`assign_partitions` deterministically buckets systems into home/open partitions using BFS/DFS order on lowered
index-order grid adjacency, enforcing `partition_min_systems` / `partition_max_systems` fail-closed.

`generate_partition_bridges` selects bounded cross-partition endpoint pairs, deduplicating against existing
hyperlane/special-route edges and respecting per-node fanout cap.

## Cluster model summary

Producer types: `ClusterId`, `ClusterAssignment`, `ClusterOptions`, `ClusterReport`, `ClusterError`,
`ClusterBridgeEdge`.

`assign_clusters` groups systems by nearest anchor among the first N sorted system ids using integer Chebyshev
distance on authored lattice coords (inert metadata — not Euclidean authority). `cluster_radius` is a producer-side
integer cap; impossible assignments fail closed.

`generate_cluster_bridges` selects bounded cross-cluster endpoint pairs for `add_hyperlane` emission.

## Bridge-selection summary

`place_and_emit_scenario_with_structure` merges hyperlanes, special routes, partition bridges, and cluster bridges
into one `HyperlaneTopology` for existing PR6 emitter output. Partition/cluster identities appear in producer reports
only.

## add_hyperlane-only emission summary

Output contains only `system = { ... }` blocks and `add_hyperlane = { from = "…" to = "…" }` — no `partition`,
`cluster`, `bridge`, `route`, `path`, or predecessor grammar.

## Parse / lattice / link lowering proof

Integration pipeline (test harness injects deposit block so PR4 RF enrollment succeeds — test-only):

```text
MapGeneratorCLI static placement
→ assign_partitions + assign_clusters
→ generate_partition_bridges + generate_cluster_bridges
→ static_galaxy_scenario add_hyperlane emission
→ parse_mapgen_neutral_document
→ generate_mapgen_lattice_hierarchy
→ generate_mapgen_resource_flow_enrollment
→ generate_mapgen_links
→ bounded links / lane_couplings evidence
```

## Closed-source gate result

**PASS** — `git diff --name-only master...HEAD` (after commit) excludes forbidden closed `src/` paths; no front-end widening.

Expected allowed paths only:

- `crates/simthing-mapgenerator/src/partition.rs`
- `crates/simthing-mapgenerator/src/cluster.rs`
- `crates/simthing-mapgenerator/src/lib.rs`
- `crates/simthing-mapgenerator/tests/partition.rs`
- `crates/simthing-mapgenerator/tests/cluster.rs`
- `crates/simthing-clausething/tests/mapgenerator_cli_partition_bridge_lower.rs`
- `docs/tests/mapgenerator_cli_pr7_partition_bridge_results.md`
- `docs/design_0_0_8_1_clausething_production_track.md`
- `docs/design_0_0_8_6_mapgenerator_cli_ladder.md`

## Forbidden semantics scan summary

Emitted scenario text contains no partition/cluster/bridge/route/path/predecessor/movement/border/frontline/
field_operator/RF/Movement-Front/PALMA/driver/GPU surfaces.

## Dependency boundary

- `simthing-clausething` dev-depends on `simthing-mapgenerator` for integration tests only.
- `simthing-mapgenerator` does **not** depend on `simthing-clausething` or other forbidden crates.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgenerator_cli_partition_bridge_lower
cargo test -p simthing-clausething --test mapgen_links
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
git diff --name-only master...HEAD
```

## Test results

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-mapgenerator` | PASS (114 tests) |
| `cargo test -p simthing-clausething --test mapgenerator_cli_partition_bridge_lower` | PASS (6 tests) |
| `cargo test -p simthing-clausething --test mapgen_links` | PASS (19 tests) |
| `cargo test -p simthing-clausething --test mapgen_neutral_ast_parse` | PASS (8 tests) |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS (10 tests) |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | PASS (21 tests) |
| `git diff --check` | PASS |
| closed-src gate | PASS (no forbidden `src/` paths) |

## DA sign-off status

**PROBATION — pending DA approval.** No executive sign-off yet.

## Whether PR8 may proceed

**No — await DA approval of PR7.** After DA approves this rung, **PR8** (remaining vanilla shape registry /
executable strategy dispatch) may proceed per ladder ordering.
