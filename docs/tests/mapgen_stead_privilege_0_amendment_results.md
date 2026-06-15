# MapGen 0.0.8.2.5 Amendment — STEAD-PRIVILEGE-0 (gridcell positions are structural-spatial)

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-authorized closed-lowerer amendment, 2026-06-15, executive design authority).

## Verdict

**PASS — DA-AUTHORIZED 0.0.8.2.5 AMENDMENT.** Corrects a drift: the closed `mapgen_lattice` lowerer assigned
gridcell `(row,col)` by **emission index**, discarding the emitted spatial position. This violated **core §7
(STEAD)** — "the parent lays out its child Locations *positionally* as a grid map… a cell is shaped by its
neighbors — falloff is the spatial arena's flow" — and §0.6 (silent flattening of the specified spatial
distribution to an index-order proxy). The amendment **privileges the STEAD mapping constitution**: the lowerer
now **honors the emitted integer position as the authoritative gridcell coordinate**, so the generated galactic
pattern (spiral, ring, …) **is** the lattice the Movement-Front automaton runs on.

## Root cause (the drift, owned)

- `mapgen_lattice.rs::assign_system_placements`: `row = index/edge, col = index%edge` — index-order row-major fill,
  ignoring the emitted `position`; the position survived only as inert `mapgen_render_x/y` metadata.
- Compounded by a DA mis-ruling (PR5/PR10/PR11 reviews) that cited §0.7 (Candidate F) to declare positions "inert
  / index-order is constitutional." **§0.7 governs decision-gate *magnitudes* (no float `sqrt`/distance gates a
  commitment) — not the spatial layout.** Integer `(col,row)` placement is **not** Euclidean authority; the stencil
  walks neighbors by **integer index arithmetic** (§7). The two were conflated.

## The change (closed `src/`, DA-authorized)

`crates/simthing-clausething/src/mapgen_lattice.rs`:
- `assign_system_placements(systems) -> (Vec<SystemPlacement>, edge)` now: parses each system's emitted
  `position.x → col`, `position.y → row` (integers; Stellaris-centered negatives allowed), **translates by the min
  to a 0-based sparse lattice** (pure integer shift — preserves spatial structure, adds **zero** Euclidean
  authority), enforces one-system-per-cell at the **authored** cell, and **derives the lattice edge from the
  position bounding box** (validated ≤ `MAPGEN_MAX_LATTICE_EDGE` 256; §7-canonical 200 fits). `generate_mapgen_lattice_hierarchy`
  uses the derived edge for `grid_metadata`.
- New guard: `parse_gridcell_axis` rejects non-integer coordinates.

## Tests

- **New guard test** `mapgen_lattice_hierarchy::authored_positions_drive_gridcell_placement`: asserts
  `grid_metadata (col,row) == emitted position` for every system, and that it is **not** an emission-order
  row-major fill. `lattice_edge_is_derived_from_authored_positions_not_the_option` asserts the edge follows the
  positions, not the requested option.
- **Fixture** `tests/fixtures/mapgen/tiny_pentad_hub_slice_raw.clause` rescaled from Stellaris-centered span-60
  coords to a compact **edge-7 spatial pentad** (hub + N4 spokes + one far corner) — a genuine ≤10 first-slice
  spatial slice that fits the §7 Movement-Front execution cap while honoring positions. Dependent value assertions
  (links 3/2 split, render-inert values, movement-front `grid_size`/reduction slots 49, palma `grid_size`, parse
  position) updated to the new positions.

## Scope (honest)

- **Spatial LAYOUT is now correct** for any scenario ≤256 edge, including the §7-canonical 200×200.
- **Movement-Front EXECUTION** remains the implemented first slice (≤10/32 cells per edge, §7); a larger sparse
  galaxy *lowers* with correct layout, but *running* the stencil at galaxy scale is the parked multi-theater/atlas
  rung — unchanged. This is **separate** from the RF participant/slot-cap admit limit (the other parked amendment).

## Battery (rerun locally)

`cargo fmt --all --check` clean. `cargo test -p simthing-clausething`: all binaries green — `mapgen_lattice_hierarchy`
10, `mapgen_links` 19, `mapgen_movement_front` 23, `mapgen_palma` 19, `mapgen_resource_flow` 16,
`mapgen_constitution_guards` 21, `mapgen_neutral_ast_parse` 8, plus the generated-scenario integration tests
(`mapgenerator_cli_pr5/pr6/partition_bridge/special_routes`). `cargo test -p simthing-mapgenerator` zero failures.
`cargo test -p simthing-driver --test mapgen_pr10_end_to_end_compact_evidence --test mapgen_pr8_scheduled_concurrency`
green (real GPU adapter).

## Records corrected

ADR **D6** (`adr/ClauseThingADR.md`), the clearinghouse (`clausething/ClauseThingDoc.md`), and the constitution
addendum (`design_0_0_8_3.md` §A gate 4) — the prior "positions inert / index-order is constitutional / shape is
cosmetic" claims are withdrawn and replaced with the §7 structural-spatial rule.

## DA sign-off

**DA-AUTHORIZED & SIGNED — 2026-06-15, executive design authority (STEAD-PRIVILEGE-0).** Closed-track amendment;
restores §7 STEAD; zero new Euclidean authority; full battery green.
