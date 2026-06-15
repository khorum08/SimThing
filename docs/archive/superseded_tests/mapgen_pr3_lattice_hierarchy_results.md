# MapGen PR3 Lattice Hierarchy Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (PR3 hierarchy generator report; DA-approved post-merge audit
> `mapgen_pr3_da_audit_results.md`).

## Verdict

**PASS / DA-APPROVED (2026-06-13, post-merge audit PR #658; merge `67d6ab8c`)** — tiny neutral-AST fixture generates scenario-container-compatible gridcell
hierarchy; all gridcells are ordinary `SimThingKind::Location` nodes with mapping-role metadata; no new
`SimThingKind`; one-system-per-cell enforced on fixture-local 3×3 lattice; canonical 200×200 documented as
metadata only; no RF/PALMA/FIELD_POLICY/Movement-Front/link output; focused tests pass.

## Track scope

0.0.8.2.5 MapGen PR3: gridcell lattice hierarchy generation (M2/M5). **DA-approved post-merge (audit
`mapgen_pr3_da_audit_results.md`).**

PR3 generates hierarchy/placement only. PR3 does not enroll RF arenas. PR3 does not generate
Movement-Front fields. PR3 does not generate PALMA feedstock. PR3 does not generate FIELD_POLICY
commitments. PR3 does not touch runtime/GPU/driver/simthing-sim. PR3 does not implement whole-corpus
import.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR3, §9
6. `docs/clausething/mapgen_corpus_manifest.md`
7. `docs/clausething/MapGenThing.md`
8. `docs/clausething/ct_vertical_consumer_contract.md`
9. `docs/clausething/ct_2c_economic_category_memo.md`
10. `docs/clausething/ct_3b_4a_movement_front_heatmap_memo.md`
11. `docs/tests/mapgen_pr1_corpus_manifest_results.md`
12. `docs/tests/mapgen_pr2_neutral_ast_results.md`
13. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| Lattice generator | `crates/simthing-clausething/src/mapgen_lattice.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| Hierarchy tests | `crates/simthing-clausething/tests/mapgen_lattice_hierarchy.rs` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| PR3 report | `docs/tests/mapgen_pr3_lattice_hierarchy_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/clausething_closeout_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/clausething/mapgen_corpus_manifest.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_pr1_corpus_manifest_results.md` | PROBATION | Unchanged |
| `docs/tests/mapgen_pr2_neutral_ast_results.md` | PROBATION | Unchanged |
| `mapgen_neutral_ast_parse.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_lattice_hierarchy.rs` | LIVE_GUARDRAIL | Promoted at DA audit |
| `mapgen_lattice.rs` | CURRENT_EVIDENCE | PR3 generator source |
| `tiny_pentad_hub_slice_raw.clause` | PROBATION / active fixture | Unchanged |
| `ct_scenario_container`, `ct_bh3_closeout_sample_driver` | LIVE_GUARDRAIL | Unchanged |
| Scratch logs / duplicates / `target/` / worktrees | DELETE | None found |

**Deleted:** none.

**Archived:** none in PR3.

## M2/M5 doctrine preserved

- Galaxy gridcell lattice doctrine documented (`MAPGEN_CANONICAL_LATTICE_EDGE = 200`, square)
- Fixture uses bounded 3×3 active subset only (no 200×200 live allocation)
- Gridcells are `SimThingKind::Location` with `mapgen` mapping-role metadata — not a new kind
- Authored Stellaris positions stored as inert render metadata (`description = inert=…`)
- Placement uses declaration-order row-major assignment — not Euclidean authority
- One system per gridcell enforced
- Hierarchy: scenario → galaxy_map → pentad_sector → system(gridcell) → planet/deposit children
- No hyperlane/link topology in PR3 (deferred to PR5)

## Forbidden surfaces not touched

- No RF arena enrollment, Movement-Front field_operator, PALMA feedstock, FIELD_POLICY commitment
- No runtime/GPU/driver/`simthing-sim` changes
- No new `SimThingKind`, no movement/pathfinding/route/predecessor/border/frontline semantics

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test ct_scenario_container
git diff --check
```

## Test results

| Command | Result |
|---|---|
| `mapgen_neutral_ast_parse` | 8 passed |
| `mapgen_lattice_hierarchy` | 10 passed |
| `ct_scenario_container` | 45 passed |

## DA review checklist

Recorded in [`mapgen_pr3_da_audit_results.md`](mapgen_pr3_da_audit_results.md) — all items **PASS** (2026-06-13).

## Constraints preserved

- 0.0.8.2 closeout closed — not reopened
- **FIELD-MOVIE-DATASET-0** / editor export deferred
- PR4 is RF arena generation (next generator rung)

## Lifecycle classification for new artifacts

| Artifact | Classification |
|---|---|
| `docs/tests/mapgen_pr3_lattice_hierarchy_results.md` | CURRENT_EVIDENCE |
| `docs/tests/mapgen_pr3_da_audit_results.md` | CURRENT_EVIDENCE |
| `mapgen_lattice.rs` | CURRENT_EVIDENCE |
| `mapgen_lattice_hierarchy.rs` | LIVE_GUARDRAIL |
