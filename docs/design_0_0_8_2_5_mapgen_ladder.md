# SimThing 0.0.8.2.5 — MapGen PR Ladder (Stellaris Starmap → SimThing Star Mapping)

> **Status: PR1 PASS (2026-06-13); PR2 PASS — DA-APPROVED + merged 2026-06-13 (`edeab38a`); PR3 PASS — genuine DA sign-off (Opus, 2026-06-14, `67d6ab8c`); PR4 PASS — DA-APPROVED after a targeted DA repair (Opus, 2026-06-14); PR5 PASS — DA-APPROVED + merged 2026-06-14 (`172d0c47`); PR6 PASS — DA-APPROVED + merged 2026-06-13 (`3f411fda`); PR7 PASS pending merge (2026-06-13); PR8 may proceed (DA review).** Planning
> artifact that **pulls the deferred corpus-import / map-generation consumer** named in the
> 0.0.8.2 closeout. Not an implementation PR. It pins schema judgments (§3) so the rungs are
> Cursor-mechanical, and it is **subordinate to the core-design paradigm and two governing ADRs (§0).**
>
> **What MapGen is.** MapGen *ingests* ClauseScript / Stellaris mapping script and *generates*
> **SimThing star mapping**: a **galaxy of gridcell-SimThings run as the Movement-Front cellular
> automaton**, with resource-flow arena enrollment feeding it. It is a **front-end / generator** onto
> already-closed substrates — not a new runtime, not a Stellaris importer, not a whole-game decoder.
>
> **0.0.8.2.5 extends, it does not reopen sign-off.** The 0.0.8.2 DA sign-off
> ([`tests/clausething_closeout_results.md`](tests/clausething_closeout_results.md)) **stands.** MapGen
> names the consumer the closeout deferred (PALMA "awaiting a named consumer"; BH-3 authoring). The
> editor/corpus/export seam (`FIELD-MOVIE-DATASET-0`) remains the **subsequent** track.

---

## 0. Binding paradigm + ADRs — READ FIRST (non-negotiable)

> Every rung is subordinate to the core-design paradigm and two ADRs. **If a rung's behavior is not
> expressible within all three, it STOPS and escalates (§9) — it never improvises a wider substrate.**
> The executive design authority drifted from these twice while sketching this track on 2026-06-13
> (once into dense-global diffusion, once over-correcting into "no galaxy grid"); that is exactly the
> drift Cursor will reproduce if these are not load-bearing in every rung. **Re-read the cited section
> before writing any rung.**

**ANCHOR / core paradigm — [`simthing_core_design.md`](simthing_core_design.md) §1.1 (Anchor A) + §7
(this OUTRANKS the ADRs where scale is concerned).** The map **is** a grid of **gridcell SimThings run
as a cellular automaton** — the **Movement-Front automaton**, the engine-native realization of Zichao
Wei, *On the Spatiotemporal Dynamics of Generalization in Neural Networks*
([arXiv:2602.01651](https://arxiv.org/abs/2602.01651)). (Wei's concept is referred to in prose as
**STEAD** — *SpatioTemporal Evolution with Attractor Dynamics* — never the military-connoted "SEAD";
the engine name agents use in code/spec/tests is **Movement-Front**. Do not write "SEAD" anywhere.)
- **The galaxy IS a 2D gridcell lattice** — **base canonical dimensions are always square** (default
  "medium" **200×200**, scaling up *square* when star density demands more cells). Star systems **occupy a
  subset** of cells; empty cells are deep space carrying ambient field. A gridcell **is a `Location`
  SimThing** — the spatial/grid identity is **intrinsic** to the `Location` kind, not a detachable role;
  **not a new `SimThingKind`** — with its RegionField columns laid out positionally `(width, height, col)`
  so the stencil walks neighbors by index arithmetic.
- **The three postulates are engine law.** **P1 Locality:** a cell's next state depends only on its
  stencil neighborhood; fronts advance at **finite speed** (H ≤ 8 tactical per tick, later-band cascade);
  they propagate across the galaxy **over many ticks** (light cone). What is **permanently rejected** is
  **widening the horizon to gain instant strategic awareness** — that is action-at-a-distance; the cure
  is **hierarchy (Layer 2), not a bigger light cone.** **P2 Symmetry:** one shared
  `StructuredFieldStencilOp` kernel + authored weights at every cell; no per-cell bespoke rule, no
  semantic WGSL. **P3 Stability:** stability-bounded operators, ping-pong, and **threshold crossings as
  the discrete attractor projection** (a decision is the automaton projecting continuous pressure onto a
  discrete attractor). Compute is adaptive: **cadence tiers + dirty skipping make compute follow the
  wavefront** — quiet regions cost nothing.
- **Three layers (§7.2):** **L1** the stencil evolves cell columns **across the 2D lattice** (values
  spill with falloff; the moving contour where opposing pressures meet **is the front**); **L2** Sum-reduce
  cells → parent columns for strategic awareness (never a wider stencil); **L3** parent `EvalEML` →
  threshold → commitment. The Movement-Front production operators are **Gu-Yang `SaturatingFlux`** (the
  conservative-flux stencil rule; saturation/choke) and **PALMA** min-plus (the reach/impedance utility;
  "the front is the route").

**ADR-MAP — [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md) (Approved 2026-05-28).**
The RegionCell architecture under the paradigm:
- **A RegionCell/gridcell IS a `Location` SimThing** (intrinsic spatial/grid identity), backed by
  RegionField columns at a `(width, height, col)` slot — **not** a new `SimThingKind` and **not** a
  detachable mapping-role (core §7). `simthing-sim` stays semantic-free (flat columns + opaque `AccumulatorOp`).
- **The rejected pattern is *long-horizon dense diffusion as a strategic-awareness shortcut*** (≈15×
  over budget) — i.e. P1. It is **not** a ban on the galaxy lattice itself: the Movement-Front L1 stencil
  runs across the 2D lattice with **bounded per-tick horizon + cadence/dirty**, and strategic rollup is
  L2 hierarchy. The ADR's ≤ 32×32 first slice was the conservative first implementation, **not** the
  ceiling — the canonical scale is the §7 galaxy lattice.
- Operators: `source_capped_normalized` default; H ≤ 8 (≤ 16 gated); ping-pong for H > 1; caller-managed
  one-shot seed-then-zero. Opt-in/default-off (`MappingExecutionProfile::Disabled`). Atlas batching,
  active masks, perception, behavioral source policy are **Provisional/Deferred** with hard gates —
  `request_atlas_batching` stays rejected at admission.

**ADR-RF — [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md) (Accepted 2026-05-26).**
The economy + the suppression front that feed the automaton:
- **Hierarchical fanout absorption is a scaling mechanism** — `galaxy → sector → system → planet →
  deposit`, each level ≤ ~100 children — the same hierarchy as ADR-MAP L2. (Combined with P1's
  compute-follows-the-wavefront, this is why a galaxy-scale lattice is cheap.)
- **Many named arenas** by column range (food, research, **a suppression arena**, …). The
  **suppression/disruption front is a resource-flow arena**, not a bespoke field; its pressure columns
  **are the cell-state columns the Movement-Front evolves.**
- **Four constitutional rules:** capability universal; **participation explicit** (selector admission —
  property possession never admits); **expansion bounded** (every arena declares `max_participants`,
  `max_coupling_fanout`, `max_orderband_depth`); **unsafe content rejected at session build.** The spec
  compiler emits an **expansion report.**
- Per-coupling delay form; all-`Algebraic` cycle rejected. `FissionPolicy ∈ {Inherit, Reevaluate(default),
  Reject}`. `AccumulatorRole` is compile-time metadata. `ArenaRegistry` lives in `simthing-driver`;
  `simthing-sim` never sees it. **No new GPU primitive.**

**Read order for any rung:** `docs/invariants.md` → core design §1.1 + §7 → ADR-MAP → ADR-RF → this
ladder §3 → closed surfaces (`clausething/ct_vertical_consumer_contract.md`, CT-2c economy) →
[`clausething/MapGenThing.md`](clausething/MapGenThing.md) for Stellaris-side detail.

---

## 1. What MapGen ingests and what it generates

**Ingest:** raw Stellaris/Clausewitz mapping script (solar_system_initializers, setup_scenarios,
static_galaxy_scenario, add_hyperlane, deposits, nebulas) and/or ClauseScript scenario authoring, via
the jomini neutral-AST path.

**Generate (SimThing star mapping — coordinated outputs on already-closed surfaces):**

| Generated output | Target surface (exists / closed) | Governing source |
|---|---|---|
| **Galaxy gridcell lattice** — star systems = **gridcell SimThings** occupying cells of the galaxy 2D map; empty cells = deep space | `scenario` + `hydrate_scenario` → root tree; gridcell = mapping-role on a SimThing (no new kind) | core §7, ADR-MAP |
| **Spatial hierarchy** — galaxy → sector → system → planet/structure → deposit (`children`) | hierarchical tree; CT-2c intrinsic flows on deposits | core §2/§7, ADR-RF |
| **Resource-flow arena enrollment** — deposits = `IntrinsicFlow`; system/sector = allocators; economy arenas + a **suppression arena** (the front source) | CT-2c economy: `ResourceFlowSpec`, gated/`value:` rates, RF arenas, `pressure_binding`; ArenaRegistry | ADR-RF |
| **Movement-Front heatmap** — suppression-arena pressure = cell columns; L1 Gu-Yang stencil across the lattice (bounded horizon), L2 hierarchy, L3 parent EML → threshold → commitment | `RegionFieldSpec` + `SaturatingFlux` (L1) + `SlotRange` Sum (L2) + `ai_will_do` EvalEML (L3) + `FirstSliceCommitmentSpec` | core §7, ADR-MAP |
| Hyperlane adjacency | `link` → bounded grid/lane-coupling metadata (never a graph object) | closeout A2 · M6 |
| Reach / influence | composed-W → `min_plus_traversal_field` D (PALMA) | core §7, BH-2C · M7 |

**Missing (the whole track):** there is no adapter from Stellaris/ClauseScript mapping script into these
generated outputs. MapGen builds a **thin, slice-scoped** generator — no whole-game importer, no new
substrate, no new GPU kernel. Corpus is read-only at `C:\Users\mvorm\Clauser\Paradox\`, **not vendored** (§3 M10).

## 2. MapGen closure definition (what 0.0.8.2.5 closes)

Closed when a **single starmap slice** (≤ 5 systems, corpus-derived, hand-authored fixtures) is
*ingested* and *generates* SimThing star mapping that:

1. **Parses** raw script into the neutral AST, **no semantic decisions in the parse pass.**
2. **Generates the gridcell-lattice spatial hierarchy** (galaxy 2D map → sector → system(gridcell) →
   planet → deposit) via `hydrate_scenario`, with **no new `SimThingKind`** and gridcells as
   **mapping-roles on SimThings** (core §7).
3. **Generates resource-flow arena enrollment** — deposits → intrinsic flow; a bounded shallow allocator
   hierarchy; **every arena declares selectors + caps + FissionPolicy**, passes the ADR-RF firewall, and
   emits a clean **expansion report.**
4. **Generates the Movement-Front heatmap** — suppression-arena pressure as cell columns; **L1 Gu-Yang
   stencil across the lattice with bounded per-tick horizon (P1; H ≤ 8; `source_capped_normalized`;
   ping-pong; cadence/dirty);** **L2 hierarchy reduction** for strategic awareness; **L3** parent EML →
   threshold → commitment (P3 attractor projection). **No horizon-widening-as-strategic-shortcut.**
5. **Admits/installs** through the driver and **exercises the GPU-resident path** (resource-flow reduction
   → Movement-Front L1/L2/L3 commitment) with **compact evidence only** (no full-field CPU decision readback).
6. **Honors every guardrail** — opt-in/default-off, bounded caps, P1/P2/P3, the Candidate-F Euclidean
   boundary (§3 M9), no atlas/active-mask/perception.
7. Is **documented** complete-vs-deferred.

**Slice-scoped starmap-generation closure — not a Stellaris importer, not playable gameplay, not
editor/corpus/export, not deep galaxy-scale hierarchical allocation, not atlas/perception** (§10).

## 3. Schema adjudications (executive design authority — spent here so PRs are mechanical)

> Extend, never weaken, closeout A1–A5, core §7, ADR-MAP, ADR-RF. Where MapGen and a governing source
> disagree, **the source governs and the rung stops.**

**M1 — Neutral AST (no semantic decisions in parse).** Raw text → `RawDocument` via jomini; preserve
repeated keys, order, nesting; zero mapping decisions; mapping is a separate pass. No
load-order/override/localization/trigger interpretation (§10).

**M2 — Gridcell-lattice spatial hierarchy (core §2/§7; ADR-RF fanout).** Generate
`galaxy(2D map) → sector/cluster → system(gridcell SimThing) → planet → deposit(children)`; Stellaris
clusters/partitions → the sector level; ≤ ~100 children/level. **Systems are gridcell SimThings
occupying cells of the galaxy lattice** — a gridcell **is a `Location` SimThing** (intrinsic spatial
identity, never a new `SimThingKind` and not a detachable role; core §7). Star systems occupy a **subset** of cells; the lattice is larger than the
star count (empty = deep space, ambient field). Planets/deposits are `children` + CT-2c intrinsic flows.

**M3 — Resource-flow arena generation (ADR-RF).** Deposits → `IntrinsicFlow`; system/sector →
hierarchical allocators; economy arenas **and** a **suppression arena** whose pressure columns are the
Movement-Front cell-state columns. **Every arena MUST declare** explicit selectors (no implicit/property
admission), hard caps (`max_participants`/`max_coupling_fanout`/`max_orderband_depth`), a `FissionPolicy`
(default `Reevaluate`), and per-edge coupling delay forms (no all-`Algebraic` cycle); it must **pass the
spec firewall and emit a clean expansion report.** `AccumulatorRole` stays compile-time metadata. **v1
targets the closed CT-2c surface with a shallow hierarchy;** deep multi-level galaxy-scale allocation +
large coupling are the architectural target but **deferred** (§10) until a slice demonstrates need.

**M4 — Movement-Front heatmap IS the cellular automaton over the lattice (core §7; ADR-MAP).** The
suppression-arena pressure columns are the **cell-state columns** the **Gu-Yang `SaturatingFlux` stencil
evolves across the 2D gridcell lattice** (L1) — values spill with falloff; the moving contour **is the
front.** **Per-tick horizon is bounded** (P1: H ≤ 8; fronts cross the galaxy over many ticks;
cadence/dirty make compute follow the wavefront). **Strategic awareness = L2 hierarchy reduction**
(cell → sector → faction columns), **never a wider stencil** (the only rejected pattern is
horizon-widening-as-strategic-shortcut). **L3** parent EML → **threshold crossing = the P3 attractor
projection** → commitment; **no CPU planner.** One shared kernel + authored weights at every cell (P2);
stability-bounded operators + ping-pong (P3). Domain-neutral identifiers only — **the engine name is
"Movement-Front"; never write "SEAD" in code/spec/tests** (M4 / PR #539 discipline). Default-off.

**M5 — Scale & gridcell lattice (supersedes BOTH 2026-06-13 sketches: the naive "big dense grid O(N)/tick"
AND the over-corrected "no galaxy grid / bounded theaters only").** The starmap **IS a galaxy gridcell
lattice run as the Movement-Front automaton** (core §7; **base canonical dimensions are always square** —
default "medium" 200×200, scaling up square with star density). **One system = one
gridcell** (the user's binding requirement); **no sub-cell x/y ever** — `position` quantizes to a free
cell and survives only as **inert render metadata** (M9). Density never subdivides a cell: stars occupy a
**subset** of the lattice; the lattice has room and empty cells carry ambient field. Scaling to 2000+
stars comes from **P1 (local per-tick update, compute follows the wavefront) + L2 hierarchy +
cadence/dirty**, not from a dense one-tick global pass. If a star count ever exceeds the lattice capacity,
the **lattice dimensions grow** (still one system per cell). The galaxy L1 stencil is real and runs
across the lattice; what is bounded is the **per-tick horizon**, not the grid extent.

**M6 — Hyperlane → bounded link + long-range lane coupling (inherits closeout A2 — highest leakage
risk).** A hyperlane → `link { from to }` → bounded topology metadata (validated endpoints, fan-out ≤ a
declared cap). The Movement-Front front propagates over the **geometric gridcell lattice** (N4, P1); a
hyperlane is a **bounded long-range coupling edge layered on top** (a sparse gather to lane-neighbors /
an ADR-RF coupling edge) so the front can also follow lanes — **how strongly the front follows lanes vs
geometry is an authored coupling weight, not a new engine.** A link is **never a graph object** in
`simthing-sim`. Arbitrary high-degree/non-representable topology → **STOP** (§9). `random_hyperlanes` has
no production.

**M7 — Reach → PALMA W/D feedstock (inherits closeout A3; core §7 "the front is the route").**
Influence/reach → composed-W → `min_plus_traversal_field` D over impedance. D is a **field**, never a
route; no `route`/`path`/`predecessor`/`waypoint` production. PALMA is the seated reach utility; **MapGen
is its named consumer.**

**M8 — Gu-Yang ∥ PALMA parallelization (efficiency + front richness; composition-first).**
Parallelization is **scheduling/composition over existing ops**: (a) independent regime-distinct
suppression fields → concurrent dispatches in one encoder (the **front-richness** lever — more
independent fronts, P2-uniform, cadence-skipped when quiet); (b) cross-tick software pipelining of
stencil shaping with PALMA relaxation; (c) shared resident tiling. **A fused single kernel is a new
primitive: NOT in the default ladder**, opt-in only behind this named consumer + DA review + a measured
win + preservation of BH-0 (symmetric flux, zero-flux, CFL χ ≤ 0.25) and min-plus (`D = W + min(N4 D)`,
no sqrt) invariants. Concurrency must respect P1 — no rung uses it to widen a horizon into a global pass.

**M9 — Candidate-F Euclidean tripwire (GUARDED, not crossed; §0.7).** Position is inert metadata;
adjacency is topological (lattice neighbors + lane coupling); reach is min-plus/impedance, never
Euclidean. Front propagation is the stencil over the lattice — no Euclidean point-radius query. Any true
Euclidean-distance/spatial-magnitude consumer → **STOP** for §0.7 review (out of MapGen scope). PR 9 guards it.

**M10 — Corpus referenced, not vendored.** Vanilla files at `C:\Users\mvorm\Clauser\Paradox\` are
read-only; rungs hand-author tiny fixtures under `tests/fixtures/mapgen/`; no Paradox files committed. No
corpus-wide decode claim; any future one passes the `modifiers.log` round-trip bar.

**M11 — Deferred boundary.** Deep galaxy-scale hierarchical allocation + large coupling; atlas batching /
active masks / perception / behavioral source policy (ADR-MAP gates); whole-corpus coverage;
load-order/override; trigger/effect interpretation; `spawn_weight`/`neighbor_system` procedural placement;
localization; `prescripted_countries`; graphical galaxy; arbitrary-graph topology; pathfinding/movement;
editor/corpus/export (`FIELD-MOVIE-DATASET-0`, the next track).

## 4. PR ladder table

| PR | Title | Owner | Governing source | Depends on |
|---|---|---|---|---|
| 1 | Index + corpus manifest + slice selection + read-order | Cursor | — | — |
| 2 | Neutral-AST adapter spike (parse-only) | Cursor (DA review) | M1 | 1 |
| 3 | Gridcell-lattice spatial hierarchy generation | Cursor (DA review) | core §7, ADR-RF · M2 | 2 |
| 4 | Resource-flow arena generation (caps + selectors + expansion report) | Cursor (DA review) | ADR-RF · M3 | 3 |
| 5 | Hyperlane → bounded link + lane coupling + inert position | Cursor (DA review) | A2 · M6 · M9 | 3 |
| 6 | Movement-Front heatmap: L1 lattice stencil + L2 hierarchy + L3 EML → commitment | Cursor (DA review) | core §7, ADR-MAP · M4 | 4,5 |
| 7 | PALMA W/D reach feedstock (min-plus over impedance) | Cursor | A3 · M7 | 5,6 |
| 8 | Gu-Yang ∥ PALMA parallelization spike (bounded; fused kernel DA-gated) | Cursor (DA review) | core §7, ADR-MAP · M8 | 6,7 |
| 9 | Candidate-F Euclidean + P1/horizon + one-system-per-cell guard | Cursor (DA review) | §0.7 · M4 · M5 · M9 | 5,6 |
| 10 | Canonical sample: ingest → generate lattice+arenas+front → admit/install → GPU exercise | Cursor (DA review) | all | 4,6,7,9 |
| 11 | Closeout report + docs + ledger | Cursor (DA sign-off) | M11 | 8,10 |

"DA review" = executive design authority reviews the merge diff against §0/§3. A rung stops/escalates only on a §9 stop condition.

## 5. Rare Opus / design-authority gates

Merge-time review of the paradigm/ADR-sensitive rungs — **2** (no-semantics parse), **3** (gridcell
lattice + no-new-kind), **4** (arena firewall + caps + expansion report), **5** (link/lane-coupling
boundary), **6** (Movement-Front 3-layer + P1 horizon), **8** (parallelization / fused-kernel proposal),
**9** (Euclidean + P1/scale guards) — and **sign-off** on the closeout (11). §0 + §3 pre-spent the design gates.

## 6. Cursor-granular PR handoffs

> Each rung: re-read the cited core/ADR section **before** writing code. Acceptance includes "no
> paradigm/ADR guardrail crossed." Stop conditions cite §9. **Never introduce "SEAD" into code/spec/tests.**

### PR 1 — Index + corpus manifest + slice selection + read-order
Owner: Cursor. Docs only. Files: this doc (§6.1 manifest); `docs/clausething/mapgen_corpus_manifest.md`;
`tests/fixtures/mapgen/README`; optional `docs/tests/mapgen_pr1_corpus_manifest_results.md`.
Steps: pin the vanilla files the slice draws from (read-only); name the slice — **one
`solar_system_initializer` + a ≤ 5-system `static_galaxy_scenario` with explicit `add_hyperlane` and ≥ 1
deposit** (use `vanilla/map/setup_scenarios/static_galaxy_example.txt` + one
`vanilla/common/solar_system_initializers/` entry); restate M10 + §0 read-order. Acceptance: pinned. Stop: §9.

**Status: PASS (2026-06-13, Cursor PR 1).** Pinned read-only corpus manifest
([`clausething/mapgen_corpus_manifest.md`](clausething/mapgen_corpus_manifest.md)); selected
**`tiny_pentad_hub_slice`** (5 systems, hub pentad + explicit links, one deposit child, optional nebula
metadata). Added inert hand-authored fixture stub
`crates/simthing-clausething/tests/fixtures/mapgen/tiny_static_starmap_slice.clause` (**not parsed** until
PR2). Performed artifact lifecycle audit; no scratch DELETE items; closeout guardrails unchanged. Result:
[`tests/mapgen_pr1_corpus_manifest_results.md`](tests/mapgen_pr1_corpus_manifest_results.md) (PROBATION).
No parser/importer/runtime/GPU/editor code.

### PR 1 artifact lifecycle audit (§6.1)

Re-ingested closeout + BH/PALMA artifact posture before MapGen work. No closeout PROBATION artifacts
remain. MapGen PR1 adds only docs/fixtures.

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/clausething_closeout_results.md` | CURRENT_EVIDENCE | Unchanged; 0.0.8.2 closed |
| `docs/tests/bh3_closeout_pr7..pr9_*` | CURRENT_EVIDENCE | Folded into closeout report |
| `docs/archive/superseded_tests/bh3_closeout_pr2..pr6_*` | ARCHIVE | Unchanged |
| `docs/tests/fable_review_*`, `bh2d_ct4b_100tick_*`, `r1_default_workspace_purge_*` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_sweep_results.md` | CURRENT_EVIDENCE | Candidate F chain |
| `docs/tests/bh0_*` … `bh2d_*`, `palma_path_*` | CURRENT_EVIDENCE | 0.0.8.1 track seating |
| `ct_scenario_container`, `ct_bh3_closeout_sample_driver` | LIVE_GUARDRAIL | Unchanged |
| `docs/clausething/mapgen_corpus_manifest.md` | CURRENT_EVIDENCE | New PR1 manifest pin |
| `docs/tests/mapgen_pr1_corpus_manifest_results.md` | PROBATION | New PR1 report |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

**Corpus manifest pin:** [`clausething/mapgen_corpus_manifest.md`](clausething/mapgen_corpus_manifest.md).

**Slice pin:** `tiny_pentad_hub_slice` — see manifest § "PR1 tiny slice pin".

### PR 2 — Neutral-AST adapter spike (parse-only)
Owner: Cursor (DA review). Re-read ADR-MAP semantic-free posture. Parse fixtures → `RawDocument`; assert
repeated keys/order/nesting; **zero semantic decisions; no spec/sim change.** Tests: `mapgen_neutral_ast_parse`.
Stop: jomini can't represent a construct → escalate (§9).

**Status: PASS / DA-APPROVED (2026-06-13, Cursor PR 2; Opus / Design Authority — battery reran green under DA review: `mapgen_neutral_ast_parse` 8 passed, `ct_scenario_container` 45 passed; `fmt`/`git diff --check` clean; merge `edeab38a`).** Added parse-only neutral-AST adapter
(`mapgen_neutral_ast.rs`: `parse_mapgen_neutral_document` → `RawDocument` via jomini). Hand-authored raw
fixture `tiny_pentad_hub_slice_raw.clause` (Stellaris-style idioms; not lowered). Focused tests assert
repeated keys, nesting, and sibling order/count; no semantic mapping; no SimThing structures. PR2 is
parse-only — PR3 is the first hierarchy-generation rung. No Paradox files committed. No
parser/importer/runtime/GPU/driver/simthing-sim change. Result:
[`tests/mapgen_pr2_neutral_ast_results.md`](tests/mapgen_pr2_neutral_ast_results.md) (PROBATION).

### PR 2 artifact lifecycle audit (§6.2)

Re-ingested PR1 + closeout posture before PR2 implementation. No scratch DELETE items; closeout
guardrails unchanged.

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/clausething_closeout_results.md` | CURRENT_EVIDENCE | Unchanged; 0.0.8.2 closed |
| `docs/clausething/mapgen_corpus_manifest.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_pr1_corpus_manifest_results.md` | PROBATION | Unchanged |
| `tiny_static_starmap_slice.clause` | PROBATION | Inert PR1 stub; still not MapGen-parsed |
| `tiny_pentad_hub_slice_raw.clause` | CURRENT_EVIDENCE | New PR2 raw parse fixture |
| `mapgen_neutral_ast.rs`, `mapgen_neutral_ast_parse.rs` | CURRENT_EVIDENCE | New PR2 adapter + tests |
| `ct_scenario_container`, `ct_bh3_closeout_sample_driver` | LIVE_GUARDRAIL | Unchanged |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

### PR 3 — Gridcell-lattice spatial hierarchy generation
Owner: Cursor (DA review). **Re-read core §7 + §2 + ADR-RF fanout.** Generate
`galaxy(2D map)→sector→system(gridcell)→planet→deposit`; systems = **gridcell SimThings (mapping-role,
no new kind)** occupying lattice cells; ≤ ~100 children/level; deposits carry CT-2c intrinsic flows.
Tests: `mapgen_gridcell_lattice_hierarchy`, `mapgen_no_new_simthingkind`, `mapgen_gridcell_is_mapping_role`,
default-off/semantic-free. Stop: a node needs a new sim type / RegionCell-as-entity → escalate (§9).

**Status: PASS / DA-APPROVED (2026-06-13, PR #658 merge `67d6ab8c`; post-merge audit
[`mapgen_pr3_da_audit_results.md`](tests/mapgen_pr3_da_audit_results.md)).** Added
`generate_mapgen_lattice_hierarchy` (`mapgen_lattice.rs`) lowering the tiny neutral-AST fixture into
scenario-container-compatible hierarchy via existing `hydrate_scenario`. Galaxy → sector → gridcell systems
as ordinary `SimThingKind::Location` nodes with `mapgen` mapping-role metadata; fixture-local 3×3 lattice
placements; canonical 200×200 documented in metadata only; inert render positions; initializer planet/deposit
as child payload metadata (not RF). No links, RF, Movement-Front, PALMA, or FIELD_POLICY output. Tests:
`mapgen_lattice_hierarchy` (10 passed). Result:
[`tests/mapgen_pr3_lattice_hierarchy_results.md`](tests/mapgen_pr3_lattice_hierarchy_results.md) (CURRENT_EVIDENCE);
DA audit: [`tests/mapgen_pr3_da_audit_results.md`](tests/mapgen_pr3_da_audit_results.md) — **genuine DA
sign-off (Opus, 2026-06-14)** after an independent post-merge audit that ratified and corrected the
Cursor-prefiled approval; the code stands, one non-blocking advisory recorded. **PR4 may proceed** (subject
to its own DA-review gate). **Governance: only the Design Authority writes a DA sign-off — an implementing
agent must not author its own "DA APPROVED" line.**

### PR 3 artifact lifecycle audit (§6.3)

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr2_neutral_ast_results.md` | PROBATION | Unchanged |
| `mapgen_neutral_ast_parse.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_lattice.rs` | CURRENT_EVIDENCE | PR3 generator source |
| `mapgen_lattice_hierarchy.rs` | LIVE_GUARDRAIL | Promoted at DA audit |
| `docs/tests/mapgen_pr3_lattice_hierarchy_results.md` | CURRENT_EVIDENCE | Promoted at DA audit |
| `docs/tests/mapgen_pr3_da_audit_results.md` | CURRENT_EVIDENCE | Post-merge DA audit |
| `tiny_pentad_hub_slice_raw.clause` | PROBATION / active fixture | Unchanged |
| `ct_scenario_container`, `ct_bh3_closeout_sample_driver` | LIVE_GUARDRAIL | Unchanged |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

### PR 4 — Resource-flow arena generation
Owner: Cursor (DA review). **Re-read ADR-RF §"Four commitments" + §"Draconian guardrail" + §"Invariants."**
Deposits → `IntrinsicFlow`; shallow allocator hierarchy; declare **selectors + caps + FissionPolicy +
coupling delay forms**; clean **expansion report**; one **suppression arena** as the Movement-Front source.
Tests: `mapgen_arena_enrolls_with_caps`, `mapgen_arena_rejects_uncapped`, `mapgen_arena_rejects_algebraic_cycle`,
`mapgen_expansion_report`. Stop: deep multi-level allocation / large coupling beyond CT-2c → escalate (§9 / §10).

**Status: PASS / DA-APPROVED after a targeted DA repair (Cursor PR 4, 2026-06-13; Opus / Design Authority
sign-off 2026-06-14).** DA finding: the deposit arena's `InstallTarget(ScenarioListed{deposits[0]})`
enrollment singled out the first deposit (latent multi-deposit bug, masked by the single-deposit fixture)
— DA-repaired to `ExplicitOnly` over the authoritative participant list; battery reran green
(`mapgen_resource_flow` 16, `mapgen_lattice_hierarchy` 10, `mapgen_neutral_ast_parse` 8,
`ct_scenario_container` 45). Added
`generate_mapgen_resource_flow_enrollment` (`mapgen_resource_flow.rs`) lowering PR3 hierarchy into bounded
`ResourceFlowSpec` enrollment: deposit minerals intrinsic-flow feedstock arena (1 explicit participant) +
suppression/disruption arena (5 gridcell participants), all caps declared, shallow deposit→suppression coupling,
expansion report, `opt_in_mode = Disabled`. No Movement-Front, SaturatingFlux, PALMA, FIELD_POLICY, hyperlane,
or runtime/GPU/driver/simthing-sim changes. Tests: `mapgen_resource_flow` (16 passed). Result:
[`tests/mapgen_pr4_resource_flow_results.md`](tests/mapgen_pr4_resource_flow_results.md) (PROBATION).

### PR 4 artifact lifecycle audit (§6.4)

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_resource_flow.rs` | CURRENT_EVIDENCE | New PR4 generator (DA repair applied) |
| `mapgen_resource_flow.rs` (tests) | LIVE_GUARDRAIL | Promoted at DA approval |
| `docs/tests/mapgen_pr4_resource_flow_results.md` | CURRENT_EVIDENCE | New PR4 report; DA-approved |
| Prior PR1–PR3 guardrails | unchanged | See PR3 audit |

### PR 5 — Hyperlane → bounded link + lane coupling + position metadata
Owner: Cursor (DA review). Re-read A2 + M6 + M9. Static `system` → gridcell (position **inert metadata**,
quantized to a free cell); `add_hyperlane` → `link` (endpoint validation, fan-out cap); lane coupling as a
**bounded long-range gather** layered on the lattice. Tests: `mapgen_links`. Stop:
non-representable topology → escalate (§9).

**Status: PASS / DA-APPROVED (Cursor PR 5, 2026-06-13; Opus / Design Authority sign-off
2026-06-14).** Added
`generate_mapgen_links` / `lower_hyperlane_topology` (`mapgen_links.rs`) lowering PR4 enrollment plus
neutral-AST `add_hyperlane` declarations into bounded `HydratedScenarioGridMetadata.links` (N4-adjacent
lattice edges only) and bounded `mapgen::lane_coupling` inert authoring properties (long-range edges).
Validated endpoints; rejected self-links and unknown endpoints; deterministic duplicate canonicalization;
link/lane-coupling/per-node fanout caps; expansion report; inert render positions preserved; no Euclidean
adjacency authority; no route/path/predecessor/movement/border/frontline semantics; no
Movement-Front/SaturatingFlux/PALMA/FIELD_POLICY/runtime/GPU/driver/simthing-sim changes; no new
`SimThingKind`. DA confirmed `mapgen::lane_coupling` is inert authoring metadata only (field-propagation
consumption deferred to later rungs). Tests: `mapgen_links` (19 passed). Result:
[`tests/mapgen_pr5_links_results.md`](tests/mapgen_pr5_links_results.md) (CURRENT_EVIDENCE).

### PR 5 artifact lifecycle audit (§6.5)

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_links.rs` | CURRENT_EVIDENCE | New PR5 generator (DA-approved) |
| `mapgen_links.rs` (tests) | LIVE_GUARDRAIL | Promoted at DA approval |
| `docs/tests/mapgen_pr5_links_results.md` | CURRENT_EVIDENCE | New PR5 report; DA-approved |
| Prior PR1–PR4 guardrails | unchanged | See PR4 audit |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

### PR 6 — Movement-Front heatmap (L1 lattice stencil + L2 hierarchy + L3 EML)
Owner: Cursor (DA review). **Re-read core §7 + ADR-MAP three-layer + P1 — the rung most prone to the drift
this ladder exists to prevent.** Suppression-arena pressure = cell columns; **L1 Gu-Yang stencil across the
lattice, bounded per-tick horizon (H ≤ 8, `source_capped_normalized`, ping-pong), cadence/dirty;** L2
`SlotRange` Sum → sector/faction columns; L3 `ai_will_do` EML → threshold → commitment; default-off.
**No horizon-widening-as-strategic-shortcut.** Tests: `mapgen_movement_front`. Stop: a field wants a wider
horizon for strategic awareness → escalate (§9).

**Status: PASS — DA-APPROVED (Opus, 2026-06-13).** Added
`generate_mapgen_movement_front_authoring` (`mapgen_movement_front.rs`) lowering PR5 enrollment into
existing Movement-Front authoring surfaces: L1 `RegionFieldSpec`/`SaturatingFlux` with bounded horizon and
`ArenaPressureBindingSpec` from PR4 suppression RF; L2 `RegionFieldReductionSpec` hierarchy feedstock; L3
`FirstSliceCommitmentSpec`/`HydratedScenarioCommitment` threshold feedstock. No PALMA, no driver/GPU/runtime
execution, no pathfinding/movement/route/predecessor/border/frontline semantics, no Euclidean authority, no
new `SimThingKind`. Tests: `mapgen_movement_front` (23 passed). Result:
[`tests/mapgen_pr6_movement_front_results.md`](tests/mapgen_pr6_movement_front_results.md) (CURRENT_EVIDENCE).

### PR 6 artifact lifecycle audit (§6.6)

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_movement_front.rs` | CURRENT_EVIDENCE | New PR6 generator (DA-approved) |
| `mapgen_movement_front.rs` (tests) | LIVE_GUARDRAIL | Promoted at DA approval |
| `docs/tests/mapgen_pr6_movement_front_results.md` | CURRENT_EVIDENCE | New PR6 report; DA-approved |
| Prior PR1–PR5 guardrails | unchanged | See PR5 audit |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

### PR 7 — PALMA W/D reach feedstock
Owner: Cursor. Re-read A3 + M7 + core §7. `palma_feedstock { w_source d_output_col }` composing W from
suppression/choke columns → `min_plus_traversal_field`. Tests: `mapgen_palma`,
route/movement-vocabulary rejection. Stop: route/predecessor need → escalate (§9).

**Status: PASS (Cursor PR 7, 2026-06-13).** Added `generate_mapgen_palma_feedstock` (`mapgen_palma.rs`)
lowering PR6 Movement-Front enrollment into existing `HydratedScenarioPalmaFeedstock` plus generic
`WImpedanceComposeSpec` bound to PR6 SaturatingFlux choke/suppression columns (W col 3, D col 4 on the
tiny slice). Default-off mapping profile preserved. No routes/paths/predecessors/movement orders,
no driver/GPU execution, no Euclidean authority, no runtime/GPU/driver/simthing-sim changes. Stays inside
pre-adjudicated M7 boundary — no DA escalation required. Tests: `mapgen_palma` (19 passed). Result:
[`tests/mapgen_pr7_palma_results.md`](tests/mapgen_pr7_palma_results.md) (PROBATION).

### PR 7 artifact lifecycle audit (§6.7)

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_palma.rs` | PROBATION | New PR7 generator |
| `mapgen_palma.rs` (tests) | PROBATION | New PR7 guardrail battery |
| `docs/tests/mapgen_pr7_palma_results.md` | PROBATION | New PR7 report |
| Prior PR1–PR6 guardrails | unchanged | See PR6 audit |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

### PR 8 — Gu-Yang ∥ PALMA parallelization spike
Owner: Cursor (DA review). Re-read M8 + P1. Scheduled-concurrency (independent suppression fields + W
compose + PALMA in one encoder, double-buffered, zero readback); **measure vs serial BH-2C baseline**;
fused kernel only as a DA-gated escalation carrying the M8 gate. Tests:
`mapgen_gu_yang_palma_scheduled_concurrency` (GPU-gated, compact timing). Stop: needs a new primitive
un-gated, violates BH-0/min-plus invariants, or widens a horizon into a global pass → escalate (§9).

### PR 9 — Candidate-F + P1/scale guards
Owner: Cursor (DA review). Re-read M5 + M9 + §0.7. Prove position inert; no Euclidean
magnitude/sqrt/`length`/`normalize`/`distance(type=euclidean)`; reach is min-plus; **one system per cell;
per-tick horizon bounded (no horizon-widening-as-strategic-shortcut).** Tests:
`mapgen_no_euclidean_magnitude_guard`, `mapgen_position_inert_guard`, `mapgen_one_system_per_cell_guard`,
`mapgen_bounded_horizon_guard`. Stop: genuine Euclidean need → §0.7 escalation.

### PR 10 — Canonical sample (end to end)
Owner: Cursor (DA review). A ≤ 5-system slice → neutral AST → generate gridcell lattice + arenas (capped,
expansion report) + Movement-Front heatmap + PALMA D + commitment → `open_from_spec` → run a few ticks →
assert resource-flow reduction + L1/L2/L3 commitment via **compact probe/threshold only.** Tests:
`mapgen_canonical_sample_installs_and_runs` (GPU-gated; CPU path otherwise). Stop: install needs a new
sim-aware surface / full-field readback / uncapped arena → escalate (§9).

### PR 11 — Closeout report + docs + ledger
Owner: Cursor (DA sign-off). Docs only. Files: `docs/tests/mapgen_0_0_8_2_5_closeout_results.md` (new,
CURRENT_EVIDENCE); this ladder's ledger; pointers in `design_0_0_8_1_border_hack_track.md`,
`design_0_0_8_1_palma_pathfinding_integration_guide.md`, `clausething/MapGenThing.md`,
`design_0_0_8_2_clausething_closeout_ladder.md` §12. Write complete-vs-deferred (§2/§11); confirm core §7
+ both ADRs honored and Candidate F unmoved; classify artifacts. **No close until DA sign-off.** Tests:
`cargo fmt --all -- --check`; `cargo test -p simthing-clausething`; `-p simthing-driver`; `git diff --check`. Stop: §2 unmet (→ PARTIAL).

## 7. Test strategy

Focused, fast, GPU-skipping-clean. Cover once each: neutral-AST fidelity; gridcell-lattice hierarchy;
no-new-`SimThingKind`; gridcell-is-mapping-role; arena caps + selectors + **uncapped/cycle rejection** +
expansion report; hyperlane→bounded-link; one-system-per-cell; position-inert; **no-horizon-widening**;
L1-lattice-stencil / L2-hierarchy / L3-commitment; default-off; PALMA-D-not-route; scheduled-concurrency
vs serial oracle (compact timing); Euclidean/sqrt guard; end-to-end compact exercise. **Forbidden:**
report-checksum gates, replay theater, prior-rung parity ledgers, > 60s default tests. Close-out commands:
`cargo test -p simthing-clausething --test mapgen_scenario` + the canonical driver test.

## 8. Principle compliance (binding, restated for MapGen)

- **Movement-Front P1/P2/P3 (core §7):** local per-tick update + finite-speed fronts + cadence (P1, no
  horizon-widening-as-strategic-shortcut); one shared kernel + authored weights (P2); stability-bounded +
  threshold attractor projection (P3).
- **ADR-RF four rules:** capability universal; participation explicit (selectors); expansion bounded
  (caps + expansion report); unsafe content rejected at build. `simthing-sim` arena-ignorant.
- **ADR-MAP:** a RegionCell/gridcell **is a `Location` SimThing** (intrinsic spatial identity; the field columns are its backing, not a detachable role), **not a new `SimThingKind`**; opt-in/default-off.
- **Everything is a SimThing.** No per-system map objects, no noun engines, no CPU planner. Decisions =
  threshold crossings over L3 columns. **No movement/pathfinding/route/predecessor/border** objects (M6/M7).
- **No new GPU primitive by default** (M8); fused kernels DA-gated + invariant-preserving. **Candidate F
  unmoved; Euclidean guarded** (M9). **Default-off.** **Never write "SEAD" in code/spec/tests** (M4).

## 9. Stop conditions (escalate → PARTIAL; do not improvise)

Halt and escalate on: **horizon-widening as a strategic-awareness shortcut** (P1 / ADR-MAP rejected
pattern); a **per-cell bespoke rule or coordinate-dependent logic** (P2 breach); **unbounded recurrence /
non-stability-bounded operator** (P3 breach); **a gridcell treated as a `SimThingKind`/entity, or
sub-cell x/y, or >1 system per cell** (M5/core §7 breach); **an arena without declared caps / selectors /
fission policy, or an all-`Algebraic` coupling cycle** (ADR-RF firewall); arbitrary-graph topology (M6);
route/predecessor production (M7); a Euclidean-distance/magnitude consumer (M9 / §0.7); a new
`SimThingKind`/sim-aware surface to install; full-field CPU readback to decide; a fused GPU kernel without
the M8 gate; deep galaxy-scale allocation beyond CT-2c (M3/§10); a vendored-Paradox-file fixture (M10);
atlas/active-mask/perception without its ADR-MAP gate.

## 10. Deferred boundary (the subsequent tracks)

Deferred, not closed here: deep galaxy-scale hierarchical allocation + large coupling; atlas batching /
active masks / perception / behavioral source policy; whole-corpus coverage; load-order/override;
trigger/effect interpretation; weighted procedural placement; localization; `prescripted_countries`;
graphical galaxy; arbitrary-graph topology; pathfinding/movement; and the **editor/corpus/export seam**
(`FIELD-MOVIE-DATASET-0`), the next minor track on the 0.0.8.2 §10 boundary.

## 11. References

- **Paradigm (read first):** [`simthing_core_design.md`](simthing_core_design.md) §1.1 (Anchor A — Wei, arXiv:2602.01651) + §7 (Movement-Front automaton; Gu-Yang `SaturatingFlux` + PALMA operators); `docs/invariants.md`.
- **Governing ADRs:** [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md), [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md).
- Destination / production contract: [`clausething/ct_vertical_consumer_contract.md`](clausething/ct_vertical_consumer_contract.md); CT-2c economy [`clausething/ct_2c_economic_category_memo.md`](clausething/ct_2c_economic_category_memo.md); heatmap/terminology [`clausething/ct_3b_4a_movement_front_heatmap_memo.md`](clausething/ct_3b_4a_movement_front_heatmap_memo.md).
- Stellaris-side detail: [`clausething/MapGenThing.md`](clausething/MapGenThing.md).
- Corpus manifest (PR1): [`clausething/mapgen_corpus_manifest.md`](clausething/mapgen_corpus_manifest.md).
- Closeout this extends (sign-off stands): [`tests/clausething_closeout_results.md`](tests/clausething_closeout_results.md), [`design_0_0_8_2_clausething_closeout_ladder.md`](design_0_0_8_2_clausething_closeout_ladder.md).
- Gu-Yang `SaturatingFlux`: [`design_0_0_8_1_border_hack_track.md`](design_0_0_8_1_border_hack_track.md) (arXiv:2509.20797). PALMA: [`design_0_0_8_1_palma_pathfinding_integration_guide.md`](design_0_0_8_1_palma_pathfinding_integration_guide.md) (arXiv:2601.17028). Candidate F: [`design_0_0_8_1.md`](design_0_0_8_1.md) §0.7.
- Stellaris corpus (read-only, not vendored): `C:\Users\mvorm\Clauser\Paradox\`.
