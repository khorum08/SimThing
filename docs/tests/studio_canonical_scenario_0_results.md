# STUDIO-CANONICAL-SCENARIO-0 Results

## Status
**DA-GRADUATED / COMPLETE** — merged [#1300](https://github.com/khorum08/SimThing/pull/1300) @ `46f6151e`.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1300](https://github.com/khorum08/SimThing/pull/1300) |
| branch | `studio-canonical-scenario-0` |
| base | `master` |
| head_sha | `e768795b8304082d39110ab60c8229bd203e105e` |
| merge | `46f6151e` (squash) |

## What changed
- `hydrate_scenario_with_source_base` + `resolve_clause_source_path` — relative `source_json`/`include_json` resolve against clause-file directory; absolute pass-through; tokens caller-substituted before parse
- Legacy `hydrate_scenario` = `with_source_base(..., None)` (CWD-relative)
- Committed `scenarios/terran_pirate_galaxy.clause` + sibling `terran_pirate_galaxy.base_disc.json` (no `{{FIXTURE_JSON}}`)
- 4 headless proofs + TEST-BUDGET triage row

## Proof matrix
| test | catches |
|---|---|
| empty_resolver hydrates from non-scenarios cwd | CWD-relative leakage |
| multi_tick holds identity | hydrate-only without session stability |
| backcompat FIXTURE_JSON token | resolver-token regression |
| no sibling from-clause output | generated cruft near scenarios/ |

## Scope Ledger
| | |
|---|---|
| Specified | Clause-relative paths + portable TP scenario + empty-resolver proof |
| Implemented | clausething API + scenarios pair + proofs |
| Proxied | none |
| Deferred | 11.2–11.7; production `clause_scenario_ingest` still calls bare `hydrate_scenario` (wire `source_base` in 11.4) |
| Out of scope | mapeditor UI, spec faction fields, mapgenerator naming, driver/kernel/GPU, clearance |

## Conformance
empty resolver YES · non-scenarios cwd YES · multi-tick identity YES · token back-compat YES · no sibling output YES · no 11.2+ leak YES · triage_log TEST-BUDGET YES

## Known residuals
- Production Studio ingest (`clause_scenario_ingest`) still uses bare `hydrate_scenario`; relative-sibling operator load from non-scenarios CWD needs `source_base` wiring in **11.4** (or thin follow-up).
- Next: `STUDIO-FACTION-IDENTITY-FIELDS-0` (11.2, sequential).

## Graduation routing
**DA PASS** — sticky `admitted-scope-router-gap` is router debt (Tier-A, no class yet); not a design breach. Pointer → `STUDIO-FACTION-IDENTITY-FIELDS-0`.

## DA ACK
```text
ANCHOR-ACK: clausething-closed-vertical@beb30ffaba50
ANCHOR-ACK: stead-spatial-contract-core@b4a112cd02e8
ANCHOR-ACK: orientation-harness-core@8a365d1c0864
```
