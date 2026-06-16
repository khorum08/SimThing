# MAPGENCLI-TOPOLOGY-STEAD-1 — results (sibling producer couplings on authored coordinates)

**Classification: PROBATION until DA approval.** Owner-directed follow-up to MAPGENCLI-TOPOLOGY-STEAD-0:
finish migrating the remaining producer adjacency heuristics off lowered index-order coordinates, and add an
integrity guard proving the generated map is spatially dispersed (not a brick).

## Artifact lifecycle audit

| Artifact | Classification | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated (guardrail row broadened + PROBATION amendment row) |
| MAPGENCLI-TOPOLOGY-STEAD-0 | CURRENT_EVIDENCE pending DA | `topology.rs` base hyperlanes (PR #706) |
| `crates/simthing-mapgenerator/tests/topology_stead.rs` | PROBATION | +1 dispersion guard (9 tests total) |
| `crates/simthing-mapgenerator/tests/special_routes.rs` | PROBATION | 1 fail-closed test updated for authored-coord semantics |
| `docs/tests/mapgenerator_cli_topology_stead_1_results.md` | PROBATION | This report |
| `docs/tests/mapgenerator_cli_visual_spiral_1500.png` | unchanged | The BaseOnly preview draws only base hyperlanes (STEAD-0); sibling couplings are not rendered, so the PNG is byte-identical |

## Scope
Three sibling producer modules carried the **identical** index-order drift
(`lowered_grid_position(index, fixture_lattice_edge)`) as `topology.rs` did before STEAD-0. All emit
**visualized** `add_hyperlane` couplings, so the drift was the same severity. STEAD-0 deliberately left them
out to keep that PR reviewable; this PR finishes the job.

## Old → new behavior
| Module | Coupling | Old position source | New position source |
|---|---|---|---|
| `special_routes.rs:137` | long-range wormhole / gateway | `lowered_grid_position(i, fixture_edge)` | `(coord.col, coord.row)` |
| `partition.rs:251` | partition bridges | `lowered_grid_position(i, fixture_edge)` | `(coord.col, coord.row)` |
| `partition.rs:394` | BFS/DFS partition ordering adjacency | `lowered_grid_position(i, fixture_edge)` | `(coord.col, coord.row)` |
| `cluster.rs:229` | cluster bridges | `lowered_grid_position(i, fixture_edge)` | `lattice_coord(system)` = `(coord.col, coord.row)` |

Notes:
- `cluster.rs`'s *assignment* phase already used authored `lattice_coord`; only its *bridge* phase had
  drifted — now consistent.
- `partition.rs` BFS/DFS ordering still mirrors Stellaris `home_system_partitions { method }` and remains
  **offline ordering, not runtime pathfinding** (no source→target / predecessors / shortest-path) — only its
  adjacency is now over authored coords.
- `fixture_lattice_edge` is retained on each options struct for back-compat / fail-closed validation but no
  longer drives any candidate coordinates. All caps, fanout, dedup, self-link/unknown-endpoint rejection,
  prevent/occupied filtering, and fail-closed count checks are preserved. No sqrt; integer Chebyshev only.
- `lowered_grid_position` is now unused by every producer pass (kept as `pub` API; harmless).

## Behavior change requiring a test update (expected, correct)
`special_routes_fail_closed_when_count_impossible` used `grid_placement(2)` (systems on the diagonal
`(0,0)`,`(1,1)`). Under the old index-order mapping those lowered to N4-adjacent cells `(0,0)`,`(0,1)` →
skipped as too-close → 0 candidates → `UnsatisfiedRouteCount`. Under authored coords they are **diagonal**
(non-N4), which is a *valid* long-range candidate, so a wormhole is now correctly selectable. The test was
updated to an explicitly **N4-adjacent** 2-system placement (`(0,0)`,`(1,0)`) so the genuinely-impossible
fail-closed path is still exercised. This is a correctness improvement (special routes connect systems that
are far in the *authored* layout), not a regression.

## Integrity guard (answers the "is it a brick / is it really 1500 stars?" concern)
New `spiral_1500_placement_is_spatially_dispersed_not_a_brick` asserts, on the real 1500-star/300-edge spiral:
- 1500 systems, **1500 distinct coords** (one system per cell);
- fill ratio `< 0.25` of the bounding box (measured **0.026** — a brick would be ~1.0);
- max stars in any single row `< bbox_width / 4` (measured **15** vs bbox width 241; a brick row would hold
  ~241);
- bounding box spans ≥ 100×100 cells (measured **241×241** on the 300 lattice).

The renderer draws each star at `rendered_star_pixel(seed, id, coord, …)` = `cell_center_pixel(coord)` +
sub-cell jitter, so the rendered position **is** the authored gridcell coordinate. The "looks like more than
1500 stars" impression is a density illusion: 1500 small bright discs concentrated onto thin spiral arms on a
black field. The placement is honest and dispersed — confirmed by the guard above.

## Tests
- `tests/topology_stead.rs`: 8 → **9** (added the dispersion guard).
- `tests/special_routes.rs`: fail-closed test re-fixtured for authored-coord semantics.

## Commands run
```
cargo fmt --all -- --check                                              # clean
cargo test -p simthing-mapgenerator                                     # all green (20 files)
cargo test -p simthing-clausething --test stead_spatial_contract_guards # 11 green
git diff --check                                                        # clean
```
GPU tests not run (out of scope). Full workspace not run (no shared-crate wiring changed). PNG not
regenerated (base-only preview is unchanged by sibling couplings).

## Files changed
- `crates/simthing-mapgenerator/src/special_routes.rs`, `partition.rs`, `cluster.rs` (authored-coord positions)
- `crates/simthing-mapgenerator/tests/topology_stead.rs` (dispersion guard)
- `crates/simthing-mapgenerator/tests/special_routes.rs` (fail-closed test re-fixtured)
- `docs/clausething/MapGeneratorCLI.md` (§4.3 note broadened to all couplings + dispersion)
- `docs/tests/current_evidence_index.md`, `docs/tests/mapgenerator_cli_topology_stead_1_results.md`

## DA status
**DA-APPROVED 2026-06-15 (owner sign-off).** The owner — design authority for the MapGeneratorCLI/Mapping
track — reviewed and approved; PROBATION cleared → CURRENT_EVIDENCE in `current_evidence_index.md`.
