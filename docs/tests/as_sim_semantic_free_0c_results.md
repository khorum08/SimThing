# AS-SIM-SEMANTIC-FREE-0C Results

## Status

PASS — public owned-`SimThing` escape seams sealed; driver production paths no longer round-trip raw trees through `simthing-sim`. Parent **AS-SIM-SEMANTIC-FREE-0** moved to **PROBATION**.

## PR / branch / merge

- Branch: `codex/as-sim-semantic-free-0c`
- PR: #960
- Merge: `2fb4cf6f39`

## What changed

- `SimRuntimeTree::into_admitted` / `replace` demoted to `pub(crate)`; public `swap`, `append_child`, and kind-free `node_is_arena_participant` added.
- Public surface audit extended for owned `SimThing` payloads (single exempt seam: `SimRuntimeTree::admit(SimThing)`).
- `compile_fail` doc tests added for public `into_admitted().kind` and `replace(...).kind` escapes.
- Driver session/resource-flow/fission enrollment operate on `&SimRuntimeTree` / `&mut SimRuntimeTree` without public extraction.
- `build_execution_plan` takes `&SimRuntimeTree`; authoring/test adapters (`*_from_authoring`, `*_on_authoring`) preserve fixture trees in driver/spec/clausething tests.
- `simthing_core::is_arena_participant_node` added for kind-free topology predicates at the core boundary.
- AS-4 closure composite test ties public-surface and AS-3 kind-read audits.

## Owned-SimThing seam audit

| Surface | Before 0C | After 0C |
|---|---|---|
| `SimRuntimeTree::into_admitted()` | public | `pub(crate)` |
| `SimRuntimeTree::replace(SimThing)` | public | `pub(crate)` |
| `SimRuntimeTree::admit(SimThing)` | public (documented seam) | unchanged — sole public owned parameter |
| Public owned `SimThing` returns | `into_admitted` / `replace` | none |
| Driver `clone().into_admitted()` + `replace()` | session install/sync/fission | removed — `scenario.root` + `admit`, in-place `&mut SimRuntimeTree` |
| `build_execution_plan` input | `&SimThing` via extraction | `&SimRuntimeTree` in production |
| Public audit owned-payload rule | borrow-only (0B) | kind + borrow + owned (exempt `admit` only) |

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `sim_runtime_tree_rejects_into_admitted_kind_escape_compile_fail` | public `tree.into_admitted().kind` |
| `sim_runtime_tree_rejects_replace_kind_escape_compile_fail` | public `tree.replace(...).kind` |
| `sim_public_surface_rejects_owned_simthing_escape` | public fn owned `SimThing` returns/params except `admit` |
| `as_sim_semantic_free_public_surface_audit` | kind tokens + raw borrows + owned payloads |
| `as_sim_semantic_free_public_surface_closure_is_clean` | public-surface + AS-3 production kind-read composite |
| `sim_public_surface_rejects_kind_import_compile_fail` | `use simthing_sim::SimThingKind` |
| `sim_runtime_tree_hides_kind_compile_fail` | `tree.kind` field access |
| `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` | AS-3 regression |

## Scope Ledger

| File | Change |
|---|---|
| `crates/simthing-core/src/simthing.rs` | `is_arena_participant_node` |
| `crates/simthing-core/src/lib.rs` | export helper |
| `crates/simthing-sim/src/sim_runtime_tree.rs` | demote owned seams; kind-free APIs; compile_fail docs |
| `crates/simthing-sim/src/semantic_surface_audit.rs` | owned-payload audit |
| `crates/simthing-sim/src/lib.rs` | closure test module |
| `crates/simthing-sim/src/kind_production_audit.rs` | pub test fn for closure |
| `crates/simthing-driver/src/session.rs` | no public tree extraction |
| `crates/simthing-driver/src/arena_allocation_sync.rs` | `&SimRuntimeTree` sync |
| `crates/simthing-driver/src/arena_hierarchy.rs` | runtime-tree planning + authoring adapter |
| `crates/simthing-driver/src/arena_participant.rs` | runtime enrollment + authoring adapters |
| `crates/simthing-driver/src/resource_flow_fission_enrollment.rs` | `&mut SimRuntimeTree` + authoring adapter |
| `crates/simthing-driver/src/resource_flow_opt_in_burn_in.rs` | plan from runtime root |
| `crates/simthing-driver/src/lib.rs` | re-exports |
| `crates/simthing-spec/tests/pr6_capability_preview.rs` | serde roundtrip vs `into_admitted` |
| `crates/simthing-clausething/tests/ct_3b_4a_headline.rs` | runtime-root plan path |
| `crates/simthing-driver/tests/support/palma_terran_pirate_tree.rs` | serde roundtrip |
| `crates/simthing-driver/tests/e2b5_dynamic_fission_enrollment.rs` | authoring enrollment adapters |
| `crates/simthing-driver/tests/support/e2b5_dynamic_enrollment_soak.rs` | authoring enrollment adapter |
| `crates/simthing-driver/tests/e11b_nested_fission_gap.rs` | refresh on authoring adapter |
| `crates/simthing-driver/tests/phase_e_a0_nested_resource_flow_static.rs` | refresh on authoring adapter |
| Driver/clausething RF tests (multiple) | `build_execution_plan_from_authoring` |
| `docs/tests/as_sim_semantic_free_0c_results.md` | evidence (this file) |
| `docs/tests/current_evidence_index.md` | index rows (0B fix + 0C) |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-SIM-SEMANTIC-FREE-0 → PROBATION |

## Conformance (spine / D-directives held)

- No new subsystem, validation registry, WGSL semantic layer, or AS-5+ work.
- Doctrine as type: public owned-tree extraction uncompilable; single documented CPU admission seam.
- Replay/delta JSON compatibility preserved (targeted replay/delta tests green).

## Known gaps / next

- WGSL/shader-text semantic residue remains future AS-4 true residue (not Rust public surface).
- Driver authoring adapters (`*_from_authoring`, `*_on_authoring`) are intentional test/authoring bridges — not `simthing-sim` public API.
- `pub(crate)` `replace` / `access*` remain internal boundary residue until a later rung retires them.
