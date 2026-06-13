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

**Contents (hand-authored):** five system locations, five bounded N4-style links, one deposit child,
optional nebula metadata comment on the hub sector.

## PR2 raw parse fixture — `tiny_pentad_hub_slice_raw.clause`

| File | Status |
|---|---|
| `tiny_pentad_hub_slice_raw.clause` | **Parsed by PR2** neutral-AST adapter (parse-only; not lowered) |

Stellaris-style raw authoring idioms: `static_galaxy_scenario`, repeated `system` / `add_hyperlane`,
`nebula`, and `example_rim_initializer` with `planet` + `deposit` child. Header uses jomini `#` line
comment (Paradox-style) with required hand-authored disclaimer; no Paradox copy.

**Test command:**

```text
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
```

## Closeout guardrails (unchanged)

0.0.8.2 closeout batteries remain the active ClauseThing guardrails:

```text
cargo test -p simthing-clausething --test ct_scenario_container
cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver
```

MapGen PR2 adds a focused parse-only test battery only.
