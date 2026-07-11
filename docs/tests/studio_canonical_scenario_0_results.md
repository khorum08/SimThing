# STUDIO-CANONICAL-SCENARIO-0 Results

## Status
**PROBATION** — not complete; not graduated. DA-reserve by envelope.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1300](https://github.com/khorum08/SimThing/pull/1300) |
| branch | `studio-canonical-scenario-0` |
| base | `master` |
| head_sha | `94471cabc27de750e35c75aee5b9714d14beb955` |
| merge | NOT MERGED |

## What changed
- `hydrate_scenario_with_source_base` + `resolve_clause_source_path` — relative `source_json`/`include_json` resolve against clause-file directory
- Committed `scenarios/terran_pirate_galaxy.clause` + sibling `terran_pirate_galaxy.base_disc.json` (empty-resolver portable)
- Tests: `studio_canonical_scenario_0` (4 proofs)
- TEST-BUDGET triage row for 4 named proofs

## Proof matrix
| test | catches |
|---|---|
| empty_resolver hydrates from non-scenarios cwd | CWD-relative leakage |
| multi_tick holds identity | hydrate-only without session stability |
| backcompat FIXTURE_JSON token | resolver-token regression |
| no sibling from-clause output | generated cruft near scenarios/ |

## Scope Ledger
Specified: clause-relative paths + portable TP scenario + empty-resolver proof
Implemented: as above (clausething + scenarios only)
Deferred: 11.2 faction fields; 11.3 star names; 11.4 loader UI
Out of scope: mapeditor/spec/driver/kernel/GPU/clearance

## Conformance
empty resolver YES · non-scenarios cwd YES · multi-tick identity YES · token back-compat YES · no sibling output YES · no phase-UI YES

## Known residuals
JSON/Clause load bridge reset remains 9.8. Phase 11 UI rungs follow.

## Graduation routing
**PROBATION** — expect DA-RESERVE. Do not self-merge.
