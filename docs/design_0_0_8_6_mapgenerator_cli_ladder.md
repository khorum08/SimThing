# SimThing 0.0.8.6 — MapGeneratorCLI PR Ladder (high-level galaxy params → declarative MapGen payloads)

> **Status: CLOSED — DA-APPROVED (2026-06-15, #693). PR1–PR11 DA-APPROVED & MERGED (#674–#692). PR12
> (docs-only closeout ledger + UI handoff + extensibility note) — PROBATION pending DA review. Next track:
> FIELD-MOVIE-DATASET-0 unless DA reorders.** (Executive summary of PR11: 1000-star producer generation +
> parse/lattice-lower proven; RF/admit/GPU-at-scale blocked by closed RF slot caps — DA-authorized 0.0.8.2.5
> amendment candidate, not widened. PR10 tiny-fixture GPU compact evidence remains LIVE guardrail.)
> This is the planning artifact for the **producer layer** above the now-closed 0.0.8.2.5 MapGen
> ingest/lowering ladder. It is not an implementation PR. It pins the producer-side schema judgments (§3)
> so the rungs are Cursor-mechanical with Codex reviews, and it is **subordinate to the core-design
> paradigm, the two governing ADRs, and the closed 0.0.8.2.5 contract (§0).**
>
> **What MapGeneratorCLI is.** A **thin, standalone, declarative producer**: it turns the high-level galaxy
> lever surface (§3A — scale, **a registered shape**, clustering, partitioning/bridges, hyperlane geometry,
> special routes, nebula fields, plus an inert metadata passthrough) into **declarative MapGen neutral-AST
> text** that the *already-closed* 0.0.8.2.5 MapGen front-end ingests and lowers. **Current proven PR4/PR5
> output path:** MapGeneratorCLI emits `static_galaxy_scenario` neutral-AST text (not `hydrate_scenario`
> `scenario/location` grammar). **Later rungs** may add `add_hyperlane` and `field_operator` declarations
> only through already-accepted closed MapGen surfaces — never by widening `hydrate_scenario` or the lowerer.
> **It is NOT a runtime, NOT the MapGen lowerer, NOT a UI** — it is the missing "Galaxy Shape Generator"
> between UI levers and the lowering path.
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
>
> **Mandatory closed-`src/` gate (PR5+):** If any file under `crates/simthing-clausething/src/` would change,
> **stop immediately** — do not continue the producer PR. Open a separate DA-authorized 0.0.8.2.5 amendment PR,
> merge it, then resume the producer proof PR with zero closed `src/` changes.

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
- **Current proven output (PR4/PR5):** `static_galaxy_scenario` neutral-AST text (`system { id position initializer }`
  plus sibling `*_initializer` definitions) — lowered by the existing neutral-AST parser and closed
  `mapgen_lattice` path; **not** `hydrate_scenario` `scenario/location` grammar.
- **Later rungs (PR6+):** may add `add_hyperlane`, `field_operator`, and related declarations only through
  already-accepted closed MapGen surfaces; never by widening `hydrate_scenario` or the lowerer.
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

- **C1 — Output is declarative MapGen neutral-AST text.** **Current proven path (PR4/PR5):**
  `static_galaxy_scenario` blocks lowered via `parse_mapgen_neutral_document` → closed `mapgen_lattice`.
  **Later rungs** may add `add_hyperlane` / `field_operator` through already-accepted closed surfaces only —
  not by reopening `hydrate_scenario` or widening the lowerer. The CLI emits *text*; it never builds
  `simthing-spec`/sim structures.
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
| 4 | Declarative `static_galaxy_scenario` emitter for trivial strategies | Cursor (DA review) | C1/C5 | 3 |
| 5 | Generated tiny scenario **through existing MapGen lowering** (neutral AST → lattice) | Cursor (DA review) | C1/C7 | 4 |
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
  **Status: DA-APPROVED (2026-06-14, #674).** Result:
  [`tests/mapgenerator_cli_pr1_params_results.md`](tests/mapgenerator_cli_pr1_params_results.md) (CURRENT_EVIDENCE).
- **PR2** — seeded RNG (pinned algorithm) + square-lattice occupancy set; `core_radius` masks central
  cells; place N points one-per-cell with deterministic collision relocation. Accept: same seed ⇒ identical
  occupancy; square; no sub-cell coords. Stop: needs Euclidean *authority* (only quantized float allowed).
  **Status: DA-APPROVED (2026-06-14, #676).** Result:
  [`tests/mapgenerator_cli_pr2_lattice_results.md`](tests/mapgenerator_cli_pr2_lattice_results.md) (CURRENT_EVIDENCE).
- **PR3** — `ShapeStrategy` trait + registry + descriptor (advertised name/params); register `elliptical`
  (uniform-ish disc) and `static` (passthrough). Accept: strategies resolved by name from the registry;
  unknown shape ⇒ clean error listing registered shapes; adding a strategy touches only registry+tests.
  Stop: registry pressure to encode runtime semantics (§7).
  **Status: DA-APPROVED (2026-06-14, #677).** Result:
  [`tests/mapgenerator_cli_pr3_strategy_results.md`](tests/mapgenerator_cli_pr3_strategy_results.md) (CURRENT_EVIDENCE).
- **PR4** — emit declarative `static_galaxy_scenario` neutral-AST text from a strategy's placements (single
  root block, `system { id position initializer }`, sibling minimal `*_initializer` definition). Accept: text
  only; positions inert; no links/fields yet; **not** `hydrate_scenario` `scenario/location` grammar. Stop:
  emitter wants to build spec structs or widen the closed front-end (§7).
  **Status: DA-APPROVED (2026-06-14, #678).** Result:
  [`tests/mapgenerator_cli_pr4_emitter_results.md`](tests/mapgenerator_cli_pr4_emitter_results.md) (CURRENT_EVIDENCE).
- **PR5** — feed PR4 text through `parse_mapgen_neutral_document` → `generate_mapgen_lattice_hierarchy` on the
  **amended** closed lowerer; assert gridcell `Location`s. Accept: **zero** `crates/simthing-clausething/src/`
  changes in PR5; `simthing-mapgenerator` dev-dependency only; every system emits initializer bareword; LIVE_GUARDRAIL
  battery green. Stop: front-end rejects ⇒ fix producer output; closed defect ⇒ split to 0.0.8.2.5 amendment PR (§0 gate).
  **Status: DA-APPROVED (2026-06-14, #682, superseding auto-closed #679).** Requires merged child-id amendment (#680) first. Result:
  [`tests/mapgenerator_cli_pr5_lowering_results.md`](tests/mapgenerator_cli_pr5_lowering_results.md) (CURRENT_EVIDENCE).
- **PR6** — emit bounded `link`/`add_hyperlane` honoring `max_hyperlane_distance` adjacency +
  `num_hyperlanes` density scaling + `random_hyperlanes` toggle + `prevent_hyperlane`; emit wormhole-pair /
  gateway **special routes** as bounded long-range lane couplings (per defaults/min-max). Accept: M3/M6 caps
  respected; no arbitrary graph; no route/predecessor; lowers green. Stop: a route/path is implied (§7).
  **Status: DA-APPROVED & MERGED (2026-06-14, #684) — HYPERLANES ONLY.** Bounded `add_hyperlane` emission +
  closed-link-surface lowering proof (`extract_hyperlane_declarations` → `lower_hyperlane_topology`), undirected
  pairs, Chebyshev bound on lowered index-order positions, fanout cap 4, prevent-list, no route/predecessor; zero
  closed `src/`. **Special routes (wormhole-pair / gateway long-range lane couplings) COMPLETED in PR6b (#686,
  DA-APPROVED):** `special_routes.rs` selects bounded long-range non-N4 pairs honoring `num_wormhole_pairs`/
  `num_gateways`, fails closed on `UnsatisfiedRouteCount`, lowers as `add_hyperlane` → lane couplings (kind is
  producer-report-only, not in grammar). **PR6 rung scope now COMPLETE.** Result:
  [`tests/mapgenerator_cli_pr6_hyperlane_results.md`](tests/mapgenerator_cli_pr6_hyperlane_results.md) +
  [`tests/mapgenerator_cli_special_routes_results.md`](tests/mapgenerator_cli_special_routes_results.md)
  (both CURRENT_EVIDENCE). DA note: `generate_hyperlane_topology` **and** `generate_special_routes` are O(N²)
  candidate enumeration — bound before the PR11 scale rung.
- **PR6R** — record correction + fail-closed `HyperlaneOptions` validation (`InvalidEdgeCounts`, `InvalidFanoutCap`,
  `UnsatisfiedMinEdgeCount`). Accept: zero closed `src/`; PR6 happy path unchanged; invalid public options return
  `Err` not panic. Stop: needs route/path semantics or closed-front-end widening (§7).
  **Status: DA-APPROVED & MERGED (2026-06-14, #685).** Result:
  [`tests/mapgenerator_cli_pr6r_hardening_results.md`](tests/mapgenerator_cli_pr6r_hardening_results.md) (CURRENT_EVIDENCE).
- **PR6b** — bounded wormhole-pair / gateway special-route endpoint selection represented **only** as existing
  `add_hyperlane` declarations; long-range pairs lower as bounded lane couplings via closed `mapgen_links` (no new
  grammar). Accept: `num_wormhole_pairs` / `num_gateways` bounded; deterministic; fail-closed when impossible; zero
  closed `src/`; no route/predecessor/path/movement/border/frontline. Stop: new wormhole/gateway grammar or closed
  lowerer amendment required (§7).
  **Status: DA-APPROVED & MERGED (2026-06-14, #686).** Result:
  [`tests/mapgenerator_cli_special_routes_results.md`](tests/mapgenerator_cli_special_routes_results.md) (CURRENT_EVIDENCE).
  **PR7 next:** partition/bridge structural producer + clustering — no route/path/predecessor semantics and no GPU.
- **PR7** — bounded producer-side partition/cluster assignment and cross-group bridge endpoint selection
  represented **only** as existing `add_hyperlane` declarations; partition/cluster identities in producer reports
  only. Accept: bounded; deterministic; fail-closed when impossible; dedup against hyperlanes/special routes; fanout
  cap; lowers through closed `mapgen_links`. Stop: new partition/cluster/bridge grammar, route/path semantics, or
  closed lowerer amendment (§7).
  **Status: DA-APPROVED & MERGED (2026-06-14, #687).** Bounded partition (BFS/DFS method mirror of Stellaris
  `home_system_partitions`) + single-pass anchor clustering; bridges = bounded cross-group `add_hyperlane`
  couplings; fail-closed throughout; zero closed `src/`; lowers via closed `mapgen_links`. **DA constitutional
  ruling:** the producer-side BFS/DFS partition ordering is the offline generator's Stellaris `method` mirror,
  **not** runtime pathfinding (no source→target/predecessor/field-planning; nothing traversal-related reaches
  output) — approved, do not flag as pathfinding. DA note: O(N²) adjacency/bridge enumeration — bound before PR11.
  Result: [`tests/mapgenerator_cli_pr7_partition_bridge_results.md`](tests/mapgenerator_cli_pr7_partition_bridge_results.md) (CURRENT_EVIDENCE).
  **PR8 next:** registry completion / remaining vanilla shape descriptors / single-source executable strategy
  dispatch, unless DA reorders.
- **PR8** — register the remaining vanilla shapes (spiral_2/3/4/6, ring, bar, starburst, cartwheel, spoked)
  as strategies; arm/curve/jitter float math quantized to cells; **single-source** `ShapeStrategyEntry` registry
  (descriptor + executable strategy in one map — no parallel `executable_strategy_names` / `strategy_by_name`
  lists). Accept: each shape deterministic, square, one-per-cell; **producer core/emitter/lowering contract
  unchanged** by adding shapes (proves C10); static/arbitrary_static procedural mode-gate unified.
  **Status: DA-APPROVED & MERGED (2026-06-14, #688).** All 12 vanilla shapes executable from one
  `vanilla_entries()` `BTreeMap` (descriptor + dispatch single-sourced — parallel `strategy_by_name`/
  `executable_strategy_names` REMOVED; adding a shape = one row + module, proving C10); polar float sampling
  quantized to integer cells (no Euclidean authority in output); procedural mode-gate now rejects both `static`
  and `arbitrary_static`. **Closes the two carried PR3 notes (single-source dispatch + mode gate).** Zero closed
  `src/`; spiral_4/ring lower through closed surfaces. Result:
  [`tests/mapgenerator_cli_pr8_shape_registry_results.md`](tests/mapgenerator_cli_pr8_shape_registry_results.md) (CURRENT_EVIDENCE).
  **PR9 next:** nebula / field_operator declarative producer — no GPU/runtime.
- **PR9** — bounded producer-side nebula placement (`num_nebulas`/`nebula_size`/`nebula_min_dist`) → closed
  `static_galaxy_scenario` `nebula = { name radius }` feedstock; initializer-bucket bareword refs with sibling
  definitions emitted once; inert metadata captured in dry-run report only (deferred from scenario text). Accept:
  parse + lattice + Movement-Front `RegionFieldSpec` lowering through existing closed surfaces; zero closed
  `src/`; no GPU/runtime. Stop: new grammar, closed lowerer amendment, or metadata emission requiring widening (§7).
  **Status: DA-APPROVED & MERGED (2026-06-14, #689).** Emits only accepted `nebula` keys (`name`, `radius`);
  Movement-Front produces SaturatingFlux from PR3–PR6 pipeline; metadata passthrough explicitly deferred. Result:
  [`tests/mapgenerator_cli_pr9_field_operator_results.md`](tests/mapgenerator_cli_pr9_field_operator_results.md) (CURRENT_EVIDENCE).
  **PR10 next:** generated scenario admit/install + GPU compact evidence on real adapter.
- **PR10** — take one **MapGeneratorCLI-generated** (shaped) slice through parse/lowering → `install_atomic` +
  `SimSession::open_from_spec` + GPU mapping tick with **compact evidence only** (`field_values`/`reduction_parent_value`/`eml_output`
  `is_none()`), GPU-adapter gated, default-off. Accept: mirrors the 0.0.8.2.5 MapGen PR10 harness; real GPU run;
  zero closed `src/` edits. Stop: needs full-field readback or new kernel (§7).
  **Status: DA-APPROVED & MERGED (2026-06-15, #690).** Five-system static layout admits/installs within RF slot cap;
  extended nine-system layout proves special-route `add_hyperlane` lowering (parse/links only). Result:
  [`tests/mapgenerator_cli_pr10_gpu_compact_evidence_results.md`](tests/mapgenerator_cli_pr10_gpu_compact_evidence_results.md) (CURRENT_EVIDENCE).
  **PR11 next:** scale-envelope proof / 1000-star generated map stress.
- **PR11** — prove the **1000-star producer scale envelope** and close carried scale risks: lattice capacity
  overflow (`u32` edge²), occupancy relocation scaling, bounded topology/special-route/partition/cluster pair
  enumeration (`PRODUCER_PAIR_CANDIDATE_CAP`), 1000-star `static_galaxy_scenario` generation + parse/lattice proof,
  honest admission/GPU status under closed RF slot caps (no galaxy-scale dense install). Accept: producer-only
  hardening in `simthing-mapgenerator`; integration tests in `simthing-clausething`; PR10 GPU harness remains live;
  zero closed `src/` edits. Stop: needs closed lowerer widening or galaxy-scale GPU install (§7).
  **Status: DA-APPROVED & MERGED (2026-06-15, #692; DA min-heap remediation closed the O(N²·cap) time cliff,
  43s→1s).** All three carried scale notes CLOSED (overflow fail-closed; relocation de-O(cells)'d; long-range
  enumeration O(log cap)/pair). 1000-star producer generation + parse/lattice-lower proven; RF/admit/GPU-at-scale
  honestly BLOCKED by closed RF slot caps (DA-authorized 0.0.8.2.5 amendment candidate, not widened here). Result:
  [`tests/mapgenerator_cli_pr11_scale_envelope_results.md`](tests/mapgenerator_cli_pr11_scale_envelope_results.md) (CURRENT_EVIDENCE).
- **PR12** — closeout report + ledger; classify CLI artifacts; **UI handoff note** (the UI is the next,
  separate consumer) + an **extensibility note** (register a new `ShapeStrategy`, or supply an
  arbitrary/static layout, without touching the producer core or the lowering contract). Accept: docs-only;
  honest; no code. Stop: any `crates/` path in diff (§7).
  **Status: DA-APPROVED & MERGED (2026-06-15, #693) — closeout sign-off.** Docs-only closeout ledger, artifact lifecycle
  promotion, UI/editor handoff, extensibility note, RF cap amendment candidate, FIELD-MOVIE-DATASET-0 pointer.
  Result: [`tests/mapgenerator_cli_pr12_closeout_results.md`](tests/mapgenerator_cli_pr12_closeout_results.md)
  (CURRENT_EVIDENCE). **0.0.8.6 MapGeneratorCLI — CLOSED (DA-APPROVED 2026-06-15, #693).**

### Track closeout table (PR12)

| Rung | Status | Evidence class |
|---|---|---|
| PR1 | COMPLETE | CURRENT_EVIDENCE |
| PR2 | COMPLETE | CURRENT_EVIDENCE |
| PR3 | COMPLETE | CURRENT_EVIDENCE |
| PR4 | COMPLETE | CURRENT_EVIDENCE |
| PR5 | COMPLETE | CURRENT_EVIDENCE |
| PR6 | COMPLETE | CURRENT_EVIDENCE |
| PR6R | COMPLETE | CURRENT_EVIDENCE |
| PR6b | COMPLETE | CURRENT_EVIDENCE |
| PR7 | COMPLETE | CURRENT_EVIDENCE |
| PR8 | COMPLETE | CURRENT_EVIDENCE |
| PR9 | COMPLETE | CURRENT_EVIDENCE |
| PR10 | COMPLETE | CURRENT_EVIDENCE + LIVE GPU GUARDRAIL |
| PR11 | COMPLETE | CURRENT_EVIDENCE |
| PR12 | COMPLETE (DA-APPROVED #693) | CURRENT_EVIDENCE |

**Honest 1000-star scope (PR11, binding):** producer generation, parse, and lattice lower are proven.
1000-star RF/admit/install/GPU remain blocked by closed RF lowerer caps. No closed lowerer caps were widened
in PR11.

**RF cap amendment (deferred):** future galaxy-scale admission/install requires a DA-authorized 0.0.8.2+
closed-lowerer capacity amendment (raise/scalable RF participant/slot caps and/or scalable deposit initializer
feedstock). No producer-only patch may silently bypass the gate.

**Next track:** FIELD-MOVIE-DATASET-0 unless DA reorders.

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
