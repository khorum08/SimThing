# MapGeneratorCLI PR6b — Bounded Special-Route Emission Results

> **Artifact lifecycle: PROBATION** (pending DA approval after independent branch-source audit).

## Verdict

**PROBATION — pending DA review.** Promotes PR6R evidence to CURRENT_EVIDENCE and adds bounded producer-side
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
| `docs/tests/mapgenerator_cli_special_routes_results.md` | PROBATION | New (this report) |
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

**PROBATION — pending DA approval.** No executive sign-off yet.

## Whether partition/bridge PR may proceed

**No — await DA approval of PR6b.** After DA approves this rung, **PR7** (partition/bridge structural producer +
clustering) may proceed per ladder ordering.
