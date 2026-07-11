# STUDIO-LIVE-OPS-CLASS-0 Results

## Status
**PROBATION** — gate-wiring class registration. Not complete / not graduated. Do not self-merge.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1293](https://github.com/khorum08/SimThing/pull/1293) |
| branch | `studio-live-ops-class-0` |
| base | `master` |
| head_sha | `7897f650a8906981629d8f1a91554942a799a157` |
| merge | NOT MERGED |

## What changed
- Precedented class `studio-live-ops-ui-clock` in `scripts/ci/precedented_classes.tsv`
- Predicate row in `scripts/ci/class_predicates.tsv` (`any_then_envelope`, priority 40)
- 12 clearance fixtures under `scripts/ci/fixtures/clearance/clearance_selftest_live_ops_class_*`
- Selftest registration in `scripts/ci/clearance_check.sh` (fixture list only; no routing algorithm change)
- Clock-class `rejects_live_bridge` expected verdict updated to `ORCHESTRATOR-CLEARABLE` when live-bridge path is present (live-ops class supersedes substrate class)
- Evidence + design 9.7 PROBATION row + inventory

No production Studio behavior changes.

## Class row and predicate row

```text
class_id: studio-live-ops-ui-clock
envelope: 0.0.8.6-studio-live-ops
requirements: tested_code_sha|coverage_basis|ci_green
status: active
promotion_blocker: STUDIO-LIVE-OPS-CLASS-0
detect_mode: any_then_envelope
priority: 40
```

**match_any** (distinctive live-ops modules / tests / results — not bare clock substrate alone):
`studio_sim_clock_ui`, `studio_live_observe`, `studio_live_session_bridge`, `studio_scenario_library_ui`, their tests/results, class results.

**scope** (companions allowed): app/ui, app/mod, lib, studio_sim_clock substrate, live-ops modules, tests, studio results, design, orientation, inventory, triage.

**forbidden**: driver, kernel, sim, gpu, workshop, spec/src, clausething/src, wgsl, workflows, clearance_check, binding/ledger/anchors/anchor scripts, allow/**

## Clearable fixture proof
| Fixture | Expected |
|---|---|
| `clearance_selftest_live_ops_class_clearable` | `ORCHESTRATOR-CLEARABLE` |
| `clearance_selftest_live_ops_class_clock_substrate_nonregression` | `ORCHESTRATOR-CLEARABLE` (9.1h substrate still clearable) |

## Envelope reject fixture proofs
| Fixture | Expected | catches |
|---|---|---|
| `..._rejects_driver` | `class-envelope-violation` | driver surface |
| `..._rejects_kernel_sim_wgsl` | `class-envelope-violation` | kernel/sim/WGSL |
| `..._rejects_workshop` | `class-envelope-violation` | workshop |
| `..._rejects_spec_clause` | `class-envelope-violation` | spec/clause compiler |
| `..._rejects_gate_wiring` | `gate-wiring` | clearance router / class TSV |
| `..._missing_tested_sha` | `FAIL(missing-tested-code-sha...)` | head-bound fields |
| `..._missing_coverage` | `FAIL(missing-coverage-basis...)` | head-bound fields |
| `..._missing_ci_green` | `FAIL(ci-not-green...)` | ci_green required |
| `..._explicit_novelty` | `DA-RESERVE(novelty)` | novelty overrides class |
| `..._admitted_scope_gap` | `admitted-scope-router-gap` | no accidental clearable |

## Scope Ledger
| | |
|---|---|
| Specified | Precedented class for stabilized live-ops UI/clock/library/observation |
| Implemented | Class + predicate + fixtures + selftest registration + evidence |
| Proxied | none |
| Deferred | 9.8 hardening; JSON/Clause load bridge-reset residual (create does the right thing) |
| Out of scope | Production Studio code; workflow rewrite; binding/anchors; algorithm rewrite |

## Conformance
- Intended shape routes clearable: YES
- Envelope rejects engine/driver/sim/WGSL/workshop/spec/clause: YES
- Router/workflow/binding edits gate-wiring: YES
- Required fields enforced: YES
- Explicit novelty preserved: YES
- Existing studio-sim-clock-substrate selftests still pass: YES
- No production Studio behavior change: YES

## Known residuals / next
- **9.8 candidate:** JSON/Clause load still omit bridge reset; create does the right thing (DA residual). Do not fix in 9.7.
- Next rung: `STUDIO-LIVE-OPS-HARDENING-0`

## Graduation routing
**PROBATION** — expected clearance for this PR: `DA-RESERVE(gate-wiring)` (touches class/predicate/clearance selftest list). DA review expected. Do not self-merge.

## Selftest result
```text
CLEARANCE-SELFTEST: PASS (90 fixtures)
```
