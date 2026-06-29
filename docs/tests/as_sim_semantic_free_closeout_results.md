# AS-SIM-SEMANTIC-FREE-0 Closeout Ledger

Consolidated closeout evidence for rung **AS-SIM-SEMANTIC-FREE-0** (slices 0A–0C). Sub-slice result docs remain as historical record.

## Status

**DONE -- DA-APPROVED** — public `simthing-sim` surface cannot name semantic kinds or expose raw `SimThing` tick paths; audit narrowed to true residue.  DA-approved per graduation log in docs/design_0_0_8_4_admission_substrate.md (2026-06-29).

## Sub-slice summary

| Slice | PR | Merge | What landed |
|---|---|---|---|
| **0A** | #958 | `51a8acc635` | `SimRuntimeTree`; `BoundaryProtocol::new(SimRuntimeTree)`; demoted tree_index / entries_from_outcome / from_node leaks |
| **0B** | #959 | `886ad2d74f` | Sealed CPU-boundary raw-tree backdoors; driver migrated to kind-free tree access |
| **0C** | #960 | `2fb4cf6f39` | Sealed owned-`SimThing` seams; driver no longer extracts raw trees from sim |

Detail: `docs/tests/as_sim_semantic_free_0{a,b,c}_results.md`.

## Public surface — narrowed residue

| Item | Status |
|---|---|
| `SimThingKind` / semantic re-exports at `lib.rs` | **Absent** — compile_fail |
| `BoundaryProtocol::new(SimThing)` | **Deleted** — `SimRuntimeTree` only |
| Public `&SimThing` / owned `SimThing` tick APIs | **Sealed or pub(crate)** |
| Production `simthing-sim/src` semantic kind names | **Clean** — audit green |
| WGSL / shader text | **True residue** — Rust type space cannot see shader strings; remains scan-only |
| Replay/delta owned `SimThing` payloads | **CPU-boundary residue** — not tick dispatch |
| `observability` module (not re-exported) | **Peripheral residue** — documented in 0A audit |

## Retired guards

- Broad scattered semantic-free source scans as primary gate — narrowed to `as_sim_semantic_free_public_surface_audit` on declared public modules + compile_fail seams.

## Load-bearing proofs (consolidated)

- `as_sim_semantic_free_public_surface_audit`
- `boundary_protocol_rejects_semantic_root_compile_fail`
- `sim_public_surface_rejects_kind_import_compile_fail`
- `sim_runtime_tree_rejects_kind_backdoor_compile_fail` (0C)

## Hold point

No further AS-4 coding in AS-2 remediation. WGSL-text-only residue acknowledged; DA confirmation is Opus queue item.
