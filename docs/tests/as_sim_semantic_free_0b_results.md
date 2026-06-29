# AS-SIM-SEMANTIC-FREE-0B Results

## Status

PASS — public CPU-boundary raw-tree backdoors sealed; replay/delta payloads and driver consumers migrated to kind-free `SimRuntimeTree` APIs. Parent AS-SIM-SEMANTIC-FREE-0 remains **IN PROGRESS** (internal boundary modules still walk `SimThing` via `pub(crate)`).

## PR / branch / merge

- Branch: `codex/as-sim-semantic-free-0b`
- PR: (pending push)
- Merge: (pending)

## What changed

- `SimRuntimeTree::access` / `access_mut` demoted to `pub(crate)`; kind-free query/mutation helpers added (`snapshot_node`, `project_to_values`, `seed_properties_on_node`, etc.).
- `BoundaryDeltaEntry::SimThingAdded` / `FissionOccurred` and `ReplaySnapshot.root` / `ReplayDriver.root` carry `SimRuntimeTree` instead of raw `SimThing`.
- `apply_structural_mutations` and `observe` take `&mut SimRuntimeTree` / `&SimRuntimeTree` (no public `&SimThing` borrows).
- Public `ThresholdBuilder::build*` and `sync_gpu_buffers` take `&SimRuntimeTree`.
- Driver session/resource-flow paths use `project_to_values`, `seed_properties_on_node`, and `clone().into_admitted()` + `replace()` instead of `.access` / `.access_mut`.
- Public surface audit extended to reject `pub fn` raw `&SimThing` / `&mut SimThing` signatures.

## Public raw-tree residue audit

| Surface | Before 0B | After 0B |
|---|---|---|
| `SimRuntimeTree::access` / `access_mut` | public | `pub(crate)` |
| `ReplaySnapshot.root` | `SimThing` | `SimRuntimeTree` |
| `ReplayDriver.root` | `SimThing` | `SimRuntimeTree` |
| `BoundaryDeltaEntry` spawn payloads | `SimThing` | `SimRuntimeTree` |
| `apply_structural_mutations` | `&mut SimThing` (public) | `&mut SimRuntimeTree` (public) |
| `observability::observe` | `&SimThing` (public fn in module) | `&SimRuntimeTree` |
| `ThresholdBuilder::build*` | `&SimThing` | `&SimRuntimeTree` |
| Driver `proto.root.access*` | public backdoor usage | removed — kind-free APIs |
| CPU admission seam (`admit` / `into_admitted` / `replace`) | allowed | allowed (documented) |
| Internal boundary walkers (`fission`, `overlay_lifecycle`, …) | `pub(crate)` / private `&SimThing` | unchanged — not public surface |

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `sim_runtime_tree_rejects_access_kind_backdoor_compile_fail` | public `tree.access(\|root\| root.kind…)` |
| `sim_runtime_tree_rejects_access_mut_kind_backdoor_compile_fail` | public `tree.access_mut` kind peek/mutate |
| `sim_public_surface_rejects_raw_simthing_borrows` | `pub fn` signatures with `&SimThing` / `&mut SimThing` |
| `as_sim_semantic_free_public_surface_audit` | kind tokens + raw borrow signatures on public modules |
| `sim_public_surface_rejects_kind_import_compile_fail` | `use simthing_sim::SimThingKind` |
| `sim_runtime_tree_hides_kind_compile_fail` | `tree.kind` field access |
| `replay_snapshot_hides_raw_root_kind_compile_fail` | `snap.root.access(… kind …)` |
| `boundary_delta_entry_hides_raw_simthing_kind_compile_fail` | delta payload `node.access(… kind …)` |
| `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` | AS-3 production kind-read regression |

## Scope Ledger

| File | Change |
|---|---|
| `crates/simthing-sim/src/sim_runtime_tree.rs` | `pub(crate)` access; kind-free APIs; Debug; compile_fail docs |
| `crates/simthing-sim/src/delta_log.rs` | `SimRuntimeTree` payloads; compile_fail doc |
| `crates/simthing-sim/src/replay.rs` | `SimRuntimeTree` snapshot/driver; compile_fail doc |
| `crates/simthing-sim/src/boundary.rs` | snapshot/observe/structural path |
| `crates/simthing-sim/src/tree_mutation.rs` | `&mut SimRuntimeTree` entry |
| `crates/simthing-sim/src/observability.rs` | `&SimRuntimeTree` entry |
| `crates/simthing-sim/src/threshold_registry.rs` | public builders → `&SimRuntimeTree` |
| `crates/simthing-sim/src/gpu_sync.rs` | `sync_gpu_buffers` → `&SimRuntimeTree` |
| `crates/simthing-sim/src/semantic_surface_audit.rs` | raw-borrow audit |
| `crates/simthing-sim/tests/boundary_integration.rs` | kind-free tree queries |
| `crates/simthing-sim/tests/pivot_forward_remedial.rs` | `direct_child_id` |
| `crates/simthing-sim/tests/s2_legacy_intensity_sunset.rs` | `direct_child_id` |
| `crates/simthing-sim/tests/c3_overlay_add_accumulator_parity.rs` | `direct_child_id` |
| `crates/simthing-driver/src/session.rs` | kind-free driver tree access |
| `crates/simthing-driver/src/resource_flow_opt_in_burn_in.rs` | `into_admitted` plan path |
| `crates/simthing-spec/tests/pr6_capability_preview.rs` | `SimRuntimeTree` structural API |
| `crates/simthing-spec/tests/pr10_scripted_event_thresholds.rs` | `ThresholdBuilder` API |
| `crates/simthing-clausething/tests/ct_3b_4a_headline.rs` | structural / plan paths |
| `crates/simthing-driver/tests/support/palma_terran_pirate_tree.rs` | structural API |
| `docs/tests/as_sim_semantic_free_0b_results.md` | evidence (this file) |
| `docs/tests/current_evidence_index.md` | index row |
| `docs/design_0_0_8_4_admission_substrate.md` | parent state IN PROGRESS |

## Conformance (spine / D-directives held)

- No new subsystem, validation registry, WGSL semantic layer, or AS-5+ work.
- Boundary behavior preserved; replay/delta round-trips green on targeted tests.
- Doctrine as type: public raw-tree borrows uncompilable at crate boundary; admission seam documented.

## Known gaps / next

- **AS-SIM-SEMANTIC-FREE-0C** (if needed): audit remaining `pub` re-exports and cross-crate helpers that still mention `SimThing` by value at documented seams only.
- Full AS-4 **PROBATION** when public surface has zero documented residue and WGSL/text scans are narrowed.
- Driver test support modules outside this slice may still pass `&SimThing` to arena helpers — migrate opportunistically.
