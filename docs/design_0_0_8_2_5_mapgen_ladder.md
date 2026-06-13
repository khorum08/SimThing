# SimThing 0.0.8.2.5 — MapGen PR Ladder (Stellaris Starmap → ClauseThing)

> **Status: DESIGN / READY FOR CURSOR EXECUTION (2026-06-13, executive design authority).**
> This is the planning artifact that **pulls the deferred corpus-import / map-generation consumer**
> named in the 0.0.8.2 closeout. It is not an implementation PR. It pins the schema judgments (§3)
> so the implementation rungs are Cursor-mechanical, names the Stellaris-side detail by reference to
> [`clausething/MapGenThing.md`](clausething/MapGenThing.md), and reserves only the leakage-sensitive
> and architecture rungs for Opus / design-authority review.
>
> **0.0.8.2.5 rationale — extension, not reopening of sign-off.** 0.0.8.2 closed the
> ClauseThing/BH/PALMA **authoring-import / runtime-feedstock** surfaces with DA sign-off
> (2026-06-13, [`tests/clausething_closeout_results.md`](tests/clausething_closeout_results.md)) and
> explicitly **deferred** the corpus-import consumer. That sign-off **stands and is not reopened.**
> What 0.0.8.2.5 does is *name the consumer the closeout was waiting for*: the standing guardrail
> "PALMA/min-plus is a seated generic GPU utility **awaiting a named consumer**"
> ([`clausething/ct_vertical_consumer_contract.md`](clausething/ct_vertical_consumer_contract.md))
> and the deferred "BH-3 ClauseThing authoring of map-specific field behaviors"
> ([`design_0_0_8_1_border_hack_track.md`](design_0_0_8_1_border_hack_track.md)) are both pulled by
> **one consumer: a Stellaris starmap adapter**. It earns a point-five doc because it widens the
> *front-end* (parse + map) without reopening the closed runtime/GPU spine. The editor/corpus/export
> seam (0.0.8.2 §10 / `FIELD-MOVIE-DATASET-0`) remains the **subsequent** track, untouched here.
>
> **No constitution change.** `design_0_0_8_1.md` §0.7 (Candidate F) and `simthing_core_design.md`
> are untouched. This ladder adds a named consumer; it does not amend doctrine. The single doctrinal
> tripwire it must respect is the Candidate-F Euclidean boundary (§3 M7), which it **guards**, never
> crosses by default.

---

## 1. Current-state assessment (verified 2026-06-13)

**The destination is closed and signed off.** Everything a starmap lowers *into* already exists as
generic `simthing-spec` surfaces (0.0.8.2 DA-approved). MapGen adds a *source-side adapter*, not new
runtime:

| What MapGen needs to emit | Already-closed target surface | Source |
|---|---|---|
| star system / sector node | `scenario.location { … }` → child of root `World` (`hydrate_scenario`) | closeout A1 |
| planet / deposit / moon under a system | `children` + properties + overlays | CT-1a / CT-2c |
| hyperlane adjacency | `link { from to }` → **bounded N4 grid metadata** (never a graph object) | closeout A2 |
| nebula / storm / threat / supply / influence map field | `RegionFieldSpec` + `RegionFieldOperatorSpec::SaturatingFlux` (Gu-Yang) | BH-0 / closeout A* |
| map traversal / reach / influence spread | composed-W → `min_plus_traversal_field` D (PALMA) | BH-2C / PALMA PATH-8 |
| map decision (fortify / expand / cut-supply) | ai_will_do urgency → threshold → `CommitmentEffectSpec` via `BoundaryRequest::AttachOverlay` | CT-3b+4a |
| parse of raw text | jomini text path → `RawDocument` (`parse_raw_document`) | CLOSED |

**What is missing (the entire scope of this track):** there is **no adapter from raw
Stellaris/Clausewitz text into the ClauseThing scenario surfaces.** No file-family mapping
(`solar_system_initializers`, `setup_scenarios`, `static_galaxy_scenario`), no
initializer→location/children mapping, no `add_hyperlane`→link mapping, no map-field authoring from
Stellaris idioms. The closeout report says this verbatim: *full ClauseScript corpus coverage and
corpus/import scope remain deferred.* 0.0.8.2.5 builds a **thin, one-directional, slice-scoped**
adapter — not a whole-game importer.

**Corpus availability.** The full vanilla Stellaris corpus + generated logs are present read-only at
`C:\Users\mvorm\Clauser\Paradox\` (`vanilla/common/solar_system_initializers/*.txt`,
`vanilla/map/setup_scenarios/{tiny,small,medium,large,huge,static_galaxy_example}.txt`,
`vanilla/map/galaxy/`, `script_documentation/{effects,triggers,scopes,modifiers,localizations}.log`).
It is **not vendored** into this repo (see §3 M8).

## 2. MapGen closure definition (what 0.0.8.2.5 closes)

MapGen 0.0.8.2.5 is **closed** when a **single starmap slice** — derived from the vanilla corpus but
authored as hand-checked fixtures — does all of the following with **zero `simthing-sim` semantic
leakage**:

1. **Parses** representative raw Stellaris text (one `solar_system_initializer` + a minimal
   `static_galaxy_scenario`) into the existing `RawDocument` neutral AST, preserving repeated keys,
   declaration order, and nesting, **making no semantic decisions in the parse pass**.
2. **Maps** that slice into the existing `scenario`/`location`/`link`/`children` surfaces and the
   field/PALMA/commitment sub-blocks — reusing `hydrate_scenario`, introducing **no new
   `SimThingKind`, no new spec lowering target, no graph object**.
3. **Authors map-scale fields** (nebula/storm/threat as a `RegionField` + Gu-Yang `SaturatingFlux`
   suppression/disruption front) and **map traversal feedstock** (composed-W → PALMA D), default-off.
4. **Admits and installs** through the driver (`open_from_spec`) and **exercises the GPU-resident
   field path** (Gu-Yang + PALMA + commitment) under a focused test with **compact evidence only**
   (no full-field CPU decision readback).
5. **Honors the Candidate-F Euclidean boundary** (§3 M7): position is inert metadata, adjacency is
   topological, no Euclidean magnitude on any runtime path.
6. Is **documented** as to exactly what is complete vs deferred.

**This is starmap-adapter / authoring closure for one slice — not a Stellaris importer, not playable
gameplay, not editor/corpus/export.** Whole-corpus coverage, scripted triggers/effects
interpretation, localization, prescripted empires, and the graphical galaxy stay deferred (§10).

## 3. Schema adjudications (executive design authority — spent here so PRs are mechanical)

> These extend, and do not weaken, the 0.0.8.2 closeout adjudications A1–A5. Where this track touches
> a surface the closeout already adjudicated, the closeout letter is cited and **inherited verbatim.**

**M1 — Stellaris neutral AST (no semantic decisions in parse).** Raw `.txt` is parsed through the
existing jomini text path into `RawDocument`. The parser **preserves repeated keys** (Clausewitz
allows `planet = {…} planet = {…}`), declaration order, and nested blocks, and makes **zero** mapping
decisions. Semantic mapping is a **separate pass** over the neutral AST. No load-order resolution, no
override merging, no localization, no trigger/effect evaluation in this track — those are explicitly
out of scope (§10). The neutral AST is a faithful structural mirror, nothing more.

**M2 — `solar_system_initializer` / static `system` → `location` + children (inherits closeout A1).**
An initializer (or a `static_galaxy_scenario` `system = { … }` entry) maps to one `scenario.location`
node. `planet`/`moon`/`deposit` children map to `children` SimThing nodes with properties/overlays
(planet class/size → property tags; deposit → intrinsic resource flow per CT-2c). `init_effect`
payloads map to authored `CommitmentEffectSpec` / init overlays (closeout PR6 path), **never** to a
runtime effect interpreter. Reusable initializers map to reusable fixture fragments. **No new sim
type; locations are children of root**, exactly as A1.

**M3 — `add_hyperlane` → bounded link (inherits closeout A2 verbatim — highest leakage risk).** A
hyperlane edge maps to `link { from to }`, lowered to **bounded admission-time grid topology
metadata** (N4 neighborhood, validated endpoints, bounded fan-out ≤ 4). **A link is never a graph
object, edge struct, or topology engine in `simthing-sim`.** Stellaris galaxies are arbitrary graphs;
v1 MapGen represents only the **grid-embeddable** subset. The day a slice needs non-grid adjacency,
the rung **STOPS** (§9) and opens a topology-spec rung — it does **not** silently widen `link` into a
graph. `random_hyperlanes`/procedural density are engine concepts with **no production** in this
grammar (the adapter authors explicit topology or nothing).

**M4 — Map-scale fields → Gu-Yang `SaturatingFlux` suppression/disruption front (domain-neutral).**
Nebula/storm/threat/supply/influence become `RegionField`s over the location grid, shaped by the
Gu-Yang `SaturatingFlux` operator (state-dependent conservative flux; choke/saturation output).
**Terminology discipline (binding, per PR #539 / the heatmap memo):** the decision front is named
with **domain-neutral** vocabulary — *suppression/disruption front*, *management front*, *pressure
front* — in all code, spec, and test identifiers. "SEAD" is retained **only** as design-intent
shorthand in prose for *the richness of that composed front*; literal SEAD/air-defense domain
semantics are **not** imported into any runtime path. The richness goal ("SEAD richness") is achieved
by composing **many regime-distinct independently-sourced fields**, not by special-case engines.

**M5 — Map traversal → PALMA W/D feedstock (inherits closeout A3 verbatim).** Influence spread /
reachability / "shortest-hyperlane-equivalent" gradients map to composed-W
(`base_w + Σ weightᵢ·chokeᵢ` over threat/supply/congestion) → `min_plus_traversal_field` D. D is a
**field**, never a route. The grammar has **no production** for `route`, `path`, `plan`,
`predecessor`, `waypoint`, `movement_order`, or destinations — unrepresentable, not merely rejected.
PALMA stays the seated generic GPU utility; **this track is the named consumer that activates it.**

**M6 — Gu-Yang ∥ PALMA parallelization (efficiency + front richness; composition-first, no new
primitive by default).** The proven baseline (BH-2C) is the **serial** per-tick chain: Gu-Yang shapes
choke columns → W-compose → PALMA min-plus relaxes D. That baseline is the fallback and the
correctness oracle. The parallelization this track seeks is achieved **by scheduling and composition
over the existing primitives — not a new kernel:**
  - **(a) Field-family concurrency.** Independent regime-distinct fields (threat, supply, influence,
    congestion) each get their own `SaturatingFlux` dispatch; these are **data-independent** and are
    encoded in one command buffer without intervening barriers so the GPU overlaps them. *This is
    where "SEAD richness" comes from* — more independent fronts at ~constant wall-time given occupancy
    headroom.
  - **(b) Cross-tick software pipelining.** PALMA min-plus needs K relaxation iterations (the long
    pole); Gu-Yang is one CFL-bounded pass. Overlap tick N+1 field-shaping with the tail of tick N's
    PALMA relaxation using double-buffered resident columns — **zero readback**.
  - **(c) Shared resident tiling.** Both are N4 stencils over the same grid; share workgroup
    tiling/resident buffers so choke columns feed W-compose in place, avoiding round-trips.
  A genuinely **fused single-kernel** Gu-Yang-flux + min-plus relaxation IS a new GPU primitive and is
  **NOT in the default ladder.** It may be *explored* (PR 7) strictly as an opt-in, gated behind: this
  named consumer (satisfied) **+** explicit DA review **+** a measured win over the
  scheduled-concurrency baseline **+** preservation of the BH-0 invariants (symmetric flux, zero-flux
  boundary, CFL χ ≤ 0.25) and the min-plus `D = W + min(N4 D)` exactness (no sqrt). If it does not
  clearly earn its place, **scheduled concurrency is the answer and the fused kernel is dropped.**

**M7 — Candidate-F Euclidean tripwire (the single highest doctrinal risk — GUARDED, not crossed).**
Stellaris positions are Euclidean (`position = { x y }`; `distance = { type = euclidean }`). The
adjudication:
  - **Position is inert layout/presentation metadata** carried on the location node. It is **not** a
    runtime magnitude and **not** a source of adjacency or fields.
  - **Adjacency is topological** (links / grid N4), per M3. **Map distance is impedance/min-plus
    (Manhattan/topological)**, per M5 — never a Euclidean norm.
  - Area/radius effects (nebula "within radius R") are expressed as **field stencils over the grid**,
    not Euclidean point-radius queries.
  - **If any rung is tempted to compute a true Euclidean-distance field or spatial magnitude, it
    STOPS for design review.** Such a consumer is out of MapGen scope; if ever pulled, it routes
    through `m_jit_mag_f_from_exact_mag2` per `design_0_0_8_1.md` §0.7. Default MapGen needs none of
    it. PR 8 is a dedicated guard rung proving position is inert and no Euclidean magnitude reaches a
    runtime path.

**M8 — Corpus discipline (referenced, not vendored).** The vanilla corpus at
`C:\Users\mvorm\Clauser\Paradox\` is **read-only reference**. Rungs **hand-author tiny fixtures**
(a few lines excerpted/transcribed into `crates/simthing-clausething/tests/fixtures/mapgen/`), never
copy Paradox files into the repo (licensing + hygiene). Any claim of *corpus-wide* decode breadth must
pass the `modifiers.log` round-trip admission bar (CT consumer contract) — but **0.0.8.2.5 makes no
corpus-wide claim**; it proves one slice. Fixture provenance (which vanilla file a fixture transcribes)
is recorded in the fixture header comment.

**M9 — Deferred boundary (named, not built).** Out of 0.0.8.2.5 scope and explicitly deferred:
whole-corpus coverage; load-order/override resolution; scripted trigger/effect interpretation;
`spawn_weight`/`neighbor_system` weighted procedural placement (authored-weight sugar only, no engine
emulation); localization; `prescripted_countries`; graphical galaxy (`map/galaxy/*`, textures,
sprites); arbitrary-graph topology; pathfinding/movement/route/predecessor; editor/corpus/export
(`FIELD-MOVIE-DATASET-0`, the next track). None of these is implemented here.

## 4. PR ladder table

| PR | Title | Owner | Theme | Depends on |
|---|---|---|---|---|
| 1 | MapGen ladder index + corpus reference manifest + slice selection | Cursor | scoping | — |
| 2 | Stellaris neutral-AST adapter spike (parse-only, no semantics) | Cursor (DA review) | M1 | 1 |
| 3 | `solar_system_initializer` → `location` + children mapping | Cursor (DA review) | M2 | 2 |
| 4 | `add_hyperlane` / static `system` → bounded link + position-metadata | Cursor (DA review) | M3 | 3 |
| 5 | Map-scale field authoring (Gu-Yang suppression/disruption front) | Cursor | M4 | 3 |
| 6 | PALMA W/D map-traversal feedstock authoring | Cursor | M5 | 5 |
| 7 | Gu-Yang ∥ PALMA parallelization spike (scheduled concurrency; fused kernel opt-in) | Cursor (DA review) | M6 | 6 |
| 8 | Candidate-F Euclidean tripwire guard + position-as-metadata proof | Cursor (DA review) | M7 | 4 |
| 9 | Canonical MapGen starmap sample: parse→lower→admit→install→GPU exercise | Cursor (DA review) | M2–M6 | 4,6,8 |
| 10 | MapGen closeout report + docs + ledger | Cursor (DA sign-off) | M9 | 7,9 |

"DA review" = executive design authority reviews the merge diff against §3 (no fresh design pass; the
judgment is pre-spent here). A rung **stops and escalates** only on a §9 stop condition.

## 5. Rare Opus / design-authority gates

Schema, leakage-boundary, and architecture decisions are **adjudicated in §3**. Residual
design-authority involvement is **merge-time review** of the leakage-/architecture-sensitive rungs —
**PR 2** (no-semantics parse boundary), **PR 3** and **PR 4** (the A1/A2 mapping + the link
leakage boundary), **PR 7** (the parallelization architecture / any fused-kernel proposal), **PR 8**
(the Candidate-F guard) — and **sign-off** on the closeout report (PR 10). No rung requires a new Opus
*design* pass; §3 closed those gates.

## 6. Cursor-granular PR handoffs

### PR 1 — MapGen ladder index + corpus reference manifest + slice selection
Owner: Cursor. Scope: docs only.
Files: this doc (append §6.1 manifest); none else.
Steps: Record the exact vanilla files the slice draws from (lab path, read-only); name the chosen
slice — **one `solar_system_initializer` + a ≤5-system `static_galaxy_scenario` with explicit
`add_hyperlane`** (prefer `vanilla/map/setup_scenarios/static_galaxy_example.txt` as the static
reference and one small entry from `vanilla/common/solar_system_initializers/`); restate the
not-vendored rule (M8). Create `crates/simthing-clausething/tests/fixtures/mapgen/` with a README
stating fixture-provenance discipline.
Tests: none. Acceptance: slice + corpus manifest pinned; not-vendored rule restated. Stop: corpus file
needed for the slice is absent at the lab path (→ PARTIAL).

### PR 2 — Stellaris neutral-AST adapter spike (parse-only)
Owner: Cursor (DA review). Scope: parse path + fixtures; **no mapping**.
Files: a `mapgen` parse helper in `simthing-clausething` over the existing jomini text path; tiny
hand-authored fixtures under `tests/fixtures/mapgen/`.
Steps: Parse the slice fixtures into `RawDocument`; assert repeated keys preserved, order preserved,
nesting faithful. **No semantic mapping, no spec types touched.**
Tests: `mapgen_neutral_ast_parse` (round-trip structural asserts). Acceptance: faithful neutral AST;
zero semantic decisions; no `simthing-spec`/`simthing-sim` change. Stop: the jomini path cannot
represent a needed Clausewitz construct (→ escalate; do not hand-roll a second parser).

### PR 3 — `solar_system_initializer` → `location` + children
Owner: Cursor (DA review). Scope: mapping pass + lowering via `hydrate_scenario`.
Files: a `mapgen` mapping module; clausething tests.
Steps: Map one initializer → `scenario.location` + `children` (planet/deposit → child nodes +
properties/overlays + CT-2c intrinsic flows). Lower through `hydrate_scenario` to
`HydratedScenarioPack`. **No new `SimThingKind`/spec target.**
Tests: `mapgen_initializer_lowers_to_location_children` (+ default-off, semantic-free asserts).
Acceptance: initializer slice lowers into existing surfaces; no sim leakage. Stop: a child needs a new
sim type or a runtime effect interpreter (→ escalate).

### PR 4 — `add_hyperlane` / static `system` → bounded link + position metadata
Owner: Cursor (DA review). Scope: link + position-metadata mapping (A2/M3, M7).
Files: `mapgen` mapping module; clausething tests.
Steps: Map static `system { id position initializer }` → location (position as **inert metadata
only**); `add_hyperlane { from to }` → `link` with endpoint validation + fan-out ≤ 4. Reject /
**STOP** on arbitrary-graph need. Prove position never feeds adjacency or a field.
Tests: `mapgen_hyperlane_lowers_to_bounded_link`, `mapgen_position_is_inert_metadata`,
`mapgen_arbitrary_topology_is_rejected`. Acceptance: links bounded + validated; position inert; no
graph object. Stop: slice needs non-grid topology (→ escalate, topology-spec rung).

### PR 5 — Map-scale field authoring (Gu-Yang suppression/disruption front)
Owner: Cursor. Scope: field authoring via existing `RegionFieldSpec` + `SaturatingFlux`.
Files: `mapgen` mapping module; clausething tests.
Steps: Author a nebula/threat field as a `RegionField` over the location grid with a Gu-Yang
`SaturatingFlux` operator (domain-neutral identifiers per M4); default-off; preserve BH-0 invariants
in admission. Multiple independent fields allowed (sets up M6 richness).
Tests: `mapgen_field_lowers_to_saturating_flux`, `mapgen_field_default_off`, neutral-naming guard.
Acceptance: field lowers to generic operator; default-off; no border/frontline service vocabulary.
Stop: a field wants state the operator can't carry without a new kernel (→ escalate).

### PR 6 — PALMA W/D map-traversal feedstock authoring
Owner: Cursor. Scope: composed-W + min-plus D authoring (A3/M5).
Files: `mapgen` mapping module; clausething tests.
Steps: Author `palma_feedstock { w_source d_output_col }` composing W from PR5 chokes; lower via the
BH-2C bridge to `min_plus_traversal_field`. **No route/predecessor grammar.**
Tests: `mapgen_palma_feedstock_lowers`, route/movement-vocabulary rejection. Acceptance: D is a field;
no route production; PALMA activated as named consumer. Stop: a consumer wants a route/predecessor
object (→ escalate; that is a different track).

### PR 7 — Gu-Yang ∥ PALMA parallelization spike
Owner: Cursor (DA review). Scope: scheduling/composition over existing GPU ops; **driver/gpu test**.
Files: a driver/gpu spike test; possibly a command-encoder composition helper — **no new WGSL
semantics** unless a fused-kernel proposal is explicitly escalated.
Steps: Build the scheduled-concurrency path (M6 a/b/c): multiple independent `SaturatingFlux`
dispatches + W-compose + PALMA in one encoder, double-buffered resident columns, zero readback;
**measure wall-time vs the serial BH-2C baseline** with compact evidence. If a fused single kernel is
proposed, it is a **separate escalation** carrying the M6 gate (DA review + measured win + invariant
preservation); the ladder does not require it.
Tests: `mapgen_gu_yang_palma_scheduled_concurrency` (GPU-gated; skips cleanly without adapter; compact
timing/probe evidence only). Acceptance: scheduled-concurrency path correct vs serial oracle, no
readback, no new primitive; any fused-kernel work is DA-gated and invariant-preserving. Stop: a win
requires a new primitive without DA approval, or violates BH-0/min-plus invariants (→ escalate).

### PR 8 — Candidate-F Euclidean tripwire guard
Owner: Cursor (DA review). Scope: guard tests + admission assertions (M7).
Files: clausething/spec guard tests.
Steps: Prove position is inert metadata (never read into a field/adjacency); prove no Euclidean
magnitude / sqrt / `length` / `normalize` / `distance(type=euclidean)` reaches any runtime path; assert
map distance is min-plus/topological only. Add a banned-construct guard test mirroring the existing
forbidden-vocabulary guards.
Tests: `mapgen_no_euclidean_magnitude_guard`, `mapgen_position_inert_guard`. Acceptance: tripwire
enforced by test; Candidate F not implicated. Stop: the slice genuinely needs Euclidean distance
(→ escalate to §0.7 design review; out of MapGen scope).

### PR 9 — Canonical MapGen starmap sample (end to end)
Owner: Cursor (DA review). Scope: canonical fixture + parse→lower→admit→install→GPU exercise.
Files: canonical `mapgen` fixture; driver test (reuse `open_from_spec` + session loop).
Steps: A small authored starmap (≤5 systems, 1–2 initializers, explicit hyperlanes, one
suppression/disruption field, one PALMA D, one commitment). Parse → map → `hydrate_scenario` →
`open_from_spec` → run a few ticks → assert compact probe/threshold event (no full-field readback).
Tests: `mapgen_canonical_sample_installs_and_runs` (GPU-gated; CPU path for non-GPU; skips Test-B-style
cleanly). Acceptance: full slice admits/installs/exercises; compact evidence only; zero sim leakage.
Stop: install needs a new sim-aware surface or full-field readback to decide (→ escalate).

### PR 10 — MapGen closeout report + docs + ledger
Owner: Cursor (DA sign-off). Scope: docs only.
Files: `docs/tests/mapgen_0_0_8_2_5_closeout_results.md` (new, CURRENT_EVIDENCE); this ladder's ledger;
pointers in `design_0_0_8_1_border_hack_track.md` (BH-3 authoring consumer pulled),
`design_0_0_8_1_palma_pathfinding_integration_guide.md` (PALMA named consumer activated),
`clausething/MapGenThing.md` (status), and `design_0_0_8_2_clausething_closeout_ladder.md` §11.
Steps: Write the closeout report; state complete vs deferred (§2/§9); record the Gu-Yang∥PALMA result;
confirm Candidate F unmoved; classify all MapGen artifacts. **Do not declare closed until DA sign-off.**
Tests: `cargo fmt --all -- --check`; `cargo test -p simthing-clausething`; `-p simthing-driver`;
`git diff --check`. Acceptance: §2 criteria met; honest complete-vs-deferred; Candidate F unmoved.
Stop: any §2 criterion unmet (→ PARTIAL with the precise gap).

## 7. Test strategy

Focused, fast, GPU-skipping-clean — mirror the closeout battery. Cover once each: neutral-AST parse
fidelity; initializer→location/children lowering; hyperlane→bounded-link; position-inert;
arbitrary-topology rejection; field→Gu-Yang default-off; PALMA-D-not-route; scheduled-concurrency vs
serial oracle (compact timing); Euclidean/sqrt guard; end-to-end admit/install/GPU compact exercise.
**No** report-checksum gates, replay theater, prior-rung parity ledgers, or >60s default tests. New
guardrail commands at close:
`cargo test -p simthing-clausething --test mapgen_scenario` and the canonical driver test.

## 8. SimThing principle compliance (binding, restated for MapGen)

- **Everything is a SimThing.** Systems/planets/deposits are locations/children; no per-system map
  objects, no noun engines.
- **No new GPU primitives by default** (M6); any fused kernel is DA-gated and invariant-preserving.
- **`simthing-sim` stays ClauseThing/Stellaris-blind.** The adapter lives in `simthing-clausething`
  (+ tests); the sim sees only the resolved tree + columns + links + fields.
- **No movement/pathfinding/route/predecessor/border/frontline** vocabulary or objects (M5/M3).
- **Management/suppression-disruption front** is the decision arena: composed pressure → ai_will_do
  urgency → threshold → `BoundaryRequest` commitment. C_u (Gu-Yang) *shapes*; it does not *decide*.
- **Candidate F unmoved**; Euclidean boundary guarded (M7).
- **Default-off** (`MappingExecutionProfile::Disabled`); opt-in `SparseRegionFieldV1` + pressure
  binding gates the vertical; bounded fan-out / stowaway budgets respected.

## 9. Stop conditions (escalate, do not improvise)

A rung halts → PARTIAL and escalates if it hits: arbitrary-graph topology need (M3 breach);
route/predecessor production temptation (M5 breach); a Euclidean-distance/magnitude consumer (M7
tripwire); a new `SimThingKind`/sim-aware surface required to install; full-field CPU readback to make
a decision; a fused GPU kernel proposed without the M6 gate; a fixture requiring a vendored Paradox
file (M8 breach); whole-corpus decode breadth claimed without the `modifiers.log` round-trip.

## 10. Deferred boundary (the subsequent tracks)

0.0.8.2.5 closes **one starmap slice adapter**. Explicitly deferred and **not** closed here:
whole-corpus coverage; load-order/override; scripted trigger/effect interpretation; weighted
procedural placement (`spawn_weight`/`neighbor_system` engine behavior); localization;
`prescripted_countries`; graphical galaxy; arbitrary-graph topology; pathfinding/movement; and the
**editor/corpus/export seam** (`FIELD-MOVIE-DATASET-0`), which remains the next minor track on the
0.0.8.2 §10 boundary, with its intrinsic-vs-ambient JEPA-corpus discipline pinned there.

## 11. References

- Stellaris-side detail (reference textbook): [`clausething/MapGenThing.md`](clausething/MapGenThing.md)
- Destination surfaces / production contract: [`clausething/ct_vertical_consumer_contract.md`](clausething/ct_vertical_consumer_contract.md)
- Closeout this extends (sign-off stands): [`tests/clausething_closeout_results.md`](tests/clausething_closeout_results.md), [`design_0_0_8_2_clausething_closeout_ladder.md`](design_0_0_8_2_clausething_closeout_ladder.md)
- BH-3 authoring / SaturatingFlux: [`design_0_0_8_1_border_hack_track.md`](design_0_0_8_1_border_hack_track.md)
- PALMA W/D utility (named-consumer-activated): [`design_0_0_8_1_palma_pathfinding_integration_guide.md`](design_0_0_8_1_palma_pathfinding_integration_guide.md)
- Candidate F authority: [`design_0_0_8_1.md`](design_0_0_8_1.md) §0.7
- Heatmap front / terminology discipline: [`clausething/ct_3b_4a_movement_front_heatmap_memo.md`](clausething/ct_3b_4a_movement_front_heatmap_memo.md)
- Stellaris corpus (read-only, not vendored): `C:\Users\mvorm\Clauser\Paradox\`
