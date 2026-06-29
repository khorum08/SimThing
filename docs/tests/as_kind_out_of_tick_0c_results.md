# AS-KIND-OUT-OF-TICK-0C Results

## Status

PASS — fission child-spawn kind resolution moved from `simthing-sim` to `simthing-core`.

## PR / branch / merge

- Branch: `codex/as-kind-out-of-tick-0c`
- PR: (pending)
- Merge: (pending)

## What changed

- Added `ResolvedFissionChildBlueprint` and `FissionTemplate::spawn_child` in `simthing-core`; `kind_tag_to_kind` mapping lives in core only.
- `execute_fission` spawns children via `ResolvedFissionChildBlueprint::from_template(...).spawn(current_day)` — no `SimThingKindTag` match in `simthing-sim` production code.
- Removed `kind_tag_to_kind` from `simthing-sim`.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `fission_child_spawn_kind_resolution_not_in_sim_runtime` | Fission execution matches core blueprint spawn; sim no longer owns tag→kind mapping |
| `fission_spawns_same_child_kind_after_core_spawn_refactor` | All authored `SimThingKindTag` variants still spawn equivalent `SimThingKind` |
| `fission_custom_child_kind_preserved` | `Custom(...)` child kinds survive the moved constructor |
| `fission_clone_capability_subtrees_behavior_preserved` | 0A/0B clone-source markers, ids, overlay remapping preserved |
| `peek_kind` (`compile_fail` on `FissionCloneSourceView`) | Runtime clone view remains kind-free |
| `fission_child_blueprint_hides_kind_compile_fail` (`compile_fail` on `ResolvedFissionChildBlueprint`) | Spawn blueprint does not expose `.kind` / `.child_kind` |

## Scope Ledger

| File | Reason |
|---|---|
| `crates/simthing-core/src/fission_child_spawn.rs` | Core spawn blueprint + kind_tag_to_kind + compile_fail proofs |
| `crates/simthing-core/src/lib.rs` | Export `ResolvedFissionChildBlueprint` |
| `crates/simthing-sim/src/fission.rs` | Use core blueprint; remove local kind_tag_to_kind; load-bearing tests |
| `docs/tests/as_kind_out_of_tick_0c_results.md` | Results + Scope Ledger |
| `docs/tests/current_evidence_index.md` | One AS-KIND-OUT-OF-TICK-0C row |

**Guard retired:** production `kind_tag_to_kind` / `SimThingKindTag` match in `simthing-sim` fission execution.

## Known gaps / next

- Other production `SimThingKind` reads in `simthing-sim` (tests, fixtures) remain — later AS-KIND-OUT-OF-TICK-0 slices.
- Full AS-3 production kind-read audit deferred; parent rung stays IN PROGRESS.
