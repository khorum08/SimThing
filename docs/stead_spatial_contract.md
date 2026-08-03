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

### 5.1 CostBand — THE resource-sink definition (BAND-QUANTIZED-DRAW-0)

**CostBand** (one word, camel-humped) is **the** definition of a resource sink — never an opt-in mode and never a rival sink beside observation.

- **Observation is the base case.** A threshold crossing that costs nothing and consumes nothing already *is* observation.
- **Action is observation with a CostBand attached.** The authored question is *"is this a sink?"*, never *"is this a CostBand?"* — every sink **is** a CostBand.
- **Algebra (exact by construction):** given available value `V` and unit cost `C`,
  `N = floor(V/C)` (optionally capped by authored `throttle_hint_max_per_tick`),
  `R = V − N·C`, and **`V = N·C + R` exactly**. Both operands ride sealed
  `BandCrossingDelta` fields (`threshold()` = C, `post_value()` = V); zero WGSL
  and zero `ThresholdRegistration` layout change.
- **Booleans are depth 1** through the same quantize path as depth N — no separate
  did-it-fire branch. Command deficits (`requested: 1`) are the depth-1 degenerate.
- **Marker is authored** per registration (and may also be per resource);
  per-registration wins; ambiguity hard-errors at admission. Marker lives on the
  CPU-side semantic table keyed by `event_kind`, never the GPU POD.
- Conservation of the draw is **per-resource-channel** (`N·C` consumed exactly);
  output minting is a separate channel (Stage 2 recipe re-expression is out of scope).

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
| Gu-Yang falloff borders | authored weighted `GridOffsets` or canonical `LinkGraph` | generic `FieldSweepRegistration` | **yes** for grid theaters (§7 P1; dense-global is rejected) |
| PALMA reach field | authored weighted `GridOffsets` or canonical `LinkGraph` | generic `FieldSweepRegistration` | **yes** for grid theaters |

As of rung 5.6, PALMA and Gu-Yang authoring compile to the one generic, proof-admitted
`FieldSweepRegistration`: authored EML map / fixed-linear-fold / post programs over the existing
input-list gather, with canonical neighbor order and conservative symmetry admitted before execution.
Weighted `GridOffsets` (N4, N8 with an authored diagonal weight, and radius-r with authored shell
weights) and `LinkGraph` are values of the same adjacency axis. `LinkGraph` uses the existing link
compiler's sorted, deduplicated, undirected neighbor rows as its `CanonicalOrderProof` basis.
Conservative registrations additionally carry a per-node certificate proving
`chi_i * sum_j(abs(c_ij)) <= admitted_bound`; degree-homogeneous execution buckets may reuse row-degree
metadata but cannot determine that bound or reorder any node's authored neighbor list. Target/neighbor
reads exist only in the field EML context, and algebra identity is authored program data rather than an
enum, tag, or operator dispatch. Multi-pass field laws may use only a sealed kernel-private transient
lane or an explicitly authored non-authoritative output; transient conductance must never borrow an
unrelated authored matrix column.

The link gather (coupling accumulation over the hyperlane graph) is **not** the same semantic field law
as a heatmap sweep. They share an admitted adjacency representation without conflating their authored
map/fold/post programs. Borders **emerge as field expressions** (SaturatingFlux falloff fronts + PALMA
min-plus reach); Gu-Yang/SaturatingFlux produces falloff fronts, never a frontline semantic service, and
PALMA `D` is a field, not a route (no predecessors/paths) — see §9. A bespoke per-surface kernel in the
Studio is a STEAD/convergence violation; if the generic registration structurally cannot host a needed
step, STOP and escalate to design authority rather than forking a kernel.

As of rung 5.8, **comparative projections** (dominance / margin / contest / border band / chokepoint)
are sealed field-EML registration chains over co-located generic field-sweep outputs (driver consumer;
no new kernel door). They are default-derived when ≥2 competing emitter classes are admitted
(authored opt-out with a visible reason only), emit a fixed comparative column count independent of
owner count, and resolve exact ties by authored emitter order. **Margin** is exact `top1−top2`
(non-negative magnitude). **Border** is winner-identity change across canonical adjacency
(`argmax(target) ≠ argmax(neighbor)`), not a sign flip of margin. **Contest** consumes Gu-Yang
stall magnitude (`gross_flux − |net_flux|` from an authored second field-sweep registration) under
both-strong/small-margin. Chokepoint is only `contested-border ∧ PALMA-low-D`. Events arrive as
ordinary anchored threshold bands; no border service or CPU full-field decision path is admitted.

They may appear ONLY inside an explicitly-named *Withdrawn doctrine* / *Correction* section (like this one) that negates them.
