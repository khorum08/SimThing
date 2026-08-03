# EVENT-GENERATION-STAMP-0 — DA audit record

- Track: 0.0.8.7 RF arena modernization (rung 6.1)
- Status: **COMPLETE / DA-GRADUATED — merged #1596 @ df117bcc**
- HD-RECEIPT: `9df0629526ec` · ORIENT-RECEIPT: `8bcf881f793a` · rule stamp: `2e9a349eddfe2d31`
- Audited head: `b6c5fcc78e50d692aa77f2844b8ae9041e8cf39d`
- Rulings: `5167040282` (remand, blocker 4) → `5167545986` (remand, blocker 4 subtler) → graduation
- DA addendum implemented: `5166364435`

This record supplements the coder's results doc with the DA's own falsification, since three
successive audits turned on defects the referees did or did not catch.

## Falsification — every defect planted by the DA at the audited head

| planted defect | audit | result |
|---|---|---|
| `IntegrationSchedule::record` → bucket-latest collapse | 1st | **RED** — `schedule_is_per_product_full_generation_set_and_replays_bit_exactly` |
| feeder egress call wrapped in `if false` (text preserved) | 2nd | **PASSED** — nothing in the tree red; this was the remand |
| feeder egress call wrapped in `if false` (text preserved) | 3rd | **RED** — `ordinary_feeder_tick_admits_to_production_event_egress` |
| unsanctioned second `readback_emissions` in a production file | 3rd | **RED** — census names the exact offending site |

The middle row is the load-bearing one: at the second audit the egress method had two real callers
and two green referees, and *still* nothing failed when the live call was killed. "Has a caller" and
"is proven live" are different claims, and only the second is worth a graduation.

## Blocker-4 closure

| requirement | closure |
|---|---|
| live path proven | `WorldGpuState::production_event_egress.admit_invocations` observed after a real `DispatchCoordinator::tick`; removal or dead-code wrap REDs |
| swallowed door | `let _ =` replaced — errors surface as `TickGpuError::ProductionEmissionEgress` and propagate into the returned `TickOutcome` |
| census is a negative claim | `walkdir_rs_files` over the whole crates tree, `/tests/` skipped, `#[cfg(test)]` items stripped, **exact-set equality** against sanctioned pins |

The census identity is `(path, api, code-snippet)` — **content-pinned, not line-pinned**. The relay
described it as `(path, api, line)`; the implementation is better than its description and avoids the
positional fragility flagged against the `compile_fail_line_N` locators.

Both tautological assertions from the prior head (`OBSERVER_EGRESS_API.contains("production_egress")`,
`PARITY_ORACLE_APIS.len() == 2`) are removed.

## Frozen surfaces — accepted at earlier audits, not re-examined

Blockers 1–3 (no production wait branch; generation bound from `coord.day_index()` on mandatory step
boundaries; zero-generation diagnostic segregated), Remand 3 (compile-fail relocation; the E0063
origin proof remains live at line 116 and executes), the per-product schedule addendum, routed
dissolve discipline, async-as-ordinary, and replay semantics.

## Batteries at the audited head (DA-run)

`core`/`sim`/`driver`/`kernel`/`spec`/`feeder`: **0 failures** · `cargo test --doc -p simthing-core`:
23 passed · anchor/orientation/doc-budget PASS · detachability PASS · artifact-provenance PASS ·
residue `scenario=0 domain=0`, `dead_exports=58` (unchanged from base — the export gained callers).

## Carried forward

- **Blocker 3's segregation is prose, not a type.** `// Diagnostic path with generation 0 is not a
  production/parity witness` is the weakest admission tier. Not blocking; worth a type boundary.
- **`compile_fail_line_N` identities remain position-pinned** and will churn on every edit above a
  fence. The census demonstrates the better shape (content pin) — 10.1's doctest pinning should adopt it.
