# AS-KIND-OUT-OF-TICK-0A Results

## Status

IN PROGRESS — first slice of AS-3: production fission clone-source eligibility no longer branches on `SimThingKind`.

## PR / branch / merge

- Branch: `codex/as-kind-out-of-tick-0a`
- PR: (pending)
- Merge: (pending)

## What changed

- Added `crates/simthing-core/src/fission_clone_source.rs`: well-known `FISSION_CLONE_SOURCE_PROPERTY_ID` marker, opaque label encode/decode, production `is_fission_clone_source` predicate, and admission/prep `prep_fission_*` helpers that resolve Custom labels once before tick selection.
- Added `crates/simthing-sim/src/fission_clone_source_view.rs`: `FissionCloneSourceView` runtime view (id/children only — no `kind` accessor) plus shared `fission_clone_source_children` iterator.
- Removed production `is_capability_container(&child.kind, …)` from `boundary.rs` slot projection and `fission.rs` clone execution; both paths now prep labels then match resolved property data.
- `projected_fission_slots` takes `&mut SimThing` so prep can stamp clone-source markers before counting.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `fission_clone_sources_no_longer_require_kind_branch` | Clone eligibility follows stamped label property, not authored `SimThingKind` |
| `projected_fission_slots_match_resolved_clone_sources` | Boundary slot projection counts the same resolved clone-source subtrees execution will clone |
| `fission_clone_capability_subtrees_behavior_preserved` | `clone_capability_children` still clones authored subtrees with fresh SimThingIds and remapped overlays |
| `peek_kind` (`compile_fail` doc-test on `fission_clone_source_view`) | `FissionCloneSourceView` cannot expose `.kind` |

## Scope Ledger

| File | Reason |
|---|---|
| `crates/simthing-core/src/fission_clone_source.rs` | New — admission-resolved clone-source marker property |
| `crates/simthing-core/src/lib.rs` | Export fission clone-source API |
| `crates/simthing-sim/src/fission_clone_source_view.rs` | New — kind-free runtime view + shared clone-source iterator |
| `crates/simthing-sim/src/fission.rs` | Replace kind predicate; prep before clone; load-bearing tests |
| `crates/simthing-sim/src/boundary.rs` | Replace kind predicate in slot projection; match test |
| `crates/simthing-sim/src/lib.rs` | Export `FissionCloneSourceView` |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-KIND-OUT-OF-TICK-0 OPEN → IN PROGRESS |
| `docs/tests/current_evidence_index.md` | One AS-KIND-OUT-OF-TICK-0A row |

**Guard retired:** production `is_capability_container` kind match in fission/boundary clone paths.

## Known gaps / next

- `kind_tag_to_kind` in `fission.rs` still reads `FissionTemplate.child_kind` to spawn fission children (AS-3 residue — not this slice).
- Full AS-3 tick/runtime view and remaining production kind-read audit deferred to AS-KIND-OUT-OF-TICK-0 follow-on slices.
- Admission layer (simthing-spec / driver) should eventually stamp `FISSION_CLONE_SOURCE_PROPERTY_ID` at tree ingest so prep is not repeated each boundary; current prep preserves parity with prior kind-based behavior.
