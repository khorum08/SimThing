# SimThing 0.0.8.2.5 — MapGen PR Ladder (Stellaris Starmap → SimThing Star Mapping)

> **Status: DESIGN / READY FOR CURSOR EXECUTION (revised 2026-06-13, executive design authority).**
> This is the planning artifact that **pulls the deferred corpus-import / map-generation consumer**
> named in the 0.0.8.2 closeout. It is not an implementation PR. It pins schema judgments (§3) so the
> rungs are Cursor-mechanical, and it is **subordinate to two governing ADRs (§0).**
>
> **What MapGen is.** MapGen *ingests* ClauseScript / Stellaris mapping script and *generates*
> **SimThing star mapping** — a spatial hierarchical tree + resource-flow arena enrollment + a bounded
> three-layer suppression/disruption ("SEAD") heatmap — shaped for exactly the substrates SimThing
> already runs: Resource Flow and the RegionCell mapping field model. It is a **front-end / generator**,
> not a new runtime, not a Stellaris importer, not a whole-game decoder.
>
> **0.0.8.2.5 is an extension, not a reopening of sign-off.** The 0.0.8.2 DA sign-off
> ([`tests/clausething_closeout_results.md`](tests/clausething_closeout_results.md)) **stands.** MapGen
> names the consumer the closeout deferred (PALMA "awaiting a named consumer"; BH-3 authoring). The
> editor/corpus/export seam (`FIELD-MOVIE-DATASET-0`) remains the **subsequent** track.

---

## 0. Binding substrate ADRs — READ FIRST (non-negotiable)

> Every rung is subordinate to these two ADRs. **If a rung's behavior is not expressible within both,
> the rung STOPS and escalates (§9) — it does not improvise a wider substrate.** The executive design
> authority drifted from the mapping ADR while sketching this track on 2026-06-13; that is precisely
> the drift Cursor will reproduce if these are not load-bearing in every rung. They are cited inline in
> §3 and §6 by section. **Do not write a rung without re-reading the cited ADR section.**

**ADR-MAP — [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md) (Approved 2026-05-28).**
The map, constitutionally:
- **The spatial tree is the physical map.** Political/faction structure is overlays on the tree, not nodes in it.
- **A RegionCell is NOT a `SimThingKind`.** It is an *authored mapping-role / profile on a SimThing*, backed by a slot range in dense matrices, addressed positionally by `(width, height, col)`. `simthing-core` gains no variant; `simthing-sim` stays semantic-free (flat columns + opaque `AccumulatorOp` only).
- **Three-layer model (load-bearing):** **L1** `StructuredFieldStencilOp` over a *bounded local theater* (first slice ≤ 32×32, H ≤ 8, `source_capped_normalized`, ping-pong) for dense tactical fields; **L2** `SlotRange` Sum reduction cell→parent for strategic awareness (~15× cheaper than widening the stencil); **L3** parent `EvalEML` interpretation → threshold → commitment. Reduce-before-interpret; propagation by later-band cascade.
- **The rejected pattern (the load-bearing negative result):** **dense, lateral, long-horizon diffusion over a large/global grid is over budget and is REJECTED as a strategic-awareness mechanism** (~3236 ms/tick at 30k cells). Strategic awareness is hierarchy + parent EML, **never dense-and-global.**
- Opt-in / bounded / default-off (`MappingExecutionProfile::Disabled`). Atlas batching, active masks, perception fields, behavioral source policy are **Provisional/Deferred** with hard gates — `request_atlas_batching` stays rejected at admission.

**ADR-RF — [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md) (Accepted 2026-05-26).**
Continuous resource dynamics:
- **Hierarchical fanout absorption is the scaling mechanism.** `faction(1) → planet(100) → district(1000) → factory(100000)`; each level ≤ ~100 children, so per-level contention is bounded **regardless of total count.** This is *the* reason 2000+ stars are cheap — it is the same hierarchy as ADR-MAP L2.
- **One unified field substrate, many named arenas** distinguished by column range, not kernel variant (food, research, **`piracy_suppression`**, …). The suppression/disruption ("SEAD") front is a **resource-flow arena**, not a bespoke field.
- **Four constitutional rules:** capability universal; **participation explicit** (selector admission — property possession never admits); **expansion bounded** (every arena declares `max_participants`, `max_coupling_fanout`, `max_orderband_depth`); **unsafe content rejected at import / session build**, not clamped at runtime. The spec compiler emits an **expansion report** per build.
- Per-coupling delay form (`Algebraic` / `OneTickDelay` / `BoundaryStage` / `AccumulatorState`); a cycle of all-`Algebraic` edges is rejected. `FissionPolicy ∈ {Inherit, Reevaluate(default), Reject}`. `AccumulatorRole` is compile-time metadata only — compiles away before `simthing-sim`. `ArenaRegistry` lives in `simthing-driver`; `simthing-sim` never sees it.
- **No new GPU primitive** — the substrate is a registration discipline over AccumulatorOp v2. ("All conflict is resource flow" — `design_0_0_8_0.md` §0.3.)

**Read order for any MapGen rung:** `docs/invariants.md` → ADR-MAP → ADR-RF → this ladder §3 → the closed authoring surfaces (`clausething/ct_vertical_consumer_contract.md`, CT-2c economy) → [`clausething/MapGenThing.md`](clausething/MapGenThing.md) for Stellaris-side detail.

---

## 1. What MapGen ingests and what it generates

**Ingest:** raw Stellaris/Clausewitz mapping script (solar_system_initializers, setup_scenarios, static_galaxy_scenario, add_hyperlane, deposits, nebulas) and/or ClauseScript scenario authoring, via the existing jomini neutral-AST path.

**Generate (SimThing star mapping — three coordinated outputs, all on already-closed surfaces):**

| Generated output | Target surface (already exists / closed) | Governing ADR |
|---|---|---|
| **(a) Spatial hierarchical tree** — galaxy → sector/cluster → system (`location`) → planet/structure → deposit (`children`) | `scenario` + `hydrate_scenario` → root `World`/`Location` tree; closeout A1 | ADR-MAP (tree = map) |
| **(b) Resource-flow arena enrollment** — deposits = `IntrinsicFlow` sources; system/sector/faction = hierarchical allocators; arenas for minerals/energy/research + a `suppression` arena | CT-2c economy: `ResourceFlowSpec`, gated/`value:` rates, RF arenas, `pressure_binding`; ArenaRegistry (driver) | ADR-RF (arenas, hierarchy, caps) |
| **(c) Bounded 3-layer SEAD heatmap** — suppression-arena pressure → RegionField; L1 bounded theaters, L2 hierarchy reduction, L3 parent EML → threshold → commitment | `RegionFieldSpec` + `StructuredFieldStencilOp`/`SaturatingFlux` (L1) + `SlotRange` Sum (L2) + `ai_will_do` EvalEML (L3) + `FirstSliceCommitmentSpec` | ADR-MAP (3 layers), ADR-RF (`piracy_suppression`) |
| Hyperlane adjacency | `link` → bounded grid/lane-coupling metadata (never a graph object) | closeout A2 |
| Map traversal / influence reach | composed-W → `min_plus_traversal_field` D (PALMA) | BH-2C; closeout A3 |

**What is missing (the whole track):** there is no adapter from Stellaris/ClauseScript mapping script into these generated outputs. MapGen builds a **thin, slice-scoped** generator. It does **not** build a whole-game importer, new substrate, or new GPU kernel.

**Corpus:** vanilla files + logs are read-only at `C:\Users\mvorm\Clauser\Paradox\` and are **not vendored** (§3 M10).

## 2. MapGen closure definition (what 0.0.8.2.5 closes)

MapGen 0.0.8.2.5 is **closed** when a **single starmap slice** (≤ 5 systems, derived from the vanilla
corpus, authored as hand-checked fixtures) is *ingested* and *generates* SimThing star mapping that:

1. **Parses** raw Stellaris/ClauseScript into the neutral AST, **no semantic decisions in the parse pass.**
2. **Generates the spatial hierarchical tree** (galaxy → sector → system → planet → deposit) via `hydrate_scenario`, with **no new `SimThingKind`** and **no RegionCell-as-entity** (ADR-MAP).
3. **Generates resource-flow arena enrollment** — deposits → intrinsic flow; a bounded shallow hierarchy of allocators; **every arena declares explicit selectors + caps + FissionPolicy**, passes the ADR-RF draconian spec firewall, and emits an **expansion report** (ADR-RF).
4. **Generates the bounded 3-layer SEAD heatmap** — suppression-arena pressure → RegionField; **L1 dense stencil only over a bounded theater** (≤ 32×32, H ≤ 8, `source_capped_normalized`, ping-pong); **L2 hierarchy reduction** for the galaxy-scale picture; **L3** parent EML → threshold → commitment. **No dense-global diffusion** (ADR-MAP).
5. **Admits and installs** through the driver and **exercises the GPU-resident path** (resource-flow reduction → L2 → L3 commitment, plus a bounded-theater field exercise) with **compact evidence only** (no full-field CPU decision readback).
6. **Honors every ADR guardrail** — opt-in/default-off, bounded caps, the Candidate-F Euclidean boundary (§3 M9), no atlas/active-mask/perception.
7. Is **documented** as to exactly what is complete vs deferred.

**This is slice-scoped starmap-generation closure — not a Stellaris importer, not playable gameplay,
not editor/corpus/export, not deep galaxy-scale hierarchical allocation.** (§10.)

## 3. Schema adjudications (executive design authority — spent here so PRs are mechanical)

> These extend, and never weaken, the 0.0.8.2 closeout (A1–A5), ADR-MAP, and ADR-RF. Each cites the
> governing ADR section. Where MapGen and an ADR disagree, **the ADR governs and the rung stops.**

**M1 — Neutral AST (no semantic decisions in parse).** Raw text → `RawDocument` via the jomini path;
preserve repeated keys, order, nesting; zero mapping decisions. Mapping is a separate pass. No
load-order/override/localization/trigger interpretation (§10).

**M2 — Spatial hierarchical tree generation (ADR-MAP: tree = map; ADR-RF: hierarchical fanout).** Generate
`galaxy(root) → sector/cluster → system(location) → planet/structure → deposit(children)`. Stellaris
clusters / `home_system_partitions` map to the sector level; each level holds ≤ ~100 children so the
tree is fanout-absorbing. Systems are **location SimThings**; planets/deposits are `children` with
properties/overlays. **A RegionCell is never a `SimThingKind`** — where a system participates in a
heatmap theater, that is a *mapping-role profile binding it to a slot range*, not a tree node (ADR-MAP
"the map, restated"). No new sim type.

**M3 — Resource-flow arena generation (ADR-RF §"Four architectural commitments" + §"Draconian
guardrail").** Deposits → `IntrinsicFlow` sources; system/sector → hierarchical allocators
(`AllocatorWeight` / `AllocatedFlow`); arenas for the economy (minerals/energy/research) **and** a
`suppression` arena (the SEAD source). **Every arena MUST declare:** explicit participant selectors
(no implicit/property-possession admission), hard caps (`max_participants`, `max_coupling_fanout`,
`max_orderband_depth`), a `FissionPolicy` (default `Reevaluate`), and per-edge coupling delay forms
(no all-`Algebraic` cycle). The generator must produce specs that **pass the spec-layer firewall and
emit a clean expansion report** — unsafe content is rejected at session build, never clamped at
runtime. `AccumulatorRole` stays compile-time metadata. **v1 targets the EXISTING closed CT-2c
resource-flow authoring surface with a shallow hierarchy**; deep multi-level galaxy-scale allocation
and large arena coupling are the *architectural target* (the scaling justification) but are **deferred**
until a slice demonstrates the need and a rung opens them under ADR-RF caps (§10).

**M4 — SEAD/suppression heatmap is bounded 3-layer, NEVER dense-global (ADR-MAP §"three-layer model"
+ §"Hard prohibitions").** The galaxy-scale suppression picture is **L2 hierarchy reduction** (the
resource-flow upward sweep into sector/faction suppression columns) **+ L3 parent EML** — *not* a
galaxy-spanning stencil. **L1 dense `SaturatingFlux`/stencil is used only over a bounded local theater**
(≤ 32×32, H ≤ 8, `source_capped_normalized`, ping-pong) where a genuine 2D tactical gradient is wanted.
Decisions emerge as **threshold crossings over the L3 parent pressure columns** (Threshold + EmitEvent
→ commitment); **no CPU planner.** Terminology stays domain-neutral (*suppression/disruption front*);
"SEAD" is design-intent prose only (PR #539 discipline). Default-off.

**M5 — Scale & grid (the corrected decision; supersedes the 2026-06-13 "one galaxy grid sized to N"
sketch).** Galaxy scale is carried by the **sparse hierarchical tree + lane coupling + L2 hierarchy** —
*that* is why 2000+ stars are cheap, not because a dense grid is cheap. **One system = one cell** holds
as **identity/addressing** (one flat column slot per system) and **within a bounded theater**; the
flat per-system column space grows ~linearly with N (fanout-absorbed), which is the only legitimate
sense of "dimensions expand." **No galaxy-spanning dense stencil grid exists** (it would be the rejected
ADR-MAP pattern). Sub-cell x/y is never introduced; Stellaris `position` is inert render metadata (M9).
Dense RegionCell stencil grids are **bounded theaters only**; multi-theater atlas is Provisional and
**not** built here.

**M6 — Hyperlane → bounded link / lane coupling (inherits closeout A2 verbatim — highest leakage risk).**
A hyperlane → `link { from to }` → bounded admission-time topology metadata (validated endpoints,
fan-out ≤ a declared cap). Inter-system field spread is a **bounded gather over lane-neighbors** (a
sparse coupling, ADR-MAP-L2-flavored / ADR-RF coupling edge), **never a dense N4 stencil across a
galaxy raster** and **never a graph object** in `simthing-sim`. Arbitrary non-grid/high-degree topology
beyond the bounded representation → **STOP** (§9), open a topology-spec rung. `random_hyperlanes` has no
production (author explicit topology or nothing).

**M7 — Map traversal → PALMA W/D feedstock (inherits closeout A3 verbatim).** Influence/reach →
composed-W → `min_plus_traversal_field` D over lane impedance. D is a **field**, never a route; no
`route`/`path`/`predecessor`/`waypoint` production. PALMA is the seated utility; **MapGen is its named
consumer.**

**M8 — Gu-Yang ∥ PALMA parallelization (efficiency + front richness; composition-first; within the
ADR-MAP/ADR-RF budget).** Parallelization is **scheduling/composition over existing ops**: (a)
independent regime-distinct suppression fields → concurrent dispatches in one encoder (this is the
"SEAD richness" lever — more independent bounded fronts, not a wider grid); (b) cross-tick software
pipelining of bounded-theater shaping with PALMA relaxation; (c) shared resident tiling. **A fused
single kernel is a new primitive: NOT in the default ladder**, opt-in only behind this named consumer
+ DA review + a measured win + preservation of BH-0 (symmetric flux, zero-flux, CFL χ ≤ 0.25) and
min-plus (`D = W + min(N4 D)`, no sqrt) invariants. **None of this may become galaxy-wide dense
diffusion** — concurrency is over *bounded* theaters/fields, not a global raster (ADR-MAP).

**M9 — Candidate-F Euclidean tripwire (GUARDED, not crossed; §0.7).** Position is inert metadata;
adjacency is topological (links/lane-coupling); map distance is min-plus/impedance, never Euclidean.
Area/radius effects are bounded-theater field stencils, not Euclidean point-radius queries. Any true
Euclidean-distance/spatial-magnitude consumer → **STOP** for §0.7 review (routes through
`m_jit_mag_f_from_exact_mag2` only if ever pulled; out of MapGen scope). PR 9 is the guard rung.

**M10 — Corpus referenced, not vendored.** Vanilla files at `C:\Users\mvorm\Clauser\Paradox\` are
read-only; rungs hand-author tiny fixtures under `tests/fixtures/mapgen/`; no Paradox files committed
(licensing + hygiene). No corpus-wide decode claim in 0.0.8.2.5; any future such claim passes the
`modifiers.log` round-trip bar.

**M11 — Deferred boundary (named, not built).** Out of scope and deferred: deep galaxy-scale
hierarchical allocation + large arena coupling (ADR-RF caps); atlas batching / active masks /
perception fields / behavioral source policy (ADR-MAP Provisional/Deferred — `request_atlas_batching`
stays rejected); whole-corpus coverage; load-order/override; trigger/effect interpretation;
`spawn_weight`/`neighbor_system` procedural placement; localization; `prescripted_countries`; graphical
galaxy; arbitrary-graph topology; pathfinding/movement; editor/corpus/export (`FIELD-MOVIE-DATASET-0`,
the next track).

## 4. PR ladder table

| PR | Title | Owner | ADR-sensitive? | Depends on |
|---|---|---|---|---|
| 1 | Index + corpus manifest + slice selection + ADR read-order | Cursor | — | — |
| 2 | Neutral-AST adapter spike (parse-only, no semantics) | Cursor (DA review) | M1 | 1 |
| 3 | Spatial hierarchical tree generation (galaxy→sector→system→planet→deposit) | Cursor (DA review) | ADR-MAP, ADR-RF · M2 | 2 |
| 4 | Resource-flow arena generation (intrinsic flow + allocators + caps + selectors + expansion report) | Cursor (DA review) | ADR-RF · M3 | 3 |
| 5 | Hyperlane → bounded link + lane coupling + inert position metadata | Cursor (DA review) | A2 · M6 · M9 | 3 |
| 6 | SEAD heatmap: L1 bounded theater + L2 hierarchy reduce + L3 parent EML → commitment | Cursor (DA review) | ADR-MAP · M4 | 4,5 |
| 7 | PALMA W/D map-traversal feedstock (min-plus over lane impedance) | Cursor | A3 · M7 | 5,6 |
| 8 | Gu-Yang ∥ PALMA parallelization spike (bounded; fused kernel opt-in, DA-gated) | Cursor (DA review) | ADR-MAP · M8 | 6,7 |
| 9 | Candidate-F Euclidean + no-dense-global-diffusion + no-galaxy-raster guard | Cursor (DA review) | §0.7 · M4 · M5 · M9 | 5,6 |
| 10 | Canonical sample: ingest → generate tree+arenas+heatmap → admit/install → GPU exercise | Cursor (DA review) | all | 4,6,7,9 |
| 11 | Closeout report + docs + ledger | Cursor (DA sign-off) | M11 | 8,10 |

"DA review" = executive design authority reviews the merge diff against §0/§3. No rung needs a fresh
design pass; §3 closed the gates. A rung **stops and escalates** only on a §9 stop condition.

## 5. Rare Opus / design-authority gates

Design-authority involvement is **merge-time review** of the ADR-sensitive rungs — **2** (no-semantics
parse), **3** (tree/hierarchy + RegionCell-not-a-kind), **4** (arena firewall + caps + expansion
report), **5** (link/lane-coupling boundary), **6** (3-layer / no-dense-global), **8** (parallelization
architecture / any fused-kernel proposal), **9** (Euclidean + scale guards) — and **sign-off** on the
closeout (11). §3 + §0 pre-spent the design gates.

## 6. Cursor-granular PR handoffs

> Each rung: re-read the cited ADR section **before** writing code. Acceptance includes "no ADR
> guardrail crossed." Stop conditions cite §9.

### PR 1 — Index + corpus manifest + slice selection + ADR read-order
Owner: Cursor. Scope: docs only. Files: this doc (append §6.1 manifest); `tests/fixtures/mapgen/README`.
Steps: pin the vanilla files the slice draws from (lab path, read-only); name the slice — **one
`solar_system_initializer` + a ≤ 5-system `static_galaxy_scenario` with explicit `add_hyperlane` and at
least one deposit** (use `vanilla/map/setup_scenarios/static_galaxy_example.txt` + one entry from
`vanilla/common/solar_system_initializers/`); restate M10 (not vendored) and the §0 read-order.
Acceptance: slice + manifest pinned; read-order restated. Stop: §9.

### PR 2 — Neutral-AST adapter spike (parse-only)
Owner: Cursor (DA review). Scope: parse path + fixtures; **no mapping, no spec types.** Re-read ADR-MAP
§"constitutional placement" (semantic-free) before starting. Steps: parse slice fixtures → `RawDocument`;
assert repeated keys/order/nesting preserved; zero semantic decisions. Tests: `mapgen_neutral_ast_parse`.
Acceptance: faithful AST; no `simthing-spec`/`-sim` change. Stop: jomini can't represent a construct → escalate (§9), don't hand-roll a parser.

### PR 3 — Spatial hierarchical tree generation
Owner: Cursor (DA review). Re-read ADR-MAP §"the map, restated" + ADR-RF §"hierarchical fanout."
Scope: mapping pass → `hydrate_scenario`. Steps: generate `galaxy→sector→system→planet→deposit`
(≤ ~100 children/level); systems = `location`, planets/deposits = `children` + properties/overlays +
CT-2c intrinsic flows. **No new `SimThingKind`; no RegionCell entity.** Tests:
`mapgen_tree_lowers_to_hierarchy`, `mapgen_no_new_simthingkind`, default-off/semantic-free asserts.
Acceptance: hierarchical tree on existing surfaces; ADR-MAP tree model honored. Stop: a node needs a new sim type / RegionCell-as-entity → escalate (§9).

### PR 4 — Resource-flow arena generation
Owner: Cursor (DA review). Re-read ADR-RF §"Four commitments" + §"Draconian guardrail" + §"Invariants."
Scope: arena enrollment on the CT-2c surface. Steps: deposits → `IntrinsicFlow`; shallow allocator
hierarchy; declare **selectors + caps (`max_participants`/`max_coupling_fanout`/`max_orderband_depth`) +
`FissionPolicy` + coupling delay forms**; produce a clean **expansion report**; one `suppression` arena
as the SEAD source. Tests: `mapgen_arena_enrolls_with_caps`, `mapgen_arena_rejects_uncapped`,
`mapgen_arena_rejects_algebraic_cycle`, `mapgen_expansion_report`. Acceptance: arenas pass the firewall;
expansion report clean; `AccumulatorRole` compiles away. Stop: the slice needs deep multi-level
allocation / large coupling beyond CT-2c (→ escalate, §9 / §10 deferral).

### PR 5 — Hyperlane → bounded link + lane coupling + position metadata
Owner: Cursor (DA review). Re-read closeout A2 + M6 + M9. Scope: link + lane-coupling + position
mapping. Steps: static `system` → location (position = **inert metadata only**); `add_hyperlane` →
`link` (endpoint validation, fan-out cap); inter-system coupling as a **bounded lane-neighbor gather**,
not a dense raster. Tests: `mapgen_hyperlane_bounded_link`, `mapgen_position_inert`,
`mapgen_arbitrary_topology_rejected`, `mapgen_no_dense_galaxy_raster`. Acceptance: bounded links;
position inert; lane coupling sparse/bounded. Stop: non-grid topology need / dense raster temptation → escalate (§9).

### PR 6 — SEAD heatmap (L1 bounded theater + L2 hierarchy + L3 parent EML)
Owner: Cursor (DA review). **Re-read ADR-MAP §"three-layer model" + §"Hard prohibitions" before
starting — this is the rung most prone to the drift this ladder exists to prevent.** Scope: heatmap
generation on RegionFieldSpec + first-slice surfaces. Steps: suppression-arena pressure → RegionField;
**L1 stencil only over a bounded theater (≤ 32×32, H ≤ 8, `source_capped_normalized`, ping-pong);** L2
`SlotRange` Sum into sector/faction suppression columns; L3 `ai_will_do` EML → threshold → commitment;
default-off. **No galaxy-spanning stencil; no long-horizon dense diffusion.** Tests:
`mapgen_heatmap_l1_bounded_theater`, `mapgen_heatmap_l2_hierarchy_reduce`, `mapgen_heatmap_l3_commitment`,
`mapgen_heatmap_default_off`, `mapgen_no_dense_global_diffusion`. Acceptance: 3-layer honored; theater
bounded; strategic via hierarchy; domain-neutral naming. Stop: a field wants galaxy-wide dense diffusion
or H beyond bounded contract → escalate (§9).

### PR 7 — PALMA W/D map-traversal feedstock
Owner: Cursor. Re-read closeout A3 + M7. Steps: author `palma_feedstock { w_source d_output_col }`
composing W from suppression/lane chokes → `min_plus_traversal_field`. Tests: `mapgen_palma_feedstock_lowers`,
route/movement-vocabulary rejection. Acceptance: D is a field; no route. Stop: route/predecessor need → escalate (§9).

### PR 8 — Gu-Yang ∥ PALMA parallelization spike
Owner: Cursor (DA review). Re-read M8 + ADR-MAP budget. Scope: scheduling/composition over existing GPU
ops; driver/gpu test. Steps: build scheduled-concurrency (independent bounded suppression fields + W
compose + PALMA in one encoder, double-buffered, zero readback); **measure vs serial BH-2C baseline**;
any fused kernel is a separate DA-gated escalation carrying the M8 gate. Tests:
`mapgen_gu_yang_palma_scheduled_concurrency` (GPU-gated, compact timing). Acceptance: concurrency correct
vs serial oracle; bounded (no global raster); no new primitive un-gated. Stop: win needs a new primitive
without DA gate, or violates BH-0/min-plus invariants, or implies galaxy-wide diffusion → escalate (§9).

### PR 9 — Candidate-F Euclidean + scale guards
Owner: Cursor (DA review). Re-read M5 + M9 + §0.7. Scope: guard tests. Steps: prove position inert; no
Euclidean magnitude/sqrt/`length`/`normalize`/`distance(type=euclidean)` on any runtime path; map
distance is min-plus; **no galaxy-spanning dense stencil exists**; per-system columns scale by hierarchy
not raster. Tests: `mapgen_no_euclidean_magnitude_guard`, `mapgen_position_inert_guard`,
`mapgen_no_galaxy_dense_grid_guard`. Acceptance: tripwires enforced by test. Stop: genuine Euclidean
need → §0.7 escalation (out of scope).

### PR 10 — Canonical sample (end to end)
Owner: Cursor (DA review). Scope: canonical fixture + ingest→generate→admit→install→GPU exercise. Steps:
a ≤ 5-system slice → neutral AST → generate tree + arenas (capped, expansion report) + bounded heatmap +
PALMA D + commitment → `open_from_spec` → run a few ticks → assert resource-flow reduction + L2/L3
commitment + bounded-theater field via **compact probe/threshold only.** Tests:
`mapgen_canonical_sample_installs_and_runs` (GPU-gated; CPU path otherwise). Acceptance: full slice
ingests/generates/admits/installs/exercises; compact evidence; zero sim leakage; all ADR guardrails
intact. Stop: install needs a new sim-aware surface / full-field readback / uncapped arena → escalate (§9).

### PR 11 — Closeout report + docs + ledger
Owner: Cursor (DA sign-off). Scope: docs only. Files: `docs/tests/mapgen_0_0_8_2_5_closeout_results.md`
(new, CURRENT_EVIDENCE); this ladder's ledger; pointers in `design_0_0_8_1_border_hack_track.md`,
`design_0_0_8_1_palma_pathfinding_integration_guide.md`, `clausething/MapGenThing.md`,
`design_0_0_8_2_clausething_closeout_ladder.md` §12. Steps: write the closeout (complete vs deferred per
§2/§11); confirm both ADRs honored and Candidate F unmoved; classify all MapGen artifacts. **Do not
declare closed until DA sign-off.** Tests: `cargo fmt --all -- --check`; `cargo test -p simthing-clausething`;
`-p simthing-driver`; `git diff --check`. Acceptance: §2 met; ADRs honored; Candidate F unmoved. Stop: any §2 criterion unmet (→ PARTIAL).

## 7. Test strategy

Focused, fast, GPU-skipping-clean. Cover once each: neutral-AST fidelity; hierarchical-tree lowering;
no-new-`SimThingKind`; arena caps + selector admission + **uncapped/cycle rejection** + expansion
report; hyperlane→bounded-link; position-inert; **no-dense-global-diffusion** and **no-galaxy-raster**
guards; L1-bounded-theater / L2-hierarchy / L3-commitment; default-off; PALMA-D-not-route;
scheduled-concurrency vs serial oracle (compact timing); Euclidean/sqrt guard; end-to-end compact
exercise. **Forbidden:** report-checksum gates, replay theater, prior-rung parity ledgers, > 60s default
tests. Close-out guardrail commands: `cargo test -p simthing-clausething --test mapgen_scenario` + the
canonical driver test.

## 8. Principle compliance (binding, restated for MapGen)

- **ADR-RF four rules:** capability universal; participation explicit (selectors); expansion bounded
  (declared caps + expansion report); unsafe content rejected at build. `simthing-sim` arena-ignorant.
- **ADR-MAP three layers:** dense only in bounded theaters (L1); strategic via hierarchy (L2) + parent
  EML (L3); **never dense-global.** RegionCell = mapping-role on a SimThing, not a `SimThingKind`.
- **Everything is a SimThing.** No per-system map objects, no noun engines, no CPU planner.
- **No new GPU primitive by default** (M8); fused kernels DA-gated + invariant-preserving.
- **No movement/pathfinding/route/predecessor/border/frontline** (M6/M7).
- **Decisions = threshold crossings** over L3 pressure columns. C_u/Gu-Yang shapes; it does not decide.
- **Candidate F unmoved; Euclidean boundary guarded** (M9). **Default-off** (`MappingExecutionProfile::Disabled`).

## 9. Stop conditions (escalate → PARTIAL; do not improvise)

Halt and escalate if a rung hits: **dense-global / long-horizon diffusion temptation** (ADR-MAP rejected
pattern); **a galaxy-spanning dense stencil grid** (M5 breach); **an arena without declared caps /
selectors / a fission policy, or an all-`Algebraic` coupling cycle** (ADR-RF firewall); **RegionCell
treated as a `SimThingKind`/entity** (ADR-MAP breach); arbitrary-graph topology (M6); route/predecessor
production (M7); a Euclidean-distance/magnitude consumer (M9 / §0.7); a new `SimThingKind`/sim-aware
surface to install; full-field CPU readback to decide; a fused GPU kernel without the M8 gate; deep
galaxy-scale hierarchical allocation beyond CT-2c (M3/§10); a fixture needing a vendored Paradox file
(M10); atlas/active-mask/perception without its ADR-MAP gate.

## 10. Deferred boundary (the subsequent tracks)

Deferred, **not** closed here: deep galaxy-scale hierarchical allocation + large arena coupling (ADR-RF
caps); atlas batching / active masks / perception fields / behavioral source policy (ADR-MAP
Provisional/Deferred); whole-corpus coverage; load-order/override; trigger/effect interpretation;
weighted procedural placement; localization; `prescripted_countries`; graphical galaxy; arbitrary-graph
topology; pathfinding/movement; and the **editor/corpus/export seam** (`FIELD-MOVIE-DATASET-0`), the next
minor track on the 0.0.8.2 §10 boundary with its intrinsic-vs-ambient JEPA-corpus discipline pinned there.

## 11. References

- **Governing ADRs (read first):** [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md), [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md); plus `docs/invariants.md` (Mapping + Resource Flow rows).
- Destination / production contract: [`clausething/ct_vertical_consumer_contract.md`](clausething/ct_vertical_consumer_contract.md); CT-2c economy memo [`clausething/ct_2c_economic_category_memo.md`](clausething/ct_2c_economic_category_memo.md); heatmap/terminology [`clausething/ct_3b_4a_movement_front_heatmap_memo.md`](clausething/ct_3b_4a_movement_front_heatmap_memo.md).
- Stellaris-side detail: [`clausething/MapGenThing.md`](clausething/MapGenThing.md).
- Closeout this extends (sign-off stands): [`tests/clausething_closeout_results.md`](tests/clausething_closeout_results.md), [`design_0_0_8_2_clausething_closeout_ladder.md`](design_0_0_8_2_clausething_closeout_ladder.md).
- BH-3 / SaturatingFlux: [`design_0_0_8_1_border_hack_track.md`](design_0_0_8_1_border_hack_track.md). PALMA: [`design_0_0_8_1_palma_pathfinding_integration_guide.md`](design_0_0_8_1_palma_pathfinding_integration_guide.md). Candidate F: [`design_0_0_8_1.md`](design_0_0_8_1.md) §0.7.
- Stellaris corpus (read-only, not vendored): `C:\Users\mvorm\Clauser\Paradox\`.
