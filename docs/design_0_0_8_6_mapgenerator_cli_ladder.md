# SimThing 0.0.8.6 — MapGeneratorCLI PR Ladder (high-level galaxy params → declarative MapGen payloads)

> **Status: DESIGN / READY FOR CURSOR EXECUTION (track-opening plan, 2026-06-14, executive design authority; lever-surface + extensibility revision 2026-06-14 after a deep read of the Stellaris corpus). PR1 PASS pending DA review (2026-06-14): standalone `simthing-mapgenerator` crate + full §3A parameter surface + shape registry descriptor shell + validation-only dry-run — no generation.**
> This is the planning artifact for the **producer layer** above the now-closed 0.0.8.2.5 MapGen
> ingest/lowering ladder. It is not an implementation PR. It pins the producer-side schema judgments (§3)
> so the rungs are Cursor-mechanical with Codex reviews, and it is **subordinate to the core-design
> paradigm, the two governing ADRs, and the closed 0.0.8.2.5 contract (§0).**
>
> **What MapGeneratorCLI is.** A **thin, standalone, declarative producer**: it turns the high-level galaxy
> lever surface (§3A — scale, **a registered shape**, clustering, partitioning/bridges, hyperlane geometry,
> special routes, nebula fields, plus an inert metadata passthrough) into the **declarative ClauseScript
> `scenario { location … link … field_operator … initializer refs … }` payload** that the *already-closed*
> 0.0.8.2.5 MapGen front-end ingests and lowers. **It is NOT a runtime, NOT the MapGen lowerer, NOT a UI** —
> it is the missing "Galaxy Shape Generator" between UI levers and the lowering path.
> Reference: [`clausething/MapGeneratorCLI.md`](clausething/MapGeneratorCLI.md). Where this ladder and the
> reference disagree, **the ladder governs.**
>
> **Two generation modes, both first-class** (the Stellaris generator has both; we mirror both — §3B):
> **(1) procedural** — parameters + a *registered* shape strategy produce placements; **(2) arbitrary/static** —
> an explicit point-cloud + explicit graph (the `static_galaxy_scenario` form) is admitted directly. The
> shape set is a **data-driven registry, never a baked enum** (§3 C10), so new vanilla shapes, modded
> shapes, and **future/arbitrary layouts are first-class without changing the producer core.**
>
> **0.0.8.6 opens a new track. It does NOT reopen 0.0.8.2.5, and it is NOT FIELD-MOVIE-DATASET-0.** The
> editor/corpus/export seam (`FIELD-MOVIE-DATASET-0`) remains the **subsequent** track after this one.

---

## 0. Binding constraints — READ FIRST (non-negotiable)

> The CLI produces **declarative payloads only**. Its output must lower cleanly onto the already-closed
> surfaces, honoring every 0.0.8.2.5 adjudication (M1–M11) and the core/ADR paradigm. **If a rung's
> behavior is not expressible as declarative output the closed front-end already accepts, it STOPS and
> escalates (§7) — it never widens the lowering layer, the spec, or the runtime.**

**Governing reads (in order):** `docs/invariants.md` → `simthing_core_design.md` §1.1 + §7 →
`adr/mapping_sparse_regioncell.md` → `adr/resource_flow_substrate.md` → **`design_0_0_8_2_5_mapgen_ladder.md`
§0/§3 (M1–M11 — the producer's output contract)** → `clausething/mapgen_corpus_manifest.md` →
`clausething/MapGenThing.md` → `clausething/MapGeneratorCLI.md` → `design_0_0_8_1.md` §0.7 (Candidate F).
Corpus (read-only, **not vendored**): `C:\Users\mvorm\Clauser\Paradox\vanilla\map\setup_scenarios\` (the
`setup_scenario` lever surface + `supports_shape` set) and `…\static_galaxy_example.txt` (the arbitrary form).

**The CLI may produce:**
- high-level parameter parsing over the **full §3A lever surface** + a parameters-file (`--params=ui.json`) mode;
- deterministic, seed-driven procedural placement via a **registered shape strategy** (§3 C10) onto a
  **square** integer lattice (one-system-per-cell), with float curve math **immediately quantized** to
  integer `(col, row)` cells;
- an **arbitrary/static mode** (§3 C11): admit an explicit point-cloud + explicit graph (the
  `static_galaxy_scenario` shape — `system{ id position initializer spawn_weight }` + `add_hyperlane` /
  `prevent_hyperlane` + `nebula`, optionally with a `coordinate_transform`), quantized to the lattice;
- declarative `scenario { metadata, location …, link …, field_operator …, system_initializer refs }` text
  (preferred: the `hydrate_scenario` ClauseScript form; fallbacks: `static_galaxy_scenario` block or a
  manifest + tiny `.txt` initializer library);
- bounded explicit links + bounded long-range couplings — hyperlanes (per M3/M6 fanout + `max_hyperlane_distance`
  caps), and **special routes** (wormhole pairs / gateways) as bounded long-range lane couplings;
- **partition/bridge structure** (home/open partitions → RegionCell grouping; min/max bridges → bounded
  cross-partition lane couplings) and **clustering** (satellite RegionCell groups);
- at least one suppression/environmental `field_operator` (nebula) so output exercises
  RF→RegionField→Gu-Yang→PALMA when lowered;
- a "dry-run / manifest-only" mode emitting a human-readable placement report (UI preview).

**The CLI must NOT (hard prohibitions — any one ⇒ STOP/REJECT):**
`simthing-sim` awareness or changes · a new `SimThingKind` · runtime simulation behavior · a new GPU
kernel · semantic WGSL · pathfinding · routes · predecessors · movement orders · border/frontline
services · CPU planning over fields · full-field reads · Euclidean *authority* in the output (positions
are inert render metadata; adjacency is lattice/topological — float curve/partition math lives only in the
producer and never reaches sim) · vendored Paradox files in the repo · opening FIELD-MOVIE-DATASET-0 ·
widening the 0.0.8.2.5 lowering layer or `simthing-spec` · generating/simulating the inert metadata
passthrough (empire/crisis/odds params are carried verbatim as render metadata only, never acted on).

**Candidate-F tripwire:** the CLI's internal shape-curve/jitter/partition float math (sin/cos/sqrt for
sampling, distances for clustering & bridge selection) is a *producer-side* convenience that is
**quantized to integer cells / topological edges before emission** and never appears in the declarative
output or any sim-authoritative path. The *output contract* carries no Euclidean distance/magnitude/
nearest-neighbor authority. If any rung needs Euclidean *authority* in the lowered surface, STOP (§7) —
that is the M9/§0.7 boundary the 0.0.8.2.5 ladder already guards.

## 1. Preserved-baseline contract (the producer's target, from PR11 closeout)

These 0.0.8.2.5 artifacts are the **target contract** the CLI's output must satisfy; do not delete or bury:
- `docs/clausething/mapgen_corpus_manifest.md` — corpus families + approved initializer/setup files the CLI references.
- `crates/simthing-clausething/tests/fixtures/mapgen/` — the accepted declarative input shape + goldens; CLI output must be structurally equivalent (or richer) and lower the same way.
- `docs/tests/mapgen_pr{1,6,7,10,11}_results.md` (+ the per-PR lifecycle tables) — lowering expectations, RF/Movement-Front/PALMA feedstock shape, compact-evidence style, the LIVE_GUARDRAIL battery, the end-to-end harness pattern.
- The 0.0.8.2.5 LIVE_GUARDRAIL battery (`mapgen_*` + `ct_scenario_container` + `ct_bh3_closeout_sample_driver`) **must stay green** as the CLI adds generated cases.

## 2. What 0.0.8.6 closes

Closed when the CLI deterministically turns the **full §3A lever surface** into declarative payloads —
across **both modes** (procedural via ≥2 registered shape strategies *and* the arbitrary/static
point-cloud+graph form) — that **lower cleanly through the closed 0.0.8.2.5 path**, and for at least one
generated slice **admit/install + exercise GPU compact evidence** on a real adapter — under all §0
constraints, with the LIVE_GUARDRAIL battery green, and with the shape set demonstrably **registry-extensible**
(a new shape or an arbitrary layout addable without touching the producer core; PR12 documents the seam).
The UI itself is **out of scope** (a later, separate consumer); this track ends at "UI-callable producer."

## 3. Producer-side adjudications (executive design authority — spent here so rungs are mechanical)

### 3A. The lever surface (corpus-grounded — `setup_scenario` of `huge.txt` is the canonical superset)

The CLI parses (and, except where noted, *acts on*) these grouped levers. Every group is **orthogonal to
the shape** (it applies across shapes), which is exactly why shape must be a registry (C10) rather than the
sole lever:

| Group | Levers (Stellaris key → CLI flag) | Producer effect |
|---|---|---|
| **Scale & core** | `num_stars`, `radius`, `core_radius`, (`--lattice_size`) | lattice edge (square) + central void radius (no placement inside core) |
| **Shape** | `shape` (one **registered** strategy), per-shape params (`num_arms`, arm `tightness`/width, jitter) | placement strategy over the lattice (C10) |
| **Clustering** | `cluster_count` {method `one_every_x_empire`\|`constant`, value, max}, `cluster_radius`, `cluster_distance_from_core` | satellite RegionCell groups offset from the core |
| **Partitioning** | `home_system_partitions`/`open_space_partitions` {`max/min_systems`, `min/max_bridges`, `method` `breadth_first`\|`depth_first`} | **carve placed stars into RegionCells; bridges → bounded cross-partition lane couplings** (the deep structural lever; maps to the regioncell substrate) |
| **Hyperlane geometry** | `max_hyperlane_distance`, `num_hyperlanes` {min max}, `num_hyperlanes_default`, `random_hyperlanes` (yes/no) | adjacency radius cap + link-density scaling + procedural-graph vs explicit-graph toggle (all bounded by M3/M6) |
| **Special routes** | `num_wormhole_pairs`(+default), `num_gateways`(+default) | bounded long-range lane couplings (never routes/paths) |
| **Fields** | `num_nebulas`, `nebula_size`, `nebula_min_dist` | sample `field_operator`(s) → `RegionFieldSpec` operators |
| **Initializers** | per-bucket initializer keys (core/arm/fringe/cluster), `spawn_weight`, `spawn_design` | `system_initializer` refs/overrides (C5) |
| **Metadata passthrough (INERT)** | `num_empires`, `fallen/marauder/advanced_empire*`, `colonizable_planet_odds`, `primitive_odds`, `crisis_strength`, `extra_crisis_strength` | carried verbatim into scenario `metadata` as inert render data — **never generated or simulated** (these are gameplay-seeding levers a *future* consumer may read) |

### 3B. The two modes (both first-class; the corpus has both)

- **Procedural** — `setup_scenario` analog: the §3A levers + a registered shape strategy generate placements.
- **Arbitrary / static** — `static_galaxy_scenario` analog: an explicit point-cloud + explicit graph is
  admitted directly (`system{ id position initializer spawn_weight spawn_design }`, `add_hyperlane`,
  `prevent_hyperlane`, `nebula`, optional `coordinate_transform`). This is the **arbitrary-layout** path and
  the **future-proofing guarantee**: any layout an external tool can express as points+edges round-trips,
  even if the procedural shape registry never grows. Imports (point-list / lattice mask) quantize to cells.

### 3C. Core adjudications

- **C1 — Output is declarative ClauseScript scenario form** (preferred `scenario { location … link …
  field_operator … system_initializer refs }`, lowered by the existing `hydrate_scenario`/MapGen
  front-end). `static_galaxy_scenario`/manifest are admitted fallbacks. The CLI emits *text*; it never
  builds `simthing-spec`/sim structures.
- **C2 — Square lattice, one-system-per-cell** (core §7 + M5). Lattice edge derived square from
  `num_stars`/`radius` (or `--lattice_size`), default "medium" 200×200; `core_radius` masks central cells;
  quantize every placement to a free integer cell; reject/relocate collisions deterministically. No
  sub-cell positions; `position` is inert render metadata.
- **C3 — Determinism.** A single `--seed` drives all placement/partition/route choices; a secondary
  variation seed may vary initializer content while holding geometry. Same params+seed ⇒ stable (byte- or
  semantically-identical) output.
- **C4 — Bounded topology only** (M3/M6). Explicit `link`/`add_hyperlane`/wormhole/gateway within fanout +
  `max_hyperlane_distance` caps; long-range edges as bounded **lane couplings**; partition bridges within
  min/max bridge caps. Never arbitrary high-degree graphs, never routes/predecessors. `prevent_hyperlane`
  emits a negative-edge directive the front-end honors (or the producer simply omits the edge).
- **C5 — Initializer references, not new definitions.** The CLI references corpus-approved initializer
  families by key (core/arm/fringe/cluster buckets) and emits minimal `system_initializer` refs/overrides;
  it does not invent runtime semantics.
- **C6 — Field operators are samples only.** Emit ≥1 declarative `field_operator` (nebula/storm) per the
  nebula levers that lowers to a `RegionFieldSpec` operator; the CLI never computes fields or touches the runtime.
- **C7 — Standalone producer crate.** The CLI is its own thin crate/binary depending only on a parameter
  model + a tiny emitter (it need not link `simthing-*`). End-to-end *tests* live where they can feed
  output through the existing lowering + driver (as PR5/PR10 patterns do), not in the CLI crate's runtime.
- **C8 — Scale is manifest-first.** Large maps (1000+ stars) are proven at **manifest/dry-run** scale
  first (placement report + lower-check), with a *single* generated slice taken to GPU compact evidence —
  no attempt to install a galaxy-scale dense grid (the 0.0.8.2.5 ladder's bounded-theater/atlas deferrals stand).
- **C9 — Partition/bridge → regioncell substrate.** `home/open` partitions map onto RegionCells; bridges
  map onto bounded cross-region lane couplings; clusters map onto satellite RegionCell groups; nebulas map
  onto RegionFieldSpec operators. This is the corpus's deep structural mechanism and it lands natively on
  the Movement-Front substrate — **as declarative grouping/coupling metadata only**, never a planner.
- **C10 — Shapes are a data-driven REGISTRY, never a baked enum (the extensibility seam).** A
  `ShapeStrategy` interface (name + advertised params + `place(params, seed, lattice) → cells`) backed by a
  registry; each scale advertises its `supports_shape` subset (mirroring the corpus). Vanilla shapes
  (elliptical, spiral_2/3/4/6, ring, bar, starburst, cartwheel, spoked), modded shapes, and the
  arbitrary/static strategy are all *registered entries*. Adding a shape = adding a registry entry + tests;
  **the producer core, emitter, and lowering contract do not change.** Unknown shape ⇒ clean error listing
  registered shapes, never a crash or a silent fallback.
- **C11 — Arbitrary/static is a first-class strategy + an import path.** The arbitrary mode (§3B) is the
  registry's `static` strategy: explicit points+edges (+`coordinate_transform`, `prevent_hyperlane`,
  per-system `spawn_weight`/`spawn_design`/`initializer`/`nebula`) quantized to the lattice and emitted as
  declarative scenario text. An optional importer accepts an external point-list (and/or a lattice mask)
  and feeds the same path. This guarantees **future/arbitrary layouts even if procedural shapes are fixed.**

## 4. PR ladder table

| PR | Title | Owner | DA-sensitive? | Depends on |
|---|---|---|---|---|
| 0 | Track design + preserved-baseline contract (this doc landed) | Cursor (DA sign-off) | yes | — |
| 1 | CLI crate + **full §3A parameter surface** + params-file parse; **no generation** | Cursor (DA review) | C7 | 0 |
| 2 | Deterministic RNG + square lattice occupancy core (one-per-cell, `core_radius` mask) | Cursor (DA review) | C2/C3 | 1 |
| 3 | **`ShapeStrategy` registry + descriptor model** (extensibility seam) + trivial `elliptical`/uniform + `static` strategies | Cursor (DA review) | C10/C11 | 2 |
| 4 | Declarative scenario emitter (`scenario { location … }` text) for trivial strategies | Cursor (DA review) | C1/C5 | 3 |
| 5 | Generated tiny scenario **through existing MapGen lowering** (parse/hydrate) | Cursor (DA review) | C1/C7 | 4 |
| 6 | Bounded hyperlane geometry (`max_hyperlane_distance`, `num_hyperlanes`, `random_hyperlanes`, `prevent`) **+ special routes** (wormhole/gateway couplings) | Cursor (DA review) | C4 | 5 |
| 7 | **Partition/bridge structural producer** (home/open partitions → RegionCells; bridges → bounded couplings) **+ clustering** (satellite groups) | Cursor (DA review) | C4/C9 | 6 |
| 8 | **Fill the shape registry** (spiral_2/3/4/6, ring, bar, starburst, cartwheel, spoked) as registered strategies (curve math quantized) | Cursor (DA review) | C10 | 7 |
| 9 | Nebula `field_operator` emission (`num_nebulas/size/min_dist`) + initializer buckets + inert metadata passthrough | Cursor | C5/C6 | 8 |
| 10 | Generated shaped slice → admission/install + **GPU compact evidence** | Cursor (DA review) | all | 6,9 |
| 11 | **Arbitrary/static import** (explicit points+edges, `coordinate_transform`, external point-list/mask) **+ scale-envelope dry run** (1000-star manifest only) | Cursor (DA review) | C8/C11 | 10 |
| 12 | Closeout + ledger + **UI handoff + extensibility note** (how to register a shape / supply an arbitrary layout) | Cursor (DA sign-off) | — | 11 |

"DA review" = executive design authority reviews the merge diff against §0/§3; **genuine audit + battery
rerun + the report must not pre-file a DA sign-off** (the 0.0.8.2.5 PR3 governance rule carries forward —
only the Design Authority writes a DA sign-off). A rung stops/escalates only on a §7 stop condition.

## 5. DA cadence

DA-review-sensitive rungs (genuine pre-merge audit): **1** (crate boundary + full lever surface — no
sim/kind/runtime), **2** (determinism + one-system-per-cell + core mask), **3** (registry seam is genuinely
data-driven; no baked enum; `static` admitted), **4** (declarative output shape), **5** (lowers through the
*closed* path unchanged), **6** (bounded hyperlanes + routes, no graph/path), **7** (partition/bridge stays
bounded couplings, no planner), **8** (registry fill changes no core/emitter/contract), **10**
(admission/install + GPU compact evidence, no full-field readback), **11** (arbitrary import quantizes &
bounds; scale stays manifest-only). **Sign-off** on **0** (track open) and **12** (closeout). PR9 is
mechanical under §3.

## 6. Per-rung acceptance/stop (granular handoffs)

- **PR1** — new CLI crate (`crates/simthing-mapgenerator`) + arg/params-file parsing of the **entire §3A
  surface** (scale/core, shape+params, clustering, partitioning, hyperlane geometry, special routes,
  nebulas, initializer buckets, inert metadata); prints parsed params in dry-run; **no placement, no
  emission.** Accept: builds; no `simthing-*` runtime dep; no sim/kind; inert metadata parsed but flagged
  non-generating. Stop: a param needs sim/spec types (§7).
  **Status: PASS pending DA review (2026-06-14).** Result:
  [`tests/mapgenerator_cli_pr1_params_results.md`](tests/mapgenerator_cli_pr1_params_results.md) (PROBATION).
  **PR2 next:** deterministic RNG + square lattice occupancy core.
- **PR2** — seeded RNG (pinned algorithm) + square-lattice occupancy set; `core_radius` masks central
  cells; place N points one-per-cell with deterministic collision relocation. Accept: same seed ⇒ identical
  occupancy; square; no sub-cell coords. Stop: needs Euclidean *authority* (only quantized float allowed).
- **PR3** — `ShapeStrategy` trait + registry + descriptor (advertised name/params); register `elliptical`
  (uniform-ish disc) and `static` (passthrough). Accept: strategies resolved by name from the registry;
  unknown shape ⇒ clean error listing registered shapes; adding a strategy touches only registry+tests.
  Stop: registry pressure to encode runtime semantics (§7).
- **PR4** — emit declarative `scenario { metadata, location { position(inert) initializer-ref } }` text from
  a strategy's placements. Accept: text only; positions inert; no links/fields yet. Stop: emitter wants to
  build spec structs (§7).
- **PR5** — feed PR4 text through `parse_mapgen_neutral_document` → `hydrate_scenario`/MapGen front-end;
  assert it lowers to the same surfaces (gridcell `Location`s, default-off). Accept: lowers **without
  changing the lowering layer**; LIVE_GUARDRAIL battery green. Stop: front-end rejects ⇒ fix the *producer
  output*, never the front-end (§7).
- **PR6** — emit bounded `link`/`add_hyperlane` honoring `max_hyperlane_distance` adjacency +
  `num_hyperlanes` density scaling + `random_hyperlanes` toggle + `prevent_hyperlane`; emit wormhole-pair /
  gateway **special routes** as bounded long-range lane couplings (per defaults/min-max). Accept: M3/M6 caps
  respected; no arbitrary graph; no route/predecessor; lowers green. Stop: a route/path is implied (§7).
- **PR7** — partition placed stars into home/open RegionCells (`max/min_systems`, `method`
  breadth/depth_first) and connect partitions with `min/max_bridges` bounded couplings; emit `cluster_count`
  satellite groups offset by `cluster_distance_from_core`. Accept: bounded; deterministic; declarative
  grouping/coupling only; lowers green. Stop: partitioning implies CPU planning/pathing over fields (§7).
- **PR8** — register the remaining vanilla shapes (spiral_2/3/4/6, ring, bar, starburst, cartwheel, spoked)
  as strategies; arm/curve/jitter float math quantized to cells; per-scale `supports_shape` advertisement.
  Accept: each shape deterministic, square, one-per-cell; **producer core/emitter/lowering contract
  unchanged** by adding shapes (proves C10).
- **PR9** — `num_nebulas`/`nebula_size`/`nebula_min_dist` → ≥1 `field_operator` lowering to a
  `RegionFieldSpec` operator; initializer-bucket assignment (core/arm/fringe/cluster) referencing corpus
  families; carry inert metadata into scenario `metadata`. Accept: refs only; field_operator lowers;
  metadata never generated/simulated.
- **PR10** — take one generated (shaped) slice through `install_atomic` + `SimSession::open_from_spec` +
  GPU mapping tick with **compact evidence only** (`field_values`/`reduction_parent_value`/`eml_output`
  `is_none()`), GPU-adapter gated, default-off. Accept: mirrors the PR10 0.0.8.2.5 harness; real GPU run.
  Stop: needs full-field readback or new kernel (§7).
- **PR11** — admit the arbitrary/static form (explicit `system`+`add_hyperlane`/`prevent_hyperlane`+`nebula`,
  `coordinate_transform`, per-system `spawn_weight`/`spawn_design`); optional importer for an external
  point-list / lattice mask → quantized to cells; lower-check (parse/hydrate admits). Then a 1000-star (or
  chosen scale) **manifest/dry-run only** + lower-check; **no galaxy-scale dense install**. Accept:
  arbitrary layout round-trips & lowers; manifest emitted; bounded. Stop: import needs Euclidean authority
  or unbounded degree (§7).
- **PR12** — closeout report + ledger; classify CLI artifacts; **UI handoff note** (the UI is the next,
  separate consumer) + an **extensibility note** (register a new `ShapeStrategy`, or supply an
  arbitrary/static layout, without touching the producer core or the lowering contract). Accept: docs-only;
  honest; battery green.

## 7. Stop conditions (escalate → PARTIAL; do not improvise)

Halt and escalate if a rung needs: a change to the closed 0.0.8.2.5 lowering layer / `simthing-spec` /
`simthing-sim`; a new `SimThingKind`; a new GPU kernel or semantic WGSL; Euclidean *authority* in the
lowered output (M9/§0.7); routes/predecessors/movement/border/frontline/pathfinding; CPU planning or
full-field readback; arbitrary high-degree/non-grid topology; generating/simulating the inert metadata
passthrough; vendored Paradox files; opening FIELD-MOVIE-DATASET-0; or galaxy-scale dense install
(atlas/multi-theater remains deferred).

## 8. References

- Reference (target direction): [`clausething/MapGeneratorCLI.md`](clausething/MapGeneratorCLI.md).
- Corpus deep function (read-only, **not vendored**): `C:\Users\mvorm\Clauser\Paradox\vanilla\map\setup_scenarios\{huge,large,medium,small,tiny}.txt` (procedural lever surface + `supports_shape` set) and `static_galaxy_example.txt` (the arbitrary/static form).
- The closed contract this produces for: [`design_0_0_8_2_5_mapgen_ladder.md`](design_0_0_8_2_5_mapgen_ladder.md) (§3 M1–M11), [`tests/mapgen_pr11_closeout_results.md`](tests/mapgen_pr11_closeout_results.md), [`tests/mapgen_pr10_end_to_end_results.md`](tests/mapgen_pr10_end_to_end_results.md).
- Grammar/isomorphism: [`clausething/MapGenThing.md`](clausething/MapGenThing.md); corpus manifest: [`clausething/mapgen_corpus_manifest.md`](clausething/mapgen_corpus_manifest.md).
- Paradigm/surfaces: [`simthing_core_design.md`](simthing_core_design.md) §1.1+§7; [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md); [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md); Candidate F [`design_0_0_8_1.md`](design_0_0_8_1.md) §0.7.
