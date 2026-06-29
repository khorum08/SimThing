# AS-SIM-SEMANTIC-FREE-0A Results

## Status

PASS — public `simthing-sim` surface audited; first load-bearing semantic leak (`BoundaryProtocol::new(SimThing)`) sealed via `SimRuntimeTree`; AS-SIM-SEMANTIC-FREE-0 promoted OPEN → IN PROGRESS.

## PR / branch / merge

- Branch: `codex/as-sim-semantic-free-0a`
- PR: #958
- Merge: `51a8acc635`

## What changed

- Added `SimRuntimeTree` opaque runtime root: `admit(SimThing)` at CPU boundary; no public `.kind`; `access` / `access_mut` for driver CPU-boundary residue.
- `BoundaryProtocol::root` is now `SimRuntimeTree`; `BoundaryProtocol::new` accepts `SimRuntimeTree` only (direct `SimThing` admission uncompilable).
- Demoted `tree_index` to `pub(crate)` (was exposing `&SimThing` path helpers publicly).
- Removed public re-export of `entries_from_outcome` (took `&SimThing`).
- Made `FissionCloneSourceView::from_node` `pub(crate)` (was public `&SimThing` constructor).
- Added `as_sim_semantic_free_public_surface_audit` and compile_fail doc-tests for kind import / semantic root / runtime-tree kind backdoor.
- Updated `simthing-driver` session + resource-flow call sites and `simthing-sim` integration tests for `SimRuntimeTree::admit` + `root.access(_mut)`.

## Public-surface semantic audit

Audit scope: all `pub mod`, `pub use`, public structs/enums, signatures, fields, and non-test production source under `crates/simthing-sim/src`.

| Item | Classification | Notes |
|---|---|---|
| `SimThingKind` / `SimThingKindTag` re-export | **Absent** | Not exported from `lib.rs`; `sim_public_surface_rejects_kind_import_compile_fail` |
| `BoundaryProtocol::new(SimThing)` | **Fixed (0A)** | Now `new(SimRuntimeTree)`; `boundary_protocol_rejects_semantic_root_compile_fail` |
| `BoundaryProtocol.root: SimThing` | **Fixed (0A)** | Now `SimRuntimeTree`; no public `.kind` |
| `pub mod tree_index` + `&SimThing` helpers | **Fixed (0A)** | `pub(crate)` |
| `pub use entries_from_outcome` | **Fixed (0A)** | `pub(crate)` |
| `FissionCloneSourceView::from_node(&SimThing)` | **Fixed (0A)** | `pub(crate)` |
| `SimRuntimeTree::access` / `access_mut` | **CPU-boundary residue (0B)** | Driver may read `.kind` inside closure; not sim-public kind API |
| `BoundaryDeltaEntry` embeds `SimThing` | **CPU-boundary residue (0B)** | Replay/delta log shape |
| `ReplaySnapshot.root: SimThing` | **CPU-boundary residue (0B)** | Replay record, not tick dispatch |
| `pub use apply_structural_mutations` (`&mut SimThing`) | **CPU-boundary residue (0B)** | Boundary-time structural queue |
| `observability::observe(root: &SimThing)` | **CPU-boundary residue (0B)** | Public module; not re-exported in `lib.rs` |
| Production `SimThingKind` / `.kind` in `src/` | **Clean (AS-3)** | Test/fixture only; `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` |
| `replay.rs` JSON `"kind"` field | **Replay-record shape** | Not SimThing kind dispatch |

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `as_sim_semantic_free_public_surface_audit` | Public production modules naming `SimThingKind` / `SimThingKindTag` / `kind_tag_to_kind` |
| `sim_public_surface_rejects_kind_import_compile_fail` | `use simthing_sim::SimThingKind` re-export leakage |
| `boundary_protocol_rejects_semantic_root_compile_fail` | Direct `SimThing` / `SimThingKind` at `BoundaryProtocol::new` |
| `sim_runtime_tree_hides_kind_compile_fail` | Public `.kind` on `SimRuntimeTree` |
| `fission_clone_source_view_hides_kind_compile_fail` | Direct `view.kind` on public runtime view |
| `fission_clone_source_view_inner_kind_backdoor_compile_fail` | `view.inner().kind` backdoor (0D) |
| `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` | AS-3 production kind-read regression |

## Scope Ledger

| File | Reason |
|---|---|
| `crates/simthing-sim/src/sim_runtime_tree.rs` | New opaque runtime tree type + compile_fail proofs |
| `crates/simthing-sim/src/boundary.rs` | `SimRuntimeTree` root/new; internal access |
| `crates/simthing-sim/src/lib.rs` | Exports, audit module, compile_fail doc-test |
| `crates/simthing-sim/src/semantic_surface_audit.rs` | Public surface audit test |
| `crates/simthing-sim/src/delta_log.rs` | `entries_from_outcome` visibility |
| `crates/simthing-sim/src/fission_clone_source_view.rs` | `from_node` visibility |
| `crates/simthing-driver/src/session.rs` | `SimRuntimeTree::admit` + `access(_mut)` |
| `crates/simthing-driver/src/resource_flow_opt_in_burn_in.rs` | `root.access` for execution plan |
| `crates/simthing-sim/tests/*.rs` (18 files) | `SimRuntimeTree::admit` + `proto.root.access(_mut)` |
| `docs/tests/as_sim_semantic_free_0a_results.md` | Results + audit table |
| `docs/tests/current_evidence_index.md` | One AS-SIM-SEMANTIC-FREE-0A row |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-SIM-SEMANTIC-FREE-0 → IN PROGRESS |

## Known gaps / next

- **AS-SIM-SEMANTIC-FREE-0B:** seal CPU-boundary residue (`access` kind backdoor, `BoundaryDeltaEntry`/`ReplaySnapshot` full `SimThing`, `apply_structural_mutations` public signature).
- **AS-SIM-SEMANTIC-FREE-0 closure:** full public surface clean + documented residue → PROBATION (not attempted in 0A).
- Integration tests still construct semantic kinds in fixtures (allowed); some tests need AS-1 `RoleOffset` migration separately (`PropertyValue.data` private).
