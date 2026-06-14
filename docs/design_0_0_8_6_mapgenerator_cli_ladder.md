# SimThing 0.0.8.6 — MapGeneratorCLI PR Ladder (high-level galaxy params → declarative MapGen payloads)

> **Status: DESIGN / READY FOR CURSOR EXECUTION (track-opening plan, 2026-06-14, executive design authority).**
> This is the planning artifact for the **producer layer** above the now-closed 0.0.8.2.5 MapGen
> ingest/lowering ladder. It is not an implementation PR. It pins the producer-side schema judgments (§3)
> so the rungs are Cursor-mechanical with Codex reviews, and it is **subordinate to the core-design
> paradigm, the two governing ADRs, and the closed 0.0.8.2.5 contract (§0).**
>
> **What MapGeneratorCLI is.** A **thin, standalone, declarative producer**: it turns high-level galaxy
> levers (`--shape=4_armed_spiral --num_stars=1000 --num_arms=4 --seed=42 …`) into the **declarative
> ClauseScript `scenario { location … link … field_operator … initializer refs … }` payload** that the
> *already-closed* 0.0.8.2.5 MapGen front-end ingests and lowers. **It is NOT a runtime, NOT the MapGen
> lowerer, NOT a UI** — it is the missing "Galaxy Shape Generator" between UI levers and the lowering path.
> Reference: [`clausething/MapGeneratorCLI.md`](clausething/MapGeneratorCLI.md). Where this ladder and the
> reference disagree, **the ladder governs.**
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

**The CLI may produce:**
- high-level parameter parsing + a parameters-file (`--params=ui.json`) mode;
- deterministic, seed-driven procedural placement of systems on a **square** integer lattice
  (one-system-per-cell), with float curve math **immediately quantized** to integer `(col, row)` cells;
- declarative `scenario { metadata, location …, link …, field_operator …, system_initializer refs }` text
  (preferred: the `hydrate_scenario` ClauseScript form; fallbacks: `static_galaxy_scenario` block or a
  manifest + tiny `.txt` initializer library);
- bounded explicit links + bounded long-range lane couplings (per M3/M6 fanout caps);
- at least one sample suppression/environmental `field_operator` (so output exercises RF→RegionField→
  Gu-Yang→PALMA when lowered);
- a "dry-run / manifest-only" mode emitting a human-readable placement report (UI preview).

**The CLI must NOT (hard prohibitions — any one ⇒ STOP/REJECT):**
`simthing-sim` awareness or changes · a new `SimThingKind` · runtime simulation behavior · a new GPU
kernel · semantic WGSL · pathfinding · routes · predecessors · movement orders · border/frontline
services · CPU planning over fields · full-field reads · Euclidean *authority* in the output (positions
are inert render metadata; adjacency is lattice/topological — float curve math lives only in the producer
and never reaches sim) · vendored Paradox files in the repo · opening FIELD-MOVIE-DATASET-0 · widening the
0.0.8.2.5 lowering layer or `simthing-spec`.

**Candidate-F tripwire:** the CLI's internal arm-curve/jitter float math (sin/cos/sqrt for sampling) is a
*producer-side* convenience that is **quantized to integer cells before emission** and never appears in
the declarative output or any sim-authoritative path. The *output contract* carries no Euclidean
distance/magnitude/nearest-neighbor authority. If any rung needs Euclidean *authority* in the lowered
surface, STOP (§7) — that is the M9/§0.7 boundary the 0.0.8.2.5 ladder already guards.

## 1. Preserved-baseline contract (the producer's target, from PR11 closeout)

These 0.0.8.2.5 artifacts are the **target contract** the CLI's output must satisfy; do not delete or bury:
- `docs/clausething/mapgen_corpus_manifest.md` — corpus families + approved initializer/setup files the CLI references.
- `crates/simthing-clausething/tests/fixtures/mapgen/` — the accepted declarative input shape + goldens; CLI output must be structurally equivalent (or richer) and lower the same way.
- `docs/tests/mapgen_pr{1,6,7,10,11}_results.md` (+ the per-PR lifecycle tables) — lowering expectations, RF/Movement-Front/PALMA feedstock shape, compact-evidence style, the LIVE_GUARDRAIL battery, the end-to-end harness pattern.
- The 0.0.8.2.5 LIVE_GUARDRAIL battery (`mapgen_*` + `ct_scenario_container` + `ct_bh3_closeout_sample_driver`) **must stay green** as the CLI adds generated cases.

## 2. What 0.0.8.6 closes

Closed when the CLI deterministically turns high-level parameters (at least the tiny static case **and** a
≥1000-star 4-arm spiral *at manifest scale*) into declarative payloads that **lower cleanly through the
closed 0.0.8.2.5 path** and, for at least one generated spiral slice, **admit/install + exercise GPU
compact evidence** on a real adapter — under all §0 constraints, with the LIVE_GUARDRAIL battery green.
The UI itself is **out of scope** (a later, separate consumer); this track ends at "UI-callable producer."

## 3. Producer-side adjudications (executive design authority — spent here so rungs are mechanical)

- **C1 — Output is declarative ClauseScript scenario form** (preferred `scenario { location … link …
  field_operator … system_initializer refs }`, lowered by the existing `hydrate_scenario`/MapGen
  front-end). `static_galaxy_scenario`/manifest are admitted fallbacks. The CLI emits *text*; it never
  builds `simthing-spec`/sim structures.
- **C2 — Square lattice, one-system-per-cell** (core §7 + M5). Lattice edge derived square from
  `num_stars` (or `--lattice_size`), default "medium" 200×200; quantize every placement to a free integer
  cell; reject/relocate collisions deterministically. No sub-cell positions; `position` is inert render metadata.
- **C3 — Determinism.** A single `--seed` drives all placement; a secondary variation seed may vary
  initializer content while holding geometry. Same params+seed ⇒ stable (byte- or semantically-identical) output.
- **C4 — Bounded topology only** (M3/M6). Explicit `link`/`add_hyperlane` within fanout caps; long-range
  edges as bounded **lane couplings**, never arbitrary high-degree graphs, never routes/predecessors.
- **C5 — Initializer references, not new definitions.** The CLI references corpus-approved initializer
  families by key (core/arm/fringe/cluster buckets) and emits minimal `system_initializer` refs/overrides;
  it does not invent runtime semantics.
- **C6 — Field operators are samples only.** Emit ≥1 declarative `field_operator` (nebula/storm) that
  lowers to a `RegionFieldSpec` operator; the CLI never computes fields or touches the runtime.
- **C7 — Standalone producer crate.** The CLI is its own thin crate/binary depending only on a parameter
  model + a tiny emitter (it need not link `simthing-*`). End-to-end *tests* live where they can feed
  output through the existing lowering + driver (as PR5/PR10 patterns do), not in the CLI crate's runtime.
- **C8 — Scale is manifest-first.** Large maps (1000+ stars) are proven at **manifest/dry-run** scale
  first (placement report + lower-check), with a *single* generated slice taken to GPU compact evidence —
  no attempt to install a galaxy-scale dense grid (the 0.0.8.2.5 ladder's bounded-theater/atlas deferrals stand).

## 4. PR ladder table

| PR | Title | Owner | DA-sensitive? | Depends on |
|---|---|---|---|---|
| 0 | Track design + preserved-baseline contract (this doc landed) | Cursor (DA sign-off) | yes | — |
| 1 | CLI crate + parameter skeleton + params-file parse; **no generation** | Cursor (DA review) | C7 | 0 |
| 2 | Deterministic RNG + square lattice occupancy core (one-system-per-cell) | Cursor (DA review) | C2/C3 | 1 |
| 3 | Static / irregular tiny placement (≤ a few systems) → in-memory placements | Cursor | C2 | 2 |
| 4 | Declarative scenario emitter (`scenario { location … }` text) | Cursor (DA review) | C1/C5 | 3 |
| 5 | Generated tiny scenario **through existing MapGen lowering** (parse/hydrate) | Cursor (DA review) | C1/C7 | 4 |
| 6 | Bounded hyperlane topology producer (links + lane couplings, caps) | Cursor (DA review) | C4 | 5 |
| 7 | 2-arm / 4-arm spiral placement (arm curve math, quantized) | Cursor | C2/C3 | 6 |
| 8 | Spiral bridge/choke topology (dense in-arm, sparse cross-arm bridges) | Cursor (DA review) | C4 | 7 |
| 9 | Initializer buckets + sample `field_operator` emission | Cursor | C5/C6 | 8 |
| 10 | Generated spiral slice → admission/install + **GPU compact evidence** | Cursor (DA review) | all | 6,9 |
| 11 | Scale-envelope dry run (e.g. 1000-star **manifest only** + lower-check) | Cursor (DA review) | C8 | 10 |
| 12 | Closeout + ledger + UI handoff note | Cursor (DA sign-off) | — | 11 |

"DA review" = executive design authority reviews the merge diff against §0/§3; **genuine audit + battery
rerun + the report must not pre-file a DA sign-off** (the 0.0.8.2.5 PR3 governance rule carries forward —
only the Design Authority writes a DA sign-off). A rung stops/escalates only on a §7 stop condition.

## 5. DA cadence

DA-review-sensitive rungs (genuine pre-merge audit): **1** (crate boundary — no sim/kind/runtime), **2**
(determinism + one-system-per-cell), **4** (declarative output shape), **5** (lowers through the *closed*
path unchanged), **6** (bounded topology, no graph/route), **8** (choke topology stays bounded), **10**
(admission/install + GPU compact evidence, no full-field readback), **11** (scale stays manifest-only, no
galaxy-scale dense install). **Sign-off** on **0** (track open) and **12** (closeout). The others are
mechanical under §3.

## 6. Per-rung acceptance/stop (granular handoffs)

- **PR1** — new CLI crate (`crates/mapgen-cli` or equiv) + arg/params-file parsing; prints parsed params
  in dry-run; **no placement, no emission.** Accept: builds; no `simthing-*` runtime dep; no sim/kind.
  Stop: a param needs sim/spec types (§7).
- **PR2** — seeded RNG (pinned algorithm) + square-lattice occupancy set; place N points one-per-cell with
  deterministic collision relocation. Accept: same seed ⇒ identical occupancy; square; no sub-cell coords.
  Stop: needs Euclidean *authority* (only quantized float allowed).
- **PR3** — tiny static/irregular placement producing in-memory `(id, col, row, bucket)` rows. Accept:
  deterministic; ≤ small N; buckets are labels only.
- **PR4** — emit declarative `scenario { metadata, location { position(inert) initializer-ref } }` text.
  Accept: text only; positions inert; no links/fields yet. Stop: emitter wants to build spec structs (§7).
- **PR5** — feed the PR4 text through `parse_mapgen_neutral_document` → `hydrate_scenario`/MapGen front-end;
  assert it lowers to the same surfaces (gridcell `Location`s, default-off). Accept: lowers **without
  changing the lowering layer**; LIVE_GUARDRAIL battery green. Stop: front-end rejects ⇒ fix the *producer
  output*, never the front-end (§7).
- **PR6** — emit bounded `link`/lane-coupling; honor M3/M6 fanout caps + N4-vs-long-range classification on
  the lattice. Accept: caps respected; no arbitrary graph; lowers green.
- **PR7** — Archimedean/log spiral arm sampling (float) → quantized cells; arm width/jitter/core+fringe
  density; one-per-cell. Accept: deterministic; square; quantized.
- **PR8** — dense in-arm links + sparse cross-arm **bridge/choke** edges as bounded lane couplings. Accept:
  bounded; deterministic; lowers green. Stop: choke logic implies a route/path (§7).
- **PR9** — initializer-bucket assignment (core/arm/fringe/cluster) referencing corpus families + ≥1 sample
  `field_operator`. Accept: refs only; field_operator lowers to a `RegionFieldSpec` operator.
- **PR10** — take one generated spiral slice through `install_atomic` + `SimSession::open_from_spec` +
  GPU mapping tick with **compact evidence only** (`field_values`/`reduction_parent_value`/`eml_output`
  `is_none()`), GPU-adapter gated, default-off. Accept: mirrors the PR10 0.0.8.2.5 harness; real GPU run.
  Stop: needs full-field readback or new kernel (§7).
- **PR11** — 1000-star (or chosen scale) **manifest/dry-run only** + a lower-check (parse/hydrate admits);
  **no galaxy-scale dense install**. Accept: manifest emitted; lower-check green; bounded.
- **PR12** — closeout report + ledger; classify CLI artifacts; **UI handoff note** (the UI is the next,
  separate consumer). Accept: docs-only; honest; battery green.

## 7. Stop conditions (escalate → PARTIAL; do not improvise)

Halt and escalate if a rung needs: a change to the closed 0.0.8.2.5 lowering layer / `simthing-spec` /
`simthing-sim`; a new `SimThingKind`; a new GPU kernel or semantic WGSL; Euclidean *authority* in the
lowered output (M9/§0.7); routes/predecessors/movement/border/frontline/pathfinding; CPU planning or
full-field readback; arbitrary high-degree/non-grid topology; vendored Paradox files; opening
FIELD-MOVIE-DATASET-0; or galaxy-scale dense install (atlas/multi-theater remains deferred).

## 8. References

- Reference (target direction): [`clausething/MapGeneratorCLI.md`](clausething/MapGeneratorCLI.md).
- The closed contract this produces for: [`design_0_0_8_2_5_mapgen_ladder.md`](design_0_0_8_2_5_mapgen_ladder.md) (§3 M1–M11), [`tests/mapgen_pr11_closeout_results.md`](tests/mapgen_pr11_closeout_results.md), [`tests/mapgen_pr10_end_to_end_results.md`](tests/mapgen_pr10_end_to_end_results.md).
- Grammar/isomorphism: [`clausething/MapGenThing.md`](clausething/MapGenThing.md); corpus: [`clausething/mapgen_corpus_manifest.md`](clausething/mapgen_corpus_manifest.md).
- Paradigm/surfaces: [`simthing_core_design.md`](simthing_core_design.md) §1.1+§7; [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md); [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md); Candidate F [`design_0_0_8_1.md`](design_0_0_8_1.md) §0.7.
- Stellaris corpus (read-only, **not vendored**): `C:\Users\mvorm\Clauser\Paradox\`.
