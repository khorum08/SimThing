# MapGeneratorCLI PR6 — Bounded Hyperlane / add_hyperlane Emission Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after independent branch-source audit + battery rerun; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-14, executive design authority)** — MapGeneratorCLI PR6 adds bounded producer-side hyperlane edge selection and
`static_galaxy_scenario` `add_hyperlane` emission. Generated output parses and lowers through existing closed
MapGen neutral-AST → `mapgen_lattice` → `lower_hyperlane_topology` surfaces without front-end widening.
**Zero** `crates/simthing-clausething/src/` changes. **No route/path/predecessor/movement/border/frontline
semantics, field operators, RF, Movement-Front, PALMA, driver/GPU, or FIELD-MOVIE-DATASET-0 work.**

## Track scope

0.0.8.6 MapGeneratorCLI PR6 extends the PR4/PR5 producer→closed-front-end seam with declarative hyperlane
topology only. **0.0.8.2.5 MapGen remains closed and is not reopened.**

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/tests/mapgenerator_cli_pr1_params_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr2_lattice_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr3_strategy_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr4_emitter_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr5_lowering_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_lowerer_child_id_amendment_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| `mapgenerator_cli_pr6_hyperlane_results.md` | CURRENT_EVIDENCE (DA-approved) | New (this report) |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| Hyperlane topology | `crates/simthing-mapgenerator/src/topology.rs` |
| Emitter | `crates/simthing-mapgenerator/src/emitter.rs` — optional `add_hyperlane` blocks |
| Pipeline | `crates/simthing-mapgenerator/src/lib.rs` — `place_and_emit_scenario_with_hyperlanes` |
| Topology tests | `crates/simthing-mapgenerator/tests/topology.rs` |
| Emitter tests | `crates/simthing-mapgenerator/tests/emitter.rs` |
| Lowering proof | `crates/simthing-clausething/tests/mapgenerator_cli_pr6_generated_hyperlanes_lower.rs` |
| Ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

**Not changed:** `crates/simthing-clausething/src/`, `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`.

## Hyperlane topology model summary

- `HyperlaneEdge`, `HyperlaneTopology`, `HyperlaneOptions`, `HyperlaneGenerationReport`, `HyperlaneError`.
- Candidate edges from in-memory placements using **index-order lowered grid positions** (matching closed
  `assign_system_placements`), Chebyshev distance bound from `max_hyperlane_distance`, canonical pair ordering,
  `prevent_hyperlane` exclusion list, min/max/target edge counts, per-node fanout cap, deterministic RNG shuffle
  when `random_hyperlanes` is enabled.

## add_hyperlane emitter summary

- `ScenarioEmitter::emit(..., hyperlanes: Option<&HyperlaneTopology>)` writes `add_hyperlane = { from = "…" to = "…" }`
  blocks inside `static_galaxy_scenario` after system blocks.
- PR4/PR5 paths pass `None` — no hyperlanes unless explicitly requested via `place_and_emit_scenario_with_hyperlanes`.

## Closed-src gate

```text
git diff --name-only master...HEAD
```

Result (2026-06-14 local): **no** paths under `crates/simthing-clausething/src/`, `simthing-sim`, `simthing-gpu`,
`simthing-driver`, or `simthing-spec`.

## Parse/lattice/link lowering proof summary

Pipeline exercised in integration tests:

```text
place_and_emit_scenario_with_hyperlanes
→ parse_mapgen_neutral_document
→ generate_mapgen_lattice_hierarchy
→ extract_hyperlane_declarations
→ lower_hyperlane_topology
```

Proof uses `lower_hyperlane_topology` on the lattice pack (equivalent closed link-lowering surface; full
`generate_mapgen_links` RF enrollment deferred until deposit-bearing initializer emission exists). N4-adjacent
pairs become bounded scenario links; longer-range pairs within the distance cap become bounded lane couplings.
Self-links, unknown endpoints, and duplicate pairs are rejected by the closed reader without widening.

## Expansion report summary

Producer `HyperlaneGenerationReport` tracks candidate/selected counts and prevent/fanout rejections.
Closed `MapGenLinksExpansionReport` from `lower_hyperlane_topology` tracks link/lane-coupling counts,
per-node fanout, and rejection counters — no unsafe expansion flags on the default PR6 sample.

## Forbidden semantics scan summary

Emitted text and lowered packs contain no `route`, `path`, `predecessor`, `movement`, `border`, `frontline`,
`field_operator`, RF, Movement-Front, PALMA, commitment, or driver/GPU surfaces.

## Dependency boundary

- `simthing-clausething` dev-depends on `simthing-mapgenerator` for integration tests only.
- `simthing-mapgenerator` does **not** depend on `simthing-clausething` or other forbidden crates.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgenerator_cli_pr6_generated_hyperlanes_lower
cargo test -p simthing-clausething --test mapgen_links
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
git diff --name-only master...HEAD
```

## Test results (2026-06-14 local validation)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-mapgenerator` | 100 passed |
| `cargo test -p simthing-clausething --test mapgenerator_cli_pr6_generated_hyperlanes_lower` | 8 passed |
| `cargo test -p simthing-clausething --test mapgen_links` | pass |
| `cargo test -p simthing-clausething --test mapgen_neutral_ast_parse` | pass |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | pass |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | pass |
| `git diff --check` | pass |
| closed-src gate | pass — no forbidden paths |

## DA sign-off status

**DA-APPROVED — 2026-06-14, executive design authority.** Independent branch-source audit: `topology.rs`
read in full — output is **undirected `from/to` endpoint pairs only** (no predecessor/route/path/direction/
weight — M3/M6 bounded-grid honored); adjacency is Chebyshev distance on **lowered index-order grid
positions** (`index/edge, index%edge` — matches closed `assign_system_placements`; authored producer coords
unused, consistent with the binding PR5 inert-coordinate ruling); bounded by `max_hyperlane_distance` +
`target_edge_count` clamp(min,max) + per-node fanout cap 4 + `prevent_pairs`; deterministic (distance/id sort +
optional pinned-RNG Fisher–Yates); validation rejects self-links/duplicates/unknown endpoints. The integration
test drives the **existing closed** link surface (`extract_hyperlane_declarations` + `lower_hyperlane_topology`
in `mapgen_links.rs` — confirmed unchanged) and includes a `rejects_unknown_endpoint_without_widening` proof
that the closed reader still enforces validity. **Zero `crates/simthing-clausething/src/` changes; no
`simthing-*` dep in the producer.** Battery rerun locally on the branch: `cargo fmt --check` clean;
`cargo test -p simthing-mapgenerator` all pass; `mapgenerator_cli_pr6_generated_hyperlanes_lower` 8 passed;
`mapgen_links` 19 passed; `mapgen_constitution_guards` 21 passed.

**Deferral accepted as honest (not an overclaim):** full `generate_mapgen_links` RF enrollment (deposit-coupled)
is deferred until deposit-bearing initializer emission exists; PR6 proves the **link-lowering** surface
(`lower_hyperlane_topology` → `grid_metadata.links`/`lane_couplings`), which is the correct scope for a
hyperlane-emission rung. **New non-blocking DA note:** `generate_hyperlane_topology` enumerates candidate pairs
O(N²) — fine for tiny fixtures, but joins the scale-envelope notes (bound/optimize before the PR11 1000-star rung).

## Whether PR7 may proceed

**Yes — DA approved PR6 (2026-06-14).** PR7 = partition/bridge structural producer + clustering — still no
route/path/predecessor semantics and no GPU.

## Carried-forward DA notes (not addressed in PR6)

1. PR2: `OccupancyGrid::insert_relocated` O(cells) rebuild per insertion — revisit before scale-envelope rung.
2. PR2: `SquareLattice::cell_count` u32 edge² overflow — revisit before scale-envelope rung.
3. PR3: `strategy_by_name` / `executable_strategy_names` should be single-sourced before PR8.
4. PR3: procedural validation rejects `arbitrary_static` but not `static`; consider unifying the mode gate.
5. PR5: proof scope is parse/lower + inert render-coordinate preservation only — not authored-position →
   authoritative gridcell placement.
