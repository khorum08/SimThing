# MapGeneratorCLI — High-Level Galaxy Shape Generator for SimThing MapGen

> **⚠ TRACK CLOSED & CONSOLIDATED (2026-06-15).** The 0.0.8.6 MapGeneratorCLI track is closed (PR1–PR12).
> **Entry point is now [`ClauseThingDoc.md`](ClauseThingDoc.md)** (concepts/practices/APIs) +
> [`../adr/ClauseThingADR.md`](../adr/ClauseThingADR.md) (decisions). This file is the producer *reference*;
> the ladder is archived at [`../archive/closed_production/design_0_0_8_6_mapgenerator_cli_ladder.md`](../archive/closed_production/design_0_0_8_6_mapgenerator_cli_ladder.md).

> **IN-REPO REFERENCE (promoted 2026-06-14, executive design authority).** Target-direction document for
> the **0.0.8.6 MapGeneratorCLI** track — the declarative *producer* layer above the closed 0.0.8.2.5
> MapGen ingest/lowering path. **Binding plan + adjudications live in
> [`../archive/closed_production/design_0_0_8_6_mapgenerator_cli_ladder.md`](../archive/closed_production/design_0_0_8_6_mapgenerator_cli_ladder.md); where
> this reference and that ladder disagree, the ADR + clearinghouse govern.** This is a *proposal/reference*, not an
> authorization to widen the lowering layer, `simthing-spec`, or the runtime. The CLI produces declarative
> payloads only and honors every 0.0.8.2.5 adjudication (M1–M11) + the §0 hard prohibitions in the ladder.
> This track is **not** FIELD-MOVIE-DATASET-0 (which remains later/subsequent) and does **not** reopen
> 0.0.8.2.5.
>
> **Status: TRACK CLOSED — DA-APPROVED (2026-06-15, #693). PR1–PR11 DA-APPROVED & MERGED; PR12 docs-only
> closeout — DA-APPROVED & MERGED (#693). Next track: FIELD-MOVIE-DATASET-0 unless DA reorders.**

> **Relationship to existing work (binding):**  
> - `clausething/MapGenThing.md` / `Clauser/mapgenthing.md` is the **target grammar reference** (what declarative payloads look like and how they map to SimThing surfaces). It deliberately contains **no generation algorithms**.  
> - `design_0_0_8_2_5_mapgen_ladder.md` is the **lowering / ingest layer** (neutral AST or direct ClauseScript scenario authoring → `ScenarioSpec` + `Location` gridcells + links + RegionFieldSpec + RF enrollment + Movement-Front + PALMA feedstock + commitments). It is a thin front-end only.  
> - **MapGeneratorCLI** is the **missing "Galaxy Shape Generator"** that sits on top: it turns UI levers ("4-armed spiral, 1000 stars, medium tightness, 6 clusters") into the concrete declarative structures the MapGen ladder then ingests and lowers.

> This CLI is the piece that actually lets you build the UI you described. Once it exists, a UI can drive it; the output is guaranteed to be something the existing MapGen path can turn into a working SimThing star-mapping scenario.

---

## 1. Problem Statement

You want a map generator with this contract:

```
mapgen --shape=4_armed_spiral --num_stars=1000 --num_arms=4 --arm_tightness=medium --cluster_count=6 --hyperlane_density=1.0 --seed=42 --output=galaxy.clause
```

And get back a complete, reproducible, ClauseScript-described starmap that:

- Places ~1000 systems in a 4-armed spiral with plausible arm curvature and inter-arm gaps.
- Distributes clusters (cores, fringes, special systems).
- Wires a sensible hyperlane graph (dense within arms, sparser cross-arm, choke points possible).
- Includes varied system initializers (rich cores vs sparse fringes).
- Optionally seeds some nebula / field operators for map-scale environmental effects.
- Is directly consumable by the MapGen front-end (or by `simthing-clausething` + the scenario container path) and lowers cleanly onto the already-closed surfaces (gridcell `Location` SimThings, bounded links, RF arenas with a suppression front, Gu-Yang Movement-Front heatmap, PALMA reach, commitments, etc.).

The current `mapgenthing.md` explains the *what* (the Stellaris idioms and their SimThing isomorphs). It does not contain the *how* (the procedural rules that turn high-level shape selections into concrete system placements and topology). That "how" is what this CLI supplies.

---

## 2. Scope and Non-Goals

**In scope (v1):**
- High-level parameter set modeled on Stellaris galaxy setup + common community extensions.
- Procedural (but deterministic) generation of declarative output structures.
- Output in a form the MapGen ladder can ingest. **Proven current path (PR1–PR11 producer scale):** closed `static_galaxy_scenario` neutral-AST text (`system`, `add_hyperlane`, `nebula`, bareword initializer refs) consumed by `parse_mapgen_neutral_document` and the closed MapGen lowerers — **not** scenario-container `hydrate_scenario scenario { location ... }` blocks. The ladder governs where documents disagree; legacy `scenario { location ... }` sketches below remain aspirational UI targets only.
- Reproducible via seed.
- Plausible but not physically perfect spirals / clusters / hyperlane graphs.
- Basic support for the most common shapes (spiral_2/3/4/6, elliptical, ring, bar, starburst, cartwheel, spoked, and static/arbitrary_static override modes). **PR8 (2026-06-14):** all vanilla shapes are registered in a single-source `ShapeStrategyEntry` map (`strategies/registry.rs`); executable names derive from registry entries — no parallel match ladder.
- **PR9 (2026-06-14, #689):** bounded producer-side nebula placement emits closed `static_galaxy_scenario` `nebula = { name radius }` feedstock only; initializer bucket bareword refs with sibling definitions emitted once; inert metadata (`num_empires`, crisis levers, etc.) captured in dry-run reports until a closed surface admits it. Generated output parses and lowers through existing MapGen lattice + Movement-Front `RegionFieldSpec` surfaces — no GPU/runtime execution, no new grammar, no closed `src/` edits. **DA-APPROVED & MERGED.**
- **PR10 (2026-06-15, #690):** MapGeneratorCLI-generated `static_galaxy_scenario` text admits/installs through the existing path and produces GPU-resident compact evidence on a real adapter via `mapgenerator_cli_pr10_gpu_compact_evidence`. Compact readback only; no new kernels or closed lowerer edits. **DA-APPROVED & MERGED.**
- **PR11 (2026-06-15, #692):** 1000-star elliptical producer scale envelope + DA heap remediation. Parse/lattice proven at 1000 stars; RF/admit/install/GPU at galaxy scale honestly deferred under closed lowerer caps. **DA-APPROVED & MERGED.**
- Emission of at least one suppression / environmental field operator (so the output exercises the RF-pressure → RegionField → Gu-Yang → PALMA path).

**Explicitly out of scope (defer to later tracks or the lowering layer):**
- Full Stellaris fidelity (prescripted countries, graphical assets, triggers/effects that run during gen, load-order, etc.).
- Runtime simulation behavior (that's the Movement-Front automaton + RF arenas + commitments).
- Deep hierarchical RF allocation beyond what the closed CT-2c surface already supports.
- Atlas, active masks, perception, or any Provisional features.
- Arbitrary high-degree topology or non-grid graphs (the ladder forbids this).
- The actual lowering to `simthing-spec` (handled by the 0.0.8.2.5 MapGen front-end).
- Editor / corpus / export (the subsequent track after closeout).

---

## 3. CLI Interface (proposed)

```bash
mapgen \
  --shape=4_armed_spiral \          # or 2_armed_spiral, elliptical, ring, irregular, static
  --num_stars=1000 \
  --num_arms=4 \                    # only for spiral shapes
  --arm_tightness=medium \          # tight | medium | loose
  --cluster_count=6 \
  --cluster_radius=80 \
  --hyperlane_density=1.0 \         # 0.5–2.0 typical
  --habitable_scale=1.0 \
  --core_radius=120 \
  --seed=42 \
  --output_format=clause \          # clause | static_galaxy | manifest
  --output=galaxy_4arm_1000.clause
```

Additional useful levers (Stellaris-compatible + SimThing extensions):
- `--lattice_size=240` (or auto-derived from num_stars; always square per core §7)
- `--arm_width=18`
- `--inter_arm_gap=45`
- `--fringe_density=0.6` (fewer systems far from arms/cores)
- `--special_systems=precursor,leviathan` (weights or explicit counts)
- `--field_operators=nebula,storm` (emit sample RegionField operators)
- `--min_distance=8` (enforce minimum cell separation)
- `--output_dir=.` (for emitting a small initializer library alongside the main scenario)

The tool should also support a "parameters file" mode for repeatability and UI integration:

```bash
mapgen --params=ui_selections.json --seed=42
```

---

## 4. High-Level Generation Algorithm (the missing piece)

The CLI implements a **declarative-first procedural generator**. It never tries to simulate the runtime automaton. It only decides *where the systems go* and *what declarative payload each location carries*, then emits the structures the MapGen front-end expects.

### 4.1 Lattice & Coordinate System
> **STEAD contract (mandatory).** The emitted integer `(col,row)` are **structural gridcell coordinates** the
> closed lowerer honors as authoritative layout — never render metadata, emission order, or row-major fill. Read
> [`../stead_spatial_contract.md`](../stead_spatial_contract.md) before changing any placement, coordinate, or
> Movement-Front/PALMA behavior; it is enforced by `stead_spatial_contract_guards.rs`.

- **Structural gridcell layout has NO fixed edge cap** (STEAD-SCALE-1): it scales by explicit admission budgets + memory (`MapgenStructuralGridBudget`, checked-`u128`), not a magic constant. **200×200 is a *small* reference, not a canonical upper bound**; `65,535` was a temporary arithmetic ceiling and is **not doctrine**. Execution profiles may impose bounded-theater limits — a vast layout may be admitted even when a particular dense execution profile defers to atlas scheduling.
- Allocate a sparse square 2D grid of `Location` cells (sized to the authored positions; scale up squarely as needed).
- Every system occupies exactly one grid cell (one-system-per-cell invariant from the ladder).
- Positions are integer (col, row). The generator works in this discrete lattice; any floating-point math is only for arm curves and is immediately quantized.

### 4.2 Shape-Specific Placement Rules

**4-armed spiral (and 2-armed variant):**
- Parametric arms: `theta = a + b * r` (Archimedean or logarithmic spiral).
- For each arm, sample points along the curve with density decreasing with radius.
- Add perpendicular jitter within `arm_width`.
- Place "core" systems near center (denser, richer initializers).
- Place "fringe" systems with lower density.
- Distribute `cluster_count` local over-densities (small Gaussian blobs) along arms and in inter-arm gaps.
- Quantize all candidate positions to free lattice cells, enforcing `--min_distance`.

**Elliptical / ring:**
- Sample from an elliptical or annular distribution.
- Add multi-scale clustering (a few large clusters + many small ones).
- Ring shapes get a central void + a dense ring band.

**Irregular / clustered:**
- Generate `cluster_count` centers (some near core, some scattered).
- For each center, sample a local cloud with configurable radius and density falloff.
- Add a light background sprinkle for "deep space" systems.

**Static override mode:**
- Accept an explicit list of (id, x, y, initializer) or a minimal static_galaxy_scenario fragment. Useful for hand-crafted maps or loading previous outputs.

### 4.3 Hyperlane Topology Rules (declarative, not runtime paths)
> **STEAD (MAPGENCLI-TOPOLOGY-STEAD-0 / -1).** **All** producer-side adjacency — base hyperlanes
> (`topology.rs`), long-range special routes / wormholes / gateways (`special_routes.rs`), partition bridges
> and BFS/DFS partition ordering (`partition.rs`), and cluster bridges (`cluster.rs`) — is selected from
> **authored structural gridcell coordinates** (`PlacedSystemSeed.coord`), **not** emission order or lowerer
> fixture placement. Generated `add_hyperlane` pairs remain declarative endpoint pairs and lower through the
> closed structural grid path. This is **producer-side topology only — no pathfinding service or runtime
> route semantics**, and no Euclidean sqrt (adjacency is integer Chebyshev over the authored `(col,row)`
> layout). Enforced by `crates/simthing-mapgenerator/tests/topology_stead.rs` (which also guards that the
> 1500-star spiral placement is a **dispersed sparse layout, never a row-major brick**).

- Within each arm (or ring band): high local connectivity (dense short hyperlanes).
- Between adjacent arms: sparser "bridge" hyperlanes at regular intervals (creates natural chokepoints).
- Long-range "lane coupling" edges for strategic connectivity (these become the bounded long-range couplings the ladder describes on top of the geometric lattice).
- The generator can bias bridges toward cluster centers or leave some arms more isolated for gameplay variety.
- Output is a set of explicit `link { from=... to=... }` (or `add_hyperlane` in static form). Density and "choke" quality are controlled by the shape parameters.

### 4.4 System Content / Initializer Assignment
- Core vs fringe vs cluster vs arm vs inter-arm buckets.
- Assign different initializer families (or weights) per bucket (rich mineral cores, habitable fringes, special precursor/leviathan systems in designated clusters).
- The CLI does **not** need to emit full initializer definitions if they already exist in a library; it can reference them by key and emit the minimal `system_initializer` or location-level overrides.
- Optional: emit a few nebula / environmental field operators placed over dense or empty regions.

### 4.5 Reproducibility & Variation
- All random choices are driven by a single `--seed`.
- A secondary "variation seed" can be used for initializer content while keeping the geometric layout identical.
- The output should be stable: same params + seed = byte-identical (or semantically identical) declarative description.

---

## 5. Output Structures (what the CLI actually emits)

The CLI's job is to produce **declarative payload**, not runtime objects.

**Proven ingest path (PR1–PR11):** emit a self-contained `static_galaxy_scenario { ... }` block with bareword
initializer refs, sibling initializer definitions, `add_hyperlane` topology, and closed `nebula = { name radius }`
feedstock. The closed MapGen neutral-AST parser and lowerers consume this directly.

**Aspirational / legacy sketch** (scenario-container `hydrate_scenario` form — not the proven MapGeneratorCLI
output contract today):

```clause
scenario = ui_4armed_1000 {
    metadata = {
        display_name = "4-Armed Spiral (1000 stars)"
        shape = "4_armed_spiral"
        num_stars = 1000
        seed = 42
        generated_by = "MapGeneratorCLI v0.1"
    }

    # High-level skeleton (maps to setup_scenario + lattice size)
    lattice = { width = 240 height = 240 }

    # Explicit systems (maps to static_galaxy_scenario systems or location nodes)
    location = arm1_core_001 {
        initializer = "core_rich"
        position = { x = 112 y = 118 }   # quantized grid cell
    }
    ...

    # Hyperlane topology (maps to links + lane coupling)
    link = { from = "arm1_core_001" to = "arm1_core_002" }
    link = { from = "arm1_core_005" to = "arm2_bridge_003" weight = 0.4 }  # sparser cross-arm

    # Optional map-scale fields (maps to RegionFieldSpec + operators)
    field_operator = {
        type = "nebula"
        center = { x = 80 y = 95 }
        radius = 35
    }

    # Initializer library references or inline payloads (maps to system_initializer)
    system_initializer = "core_rich" { ... }
    ...
}
```

Alternative / fallback output formats (for compatibility with pure Stellaris-style ingestion or older tools):
- A self-contained `static_galaxy_scenario { ... }` block + referenced initializers.
- A manifest + small set of `.txt` initializers (the style community tools like stellaristools / Retalyx derivatives emit).

The MapGen front-end (per the ladder) is responsible for turning whichever format is chosen into the final `ScenarioSpec` + tree + fields + links.

---

## 6. How This Enables the UI You Want

Once MapGeneratorCLI exists:

1. UI presents levers (shape dropdown, num_stars slider, arm_count, tightness radio, cluster sliders, density, seed, etc.).
2. UI calls the CLI (or links to the library) with those parameters.
3. CLI emits `galaxy.clause` (or equivalent).
4. The MapGen front-end (or direct `simthing-clausething` usage) ingests it, performs the lowering described in the 0.0.8.2.5 ladder, and you get a loadable SimThing scenario with a real Movement-Front galaxy lattice, RF pressure feeding the front, Gu-Yang chokes, PALMA reach, commitments from map urgency, etc.

You can iterate the generation rules (better arm math, more interesting clustering heuristics, shape-specific field placement) without touching the lowering layer or the runtime. The declarative output remains the stable contract.

---

## 7. Implementation Notes & Guardrails

- Keep the generator **thin and declarative**. It should not know about `AccumulatorOp`, `EvalEML` trees, or `BoundaryRequest`. It only knows how to place systems and write the structures the mapgenthing / ladder documents.
- All geometry is quantized to the discrete grid early. Floating-point is only for curve sampling and jitter.
- Enforce the invariants from the ladder (one system per cell, bounded fanout on explicit links, no Euclidean consumers, no new `SimThingKind`, Candidate-F tripwire respected).
- Support a "dry-run / manifest only" mode that emits human-readable placement reports (useful for UI preview).
- The CLI can be a small Rust binary (or Python for rapid iteration) that depends only on the parameter model and a tiny emitter. It does **not** need to link against `simthing-*` crates.
- Output should be round-trippable through the neutral AST path where possible.

---

## 8. Example End-to-End Flow

```bash
# User (or UI) invokes
mapgen --shape=4_armed_spiral --num_stars=1000 --num_arms=4 \
       --arm_tightness=medium --cluster_count=6 --seed=424242 \
       --output=spiral_4arm_1000.clause

# The .clause is then fed to the MapGen front-end
cargo run -p simthing-clausething -- generate \
    --input=spiral_4arm_1000.clause \
    --output=scenario_4arm_1000.ron

# Or directly via the driver
cargo run -p simthing-driver -- record \
    --scenario scenario_4arm_1000.ron --out demo.replay.ldjson
```

The resulting scenario exercises the full vertical the closeout and MapGen ladder already proved: RF pressure from the authored economy → region-field seeds → Gu-Yang Movement-Front on the lattice → ai_will_do → commitments, with PALMA reach available, all under the existing guardrails.

---

## 9. References & Integration Points

- `Clauser/mapgenthing.md` (and `docs/clausething/MapGenThing.md`) — the grammar and SimThing isomorphisms.
- `design_0_0_8_2_5_mapgen_ladder.md` — the ingest/lowering contract, M1–M11 schema pins, PR ladder, corpus discipline.
- `design_0_0_8_2_clausething_closeout_ladder.md` — the parent closeout that named this consumer.
- `simthing_core_design.md` §7 + `adr/mapping_sparse_regioncell.md` + `adr/resource_flow_substrate.md` — the runtime surfaces everything lowers onto.
- `clausething/ClauseThing_Spec.md` — broader ClauseScript-to-spec contract.
- Existing MapGen neutral-AST tests and the closed `mapgen_*` guard batteries — the proven ingest/lowering path for generated `static_galaxy_scenario` text.

---

## 11. Track closeout (PR12)

### Current proven output contract

MapGeneratorCLI emits **`static_galaxy_scenario` neutral-AST text** only.

**Supported emitted surfaces:**

- `system` blocks (id, inert position, bareword initializer ref)
- bareword initializer refs + sibling initializer definitions
- `add_hyperlane` endpoint pairs (hyperlanes, special routes, partition/cluster bridges)
- `nebula = { name radius }` blocks

**Unsupported / deferred:**

- arbitrary new metadata grammar
- new `field_operator` grammar beyond closed nebula feedstock
- route/path/predecessor/movement grammar
- partition/cluster/bridge grammar (identities are producer-report-only)
- runtime/GPU execution

The **`hydrate_scenario scenario { location ... }`** form is **not** the current proven path — see the legacy sketch in §5 only as superseded history.

### Track closeout summary

PR1–PR11 proved producer-side generation through shapes, topology, fields, tiny admit/install + real-adapter GPU
compact evidence, and a 1000-star scale envelope. PR12 records closeout only (docs). 0.0.8.2.5 MapGen remains
closed. `simthing-mapgenerator` has no runtime crate dependencies.

### UI/editor handoff

The UI/editor may call MapGeneratorCLI as a **producer**. It should expose high-level galaxy levers: shape, seed,
star count, lattice size, hyperlane density, special-route counts, partition/cluster settings, nebula settings,
initializer buckets. Generated output is **reviewable `static_galaxy_scenario` text** before admission. The
UI/editor must **not** treat MapGeneratorCLI as a runtime simulation service and must **not** add
route/path/predecessor/movement semantics.

### Extensibility model

New shapes are added by **registry entries** and producer-side strategy implementations. New emitted surfaces
require **already-accepted closed grammar/lowering surfaces**. MapGeneratorCLI cannot create new runtime
semantics, widen the lowerer, introduce runtime crate dependencies, or emit authoritative Euclidean
distances/magnitudes.

### Deferred closed-track capacity amendment

Full **1000-star RF/admit/install/GPU** is blocked by closed RF lowerer caps. A future galaxy-scale path requires
a **DA-authorized 0.0.8.2+ closed-lowerer capacity amendment** (raise/scalable RF participant/slot caps and/or
scalable deposit initializer feedstock). No producer-only patch may silently bypass this gate. PR10 tiny-fixture
GPU compact evidence remains the **LIVE GPU GUARDRAIL**.

### Next track: FIELD-MOVIE-DATASET-0

**FIELD-MOVIE-DATASET-0** should begin after PR12 closeout. It is a new production track and must start from
closed MapGen/MapGeneratorCLI evidence — it is not part of PR12.

Closeout report: [`../tests/mapgenerator_cli_pr12_closeout_results.md`](../tests/mapgenerator_cli_pr12_closeout_results.md) (PROBATION).

---

**Superseded next steps (pre-PR4 history — do not treat as current guidance):**
1. ~~Implement the CLI producing the `scenario { location ... link ... }` form.~~ **Done:** proven path is `static_galaxy_scenario` neutral-AST (PR4+).
2. ~~Wire end-to-end through hydrate.~~ **Done:** PR5/PR10 prove parse/lattice/admit + GPU compact evidence on generated text.
3. UI layer: parameter collector + producer caller — see §11 UI/editor handoff.

This document supplies the missing "how to turn shape selections into declarative map description" layer that mapgenthing.md intentionally left out. With it, your vision of levers for 4-armed spirals, tightness, clustering, etc. becomes buildable on top of the already-scoped MapGen work. 

---

## 10. Dependencies on Prior MapGen Ladder Artifacts (0.0.8.2.5 Cleanup Considerations)

**Determination (post-ingestion of current ladder state as of 2026-06-13):** 

The MapGen 0.0.8.2.5 PR ladder (design_0_0_8_2_5_mapgen_ladder.md) is effectively complete through its core rungs (PR1–PR9 passed/DA-approved/merged in multiple cases; PR10 end-to-end compact evidence and PR11 closeout are the final cleanup/closeout steps). It is entering the artifact hygiene phase where probation reports, temporary test results, and any scratch scaffolding are being classified for promotion, archiving, or deletion.

**Useful artifacts from the current ladder state (preserve / do not delete without explicit mapping to the new CLI track):**

- `docs/clausething/mapgen_corpus_manifest.md` + the PR1 tiny slice pin (`tiny_pentad_hub_slice`): This is the authoritative read-only corpus manifest pinning the external lab corpus root (`Clauser/Paradox/vanilla/`), the approved source families (solar_system_initializers for payloads, setup_scenarios for skeletons), and the canonical ≤5-system hand-authored slice used for all end-to-end validation. The manifest explicitly documents the selection rationale (hub pentad inspired by static_galaxy_example.txt + example_initializer) and the exact vanilla files approved for use. 

  **Why useful for MapGeneratorCLI:** The CLI is the *producer* of parameter-driven declarative payloads. It will need to select and reference the same corpus families and approved initializer/setup scenario files when emitting realistic system content for different high-level shapes (e.g., rich core initializers for spiral hubs vs sparse fringe for arms). Recreating the manifest logic and approved file list from scratch would duplicate work. The tiny slice also serves as a regression baseline: any CLI-generated output for a "simple" case should be compatible with the same lowering path that was proven on the pentad.

- `crates/simthing-clausething/tests/fixtures/mapgen/` directory structure and its contents (README.md, tiny_static_starmap_slice.clause, tiny_pentad_hub_slice_raw.clause, etc.): These are the hand-authored declarative inputs (in the exact ClauseScript scenario/location/link/initializer form the front-end expects). They start as inert stubs and become active test oracles across the rungs.

  **Why useful for MapGeneratorCLI:** The CLI must emit structurally equivalent (or richer) declarative descriptions for new shapes (4-armed spiral 1000-star, elliptical, etc.). These fixtures provide the exact "shape of a valid input" contract, test harness patterns, and goldens against which CLI outputs can be validated for lowering compatibility. The directory structure itself (with the mapgen/ namespace) is already established as the place for such declarative map fixtures.

- Key `docs/tests/mapgen_pr*_results.md` (especially PR1 corpus manifest, PR6 Movement-Front, PR7 PALMA, PR10 end-to-end compact evidence, PR11 closeout, plus the per-PR artifact lifecycle audit tables): These document the exact lowering behavior (neutral AST parse → lattice hierarchy as Location gridcells → RF arena enrollment with suppression front → bounded links + lane coupling → Gu-Yang L1 + L2 hierarchy + L3 EML/commitment authoring → PALMA W/D feedstock) and the compact-evidence style (real `install_atomic` + `SimSession::open_from_spec` + GPU tick with `is_none()` readbacks only).

  **Why useful for MapGeneratorCLI:** The CLI track will need to prove that its high-level-parameter outputs (the new "producer" side) successfully traverse the same chain. The existing results provide:
  - Templates for how to structure CLI-specific test reports and fixture generation proofs.
  - The precise battery (mapgen_* + ct_scenario_container + bh3 guards) that must stay green.
  - The artifact classification discipline (CURRENT_EVIDENCE / LIVE_GUARDRAIL / PROBATION) used in every ladder PR audit — this should be reused for consistency when the CLI track adds its own generated fixtures and end-to-end runs.
  - PR10 is the closest existing "canonical sample: ingest declarative → full lowering + GPU compact exercise" proof. CLI-generated maps can be slotted in as additional cases against the same harness.

- The overall M1–M11 schema adjudications and the detailed PR handoff sections (with test names, stop conditions, DA notes): These are permanent contracts (e.g., M2 lattice hierarchy, M3 RF with explicit caps/selectors, M5 one-system-per-cell + square lattice scaling, M6 bounded links, M7 PALMA as named consumer, M10 corpus discipline) that any declarative payload produced by the CLI must honor.

**Artifacts that will need to be (re)created or extended for the MapGeneratorCLI track (expected):**

- New shape-specific generated declarative fixtures and associated results (e.g., `four_armed_spiral_1000.clause` or equivalent scenario declarative, plus `mapgen_cli_prX_shape_generation_results.md` and end-to-end runs exercising the full lowering + compact GPU path). The current ladder only has the one tiny static pentad slice; the CLI will be the source of many parameter-driven variants (different arm counts, tightness, cluster densities, num_stars scales). These new outputs require fresh proofs that they lower correctly and exercise Movement-Front + RF + PALMA as expected.

- CLI-specific design/scratch context for the high-level generation rules: parametric arm curve math, system distribution along arms vs inter-arm gaps, multi-scale clustering heuristics, shape-specific hyperlane bridge/choke density rules, initializer bucket assignment (core/arm/fringe), seed-driven reproducibility, and parameter validation. None of this procedural "producer" logic exists in the current ladder (the ladder is purely the "consumer" side that takes pre-existing declarative and lowers it).

- Any extension to the corpus manifest or fixture set if broader initializer coverage is needed for realistic shape variety (the current manifest is deliberately minimal for the 5-system canonical sample).

**Recommendation to the user (for your decision on halting cleanup):**

The current ladder state's temporary testing proofs and context scratch logs contain **high-value reusable assets** for the MapGeneratorCLI track, primarily as the "target contract" and test harness patterns that the CLI must hit. Deleting the corpus manifest, the mapgen/ fixture directory (and its tiny fixtures), and the key mapgen_pr* evidence reports (especially the manifest, movement-front, palma, end-to-end, closeout, and their artifact audit tables) would force re-derivation of the exact same information and re-implementation of the same test structure later.

I recommend **preserving at minimum**:
- `docs/clausething/mapgen_corpus_manifest.md`
- The entire `crates/simthing-clausething/tests/fixtures/mapgen/` tree
- The mapgen_pr1, pr6, pr7, pr10, pr11 results + the per-PR artifact lifecycle tables (or fold them explicitly into a "MapGen baseline evidence" archive rather than general superseded_tests deletion)

The rest of the superseded 0.0.8.1 probation/scratch material can safely follow the planned cleanup. The ladder handoff sections themselves (the detailed PR descriptions) can be summarized or linked from the new CLI document rather than fully duplicated.

If you decide to halt (or partially pause) the cleanup PR for the mapgen-specific artifacts, I can immediately add a "Preserved baseline artifacts" subsection here with exact paths and justification. Let me know your decision and I will update this document + any related ladders accordingly. 

(References for this determination: full ingestion of `design_0_0_8_2_5_mapgen_ladder.md` §3 M10 + §6 PR handoffs + artifact audits, `clausething/mapgen_corpus_manifest.md`, `tests/mapgen_pr1_corpus_manifest_results.md`, `tests/mapgen_pr10_end_to_end_results.md`, and cross-checks against the 0.0.8.2 closeout ladder and current tests/ listing as of 2026-06-13.)

(End of reference document — track closeout: §11.)

---

## Editor-facing producer contract (MAPGENCLI-EDITOR-PREP-0 / 0R)

Future Bevy/editor frontends should **not scrape stdout**. They should invoke `mapgen` (or the
`simthing-mapgenerator` library) and consume the deterministic JSON report from `--report-json`.

**Shape params fail closed:**
- Each `--shape-param KEY=VALUE` must parse as a numeric assignment; malformed tokens error immediately.
- Unknown keys error (`Unknown shape param 'foo' for shape spiral_2`).
- Keys valid on other shapes but not the selected shape error (`Shape param 'arm_width' is not valid for shape elliptical`).
- Non-numeric registry keys such as `coordinate_transform` are rejected (`NonNumericShapeParam`).
- Numeric bounds are enforced via `ShapeParamSpec` (`shape_param_spec.rs`).

**Structural vs render:**
- Emitted `(col,row)` remain authoritative structural gridcell coordinates (STEAD contract).
- PNG jitter/glow/colour are presentation-only and must not feed topology or lowering.

**Machine-readable report (`mapgenerator.report.v1`):**
```bash
cargo run -p simthing-mapgenerator --bin mapgen -- \
  --shape spiral_2 --stars 3000 --lattice-edge 300 --seed 42 \
  --num-hyperlanes 6000 \
  --max-hyperlane-distance 8 \
  --shape-param arm_width=14 --shape-param arm_tightness=0.6 --shape-param jitter=2 \
  --no-partitions --cluster-count 4 --cluster-radius 500 --hyperlanes base --hyperlane-color blue --draw-core \
  --png-size 3000 \
  --render-png docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.png \
  --report-json docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.report.json
```
Connectivity is **ON by default** (`ensure_connected`); use `--allow-disconnected` to opt out.

When `--num-hyperlanes` exceeds the default `num_hyperlanes_max` (3), the CLI raises `num_hyperlanes_max`
to match the requested target so topology is not silently clamped to a handful of edges.

**Quality gates (0R):** the report `output` section exposes `actual_topology_hyperlanes` vs
`connectivity_bridge_count`, target satisfaction (`topology_target_ratio`, `topology_target_deficit`),
`connectivity_bridge_ratio`, and `map_quality_status` (`PASS` / `WARN` / `FAIL`) with
`map_quality_warnings`. Editors should reject WARN/FAIL maps without parsing stdout. CLI prints WARN/FAIL
to stderr; use `--fail-on-quality-warn` to exit non-zero.

Thresholds (see `report.rs`): `topology_target_ratio` FAIL &lt; 0.50; `connectivity_bridge_ratio` WARN
&gt; 0.25 / FAIL &gt; 0.50; dense preview (`stars` or target ≥ 1000) WARN if `average_degree` &lt; 2.5;
WARN if `longest_bridge_chebyshev` &gt; 32.

Report fields include request/options, topology counts, connectivity/degree stats, artifact paths, and
constitution flags. See `crates/simthing-mapgenerator/tests/editor_prep.rs`,
`docs/tests/mapgenerator_cli_editor_prep_0_results.md`, and
`docs/tests/mapgenerator_cli_editor_prep_0r_results.md`.