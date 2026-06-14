# MapGen test fixtures (0.0.8.2.5)

Hand-authored equivalents for the MapGen PR ladder.

## Policy

- Fixtures are **hand-authored equivalents** derived from read-only local corpus inspection
  (`C:\Users\mvorm\Clauser\Paradox\vanilla\`).
- **No Paradox files are vendored or committed** to the SimThing repo.
- Each fixture is **slice-scoped and ≤5 systems** for the initial pentad target.
- Fixtures are **not** whole-game importer tests.
- Fixtures are **not** proof of corpus-wide Stellaris import.
- Fixtures must **not** introduce movement, pathfinding, route, predecessor, border, or frontline
  semantics.
- Stellaris-style `position` metadata (when noted in comments) is **inert render metadata only**.
  Sim adjacency is lattice/topological plus bounded lane coupling — not Euclidean distance.

## Corpus manifest

See [`docs/clausething/mapgen_corpus_manifest.md`](../../../../../docs/clausething/mapgen_corpus_manifest.md).

## PR1 slice — `tiny_pentad_hub_slice`

| File | Status |
|---|---|
| `tiny_static_starmap_slice.clause` | Inert PR1 stub — ClauseScript scenario shape; **not parsed by MapGen** |

**Corpus inspiration (read-only, not copied):**

- `vanilla/map/setup_scenarios/static_galaxy_example.txt` — hub pentad + explicit hyperlanes
- `vanilla/common/solar_system_initializers/example.txt` — pedagogical initializer payload shape

**Contents (hand-authored):** five system locations, five hyperlane declarations (lowered in PR5 to three
N4 lattice links plus two lane couplings), one deposit child, optional nebula metadata comment on the hub
sector.

## PR2 raw parse fixture — `tiny_pentad_hub_slice_raw.clause`

| File | Status |
|---|---|
| `tiny_pentad_hub_slice_raw.clause` | **Parsed by PR2**; **lowered by PR3**; **RF-enrolled by PR4**; **link/coupling by PR5**; **Movement-Front authoring by PR6** |

Stellaris-style raw authoring idioms: `static_galaxy_scenario`, repeated `system` / `add_hyperlane`,
`nebula`, and `example_rim_initializer` with `planet` + `deposit` child. Header uses jomini `#` line
comment (Paradox-style) with required hand-authored disclaimer; no Paradox copy.

**Test commands:**

```text
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_resource_flow
cargo test -p simthing-clausething --test mapgen_links
cargo test -p simthing-clausething --test mapgen_movement_front
cargo test -p simthing-clausething --test mapgen_palma
```

PR3 generates `galaxy_map → pentad_sector → gridcell systems` as ordinary `Location` SimThings with
`mapgen` mapping-role metadata. Hyperlanes in the raw fixture are **not** lowered until PR5.

PR4 adds bounded Resource Flow enrollment from the PR3 hierarchy: deposit minerals intrinsic-flow
feedstock + suppression/disruption arena with explicit participants, caps, and expansion report. No
Movement-Front, PALMA, FIELD_POLICY, or hyperlane output in PR4.

PR5 lowers `add_hyperlane` declarations into bounded N4 lattice links plus bounded `mapgen::lane_coupling`
inert authoring properties (long-range edges). Adjacency uses PR3 lattice placements only — not Stellaris
`position` coordinates. No pathfinding, movement, routes, predecessors, border/frontline, Movement-Front,
PALMA, FIELD_POLICY, or runtime/GPU/driver/simthing-sim output in PR5.

PR6 adds Movement-Front L1/L2/L3 authoring feedstock on the PR5-enrolled pack: bounded `SaturatingFlux`
lattice field operator with suppression RF `pressure_binding`, hierarchy reduction feedstock, and threshold
commitment feedstock. Authoring/lowering only — no driver/GPU execution, no PALMA, no runtime/GPU/driver/
simthing-sim output in PR6.

## PR7 — PALMA W/D reach feedstock

`generate_mapgen_palma_feedstock` lowers PR6 Movement-Front enrollment into existing
`HydratedScenarioPalmaFeedstock` + generic `WImpedanceComposeSpec` (W from PR6 SaturatingFlux choke column,
D output col 4). Authoring/lowering only — no driver/GPU execution, no routes/paths/predecessors/movement
orders, no runtime/GPU/driver/simthing-sim output in PR7.

```text
cargo test -p simthing-clausething --test mapgen_palma
```

## Closeout guardrails (unchanged)

0.0.8.2 closeout batteries remain the active ClauseThing guardrails:

```text
cargo test -p simthing-clausething --test ct_scenario_container
cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver
```

MapGen PR2 adds a focused parse-only test battery. MapGen PR3 adds `mapgen_lattice_hierarchy` tests.
MapGen PR4 adds `mapgen_resource_flow` tests. MapGen PR5 adds `mapgen_links` tests. MapGen PR6 adds
`mapgen_movement_front` tests. MapGen PR7 adds `mapgen_palma` tests.
