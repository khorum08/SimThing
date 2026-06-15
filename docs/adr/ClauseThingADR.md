# ClauseThing ADR — ClauseScript ⇄ MapThing ⇄ MapGeneratorCLI (consolidated findings)

> **Status: ACCEPTED (2026-06-15, executive design authority).** This ADR is the durable decision
> record distilled from three now-closed production tracks: **ClauseThing (`CT-`)**, **MapThing /
> MapGen (0.0.8.2.5)**, and **MapGeneratorCLI (0.0.8.6)**. The tracks' blow-by-blow ladders are
> archived (see §7); this ADR is what carries forward. It is **subordinate to**
> [`../simthing_core_design.md`](../simthing_core_design.md) (permanent paradigm) and the **transient
> constitution §0** of [`../design_0_0_8_3.md`](../design_0_0_8_3.md), and it is **paired with** the
> living clearinghouse [`../clausething/ClauseThingDoc.md`](../clausething/ClauseThingDoc.md) (concepts,
> practices, APIs). Where this ADR and an archived ladder disagree, this ADR governs.
>
> **One-line thesis:** *Clausewitz/Stellaris content (script + starmap) is ingested as declarative
> SimThing structure and lowered onto the already-closed generic substrate — there is no new engine,
> no new `SimThingKind`, and no Euclidean/pathfinding authority anywhere in the lowered output.*

---

## 1. Context

The three tracks form one vertical, bottom-up:

| Layer | Track | What it is | Closed |
|---|---|---|---|
| **L-ref** | ClauseScript textbook | Reference for the *foreign* source language (Paradox Clausewitz). Not a SimThing design doc. | n/a (reference) |
| **Front-end** | **ClauseThing (`CT-`)** | Designer-facing ingestion: Clausewitz script idioms → SimThing spec structures. | 2026-06-12 |
| **Ingest/lowering** | **MapThing / MapGen (0.0.8.2.5)** | Stellaris *starmap* → SimThing star-mapping: neutral-AST → gridcell lattice + links + RF + Movement-Front + PALMA feedstock. | 2026-06-14 |
| **Producer** | **MapGeneratorCLI (0.0.8.6)** | Standalone producer: high-level galaxy levers → declarative `static_galaxy_scenario` text the ingest layer lowers. | 2026-06-15 |

The forcing question across all three: *how do you let designers (and a generator, and eventually a UI)
author rich grand-strategy content without smuggling a bespoke engine — combat, economy, AI, pathfinding,
a map renderer — back into a substrate whose entire value is that it has none?* The answer, proven three
times, is **declarative lowering onto the generic `accumulate → reduce → mask → threshold` tree**.

---

## 2. Decisions (the durable adjudications)

### D1 — Everything ingested is *declarative structure*, never engine code
ClauseScript effects/triggers/script-values, starmap systems/hyperlanes/nebulas, and generator levers all
compile to **flat `AccumulatorOp` / overlay / threshold / link registrations** on the one recursive
SimThing tree. `simthing-sim` never learns the words "combat", "hyperlane", "nebula", "wormhole",
"partition", or "shape". Semantics live at the spec/driver/producer layer and **compile away**. (Carries
transient-constitution §0.1/§0.3.)

### D2 — `Location ≡ gridcell`; spatial identity is intrinsic, **not** a `SimThingKind`
A `Location`-kind SimThing **is** a gridcell. Its grid/spatial identity is an intrinsic property of being
a Location, carried as `mapgen mapping_role` + inert `position` metadata — **not** a detachable mapping
role and **never** a new kind. `assert_allowed_simthing_kinds` rejects bespoke kinds. (Two DA drifts were
corrected to reach this; it is now binding in `simthing_core_design.md` §7.)

### D3 — The map is a **Movement-Front cellular automaton**, not a planner
The starmap is a square lattice of gridcell-SimThings run as a cellular automaton (Zichao Wei, *STEAD* —
SpatioTemporal Evolution with Attractor Dynamics, arXiv:2602.01651; engine name **Movement-Front**).
Three principles bind it: **P1 Locality** (bounded per-tick horizon — the grid is *not* bounded; the
*horizon* is, and horizon-widening-as-strategic-shortcut is the rejected pattern), **P2 Symmetry** (one
shared `StructuredFieldStencilOp` kernel), **P3 Stability** (attractor/threshold projection). Strategic
awareness is **hierarchy (reduce-up)**, never dense-global diffusion. The prose term for Wei's concept is
**STEAD**; "SEAD" is banned (military connotation / guardrail). See [[mapping_sparse_regioncell]],
[[resource_flow_substrate]].

### D4 — The three-layer model **is** `RegionFieldSpec`
- **L1 = `pressure_binding`** — an `ArenaPressureBindingSpec` projects arena flow `(arena, sub_field) →
  (target_id, row, col)` onto the cell seed **on-device** (no CPU side-channel), plus operator / horizon /
  `alpha_self` / `gamma_neighbor`.
- **L2 = `reduction`** — `SlotRange` Sum hierarchy (strategic awareness = reduction up).
- **L3 = `parent_formula`** (`ai_will_do`) + `commitment` (EvalEML → threshold → BoundaryRequest).

RF arena columns are **projected** onto the cell, never copied (the "same columns" sketch was an
imprecision; corrected to the on-device projection).

### D5 — Field operators are **conservative-flux / tropical**, with no sqrt
- **Gu-Yang `SaturatingFlux`** — conservative-flux stencil (arXiv:2509.20797).
- **PALMA min-plus** — `D = W + min(N4 D)`, tropical algebra, **no sqrt** (arXiv:2601.17028). PALMA's
  `D` is a **field, not a route**: there are no predecessors, no `came_from`, no path objects. "Pathfinding"
  is reach computed as a min-plus field and consumed by threshold crossings.

### D6 — **Candidate F** is the only exact-magnitude authority; positions are inert
Decision-critical magnitude/distance/threshold gates route through the artifact-backed exact chain
(`m_jit_mag2_fixed_exact` → Candidate F sqrt `m_jit_mag_f_from_exact_mag2`). Raw f32 `sqrt`/`length`/
`distance`/`normalize`/`hypot` are `ApproximateDiagnostic` only. In the lowered output, **positions are
inert render metadata and adjacency is lattice/topological** — there is **no Euclidean authority** in any
sim-authoritative path. Producer-side float math (spiral `cos/sin`, ellipse sampling, Chebyshev distances,
cluster anchors, bridge selection) is permitted **only** because it is *quantized to integer cells /
topological edges before emission* and never reaches sim. (Carries §0.7.)

### D7 — Bounded topology only; links/routes are **bounded couplings**, never graph objects
Hyperlanes, special routes (wormhole/gateway), partition bridges, and cluster bridges all lower as bounded
`add_hyperlane` endpoint pairs → closed `mapgen_links` (`grid_metadata.links` / `lane_couplings`) under hard
fanout/distance caps. **No arbitrary high-degree graph, no route/predecessor/movement-order semantics, no
CPU planning over fields.** Producer-side "kind" tags (wormhole/gateway/partition/cluster) are **report-only
and never emitted in grammar**. Offline producer-side BFS/DFS partition ordering mirrors Stellaris
`home_system_partitions { method = breadth_first | depth_first }` and is **not** runtime pathfinding (no
source→target, no predecessors, nothing traversal-related reaches output).

### D8 — Scaling is **hierarchical fanout absorption**, never prohibition
galaxy→sector→system→planet→deposit, ≤~100 children/level, is the scaling mechanism (same hierarchy as L2).
Every RF arena declares explicit selectors + hard caps (`max_participants` / `coupling_fanout` /
`orderband_depth`) + `FissionPolicy` + coupling-delay forms; rejected at build if unsafe; emits an
expansion report. The participant cap is on **concurrent** participants, not cumulative, not cells×capacity;
slots recycle via the REENROLL free-list. (Carries §0.4; [[resource_flow_substrate]].)

### D9 — The producer is a **thin, standalone, declarative emitter** with a closed-source gate
`simthing-mapgenerator` depends only on `clap`/`serde`/`thiserror` — **no `simthing-*` runtime deps**. It
emits **`static_galaxy_scenario` neutral-AST text** consumed by `parse_mapgen_neutral_document` →
`mapgen_lattice`/`mapgen_links` (the proven path). The `hydrate_scenario` `scenario { location … }` form is
**superseded history**, not the map-gen path. Shapes are a **data-driven `ShapeStrategyEntry` registry
(never a baked enum)**: adding a shape = one registry row + a strategy module; the core/emitter/lowering
contract is untouched. Two first-class modes: **procedural** + **arbitrary/static** (explicit point-cloud +
graph), so future/arbitrary layouts work without changing the core.

### D10 — Governance: the closed lowering layer is **closed**; producer PRs may not edit it
Once an ingest/lowering layer closes, a producer PR that needs a change under `crates/simthing-clausething/
src/` must **STOP and split** the fix into a separate **DA-authorized 0.0.8.2.5 amendment PR** with its own
battery (precedent: the `#680` lowerer child-id amendment that PR5 surfaced). Per-rung **DA-sensitivity** is
explicit on each track's ladder; mechanical rungs are merged without a DA sign-off, DA-review rungs require a
genuine independent audit + battery rerun before merge. **Only the DA writes a DA sign-off**, and never a
pre-filed one. (PR3-era fake-approval lesson; reaffirmed across the MapGeneratorCLI track.)

---

## 3. Consequences

**Positive.** One generic substrate now ingests (a) designer script, (b) a real starmap, and (c) a
parametric galaxy generator, with zero new engines and a real-adapter GPU compact-evidence path. The
producer is UI-callable; a UI can drive galaxy levers and review `static_galaxy_scenario` text before
admission. Extensibility is structural (registry rows, arena specs), not surgical.

**Accepted limitations (honest, not defects).**
- **Galaxy-scale install is gated.** Generating a 1000-star map is proven (producer + parse/lattice-lower);
  **admitting/installing it at scale is blocked by the closed RF participant/slot caps** and was correctly
  **not widened**. Raising those caps (or adding scalable deposit-initializer feedstock) is the one
  outstanding **DA-authorized 0.0.8.2.5 amendment candidate** (§5) — never a producer-only patch.
- **Shape is cosmetic; topology is the lattice.** Authored positions are inert (D6); grid placement and
  adjacency are index-order/topological. The elaborate shapes are render metadata, not simulated geometry.
- The PR10 tiny-fixture real-adapter GPU compact-evidence test remains a **live GPU guardrail**.

**Negative / watch-items.** Long-range producer enumeration is O(N²)-examined (heap-capped to O(log cap)
per pair after PR11's remediation; fine to ~1000-star, revisit for larger). The two-front-end split
(`static_galaxy_scenario` for map-gen vs `hydrate_scenario` for generic scenarios) is a known sharp edge —
the map-gen path is the only proven one.

## 4. Alternatives rejected
- **A bespoke map/combat/economy/AI/pathfinding engine** — violates §0.1/§0.3 (the whole premise).
- **`gridcell` as a new `SimThingKind`** — rejected (D2); spatial identity is intrinsic to `Location`.
- **Dense-global diffusion for strategic awareness / horizon-widening** — rejected (D3); awareness is hierarchy.
- **Euclidean authority in the lowered output / authored-position-as-authoritative-placement** — rejected
  (D6); the binding PR5 ruling fixes placement as index/topological.
- **Emitting `field_operator`/`hydrate_scenario` grammar from the producer** — rejected (D9); the producer
  targets the accepted `static_galaxy_scenario`/`nebula` surfaces only, never widening the lowerer.
- **Producer PR editing the closed lowerer** — rejected (D10); split to a DA-authorized amendment.

## 5. Outstanding (future, separate, DA-authorized — not a producer PR)
**RF capacity amendment (0.0.8.2.5/0.0.8.2+):** raise/scale RF participant/slot caps, or add scalable
deposit-initializer feedstock, so galaxy-scale generated packs can admit/install. This is a closed-track
amendment with its own battery and DA sign-off. Until then, galaxy-scale runtime is gated and that gate is
honest.

## 6. Status of each track
All three **CLOSED and DA-signed**: ClauseThing (2026-06-12), MapThing/MapGen 0.0.8.2.5 (2026-06-14, PR1–PR11),
MapGeneratorCLI 0.0.8.6 (2026-06-15, PR1–PR12 incl. the `#680` child-id amendment). FIELD-MOVIE-DATASET-0
(editor/corpus/export) is the **subsequent** track, not opened here.

## 7. References (archived ladders + live docs)
- Live clearinghouse (concepts/practices/APIs): [`../clausething/ClauseThingDoc.md`](../clausething/ClauseThingDoc.md).
- Constitution (carry-forward §0 + addendum): [`../design_0_0_8_3.md`](../design_0_0_8_3.md). Permanent paradigm: [`../simthing_core_design.md`](../simthing_core_design.md). Invariants: [`../invariants.md`](../invariants.md).
- Governing ADRs: [`mapping_sparse_regioncell.md`](mapping_sparse_regioncell.md), [`resource_flow_substrate.md`](resource_flow_substrate.md).
- **Archived production tracks** (`../archive/closed_production/`): `design_0_0_8_1_clausething_production_track.md`, `design_0_0_8_2_clausething_closeout_ladder.md`, `design_0_0_8_2_5_mapgen_ladder.md`, `design_0_0_8_6_mapgenerator_cli_ladder.md`, `design_0_0_8_1_border_hack_track.md`, `design_0_0_8_1_palma_pathfinding_integration_guide.md`.
- Layer reference docs (kept live): [`../clausething/ClauseThing_Spec.md`](../clausething/ClauseThing_Spec.md), [`../clausething/ClauseThing.md`](../clausething/ClauseThing.md) (ClauseScript textbook), [`../clausething/MapGenThing.md`](../clausething/MapGenThing.md), [`../clausething/MapGeneratorCLI.md`](../clausething/MapGeneratorCLI.md), [`../clausething/mapgen_corpus_manifest.md`](../clausething/mapgen_corpus_manifest.md).
- Corpus (read-only, not vendored): `C:\Users\mvorm\Clauser\Paradox\`.
