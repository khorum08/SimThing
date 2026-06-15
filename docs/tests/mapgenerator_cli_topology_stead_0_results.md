# MAPGENCLI-TOPOLOGY-STEAD-0 — results (select hyperlanes from authored coordinates)

**Classification: PROBATION until DA approval.** Owner-forwarded Codex handoff to fix the remaining
producer-side topology drift flagged as the open follow-up after STEAD-CONTRACT-0/0R.

## Artifact lifecycle audit

| Artifact | Classification | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated (new guardrail row + PROBATION amendment row) |
| STEAD-CONTRACT-0 / 0R | CURRENT_EVIDENCE | Unchanged (DA-APPROVED 2026-06-15) |
| `crates/simthing-mapgenerator/tests/topology_stead.rs` | PROBATION | New live regression guard for this change |
| `docs/tests/mapgenerator_cli_topology_stead_0_results.md` | PROBATION | This report |
| `docs/tests/mapgenerator_cli_visual_spiral_1500.png` | PROBATION visual evidence | Regenerated through the fixed topology (old `713b22d2…` → new `3deb44db…`) |
| `docs/tests/mapgenerator_cli_visual_spiral_1500_results.md` | unchanged | Preset/command unchanged; only the rendered artifact was refreshed |

No superseded reports restored; no duplicate failed previews created. The prior PNG is overwritten in place
(git retains the old bytes), not archived — the new render supersedes it and a side-by-side is not needed.

## Old behavior (the bug)
`crates/simthing-mapgenerator/src/topology.rs::generate_hyperlane_topology` chose candidate hyperlane pairs
from **lowered index-order** positions:
```
positions[i] = lowered_grid_position(i, options.fixture_lattice_edge)  // (i / edge, i % edge)
```
After STEAD-PRIVILEGE-0 / STEAD-CONTRACT-0, a generated system's `coord: LatticeCoord` **is** the authoritative
structural gridcell coordinate (the emitter writes `position = { x = coord.col, y = coord.row }`, which the
closed lowerer honors). So the producer connected systems that were near in **emission order**, not near in the
generated spiral/ring/etc. layout — wrong base network and (pre-render-cap) long index-order artifacts.

## New behavior (the fix)
Candidate positions now come from authored coordinates:
```
positions[i] = (systems[i].coord.row, systems[i].coord.col)  // authored structural (col,row)
```
- Deterministic ordering, bounded Chebyshev candidate generation, candidate cap, fanout caps, prevent-pair
  filtering, duplicate/self-link rejection, and min/max/target edge-count checks are all **preserved**.
- `fixture_lattice_edge` is retained on `HyperlaneOptions` for back-compat / fail-closed validation but is
  **deprecated for adjacency** and no longer computes candidate coordinates (doc-noted on the field).
- No pathfinding/routes/predecessors/movement orders; no Euclidean sqrt — adjacency is integer Chebyshev only.
- `add_hyperlane` endpoint-pair output and the closed lowering path are unchanged.

## Tests added (`tests/topology_stead.rs`, 8) + 1 updated
- `hyperlane_candidates_use_authored_structural_coords_not_index_order`
- `near_in_authored_coords_gets_candidate_even_if_far_in_index_order`
- `far_in_authored_coords_does_not_get_candidate_even_if_adjacent_in_index_order`
- `generated_edges_respect_authored_chebyshev_distance_bound`
- `spiral_1500_base_hyperlanes_have_no_self_links`
- `spiral_1500_base_hyperlanes_have_no_duplicate_undirected_links`
- `spiral_1500_base_hyperlanes_have_no_unknown_endpoints`
- `spiral_1500_base_hyperlanes_are_local_in_authored_grid`

The decorrelated 6-system fixture places A(0,0)/B(1,0) authored-adjacent but at emission indices 0/5, and
A(0,0)/X(20,0) emission-adjacent but authored-far — proving selection follows authored coords, asserted through
the public selected-edge output (there is no public candidate list). Updated:
`tests/topology.rs::hyperlane_generation_respects_max_hyperlane_distance` now asserts the bound on authored
coordinates (it previously asserted against `lowered_grid_position`).

## Commands run
```
cargo fmt --all -- --check                                              # clean
cargo test -p simthing-mapgenerator                                     # all green
cargo test -p simthing-clausething --test stead_spatial_contract_guards # 11 green
git diff --check                                                        # clean
cargo run -p simthing-mapgenerator --bin mapgen -- --spiral-visual      # regenerated the PNG
```
GPU tests not run (out of scope). Full workspace not run (no shared-crate wiring changed).

## PNG regenerated?
**Yes.** `cargo run -p simthing-mapgenerator --bin mapgen -- --spiral-visual` rewrote
`docs/tests/mapgenerator_cli_visual_spiral_1500.png` (1000×1000, 10 base hyperlane segments). Visual check:
hyperlanes are local to the spiral, no long index-order diagonal/polyline artifacts, stars remain render-only
jittered, no gridlines. (The visual preview already applied a placed-coordinate Chebyshev cap at render time;
this fix makes the underlying **selection** correct, so the cap is now belt-and-suspenders rather than the sole
guard against the drift.)

## Files changed
- `crates/simthing-mapgenerator/src/topology.rs` (position source + module/field docs)
- `crates/simthing-mapgenerator/tests/topology_stead.rs` (new)
- `crates/simthing-mapgenerator/tests/topology.rs` (one assertion migrated to authored coords)
- `docs/clausething/MapGeneratorCLI.md` (§4.3 STEAD callout)
- `docs/tests/current_evidence_index.md` (ledger)
- `docs/tests/mapgenerator_cli_topology_stead_0_results.md` (this report)
- `docs/tests/mapgenerator_cli_visual_spiral_1500.png` (regenerated)

## Follow-up flagged (NOT in this PR's scope)
The **same index-order drift** lives in three sibling producer modules that also emit visible `add_hyperlane`
couplings: `cluster.rs:230`, `partition.rs:252` / `:395`, and `special_routes.rs:137` all build positions via
`lowered_grid_position(index, fixture_lattice_edge)`. They share this fix's exact correction but carry their
own integration tests that need re-evaluation. Per the handoff's "avoid broad churn" scope, they are **left for
an immediate follow-up (suggested: MAPGENCLI-TOPOLOGY-STEAD-1)** rather than expanded into this PR. Recommended
priority: high — cluster/partition/special-route couplings are visualized and currently select on emission
order.

## DA status
**PROBATION — DA sign-off pending.** No approval pre-filed.
