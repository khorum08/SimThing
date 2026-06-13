# MapGen test fixtures (0.0.8.2.5)

Hand-authored equivalents for the MapGen PR ladder. **Not parsed by MapGen in PR1.**

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
| `tiny_static_starmap_slice.clause` | Inert stub — **not yet parsed by MapGen** (PR2+) |

**Corpus inspiration (read-only, not copied):**

- `vanilla/map/setup_scenarios/static_galaxy_example.txt` — hub pentad + explicit hyperlanes
- `vanilla/common/solar_system_initializers/example.txt` — pedagogical initializer payload shape

**Contents (hand-authored):** five system locations, five bounded N4-style links, one deposit child,
optional nebula metadata comment on the hub sector.

## Closeout guardrails (unchanged)

0.0.8.2 closeout batteries remain the active ClauseThing guardrails:

```text
cargo test -p simthing-clausething --test ct_scenario_container
cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver
```

MapGen tests are added in later rungs only.
