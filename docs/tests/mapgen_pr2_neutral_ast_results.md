# MapGen PR2 Neutral-AST Parse Results

> **Artifact lifecycle: PROBATION** (PR2 parse-only adapter report; MapGen closeout decides promotion).

## Verdict

**PASS / DA-APPROVED (2026-06-13, Opus / Design Authority; merge `edeab38a`)** — neutral-AST parse-only
adapter exists; tiny hand-authored raw mapgen fixture parses; repeated keys, nesting, and sibling
order/count preserved by tests; no semantic mapping; no SimThing structures produced; no Paradox files
committed; lifecycle audit recorded; focused tests pass. DA reran the battery green
(`mapgen_neutral_ast_parse` 8 passed, `ct_scenario_container` 45 passed, `fmt`/`git diff --check` clean)
and verified all checklist items independently. `mapgen_neutral_ast_parse` promoted to LIVE_GUARDRAIL.

## Track scope

0.0.8.2.5 MapGen PR2: neutral-AST parse-only adapter spike (M1). **Do not merge until DA review.**

PR2 is parse-only. No semantic mapping. No generated SimThing structure. No parser/importer runtime. No
Paradox files committed. PR3 is the first hierarchy-generation rung.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR2, §9
6. `docs/clausething/mapgen_corpus_manifest.md`
7. `docs/clausething/MapGenThing.md`
8. `docs/clausething/ct_vertical_consumer_contract.md`
9. `docs/clausething/ct_2c_economic_category_memo.md`
10. `docs/clausething/ct_3b_4a_movement_front_heatmap_memo.md`
11. `docs/tests/mapgen_pr1_corpus_manifest_results.md`
12. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| Neutral-AST adapter | `crates/simthing-clausething/src/mapgen_neutral_ast.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| Parse tests | `crates/simthing-clausething/tests/mapgen_neutral_ast_parse.rs` |
| Raw parse fixture | `crates/simthing-clausething/tests/fixtures/mapgen/tiny_pentad_hub_slice_raw.clause` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| PR2 report | `docs/tests/mapgen_pr2_neutral_ast_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/clausething_closeout_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/clausething/mapgen_corpus_manifest.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_pr1_corpus_manifest_results.md` | PROBATION | Unchanged |
| `tiny_static_starmap_slice.clause` | PROBATION | Inert PR1 stub |
| `tiny_pentad_hub_slice_raw.clause` | CURRENT_EVIDENCE | New PR2 raw fixture |
| `mapgen_neutral_ast.rs`, `mapgen_neutral_ast_parse.rs` | CURRENT_EVIDENCE | New PR2 adapter + tests |
| `ct_scenario_container`, `ct_bh3_closeout_sample_driver` | LIVE_GUARDRAIL | Unchanged |
| Scratch logs / duplicates / `target/` / worktrees | DELETE | None found |

**Deleted:** none.

**Archived:** none in PR2.

## M1 doctrine preserved

- Raw text → neutral AST / `RawDocument` via jomini
- Repeated keys, order, and nesting preserved
- Zero mapping decisions
- No load-order/override/localization/trigger/effect interpretation
- No generated SimThing structures
- No spec/driver/sim change

## Forbidden surfaces not touched

- No lowering to `scenario`, locations, links, RF arenas, PALMA feedstock, or commitments
- No runtime/GPU/driver/`simthing-sim` changes
- No typed System/Hyperlane/Deposit semantic model
- No movement/pathfinding/route/predecessor/border/frontline semantics in code/API
- No Candidate F numeric algorithms (positions are parse tokens only)

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test ct_scenario_container
git diff --check
```

## Test results

| Command | Result |
|---|---|
| `mapgen_neutral_ast_parse` | 8 passed |
| `ct_scenario_container` | 45 passed |

## DA review checklist (verified 2026-06-13, Opus / Design Authority)

- [x] No semantic decisions in parse
- [x] No mapgen typed semantic AST yet
- [x] Repeated keys, order, and nesting are preserved
- [x] No generated SimThing structures
- [x] No runtime/GPU/driver/simthing-sim changes
- [x] No Paradox files committed
- [x] No movement/pathfinding/route/predecessor/border/frontline semantics
- [x] No Candidate F implication
- [x] Read-order and lifecycle audit performed
- [x] Tests are focused and not proof theater

## Constraints preserved

- 0.0.8.2 closeout closed — not reopened
- **FIELD-MOVIE-DATASET-0** / editor export deferred
- PR3 is the first hierarchy-generation rung

## Lifecycle classification for new artifacts

| Artifact | Classification |
|---|---|
| `docs/tests/mapgen_pr2_neutral_ast_results.md` | PROBATION |
| `tiny_pentad_hub_slice_raw.clause` | CURRENT_EVIDENCE |
| `mapgen_neutral_ast.rs`, `mapgen_neutral_ast_parse.rs` | CURRENT_EVIDENCE |
