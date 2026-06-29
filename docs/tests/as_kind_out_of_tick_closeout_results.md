# AS-KIND-OUT-OF-TICK-0 Closeout Ledger

Consolidated closeout evidence for rung **AS-KIND-OUT-OF-TICK-0** (slices 0A–0E). Sub-slice result docs remain as historical record; this ledger is the DA closeout view.

## Status

**PROBATION** — production `simthing-sim/src` has zero runtime kind reads; core §9 “no match kind” promoted to type boundary + production audit. Pending DA net-negative confirmation (not DA-APPROVED in this remediation).

## Sub-slice summary

| Slice | PR | Merge | What landed |
|---|---|---|---|
| **0A** | #952 | `ba132e2041` | Fission clone-source via marker property + `FissionCloneSourceView`; removed `is_capability_container` kind branch from boundary/fission |
| **0B** | #953 | `fd47c75280` | Markers prepared at `BoundaryProtocol::new` / AddChild; projection/execution read markers only |
| **0C** | #954 | `7f772392c3` | `ResolvedFissionChildBlueprint` in core; `kind_tag_to_kind` removed from sim production |
| **0D** | #955 | `20f5d2e76a` | Removed public `FissionCloneSourceView::inner()` kind backdoor |
| **0E** | #957 | `5e539ab007` | Production kind-read audit; parent promoted PROBATION |

Detail: `docs/tests/as_kind_out_of_tick_0{a,b,c,d,e}_results.md`.

## Type boundary promotion

| Before | After |
|---|---|
| Host `match kind` / `.kind` in tick-adjacent production paths | Resolved marker columns + kind-free views (`FissionCloneSourceView`, `ResolvedFissionChildBlueprint`) |
| Scattered grep for “no match kind” | `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` over `simthing-sim/src` production modules |
| Public kind backdoors on tick views | `compile_fail` doc-tests on `.kind` access |

## Remaining kind mentions (classified)

| Location | Classification |
|---|---|
| `simthing-sim` unit/integration tests | Test/fixture — constructs `SimThingKind` for scenarios |
| `simthing-core` deprecated `Faction` variant | Legacy serialization compatibility — not tick production path |
| CPU-boundary `SimRuntimeTree::access` closures in driver | Boundary-time residue — outside tick dispatch (AS-4 ledger) |

## Retired guards

- Per-slice prose “no kind in tick” restatements as primary enforcement — superseded by production audit + type boundaries.
- Reliance on `is_capability_container(&child.kind, …)` in fission/boundary production paths — deleted.

## Load-bearing proofs (consolidated)

- `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads`
- `peek_kind` / fission view compile_fail doc-tests
- `fission_clone_sources_no_longer_require_kind_branch`
- `projected_fission_slots_match_resolved_clone_sources`

## Hold point

No further AS-3 coding in AS-2 remediation. DA confirmation is Opus queue item.
