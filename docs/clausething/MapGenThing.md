# MapGenThing: Stellaris Starmap Generation with ClauseScript — Rigorous Engineering Reference

> **IN-REPO AUTHORITY (added 2026-06-13, executive design authority).** This reference was promoted
> from the lab corpus into the formal `docs/clausething/` set to power the **0.0.8.2.5 MapGen PR
> ladder** ([`../design_0_0_8_2_5_mapgen_ladder.md`](../design_0_0_8_2_5_mapgen_ladder.md)). It is a
> **reference textbook**, not a runtime spec or a license to widen `simthing-spec`: implement the
> *isomorphism* onto the already-closed ClauseThing/BH/PALMA surfaces (0.0.8.2, DA-signed-off), never
> the Clausewitz engine model. Binding adjudications and rung boundaries live in the ladder's §3; this
> doc supplies the Stellaris-side detail those rungs lower from. Where this doc and the ladder's §3
> disagree, **the ladder governs.**
>
> **PR1 corpus pin (2026-06-13):** read-only manifest and tiny slice selection live in
> [`mapgen_corpus_manifest.md`](mapgen_corpus_manifest.md). Hand-authored fixture stub:
> `crates/simthing-clausething/tests/fixtures/mapgen/` (**inert until PR2**). No Paradox files committed.
>
> **PR2 neutral-AST parse-only (2026-06-13, DA-APPROVED):** `parse_mapgen_neutral_document` in
> `mapgen_neutral_ast.rs` wraps the existing jomini/`RawDocument` path. Raw fixture
> `tiny_pentad_hub_slice_raw.clause` is parsed with zero semantic decisions — no lowering to `scenario`,
> locations, links, RF arenas, PALMA feedstock, or commitments. PR3 is the first hierarchy-generation rung.
> No Paradox files committed. No parser/importer runtime.
>
> **PR3 gridcell lattice hierarchy (2026-06-13, DA-APPROVED post-merge):** `generate_mapgen_lattice_hierarchy`
> in `mapgen_lattice.rs` lowers the tiny raw fixture into scenario-container-compatible hierarchy
> (`galaxy_map` → `pentad_sector` → gridcell systems as ordinary `Location` nodes). Mapping-role metadata
> lives under the `mapgen` property namespace — not a new `SimThingKind`. Fixture-local 3×3 placements;
> canonical 200×200 square lattice documented in metadata only; Stellaris positions are inert render
> metadata. No RF, Movement-Front, PALMA, FIELD_POLICY, or hyperlane link output in PR3.
> Audit: [`mapgen_pr3_da_audit_results.md`](../tests/mapgen_pr3_da_audit_results.md) — **genuine DA sign-off
> (Opus, 2026-06-14)** ratifying a Cursor-prefiled approval. **PR4 may proceed.**
>
> **PR4 bounded RF enrollment (2026-06-13, DA-APPROVED after a targeted repair, Opus 2026-06-14):**
> `generate_mapgen_resource_flow_enrollment` in `mapgen_resource_flow.rs` lowers the PR3 hierarchy into
> bounded `ResourceFlowSpec` enrollment: deposit minerals intrinsic-flow feedstock + suppression/disruption
> arena. **Both arenas enroll via `ExplicitOnly`** over their authoritative `explicit_participants`
> (multi-deposit-safe — DA repaired the deposit arena from an `InstallTarget(deposits[0])` selector), with
> arena caps, shallow deposit→suppression coupling, and expansion report. No Movement-Front, SaturatingFlux,
> PALMA, FIELD_POLICY, hyperlane, or runtime/GPU/driver/simthing-sim output in PR4.
> Report: [`mapgen_pr4_resource_flow_results.md`](../tests/mapgen_pr4_resource_flow_results.md).
>
> **PR5 bounded hyperlane links + lane coupling (2026-06-13, DA-APPROVED Opus 2026-06-14):**
> `generate_mapgen_links` in `mapgen_links.rs` lowers PR4 enrollment plus neutral-AST `add_hyperlane`
> declarations into bounded N4 lattice links (`HydratedScenarioGridMetadata.links`) and bounded
> `mapgen::lane_coupling` inert authoring properties for long-range edges. Endpoints validated;
> self-links/unknown endpoints rejected; duplicates canonicalized; caps enforced; Stellaris positions remain
> inert render metadata; lattice row/col adjacency only (no Euclidean authority). No pathfinding, movement,
> routes, predecessors, border/frontline, Movement-Front, SaturatingFlux, PALMA, FIELD_POLICY, or
> runtime/GPU/driver/simthing-sim output in PR5. Report:
> [`mapgen_pr5_links_results.md`](../tests/mapgen_pr5_links_results.md).
>
> **PR6 Movement-Front L1/L2/L3 authoring (2026-06-13, DA-APPROVED + merged `3f411fda`):**
> `generate_mapgen_movement_front_authoring` in `mapgen_movement_front.rs` lowers PR5 enrollment into
> existing Movement-Front authoring surfaces: L1 bounded `SaturatingFlux`/`RegionFieldSpec` with suppression
> RF `ArenaPressureBindingSpec`; L2 hierarchy `RegionFieldReductionSpec`; L3
> `FirstSliceCommitmentSpec`/`HydratedScenarioCommitment`. Default-off mapping profile preserved. No PALMA,
> no driver/GPU execution, no pathfinding/movement/routes/predecessors/border/frontline, no Euclidean
> authority, no runtime/GPU/driver/simthing-sim output in PR6. Report:
> [`mapgen_pr6_movement_front_results.md`](../tests/mapgen_pr6_movement_front_results.md).
>
> **PR7 PALMA W/D reach feedstock (2026-06-13, PASS):**
> `generate_mapgen_palma_feedstock` in `mapgen_palma.rs` lowers PR6 enrollment into existing
> `HydratedScenarioPalmaFeedstock` + generic `WImpedanceComposeSpec` (W from PR6 choke column, D col 4).
> Default-off authoring only. No routes/paths/predecessors/movement orders, no driver/GPU execution, no
> Euclidean authority, no runtime/GPU/driver/simthing-sim output in PR7. Report:
> [`mapgen_pr7_palma_results.md`](../tests/mapgen_pr7_palma_results.md).
>
> **PR8 scheduled-concurrency GPU measurement (2026-06-13, PASS pending DA review):**
> `mapgen_pr8_scheduled_concurrency` driver test + `scheduled_w_palma_batch` helpers compare serial queue
> submits vs single-encoder W compose + PALMA min-plus on the PR7 tiny slice. Compact D probe only. No fused
> kernel, no semantic WGSL, no simthing-sim changes. Report:
> [`mapgen_pr8_scheduled_concurrency_results.md`](../tests/mapgen_pr8_scheduled_concurrency_results.md).
>
> **GOVERNING PARADIGM + ADRs — these outrank this reference.** The map is the **Movement-Front
> automaton**: a grid of **gridcell SimThings run as a cellular automaton** (core design
> [`simthing_core_design.md`](simthing_core_design.md) §1.1 Anchor A + §7, after Zichao Wei
> [arXiv:2602.01651](https://arxiv.org/abs/2602.01651), whose concept is referred to in prose as
> **STEAD** (*SpatioTemporal Evolution with Attractor Dynamics*) — never the military-connoted "SEAD";
> the engine name agents use in code/spec/tests is **Movement-Front**). The galaxy **is** a 2D gridcell lattice — **base canonical dimensions are always square**
> (default "medium" 200×200, scaling up square with star density) — with star systems occupying a
> subset of cells; one system = one gridcell; a gridcell **is a `Location` SimThing** (intrinsic spatial
> identity, never a new **`SimThingKind`** and not a detachable mapping-role; core §7 + the alignment
> amendment in [`../adr/mapping_sparse_regioncell.md`](../adr/mapping_sparse_regioncell.md)). The
> Layer-1 stencil **does** run across the whole lattice — values spill with falloff — but the **per-tick
> horizon is bounded** (P1 light cone; fronts cross the galaxy over many ticks; cadence/dirty make
> compute follow the wavefront). The economy and the **suppression/disruption front are resource-flow
> arenas** ([`../adr/resource_flow_substrate.md`](../adr/resource_flow_substrate.md)) with explicit
> selectors + hard caps + fission policy, rejected at build if unsafe. **Wrong against the paradigm
> (not merely against this textbook):** *widening the stencil horizon to gain instant strategic
> awareness* (use L2 hierarchy instead — P1); *a per-cell bespoke rule* (P2); *giving a star a Euclidean
> coordinate the sim reads for distance/adjacency* (position is inert metadata; distance is min-plus —
> §0.7). The ladder's §0/M4/M5 govern.
>
> **Stellaris corpus is referenced, not vendored.** The vanilla game files and generated logs
> (`common/solar_system_initializers/`, `map/setup_scenarios/`, `map/galaxy/`,
> `script_documentation/{effects,triggers,scopes,modifiers,localizations}.log`) live at the lab path
> `C:\Users\mvorm\Clauser\Paradox\` and are used **read-only** for adjudication and minimal-fixture
> derivation. Paradox game files are **not** committed to this repo (licensing + repo hygiene); rungs
> hand-author tiny `.clause`/excerpt fixtures rather than copying corpus files in. The `modifiers.log`
> round-trip remains the admission bar for any corpus-wide decoder claim (CT consumer contract).

> **STATUS AND PROVENANCE (binding for all consumers).**  
> This document is a secondary synthesis (full revision 2026-06-13) drawn from the lab corpus in `C:\Users\mvorm\Clauser\` (including the full vanilla Stellaris corpus copied into `Paradox/vanilla/` on 2026-06-13) plus targeted external community sources (Paradox forums threads on map generation/custom galaxies/static maps, r/Stellaris/r/StellarisMods discussions, Stellaris Wiki Map modding and Game start modding pages, Dev Diaries noting spawn_system changes, and related mod examples).  
> Primary ground truth now includes:  
> - Local vanilla corpus: `Paradox/vanilla/common/solar_system_initializers/`, `Paradox/vanilla/map/setup_scenarios/`, `Paradox/vanilla/map/galaxy/`, `Paradox/vanilla/prescripted_countries/` (direct copies of the installed game files for parser/corpus-driven work).  
> - `Paradox/script_documentation/{effects.log, triggers.log, scopes.log}` (generated 2026-05-29 from a running Stellaris client; the authoritative, version-exact list of every built-in; can be refreshed from the game and copied here).  
> - `Paradox/01_Modding_Main.md`, `02_Dynamic_Modding_Scripting_Format.md`, `03_Effects_List.md` (wiki snapshots).  
> - `ClauseThing_Spec.md` (Clauser lab copy) and the moved canonical textbook `SimThing/docs/clausething/ClauseThing.md`.  
> - `SimThingModdersGuide.md`, `worklog.md`, `simthing-repoguide.md`.  
> - `jomini/` reference parser (text path used by `simthing-clausething`).  
> - Direct examination of `SimThing/crates/simthing-clausething/` (hydrate_scenario.rs, scenario container fixtures, existing `location`/`link` grammar) and related SimThing docs (core design, mapping ADR references, production track, stellaris-scale benchmark results).  
> External corroboration (forums, wiki, Reddit) supplies idioms, execution order, spawn_weight/neighbor_system patterns, static_galaxy_scenario syntax, scale/perf realities, and community tooling/workarounds. All such material is treated as secondary and provisional. The local vanilla corpus is the authoritative primary source for syntax and structure going forward.  
>  
> **Engine opacity warning:** The core galaxy placement, hyperlane routing algorithm, system coordinate generation, clustering logic, and initial "seed" decisions live in the closed Clausewitz C++ engine. ClauseScript only supplies *declarative payloads* (initializers, setup_scenarios, static_galaxy_scenario system lists) and *imperative post-placement or mid-gen mutations* (spawn/move/add_hyperlane). All claims about "how the generator works" not directly evidenced by effect signatures, triggers, documented on_actions, or the wiki Map modding page are provisional and must be validated against the local vanilla corpus now present at `Clauser/Paradox/vanilla/common/solar_system_initializers/`, `Clauser/Paradox/vanilla/map/setup_scenarios/`, live runs, and save inspection.  
>  
> **Drift warning (identical to ClauseThing.md):** This is *not* a design doc for a SimThing map generator. It is a reference textbook for implementation agents (Gemini, Opus, Codex, etc.) who will build the SimThing-Clausething / RON analog. The actual target architecture is defined by `SimThing/docs/simthing_core_design.md`, `invariants.md`, `design_0_0_8_1.md`, `adr/mapping_sparse_regioncell.md`, `hydrate_scenario.rs`, and the live CT/BH production ledgers. Implement the *isomorphism*, not the engine model. CPU-side event-driven placement, per-system objects, engine "phases," manual coordinate lists, and exponential perf cliffs at 1000+ stars are exactly what the SimThing substrate (flat columns, recursive SimThing tree, install-time projection + boundary requests, GPU-resident stencils/EML/thresholds) compiles *away from*.  
>  
> **ERRATA / KNOWN GAPS (updated 2026-06-13, post-vanilla corpus copy):**  
> - Full initializer syntax and static_galaxy_scenario details are now directly verifiable against the local vanilla corpus at `Paradox/vanilla/common/solar_system_initializers/*.txt`, `Paradox/vanilla/map/setup_scenarios/*.txt`, and related files (copied 2026-06-13). The descriptions and examples in this document have been reviewed against that primary corpus.  
> - Precise init_effect scoping isolation during gen (frequent community report that blocks are system-local and cannot reliably set external flags or influence other systems early) is only indirectly evidenced; confirmed via testing threads but engine-internal.  
> - Bypass objects (wormholes, gateways, L-gates, astral rifts) are first-class galactic_objects with initializer-driven placement in many cases; exhaustively cross-referenced from scopes/triggers + wiki but not complete.  
> - Scale/perf for huge maps (1000+ stars exponential gen/load costs, late-game cache/war rebuilds, visual layout) and community tooling (outdated external generators, coordinate hacks, manual crafting tedium) are drawn from forum/Reddit/wiki reports and are real modder experience, not engine source.  
> - Graphical map (map/galaxy/*, textures, system icons) and prescripted_countries are out of scope for runtime simulation analog (presentation + empire data only).  
> - Version drift in spawn_system params (e.g., halved default max_distance in 3.5, added direction= param) and hyperlane behavior is noted in Dev Diaries but exact behavior must be re-validated per patch.

**Purpose for the consumer (Gemini/Opus/Codex):** This document, together with the ClauseScript textbook and the existing `scenario` / `location` / `link` grammar in `simthing-clausething`, supplies the detailed Stellaris-side model required to design and implement a faithful "map generator" front-end dialect inside Clausething. The output of that front-end must be the *same* `simthing-spec` surfaces (ScenarioSpec, child AddChild projections, adjacency/link metadata, RegionFieldSpec, property/overlay declarations, commitment effects) that RON authoring and current clause scenario containers already target. `simthing-sim` and the GPU kernels remain ignorant of stars, hyperlanes, or initializers.

> **Community Privilege Note (binding intent for this reference):** Community discussions, pain points, third-party tools, and workarounds (Paradox Forums user threads, r/Stellaris/r/StellarisMods, Stellaris Modding Den Discord, external generators like stellaristools/Retalyx derivatives, coordinate hacks, etc.) are deliberately privileged and cited throughout this document. Official sources (Stellaris Wiki, in-game `script_documentation` logs, vanilla game files, Dev Diaries) provide the baseline grammar and effect surface. Community material captures the *real* authoring experience, limitations, and creative hacks that Stellaris modders actually use. We privilege the community layer because the explicit goal of the SimThing-Clausething map generator analog is to entice the existing Stellaris grand-strategy modding community to embrace SimThings as a more powerful, cleaner, GPU-native substrate for the exact problems they already fight with the Clausewitz engine.

The research emphasizes two realities for any analog:
1. Vanilla modders mostly *gently steer* a hybrid engine via declarative initializers/setup_scenarios + limited imperative surgery and flags, rather than fully scripting the map.
2. Ambitious static/custom work quickly hits tedium, "broken" features (per community), and severe perf cliffs. A clean SimThing-Clausething base generator should target the *declarative steering surface* (initializer-style payloads + explicit locations/links + early projection) while avoiding emulation of the engine's mess.

## Preface for Opus Review: Concept Mappings — Gu-Yang, PALMA, and SimThing-ClauseThing to the MapGen Regime

**Goal of this preface (streamlining review):** This document describes the *Stellaris side* of starmap generation (declarative initializers + setup_scenarios + static_galaxy_scenario for system contents/topology + spawn/add_hyperlane/move effects for influence + galaxy_setup queries + init_effect timing). The SimThing-Clausething analog must emit *exactly the same* `simthing-spec` surfaces already exercised by landed CT work and seated BH work — no new GPU kernels, no WGSL semantic changes, no `simthing-sim` awareness, no full movement engine. 

The mapgen regime (star systems as spatial nodes, hyperlanes as adjacency/topology, initializers as system payloads/children, map-scale fields for nebulas/storms/threat/supply/influence, chokepoints/borders, traversal/gradients over the map, install-time "generation" via projection, pressure-driven decisions) maps cleanly onto **already-implemented or seated** primitives. The Clausething front-end (parser → raw model → hydrate) + `simthing-spec` admission/install does the heavy lifting; the GPU sees only flat columns, stencils, EML trees, thresholds, and boundary overlays.

**Corpus status note (addressing prior caution):** As of the 2026-06-13 copy of the installed Stellaris files into `Clauser/Paradox/vanilla/`, the primary vanilla corpus (`solar_system_initializers/`, `map/setup_scenarios/`, etc.) is now present locally. This satisfies the parser-first, corpus-driven requirement before any grammar widening or authoring work against mapgen idioms. All syntax descriptions below are grounded against that local primary source (in addition to the generated logs). No more reliance on secondary reconstruction for the core declarative constructs.

### Core SimThing-ClauseThing mappings (CT track CLOSED 2026-06-12)
From `SimThing/docs/worklog.md` (CT-3b+4a vertical), `SimThing/crates/simthing-clausething/src/hydrate_scenario.rs` (HydratedScenarioPack + grid_metadata + links + field_operators + palma_feedstock + commitment), `clausething/ct_3b_4a_movement_front_heatmap_memo.md` (RF-fed pressure map), `design_0_0_8_1_clausething_production_track.md`, and `hydrate_*` modules:

- **`scenario` + `location` (with children) + `link` + `HydratedScenarioGridMetadata` (PR2/PR3)**: Primary surface for starmaps. `location` nodes = star systems / sectors / containers (with `kind = Location`); nested `children` = planets/deposits/structures from initializers (properties + intrinsic flows + overlays). `link = { from = A to = B }` + `placements` (row/col) + bounded fanout = hyperlane adjacency / topology (explicit static or lightly steered). `grid_metadata` is authoring/admission feedstock shaped like RegionField pressure placements — not a runtime object. Forbidden full movement terms (route/path/predecessor) remain in force; mapgen stays at topology + fields.
- **`RegionFieldSpec` + `pressure_binding` (ArenaPressureBindingSpec + PressureSourceSpec::Named) + `StructuredFieldStencilOp` (seed-then-zero, horizon-capped) + Layer-2 reduce → `ai_will_do` EML urgency (authored `RegionFieldFormulaBindingSpec.weight_pressure/weight_resource`) → threshold (event_kind 7) → `BoundaryRequest::AttachOverlay` commitment (PR6, `CommitmentEffectSpec`)**: The heartbeat of map "pressure" and decision fronts. RF arena pressure (from system economies, fleets, or authored intrinsics in initializers) seeds map-scale heatmaps (nebula/storm/threat/supply/influence as RegionFields over the location grid). `ai_will_do` weights become map urgency; crossings author structural consequences on the acting location/system (e.g., "fortify chokepoint", "expand here", "cut supply"). ClauseScript `region_field` / `mapping` dialect + `effect { target targets_property amount_add }` inside urgency blocks.
- **`IndexedScatterOp` (GPU on-device projection) + seed-then-zero + EvalEML gated rates / value: trees (CT-RF-EML-RATE-0, CT-2c)**: Install/boundary-time initialization of map fields from scenario-authored values or live RF columns. Gated produces/upkeep conditional on map state (e.g., blockaded systems). `value:` formulas for dynamic map quantities.
- **Scenario admission / hydrate at install (HydratedScenarioPack)**: The "map generator" itself. Declarative ClauseScript (initializers as location payloads, links as topology, field operators as map fields, commitments as map decisions) lowers to the exact tree + metadata + specs that RON scenarios produce. Zero runtime procedural gen; everything before first tick. Matches Stellaris `solar_system_initializers` + `static_galaxy_scenario` + `on_game_start` timing while sidestepping its scoping limits and "define everything or it breaks" fragility.
- **SparseRegionFieldV1 + MappingExecutionProfile (opt-in, Disabled default)**: The admitted execution shape for map fields. `use_accumulator_resource_flow` + pressure binding gates the whole vertical.

All of this is **ClauseThing-blind on the GPU** (already proven in CT-3b+4a session loop + 100-tick observations). `simthing-spec` admission is the guardrail.

### Gu-Yang / C_u / SaturatingFlux / Border Hack (BH track, seated 2026-06-12; BH-3 ClauseThing authoring deferred)
From `SimThing/docs/design_0_0_8_1_border_hack_track.md`, worklog BH entries (BH-0…BH-2S, Gu & Yang 2025 ansatz), `bh*_results.md`, and `hydrate_field_operator.rs`:

- **`RegionFieldOperatorSpec::SaturatingFlux { u_sat, chi }` (BH-0; exact math: σ=clamp(u/u_sat), C product over N/S/E/W diamond, zero-flux boundaries, antisymmetric conservative flux, CFL χ≤0.25/dt)**: Natural for map chokepoints, hyperlane congestion, and system-boundary "walls". State-dependent stencil weights (inexpressible on fixed-weight Normalized/Gradient) produce saturation choking and conservative mass transport — ideal for RF-fed supply/threat/influence propagating (or being cut off) across map links/borders. Zero-flux out-of-bounds semantics match closed galactic map.
- **BH-1 (choke readout column), BH-1R (compact threshold consumer)**: Direct map "choke" or "frontline" columns readable by EML or commitments. Thresholds on map fields trigger decisions without new objects.
- **BH-2 WImpedanceComposeOp + OverlapStressComposition (stress compose)**: Compose map impedance (base_w + weighted chokes from multiple fields: threat + supply + congestion) and stress (overlap, mismatch, velocity, weighted). Feeds higher-level map analysis.
- **BH-3 ClauseThing authoring (deferred but explicitly planned)**: The hook for expressing mapgen-specific border/field behaviors in ClauseScript (e.g., custom choke rules, map pressure sources with saturation, border readouts). Perfect landing place for future mapgen extensions.
- **Overall posture**: Borders/fronts/chokes are **free-ish side effects of the heatmap pass** over admitted topology (no border service, no segmentation, no objects). Gradient descent over the shaped fields remains the decision arena (C_u shapes; it does not decide). Matches Stellaris chokepoint/hyperlane density control and nebula/storm environmental fields while staying semantic-free.

### PALMA + W-impedance / min-plus traversal (seated generic utility; BH-2C/BH-2D coupling)
From `hydrate_palma_feedstock.rs`, `bh2c_palma_w_feedstock.rs`, `palma_path_*_results.md` (especially 4_stellaris_scale_benchmark, 7_gpu_traversal_utility, 8_gpu_native_field_graph), worklog BH-2C/BH-2D:

- **`WImpedanceComposeOp` (base_w + weight_a*choke_a + weight_b*choke_b per profile) → `MinPlusTraversalInput::GpuInterleavedW` → resident D + compact `MinPlusTraversalDProbeOp`**: Map "traversal" and gradient-valley work. Composed W (impedance) from multiple map fields (chokes, threat, supply, stress) feeds min-plus for influence spread, "shortest hyperlane path" equivalents, exploration/AI decision gradients, or valley following across the starmap. Live API `composed_w_min_plus_stencil_config`.
- **PALMA as generic GPU utility (GpuInterleavedW, D probes, traversal)**: Not a full pathfinding engine (forbidden in scenario hydrate until pulled). Seeded by map fields at install/boundary; produces feedstock for EML or commitments. Stellaris-scale benchmarks (palma_path_4_*) and session regionfield work prove it scales to map-sized grids.
- **Coupling to BH (BH-2)**: (1−C_u) shaped fields → W impedance → PALMA for map-front decisions. BH-2S stress as additional map feedstock.

### Cross-cutting invariants and posture (binding)
- **No new GPU primitives**: Mapgen rides `StructuredFieldStencilOp`, `SaturatingFlux` (BH), min-plus (PALMA), `EvalEML`, `IndexedScatterOp`, threshold → `BoundaryRequest`, and existing arena/RF machinery.
- **Install/admission time is the map generator**: Hydrate + projection + seed (scenario containers + RF pressure bindings) replaces Stellaris' opaque procedural phase + timing gotchas + "define everything or it breaks" static rules. Authored ClauseScript (or RON) declares the map; the engine receives a resolved tree + columns + links + fields.
- **Opt-in and guardrails**: `SparseRegionFieldV1` + exactly one region field + pressure binding + ai_will_do + commitment threshold (hard errors otherwise). `MappingExecutionProfile::Disabled` default. Bounded fanout, stowaway budgets, Candidate-F sqrt (no sqrt in hot map paths unless pulled).
- **Spatial tree + reparenting (mobility heritage)**: Systems/locations as parent nodes; entities (fleets, pops) reparent when crossing links — automatically changes local arenas (economy/combat inside systems) without special map code.
- **BH-3 + future mapgen rung**: ClauseThing authoring of map-specific SaturatingFlux / field operators / pressure sources is the natural extension point (deferred until named consumer).

These mappings are **not aspirational** — they are the exact surfaces proven in CT-3b+4a (full vertical on GPU, zero readback, 100-tick observations, authored commitments), BH-0..BH-2S (SaturatingFlux seated with pinned math, W compose + PALMA coupling, stress), and the scenario container hydrate (locations/links/field_operators/palma/commitment all wired). The mapgen front-end in `simthing-clausething` simply widens the parser/hydrate to accept Stellaris-style initializer + topology + map-field idioms and lowers them to the above.

Any proposed mapgen implementation that introduces per-system objects, CPU planners, new kernels, or runtime procedural generation violates the invariants and the CT/BH ledgers.

(References: `design_0_0_8_1_border_hack_track.md`, `clausething/ct_3b_4a_movement_front_heatmap_memo.md`, `hydrate_scenario.rs` + `hydrate_palma_feedstock.rs` + `hydrate_scenario_commitment.rs`, worklog BH/CT entries 2026-06-11/12, `palma_path_4_stellaris_scale_benchmark_results.md` and related, `design_0_0_8_1_clausething_production_track.md`.)

---

## 1. Thesis: The Stellaris Map Is a Hybrid Declarative + Effect Graph That Maps Directly onto SimThing's Spatial Tree + Scenario Model

---

## 1. Thesis: The Stellaris Map Is a Hybrid Declarative + Effect Graph That Maps Directly onto SimThing's Spatial Tree + Scenario Model

Stellaris (and other Clausewitz titles) never give the designer a single "map script." Instead the starmap emerges from:

1. **Engine procedural skeleton** (galaxy size/shape/num_systems/hyperlane_density/arm count/clustering from new-game setup; coordinate assignment; base connectivity graph generation; cluster and partition logic defined in setup scenarios).
2. **Declarative ClauseScript payloads** (`common/solar_system_initializers/`, `map/setup_scenarios/*.txt`, and for true static maps `static_galaxy_scenario` blocks listing explicit `system = { id position initializer spawn_weight }` + `add_hyperlane` + `nebula`).
3. **Imperative placement and topology-mutation effects** (`spawn_system`, `add_hyperlane`/`remove_hyperlane`, `move_system`, batch wrappers, `set_star_class`, `create_nebula`) that are executed from events, on_actions, or special startup paths and that can be conditional on `check_galaxy_setup_value`, distance, ownership, flags, etc.
4. **Post-gen customization** via the same scoping + modifier + on_action machinery used for everything else in the game.

The result is a collection of `galactic_object` (primarily `solar_system`) nodes connected by a hyperlane graph (plus bypass edges), each carrying a tree of planets/moons/deposits/starbase as child objects. Ownership, intel, fog, storms, and later dynamic entities (fleets, stations) are layered on top via the normal scope + effect + modifier system.

**The isomorphism to SimThing is already partially seated (and the research makes the value of the analog clearer):**

- `solar_system` / `galactic_object` (or explicit static `system` entry) ≈ a `location = name { ... }` node in a `scenario` container (or a spatial-tree SimThing under the session root).
- Hyperlane edge (or explicit `add_hyperlane` in static scenario) ≈ `link = { from = X to = Y }` (PR3 bounded grid metadata) + future adjacency/coupling topology.
- Initializer contents (planets, deposits, moons, init_effect) + static system initializer ref ≈ nested `children = { child = planet_or_deposit { properties overlays ... } }`; reusable fragments.
- `spawn_system initializer=... min_jumps=... hyperlane=...` + distance/jump constraints + spawn_weight ≈ install-time or boundary `AddChild` projection of initializer payload; conditional/weighted spawn maps to EML urgency/threshold or authored random-list lowering.
- `add_hyperlane` / `remove_hyperlane` / `move_system` ≈ boundary requests that mutate topology or reparent spatial nodes (re-enrollment into arenas is a core SimThing invariant).
- Galaxy setup values + `galaxy_size`/`galaxy_shape` checks + setup_scenario params (num_stars, cluster_*, num_hyperlanes) ≈ scenario metadata + profile parameters or early EvalEML guards.
- Dense per-system fields (environmental effects, storms, threat, nebulas) ≈ `RegionFieldSpec` + `StructuredFieldStencilOp` + Layer-2 reduce + ai_will_do (already demonstrated in CT-3b+4a).
- The whole pre-game "map gen" phase + static_galaxy_scenario definition ≈ a special install / scenario admission pass that builds the initial spatial tree + links + seeds pressure / intrinsic columns before the first tick.

See `ClauseThing_Spec.md` §4 table row (and the expanded mapping below):

> Stars + hyperlanes | physical **spatial tree** (`design_v6` §2) + adjacency/coupling topology | `design_v6.md` §2 |
> ... |
> `setup_scenario`, `solar_system_initializers` | `ScenarioSpec` + `AddChild` boundary requests / projection | `FirstSliceScenarioSpec` |

The existing `ct_scenario_container*.clause` fixtures and `hydrate_scenario.rs` (PR2/PR3 location+link, PR4 field operators, PR5 palma feedstock, PR6 commitments) are the concrete seed of the map generator analog. MapGenThing's job is to make the *Stellaris idiom* (initializer blocks + spawn effects + hyperlane surgery + setup_scenarios + static system lists) a first-class way to author those same surfaces.

**No new GPU primitives are required for the core topology.** The substrate (recursive SimThing tree, bounded stencil fields over region cells, link metadata for adjacency, install-time child projection) already exists or is under active BH/CT extension. The front-end work is parsing + scope expansion + lowering of initializer-style nested blocks, static system lists, link/spawn statements, and setup_scenario-derived metadata into the admitted spec types, plus any necessary widening of `simthing-spec` (under the same parser-first, corpus-driven discipline as the rest of the CT track).

A key lesson from community sources: ambitious full-static or huge-map work is painful precisely because of engine opacity, timing gotchas, "broken" hyperlane control, manual tedium, and severe perf cliffs at 1000+ stars. The SimThing analog should deliver a *clean, declarative base working map generator* that lets designers achieve similar results (rich system payloads, explicit or steered topology, field overlays) without the engine's mess.

---

## 2. Engine vs Script Division of Labor (Critical Boundary)

### 2.1 What the Engine Owns (ClauseScript only observes or lightly mutates)
- Choice of galaxy size/shape (via setup_scenarios or vanilla `map/setup_scenarios/*.txt`; supports_shape lists, cluster_count/radius, home_system_partitions, open_space_partitions, max_hyperlane_distance, num_nebulas, etc.).
- Base system count, coordinate assignment, and clustering (positions are engine-generated from the scenario params; `move_system` or explicit static `position = { x y }` can relocate).
- Procedural hyperlane generation (density slider + engine algorithm + `num_hyperlanes` range + `random_hyperlanes` flag in static scenarios). Result is a connected undirected graph (or fully manual in static mode).
- Initial "guaranteed" home systems, special-system seeding weights, fallen empire / marauder / gateway / wormhole counts (constrained by hardcoded limits, e.g., max 5 fallen empires).
- Certain crisis / mid-game content placement and most graphical projection (3D map, star sprites, hyperlane lines, nebula clouds, storm sectors) — driven by star/planet class, nebula, storm objects defined in script but rendered by engine.
- Performance envelope: even vanilla "huge" (1000 stars) is marginal in late game; 1000+ stars "can significantly reduce your performance" (wiki); larger modded maps (5k–10k+) exhibit exponential generation times, slow visual layout, cache rebuilds after wars, and overall engine chokepoints.

Observable via triggers and setup queries:
- `galaxy_size = <token>`, `galaxy_shape = <token>`
- `num_galaxy_systems > N`
- `check_galaxy_setup_value = { setting = num_hyperlanes value > 0.8 }` (and ~15 other settings: habitable_worlds_scale, num_gateways, crisis_strength_scale, num_empires, etc.)
- `has_hyperlane_to = <target system>`
- `closest_system = { min_steps=... max_steps=... use_bypasses=... <triggers> }`
- `distance = { type = hyperlane / euclidean same_solar_system=... }`
- `num_active_gateways`, `num_marauder_empires_to_spawn`, `galaxy_percentage`, storm-related, etc.

Effects that read setup:
- `get_galaxy_setup_value = { which = <var> setting = <string> [scale_by = <float>] }`

These values are also used inside `complex_trigger_modifier` for scaling (e.g., menace or other values divided by habitable_worlds_scale).

### 2.2 What ClauseScript Fully Owns (Declarative)
- Content of every star system (initializer payloads or explicit static `system = { id position initializer ... }`).
- The library of reusable "system templates" in `common/solar_system_initializers/`.
- High-level skeleton params via `map/setup_scenarios/*.txt` (`setup_scenario = { name priority num_stars radius num_empires ... cluster_* supports_shape ... }`).
- Fully static & pre-scripted maps via `static_galaxy_scenario = { name priority radius num_* system = { id name position = { x y [z] } initializer spawn_weight = {...} } add_hyperlane = { from to } nebula = {...} random_hyperlanes = yes/no ... }`.
- Definitions of star classes, planet classes, deposits, etc.
- Weights, odds, conditions, and spawn_weight blocks that influence which initializer is chosen (engine consults `usage_odds`, `spawn_weight` modifiers keyed on country flags, etc.).
- Explicit hyperlane wiring and nebula placement in static scenarios.

### 2.3 What ClauseScript Imperatively Controls (Effects + Events)
- Additional system creation: `spawn_system`.
- Topology surgery: `add_hyperlane`, `remove_hyperlane`.
- Relocation: `move_system`.
- Cosmetic/functional classification: `set_star_class`.
- Area effects: `create_nebula`.
- Batching: `set_spawn_system_batch = begin ... end`.
- Post-placement customization (the `effect = { ... }` block inside initializers or subsequent events).

These are usable in the galaxy-generation window (limited) and later. Static scenarios bypass much of the procedural pass by providing an explicit list.

**Important scoping and timing note:** `init_effect` inside an initializer (or static system) runs in the new galactic_object scope at instantiation. Community testing indicates these blocks are frequently system-local and have limited ability to set flags or influence other systems/global state early enough to affect generation decisions. `on_game_start` (and most events) run *after* the full map + home systems + empires are created.

---

## 3. Core Declarative Constructs: solar_system_initializers + setup_scenarios + Static Galaxies

### 3.1 solar_system_initializers (and random_system)
Location: `common/solar_system_initializers/*.txt` (and subdirs).

(High-level shape as previously synthesized; see original sections for planet/moon/deposit/init_effect examples. Key additions from research:)

- `init_effect` is the primary hook for immediate post-creation setup (create_starbase, set flags, prevent_anomaly, generate_empire_home_planet, etc.).
- `spawn_weight` inside or alongside initializer references (especially in static or origin contexts): `spawn_weight = { base = 0 modifier = { add = 100000 has_country_flag = my_country } }`.
- `neighbor_system` blocks (commonly in empire_initializers.txt / custom_initializers.txt, referenced from origins): `neighbor_system = { trigger = { num_guaranteed_colonies >= 2 } hyperlane_jumps = { min = 1 max = @jumps } initializer = "neighbor_t1" }`. This is a declarative way to steer guaranteed colony placement and hyperlane topology near player starts.

The initializer (or static system initializer ref) is a *template*. Instantiation produces a concrete galactic_object that carries the children + runs init_effect.

In the SimThing analog: reusable `ScenarioChildSpec` / `SystemPayload` fragments projected onto `location` nodes at install (see existing `children = { ... }` in scenario fixtures). `init_effect` lowering maps to the commitment / boundary effect machinery (PR6).

### 3.2 map/setup_scenarios (high-level generated skeleton control)
Location: `map/setup_scenarios/*.txt` (copy vanilla `huge.txt` etc. into mod).

Easy declarative way to add new galaxy sizes or tweak params:

```script
setup_scenario = {
    name = "huge"
    priority = 4
    num_stars = 1000
    radius = 450
    num_empires = { min = 0 max = 30 }
    num_empire_default = 15
    fallen_empire_default = 4
    fallen_empire_max = 5   # Hard limit ~5 due to base-game templates
    marauder_empire_default = 3
    advanced_empire_default = 4
    colonizable_planet_odds = 1.0
    primitive_odds = 1.0
    crisis_strength = 1.5
    extra_crisis_strength = { 10 25 }
    cluster_count = { method = one_every_x_empire value = 1 max = 6 }
    cluster_radius = 150
    cluster_distance_from_core = 300
    max_hyperlane_distance = 50
    home_system_partitions = { max_systems = 15 min_systems = 8 min_bridges = 2 max_bridges = 4 method = breadth_first }
    open_space_partitions = { ... }
    num_nebulas = 10
    nebula_size = 60
    nebula_min_dist = 200
    num_wormhole_pairs = { min = 0 max = 5 }
    num_gateways = { min = 0 max = 5 }
    num_hyperlanes = { min = 0.5 max = 3 }
    num_hyperlanes_default = 1
    supports_shape = elliptical
    supports_shape = spiral_2
    # ... more
}
```

Scripts observe the chosen values via `check_galaxy_setup_value` / `get_galaxy_setup_value`. This is the cleanest vanilla lever for "galaxy skeleton" without touching individual systems.

**Performance note (wiki):** "creating too big maps (1000+ stars) can significantly reduce your performance, keep that in mind!"

### 3.3 Fully Static & Pre-Scripted Galaxies (static_galaxy_scenario)
For total control (map packs, total conversions):

```script
static_galaxy_scenario = {
    name = "MyGalaxy"
    priority = 10
    radius = 500   # Max 500 (map is effectively 1000x1000 box)
    num_empires = { min = 10 max = 30 }
    num_empire_default = 10
    fallen_empire_default = 4
    fallen_empire_max = 4   # Cannot exceed ~5 (hardcoded templates)
    # ... other num_* , colonizable_odds, crisis_strength, num_wormhole_pairs, num_gateways ...
    random_hyperlanes = no   # If no, you must supply all via add_hyperlane
    core_radius = 0

    system = {
        id = "0"
        name = ""
        position = { x = 148 y = 121 }
        initializer = my_initializer_1
    }
    system = {
        id = "1"
        name = ""
        position = { x = 148 y = 121 }
        initializer = my_initializer_1
        spawn_weight = { base = 0 modifier = { add = 10 has_country_flag = my_country } }
    }
    system = {
        id = "2"
        name = ""
        position = { x = { min = 150 max = 200 } y = 121 }   # random within range
    }
    system = { id = "3" name = "" position = { x = 148 y = 121 z = 3 } }

    add_hyperlane = { from = "1" to = "2" }
    add_hyperlane = { from = "0" to = "3" }
    # ... more

    nebula = { name = "Random Name" position = { x = 0 y = 0 } radius = 10 }
}
```

**Critical warnings from wiki/community:** "generally if you are making a static galaxy, expect to define close to everything." Partial pre-definition + generator often breaks homeworld climate matching, fallen empires, precursors, origins, events, crises. Climate of initializer must match the home planet the engine would generate. Large static maps still inherit many of the same perf issues.

In the SimThing analog: A full `scenario = { metadata location = { initializer = ... } link = ... }` (or explicit position metadata) can provide the equivalent of a clean static declaration without the "define everything or it breaks" fragility.

---

## 4. Key Effects for Starmap Construction (from effects.log)

(Contents largely unchanged from prior version; batch, spawn_system with full params including hyperlane/is_discovered, add/remove_hyperlane, move_system with direction-related evolution in patches, set_star_class, create_nebula, etc. All signatures from local 2026-05-29 logs + corroborating wiki.)

**Important scoping note:** Many placement effects usable from no_scope/country at startup. `spawn_system` callable from inside initializer `init_effect`. `set_spawn_system_batch` exists specifically because spawning hundreds of full systems (planets + effects) is expensive.

---

## 5. Query / Conditional Surface (from triggers.log and scopes.log)

(Unchanged core list; add that these are heavily used inside `complex_trigger_modifier` and in origin/neighbor logic.)

---

## 6. Timing, On Actions, Execution Order, and the Galaxy Generation Window (Expanded)

Precise order (synthesized from wiki Game start modding + forum testing):

1. Galaxy created using `common/solar_system_initializers` + `map/setup_scenarios` (or static_galaxy_scenario).
2. Home planet / empire generation steps (generate_empire_home_planet, etc.).
3. `on_game_start` (and many flavor/DLC events).
4. Later post-start events.

`init_effect` inside initializers/static systems runs at the moment of instantiation (system-local scope in practice per community tests). Many effects explicitly note they will not run (or have limited effect) if executed during galaxy generation.

For custom map mods the pattern is often:
- Large library of initializers.
- Early hooks or origin-tied events (or static scenario) to force topology.
- Batch spawn + immediate add_hyperlane surgery + flags.
- Heavy reliance on spawn_weight + country flags + neighbor_system for "steered" rather than fully manual placement.

`on_game_start` is safe for post-gen cleanup but too late for anything that must influence the generator itself.

---

## 7. Custom Map Authoring Patterns, Static Maps, and Mod Community Practice (Expanded)

- **setup_scenarios for sizes/shapes**: Easiest declarative extension.
- **Map packs / total conversions**: Fixed named initializers + master spawn or full `static_galaxy_scenario` with explicit systems + hyperlanes. Often requires defining *almost everything* to avoid generator breakage.
- **"More interesting galaxy" and origin mods**: New weighted initializers, spawn_weight + flags, neighbor_system tweaks, tight-distance spawn_system from player start.
- **Dynamic maps**: Mid-game spawn_system reuse.
- **Bypass objects**: Frequently placed/activated inside dedicated initializers rather than pure effects.
- **Performance considerations**: `set_spawn_system_batch` helps, but real cost is in the number of systems + planet trees + late-game simulation.

**Scale and performance complaints for huge maps (community + wiki):**
- Vanilla "huge" (1000 stars) is already marginal in late game (cache rebuilds after wars, overall slowdown).
- "creating too big maps (1000+ stars) can significantly reduce your performance" (official wiki guidance).
- Modded colossal maps (5k stars): ~40 minutes generation on a laptop; 6k stars: ~2 hours. Exponential growth in time and memory.
- Even larger experiments (10k+ stars) hit engine limitations hard (visual layout generation, loading, runtime simulation).
- Forum consensus: the 1000-system limit exists for performance reasons; the game "doesn't cope gracefully" beyond vanilla max. Late-game chokepoints (pops, fleets, wars) are exacerbated.
- Static or heavily custom large maps inherit (or worsen) these costs because the engine still has to hydrate the full set of systems/planets.

**Official sources (baseline mechanics and guidance):**
- Stellaris Wiki "Map modding" (https://stellaris.paradoxwikis.com/Map_modding): Authoritative coverage of `map/setup_scenarios/*`, `static_galaxy_scenario`, directories, and explicit performance warning for 1000+ star maps.
- In-game generated `script_documentation` logs + vanilla `common/solar_system_initializers/`, `map/setup_scenarios/`, and Dev Diaries (Paradox forums).
- These define the language surface and what the engine promises.

**Community discussions, concerns, and tooling (privileged — see top note):**
These capture the *lived experience* of Stellaris modders and are cited to make the SimThing analog feel like a direct solution to problems the community already knows.
- [Galaxy building tool thread — Paradox Forums (2019)](https://forum.paradoxplaza.com/forum/threads/galaxy-building-tool.1174736/): Core reference for pain points. Direct quote: "custom hyperlanes are broken — many bigger mods are waiting for a fix from PDX. There is a workaround but it requires more knowledge and is very tedious since you basically have to craft each system manually, and keep in mind that galaxies in stellaris can contain up to 1000 systems." Recommends studying Star Trek: New Horizons mod's `map/scenarios` folder + `solar_system_initializers`. Describes using coordinate systems (e.g. geogebra) with mirroring because "the galaxy will be upside down".
- stellaristools.com (third-party community System Builder): http://stellaristools.com/ — speeds initializer creation; frequently called out as useful but outdated (no full binary/trinary star support in older reports).
- Retalyx Static Galaxy Generator and derivatives (community tool for static/pre-scripted maps): Original GitHub https://github.com/Retalyx/Stellaris-Static-Galaxy-Generator ; updated Nexus reconstruction https://www.nexusmods.com/stellaris/mods/67 ("Static Galaxy Editor aka Retalyx Galaxy Generator"). Used for PNG-based or hand-crafted static galaxies with explicit hyperlane editing. 2016-era Reddit/Forum discussions (e.g. https://www.reddit.com/r/Stellaris/comments/4keaem/modding_tool_retalyx_static_galaxy_generator_with/ ) show it was a go-to before falling out of maintenance.
- Modern community map tools (Reddit r/Stellaris / r/StellarisMods): "Paint a Galaxy" web app + mod for painting custom stars/hyperlanes (https://www.reddit.com/r/Stellaris/comments/1sblcmn/make_your_own_custom_maps_and_scenarios/ ); various standalone editors for positions and hyperlane connections (e.g. https://www.reddit.com/r/Stellaris/comments/wkc1fo/i_made_a_galaxy_map_editor_that_edits_star/ ); StellarMaps for visualization from saves.
- Live collaboration and tooling development: **Stellaris Modding Den** (unofficial central Discord for Stellaris modding, CWTools development, and map discussions) — https://discord.gg/bHVez2C (permanent invite; referenced in the official Wiki Modding page and multiple forum posts as the place for real-time help on initializers, static scenarios, and hyperlane issues).
- CWTools (community-driven VSCode language server / validator for Paradox scripting, including solar_system_initializers support) — frequently discussed in the Modding Den and wiki as essential for catching syntax issues in complex initializer and static_galaxy_scenario work.

These community artifacts (tools born of necessity + public venting of engine limitations) are privileged here precisely to entice the Stellaris modding community: by showing we understand the exact tedium of manual hyperlane surgery, the fragility of static maps, the pain of 1000+ star generation times, and the beloved (if aging) external generators they already rely on, the Clausething mapgen front-end can present itself as the cleaner, more powerful alternative that lets them keep their idioms while escaping the Clausewitz engine's constraints.

These realities are exactly why a clean Clausething mapgen analog (declarative initializer-style payloads + explicit or steered locations/links at install time, with field/commitment overlays) can be a significant quality-of-life improvement for designers targeting similar grand-strategy spatial experiences.

Duplication/override and load-order rules apply as elsewhere in ClauseScript.

---

## 8. Direct Mapping Table — Stellaris Map Primitives → SimThing / Clausething Surfaces (Expanded)

(Extends prior table with new rows for setup_scenarios, static system lists, spawn_weight, neighbor_system, and explicit notes on scale/perf.)

| Stellaris ClauseScript Concept                  | SimThing / Clausething Analog (current + near-term)                          | Notes / Lowering Site |
|-------------------------------------------------|-----------------------------------------------------------------------------|-----------------------|
| `solar_system` / `galactic_object` (node)       | `location = name { ... }` inside `scenario`                                 | hydrate_scenario.rs (PR2/PR3); spatial tree |
| Hyperlane edge / explicit add_hyperlane         | `link = { from = A to = B }`; future adjacency/coupling                     | PR3 links; mapping ADR |
| `solar_system_initializer` + static `system = { position initializer spawn_weight }` | Nested `children` + reusable payload fragments; optional position metadata | ct_scenario fixtures; hydrate_scenario |
| `planet = { class size deposit moon=... }` + deposit | Child SimThing + intrinsic resource flows                                   | CT-2 economy |
| `spawn_system ...` + distance/jump + spawn_weight | Install-time/boundary `AddChild` + EML-weighted projection                  | FirstSliceScenarioSpec + commitments |
| `add_hyperlane` / `remove_hyperlane` / `move_system` | BoundaryRequest mutating links / reparenting                                | PR3+ link handling; re-enrollment |
| `set_star_class`, planet class                  | Property/kind tag + modifiers                                               | Property registry (CT-2c) |
| `create_nebula { radius effect }`               | RegionField / multi-cell stencil + field operator / commitment              | PR4/ BH-2S / PALMA |
| `check_galaxy_setup_value` / setup_scenario params (num_stars, cluster_*, num_hyperlanes) | Scenario metadata + early guards / profile params                           | CT-1c/2a + EvalEML |
| `init_effect { ... }`                           | Authored commitment / init overlays                                         | PR6 |
| `on_game_start` + startup events                | Cadence tiers + one-time install threshold                                  | Mapping ADR |
| `map/setup_scenarios` (size/shape skeleton)     | Scenario-level metadata or top-level profile                                | Admission layer |
| `static_galaxy_scenario` explicit systems + hyperlanes | Full `scenario` with explicit locations + links + payloads                  | Future widening of scenario container |
| Home system / capital                           | Root faction child + special location flag                                  | Owner-targeted overlays |
| Bypass objects                                  | Special link type or dedicated child node                                   | Link grammar extension |
| Huge-map scale/perf realities (1000+ stars exponential costs) | Explicit admission guardrails + bounded fields; avoid per-system objects    | simthing-spec + invariants (core design) |

**Honest constraints (unchanged + reinforced):**
- hydrate_scenario.rs forbids many movement/pathfinding terms until pulled by a named consumer.
- PR3 links currently bounded (fanout ≤4); arbitrary graphs require widening.
- No dense long-horizon lateral diffusion for awareness.
- All at designer/spec admission; GPU sees only the resulting tree + columns + stencils + AccumulatorOps.
- The analog should target *base working vanilla-steered generation*, not full manual static 10k-star maps (those hit real engine cliffs that SimThing's flat substrate is designed to avoid).

---

## 9. Implications for Building the SimThing-Clausething Map Generator Analog (and RON Target) — Updated

The goal is **not** to re-implement the Clausewitz galaxy generator (with its procedural core, timing quirks, "define everything or it breaks" static rules, manual tedium, and 1000+ star perf cliffs). It is to let a designer write in a natural Stellaris-like surface that lowers to the same clean surfaces SimThing already uses:

```clause
scenario = my_ron_galaxy_01 {
    metadata = { display_name = "The Shattered Arm" num_stars = 800 ... }
    system_initializer = shattered_core { ... nested planets deposits ... }
    system_initializer = fringe_rich { ... }
    location = core_a { initializer = shattered_core ... }
    location = fringe_3 { initializer = fringe_rich ... }
    link = { from = core_a to = fringe_3 weight = 0.7 chokepoint = yes }
    # conditional via existing mechanisms (value:, ai_will_do-style pressure, EML)
}
```

Or, for closer static analog:
```clause
scenario = my_static_galaxy {
    metadata = { ... radius = 450 ... }
    location = sys0 { initializer = my_init_1 position = { x = 148 y = 121 } }
    ...
    link = { from = "sys0" to = "sys1" }
}
```

Hydration emits the identical `ScenarioSpec` + children + link metadata + fields that a hand-written RON scenario would produce. The "spawn" / "static list" semantics become install-time / first-boundary child projection + link registration. Galaxy setup conditionals become early guards or metadata. `init_effect` payloads become commitments or init overlays. Field overlays (nebula/storm analogs) ride the already-proven RegionField + stencil + EML path.

**Sequencing (mirror CT track):** Parser extensions for `system_initializer`, `location` with initializer/position, `link` with attributes, static-style system lists, spawn_weight/neighbor_system sugar (CT-0/T1). Lower to (widened) ScenarioSpec + SpatialLinkSpec + existing property/overlay/commitment/RegionField surfaces. Use proven scenario container + field-operator + commitment vertical (PR4–PR6). Conditional/weighted placement lowers to EML urgency + threshold + commitment (CT-3b+4a). No new GPU primitives.

**Drift to avoid:** No per-system map objects, no CPU placement planner, no emulation of engine phases or manual coordinate lists. The spatial tree *is* the map. Adjacency is metadata or explicit links. Dense fields are RegionCell stencils. Performance cliffs at huge scale are avoided by the flat column + bounded stencil model (core design + invariants).

Including scale/perf and tooling realities in this reference helps implementers design *defensively*: provide a clean declarative base that delivers 80–90% of what modders want from initializers + setup_scenarios + limited surgery, while the engine-side perf/tedium problems simply do not exist on the SimThing side.

---

## 10. Worked Mini-Examples (Stellaris Idiom → Clausething Idiom)

(Updated examples incorporating spawn_weight, neighbor_system flavor, static system style, and setup_scenario awareness.)

**Stellaris-style (initializer + spawn_weight + neighbor + post-gen surgery):**

```script
# initializer with init_effect and spawn_weight usage in context
initializer = { ... init_effect = { create_starbase = ... } }

# In origin or startup
neighbor_system = { ... initializer = "neighbor_t1" ... }

spawn_system = { initializer = "Fringe Rich" min_jumps = 3 max_jumps = 5 hyperlane = yes }
add_hyperlane = { from = event_target:home to = event_target:new_system }
```

**Clausething / SimThing scenario analog (base working generator):**

```clause
scenario = ron_fringe_map {
    metadata = { display_name = "Fringe" num_stars = 600 ... }
    system_initializer = fringe_rich { child = ... produces = { minerals = 4 ... } ... }
    location = fringe_01 {
        name = "Fringe Rich 01"
        initializer = fringe_rich
        # optional position metadata for static-style
    }
    link = { from = player_home to = fringe_01 }
    # spawn_weight / neighbor analogs via authored weights or early EML guards on the location
}
```

The hydrator produces bit-identical (or RON-roundtrippable) spec artifacts. The engine's timing, scoping, and perf problems are compiled away at the admission layer.

---

## 11. Primary Sources, Refresh Procedure, and Tooling

**Lab-local primary corpus (now available post-copy):**  
- Vanilla game files: `Clauser/Paradox/vanilla/common/solar_system_initializers/`, `Clauser/Paradox/vanilla/map/setup_scenarios/`, `Clauser/Paradox/vanilla/map/galaxy/`, `Clauser/Paradox/vanilla/prescripted_countries/`.  
  These are direct copies of the installed Stellaris content and are the authoritative source for initializer syntax, `static_galaxy_scenario`, `setup_scenario` blocks, etc. Use these for all corpus-driven parser work, golden-file tests, and grammar development.

**Refresh procedure for logs (effects, triggers, scopes, etc.):**  
- Launch Stellaris (any new game or load is sufficient).  
- Copy the freshly generated files from `Documents\Paradox Interactive\Stellaris\logs\script_documentation\` into `Clauser/Paradox/script_documentation/` (overwriting as needed). The existing copy there is from 2026-05-29.

**Other references:**  
- Dynamic format & meta-scripting: `Clauser/Paradox/02_Dynamic...md`.  
- Map-specific wiki: Stellaris Wiki "Map modding" page (setup_scenarios syntax, static_galaxy_scenario, performance notes, directories).  
- Game start order: Wiki "Game start modding".  
- Community: Paradox forums (galaxy-building-tool, static-galaxies, custom-galaxies, etc.), r/StellarisMods, large overhaul mods (e.g., STNH examples).  
- jomini / CWTools: Parser and validation support for initializers (useful for round-trip testing of the front-end).  
- SimThing companions: `hydrate_scenario.rs` + fixtures, `simthing-spec`, mapping/resource-flow ADRs, `design_0_0_8_1_clausething_production_track.md`, core design, invariants.

**Testing discipline:** Bit-exact round-trip (ClauseScript map doc → raw model → emitted RON spec → installed tree). CPU-oracle parity on any seeded fields/pressures/commitments. Golden RON per fixture. Explicit admission guards for dangerous scale or forbidden movement terms. All semantic and syntactic claims must now be verifiable against the local vanilla corpus in `Clauser/Paradox/vanilla/`.

---

**End of revised MapGenThing reference.**  
This file lives in `Clauser/` (lab / secondary-source side with expanded community context). Canonical engineering use belongs in `SimThing/docs/clausething/`, `simthing-clausething/src/`, `simthing-spec`, the mapping/scenario ADRs, and future production-track rungs once a named consumer pulls the map-gen vertical (optimizing for a clean base working generator using vanilla-style declarative steering).

Cross-link from `ClauseThing_Spec.md` and the CT track ledger when the map-gen work is authorized. The inclusion of scale/perf complaints and community tooling provides necessary context so that the analog can be designed to deliver the useful parts of the Stellaris idiom while avoiding the engine's well-known cliffs and tedium. 

(End of document.)