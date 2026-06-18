# STEAD / Mapping spatial contract (normative)

> **Status: BINDING CONTRACT (STEAD-CONTRACT-0, 2026-06-15, executive design authority).** STEAD/Mapping
> is a central pillar of SimThing (see `simthing_core_design.md` → "Spatial substrate: STEAD/Mapping is not
> optional" and §7, and the transient constitution `design_0_0_8_3.md` §0.8 — a **carry-forward** clause
> that, with this pointer, every future constitution version must propagate verbatim). This contract is
> **mandatory reading** (`agents.md`) for any task touching MapGen,
> MapGeneratorCLI, Location grids, Movement-Front, STEAD, heatmaps, falloff, PALMA, Gu-Yang/SaturatingFlux,
> Resource Flow or Accumulator arenas over Location participants, field visualization, or spatial dynamics.
> Short, normative, hard to misread. Three catastrophic drifts (positions-inert, dense-global, edge-cap)
> are the reason it exists.

## Terms (defined once, used everywhere)
- **StructuralGridFrame** — the structural spatial extent of a gridcell lattice (`width`, `height`, `occupied_cells`), derived from the authoritative `(row,col)` placements. The substrate spatially-bound surfaces index *through*. (Code: `simthing_clausething::StructuralGridFrame`.)
- **StructuralGridCoordinate** — a gridcell `Location`'s authoritative `(col,row)` on the lattice. **Structural**, not render. Integer; the stencil walks neighbors by index arithmetic (no float Euclidean authority).
- **StructuralGridPlacement** — the binding of a `Location` id to its `StructuralGridCoordinate` in `grid_metadata` (one-system-per-cell).
- **RenderCoordinate** — an *optional secondary* cosmetic copy for display (`mapgen_render_*` props). Never authoritative; never the placement source.
- **ExecutionTheater** — a *bounded local window* over the structural lattice on which the dense Movement-Front/PALMA stencil executes (≤10/32 per edge, P1). It is **not** the lattice.
- **AtlasDeferral** — the typed result (`MapGenMovementFrontErrorKind::AtlasDeferralRequired`) returned when a dense execution profile cannot cover a layout in one theater. The layout stays valid; only execution defers to multi-theater scheduling.
- **SpatiallyBoundAccumulator** — an RF/Accumulator arena whose participants are gridcell `Location`s; it is spatially indexed through STEAD and requires `StructuralGridPlacement`s (`SpatialBindingMode::SpatiallyBoundToGridcellLocations`).
- **SpatiallyNeutralAccumulator** — a generic RF/Accumulator arena not bound to Locations; needs no grid (`SpatialBindingMode::SpatiallyNeutral`).

## 1. A Location is a gridcell
A `SimThingKind::Location` **is** a gridcell. Spatial identity is intrinsic — not a detachable role, not a new kind. There is no non-spatial Location. (core §7; ADR D2.)

## 2. Structural coordinates vs render coordinates
The emitted integer `(col,row)` are **StructuralGridCoordinates** — the lowerer honors them as the authoritative gridcell layout (STEAD-PRIVILEGE-0). The emitted galactic pattern (spiral/ring/…) **is** the lattice. RenderCoordinates are an optional cosmetic copy and are never authoritative. Emission order is **not** spatial order; row-major fill is **not** an acceptable lowerer layout.

## 3. Layout admission vs execution-profile admission
These are **separate** (STEAD-SCALE-0/-1). **Layout admission** (`admit_structural_grid` + `MapgenStructuralGridBudget`, checked-`u128`) has **no fixed edge cap** — it scales by explicit budgets + memory. **Execution-profile admission** (`simthing-spec` region-field ≤10/32-per-edge bounded theater) is a separate gate. A vast layout may pass while a dense execution profile defers (`AtlasDeferral`). Dense theater caps cannot shrink, compact, invalidate, or reinterpret the structural layout.

## 4. Ambient cells and sparse spatial domains
Unoccupied cells are **ambient field**, not absent ontology. Lattices are **sparse** (placements stored, not `edge²` cells). `200×200` is a *small* reference, not an upper bound; vast lattices are anticipated.

## 5. RF / Accumulator relationship to STEAD
RF/Accumulator stays **generic**. **But when an arena's participants are gridcell `Location`s, it is spatially indexed through STEAD**: each participant **must** have a `StructuralGridPlacement` in `grid_metadata` (never render metadata), and the arena records its `StructuralGridFrame` (`SpatialArenaBindingReport`). Code: `validate_spatial_binding` rejects a Location participant without a structural placement; `SpatiallyNeutral` arenas need no grid. Resource-flow code touching Location participants **must confront** this.

## 6. PALMA relationship to STEAD
PALMA W/D feedstock is a **field over the structural lattice** (`grid_size` from `grid_metadata`), not render coordinates. PALMA `D` is a **field, not a route** — no predecessors, no `came_from`, no path objects, no pathfinding.

## 7. Gu-Yang / SaturatingFlux relationship to STEAD
Gu-Yang/SaturatingFlux falloff propagates over **structural neighbor cells** within the `StructuralGridFrame`/`ExecutionTheater`. It is a conservative-flux stencil over the spatial substrate — **not** a border/frontline semantic service.

## 8. Required tests for any spatial/RF/PALMA/Movement-Front change
Any change to spatial dynamics, RF/Accumulator over Locations, PALMA, Gu-Yang, or Movement-Front MUST keep green (and extend where relevant): `stead_spatial_contract_guards`, `mapgen_structural_admission`, `mapgen_vast_scale_layout`, `mapgen_rf_stead_binding`, `mapgen_lattice_hierarchy`, `mapgen_resource_flow`, `mapgen_palma`, `mapgen_movement_front`, `mapgen_constitution_guards`. New spatial behavior needs a test proving it uses `grid_metadata` (structural), not render metadata.

## 9. Forbidden drift phrases (WITHDRAWN — never assert these in active source/docs)
The following are **withdrawn doctrine**; asserting any of them in active (non-archive) source or docs is a contract violation guarded by `stead_spatial_contract_guards`:
- "positions are inert"
- "shape is cosmetic"
- "topology is the lattice"
- "fixture-local placement is production placement"
- "emission order is spatial order"
- "row-major fill is acceptable lowerer layout"
- "RF is independent of spatial grid constraints when bound to Locations"
- "PALMA is pathfinding"
- "Gu-Yang is a border/frontline semantic service"
- "Movement-Front dense theater cap limits structural layout"

## 10. Structural execution convergence (Studio → GPU horizon)
Every Studio→GPU structural execution surface (loading a runtime scenario and playing it out) MUST: route to an **existing sanctioned `simthing-gpu` operator** — never a new bespoke Studio/GPU kernel; be **compiled from `SimThingScenarioSpec` by `simthing-driver`**; be **dispatched under `simthing-sim` tick/boundary**; operate over the **correct structural adjacency**; and keep GPU output as projection/cache, never authority. "One mechanism" means one discipline with admitted operator variants — not one literal kernel, and not a parallel Studio engine.

The three horizon surfaces, their adjacency, and their convergence targets:

| Surface | Adjacency | Existing operator (target) | Bounded theater + atlas |
|---|---|---|---|
| RF / link coupling | hyperlane **link graph** (bounded fanout) | `AccumulatorOp` Sum-over-`INPUT_LIST` | no |
| Gu-Yang falloff borders | **grid N4** of `(col,row)` | `saturating_flux_choke_threshold` + `structured_field_stencil` | **yes** (§7 P1; dense-global is rejected) |
| PALMA reach field | **grid N4** of `(col,row)` | `min_plus_stencil` + `w_impedance_compose` | **yes** |

The link gather (coupling accumulation over the hyperlane graph) is **not** the heatmap stencil; Gu-Yang/PALMA are grid stencils over N4 lattice neighbors within a bounded `ExecutionTheater`. These adjacencies must not be conflated. Borders **emerge as field expressions** (SaturatingFlux falloff fronts + PALMA min-plus reach); Gu-Yang/SaturatingFlux produces falloff fronts, never a frontline semantic service, and PALMA `D` is a field, not a route (no predecessors/paths) — see §9. A bespoke per-surface kernel in the Studio is a STEAD/convergence violation; if an existing operator structurally cannot host a needed step, STOP and escalate to design authority rather than forking a kernel.

They may appear ONLY inside an explicitly-named *Withdrawn doctrine* / *Correction* section (like this one) that negates them.
