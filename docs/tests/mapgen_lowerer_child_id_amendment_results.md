# MapGen 0.0.8.2.5 Lowerer Child-ID Amendment Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after independent branch-source audit + regression-battery rerun; promoted from PROBATION. NOTE: amendment merged in #680 `a1103705`; this DA sign-off was re-applied as a record correction because the approval commit was lost from the #680 merge — the audit + battery below were genuinely performed before merge).

## Verdict

**PASS — DA-APPROVED (2026-06-14, executive design authority)** — minimal closed-track amendment fixing duplicate hydrated child node IDs when
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
| `mapgen_lowerer_child_id_amendment_results.md` | CURRENT_EVIDENCE (DA-approved) | New (this report) |
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

**DA-APPROVED — 2026-06-14, executive design authority.** Independent branch-source audit: the only `src/`
change is the `build_scenario_clause` child-id scoping (`{initializer.id}_planet/_deposit` →
`{system.id}_{initializer.id}_…`) — a strict id-uniqueness fix; **no `hydrate_scenario` change, no accepted-grammar
change, no new kind/kernel/runtime, no producer special-casing.** Regression battery rerun locally on the branch:
`cargo fmt --check` clean; `mapgen_lowerer_child_id_amendment` 4, `mapgen_lattice_hierarchy` 10, `mapgen_links`
19, `mapgen_neutral_ast_parse` 8, `mapgen_constitution_guards` 21, **and `mapgen_resource_flow` 16** — the last
is the decisive regression check (RF arenas still resolve deposit participants under the new
`{system.id}_{initializer.id}_deposit` ids), all green.

**Battery-scope clarification (Codex point #3):** GPU PR10 end-to-end was intentionally **not** rerun. Judged
unnecessary for this minimal id-uniqueness fix: the affected surfaces (hydration ids, RF arena participant
resolution, link grid metadata, kind allow-list) are all covered by the non-GPU battery above; the install/GPU
path resolves slots by DFS over *unique* property names, which only benefits from more-unique child ids; and the
report makes no GPU claim. The earlier "full 0.0.8.2.5 battery" phrasing is hereby scoped to the regression
surface of this change — the non-GPU `mapgen_*` + RF battery, which is sufficient. If a future amendment touches
install/GPU semantics, PR10 e2e on a real adapter is required.

## Whether PR5 may rebase onto this amendment

**Yes — after DA approval and merge of this amendment.** Cleaned 0.0.8.6 PR5 must contain **zero**
`crates/simthing-clausething/src/` changes and prove lowering against the amended closed lowerer.

## Carried-forward DA notes (not addressed here)

1. PR2: `OccupancyGrid::insert_relocated` O(cells) rebuild per insertion — revisit before scale-envelope rung.
2. PR2: `SquareLattice::cell_count` u32 edge² overflow — revisit before scale-envelope rung.
3. PR3: `strategy_by_name` / `executable_strategy_names` should be single-sourced before PR8 fills remaining vanilla shapes.
4. PR3: procedural validation rejects `arbitrary_static` but not `static`; consider unifying the mode gate.
