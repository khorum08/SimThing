# AS-KIND-OUT-OF-TICK-0B Results

## Status

IN PROGRESS — moved fission clone-source marker stamping out of boundary slot projection and fission execution into earlier admission seams.

## PR / branch / merge

- Branch: `codex/as-kind-out-of-tick-0b`
- PR: (pending)
- Merge: (pending)

## What changed

- Added `prepare_fission_clone_sources_for_registry`, `prepare_fission_clone_sources_subtree`, and `fission_clone_source_container_kinds_for_registry` in `simthing-core`; prep is idempotent (already-stamped nodes are skipped).
- `BoundaryProtocol::new` prepares clone-source markers from active fission templates before any boundary tick.
- `apply_add_child` prepares incoming subtrees at structural admission before attach.
- Removed `prep_fission_parent_clone_source_labels` calls from `projected_fission_slots` and `execute_fission`; slot projection again takes `&SimThing` (read-only).

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `fission_clone_sources_prepared_before_boundary_projection` | Markers exist before `projected_fission_slots` runs |
| `projected_fission_slots_does_not_stamp_or_mutate_root` | Slot projection is read-only with respect to clone-source markers |
| `execute_fission_does_not_stamp_or_mutate_source_markers` | Fission execution consumes existing markers without kind resolution |
| `structural_add_child_prepares_clone_source_markers_once` | AddChild ingress stamps markers once; repeated prep does not restamp |
| `fission_clone_capability_subtrees_behavior_preserved` | 0A clone behavior preserved with earlier preparation |
| `peek_kind` (`compile_fail` on `FissionCloneSourceView`) | Runtime view remains kind-free |
| `prepare_fission_clone_sources_for_registry_is_idempotent` | Repeated registry prep does not mutate marker bytes |

## Scope Ledger

| File | Reason |
|---|---|
| `crates/simthing-core/src/fission_clone_source.rs` | Registry/subtree prep API + idempotent prep |
| `crates/simthing-core/src/lib.rs` | Export prep API |
| `crates/simthing-sim/src/boundary.rs` | Prep at `BoundaryProtocol::new`; remove boundary stamping; new tests |
| `crates/simthing-sim/src/fission.rs` | Remove execute_fission prep; test updates + execute marker test |
| `crates/simthing-sim/src/tree_mutation.rs` | AddChild structural admission prep |
| `docs/tests/as_kind_out_of_tick_0b_results.md` | Results + Scope Ledger |
| `docs/tests/current_evidence_index.md` | One AS-KIND-OUT-OF-TICK-0B row |

**Guard retired:** boundary/fission hot-path calls to `prep_fission_parent_clone_source_labels`.

## Known gaps / next

- `kind_tag_to_kind` in `fission.rs` still used to spawn fission children (AS-3 residue — later slice).
- Driver/install admission may call `prepare_fission_clone_sources_for_registry` when constructing roots outside `BoundaryProtocol::new` (optional follow-up; `BoundaryProtocol::new` and AddChild cover simthing-sim ingress).
- Full AS-3 production kind-read audit deferred to later AS-KIND-OUT-OF-TICK-0 slices.
