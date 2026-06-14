# MapGen 0.0.8.2.5 Lowerer Child-ID Amendment Results

> **Artifact lifecycle: PROBATION** (pending DA review — do not treat as CURRENT_EVIDENCE until DA approves).

## Verdict

**PASS pending DA review** — minimal closed-track amendment fixing duplicate hydrated child node IDs when
multiple systems reference the same initializer bareword. **Not a reopening of 0.0.8.2.5 scope.** No grammar
widening, no producer special-casing, no topology, field operators, RF, Movement-Front, PALMA, driver/GPU, or
runtime work.

## Root cause

`mapgen_lattice.rs` `build_scenario_clause` emitted initializer payload children using only
`{initializer.id}_planet` and `{initializer.id}_deposit`. When multiple systems share one initializer
definition (normal Stellaris shape; required for later bucketed initializer work), hydration failed with
duplicate global scenario node IDs.

## Exact lowerer change

In `crates/simthing-clausething/src/mapgen_lattice.rs`:

```text
child = {system.id}_{initializer.id}_planet
child = {system.id}_{initializer.id}_deposit
```

No `hydrate_scenario` changes. No accepted grammar changes. No producer-specific forgiveness.

## Why this is a closed-track amendment

0.0.8.2.5 MapGen remains **CLOSED** as the ingest/lowering contract. This is a DA-authorized bugfix to the
closed lowerer's intermediate hydration IDs — not a new rung, not a producer-track change, and not a reopening
for topology/RF/GPU work.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| `mapgen_lowerer_child_id_amendment_results.md` | PROBATION | New (this report) |
| LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Re-run; no edits unless required |

## Files changed

| Area | Path |
|---|---|
| Lowerer fix | `crates/simthing-clausething/src/mapgen_lattice.rs` |
| Amendment tests | `crates/simthing-clausething/tests/mapgen_lowerer_child_id_amendment.rs` |
| Ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGeneratorCLI ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |

## Test commands

```text
cargo fmt --all -- --check
cargo test -p simthing-clausething --test mapgen_lowerer_child_id_amendment
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_resource_flow
cargo test -p simthing-clausething --test mapgen_links
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
```

## Test results (2026-06-14 local validation)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-clausething --test mapgen_lowerer_child_id_amendment` | 4 passed |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | 10 passed |
| `cargo test -p simthing-clausething --test mapgen_resource_flow` | pass |
| `cargo test -p simthing-clausething --test mapgen_links` | pass |
| `cargo test -p simthing-clausething --test mapgen_neutral_ast_parse` | 8 passed |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `git diff --check` | pass |

GPU end-to-end compact evidence not re-run in this amendment (no GPU claim).

## DA sign-off status

**Pending DA review.** Only the Design Authority writes DA sign-off.

## Whether PR5 may rebase onto this amendment

**Yes — after DA approval and merge of this amendment.** Cleaned 0.0.8.6 PR5 must contain **zero**
`crates/simthing-clausething/src/` changes and prove lowering against the amended closed lowerer.

## Carried-forward DA notes (not addressed here)

1. PR2: `OccupancyGrid::insert_relocated` O(cells) rebuild per insertion — revisit before scale-envelope rung.
2. PR2: `SquareLattice::cell_count` u32 edge² overflow — revisit before scale-envelope rung.
3. PR3: `strategy_by_name` / `executable_strategy_names` should be single-sourced before PR8 fills remaining vanilla shapes.
4. PR3: procedural validation rejects `arbitrary_static` but not `static`; consider unifying the mode gate.
