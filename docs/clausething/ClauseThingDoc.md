# ClauseThingDoc — the clearinghouse for ClauseScript, the MapThing layer, and the Map Generator

> **Status: LIVING CLEARINGHOUSE (opened 2026-06-15, executive design authority).** This is the single
> entry point for everything in the ClauseThing vertical: **ClauseScript ingestion**, the **MapThing /
> MapGen ingest-lowering layer**, and the **MapGeneratorCLI producer**. It collects the concepts,
> practices, and APIs and **links out** to the reference specs and the (now archived) production tracks.
> It does not restate the durable decisions — those live in
> [`../adr/ClauseThingADR.md`](../adr/ClauseThingADR.md) (the decision record) and the transient
> constitution **§0** of [`../design_0_0_8_3.md`](../design_0_0_8_3.md). **Read order:**
> `simthing_core_design.md` §1.1 + §7 → constitution §0 → `ClauseThingADR.md` → this doc → the spec you need.
>
> All three tracks are **CLOSED**. This doc is where you start when extending the producer, adding a shape,
> wiring a UI, or opening the next track (FIELD-MOVIE-DATASET-0).

---

## 1. The vertical at a glance

```
high-level galaxy levers ──(MapGeneratorCLI producer, 0.0.8.6)──▶ static_galaxy_scenario neutral-AST text
                                                                          │
   designer ClauseScript ──(ClauseThing front-end, CT-)──▶ declarative SimThing spec structures
                                                                          │
                                          parse_mapgen_neutral_document   ▼
                              (MapThing / MapGen ingest-lowering, 0.0.8.2.5)
                                                                          │
   gridcell Location lattice + bounded links/lane-couplings + RF arenas + Movement-Front + PALMA feedstock + commitments
                                                                          ▼
                            the generic accumulate → reduce → mask → threshold tree (simthing-sim, semantic-free)
```

Nothing in this stack is an engine. Every box **compiles away** to flat registrations on the one recursive
SimThing tree. See [`../adr/ClauseThingADR.md`](../adr/ClauseThingADR.md) D1.

## 2. Concepts (the vocabulary)

| Concept | One line | Authority |
|---|---|---|
| **ClauseScript** | The *foreign* Paradox/Clausewitz scripting language (scripted_effects / triggers / script_values / modifiers). Reference only. | [`ClauseThing.md`](ClauseThing.md) |
| **ClauseThing** | Treating ClauseScript as SimThing's *designer-facing* language: idioms → declarative spec structures. | [`ClauseThing_Spec.md`](ClauseThing_Spec.md) |
| **MapThing / MapGen** | Stellaris *starmap* → SimThing star-mapping: neutral-AST → gridcell lattice + links + RF + Movement-Front + PALMA. | [`MapGenThing.md`](MapGenThing.md) |
| **MapGeneratorCLI** | The producer above the lowering path: galaxy levers → `static_galaxy_scenario` text. | [`MapGeneratorCLI.md`](MapGeneratorCLI.md) |
| **Location ≡ gridcell** | A `Location` SimThing **is** a gridcell; spatial identity intrinsic, not a `SimThingKind`. | ADR D2, core §7 |
| **Movement-Front (STEAD)** | The map run as a cellular automaton (Wei, arXiv:2602.01651); P1 locality / P2 symmetry / P3 stability. Prose: **STEAD**, never "SEAD". | ADR D3, [[movement-front-automaton]] |
| **RegionFieldSpec (3-layer)** | L1 `pressure_binding` (on-device arena→cell projection) + operator/horizon; L2 `reduction`; L3 `parent_formula` + `commitment`. | ADR D4 |
| **SaturatingFlux / PALMA** | Gu-Yang conservative-flux stencil; PALMA min-plus `D = W + min(N4 D)` (tropical, no sqrt). PALMA `D` is a **field, not a route**. | ADR D5 |
| **Candidate F** | The only exact-magnitude authority for *decision gates* (no float `sqrt`/distance gates a commitment). **Gridcell positions are structural-spatial** (§7) — the lowerer honors emitted **integer** positions as the lattice layout; that is not "Euclidean authority" (placement/stencil are integer index arithmetic). | ADR D6, constitution §0.7 + §7 |
| **Bounded coupling** | Hyperlanes / special routes / bridges = bounded `add_hyperlane` pairs → `mapgen_links`; no graph/route/predecessor. | ADR D7 |
| **Structural grid budget (no edge cap)** | Structural layout has **no fixed edge cap** — it scales by explicit `MapgenStructuralGridBudget` (cells / occupied / links / metadata-bytes; checked-`u128`, default unbounded). `200×200` is a *small* reference; `65,535` was a temporary arithmetic ceiling, not doctrine. Decoupled from **execution-profile** limits (bounded theater ≤10/32): a vast layout may admit while a dense field defers to atlas. | core §7, STEAD-SCALE-1, `admit_structural_grid` |

## 3. Practices (how to work in this vertical — binding)

1. **Lower, don't engine.** Express new behavior as declarative structure that lowers onto the closed
   surfaces. If it cannot be expressed as `accumulate → reduce → mask → threshold` + bounded links + RF
   arenas, **escalate to the DA** — do not add a special case. (Constitution §0.5 checklist.)
2. **The closed lowering layer is closed.** Producer/front-end work makes **zero** edits under
   `crates/simthing-clausething/src/`. If you need a lowerer change, **STOP and split** it into a
   DA-authorized 0.0.8.2.5 amendment PR with its own battery (precedent: `#680`). (ADR D10.)
3. **Producer emits the proven grammar only.** `static_galaxy_scenario { system{ id name position{x y z} initializer } add_hyperlane nebula } <name>_initializer{…}` — the form `mapgen_lattice` reads. **Not** `hydrate_scenario` `scenario { location … }` (superseded history). (ADR D9.)
4. **Quantize before emission; positions are the spatial layout.** Producer-side float (cos/sin, ellipse,
   distances, anchors) is fine **only** if quantized to integer cells before emission. The output carries integer
   `LatticeCoord` — and the lowerer **honors those positions as the authoritative gridcell `(col,row)`** (§7,
   STEAD-PRIVILEGE-0): the emitted galactic pattern **is** the lattice. No float Euclidean magnitude gates a
   commitment (§0.7), but integer positions are **structural**, not inert. (ADR D6.)
5. **Bounded everything, fail closed.** Every topology/arena pass declares hard caps and fails closed when
   impossible (`UnsatisfiedRouteCount`, `UnsatisfiedBridgeCount`, `CapacityOverflow`, …). No arbitrary graph.
6. **Add a shape = one registry row + a strategy module.** The `ShapeStrategyEntry` registry is the single
   source for descriptors *and* executable dispatch; the core/emitter/lowering contract must not change. (ADR D9.)
7. **Governance.** Per-rung DA-sensitivity is on the ladder; mechanical rungs merge without a DA sign-off,
   DA-review rungs need a genuine independent audit + battery rerun; **only the DA writes a DA sign-off**,
   never pre-filed. (ADR D10.)

## 4. APIs (where the code lives)

### 4.1 Producer — `crates/simthing-mapgenerator/` (standalone; no `simthing-*` deps)
| Module | Responsibility |
|---|---|
| `params.rs` | Full §3A lever surface (`MapGeneratorParams`): scale/core, shape, clustering, partitioning, hyperlane geometry, special routes, nebula fields, initializer buckets, inert metadata, arbitrary/static, output. Validation; `ExplicitShapeInProceduralMode` mode gate. |
| `rng.rs` | Pinned **SplitMix64** (`MapGenSeed`/`MapGenRng`); no system entropy. |
| `lattice.rs` | Square integer lattice + core mask; `try_cell_count()`/`cell_count_u64()` fail-closed overflow. |
| `occupancy.rs` | One-system-per-cell; precomputed placeable + free-index list (no per-insert full rebuild). |
| `shape_registry.rs` + `strategies/` | `ShapeStrategyEntry` single-source registry; 12 vanilla shapes (elliptical, spiral_2/3/4/6, ring, bar, starburst, cartwheel, spoked, static, arbitrary_static); `common.rs::quantize_polar` (float→integer cells). |
| `topology.rs` / `special_routes.rs` / `partition.rs` / `cluster.rs` | Bounded `add_hyperlane` couplings; report-only kinds; producer-side BFS/DFS partition ordering (Stellaris method mirror, not pathfinding). |
| `pair_candidates.rs` | Bounded enumeration: windowed hyperlanes (`collect_pairs_within_chebyshev`), heap-capped long-range (`collect_farthest_pairs_with_filter`, O(log cap)/pair). |
| `emitter.rs` | `static_galaxy_scenario` neutral-AST text; bareword initializers + sibling defs; **structural integer positions** (honored by the lowerer as the gridcell layout). |
| `lib.rs` | `place_and_emit_scenario*` pipelines (validate → place → topology → emit). |
| binary `mapgen` | CLI (`--shape --num_stars --seed --dry-run …`). |

### 4.2 Ingest / lowering — `crates/simthing-clausething/src/` (**CLOSED — do not edit without an amendment**)
| Entry point | Responsibility |
|---|---|
| `parse_mapgen_neutral_document` | Parse Paradox-style neutral AST (the `static_galaxy_scenario` form). |
| `mapgen_lattice.rs` | Build the gridcell `Location` lattice + initializer payloads (`build_scenario_clause`). |
| `mapgen_links.rs` | `extract_hyperlane_declarations` + `lower_hyperlane_topology` → `grid_metadata.links` / `lane_couplings`. |
| `hydrate_scenario.rs` | Generic `scenario { location … }` hydration — **superseded** for map-gen; a different consumer. |
| RF / Movement-Front / PALMA | `generate_mapgen_resource_flow_enrollment`, movement-front + PALMA feedstock on the closed CT surfaces. |

### 4.3 Tests / guardrails
The **LIVE GPU guardrail** is `simthing-clausething/tests/mapgenerator_cli_pr10_gpu_compact_evidence.rs`
(tiny generated pack → `install_atomic` + `SimSession::open_from_spec` + real-adapter GPU compact readback,
`field_values`/`reduction_parent_value`/`eml_output` `is_none()`). The `mapgen_constitution_guards` battery
scans the closed generators for Euclidean/forbidden-vocab/kind tokens. Per-track PR-evidence reports are
archived under `../archive/superseded_tests/` (see §6).

## 5. Extending the vertical
- **New shape** → add a `strategies/<shape>.rs` + one `vanilla_entries()` row + tests. Nothing else changes.
- **New arbitrary layout** → use the `arbitrary/static` mode (explicit point-cloud + graph), no core change.
- **New lowered surface** → only through an **already-accepted** closed grammar/surface; never widen
  `hydrate_scenario` or the lowerer from a producer PR. New surface ⇒ DA-authorized 0.0.8.2.5 amendment.
- **Galaxy-scale install** → blocked until the **RF capacity amendment** (ADR §5) lands (closed-track, DA-authorized).
- **UI** → drive the producer's levers, render the dry-run/manifest preview, let the user review
  `static_galaxy_scenario` text before admission. The UI is **not** a runtime service and must not add
  route/path/predecessor/movement semantics.

## 6. Index — every file in the vertical
**Live reference specs (this directory):** [`ClauseThing_Spec.md`](ClauseThing_Spec.md) ·
[`ClauseThing.md`](ClauseThing.md) (ClauseScript textbook) · [`MapGenThing.md`](MapGenThing.md) ·
[`MapGeneratorCLI.md`](MapGeneratorCLI.md) · [`mapgen_corpus_manifest.md`](mapgen_corpus_manifest.md) ·
memos: [`ct_2c_economic_category_memo.md`](ct_2c_economic_category_memo.md),
[`ct_3b_4a_movement_front_heatmap_memo.md`](ct_3b_4a_movement_front_heatmap_memo.md),
[`ct_vertical_consumer_contract.md`](ct_vertical_consumer_contract.md), [`scope_memo.md`](scope_memo.md).

**Decision record:** [`../adr/ClauseThingADR.md`](../adr/ClauseThingADR.md). **Governing ADRs:**
[`../adr/mapping_sparse_regioncell.md`](../adr/mapping_sparse_regioncell.md),
[`../adr/resource_flow_substrate.md`](../adr/resource_flow_substrate.md).

**Archived production tracks (`../archive/closed_production/`):**
- ClauseThing: `design_0_0_8_1_clausething_production_track.md`, `design_0_0_8_2_clausething_closeout_ladder.md`
- MapThing / MapGen: `design_0_0_8_2_5_mapgen_ladder.md`
- MapGeneratorCLI: `design_0_0_8_6_mapgenerator_cli_ladder.md`
- Substrate sub-tracks: `design_0_0_8_1_border_hack_track.md` (BH / SaturatingFlux / W-impedance), `design_0_0_8_1_palma_pathfinding_integration_guide.md` (PALMA min-plus)

**Constitution & paradigm:** [`../design_0_0_8_3.md`](../design_0_0_8_3.md) ·
[`../simthing_core_design.md`](../simthing_core_design.md) · [`../invariants.md`](../invariants.md).

**Next track (not opened here):** FIELD-MOVIE-DATASET-0 (editor/corpus/export) — digest at
[`../workshop/field_movie_dataset_0_mapping_digest.md`](../workshop/field_movie_dataset_0_mapping_digest.md).
