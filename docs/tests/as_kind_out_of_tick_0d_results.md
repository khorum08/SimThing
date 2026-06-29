# AS-KIND-OUT-OF-TICK-0D Results

## Status

PASS — sealed `FissionCloneSourceView` kind backdoor by removing public `inner()`.

## PR / branch / merge

- Branch: `codex/as-kind-out-of-tick-0d`
- PR: (pending)
- Merge: (pending)

## What changed

- Removed public `FissionCloneSourceView::inner() -> &SimThing`; the view exposes only `id()` and `children()`.
- Added compile-fail proof that `view.inner().kind` is uncompilable (method absent).
- Existing compile-fail proof that `view.kind` is uncompilable retained.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `fission_clone_source_view_hides_kind_compile_fail` | Direct `view.kind` access does not compile |
| `fission_clone_source_view_inner_kind_backdoor_compile_fail` | `view.inner().kind` does not compile (`inner()` removed) |
| `fission_clone_capability_subtrees_behavior_preserved` | 0A/0B/0C clone behavior intact |
| `projected_fission_slots_match_resolved_clone_sources` | Slot projection and clone execution agree on marker state |

## Scope Ledger

| File | Reason |
|---|---|
| `crates/simthing-sim/src/fission_clone_source_view.rs` | Remove `inner()` backdoor; add inner compile_fail doc test |
| `docs/tests/as_kind_out_of_tick_0d_results.md` | Results + Scope Ledger |
| `docs/tests/current_evidence_index.md` | One AS-KIND-OUT-OF-TICK-0D row |

**Guard retired:** public `FissionCloneSourceView::inner()` kind recovery path.

## Known gaps / next

- Other production `SimThingKind` reads in `simthing-sim` (tests, fixtures) remain — later AS-KIND-OUT-OF-TICK-0 slices.
- Full AS-3 production kind-read audit deferred; parent rung stays IN PROGRESS.
