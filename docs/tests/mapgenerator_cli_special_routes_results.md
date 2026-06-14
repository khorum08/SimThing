# MapGeneratorCLI PR6b — Bounded Special-Route Emission Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after independent branch-source audit + battery rerun; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-14, executive design authority)** — promotes PR6R evidence to CURRENT_EVIDENCE and adds bounded producer-side
wormhole-pair / gateway special-route endpoint selection represented **only** as `static_galaxy_scenario`
`add_hyperlane` declarations. Generated long-range pairs parse and lower through existing closed MapGen link surfaces
as bounded lane couplings. **Zero** `crates/simthing-clausething/src/` changes. No new grammar, no
route/path/predecessor/movement/border/frontline semantics, field operators, RF, Movement-Front, PALMA, driver/GPU,
simthing-sim, new `SimThingKind`, or FIELD-MOVIE-DATASET-0 work.

## Track scope

0.0.8.6 MapGeneratorCLI PR6b completes the outstanding special-route rung deferred from PR6 (#684). **0.0.8.2.5 MapGen
remains closed and is not reopened.** PR7 (partition/bridge structural producer + clustering) is **not** started in
this PR.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgenerator_cli_pr6_hyperlane_results.md` | CURRENT_EVIDENCE | Unchanged — preserved |
| `docs/tests/mapgenerator_cli_pr6r_hardening_results.md` | CURRENT_EVIDENCE (DA-approved #685) | Promoted from PROBATION (Part A) |
| `docs/tests/mapgenerator_cli_special_routes_results.md` | CURRENT_EVIDENCE (DA-approved) | New (this report) |
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/tests/mapgenerator_cli_pr1_params_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr2_lattice_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr3_strategy_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr4_emitter_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr5_lowering_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_lowerer_child_id_amendment_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## PR6R record correction summary

Part A of this PR promotes PR6R (#685) to **DA-APPROVED & MERGED** in the production track and ladder; updates
`mapgenerator_cli_pr6r_hardening_results.md` from PROBATION to **CURRENT_EVIDENCE**.

## Files changed

| Area | Path |
|---|---|
| Special-route selection | `crates/simthing-mapgenerator/src/special_routes.rs` |
| Coupling pipeline | `crates/simthing-mapgenerator/src/lib.rs` |
| Crate-local tests | `crates/simthing-mapgenerator/tests/special_routes.rs` |
| Integration lowering proof | `crates/simthing-clausething/tests/mapgenerator_cli_special_routes_lower.rs` |
| PR6R promotion + PR6b addendum | `docs/design_0_0_8_1_clausething_production_track.md` |
| Ladder PR6R/PR6b status | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |
| PR6R report promotion | `docs/tests/mapgenerator_cli_pr6r_hardening_results.md` |
| PR6b results (this report) | `docs/tests/mapgenerator_cli_special_routes_results.md` |

## Special route model summary

Producer types: `SpecialRouteKind` (`WormholePair`, `Gateway`), `SpecialRouteEdge`, `SpecialRouteOptions`,
`SpecialRouteReport`, `SpecialRouteError`.

`generate_special_routes` selects long-range endpoint pairs (non-N4 on lowered index-order grid positions), honors
`num_wormhole_pairs` and `num_gateways`, respects per-node fanout cap and deduplication against existing hyperlane
edges, and fails closed with `UnsatisfiedRouteCount` when bounded counts are impossible. Kind is tracked in producer
reports only — **not** emitted in scenario grammar.

## Wormhole / gateway count handling

Counts come from `MapGeneratorParams.special_routes` (`num_wormhole_pairs`, `num_gateways`). Wormhole pairs are
selected first, then gateways from remaining long-range candidates. Same seed yields stable selection; different seeds
differ when multiple equally valid candidates exist (deterministic Fisher–Yates shuffle after distance/id sort).

## add_hyperlane-only emission summary

`place_and_emit_scenario_with_couplings` merges hyperlane and special-route edges into one `HyperlaneTopology` consumed
by the existing PR6 emitter. Output contains only `add_hyperlane = { from = "…" to = "…" }` blocks — no `wormhole`,
`gateway`, `special_route`, `route`, `path`, or predecessor grammar.

## Parse / lattice / link lowering proof

Integration pipeline (test harness injects deposit block so PR4 RF enrollment succeeds — test-only, no emitter change):

```text
MapGeneratorCLI static placement
→ generate_special_routes (wormhole + gateway)
→ static_galaxy_scenario add_hyperlane emission
→ parse_mapgen_neutral_document
→ generate_mapgen_lattice_hierarchy
→ generate_mapgen_resource_flow_enrollment
→ generate_mapgen_links
→ lane_coupling evidence for long-range pairs
```

## Lane-coupling evidence

Non-N4 `add_hyperlane` pairs from special-route selection lower to `MapGenLaneCoupling` entries via existing
`lower_hyperlane_topology` inside `generate_mapgen_links`. Integration test asserts non-empty `lane_couplings`,
zero unknown/self/duplicate rejections, and empty unsafe expansion flags on the default caps.

## Closed-source gate result

**PASS (expected)** — diff excludes `crates/simthing-clausething/src/`, `simthing-sim`, `simthing-gpu`,
`simthing-driver`, `simthing-spec`. No closed-front-end widening.

## Forbidden semantics scan summary

Emitted scenario text contains no `wormhole`, `gateway`, `special_route`, `route`, `path`, `predecessor`, `movement`,
`border`, `frontline`, `field_operator`, RF, Movement-Front, PALMA, commitment, or driver/GPU surfaces.

## Dependency boundary

- `simthing-clausething` dev-depends on `simthing-mapgenerator` for integration tests only.
- `simthing-mapgenerator` does **not** depend on `simthing-clausething` or other forbidden crates.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgenerator_cli_special_routes_lower
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
| `cargo test -p simthing-mapgenerator` | 114 passed |
| `cargo test -p simthing-clausething --test mapgenerator_cli_special_routes_lower` | 6 passed |
| `cargo test -p simthing-clausething --test mapgen_links` | 19 passed |
| `cargo test -p simthing-clausething --test mapgen_neutral_ast_parse` | 8 passed |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | 10 passed |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `git diff --check` | pass |
| closed-src gate | pass — no forbidden paths |

## DA sign-off status

**DA-APPROVED — 2026-06-14, executive design authority.** Independent branch-source audit of `special_routes.rs`:
bounded **long-range** endpoint-pair selection — explicitly **skips N4-neighbors** (`is_n4_neighbor`, mirroring
the closed `mapgen_links` adjacency), sorts candidates by distance **descending**, dedups against existing
hyperlane edges, shares the per-node fanout cap (4), and **fails closed** with `UnsatisfiedRouteCount` when
`num_wormhole_pairs`/`num_gateways` cannot be met. Deterministic (sort + pinned-RNG Fisher–Yates). **The
wormhole/gateway `kind` is producer-report-only and is NOT emitted in grammar** — special routes lower as
ordinary `add_hyperlane` endpoint pairs that the closed `lower_hyperlane_topology` classifies as bounded **lane
couplings** (proven by the 6 integration tests: parse → lattice → RF → `generate_mapgen_links` → lane couplings).
No route/path/predecessor/movement semantics; **zero `crates/simthing-clausething/src/` changes**; no `simthing-*`
dep in the producer. Battery rerun on the branch: `cargo fmt --check` clean; `cargo test -p simthing-mapgenerator`
114 passed; `mapgenerator_cli_special_routes_lower` 6; `mapgen_links` 19; `mapgen_constitution_guards` 21;
`mapgen_lattice_hierarchy` 10. Forbidden-semantics scan of producer `src/` returned NONE. PR6R (#685) ratified
retroactively in the same review (see its report — owner-merged before DA review; benign hardening). **This
completes the planned PR6 rung scope (hyperlanes #684 + special routes #686).**

## Whether partition/bridge PR may proceed

**Yes — DA approved PR6b (2026-06-14).** **PR7** (partition/bridge structural producer + clustering) may proceed
per ladder ordering — no route/path/predecessor semantics and no GPU.
