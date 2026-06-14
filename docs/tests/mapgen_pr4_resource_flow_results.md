# MapGen PR4 Resource Flow Enrollment Results

> **Artifact lifecycle: PROBATION** (PR4 RF enrollment report; MapGen closeout decides promotion).

## Verdict

**PASS pending DA review** — PR3 lattice hierarchy generates bounded Resource Flow enrollment with
explicit selector admission, arena caps, deposit intrinsic-flow feedstock, suppression/disruption arena,
expansion report, and no Movement-Front/PALMA/FIELD_POLICY/hyperlane/runtime leakage.

## Track scope

0.0.8.2.5 MapGen PR4: bounded RF enrollment/feedstock from PR3 hierarchy (M3). **Do not merge until
DA review.**

PR4 enrolls RF arena/feedstock only. PR4 does not implement Movement-Front. PR4 does not implement
SaturatingFlux. PR4 does not implement PALMA W/D. PR4 does not implement FIELD_POLICY commitment. PR4
does not implement hyperlane lane coupling. PR4 does not touch runtime/GPU/driver/simthing-sim. PR4 does
not import the whole Stellaris corpus.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR4, §9
6. `docs/clausething/mapgen_corpus_manifest.md`
7. `docs/clausething/MapGenThing.md`
8. `docs/clausething/ct_vertical_consumer_contract.md`
9. `docs/clausething/ct_2c_economic_category_memo.md`
10. `docs/clausething/ct_3b_4a_movement_front_heatmap_memo.md`
11. `docs/tests/mapgen_pr1_corpus_manifest_results.md`
12. `docs/tests/mapgen_pr2_neutral_ast_results.md`
13. `docs/tests/mapgen_pr3_lattice_hierarchy_results.md`
14. `docs/tests/mapgen_pr3_da_audit_results.md`
15. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| RF enrollment generator | `crates/simthing-clausething/src/mapgen_resource_flow.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| RF enrollment tests | `crates/simthing-clausething/tests/mapgen_resource_flow.rs` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| PR4 report | `docs/tests/mapgen_pr4_resource_flow_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr1_corpus_manifest_results.md` | PROBATION | Unchanged |
| `docs/tests/mapgen_pr2_neutral_ast_results.md` | PROBATION | Unchanged |
| `docs/tests/mapgen_pr3_lattice_hierarchy_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_pr3_da_audit_results.md` | CURRENT_EVIDENCE | Unchanged |
| `mapgen_neutral_ast_parse.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_lattice_hierarchy.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_resource_flow.rs` | CURRENT_EVIDENCE | New PR4 generator |
| `mapgen_resource_flow.rs` (tests) | CURRENT_EVIDENCE | New PR4 tests |
| `ct_scenario_container` | LIVE_GUARDRAIL | Unchanged |
| `docs/tests/mapgen_pr4_resource_flow_results.md` | PROBATION | New PR4 report |
| Scratch logs / duplicates / worktrees | DELETE | None found |

## M3 / RF doctrine preserved

- Deposit minerals authored value → `BaseFlowObligationSpec` produce rate (feedstock only)
- Two bounded arenas: `mapgen_deposit_minerals` (1 participant) + `mapgen_suppression` (5 gridcells)
- Explicit `EnrollmentSelectorSpec` + `explicit_participants` on every arena
- All arenas declare `max_participants`, `max_coupling_fanout`, `max_orderband_depth`
- Shallow coupling deposit → suppression (`OneTickDelay`); fanout capped
- `ResourceFlowOptInMode::Disabled` — no GPU execution in PR4
- Expansion report per arena with caps, enrolled properties, implicit-reject count
- Fixture-local 5 gridcells only — no 200×200 RF participant allocation

## Forbidden surfaces not touched

- No Movement-Front field_operator, PALMA feedstock, FIELD_POLICY commitment, hyperlane links
- No runtime/GPU/driver/simthing-sim changes
- No new `SimThingKind`, no movement/pathfinding/route/predecessor/border/frontline semantics
- No Candidate F Euclidean authority

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_resource_flow
cargo test -p simthing-clausething --test ct_scenario_container
git diff --check
```

## Test results

| Command | Result |
|---|---|
| `mapgen_neutral_ast_parse` | 8 passed |
| `mapgen_lattice_hierarchy` | 10 passed |
| `mapgen_resource_flow` | 16 passed |
| `ct_scenario_container` | 45 passed |

## DA review checklist

- [ ] Resource Flow participation is explicit
- [ ] Property possession alone does not admit a participant
- [ ] Every arena has max_participants
- [ ] Every arena has max_coupling_fanout
- [ ] Every arena has max_orderband_depth
- [ ] Expansion report exists and is bounded
- [ ] No deep unbounded fanout
- [ ] No runtime/GPU/driver/simthing-sim changes
- [ ] No Movement-Front output
- [ ] No SaturatingFlux field_operator output
- [ ] No PALMA feedstock output
- [ ] No FIELD_POLICY commitment output
- [ ] No hyperlane lane coupling output
- [ ] No pathfinding/movement/route/predecessor/border/frontline semantics
- [ ] No new SimThingKind
- [ ] No Candidate F implication
- [ ] Proof/test lifecycle performed
- [ ] Tests are focused and not proof theater

## Constraints preserved

- 0.0.8.2 closeout closed — not reopened
- **FIELD-MOVIE-DATASET-0** / editor export deferred
- PR5 is hyperlane → bounded link (next generator rung)
