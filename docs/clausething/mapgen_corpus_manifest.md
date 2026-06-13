# MapGen Corpus Manifest (0.0.8.2.5)

> **Artifact lifecycle: CURRENT_EVIDENCE** (read-only corpus pin for MapGen PR1; not a decoder claim).

## Read-only corpus policy

The local Stellaris corpus is **read-only** lab material used for adjudication and hand-authored fixture
derivation.

| Item | Value |
|---|---|
| Root path | `C:\Users\mvorm\Clauser\Paradox\vanilla\` |
| Script documentation logs | `C:\Users\mvorm\Clauser\Paradox\script_documentation\` |
| Committed to SimThing repo | **No Paradox files may be committed** |
| Fixture area | `crates/simthing-clausething/tests/fixtures/mapgen/` |
| Whole-corpus decode claim | **None** — slice-scoped fixtures only |
| Out of scope in MapGen | load-order/override, localization, trigger/effect interpretation |

Verified on host (2026-06-13): `vanilla\map\setup_scenarios\static_galaxy_example.txt` exists;
`script_documentation\{effects,triggers,scopes,modifiers,localizations}.log` exists.

## Pinned source families (later rungs)

| Family | Corpus path | MapGen use |
|---|---|---|
| Solar system initializers | `vanilla/common/solar_system_initializers/` | System payload / planet / deposit idioms |
| Setup scenarios | `vanilla/map/setup_scenarios/` | `static_galaxy_scenario` skeleton, explicit topology |
| Galaxy presentation | `vanilla/map/galaxy/` | Reference only; graphical — not runtime |
| Script documentation | `Paradox/script_documentation/*.log` | Neutral-AST / scope vocabulary cross-check (PR2+) |

Primary PR1 inspection files:

- `vanilla/map/setup_scenarios/static_galaxy_example.txt` — commented `static_galaxy_scenario`
  template with explicit `system { … }` entries and `add_hyperlane = { from … to … }`
- `vanilla/common/solar_system_initializers/example.txt` — `example_initializer` payload reference
  (stars/planets/orbits; no production usage)

## PR1 tiny slice pin — `tiny_pentad_hub_slice`

**Selection rationale:** a ≤5-system hub-and-spoke excerpt inspired by the commented pentad around
system id `"0"` in `static_galaxy_example.txt`, without copying Paradox text. One initializer-style
payload is modeled on the pedagogical shape of `example_initializer` (star + one planet + one deposit
child), not on procedural `neighbor_system` placement.

| Field | Pin |
|---|---|
| Slice name | `tiny_pentad_hub_slice` |
| System count | **5** (≤5 guardrail) |
| Hand-authored fixture | `crates/simthing-clausething/tests/fixtures/mapgen/tiny_static_starmap_slice.clause` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen parser status (PR1) | **Not parsed** — inert stub until PR2 neutral-AST adapter |
| Stellaris position fields | **Inert render metadata only** — sim adjacency is lattice + bounded links |
| Hyperlane idiom | Explicit pairwise `link { from … to … }` (bounded fan-out) |
| Deposit | One hand-authored deposit child under the rim anchor system |
| Nebula idiom | Optional metadata tag on hub sector (render-only; no field kernel in PR1) |

### Topology (logical ids)

```text
sys_rim_a (2) ---- sys_rim_b (15)
       \               /
        \             /
      sys_hub (0) -- sys_inner_a (9)
              \---- sys_inner_b (31)

Links (PR1 fixture): 0-9, 0-31, 0-2, 9-15, 31-15
```

### What this slice is not

- Not a whole-game Stellaris importer test
- Not proof of corpus-wide decode
- Not procedural placement (`spawn_weight`, `neighbor_system`, `random_hyperlanes`)
- Not movement, pathfinding, route, predecessor, border, or frontline semantics

## Governing read-order (binding)

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0 and §3
6. `docs/clausething/ct_vertical_consumer_contract.md`
7. `docs/clausething/ct_2c_economic_category_memo.md`
8. `docs/clausething/ct_3b_4a_movement_front_heatmap_memo.md`
9. `docs/clausething/MapGenThing.md`
10. `docs/tests/clausething_closeout_results.md`

Candidate F authority remains in `docs/design_0_0_8_1.md` §0.7 — not duplicated here.
